use near_sdk::{ext_contract, PromiseResult, Gas, env};
use near_sdk::log;
use crate::*;
// const NFT_CONTRACT_ID :String = "abc.testnet".to_string();
// const ACCOUNTID : AccountId = NFT_CONTRACT_ID.try_into().unwrap();

const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = Gas(5_000_000_000_000);
const ONE_YOCTO: Balance = 100000000000000000000000;

trait NftCalls{
    fn metadata_insert(&mut self, token_type :String, metadata: TokenMetadata);
    fn nft_purchase_callback(&mut self, account_id: AccountId, number_of_tokens: U128, token_id : TokenId);
    fn did_promise_succeded() -> bool;
}
#[ext_contract(ext_nft)]


trait NftContractMethods{
    fn metadata_insert(&mut self, token_type :String, metadata: TokenMetadata);
    fn nft_custom_mint(&mut self, quantity: u128, receiver_id: AccountId); 

}
#[ext_contract(this_contract)]
trait Callbacks {
    fn nft_purchase_callback(account_id: AccountId);
}

#[near_bindgen]
impl NftCalls for Contract2{
    fn metadata_insert(&mut self, token_type: String, metadata: TokenMetadata){
        ext_nft::metadata_insert(token_type, metadata, self.nft_contract_id.clone(), NO_DEPOSIT, BASE_GAS);
    
    }



   #[payable]
fn nft_purchase_callback(&mut self, account_id: AccountId, number_of_tokens: U128,  token_id : TokenId) {
    let price = self.nft_price.get(&token_id).unwrap();
    // price = (number_of_tokens)*(price);
    assert!(env::attached_deposit() >= price.into());
    ext_nft::nft_custom_mint(number_of_tokens.try_into().unwrap()  , account_id, self.nft_contract_id.clone(),  ONE_YOCTO, BASE_GAS*4);
    }


    fn did_promise_succeded() -> bool {
        if env::promise_results_count() != 1 {
            log!("Expected a result on the callback");
            return false;
        }
        match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        }
    }
}