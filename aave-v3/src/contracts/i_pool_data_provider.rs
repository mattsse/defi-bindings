pub use i_pool_data_provider::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_pool_data_provider {
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
    #[doc = "IPoolDataProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLDATAPROVIDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getATokenTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"unbacked\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"accruedToTreasuryScaled\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalAToken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalStableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalVariableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"averageStableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableBorrowIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"lastUpdateTimestamp\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalDebt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IPoolDataProvider<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPoolDataProvider<M> {
        fn clone(&self) -> Self {
            IPoolDataProvider(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPoolDataProvider<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPoolDataProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPoolDataProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPoolDataProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOLDATAPROVIDER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getATokenTotalSupply` (0x51460e25) function"]
        pub fn get_a_token_total_supply(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([81, 70, 14, 37], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveData` (0x35ea6a75) function"]
        pub fn get_reserve_data(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u64,
            ),
        > {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalDebt` (0x4d44ac4f) function"]
        pub fn get_total_debt(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 68, 172, 79], asset)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPoolDataProvider<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getATokenTotalSupply` function with signature `getATokenTotalSupply(address)` and selector `[81, 70, 14, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getATokenTotalSupply", abi = "getATokenTotalSupply(address)")]
    pub struct GetATokenTotalSupplyCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReserveData", abi = "getReserveData(address)")]
    pub struct GetReserveDataCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getTotalDebt` function with signature `getTotalDebt(address)` and selector `[77, 68, 172, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTotalDebt", abi = "getTotalDebt(address)")]
    pub struct GetTotalDebtCall {
        pub asset: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolDataProviderCalls {
        GetATokenTotalSupply(GetATokenTotalSupplyCall),
        GetReserveData(GetReserveDataCall),
        GetTotalDebt(GetTotalDebtCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolDataProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetATokenTotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolDataProviderCalls::GetATokenTotalSupply(decoded));
            }
            if let Ok(decoded) =
                <GetReserveDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolDataProviderCalls::GetReserveData(decoded));
            }
            if let Ok(decoded) =
                <GetTotalDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolDataProviderCalls::GetTotalDebt(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolDataProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolDataProviderCalls::GetATokenTotalSupply(element) => element.encode(),
                IPoolDataProviderCalls::GetReserveData(element) => element.encode(),
                IPoolDataProviderCalls::GetTotalDebt(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolDataProviderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolDataProviderCalls::GetATokenTotalSupply(element) => element.fmt(f),
                IPoolDataProviderCalls::GetReserveData(element) => element.fmt(f),
                IPoolDataProviderCalls::GetTotalDebt(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetATokenTotalSupplyCall> for IPoolDataProviderCalls {
        fn from(var: GetATokenTotalSupplyCall) -> Self {
            IPoolDataProviderCalls::GetATokenTotalSupply(var)
        }
    }
    impl ::std::convert::From<GetReserveDataCall> for IPoolDataProviderCalls {
        fn from(var: GetReserveDataCall) -> Self {
            IPoolDataProviderCalls::GetReserveData(var)
        }
    }
    impl ::std::convert::From<GetTotalDebtCall> for IPoolDataProviderCalls {
        fn from(var: GetTotalDebtCall) -> Self {
            IPoolDataProviderCalls::GetTotalDebt(var)
        }
    }
    #[doc = "Container type for all return fields from the `getATokenTotalSupply` function with signature `getATokenTotalSupply(address)` and selector `[81, 70, 14, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetATokenTotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveDataReturn {
        pub unbacked: ethers::core::types::U256,
        pub accrued_to_treasury_scaled: ethers::core::types::U256,
        pub total_a_token: ethers::core::types::U256,
        pub total_stable_debt: ethers::core::types::U256,
        pub total_variable_debt: ethers::core::types::U256,
        pub liquidity_rate: ethers::core::types::U256,
        pub variable_borrow_rate: ethers::core::types::U256,
        pub stable_borrow_rate: ethers::core::types::U256,
        pub average_stable_borrow_rate: ethers::core::types::U256,
        pub liquidity_index: ethers::core::types::U256,
        pub variable_borrow_index: ethers::core::types::U256,
        pub last_update_timestamp: u64,
    }
    #[doc = "Container type for all return fields from the `getTotalDebt` function with signature `getTotalDebt(address)` and selector `[77, 68, 172, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTotalDebtReturn(pub ethers::core::types::U256);
}
