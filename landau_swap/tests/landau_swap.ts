import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import { LandauSwap } from "../target/types/landau_swap";

const poolSeed = Buffer.from("pool");

describe("landau_swap", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.landauSwap as Program<LandauSwap>;

  let mintA: PublicKey;
  let mintB: PublicKey;
  let poolPda: PublicKey;
  let poolBump: number;

  before(async () => {
    const payer = (provider.wallet as anchor.Wallet).payer;
    mintA = await createMint(provider.connection, payer, payer.publicKey, null, 6);
    mintB = await createMint(provider.connection, payer, payer.publicKey, null, 6);
    [poolPda, poolBump] = PublicKey.findProgramAddressSync(
      [poolSeed, mintA.toBuffer(), mintB.toBuffer()],
      program.programId
    );
  });

  it("initializes pool and settles differential trades", async () => {
    const vaultA = Keypair.generate().publicKey;
    const vaultB = Keypair.generate().publicKey;

    await program.methods
      .initializePool(poolBump, { rational: {} })
      .accounts({
        authority: provider.wallet.publicKey,
        pool: poolPda,
        tokenMintA: mintA,
        tokenMintB: mintB,
        vaultA,
        vaultB,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .addLiquidity(new anchor.BN(1_000_000), new anchor.BN(1_000_000))
      .accounts({
        pool: poolPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    let poolAccount = await program.account.pool.fetch(poolPda);
    console.log("Reserves after add:", poolAccount.reserveA, poolAccount.reserveB);

    await program.methods
      .placeOrder({ aForB: {} }, new anchor.BN(1_000))
      .accounts({
        pool: poolPda,
        trader: provider.wallet.publicKey,
      })
      .rpc();

    await program.methods
      .settleBatch()
      .accounts({
        pool: poolPda,
        settler: provider.wallet.publicKey,
      })
      .rpc();

    poolAccount = await program.account.pool.fetch(poolPda);
    const reserveBAfterSmall = poolAccount.reserveB.toNumber();
    const feeAfterSmall = poolAccount.accumulatedFeeB;
    console.log("Post small swap reserves:", poolAccount.reserveA, reserveBAfterSmall);
    console.log("Fees accrued small:", feeAfterSmall.toString());
    if (!feeAfterSmall.lt(new anchor.BN(50))) {
      throw new Error("Small trade fee expected to be near zero");
    }

    await program.methods
      .placeOrder({ aForB: {} }, new anchor.BN(400_000))
      .accounts({
        pool: poolPda,
        trader: provider.wallet.publicKey,
      })
      .rpc();

    await program.methods
      .settleBatch()
      .accounts({
        pool: poolPda,
        settler: provider.wallet.publicKey,
      })
      .rpc();

    poolAccount = await program.account.pool.fetch(poolPda);
    const reserveBAfterLarge = poolAccount.reserveB.toNumber();
    const feeAfterLarge = poolAccount.accumulatedFeeB;
    const incrementalFee = feeAfterLarge.sub(feeAfterSmall);
    console.log("Post large swap reserves:", poolAccount.reserveA, reserveBAfterLarge);
    console.log("Fees accrued large:", feeAfterLarge.toString());

    if (!feeAfterLarge.gt(feeAfterSmall)) {
      throw new Error("Large trade should accumulate more fees than small trade");
    }
    if (!incrementalFee.gt(new anchor.BN(10_000))) {
      throw new Error("Large trade should pay significant resistance fees");
    }
    if (!(reserveBAfterLarge < reserveBAfterSmall)) {
      throw new Error("Large trade should consume more of token B reserves.");
    }
  });
});
