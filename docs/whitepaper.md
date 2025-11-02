# LandauSwap Resistant AMM — Whitepaper Outline

> Working draft for the full technical paper. Each section lists key questions, data, or artifacts we need to gather before writing the final prose.

---

## 0. Abstract
- One-paragraph summary: pain points of CPMM, core innovation (resistance curve + batch clearing), expected benefits to LPs and aggregators.
- Include headline metrics once simulations are ready (e.g., small-trade slip < X bps, large-trade fee capture > Y%).

## 1. Background & Problem Statement
- **Market gaps**: quantify MEV/LVR leakage in CPMMs, quote studies (Uniswap v2, Curve).
- **Target assets**: stablecoins, payment tokens, long-tail? Provide rationale.
- **Design constraints**: oracle avoidance, Solana performance requirements, fairness for small traders.
- Data to collect: existing research citations, on-chain metrics, hackathon feedback.

## 2. Design Goals & Principles
- Protect LP principal and fee yield.
- Provide near-zero slip for micro trades.
- Make whale trades pay a convex resistance cost that disincentivises price manipulation.
- Remain permissionless and oracle-free; minimize external dependencies.
- Outline fairness and decentralisation principles (batch clearing, open participation).

## 3. Mathematical Model
### 3.1 Definitions
- Introduce reserve variables, trade ratio `phi`, resistance variable `r = phi/2`, pool state.
- State assumptions (continuous reserves, discrete batch updates, no external price feed).

### 3.2 Resistance Curves
- Primary curve: `s(r) = r^2 / (1 + r^2)` (include derivation).
- Alternate curve: `s(r) = 1 - e^{-r^2}` and potential hybrids.
- Constraints satisfied: zero-slip baseline, second-order start, monotonicity, asymptotic protection.
- Include plots comparing to `xy = k` slip.

### 3.3 Execution Price & Fees
- Derive actual output: `Δy = (1 - s(r)) * p * Δx`.
- Show fee capture: `fee = s(r) * p * Δx`, prove it returns to LP vaults.
- Discuss discrete batch impact vs. continuous curve.

### 3.4 Comparison with CPMM
- Analytical comparison of slippage functions.
- Example scenarios (micro trade, whale trade) with numeric tables.
- Highlight LP payoff differences (LVR, fee recovery).

### 3.5 Edge Cases
- Low-liquidity pools, asymmetric reserves.
- Handling of trade ratios approaching pool exhaustion.
- Sensitivity to sandwich attacks more than one batch length.

## 4. Mechanism Design
### 4.1 Batch Clearing
- Define batch window (slot-based), order aggregation rules, settlement triggers.
- Explain how batch neutrality mitigates MEV (same clearing price for same-direction orders).
- Outline incentive model for batch settlers (fees, priority, fail-safes).

### 4.2 Liquidity Management
- LP deposit/withdraw flows, share issuance, fee accounting.
- Strategies for distributing resistance fees back to LPs (per-batch vs. continuous).
- Parameter governance (curve selection, batch length) and safety rails.

### 4.3 Integration with Aggregators
- Quote path: quoting API, computation of expected slip for routing algorithms.
- Partial fills and cross-venue arbitrage handling.
- Security considerations (re-entrancy, front-running mitigations).

## 5. Implementation Architecture (Solana + Anchor)
### 5.1 Account Layout
- `Pool`, `BatchBook` (or `BatchTotals`), `LpShares`, vault accounts, program-derived addresses.
- Memory footprint & rent-exemption analysis.

### 5.2 Instruction Flow
- Sequence diagrams for `initialize_pool`, `add_liquidity`, `place_order`, `settle_batch`.
- Pseudocode aligning on-chain logic with mathematical model.

### 5.3 Performance & Compute
- Expected compute units per swap/batch.
- Parallelism opportunities (concurrent pools, batching frequency).
- Failure handling: partial settlement, rollbacks, slippage bounds.

### 5.4 Security Surfaces
- Access control, PDA validation, signature requirements.
- Batch manipulation risks, DoS vectors, fallback procedures.
- Upgrade path (program-derived upgrade authority, governance plan).

## 6. Evaluation & Testing
- **Simulation framework**: design local simulator or fork-test environment.
- **Metrics**: LP net returns vs. CPMM, small-trade slippage, large-trade fee capture, settlement latency.
- **Scenarios**: stress tests (flash swings, liquidity withdrawals), MEV bots, adversarial traders.
- **Formal verification**: potential invariants, opportunities for model checking.
- Include preliminary charts/plots once data exists.

## 7. Roadmap & Deployment Plan
- Milestones: prototype, testnet beta, audits, mainnet guarded launch.
- Incentive programs: LP mining, settler rewards.
- Governance: multisig → DAO transition plan, emergency actions.

## 8. Tokenomics (If Applicable)
- Fee distribution, potential protocol token utility.
- Emission schedules, lockups, LP incentives (if planned).
- Compliance considerations (KYC/AML for certain regions?).

## 9. Risk Analysis & Mitigations
- Economic risks: low liquidity, whale dominance, cross-market arbitrage.
- Technical risks: Solana outages, compute spikes, batch failures.
- Legal/security disclosures, bug bounty program outline.

## 10. References & Related Work
- Academic citations (dynamic fees, resistance curves, batch auctions).
- Industry precedents (CoW AMM, Algebra, Curve dynamic fees).
- Hackathon or benchmarking resources.

## Appendices
- A. Full derivations of resistance functions.
- B. Simulation scripts or pseudo-code.
- C. Glossary of terms (MEV, LVR, resistance curve, batch clearing).
- D. Change log for the whitepaper (revision history).

---

## Immediate Next Steps
- Populate sections 1-3 with existing math and research links from previous notes.
- Prepare diagrams: resistance curve plot vs. CPMM, batch timeline, account architecture.
- Draft evaluation plan (tools, datasets) to support claims in abstract.
- Decide on publishing format (Markdown + Pandoc, LaTeX, Notion export) and versioning cadence.
