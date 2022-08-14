pub use c_delegate_interface::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod c_delegate_interface {
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
    #[doc = "CDelegateInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CDELEGATEINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_becomeImplementation\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_resignImplementation\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct CDelegateInterface<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CDelegateInterface<M> {
        fn clone(&self) -> Self {
            CDelegateInterface(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CDelegateInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CDelegateInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CDelegateInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CDelegateInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CDELEGATEINTERFACE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `_becomeImplementation` (0x56e67728) function"]
        pub fn become_implementation(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 230, 119, 40], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_resignImplementation` (0x153ab505) function"]
        pub fn resign_implementation(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 58, 181, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CDelegateInterface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `_becomeImplementation` function with signature `_becomeImplementation(bytes)` and selector `[86, 230, 119, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_becomeImplementation", abi = "_becomeImplementation(bytes)")]
    pub struct BecomeImplementationCall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `_resignImplementation` function with signature `_resignImplementation()` and selector `[21, 58, 181, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_resignImplementation", abi = "_resignImplementation()")]
    pub struct ResignImplementationCall;
    #[doc = "Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CDelegateInterfaceCalls {
        BecomeImplementation(BecomeImplementationCall),
        ResignImplementation(ResignImplementationCall),
        Implementation(ImplementationCall),
    }
    impl ethers::core::abi::AbiDecode for CDelegateInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BecomeImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDelegateInterfaceCalls::BecomeImplementation(decoded));
            }
            if let Ok(decoded) =
                <ResignImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDelegateInterfaceCalls::ResignImplementation(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDelegateInterfaceCalls::Implementation(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CDelegateInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CDelegateInterfaceCalls::BecomeImplementation(element) => element.encode(),
                CDelegateInterfaceCalls::ResignImplementation(element) => element.encode(),
                CDelegateInterfaceCalls::Implementation(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CDelegateInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CDelegateInterfaceCalls::BecomeImplementation(element) => element.fmt(f),
                CDelegateInterfaceCalls::ResignImplementation(element) => element.fmt(f),
                CDelegateInterfaceCalls::Implementation(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BecomeImplementationCall> for CDelegateInterfaceCalls {
        fn from(var: BecomeImplementationCall) -> Self {
            CDelegateInterfaceCalls::BecomeImplementation(var)
        }
    }
    impl ::std::convert::From<ResignImplementationCall> for CDelegateInterfaceCalls {
        fn from(var: ResignImplementationCall) -> Self {
            CDelegateInterfaceCalls::ResignImplementation(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for CDelegateInterfaceCalls {
        fn from(var: ImplementationCall) -> Self {
            CDelegateInterfaceCalls::Implementation(var)
        }
    }
    #[doc = "Container type for all return fields from the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ImplementationReturn(pub ethers::core::types::Address);
}
