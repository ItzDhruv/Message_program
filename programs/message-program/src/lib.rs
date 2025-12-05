use anchor_lang::prelude::*;

declare_id!("FZNeSF4KYFhnZh8E6zstzfs3WLdre3Cc1tSQbWLF6ZFq");

#[program]
pub mod message_program {
    use super::*;

    /// Initialize a new message account with an initial message.
    pub fn initialize(ctx: Context<Initialize>, message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;

        // Save who is allowed to update this message
        message_account.authority = ctx.accounts.authority.key();

        // Save the initial message
        message_account.message = message;

        Ok(())
    }

    /// Update the stored message in an existing message account.
    pub fn update_message(ctx: Context<UpdateMessage>, new_message: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;

        // Only the original authority is allowed to update the message
        require_keys_eq!(
            message_account.authority,
            ctx.accounts.authority.key(),
            MessageError::Unauthorized
        );

        message_account.message = new_message;

        Ok(())
    }
}

/// Accounts required for the `initialize` instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// New account that will store the message.
    /// `space` = 8 (account discriminator)
    ///         + 32 (Pubkey: authority)
    ///         + 4 + 256 (String: 4 bytes length prefix + up to 256 chars)
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 256,
    )]
    pub message_account: Account<'info, MessageAccount>,

    /// The user who pays for the account creation and becomes the authority.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program used to create the account.
    pub system_program: Program<'info, System>,
}

/// Accounts required for the `update_message` instruction.
#[derive(Accounts)]
pub struct UpdateMessage<'info> {
    /// The existing message account. It must be mutable.
    #[account(mut)]
    pub message_account: Account<'info, MessageAccount>,

    /// The authority that is allowed to update the message.
    pub authority: Signer<'info>,
}

/// Data stored in the on-chain message account.
#[account]
pub struct MessageAccount {
    /// Who is allowed to update this account.
    pub authority: Pubkey,

    /// The actual text message. (Max ~256 bytes due to space above)
    pub message: String,
}
// HbRjaEfZt5Heb9aBptYhWtmP4tUc4kQQUq1xQqmpUCdu
// DDXF5xhDoEbdijsQQQNoARvAE5Dx5Dfo19He6WxcMrsP

/// Custom errors.
#[error_code]
pub enum MessageError {
    #[msg("You are not authorized to update this message.")]
    Unauthorized,
}
