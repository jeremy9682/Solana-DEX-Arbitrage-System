use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::{instruction::Instruction, program::invoke, pubkey::Pubkey};
use std::str::FromStr;

declare_id!("kjsutC39nSzGk4FiATA79QdrbiYkKzRRcyh9FQmC1DP");

#[program]
pub mod solana_dex_arbitrage {
    use super::*;

    // 主套利函数，执行跨DEX套利操作
    pub fn execute_arbitrage(
        ctx: Context<ExecuteArbitrage>,
        amount_in: u64,            // 输入金额
        min_profit: u64,           // 最小利润阈值
        max_slippage_bps: u64,     // 最大允许滑点（基点）
    ) -> Result<()> {
        require!(amount_in > 0, ProgramError::InvalidInput);
        require!(max_slippage_bps <= 10000, ProgramError::InvalidInput);

        let price_a = get_pool_price(&ctx.accounts.pool_a);
        let price_b = get_pool_price(&ctx.accounts.pool_b);

        let discrepancy = if price_a > price_b {
            price_a - price_b
        } else {
            price_b - price_a
        };

        require!(discrepancy >= min_profit as f64, ProgramError::InsufficientProfit);

        let sim_result = simulate_trade(&ctx, amount_in, max_slippage_bps)?;
        require!(sim_result.profit >= min_profit, ProgramError::InsufficientProfit);

        let trade_a_output = trade_on_dex_a(&ctx, amount_in, max_slippage_bps)?;
        let final_output = trade_on_dex_b(&ctx, trade_a_output, max_slippage_bps)?;

        let profit = final_output - amount_in;
        require!(profit >= min_profit, ProgramError::InsufficientProfit);

        msg!("Arbitrage executed successfully. Profit: {}", profit);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteArbitrage<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_a: Account<'info, Pool>,
    #[account(mut)]
    pub pool_b: Account<'info, Pool>,
    pub orca_pool: AccountInfo<'info>,
    pub orca_program: Program<'info, Token>,
    pub meteora_pool: AccountInfo<'info>,
    pub meteora_program: Program<'info, Token>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub reserves_a: u64,
    pub reserves_b: u64,
}

#[error_code]
pub enum ProgramError {
    #[msg("Profit below minimum threshold")]
    InsufficientProfit,
    #[msg("Exceeded allowed slippage")]
    SlippageExceeded,
    #[msg("Simulation of trade failed")]
    SimulationFailed,
    #[msg("Invalid input parameters")]
    InvalidInput,
}

fn get_pool_price(pool: &Account<Pool>) -> f64 {
    pool.reserves_a as f64 / pool.reserves_b as f64
}

fn simulate_trade(
    ctx: &Context<ExecuteArbitrage>,
    amount_in: u64,
    max_slippage_bps: u64,
) -> Result<SimulatedResult> {
    let trade_a_out = calculate_output(ctx.accounts.pool_a.reserves_a, ctx.accounts.pool_a.reserves_b, amount_in, max_slippage_bps)?;
    let trade_b_out = calculate_output(ctx.accounts.pool_b.reserves_a, ctx.accounts.pool_b.reserves_b, trade_a_out, max_slippage_bps)?;
    let profit = trade_b_out - amount_in;
    Ok(SimulatedResult { profit })
}

fn calculate_output(reserve_in: u64, reserve_out: u64, amount_in: u64, max_slippage_bps: u64) -> Result<u64> {
    let amount_out = (amount_in * reserve_out) / (reserve_in + amount_in);
    let ideal_rate = reserve_out as f64 / reserve_in as f64;
    let actual_rate = amount_out as f64 / amount_in as f64;
    let slippage = ((ideal_rate - actual_rate) / ideal_rate) * 10000.0;

    require!(slippage as u64 <= max_slippage_bps, ProgramError::SlippageExceeded);
    Ok(amount_out)
}

pub struct SimulatedResult {
    pub profit: u64,
}

fn trade_on_dex_a(ctx: &Context<ExecuteArbitrage>, amount_in: u64, max_slippage_bps: u64) -> Result<u64> {
    let min_amount_out = amount_in * (10000 - max_slippage_bps) / 10000;
    let orca_program_id = Pubkey::from_str("9WwqVbDVCZGxi4DDJypAhok6QeL4k8h78F6VZZ5EkVpS")?;

    let swap_instruction = Instruction {
        program_id: orca_program_id,
        accounts: vec![
            AccountMeta::new(ctx.accounts.orca_pool.key(), false),
            AccountMeta::new(ctx.accounts.user_token_a.key(), false),
            AccountMeta::new(ctx.accounts.user_token_b.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        ],
        data: vec![], // TODO: 根据Orca IDL定义
    };

    invoke(&swap_instruction, &[ctx.accounts.orca_pool.clone(), ctx.accounts.user_token_a.to_account_info(), ctx.accounts.user_token_b.to_account_info(), ctx.accounts.user.to_account_info(), ctx.accounts.token_program.to_account_info()])?;

    Ok(min_amount_out)
}

fn trade_on_dex_b(ctx: &Context<ExecuteArbitrage>, amount_in: u64, max_slippage_bps: u64) -> Result<u64> {
    let min_amount_out = amount_in * (10000 - max_slippage_bps) / 10000;
    let meteora_program_id = Pubkey::from_str("MeteoraProgramIDHere")?;

    let swap_instruction = Instruction {
        program_id: meteora_program_id,
        accounts: vec![
            AccountMeta::new(ctx.accounts.meteora_pool.key(), false),
            AccountMeta::new(ctx.accounts.user_token_b.key(), false),
            AccountMeta::new(ctx.accounts.user_token_a.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        ],
        data: vec![], // TODO: 根据Meteora IDL定义
    };

    invoke(&swap_instruction, &[ctx.accounts.meteora_pool.clone(), ctx.accounts.user_token_b.to_account_info(), ctx.accounts.user_token_a.to_account_info(), ctx.accounts.user.to_account_info(), ctx.accounts.token_program.to_account_info()])?;

    Ok(min_amount_out)
}
