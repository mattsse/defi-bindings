pub use interest_rate_model::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod interest_rate_model {
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
    #[doc = "InterestRateModel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static INTERESTRATEMODEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInterestRateModel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct InterestRateModel<M>(ethers::contract::Contract<M>);
    impl<M> Clone for InterestRateModel<M> {
        fn clone(&self) -> Self {
            InterestRateModel(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for InterestRateModel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for InterestRateModel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(InterestRateModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> InterestRateModel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), INTERESTRATEMODEL_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getBorrowRate` (0x15f24053) function"]
        pub fn get_borrow_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 242, 64, 83], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyRate` (0xb8168816) function"]
        pub fn get_supply_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
            reserve_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [184, 22, 136, 22],
                    (cash, borrows, reserves, reserve_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInterestRateModel` (0x2191f92a) function"]
        pub fn is_interest_rate_model(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([33, 145, 249, 42], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for InterestRateModel<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowRate", abi = "getBorrowRate(uint256,uint256,uint256)")]
    pub struct GetBorrowRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
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
        name = "getSupplyRate",
        abi = "getSupplyRate(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetSupplyRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
        pub reserve_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isInterestRateModel", abi = "isInterestRateModel()")]
    pub struct IsInterestRateModelCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum InterestRateModelCalls {
        GetBorrowRate(GetBorrowRateCall),
        GetSupplyRate(GetSupplyRateCall),
        IsInterestRateModel(IsInterestRateModelCall),
    }
    impl ethers::core::abi::AbiDecode for InterestRateModelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::GetBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::GetSupplyRate(decoded));
            }
            if let Ok(decoded) =
                <IsInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::IsInterestRateModel(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for InterestRateModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                InterestRateModelCalls::GetBorrowRate(element) => element.encode(),
                InterestRateModelCalls::GetSupplyRate(element) => element.encode(),
                InterestRateModelCalls::IsInterestRateModel(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for InterestRateModelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                InterestRateModelCalls::GetBorrowRate(element) => element.fmt(f),
                InterestRateModelCalls::GetSupplyRate(element) => element.fmt(f),
                InterestRateModelCalls::IsInterestRateModel(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetBorrowRateCall> for InterestRateModelCalls {
        fn from(var: GetBorrowRateCall) -> Self {
            InterestRateModelCalls::GetBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyRateCall> for InterestRateModelCalls {
        fn from(var: GetSupplyRateCall) -> Self {
            InterestRateModelCalls::GetSupplyRate(var)
        }
    }
    impl ::std::convert::From<IsInterestRateModelCall> for InterestRateModelCalls {
        fn from(var: IsInterestRateModelCall) -> Self {
            InterestRateModelCalls::IsInterestRateModel(var)
        }
    }
    #[doc = "Container type for all return fields from the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBorrowRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsInterestRateModelReturn(pub bool);
}
