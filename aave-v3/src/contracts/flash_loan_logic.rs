pub use flash_loan_logic::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod flash_loan_logic {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "FlashLoanLogic was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FLASHLOANLOGIC_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"initiator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"enum DataTypes.InterestRateMode\",\"name\":\"interestRateMode\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FlashLoan\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    pub struct FlashLoanLogic<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FlashLoanLogic<M> {
        fn clone(&self) -> Self {
            FlashLoanLogic(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FlashLoanLogic<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FlashLoanLogic<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FlashLoanLogic))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FlashLoanLogic<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), FLASHLOANLOGIC_ABI.clone(), client)
                .into()
        }
        #[doc = "Gets the contract's `FlashLoan` event"]
        pub fn flash_loan_filter(&self) -> ethers::contract::builders::Event<M, FlashLoanFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, FlashLoanFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for FlashLoanLogic<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "FlashLoan",
        abi = "FlashLoan(address,address,address,uint256,uint8,uint256,uint16)"
    )]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub initiator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: u8,
        pub premium: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
    }
}
