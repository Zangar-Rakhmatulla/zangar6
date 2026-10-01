use anchor_lang::prelude::*;

declare_id!("v7gfgu9G1QPmEvJtXah1Srbr9Xtg5Lrb4QfhTd1iRoW");

#[program]
pub mod anchor_profile {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String, bio: String) -> Result<()> {
        require!(name.len() <= 32, ProfileError::NameTooLong);
        require!(bio.len() <= 160, ProfileError::BioTooLong);

        let profile = &mut ctx.accounts.profile;
        profile.owner = ctx.accounts.owner.key();
        profile.name = name;
        profile.bio = bio;
        profile.bump = ctx.bumps.profile;
        msg!("Profile created");
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, name: String, bio: String) -> Result<()> {
        require!(name.len() <= 32, ProfileError::NameTooLong);
        require!(bio.len() <= 160, ProfileError::BioTooLong);

        let profile = &mut ctx.accounts.profile;
        profile.name = name;
        profile.bio = bio;
        msg!("Profile updated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", owner.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"profile", owner.key().as_ref()],
        bump = profile.bump,
        has_one = owner
    )]
    pub profile: Account<'info, Profile>,
}

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub owner: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(160)]
    pub bio: String,
    pub bump: u8,
}

#[error_code]
pub enum ProfileError {
    #[msg("Name is too long (max 32 bytes)")]
    NameTooLong,
    #[msg("Bio is too long (max 160 bytes)")]
    BioTooLong,
}