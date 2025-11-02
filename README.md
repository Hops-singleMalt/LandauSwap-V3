# LandauSwap

> Capital-efficient AMM for Solana: zero-slip micro trades, bounded price impact for large orders.

## Mission
- Unlock higher capital efficiency for LPs by redesigning the slippage curve around resistance-driven pricing.
- Translate the math into production-grade Solana programs, SDKs, and dashboards so aggregators can route volume efficiently.

## Capital Efficiency Flywheel
- **Algorithmic edge**: non-convex resistance curves already reduce slip; next we extend them into adaptive fees and banded liquidity so every LP dollar drives more volume.
- **Depth operations**: ship tooling for position management, visual analytics, and auto-rebalancing that keeps pools deep with lean capital.
- **Fee routing & incentives**: recycle resistance-fee surplus back to LPs through dynamic splits, protocol fees, and tailored liquidity mining.
- **Risk discipline**: audits, real-time TVL/oracle monitoring, multi-sig upgrades, and flash-loan-aware parameter guards.
- **Integrator experience**: composable SDKs, pricing previews, subgraphs, and hooks into lending/leveraged protocols where efficient pools become base liquidity.

## Project Status
- MVP Anchor program scaffold lives under `landau_swap/programs/landau_swap`.
- Batch math + integration tests exist, but vault transfers and LP accounting are still TODO.
- `anchor build` currently blocked by missing crates.io network access.

## Immediate Next Steps
1. Formalise the resistance-curve roadmap: specify adaptive fee mechanics and design the LP revenue-sharing model.
2. Build the security runway: schedule audits, define on-chain monitoring, and codify the multi-sig + parameter guard playbook.
3. Productise liquidity ops: deliver maker tooling, visual dashboards, and SDK/subgraph packages so external protocols can plug in fast.

## Whitepaper
- Latest draft (PDF): [`docs/landauswap-resistant-amm.pdf`](docs/landauswap-resistant-amm.pdf)
- https://www.notion.so/Cypherpunk-guid-29f9a105db93809a976ee97b1a87cde5?source=copy_link

---


=======
## 中文简介
- 项目定位: 面向 Solana 的阻力式 AMM, 小额交易近乎零滑点, 大额单造成的瞬时 Delta y 价差被显著削弱.
- 当前进度: Anchor 程序骨架 + 测试已就绪, 真实资产转账/LP 记账/网络构建仍在开发中.
- 白皮书: [`docs/landauswap-resistant-amm.pdf`](docs/landauswap-resistant-amm.pdf)

### 资本效率飞轮
- **核心算法**: 在非凸阻力曲线基础上演化出自适应费率或区间流动性, 让 LP 单位资金创造更大成交量. 非凸优化已经改善滑点，可继续扩展到自适应费率或区间流动性管理，把算法优势转化为对 LP 的更高资本效率。
- **深度运营**: 提供仓位工具、可视化与自动再平衡, 配合早期激励, 让池子以更少资金保持深度.
- **费用与激励**: 将阻力溢价回流给 LP, 结合协议费、积分、流动性挖矿, 匹配不同池子的风险收益.
- **安全纪律**: 审计、TVL/预言机实时监控、多签升级, 并对闪电贷敏感参数设定动态阈值.
- **生态体验**: 输出可组合 SDK/子图与定价展示, 对接借贷/杠杆协议, 让高效池子成为底层流动性。

### 下一步
1. 梳理核心算法与动态费率落地方案, 并设计 LP 收益分配模型。
2. 制定安全审计与监控路线图, 明确多签与参数守护机制。
3. 落实做市工具与 SDK/子图, 降低外部协议集成门槛。
