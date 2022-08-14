pub use i_initializable_a_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_initializable_a_token {
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
    #[doc = "IInitializableAToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IINITIALIZABLEATOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"aTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"aTokenName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"aTokenSymbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"aTokenDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IInitializableAToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IInitializableAToken<M> {
        fn clone(&self) -> Self {
            IInitializableAToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IInitializableAToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IInitializableAToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IInitializableAToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IInitializableAToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IINITIALIZABLEATOKEN_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `initialize` (0x183fb413) function"]
        pub fn initialize(
            &self,
            pool: ethers::core::types::Address,
            treasury: ethers::core::types::Address,
            underlying_asset: ethers::core::types::Address,
            incentives_controller: ethers::core::types::Address,
            a_token_decimals: u8,
            a_token_name: String,
            a_token_symbol: String,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [24, 63, 180, 19],
                    (
                        pool,
                        treasury,
                        underlying_asset,
                        incentives_controller,
                        a_token_decimals,
                        a_token_name,
                        a_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IInitializableAToken<M>
    {
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
        name = "Initialized",
        abi = "Initialized(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
        pub treasury: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: String,
        pub a_token_symbol: String,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,uint8,string,string,bytes)` and selector `[24, 63, 180, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub pool: ethers::core::types::Address,
        pub treasury: ethers::core::types::Address,
        pub underlying_asset: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: String,
        pub a_token_symbol: String,
        pub params: ethers::core::types::Bytes,
    }
}
