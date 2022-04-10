pub use mockfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod mockfactory_mod {
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
    #[doc = "MockFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mapleGlobals\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"globals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGlobals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeInstance\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610295806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80633a60339a14610046578063cc2e0a2614610075578063fe69f708146100a7575b600080fd5b600054610059906001600160a01b031681565b6040516001600160a01b03909116815260200160405180910390f35b6100a561008336600461016c565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b005b6100a56100b5366004610190565b60006100c38284018461016c565b6040516001600160a01b0382166024820152909150600090339060440160408051601f198184030181529181526020820180516001600160e01b0316636bc26a1360e11b17905251610115919061020c565b6000604051808303816000865af19150503d8060008114610152576040519150601f19603f3d011682016040523d82523d6000602084013e610157565b606091505b505090508061016557600080fd5b5050505050565b60006020828403121561017e57600080fd5b813561018981610247565b9392505050565b6000806000604084860312156101a557600080fd5b83359250602084013567ffffffffffffffff808211156101c457600080fd5b818601915086601f8301126101d857600080fd5b8135818111156101e757600080fd5b8760208285010111156101f957600080fd5b6020830194508093505050509250925092565b6000825160005b8181101561022d5760208186018101518583015201610213565b8181111561023c576000828501525b509190910192915050565b6001600160a01b038116811461025c57600080fd5b5056fea264697066735822122044c2952481de96a68fedc9e8ca53c09f645eaad1bc30867ed73a969abcfe58a864736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MockFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MockFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MockFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKFACTORY_ABI.clone(), client).into()
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
                MOCKFACTORY_ABI.clone(),
                MOCKFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `mapleGlobals` (0x3a60339a) function"]
        pub fn maple_globals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 96, 51, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGlobals` (0xcc2e0a26) function"]
        pub fn set_globals(
            &self,
            globals: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 46, 10, 38], globals)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeInstance` (0xfe69f708) function"]
        pub fn upgrade_instance(
            &self,
            p0: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 105, 247, 8], (p0, arguments))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockFactory<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `mapleGlobals`function with signature `mapleGlobals()` and selector `[58, 96, 51, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mapleGlobals", abi = "mapleGlobals()")]
    pub struct MapleGlobalsCall;
    #[doc = "Container type for all input parameters for the `setGlobals`function with signature `setGlobals(address)` and selector `[204, 46, 10, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGlobals", abi = "setGlobals(address)")]
    pub struct SetGlobalsCall {
        pub globals: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeInstance`function with signature `upgradeInstance(uint256,bytes)` and selector `[254, 105, 247, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeInstance", abi = "upgradeInstance(uint256,bytes)")]
    pub struct UpgradeInstanceCall {
        pub p0: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockFactoryCalls {
        MapleGlobals(MapleGlobalsCall),
        SetGlobals(SetGlobalsCall),
        UpgradeInstance(UpgradeInstanceCall),
    }
    impl ethers::core::abi::AbiDecode for MockFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MapleGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFactoryCalls::MapleGlobals(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFactoryCalls::SetGlobals(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFactoryCalls::UpgradeInstance(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockFactoryCalls::MapleGlobals(element) => element.encode(),
                MockFactoryCalls::SetGlobals(element) => element.encode(),
                MockFactoryCalls::UpgradeInstance(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFactoryCalls::MapleGlobals(element) => element.fmt(f),
                MockFactoryCalls::SetGlobals(element) => element.fmt(f),
                MockFactoryCalls::UpgradeInstance(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MapleGlobalsCall> for MockFactoryCalls {
        fn from(var: MapleGlobalsCall) -> Self {
            MockFactoryCalls::MapleGlobals(var)
        }
    }
    impl ::std::convert::From<SetGlobalsCall> for MockFactoryCalls {
        fn from(var: SetGlobalsCall) -> Self {
            MockFactoryCalls::SetGlobals(var)
        }
    }
    impl ::std::convert::From<UpgradeInstanceCall> for MockFactoryCalls {
        fn from(var: UpgradeInstanceCall) -> Self {
            MockFactoryCalls::UpgradeInstance(var)
        }
    }
}
