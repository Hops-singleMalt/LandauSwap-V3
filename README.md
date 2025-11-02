# LandauSwap

> Resistant AMM prototype for Solana: zero-slip micro trades, resistance-fee protection for whales.

## Project Status
- MVP Anchor program scaffold lives under `landau_swap/programs/landau_swap`.
- Batch math + integration tests exist, but vault transfers and LP accounting are still TODO.
- `anchor build` currently blocked by missing crates.io network access.

## Whitepaper
- Latest draft (PDF): [`docs/landauswap-resistant-amm.pdf`](docs/landauswap-resistant-amm.pdf)

---

## 中文简介 CN README
- 项目定位: 面向 Solana 的阻力式 AMM, 小额交易近乎零滑点, 大额单需要向 LP 支付高额阻力费.
- 当前进度: Anchor 程序骨架 + 测试已就绪, 真实资产转账/LP 记账/网络构建仍在开发中.
- 白皮书: [`docs/landauswap-resistant-amm.pdf`](docs/landauswap-resistant-amm.pdf)
