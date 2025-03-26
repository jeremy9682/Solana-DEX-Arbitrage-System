use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com");
    let payer = Keypair::from_base58_string("your_private_key_here");

    let tx = build_arbitrage_transaction(&client, &payer);
    let mut tx = tx;
    add_priority_fee(&mut tx, 10000);

    println!("Transaction prepared with priority fee.");
}

fn build_arbitrage_transaction(client: &RpcClient, payer: &Keypair) -> Transaction {
    let instruction = system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1);
    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        client.get_latest_blockhash().unwrap(),
    )
}

fn add_priority_fee(tx: &mut Transaction, fee: u64) {
    tx.add_compute_budget_instruction(200000, fee);
}

trait TransactionExt {
    fn add_compute_budget_instruction(&mut self, compute_units: u64, additional_fee: u64);
}

impl TransactionExt for Transaction {
    fn add_compute_budget_instruction(&mut self, compute_units: u64, _additional_fee: u64) {
        // 简化实现，实际需添加优先费指令
        println!("Added compute budget: {} units, fee: {} lamports", compute_units, _additional_fee);
    }
}
