pub use imapleproxied_mod::*;
#[allow(clippy::too_many_arguments)]
mod imapleproxied_mod {
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
    #[doc = "IMapleProxied was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IMAPLEPROXIED_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IMAPLEPROXIED_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IMapleProxied<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IMapleProxied<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IMapleProxied<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMapleProxied))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IMapleProxied<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMAPLEPROXIED_ABI.clone(), client)
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
                IMAPLEPROXIED_ABI.clone(),
                IMAPLEPROXIED_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
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
        #[doc = "Calls the contract's `migrate` (0xc3fbb6fd) function"]
        pub fn migrate(
            &self,
            migrator: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 251, 182, 253], (migrator, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setImplementation` (0xd784d426) function"]
        pub fn set_implementation(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 132, 212, 38], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgrade` (0x3b99bcee) function"]
        pub fn upgrade(
            &self,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 153, 188, 238], (to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IMapleProxied<M> {
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(uint256,bytes)")]
    pub struct UpgradedFilter {
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
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
    #[doc = "Container type for all input parameters for the `migrate`function with signature `migrate(address,bytes)` and selector `[195, 251, 182, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "migrate", abi = "migrate(address,bytes)")]
    pub struct MigrateCall {
        pub migrator: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setImplementation`function with signature `setImplementation(address)` and selector `[215, 132, 212, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setImplementation", abi = "setImplementation(address)")]
    pub struct SetImplementationCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgrade`function with signature `upgrade(uint256,bytes)` and selector `[59, 153, 188, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(uint256,bytes)")]
    pub struct UpgradeCall {
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleProxiedCalls {
        Factory(FactoryCall),
        Implementation(ImplementationCall),
        Migrate(MigrateCall),
        SetImplementation(SetImplementationCall),
        Upgrade(UpgradeCall),
    }
    impl ethers::core::abi::AbiDecode for IMapleProxiedCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxiedCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxiedCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxiedCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxiedCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxiedCalls::Upgrade(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMapleProxiedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMapleProxiedCalls::Factory(element) => element.encode(),
                IMapleProxiedCalls::Implementation(element) => element.encode(),
                IMapleProxiedCalls::Migrate(element) => element.encode(),
                IMapleProxiedCalls::SetImplementation(element) => element.encode(),
                IMapleProxiedCalls::Upgrade(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMapleProxiedCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleProxiedCalls::Factory(element) => element.fmt(f),
                IMapleProxiedCalls::Implementation(element) => element.fmt(f),
                IMapleProxiedCalls::Migrate(element) => element.fmt(f),
                IMapleProxiedCalls::SetImplementation(element) => element.fmt(f),
                IMapleProxiedCalls::Upgrade(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FactoryCall> for IMapleProxiedCalls {
        fn from(var: FactoryCall) -> Self {
            IMapleProxiedCalls::Factory(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for IMapleProxiedCalls {
        fn from(var: ImplementationCall) -> Self {
            IMapleProxiedCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for IMapleProxiedCalls {
        fn from(var: MigrateCall) -> Self {
            IMapleProxiedCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for IMapleProxiedCalls {
        fn from(var: SetImplementationCall) -> Self {
            IMapleProxiedCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for IMapleProxiedCalls {
        fn from(var: UpgradeCall) -> Self {
            IMapleProxiedCalls::Upgrade(var)
        }
    }
}
