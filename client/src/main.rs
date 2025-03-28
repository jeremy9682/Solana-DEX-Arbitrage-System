use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::Instruction,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_program,
};
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    // RPC客户端连接devnet
    let client = RpcClient::new("https://api.devnet.solana.com");

    // 从本地安全加载私钥 (推荐使用环境变量或文件路径)
    let payer = read_keypair_file("~/.config/solana/id.json")?;

    // 构建套利交易
    let mut tx = build_arbitrage_transaction(&client, &payer)?;

    // 设置计算预算与优先费
    add_compute_budget(&mut tx, 200000, 10000);

    // 发送交易
    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Transaction submitted: {}", signature);

    Ok(())
}

// 构建套利合约交易
fn build_arbitrage_transaction(client: &RpcClient, payer: &Keypair) -> Result<Transaction, Box<dyn Error>> {
    // 定义你的套利合约程序 ID
    let arbitrage_program_id = Pubkey::from_str("kjsutC39nSzGk4FiATA79QdrbiYkKzRRcyh9FQmC1DP")?;

    // 构造调用套利合约的指令
    let instruction = Instruction {
        program_id: arbitrage_program_id,
        accounts: vec![
            // 必须的账户 (根据实际合约定义来填写)
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(system_program::id(), false),
            // 更多合约所需账户...
        ],
        data: arbitrage_instruction_data(1_000_000, 500_000, 50), // amount_in, min_profit, max_slippage_bps
    };

    // 获取最近的区块哈希
    let recent_blockhash = client.get_latest_blockhash()?;

    // 构建交易
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    Ok(tx)
}

// 生成套利调用的data字段（使用实际Anchor IDL编码）
fn arbitrage_instruction_data(amount_in: u64, min_profit: u64, max_slippage_bps: u64) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&min_profit.to_le_bytes());
    data.extend_from_slice(&max_slippage_bps.to_le_bytes());
    data
}

// 增加计算预算和优先费
fn add_compute_budget(tx: &mut Transaction, compute_units: u32, additional_fee: u64) {
    // 设置计算单元限制
    let compute_instruction = ComputeBudgetInstruction::set_compute_unit_limit(compute_units);

    // 设置额外优先费（单位：lamports）
    let fee_instruction = ComputeBudgetInstruction::set_compute_unit_price(additional_fee);

    // 将预算指令插入到交易头部
    tx.message.instructions.insert(0, compute_instruction);
    tx.message.instructions.insert(1, fee_instruction);
}
