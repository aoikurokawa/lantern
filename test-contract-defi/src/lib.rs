use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    env, ext_contract, log, near_bindgen, setup_alloc, AccountId, Balance, Gas, PanicOnDefault,
    PromiseOrValue,
};

setup_alloc!();

const BASE_GAS: Gas = 5_000_000_000_000;
const PROMISE_CALL: Gas = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = BASE_GAS + PROMISE_CALL;

const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DeFi {
    fungible_token_account_id: AccountId,
}

#[ext_contract(ext_self)]
pub trait ValueReturnTrait {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128>;
}

trait ValueReturnTrait {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl DeFi {
    #[init]
    pub fn new(fungible_token_account_id: ValidAccountId) -> Self {
        assert!(!env::state_exists(), "Already initializing");
        Self {
            fungible_token_account_id: fungible_token_account_id.into(),
        }
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for DeFi {
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );
        log!(
            "in {} tokens from @w{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        match msg.as_str() {
            "take-my-money" => PromiseOrValue::Value(U128::from(0)),
            _ => {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                ext_self::value_please(
                    msg,
                    &account_id,
                    NO_DEPOSIT,
                    prepaid_gas - GAS_FOR_FT_ON_TRANSFER,
                )
                .into()
            }
        }
    }
}

#[near_bindgen]
impl ValueReturnTrait for DeFi {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128> {
        log!("in value_please, amount_to_return = {}", amount_to_return);
        let amount: Balance = amount_to_return.parse().expect("Not an integer");
        PromiseOrValue::Value(amount.into())
    }
}