decloare_id!("22222222222222222222222222222222222222222222"); // Declares at what onchain address the program lives
// the declare_id!() macro assigns to your program its onchain address; a unique public key derived from the keypair in the project's target folder. That keypair signs and deploys the compiled .so binary containing all program logic and data.

// NOTE: we are using the 2222... because, of Blueshift's internal test suite. During production, Anchor will generate a fresh program Id for you when you run the standard build and deploy commands


#[program] // Marks the module that contains every instrunction entrypoint and business-logic function
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)] // Lists the accounts an instruction requires and enforces their constraints automatically
pub struct VaultAction<'info>{

}

#[error_code] // Defines custom, human-readable error types that make debugging clearer and faster
pub enum VaultError{

}


/////////////////////////////////////////////




#[derive(Accounts)]
pub struct VaultAction<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
 
  #[account(
    mut,
    seeds = [b"vault", signer.key().as_ref()],
    bump,
  )]
  pub vault: SystemAccount<'info>,
 
  pub system_program: Program<'info, System>,
}