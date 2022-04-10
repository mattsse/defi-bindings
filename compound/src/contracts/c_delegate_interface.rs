pub use cdelegateinterface_mod::*;
#[allow(clippy::too_many_arguments)]
mod cdelegateinterface_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_becomeImplementation\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_resignImplementation\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CDELEGATEINTERFACE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct CDelegateInterface<M>(ethers::contract::Contract<M>);
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
    impl<'a, M: ethers::providers::Middleware> CDelegateInterface<M> {
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
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                CDELEGATEINTERFACE_ABI.clone(),
                CDELEGATEINTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    #[doc = "Container type for all input parameters for the `_becomeImplementation`function with signature `_becomeImplementation(bytes)` and selector `[86, 230, 119, 40]`"]
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
    #[doc = "Container type for all input parameters for the `_resignImplementation`function with signature `_resignImplementation()` and selector `[21, 58, 181, 5]`"]
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
    #[doc = "Container type for all input parameters for the `implementation`function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
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
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
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
}
