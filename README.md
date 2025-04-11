# Solana DEX 套利系统

这是一个基于 **Solana 区块链** 的去中心化交易所（DEX）套利系统，目标是在 Raydium 和 Orca 等 DEX 上捕捉价格差异并执行套利交易。本项目结合了链上智能合约（使用 Rust 和 Anchor 开发）、链下实时监控（Python）和交易提交客户端（Rust），并利用 **Jito MEV** 基础设施优化交易优先级。

---

## 🧭 项目简介

### 🎯 目标
通过自动化系统在 Solana 链上 DEX 之间发现并执行套利机会。

### 🔧 核心功能
- ✅ 使用 **Jito MEV Bundle** 提交交易捆绑并支付小费，提升交易优先级
- ✅ 设置滑点保护和最小盈利阈值，确保交易稳健性
- ✅ 利用 Solana 的预模拟功能验证交易盈利性
- ✅ 实时监控链上价格数据，快速触发套利
- ✅ 支持优先费机制，提高交易打包概率

### 🧱 技术栈
- **链上合约**: Rust + Anchor
- **链下监控**: Python（基于 [solana-py](https://github.com/michaelhly/solana-py)）
- **交易客户端**: Rust（集成 Jito SDK）

---

## 📁 项目结构
solana-dex-arbitrage/ ├── Anchor.toml # Anchor 配置文件，指定网络和钱包 ├── Cargo.toml # Rust 项目依赖和配置

├── programs/ # 链上智能合约目录 │ └── solana-dex-arbitrage/ │ ├── Cargo.toml # 合约依赖配置 │ └── src/ │ └── lib.rs # 智能合约主逻辑（套利执行、滑点控制等）

├── src/ # Rust 客户端代码 │ ├── bin/ │ │ └── client.rs # 链下客户端：构造交易、Jito Bundle、优先费提交 │ └── lib.rs # 客户端共享逻辑（可选）

├── scripts/ # 链下监控脚本 │ └── monitor.py # Python 监控脚本：实时监听价格变化

├── tests/ # 测试目录 │ └── arbitrage_test.js # Anchor 测试脚本（JavaScript）

├── config/ # 环境配置 │ ├── devnet.json # devnet 环境配置（RPC、钱包等） │ └── mainnet.json # mainnet 环境配置

├── README.md # 项目说明文档（本文件） └── .gitignore # Git 忽略文件（如 target/、node_modules/ 等）

---

## ⚙️ 环境搭建

### 🧰 依赖安装

#### 安装 Rust 和 Solana CLI

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

安装 Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

安装 Python 依赖
pip install solana websockets
🚀 项目初始化
克隆仓库
git clone https://github.com/your-username/solana-dex-arbitrage.git
cd solana-dex-arbitrage
构建项目

anchor build
🏃 运行指南
🔁 部署链上合约
部署到 devnet：

anchor deploy --provider.cluster devnet
切换到 mainnet：


anchor deploy --provider.cluster mainnet
🧠 启动链下监控

python scripts/monitor.py
📦 运行链下客户端（提交 Jito 交易）

cargo run --bin client
✅ 执行测试

anchor test
🌟 功能亮点
💰 Jito MEV Bundle: 将套利交易打包提交并支付小费，确保优先执行

📈 优先费支持: 在交易中附加优先费，提高打包概率

🛡️ 滑点控制: 设置最大滑点和最小盈利阈值，避免亏损

🔍 交易模拟: 在发送交易前模拟执行，确认盈利性

📡 实时监控: 通过 WebSocket 订阅链上数据，快速发现套利机会

🧾 配置说明
config/devnet.json: devnet 环境配置，包含 RPC 和 WebSocket 端点

config/mainnet.json: mainnet 环境配置，需更新为实际生产参数

钱包配置: 在 Anchor.toml 中指定钱包路径，或通过命令行传入

⚠️ 注意事项
🔐 安全性: 在主网部署前，建议对智能合约进行全面审计

⚡ 基础设施: 推荐使用高性能 RPC（如 Helius）或自托管 Solana 节点以降低延迟

🎯 竞争优化: 根据网络状况动态调整 Jito 小费和优先费，提升成功率

🤝 贡献指南
欢迎提交 Issue 或 Pull Request！请遵循以下步骤：

Fork 本仓库

创建新分支：

git checkout -b feature/your-feature
提交更改：

git commit -m "Add your feature"
推送分支：


git push origin feature/your-feature
创建 Pull Request

📄 许可证
本项目采用 MIT 许可证，详情见 LICENSE 文件。
