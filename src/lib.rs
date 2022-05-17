// use std::collections::HashMap;
// use std::thread::AccessError;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use crate::metadata::*;
use crate::nft_calls::*;

mod metadata;
mod nft_calls;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct Contract2{
    pub nft_price : LookupMap<TokenId, U128>,
    
    pub nft_token_supply: LookupMap<TokenId, U128>,

    pub minted_nft_tokens:  LookupMap<TokenId, U128>,

    pub owner : AccountId,

    pub nft_contract_id: AccountId,
}
#[derive(BorshSerialize)]
pub enum StorageKey{
    NftPrice,
    NftTokenSupply,
    MintedNftTokens,

}
#[near_bindgen]
impl Contract2{


    #[init]
    pub fn new(owner_id: AccountId, nft_contract_id : AccountId) -> Self {
        let this = Self{
            owner:owner_id,
            nft_price: LookupMap::new(StorageKey::NftPrice.try_to_vec().unwrap()),
            nft_token_supply: LookupMap::new(StorageKey::NftTokenSupply.try_to_vec().unwrap()),
            minted_nft_tokens: LookupMap:: new(StorageKey::MintedNftTokens.try_to_vec().unwrap()),
            nft_contract_id 
        
        };
        this
    }

    pub fn total_supply_insert(&mut self, token_id: TokenId, supply:U128){
        let _owner = &self.owner;
        assert_eq!(_owner.to_string() , env::current_account_id().to_string(), "unathorized");
        self.nft_token_supply.insert(&token_id, &supply);

    }
    pub fn nft_price_insert(&mut self, token_id: TokenId, price:U128){
        let _owner = &self.owner;
        assert_eq!(_owner.to_string() , env::current_account_id().to_string(), "unathorized");
        self.nft_price.insert(&token_id, &price);
    }
    pub fn nft_get_price(self, token_id: TokenId) -> Option<U128>{
        self.nft_price.get(&token_id)
    }

}
// #[near_bindgen]
// impl NftCalls for Contract2{
//     fn metadata_insert(&mut self, token_type: String, metadata: TokenMetadata){
//         ext_nft::metadata_insert(token_type, metadata, self.nft_contract_id.clone(), NO_DEPOSIT, BASE_GAS);
    
//     }
//     fn nft_purchase_callback(account_id: AccountId) {
        
        
//     }
//     fn did_promise_succeded() -> bool {
//         if env::promise_results_count() != 1 {
//             log!("Expected a result on the callback");
//             return false;
//         }
//         match env::promise_result(0) {
//             PromiseResult::Successful(_) => true,
//             _ => false,
//         }
//     }
// }
