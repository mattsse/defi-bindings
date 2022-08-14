pub use c_delegator_interface::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod c_delegator_interface {
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
    #[doc = "CDelegatorInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CDELEGATORINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldImplementation\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewImplementation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowResign\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"becomeImplementationData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setImplementation\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct CDelegatorInterface<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CDelegatorInterface<M> {
        fn clone(&self) -> Self {
            CDelegatorInterface(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CDelegatorInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CDelegatorInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CDelegatorInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CDelegatorInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CDELEGATORINTERFACE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `_setImplementation` (0x555bcc40) function"]
        pub fn set_implementation(
            &self,
            implementation: ethers::core::types::Address,
            allow_resign: bool,
            become_implementation_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [85, 91, 204, 64],
                    (implementation, allow_resign, become_implementation_data),
                )
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
        #[doc = "Gets the contract's `NewImplementation` event"]
        pub fn new_implementation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewImplementationFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewImplementationFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CDelegatorInterface<M>
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
    #[ethevent(name = "NewImplementation", abi = "NewImplementation(address,address)")]
    pub struct NewImplementationFilter {
        pub old_implementation: ethers::core::types::Address,
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setImplementation` function with signature `_setImplementation(address,bool,bytes)` and selector `[85, 91, 204, 64]`"]
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
        name = "_setImplementation",
        abi = "_setImplementation(address,bool,bytes)"
    )]
    pub struct SetImplementationCall {
        pub implementation: ethers::core::types::Address,
        pub allow_resign: bool,
        pub become_implementation_data: ethers::core::types::Bytes,
    }
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
    pub enum CDelegatorInterfaceCalls {
        SetImplementation(SetImplementationCall),
        Implementation(ImplementationCall),
    }
    impl ethers::core::abi::AbiDecode for CDelegatorInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDelegatorInterfaceCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDelegatorInterfaceCalls::Implementation(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CDelegatorInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CDelegatorInterfaceCalls::SetImplementation(element) => element.encode(),
                CDelegatorInterfaceCalls::Implementation(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CDelegatorInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CDelegatorInterfaceCalls::SetImplementation(element) => element.fmt(f),
                CDelegatorInterfaceCalls::Implementation(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SetImplementationCall> for CDelegatorInterfaceCalls {
        fn from(var: SetImplementationCall) -> Self {
            CDelegatorInterfaceCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for CDelegatorInterfaceCalls {
        fn from(var: ImplementationCall) -> Self {
            CDelegatorInterfaceCalls::Implementation(var)
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
