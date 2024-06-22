use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("DxjMS82jPTzPGk7pmvt96Bvyn8XNNzjHH3J76DgEZ9Ff");

#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, text: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = text;
        Ok(())
    }

    pub fn add(ctx:Context<Add>, num1:i64, num2:i64 ) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space =100)]
    pub calculator: Account<'info, Calculator>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
 }

 #[derive(Accounts)]
 pub struct Add<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>

 }

#[account]
pub struct Calculator {
    greeting: String,
    result: i64

}

