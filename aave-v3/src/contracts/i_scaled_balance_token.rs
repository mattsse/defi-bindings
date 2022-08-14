pub use i_scaled_balance_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_scaled_balance_token {
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
    #[doc = "IScaledBalanceToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISCALEDBALANCETOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPreviousIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getScaledUserBalanceAndSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IScaledBalanceToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IScaledBalanceToken<M> {
        fn clone(&self) -> Self {
            IScaledBalanceToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IScaledBalanceToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IScaledBalanceToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IScaledBalanceToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IScaledBalanceToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISCALEDBALANCETOKEN_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getPreviousIndex` (0xe0753986) function"]
        pub fn get_previous_index(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 117, 57, 134], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getScaledUserBalanceAndSupply` (0x0afbcdc9) function"]
        pub fn get_scaled_user_balance_and_supply(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 251, 205, 201], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledBalanceOf` (0x1da24f3e) function"]
        pub fn scaled_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 162, 79, 62], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledTotalSupply` (0xb1bf962d) function"]
        pub fn scaled_total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 191, 150, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IScaledBalanceTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IScaledBalanceToken<M>
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IScaledBalanceTokenEvents {
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
    }
    impl ethers::contract::EthLogDecode for IScaledBalanceTokenEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IScaledBalanceTokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IScaledBalanceTokenEvents::MintFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IScaledBalanceTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IScaledBalanceTokenEvents::BurnFilter(element) => element.fmt(f),
                IScaledBalanceTokenEvents::MintFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getPreviousIndex` function with signature `getPreviousIndex(address)` and selector `[224, 117, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPreviousIndex", abi = "getPreviousIndex(address)")]
    pub struct GetPreviousIndexCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getScaledUserBalanceAndSupply` function with signature `getScaledUserBalanceAndSupply(address)` and selector `[10, 251, 205, 201]`"]
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
        name = "getScaledUserBalanceAndSupply",
        abi = "getScaledUserBalanceAndSupply(address)"
    )]
    pub struct GetScaledUserBalanceAndSupplyCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `scaledBalanceOf` function with signature `scaledBalanceOf(address)` and selector `[29, 162, 79, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledBalanceOf", abi = "scaledBalanceOf(address)")]
    pub struct ScaledBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `scaledTotalSupply` function with signature `scaledTotalSupply()` and selector `[177, 191, 150, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledTotalSupply", abi = "scaledTotalSupply()")]
    pub struct ScaledTotalSupplyCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IScaledBalanceTokenCalls {
        GetPreviousIndex(GetPreviousIndexCall),
        GetScaledUserBalanceAndSupply(GetScaledUserBalanceAndSupplyCall),
        ScaledBalanceOf(ScaledBalanceOfCall),
        ScaledTotalSupply(ScaledTotalSupplyCall),
    }
    impl ethers::core::abi::AbiDecode for IScaledBalanceTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetPreviousIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IScaledBalanceTokenCalls::GetPreviousIndex(decoded));
            }
            if let Ok(decoded) =
                <GetScaledUserBalanceAndSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IScaledBalanceTokenCalls::GetScaledUserBalanceAndSupply(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ScaledBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IScaledBalanceTokenCalls::ScaledBalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ScaledTotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IScaledBalanceTokenCalls::ScaledTotalSupply(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IScaledBalanceTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IScaledBalanceTokenCalls::GetPreviousIndex(element) => element.encode(),
                IScaledBalanceTokenCalls::GetScaledUserBalanceAndSupply(element) => {
                    element.encode()
                }
                IScaledBalanceTokenCalls::ScaledBalanceOf(element) => element.encode(),
                IScaledBalanceTokenCalls::ScaledTotalSupply(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IScaledBalanceTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IScaledBalanceTokenCalls::GetPreviousIndex(element) => element.fmt(f),
                IScaledBalanceTokenCalls::GetScaledUserBalanceAndSupply(element) => element.fmt(f),
                IScaledBalanceTokenCalls::ScaledBalanceOf(element) => element.fmt(f),
                IScaledBalanceTokenCalls::ScaledTotalSupply(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetPreviousIndexCall> for IScaledBalanceTokenCalls {
        fn from(var: GetPreviousIndexCall) -> Self {
            IScaledBalanceTokenCalls::GetPreviousIndex(var)
        }
    }
    impl ::std::convert::From<GetScaledUserBalanceAndSupplyCall> for IScaledBalanceTokenCalls {
        fn from(var: GetScaledUserBalanceAndSupplyCall) -> Self {
            IScaledBalanceTokenCalls::GetScaledUserBalanceAndSupply(var)
        }
    }
    impl ::std::convert::From<ScaledBalanceOfCall> for IScaledBalanceTokenCalls {
        fn from(var: ScaledBalanceOfCall) -> Self {
            IScaledBalanceTokenCalls::ScaledBalanceOf(var)
        }
    }
    impl ::std::convert::From<ScaledTotalSupplyCall> for IScaledBalanceTokenCalls {
        fn from(var: ScaledTotalSupplyCall) -> Self {
            IScaledBalanceTokenCalls::ScaledTotalSupply(var)
        }
    }
    #[doc = "Container type for all return fields from the `getPreviousIndex` function with signature `getPreviousIndex(address)` and selector `[224, 117, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPreviousIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getScaledUserBalanceAndSupply` function with signature `getScaledUserBalanceAndSupply(address)` and selector `[10, 251, 205, 201]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetScaledUserBalanceAndSupplyReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `scaledBalanceOf` function with signature `scaledBalanceOf(address)` and selector `[29, 162, 79, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ScaledBalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `scaledTotalSupply` function with signature `scaledTotalSupply()` and selector `[177, 191, 150, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ScaledTotalSupplyReturn(pub ethers::core::types::U256);
}
