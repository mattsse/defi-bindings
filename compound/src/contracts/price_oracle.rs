pub use price_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod price_oracle {
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
    #[doc = "PriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUnderlyingPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPriceOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct PriceOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PriceOracle<M> {
        fn clone(&self) -> Self {
            PriceOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PriceOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEORACLE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `getUnderlyingPrice` (0xfc57d4df) function"]
        pub fn get_underlying_price(
            &self,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 87, 212, 223], c_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPriceOracle` (0x66331bba) function"]
        pub fn is_price_oracle(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 51, 27, 186], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PriceOracle<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `[252, 87, 212, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUnderlyingPrice", abi = "getUnderlyingPrice(address)")]
    pub struct GetUnderlyingPriceCall {
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isPriceOracle` function with signature `isPriceOracle()` and selector `[102, 51, 27, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPriceOracle", abi = "isPriceOracle()")]
    pub struct IsPriceOracleCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleCalls {
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        IsPriceOracle(IsPriceOracleCall),
    }
    impl ethers::core::abi::AbiDecode for PriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetUnderlyingPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) =
                <IsPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::IsPriceOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceOracleCalls::GetUnderlyingPrice(element) => element.encode(),
                PriceOracleCalls::IsPriceOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleCalls::GetUnderlyingPrice(element) => element.fmt(f),
                PriceOracleCalls::IsPriceOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetUnderlyingPriceCall> for PriceOracleCalls {
        fn from(var: GetUnderlyingPriceCall) -> Self {
            PriceOracleCalls::GetUnderlyingPrice(var)
        }
    }
    impl ::std::convert::From<IsPriceOracleCall> for PriceOracleCalls {
        fn from(var: IsPriceOracleCall) -> Self {
            PriceOracleCalls::IsPriceOracle(var)
        }
    }
    #[doc = "Container type for all return fields from the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `[252, 87, 212, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUnderlyingPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isPriceOracle` function with signature `isPriceOracle()` and selector `[102, 51, 27, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsPriceOracleReturn(pub bool);
}
