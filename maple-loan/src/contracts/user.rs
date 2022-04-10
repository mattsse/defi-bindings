pub use user_mod::*;
#[allow(clippy::too_many_arguments)]
mod user_mod {
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
    #[doc = "User was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static USER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxied_upgrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_createInstance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxied_upgrade\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_createInstance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static USER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506104b5806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806348b4e6bc1461005157806399c96d4c14610081578063b501482f146100a4578063e5743b4f146100b9575b600080fd5b61006461005f36600461030d565b6100cc565b6040516001600160a01b0390911681526020015b60405180910390f35b61009461008f36600461030d565b61015a565b6040519015158152602001610078565b6100b76100b2366004610369565b61020f565b005b6100946100c7366004610369565b610277565b60405163517b657f60e01b81526000906001600160a01b0386169063517b657f906100ff90879087908790600401610429565b602060405180830381600087803b15801561011957600080fd5b505af115801561012d573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061015191906102e9565b95945050505050565b6000846001600160a01b031663517b657f60e01b85858560405160240161018393929190610429565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516101c191906103ee565b6000604051808303816000865af19150503d80600081146101fe576040519150601f19603f3d011682016040523d82523d6000602084013e610203565b606091505b50909695505050505050565b604051631dccde7760e11b81526001600160a01b03851690633b99bcee9061023f9086908690869060040161044d565b600060405180830381600087803b15801561025957600080fd5b505af115801561026d573d6000803e3d6000fd5b5050505050505050565b6000846001600160a01b0316633b99bcee60e01b8585856040516024016101839392919061044d565b60008083601f8401126102b257600080fd5b50813567ffffffffffffffff8111156102ca57600080fd5b6020830191508360208285010111156102e257600080fd5b9250929050565b6000602082840312156102fb57600080fd5b815161030681610467565b9392505050565b6000806000806060858703121561032357600080fd5b843561032e81610467565b9350602085013567ffffffffffffffff81111561034a57600080fd5b610356878288016102a0565b9598909750949560400135949350505050565b6000806000806060858703121561037f57600080fd5b843561038a81610467565b935060208501359250604085013567ffffffffffffffff8111156103ad57600080fd5b6103b9878288016102a0565b95989497509550505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b8181101561040f57602081860181015185830152016103f5565b8181111561041e576000828501525b509190910192915050565b60408152600061043d6040830185876103c5565b9050826020830152949350505050565b8381526040602082015260006101516040830184866103c5565b6001600160a01b038116811461047c57600080fd5b5056fea2646970667358221220c9c940a645630b888fd4845c51b71798534e88a720ae296f2f6f36db6cb2931f64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct User<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for User<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for User<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(User))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> User<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), USER_ABI.clone(), client).into()
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
                USER_ABI.clone(),
                USER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `mapleProxied_upgrade` (0xb501482f) function"]
        pub fn maple_proxied_upgrade(
            &self,
            instance: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 1, 72, 47], (instance, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_createInstance` (0x48b4e6bc) function"]
        pub fn maple_proxy_factory_create_instance(
            &self,
            factory: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([72, 180, 230, 188], (factory, arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxied_upgrade` (0xe5743b4f) function"]
        pub fn try_maple_proxied_upgrade(
            &self,
            instance: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 116, 59, 79], (instance, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_createInstance` (0x99c96d4c) function"]
        pub fn try_maple_proxy_factory_create_instance(
            &self,
            factory: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 201, 109, 76], (factory, arguments, salt))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for User<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `mapleProxied_upgrade`function with signature `mapleProxied_upgrade(address,uint256,bytes)` and selector `[181, 1, 72, 47]`"]
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
        name = "mapleProxied_upgrade",
        abi = "mapleProxied_upgrade(address,uint256,bytes)"
    )]
    pub struct MapleProxiedUpgradeCall {
        pub instance: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_createInstance`function with signature `mapleProxyFactory_createInstance(address,bytes,bytes32)` and selector `[72, 180, 230, 188]`"]
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
        name = "mapleProxyFactory_createInstance",
        abi = "mapleProxyFactory_createInstance(address,bytes,bytes32)"
    )]
    pub struct MapleProxyFactoryCreateInstanceCall {
        pub factory: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxied_upgrade`function with signature `try_mapleProxied_upgrade(address,uint256,bytes)` and selector `[229, 116, 59, 79]`"]
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
        name = "try_mapleProxied_upgrade",
        abi = "try_mapleProxied_upgrade(address,uint256,bytes)"
    )]
    pub struct TryMapleProxiedUpgradeCall {
        pub instance: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_createInstance`function with signature `try_mapleProxyFactory_createInstance(address,bytes,bytes32)` and selector `[153, 201, 109, 76]`"]
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
        name = "try_mapleProxyFactory_createInstance",
        abi = "try_mapleProxyFactory_createInstance(address,bytes,bytes32)"
    )]
    pub struct TryMapleProxyFactoryCreateInstanceCall {
        pub factory: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UserCalls {
        MapleProxiedUpgrade(MapleProxiedUpgradeCall),
        MapleProxyFactoryCreateInstance(MapleProxyFactoryCreateInstanceCall),
        TryMapleProxiedUpgrade(TryMapleProxiedUpgradeCall),
        TryMapleProxyFactoryCreateInstance(TryMapleProxyFactoryCreateInstanceCall),
    }
    impl ethers::core::abi::AbiDecode for UserCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MapleProxiedUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UserCalls::MapleProxiedUpgrade(decoded));
            }
            if let Ok(decoded) =
                <MapleProxyFactoryCreateInstanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UserCalls::MapleProxyFactoryCreateInstance(decoded));
            }
            if let Ok(decoded) =
                <TryMapleProxiedUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UserCalls::TryMapleProxiedUpgrade(decoded));
            }
            if let Ok(decoded) =
                <TryMapleProxyFactoryCreateInstanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UserCalls::TryMapleProxyFactoryCreateInstance(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UserCalls::MapleProxiedUpgrade(element) => element.encode(),
                UserCalls::MapleProxyFactoryCreateInstance(element) => element.encode(),
                UserCalls::TryMapleProxiedUpgrade(element) => element.encode(),
                UserCalls::TryMapleProxyFactoryCreateInstance(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UserCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UserCalls::MapleProxiedUpgrade(element) => element.fmt(f),
                UserCalls::MapleProxyFactoryCreateInstance(element) => element.fmt(f),
                UserCalls::TryMapleProxiedUpgrade(element) => element.fmt(f),
                UserCalls::TryMapleProxyFactoryCreateInstance(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MapleProxiedUpgradeCall> for UserCalls {
        fn from(var: MapleProxiedUpgradeCall) -> Self {
            UserCalls::MapleProxiedUpgrade(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactoryCreateInstanceCall> for UserCalls {
        fn from(var: MapleProxyFactoryCreateInstanceCall) -> Self {
            UserCalls::MapleProxyFactoryCreateInstance(var)
        }
    }
    impl ::std::convert::From<TryMapleProxiedUpgradeCall> for UserCalls {
        fn from(var: TryMapleProxiedUpgradeCall) -> Self {
            UserCalls::TryMapleProxiedUpgrade(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactoryCreateInstanceCall> for UserCalls {
        fn from(var: TryMapleProxyFactoryCreateInstanceCall) -> Self {
            UserCalls::TryMapleProxyFactoryCreateInstance(var)
        }
    }
}
