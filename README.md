# LandauSwap-V3

cypherphonk project

## Dev Status (2025-10-21)

- **方向**: 基于 Solana + Anchor 的“Resistant AMM”实验，实现二阶阻力曲线 + 批量撮合以保护 LP、优化小额交易。
- **当前工作**:
  - 在 `landau_swap/programs/landau_swap/src/` 下完成了 MVP 程序骨架：账户结构、错误码、阻力曲线整数数学、`initialize_pool`/`{add,remove}_liquidity`/`place_order`/`settle_batch` 等指令。
  - `compute_rational_trade` 已有 Rust 单测；`tests/landau_swap.ts` 搭建了 Anchor 集成测试，演示小额 vs. 大额交易的费率差异。
  - 解压了本地 Solana 工具链至 `~/solana-release/bin`，需手动 `export PATH="$HOME/solana-release/bin:…"`。
- **阻塞**: `anchor build` 仍需下载 crates.io 依赖，当前环境的网络/代理不可用（反复报 `Failed to connect to localhost:7890` 或 `Could not resolve host: static.crates.io`）。

## Next Steps
1. 修复网络或代理，使 `cargo-build-sbf` 可以访问 crates.io；重新执行 `anchor build` / `anchor test`（或 `npm test`）。
2. 待测试通过后，将真实 token 转账逻辑补进流动性与下单流程，并完善对未校验账户的检查。
3. 进一步扩展：批次时间窗口管理、LP 份额/费用结算、更多曲线实现、性能调优等。
