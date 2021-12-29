use anchor_lang::prelude::*;
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
