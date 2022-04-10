pub use baseadminupgradeabilityproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod baseadminupgradeabilityproxy_mod {
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
    #[doc = "BaseAdminUpgradeabilityProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BASEADMINUPGRADEABILITYPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BASEADMINUPGRADEABILITYPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b506106be806100206000396000f3fe60806040526004361061004a5760003560e01c80633659cfe6146100545780634f1ef286146100745780635c60da1b146100875780638f283970146100b8578063f851a440146100d8575b6100526100ed565b005b34801561006057600080fd5b5061005261006f3660046105b3565b610127565b6100526100823660046105d5565b610164565b34801561009357600080fd5b5061009c610213565b6040516001600160a01b03909116815260200160405180910390f35b3480156100c457600080fd5b506100526100d33660046105b3565b610275565b3480156100e457600080fd5b5061009c610387565b6100f56103cc565b6101256101207f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b610458565b565b600080516020610669833981519152546001600160a01b0316336001600160a01b0316141561015c576101598161047c565b50565b6101596100ed565b600080516020610669833981519152546001600160a01b0316336001600160a01b03161415610206576101968361047c565b6000836001600160a01b031683836040516101b2929190610658565b600060405180830381855af49150503d80600081146101ed576040519150601f19603f3d011682016040523d82523d6000602084013e6101f2565b606091505b505090508061020057600080fd5b50505050565b61020e6100ed565b505050565b600061022b6000805160206106698339815191525490565b6001600160a01b0316336001600160a01b0316141561026a57507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b6102726100ed565b90565b600080516020610669833981519152546001600160a01b0316336001600160a01b0316141561015c576001600160a01b0381166103185760405162461bcd60e51b815260206004820152603660248201527f43616e6e6f74206368616e6765207468652061646d696e206f6620612070726f604482015275787920746f20746865207a65726f206164647265737360501b60648201526084015b60405180910390fd5b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f61034f6000805160206106698339815191525490565b604080516001600160a01b03928316815291841660208301520160405180910390a16101598160008051602061066983398151915255565b600061039f6000805160206106698339815191525490565b6001600160a01b0316336001600160a01b0316141561026a57506000805160206106698339815191525490565b600080516020610669833981519152546001600160a01b0316336001600160a01b031614156101255760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b606482015260840161030f565b3660008037600080366000845af43d6000803e808015610477573d6000f35b3d6000fd5b610485816104bc565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b6104c58161055b565b6105375760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e747261637420616464726573730000000000606482015260840161030f565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061058f57508115155b949350505050565b80356001600160a01b03811681146105ae57600080fd5b919050565b6000602082840312156105c557600080fd5b6105ce82610597565b9392505050565b6000806000604084860312156105ea57600080fd5b6105f384610597565b9250602084013567ffffffffffffffff8082111561061057600080fd5b818601915086601f83011261062457600080fd5b81358181111561063357600080fd5b87602082850101111561064557600080fd5b6020830194508093505050509250925092565b818382376000910190815291905056feb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103a26469706673582212203ae61454bcc4f7029e55939c51da1c89c8aa4b00ed6cbb8fa0aacbb9ee24bac164736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct BaseAdminUpgradeabilityProxy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BaseAdminUpgradeabilityProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BaseAdminUpgradeabilityProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BaseAdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BaseAdminUpgradeabilityProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                BASEADMINUPGRADEABILITYPROXY_ABI.clone(),
                client,
            )
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
                BASEADMINUPGRADEABILITYPROXY_ABI.clone(),
                BASEADMINUPGRADEABILITYPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeAdmin` (0x8f283970) function"]
        pub fn change_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 40, 57, 112], new_admin)
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
        #[doc = "Calls the contract's `upgradeTo` (0x3659cfe6) function"]
        pub fn upgrade_to(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 89, 207, 230], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeToAndCall` (0x4f1ef286) function"]
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, BaseAdminUpgradeabilityProxyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for BaseAdminUpgradeabilityProxy<M>
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BaseAdminUpgradeabilityProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for BaseAdminUpgradeabilityProxyEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(BaseAdminUpgradeabilityProxyEvents::AdminChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(BaseAdminUpgradeabilityProxyEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BaseAdminUpgradeabilityProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseAdminUpgradeabilityProxyEvents::AdminChangedFilter(element) => element.fmt(f),
                BaseAdminUpgradeabilityProxyEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `changeAdmin`function with signature `changeAdmin(address)` and selector `[143, 40, 57, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeAdmin", abi = "changeAdmin(address)")]
    pub struct ChangeAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `upgradeTo`function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address)")]
    pub struct UpgradeToCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeToAndCall`function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BaseAdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        ChangeAdmin(ChangeAdminCall),
        Implementation(ImplementationCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for BaseAdminUpgradeabilityProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseAdminUpgradeabilityProxyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ChangeAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseAdminUpgradeabilityProxyCalls::ChangeAdmin(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseAdminUpgradeabilityProxyCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseAdminUpgradeabilityProxyCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseAdminUpgradeabilityProxyCalls::UpgradeToAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BaseAdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BaseAdminUpgradeabilityProxyCalls::Admin(element) => element.encode(),
                BaseAdminUpgradeabilityProxyCalls::ChangeAdmin(element) => element.encode(),
                BaseAdminUpgradeabilityProxyCalls::Implementation(element) => element.encode(),
                BaseAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.encode(),
                BaseAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BaseAdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseAdminUpgradeabilityProxyCalls::Admin(element) => element.fmt(f),
                BaseAdminUpgradeabilityProxyCalls::ChangeAdmin(element) => element.fmt(f),
                BaseAdminUpgradeabilityProxyCalls::Implementation(element) => element.fmt(f),
                BaseAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.fmt(f),
                BaseAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdminCall> for BaseAdminUpgradeabilityProxyCalls {
        fn from(var: AdminCall) -> Self {
            BaseAdminUpgradeabilityProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ChangeAdminCall> for BaseAdminUpgradeabilityProxyCalls {
        fn from(var: ChangeAdminCall) -> Self {
            BaseAdminUpgradeabilityProxyCalls::ChangeAdmin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for BaseAdminUpgradeabilityProxyCalls {
        fn from(var: ImplementationCall) -> Self {
            BaseAdminUpgradeabilityProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for BaseAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            BaseAdminUpgradeabilityProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for BaseAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            BaseAdminUpgradeabilityProxyCalls::UpgradeToAndCall(var)
        }
    }
}
