use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{state::*};

use crate::constants::nft::whirlpool_nft_update_auth::ID as WP_NFT_UPDATE_AUTH;

#[derive(Accounts)]
pub struct OpenPositionWithMetadata<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    /// CHECK: safe, the account that will be the owner of the position can be arbitrary
    pub owner: UncheckedAccount<'info>,

    #[account(init,
      payer = funder,
      space = Position::LEN,
      seeds = [b"position".as_ref(), position_mint.key().as_ref()],
      bump,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(init,
        payer = funder,
        mint::authority = whirlpool,
        mint::decimals = 0,
    )]
    pub position_mint: Account<'info, Mint>,

    /// CHECK: checked via the Metadata CPI call
    /// https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/utils.rs#L873
    #[account(mut)]
    pub position_metadata_account: UncheckedAccount<'info>,

    #[account(init,
      payer = funder,
      associated_token::mint = position_mint,
      associated_token::authority = owner,
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: checked via account constraints
    pub metadata_program: UncheckedAccount<'info>,

    /// CHECK: checked via account constraints
    #[account(address = WP_NFT_UPDATE_AUTH)]
    pub metadata_update_auth: UncheckedAccount<'info>,
}

