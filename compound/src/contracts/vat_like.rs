pub use vat_like::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod vat_like {
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
    #[doc = "VatLike was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VATLIKE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dai\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hope\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct VatLike<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VatLike<M> {
        fn clone(&self) -> Self {
            VatLike(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VatLike<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VatLike<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VatLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VatLike<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VATLIKE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `dai` (0x6c25b346) function"]
        pub fn dai(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 37, 179, 70], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hope` (0xa3b22fc4) function"]
        pub fn hope(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 178, 47, 196], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for VatLike<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `dai` function with signature `dai(address)` and selector `[108, 37, 179, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dai", abi = "dai(address)")]
    pub struct DaiCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `hope` function with signature `hope(address)` and selector `[163, 178, 47, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hope", abi = "hope(address)")]
    pub struct HopeCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VatLikeCalls {
        Dai(DaiCall),
        Hope(HopeCall),
    }
    impl ethers::core::abi::AbiDecode for VatLikeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <DaiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VatLikeCalls::Dai(decoded));
            }
            if let Ok(decoded) = <HopeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VatLikeCalls::Hope(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VatLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VatLikeCalls::Dai(element) => element.encode(),
                VatLikeCalls::Hope(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VatLikeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VatLikeCalls::Dai(element) => element.fmt(f),
                VatLikeCalls::Hope(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DaiCall> for VatLikeCalls {
        fn from(var: DaiCall) -> Self {
            VatLikeCalls::Dai(var)
        }
    }
    impl ::std::convert::From<HopeCall> for VatLikeCalls {
        fn from(var: HopeCall) -> Self {
            VatLikeCalls::Hope(var)
        }
    }
    #[doc = "Container type for all return fields from the `dai` function with signature `dai(address)` and selector `[108, 37, 179, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DaiReturn(pub ethers::core::types::U256);
}
