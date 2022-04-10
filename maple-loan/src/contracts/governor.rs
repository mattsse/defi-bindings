pub use governor_mod::*;
#[allow(clippy::too_many_arguments)]
mod governor_mod {
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
    #[doc = "Governor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GOVERNOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_disableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_enableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_registerImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_setDefaultVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_setGlobals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_disableUpgradePath\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_enableUpgradePath\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_registerImplementation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_setDefaultVersion\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_setGlobals\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static GOVERNOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610738806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c8063b59f2fc711610066578063b59f2fc714610118578063b8b2944f1461012b578063da2a0ce81461013e578063e3013dfe14610151578063eb10b6d51461016457600080fd5b806319cbc582146100a35780632f44fb30146100b857806336e71309146100cb57806342a78438146100f257806356d8d40e14610105575b600080fd5b6100b66100b136600461060c565b610177565b005b6100b66100c6366004610659565b6101e8565b6100de6100d93660046105e2565b61024e565b604051901515815260200160405180910390f35b6100de6101003660046105af565b6102ff565b6100de61011336600461068c565b61032a565b6100de61012636600461060c565b6103eb565b6100b66101393660046105e2565b610425565b6100b661014c3660046105af565b610484565b6100b661015f36600461068c565b6104b2565b6100de610172366004610659565b6104ee565b60405163093805d760e11b8152600481018490526001600160a01b03838116602483015282811660448301528516906312700bae906064015b600060405180830381600087803b1580156101ca57600080fd5b505af11580156101de573d6000803e3d6000fd5b5050505050505050565b60405163440c148960e11b815260048101839052602481018290526001600160a01b03841690638818291290604401600060405180830381600087803b15801561023157600080fd5b505af1158015610245573d6000803e3d6000fd5b50505050505050565b6000826001600160a01b031663b4e6747f60e01b8360405160240161027591815260200190565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516102b391906106c7565b6000604051808303816000865af19150503d80600081146102f0576040519150601f19603f3d011682016040523d82523d6000602084013e6102f5565b606091505b5090949350505050565b6040516001600160a01b03828116602483015260009190841690636617051360e11b90604401610275565b60405160248101849052604481018390526001600160a01b0382811660648301526000919086169063b28317bf60e01b906084015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161039d91906106c7565b6000604051808303816000865af19150503d80600081146103da576040519150601f19603f3d011682016040523d82523d6000602084013e6103df565b606091505b50909695505050505050565b604051602481018490526001600160a01b03838116604483015282811660648301526000919086169063093805d760e11b9060840161035f565b60405163b4e6747f60e01b8152600481018290526001600160a01b0383169063b4e6747f906024015b600060405180830381600087803b15801561046857600080fd5b505af115801561047c573d6000803e3d6000fd5b505050505050565b604051636617051360e11b81526001600160a01b03828116600483015283169063cc2e0a269060240161044e565b60405163b28317bf60e01b815260048101849052602481018390526001600160a01b03828116604483015285169063b28317bf906064016101b0565b6040805160248101849052604480820184905282518083039091018152606490910182526020810180516001600160e01b031663440c148960e11b17905290516000916001600160a01b0386169161054691906106c7565b6000604051808303816000865af19150503d8060008114610583576040519150601f19603f3d011682016040523d82523d6000602084013e610588565b606091505b509095945050505050565b80356001600160a01b03811681146105aa57600080fd5b919050565b600080604083850312156105c257600080fd5b6105cb83610593565b91506105d960208401610593565b90509250929050565b600080604083850312156105f557600080fd5b6105fe83610593565b946020939093013593505050565b6000806000806080858703121561062257600080fd5b61062b85610593565b93506020850135925061064060408601610593565b915061064e60608601610593565b905092959194509250565b60008060006060848603121561066e57600080fd5b61067784610593565b95602085013595506040909401359392505050565b600080600080608085870312156106a257600080fd5b6106ab85610593565b9350602085013592506040850135915061064e60608601610593565b6000825160005b818110156106e857602081860181015185830152016106ce565b818111156106f7576000828501525b50919091019291505056fea2646970667358221220d5d77ff5bbbf576b66f39f0d61c3c717e625cf77c65cffcf309b5d219a037f8864736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Governor<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Governor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Governor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Governor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Governor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GOVERNOR_ABI.clone(), client).into()
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
                GOVERNOR_ABI.clone(),
                GOVERNOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `mapleProxyFactory_disableUpgradePath` (0x2f44fb30) function"]
        pub fn maple_proxy_factory_disable_upgrade_path(
            &self,
            factory: ethers::core::types::Address,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 68, 251, 48], (factory, from_version, to_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_enableUpgradePath` (0xe3013dfe) function"]
        pub fn maple_proxy_factory_enable_upgrade_path(
            &self,
            factory: ethers::core::types::Address,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
            migrator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 1, 61, 254],
                    (factory, from_version, to_version, migrator),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_registerImplementation` (0x19cbc582) function"]
        pub fn maple_proxy_factory_register_implementation(
            &self,
            factory: ethers::core::types::Address,
            version: ethers::core::types::U256,
            implementation_address: ethers::core::types::Address,
            initializer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [25, 203, 197, 130],
                    (factory, version, implementation_address, initializer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_setDefaultVersion` (0xb8b2944f) function"]
        pub fn maple_proxy_factory_set_default_version(
            &self,
            factory: ethers::core::types::Address,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 178, 148, 79], (factory, version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_setGlobals` (0xda2a0ce8) function"]
        pub fn maple_proxy_factory_set_globals(
            &self,
            factory: ethers::core::types::Address,
            maple_globals: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 42, 12, 232], (factory, maple_globals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_disableUpgradePath` (0xeb10b6d5) function"]
        pub fn try_maple_proxy_factory_disable_upgrade_path(
            &self,
            factory: ethers::core::types::Address,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 16, 182, 213], (factory, from_version, to_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_enableUpgradePath` (0x56d8d40e) function"]
        pub fn try_maple_proxy_factory_enable_upgrade_path(
            &self,
            factory: ethers::core::types::Address,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
            migrator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [86, 216, 212, 14],
                    (factory, from_version, to_version, migrator),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_registerImplementation` (0xb59f2fc7) function"]
        pub fn try_maple_proxy_factory_register_implementation(
            &self,
            factory: ethers::core::types::Address,
            version: ethers::core::types::U256,
            implementation_address: ethers::core::types::Address,
            initializer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [181, 159, 47, 199],
                    (factory, version, implementation_address, initializer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_setDefaultVersion` (0x36e71309) function"]
        pub fn try_maple_proxy_factory_set_default_version(
            &self,
            factory: ethers::core::types::Address,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([54, 231, 19, 9], (factory, version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_setGlobals` (0x42a78438) function"]
        pub fn try_maple_proxy_factory_set_globals(
            &self,
            factory: ethers::core::types::Address,
            maple_globals: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 167, 132, 56], (factory, maple_globals))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Governor<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_disableUpgradePath`function with signature `mapleProxyFactory_disableUpgradePath(address,uint256,uint256)` and selector `[47, 68, 251, 48]`"]
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
        name = "mapleProxyFactory_disableUpgradePath",
        abi = "mapleProxyFactory_disableUpgradePath(address,uint256,uint256)"
    )]
    pub struct MapleProxyFactoryDisableUpgradePathCall {
        pub factory: ethers::core::types::Address,
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_enableUpgradePath`function with signature `mapleProxyFactory_enableUpgradePath(address,uint256,uint256,address)` and selector `[227, 1, 61, 254]`"]
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
        name = "mapleProxyFactory_enableUpgradePath",
        abi = "mapleProxyFactory_enableUpgradePath(address,uint256,uint256,address)"
    )]
    pub struct MapleProxyFactoryEnableUpgradePathCall {
        pub factory: ethers::core::types::Address,
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
        pub migrator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_registerImplementation`function with signature `mapleProxyFactory_registerImplementation(address,uint256,address,address)` and selector `[25, 203, 197, 130]`"]
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
        name = "mapleProxyFactory_registerImplementation",
        abi = "mapleProxyFactory_registerImplementation(address,uint256,address,address)"
    )]
    pub struct MapleProxyFactoryRegisterImplementationCall {
        pub factory: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
        pub implementation_address: ethers::core::types::Address,
        pub initializer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_setDefaultVersion`function with signature `mapleProxyFactory_setDefaultVersion(address,uint256)` and selector `[184, 178, 148, 79]`"]
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
        name = "mapleProxyFactory_setDefaultVersion",
        abi = "mapleProxyFactory_setDefaultVersion(address,uint256)"
    )]
    pub struct MapleProxyFactorySetDefaultVersionCall {
        pub factory: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_setGlobals`function with signature `mapleProxyFactory_setGlobals(address,address)` and selector `[218, 42, 12, 232]`"]
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
        name = "mapleProxyFactory_setGlobals",
        abi = "mapleProxyFactory_setGlobals(address,address)"
    )]
    pub struct MapleProxyFactorySetGlobalsCall {
        pub factory: ethers::core::types::Address,
        pub maple_globals: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_disableUpgradePath`function with signature `try_mapleProxyFactory_disableUpgradePath(address,uint256,uint256)` and selector `[235, 16, 182, 213]`"]
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
        name = "try_mapleProxyFactory_disableUpgradePath",
        abi = "try_mapleProxyFactory_disableUpgradePath(address,uint256,uint256)"
    )]
    pub struct TryMapleProxyFactoryDisableUpgradePathCall {
        pub factory: ethers::core::types::Address,
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_enableUpgradePath`function with signature `try_mapleProxyFactory_enableUpgradePath(address,uint256,uint256,address)` and selector `[86, 216, 212, 14]`"]
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
        name = "try_mapleProxyFactory_enableUpgradePath",
        abi = "try_mapleProxyFactory_enableUpgradePath(address,uint256,uint256,address)"
    )]
    pub struct TryMapleProxyFactoryEnableUpgradePathCall {
        pub factory: ethers::core::types::Address,
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
        pub migrator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_registerImplementation`function with signature `try_mapleProxyFactory_registerImplementation(address,uint256,address,address)` and selector `[181, 159, 47, 199]`"]
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
        name = "try_mapleProxyFactory_registerImplementation",
        abi = "try_mapleProxyFactory_registerImplementation(address,uint256,address,address)"
    )]
    pub struct TryMapleProxyFactoryRegisterImplementationCall {
        pub factory: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
        pub implementation_address: ethers::core::types::Address,
        pub initializer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_setDefaultVersion`function with signature `try_mapleProxyFactory_setDefaultVersion(address,uint256)` and selector `[54, 231, 19, 9]`"]
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
        name = "try_mapleProxyFactory_setDefaultVersion",
        abi = "try_mapleProxyFactory_setDefaultVersion(address,uint256)"
    )]
    pub struct TryMapleProxyFactorySetDefaultVersionCall {
        pub factory: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_setGlobals`function with signature `try_mapleProxyFactory_setGlobals(address,address)` and selector `[66, 167, 132, 56]`"]
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
        name = "try_mapleProxyFactory_setGlobals",
        abi = "try_mapleProxyFactory_setGlobals(address,address)"
    )]
    pub struct TryMapleProxyFactorySetGlobalsCall {
        pub factory: ethers::core::types::Address,
        pub maple_globals: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GovernorCalls {
        MapleProxyFactoryDisableUpgradePath(MapleProxyFactoryDisableUpgradePathCall),
        MapleProxyFactoryEnableUpgradePath(MapleProxyFactoryEnableUpgradePathCall),
        MapleProxyFactoryRegisterImplementation(MapleProxyFactoryRegisterImplementationCall),
        MapleProxyFactorySetDefaultVersion(MapleProxyFactorySetDefaultVersionCall),
        MapleProxyFactorySetGlobals(MapleProxyFactorySetGlobalsCall),
        TryMapleProxyFactoryDisableUpgradePath(TryMapleProxyFactoryDisableUpgradePathCall),
        TryMapleProxyFactoryEnableUpgradePath(TryMapleProxyFactoryEnableUpgradePathCall),
        TryMapleProxyFactoryRegisterImplementation(TryMapleProxyFactoryRegisterImplementationCall),
        TryMapleProxyFactorySetDefaultVersion(TryMapleProxyFactorySetDefaultVersionCall),
        TryMapleProxyFactorySetGlobals(TryMapleProxyFactorySetGlobalsCall),
    }
    impl ethers::core::abi::AbiDecode for GovernorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MapleProxyFactoryDisableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::MapleProxyFactoryDisableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <MapleProxyFactoryEnableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::MapleProxyFactoryEnableUpgradePath(decoded));
            }
            if let Ok (decoded) = < MapleProxyFactoryRegisterImplementationCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (GovernorCalls :: MapleProxyFactoryRegisterImplementation (decoded)) }
            if let Ok(decoded) =
                <MapleProxyFactorySetDefaultVersionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::MapleProxyFactorySetDefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <MapleProxyFactorySetGlobalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::MapleProxyFactorySetGlobals(decoded));
            }
            if let Ok(decoded) =
                <TryMapleProxyFactoryDisableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::TryMapleProxyFactoryDisableUpgradePath(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TryMapleProxyFactoryEnableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::TryMapleProxyFactoryEnableUpgradePath(
                    decoded,
                ));
            }
            if let Ok (decoded) = < TryMapleProxyFactoryRegisterImplementationCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (GovernorCalls :: TryMapleProxyFactoryRegisterImplementation (decoded)) }
            if let Ok(decoded) =
                <TryMapleProxyFactorySetDefaultVersionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::TryMapleProxyFactorySetDefaultVersion(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TryMapleProxyFactorySetGlobalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernorCalls::TryMapleProxyFactorySetGlobals(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GovernorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GovernorCalls::MapleProxyFactoryDisableUpgradePath(element) => element.encode(),
                GovernorCalls::MapleProxyFactoryEnableUpgradePath(element) => element.encode(),
                GovernorCalls::MapleProxyFactoryRegisterImplementation(element) => element.encode(),
                GovernorCalls::MapleProxyFactorySetDefaultVersion(element) => element.encode(),
                GovernorCalls::MapleProxyFactorySetGlobals(element) => element.encode(),
                GovernorCalls::TryMapleProxyFactoryDisableUpgradePath(element) => element.encode(),
                GovernorCalls::TryMapleProxyFactoryEnableUpgradePath(element) => element.encode(),
                GovernorCalls::TryMapleProxyFactoryRegisterImplementation(element) => {
                    element.encode()
                }
                GovernorCalls::TryMapleProxyFactorySetDefaultVersion(element) => element.encode(),
                GovernorCalls::TryMapleProxyFactorySetGlobals(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GovernorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GovernorCalls::MapleProxyFactoryDisableUpgradePath(element) => element.fmt(f),
                GovernorCalls::MapleProxyFactoryEnableUpgradePath(element) => element.fmt(f),
                GovernorCalls::MapleProxyFactoryRegisterImplementation(element) => element.fmt(f),
                GovernorCalls::MapleProxyFactorySetDefaultVersion(element) => element.fmt(f),
                GovernorCalls::MapleProxyFactorySetGlobals(element) => element.fmt(f),
                GovernorCalls::TryMapleProxyFactoryDisableUpgradePath(element) => element.fmt(f),
                GovernorCalls::TryMapleProxyFactoryEnableUpgradePath(element) => element.fmt(f),
                GovernorCalls::TryMapleProxyFactoryRegisterImplementation(element) => {
                    element.fmt(f)
                }
                GovernorCalls::TryMapleProxyFactorySetDefaultVersion(element) => element.fmt(f),
                GovernorCalls::TryMapleProxyFactorySetGlobals(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MapleProxyFactoryDisableUpgradePathCall> for GovernorCalls {
        fn from(var: MapleProxyFactoryDisableUpgradePathCall) -> Self {
            GovernorCalls::MapleProxyFactoryDisableUpgradePath(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactoryEnableUpgradePathCall> for GovernorCalls {
        fn from(var: MapleProxyFactoryEnableUpgradePathCall) -> Self {
            GovernorCalls::MapleProxyFactoryEnableUpgradePath(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactoryRegisterImplementationCall> for GovernorCalls {
        fn from(var: MapleProxyFactoryRegisterImplementationCall) -> Self {
            GovernorCalls::MapleProxyFactoryRegisterImplementation(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactorySetDefaultVersionCall> for GovernorCalls {
        fn from(var: MapleProxyFactorySetDefaultVersionCall) -> Self {
            GovernorCalls::MapleProxyFactorySetDefaultVersion(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactorySetGlobalsCall> for GovernorCalls {
        fn from(var: MapleProxyFactorySetGlobalsCall) -> Self {
            GovernorCalls::MapleProxyFactorySetGlobals(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactoryDisableUpgradePathCall> for GovernorCalls {
        fn from(var: TryMapleProxyFactoryDisableUpgradePathCall) -> Self {
            GovernorCalls::TryMapleProxyFactoryDisableUpgradePath(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactoryEnableUpgradePathCall> for GovernorCalls {
        fn from(var: TryMapleProxyFactoryEnableUpgradePathCall) -> Self {
            GovernorCalls::TryMapleProxyFactoryEnableUpgradePath(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactoryRegisterImplementationCall> for GovernorCalls {
        fn from(var: TryMapleProxyFactoryRegisterImplementationCall) -> Self {
            GovernorCalls::TryMapleProxyFactoryRegisterImplementation(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactorySetDefaultVersionCall> for GovernorCalls {
        fn from(var: TryMapleProxyFactorySetDefaultVersionCall) -> Self {
            GovernorCalls::TryMapleProxyFactorySetDefaultVersion(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactorySetGlobalsCall> for GovernorCalls {
        fn from(var: TryMapleProxyFactorySetGlobalsCall) -> Self {
            GovernorCalls::TryMapleProxyFactorySetGlobals(var)
        }
    }
}
