pub use imapleproxyfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod imapleproxyfactory_mod {
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
    #[doc = "IMapleProxyFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IMAPLEPROXYFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DefaultVersionSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ImplementationRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"initializationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"migrationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MapleGlobalsSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathEnabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createInstance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"defaultImplementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"defaultVersion_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInstanceAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instanceAddress_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementationOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mapleGlobals\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"migratorForPath\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDefaultVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGlobals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"upgradeEnabledForPath\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"allowed_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeInstance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"versionOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IMAPLEPROXYFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IMapleProxyFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IMapleProxyFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IMapleProxyFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMapleProxyFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IMapleProxyFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMAPLEPROXYFACTORY_ABI.clone(), client)
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
                IMAPLEPROXYFACTORY_ABI.clone(),
                IMAPLEPROXYFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `createInstance` (0x517b657f) function"]
        pub fn create_instance(
            &self,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([81, 123, 101, 127], (arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultImplementation` (0xb39c4593) function"]
        pub fn default_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([179, 156, 69, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultVersion` (0x1798d482) function"]
        pub fn default_version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 152, 212, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableUpgradePath` (0x88182912) function"]
        pub fn disable_upgrade_path(
            &self,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 24, 41, 18], (from_version, to_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableUpgradePath` (0xb28317bf) function"]
        pub fn enable_upgrade_path(
            &self,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
            migrator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 131, 23, 191], (from_version, to_version, migrator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInstanceAddress` (0x0e6e4b25) function"]
        pub fn get_instance_address(
            &self,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([14, 110, 75, 37], (arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementationOf` (0x8636f07e) function"]
        pub fn implementation_of(
            &self,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([134, 54, 240, 126], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleGlobals` (0x3a60339a) function"]
        pub fn maple_globals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 96, 51, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migratorForPath` (0x85b8a52f) function"]
        pub fn migrator_for_path(
            &self,
            old_version: ethers::core::types::U256,
            new_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([133, 184, 165, 47], (old_version, new_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerImplementation` (0x12700bae) function"]
        pub fn register_implementation(
            &self,
            version: ethers::core::types::U256,
            implementation_address: ethers::core::types::Address,
            initializer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 112, 11, 174],
                    (version, implementation_address, initializer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDefaultVersion` (0xb4e6747f) function"]
        pub fn set_default_version(
            &self,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 230, 116, 127], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGlobals` (0xcc2e0a26) function"]
        pub fn set_globals(
            &self,
            maple_globals: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 46, 10, 38], maple_globals)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeEnabledForPath` (0xd867e0de) function"]
        pub fn upgrade_enabled_for_path(
            &self,
            to_version: ethers::core::types::U256,
            from_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([216, 103, 224, 222], (to_version, from_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeInstance` (0xfe69f708) function"]
        pub fn upgrade_instance(
            &self,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 105, 247, 8], (to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `versionOf` (0x0db3ff45) function"]
        pub fn version_of(
            &self,
            implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([13, 179, 255, 69], implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DefaultVersionSet` event"]
        pub fn default_version_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DefaultVersionSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ImplementationRegistered` event"]
        pub fn implementation_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ImplementationRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InstanceDeployed` event"]
        pub fn instance_deployed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InstanceDeployedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InstanceUpgraded` event"]
        pub fn instance_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InstanceUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MapleGlobalsSet` event"]
        pub fn maple_globals_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MapleGlobalsSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpgradePathDisabled` event"]
        pub fn upgrade_path_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpgradePathDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpgradePathEnabled` event"]
        pub fn upgrade_path_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpgradePathEnabledFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IMapleProxyFactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IMapleProxyFactory<M>
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
    #[ethevent(name = "DefaultVersionSet", abi = "DefaultVersionSet(uint256)")]
    pub struct DefaultVersionSetFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
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
    #[ethevent(
        name = "ImplementationRegistered",
        abi = "ImplementationRegistered(uint256,address,address)"
    )]
    pub struct ImplementationRegisteredFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub implementation_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub initializer: ethers::core::types::Address,
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
    #[ethevent(
        name = "InstanceDeployed",
        abi = "InstanceDeployed(uint256,address,bytes)"
    )]
    pub struct InstanceDeployedFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub instance: ethers::core::types::Address,
        pub initialization_arguments: ethers::core::types::Bytes,
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
    #[ethevent(
        name = "InstanceUpgraded",
        abi = "InstanceUpgraded(address,uint256,uint256,bytes)"
    )]
    pub struct InstanceUpgradedFilter {
        #[ethevent(indexed)]
        pub instance: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
        pub migration_arguments: ethers::core::types::Bytes,
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
    #[ethevent(name = "MapleGlobalsSet", abi = "MapleGlobalsSet(address)")]
    pub struct MapleGlobalsSetFilter {
        #[ethevent(indexed)]
        pub maple_globals: ethers::core::types::Address,
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
    #[ethevent(
        name = "UpgradePathDisabled",
        abi = "UpgradePathDisabled(uint256,uint256)"
    )]
    pub struct UpgradePathDisabledFilter {
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
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
    #[ethevent(
        name = "UpgradePathEnabled",
        abi = "UpgradePathEnabled(uint256,uint256,address)"
    )]
    pub struct UpgradePathEnabledFilter {
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub migrator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleProxyFactoryEvents {
        DefaultVersionSetFilter(DefaultVersionSetFilter),
        ImplementationRegisteredFilter(ImplementationRegisteredFilter),
        InstanceDeployedFilter(InstanceDeployedFilter),
        InstanceUpgradedFilter(InstanceUpgradedFilter),
        MapleGlobalsSetFilter(MapleGlobalsSetFilter),
        UpgradePathDisabledFilter(UpgradePathDisabledFilter),
        UpgradePathEnabledFilter(UpgradePathEnabledFilter),
    }
    impl ethers::contract::EthLogDecode for IMapleProxyFactoryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DefaultVersionSetFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::DefaultVersionSetFilter(decoded));
            }
            if let Ok(decoded) = ImplementationRegisteredFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::ImplementationRegisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InstanceDeployedFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::InstanceDeployedFilter(decoded));
            }
            if let Ok(decoded) = InstanceUpgradedFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::InstanceUpgradedFilter(decoded));
            }
            if let Ok(decoded) = MapleGlobalsSetFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::MapleGlobalsSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathDisabledFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::UpgradePathDisabledFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathEnabledFilter::decode_log(log) {
                return Ok(IMapleProxyFactoryEvents::UpgradePathEnabledFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IMapleProxyFactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleProxyFactoryEvents::DefaultVersionSetFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::ImplementationRegisteredFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::InstanceDeployedFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::InstanceUpgradedFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::MapleGlobalsSetFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::UpgradePathDisabledFilter(element) => element.fmt(f),
                IMapleProxyFactoryEvents::UpgradePathEnabledFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `createInstance`function with signature `createInstance(bytes,bytes32)` and selector `[81, 123, 101, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createInstance", abi = "createInstance(bytes,bytes32)")]
    pub struct CreateInstanceCall {
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `defaultImplementation`function with signature `defaultImplementation()` and selector `[179, 156, 69, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "defaultImplementation", abi = "defaultImplementation()")]
    pub struct DefaultImplementationCall;
    #[doc = "Container type for all input parameters for the `defaultVersion`function with signature `defaultVersion()` and selector `[23, 152, 212, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "defaultVersion", abi = "defaultVersion()")]
    pub struct DefaultVersionCall;
    #[doc = "Container type for all input parameters for the `disableUpgradePath`function with signature `disableUpgradePath(uint256,uint256)` and selector `[136, 24, 41, 18]`"]
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
        name = "disableUpgradePath",
        abi = "disableUpgradePath(uint256,uint256)"
    )]
    pub struct DisableUpgradePathCall {
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `enableUpgradePath`function with signature `enableUpgradePath(uint256,uint256,address)` and selector `[178, 131, 23, 191]`"]
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
        name = "enableUpgradePath",
        abi = "enableUpgradePath(uint256,uint256,address)"
    )]
    pub struct EnableUpgradePathCall {
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
        pub migrator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getInstanceAddress`function with signature `getInstanceAddress(bytes,bytes32)` and selector `[14, 110, 75, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getInstanceAddress", abi = "getInstanceAddress(bytes,bytes32)")]
    pub struct GetInstanceAddressCall {
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `implementationOf`function with signature `implementationOf(uint256)` and selector `[134, 54, 240, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementationOf", abi = "implementationOf(uint256)")]
    pub struct ImplementationOfCall {
        pub version: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `migratorForPath`function with signature `migratorForPath(uint256,uint256)` and selector `[133, 184, 165, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "migratorForPath", abi = "migratorForPath(uint256,uint256)")]
    pub struct MigratorForPathCall {
        pub old_version: ethers::core::types::U256,
        pub new_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `registerImplementation`function with signature `registerImplementation(uint256,address,address)` and selector `[18, 112, 11, 174]`"]
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
        name = "registerImplementation",
        abi = "registerImplementation(uint256,address,address)"
    )]
    pub struct RegisterImplementationCall {
        pub version: ethers::core::types::U256,
        pub implementation_address: ethers::core::types::Address,
        pub initializer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDefaultVersion`function with signature `setDefaultVersion(uint256)` and selector `[180, 230, 116, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDefaultVersion", abi = "setDefaultVersion(uint256)")]
    pub struct SetDefaultVersionCall {
        pub version: ethers::core::types::U256,
    }
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
        pub maple_globals: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeEnabledForPath`function with signature `upgradeEnabledForPath(uint256,uint256)` and selector `[216, 103, 224, 222]`"]
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
        name = "upgradeEnabledForPath",
        abi = "upgradeEnabledForPath(uint256,uint256)"
    )]
    pub struct UpgradeEnabledForPathCall {
        pub to_version: ethers::core::types::U256,
        pub from_version: ethers::core::types::U256,
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
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `versionOf`function with signature `versionOf(address)` and selector `[13, 179, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "versionOf", abi = "versionOf(address)")]
    pub struct VersionOfCall {
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleProxyFactoryCalls {
        CreateInstance(CreateInstanceCall),
        DefaultImplementation(DefaultImplementationCall),
        DefaultVersion(DefaultVersionCall),
        DisableUpgradePath(DisableUpgradePathCall),
        EnableUpgradePath(EnableUpgradePathCall),
        GetInstanceAddress(GetInstanceAddressCall),
        ImplementationOf(ImplementationOfCall),
        MapleGlobals(MapleGlobalsCall),
        MigratorForPath(MigratorForPathCall),
        RegisterImplementation(RegisterImplementationCall),
        SetDefaultVersion(SetDefaultVersionCall),
        SetGlobals(SetGlobalsCall),
        UpgradeEnabledForPath(UpgradeEnabledForPathCall),
        UpgradeInstance(UpgradeInstanceCall),
        VersionOf(VersionOfCall),
    }
    impl ethers::core::abi::AbiDecode for IMapleProxyFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::CreateInstance(decoded));
            }
            if let Ok(decoded) =
                <DefaultImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::DefaultImplementation(decoded));
            }
            if let Ok(decoded) =
                <DefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::DefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <DisableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::DisableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <EnableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::EnableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <GetInstanceAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::GetInstanceAddress(decoded));
            }
            if let Ok(decoded) =
                <ImplementationOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::ImplementationOf(decoded));
            }
            if let Ok(decoded) =
                <MapleGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::MapleGlobals(decoded));
            }
            if let Ok(decoded) =
                <MigratorForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::MigratorForPath(decoded));
            }
            if let Ok(decoded) =
                <RegisterImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::RegisterImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::SetDefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::SetGlobals(decoded));
            }
            if let Ok(decoded) =
                <UpgradeEnabledForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::UpgradeEnabledForPath(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::UpgradeInstance(decoded));
            }
            if let Ok(decoded) =
                <VersionOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleProxyFactoryCalls::VersionOf(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMapleProxyFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMapleProxyFactoryCalls::CreateInstance(element) => element.encode(),
                IMapleProxyFactoryCalls::DefaultImplementation(element) => element.encode(),
                IMapleProxyFactoryCalls::DefaultVersion(element) => element.encode(),
                IMapleProxyFactoryCalls::DisableUpgradePath(element) => element.encode(),
                IMapleProxyFactoryCalls::EnableUpgradePath(element) => element.encode(),
                IMapleProxyFactoryCalls::GetInstanceAddress(element) => element.encode(),
                IMapleProxyFactoryCalls::ImplementationOf(element) => element.encode(),
                IMapleProxyFactoryCalls::MapleGlobals(element) => element.encode(),
                IMapleProxyFactoryCalls::MigratorForPath(element) => element.encode(),
                IMapleProxyFactoryCalls::RegisterImplementation(element) => element.encode(),
                IMapleProxyFactoryCalls::SetDefaultVersion(element) => element.encode(),
                IMapleProxyFactoryCalls::SetGlobals(element) => element.encode(),
                IMapleProxyFactoryCalls::UpgradeEnabledForPath(element) => element.encode(),
                IMapleProxyFactoryCalls::UpgradeInstance(element) => element.encode(),
                IMapleProxyFactoryCalls::VersionOf(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMapleProxyFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleProxyFactoryCalls::CreateInstance(element) => element.fmt(f),
                IMapleProxyFactoryCalls::DefaultImplementation(element) => element.fmt(f),
                IMapleProxyFactoryCalls::DefaultVersion(element) => element.fmt(f),
                IMapleProxyFactoryCalls::DisableUpgradePath(element) => element.fmt(f),
                IMapleProxyFactoryCalls::EnableUpgradePath(element) => element.fmt(f),
                IMapleProxyFactoryCalls::GetInstanceAddress(element) => element.fmt(f),
                IMapleProxyFactoryCalls::ImplementationOf(element) => element.fmt(f),
                IMapleProxyFactoryCalls::MapleGlobals(element) => element.fmt(f),
                IMapleProxyFactoryCalls::MigratorForPath(element) => element.fmt(f),
                IMapleProxyFactoryCalls::RegisterImplementation(element) => element.fmt(f),
                IMapleProxyFactoryCalls::SetDefaultVersion(element) => element.fmt(f),
                IMapleProxyFactoryCalls::SetGlobals(element) => element.fmt(f),
                IMapleProxyFactoryCalls::UpgradeEnabledForPath(element) => element.fmt(f),
                IMapleProxyFactoryCalls::UpgradeInstance(element) => element.fmt(f),
                IMapleProxyFactoryCalls::VersionOf(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateInstanceCall> for IMapleProxyFactoryCalls {
        fn from(var: CreateInstanceCall) -> Self {
            IMapleProxyFactoryCalls::CreateInstance(var)
        }
    }
    impl ::std::convert::From<DefaultImplementationCall> for IMapleProxyFactoryCalls {
        fn from(var: DefaultImplementationCall) -> Self {
            IMapleProxyFactoryCalls::DefaultImplementation(var)
        }
    }
    impl ::std::convert::From<DefaultVersionCall> for IMapleProxyFactoryCalls {
        fn from(var: DefaultVersionCall) -> Self {
            IMapleProxyFactoryCalls::DefaultVersion(var)
        }
    }
    impl ::std::convert::From<DisableUpgradePathCall> for IMapleProxyFactoryCalls {
        fn from(var: DisableUpgradePathCall) -> Self {
            IMapleProxyFactoryCalls::DisableUpgradePath(var)
        }
    }
    impl ::std::convert::From<EnableUpgradePathCall> for IMapleProxyFactoryCalls {
        fn from(var: EnableUpgradePathCall) -> Self {
            IMapleProxyFactoryCalls::EnableUpgradePath(var)
        }
    }
    impl ::std::convert::From<GetInstanceAddressCall> for IMapleProxyFactoryCalls {
        fn from(var: GetInstanceAddressCall) -> Self {
            IMapleProxyFactoryCalls::GetInstanceAddress(var)
        }
    }
    impl ::std::convert::From<ImplementationOfCall> for IMapleProxyFactoryCalls {
        fn from(var: ImplementationOfCall) -> Self {
            IMapleProxyFactoryCalls::ImplementationOf(var)
        }
    }
    impl ::std::convert::From<MapleGlobalsCall> for IMapleProxyFactoryCalls {
        fn from(var: MapleGlobalsCall) -> Self {
            IMapleProxyFactoryCalls::MapleGlobals(var)
        }
    }
    impl ::std::convert::From<MigratorForPathCall> for IMapleProxyFactoryCalls {
        fn from(var: MigratorForPathCall) -> Self {
            IMapleProxyFactoryCalls::MigratorForPath(var)
        }
    }
    impl ::std::convert::From<RegisterImplementationCall> for IMapleProxyFactoryCalls {
        fn from(var: RegisterImplementationCall) -> Self {
            IMapleProxyFactoryCalls::RegisterImplementation(var)
        }
    }
    impl ::std::convert::From<SetDefaultVersionCall> for IMapleProxyFactoryCalls {
        fn from(var: SetDefaultVersionCall) -> Self {
            IMapleProxyFactoryCalls::SetDefaultVersion(var)
        }
    }
    impl ::std::convert::From<SetGlobalsCall> for IMapleProxyFactoryCalls {
        fn from(var: SetGlobalsCall) -> Self {
            IMapleProxyFactoryCalls::SetGlobals(var)
        }
    }
    impl ::std::convert::From<UpgradeEnabledForPathCall> for IMapleProxyFactoryCalls {
        fn from(var: UpgradeEnabledForPathCall) -> Self {
            IMapleProxyFactoryCalls::UpgradeEnabledForPath(var)
        }
    }
    impl ::std::convert::From<UpgradeInstanceCall> for IMapleProxyFactoryCalls {
        fn from(var: UpgradeInstanceCall) -> Self {
            IMapleProxyFactoryCalls::UpgradeInstance(var)
        }
    }
    impl ::std::convert::From<VersionOfCall> for IMapleProxyFactoryCalls {
        fn from(var: VersionOfCall) -> Self {
            IMapleProxyFactoryCalls::VersionOf(var)
        }
    }
}
