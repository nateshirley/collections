use anchor_lang::prelude::*;
use anchor_spl::token;
// use spl_token_metadata::state::Creator;
declare_id!("G3am3SCcStwk8gCXfVEADAnjgBRDRHv6ap7QXjKjQstq");

const COLLECTION_SEED: &[u8] = b"collection";
const COLLECTION_ATTRIBUTION_SEED: &[u8] = b"c_attr";

#[program]
pub mod collections {
    use super::*;
    pub fn create_collection(
        ctx: Context<CreateCollection>,
        collection_bump: u8,
        name: String,
    ) -> ProgramResult {
        ctx.accounts.collection.bump = collection_bump;
        ctx.accounts.collection.name = name.to_verified_seed();
        Ok(())
    }

    pub fn pass_collector(ctx: Context<PassCollector>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PassCollector<'info> {
    collection: Account<'info, Collection>,
    collector: Signer<'info>,
    #[account(
        constraint = token_account.owner == collector.key(),
    )]
    token_account: Account<'info, token::TokenAccount>,
    //can probably replace collection account in a lot of them w/ the collection stored in a culture
    //or if working with a custom program u can just hardcode it
    #[account(
        seeds = [COLLECTION_ATTRIBUTION_SEED, token_account.mint.as_ref()],
        bump = collection_attribution.bump,
        constraint = collection_attribution.collection == collection.key()
    )]
    collection_attribution: Account<'info, CollectionAttribution>,
    /*
    this is all u need to verify a user holds an item in a collection
    - token account owned by signer
    - token account is a mint w/ attribution for the intended collection
    - it's just one extra account to pass (attribution)
    - i could probably even build the cultures program as one and just allow for separate ixns headers for each collection vs token
    - is there a way to do it without even passing the collection_attr account (i don't think so)
    - other option is to pass collection_attr in remaining accounts and then pull it to verify if the culture is run with a collection
    - i thnk maybe that would be best
    - if there is a remaining account, u just deserialize it into a collection_attr and verify it
    - otherwise idk
    */
}

#[derive(Accounts)]
#[instruction(collection_bump: u8, name: String)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    creator: Signer<'info>,
    #[account(
        init,
        seeds = [COLLECTION_SEED, name.clone().to_verified_seed().as_bytes()],
        bump = collection_bump,
        payer = creator,
        space = 100 //need to declare space if storing strings
    )]
    collection: Account<'info, Collection>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Collection {
    pub name: String,
    pub symbol: String,
    pub mint_authority: Option<Pubkey>,
    pub supply: u64,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub bump: u8,
}

#[account]
pub struct Culture {
    pub name: String,
    pub mint: Option<Pubkey>,
    pub collection: Option<Pubkey>,
}
//so then for every thing u pass in, u verify on a tree if it's mint or collection

//pda from ["c_attr", mint.key()]
//this is how u know a mint belongs to a collection
#[account]
#[derive(Default)]
pub struct CollectionAttribution {
    pub collection: Pubkey,
    pub bump: u8,
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In full percentage
    pub share: u8,
}

trait VerifiedSeed {
    fn to_verified_seed(self) -> String;
}
//need to add some error handling to the front end
impl VerifiedSeed for String {
    fn to_verified_seed(mut self) -> String {
        self.make_ascii_lowercase();
        self.retain(|c| !c.is_whitespace());
        self
    }
}

//can technically make this as big as u want in order to make the string longer

//mint authority and freeze authority at level of the collection
//mint authority for the actualy tokens in each NFT will be none after first mint.
//freeze authority will be retained as a program auth on each, that way i can add ixns later for
//the collection to freeze / delete accounts? if they want to do that. idk maybe just ignore for now. im gonna delete
//    pub freeze_authority: Option<Pubkey>,

//on the mint for each NFT, you're gonna have no mint authority
//but probably retain the collection program as freeze authority??

/*
#[repr(C)]
pub struct Mint {
    pub mint_authority: COption<Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: COption<Pubkey>,
}
*/
