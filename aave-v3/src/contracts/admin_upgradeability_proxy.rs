pub use adminupgradeabilityproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod adminupgradeabilityproxy_mod {
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
    #[doc = "AdminUpgradeabilityProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ADMINUPGRADEABILITYPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_logic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ADMINUPGRADEABILITYPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405260405162000afb38038062000afb8339810160408190526200002691620002ab565b82816200005560017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd6200038b565b60008051602062000adb83398151915214620000755762000075620003b1565b620000808262000168565b805115620000f7576000826001600160a01b031682604051620000a49190620003c7565b600060405180830381855af49150503d8060008114620000e1576040519150601f19603f3d011682016040523d82523d6000602084013e620000e6565b606091505b5050905080620000f557600080fd5b505b5062000127905060017fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d61046200038b565b60008051602062000abb83398151915214620001475762000147620003b1565b6200015f8260008051602062000abb83398151915255565b505050620003e5565b6200017e816200020860201b620003cc1760201c565b620001f55760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e747261637420616464726573730000000000606482015260840160405180910390fd5b60008051602062000adb83398151915255565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4708181148015906200023d57508115155b949350505050565b80516001600160a01b03811681146200025d57600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b60005b83811015620002955781810151838201526020016200027b565b83811115620002a5576000848401525b50505050565b600080600060608486031215620002c157600080fd5b620002cc8462000245565b9250620002dc6020850162000245565b60408501519092506001600160401b0380821115620002fa57600080fd5b818601915086601f8301126200030f57600080fd5b81518181111562000324576200032462000262565b604051601f8201601f19908116603f011681019083821181831017156200034f576200034f62000262565b816040528281528960208487010111156200036957600080fd5b6200037c83602083016020880162000278565b80955050505050509250925092565b600082821015620003ac57634e487b7160e01b600052601160045260246000fd5b500390565b634e487b7160e01b600052600160045260246000fd5b60008251620003db81846020870162000278565b9190910192915050565b6106c680620003f56000396000f3fe60806040526004361061004a5760003560e01c80633659cfe6146100545780634f1ef286146100745780635c60da1b146100875780638f283970146100b8578063f851a440146100d8575b6100526100ed565b005b34801561006057600080fd5b5061005261006f3660046105bb565b610127565b6100526100823660046105dd565b610164565b34801561009357600080fd5b5061009c610213565b6040516001600160a01b03909116815260200160405180910390f35b3480156100c457600080fd5b506100526100d33660046105bb565b610275565b3480156100e457600080fd5b5061009c610387565b6100f5610408565b6101256101207f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b610410565b565b600080516020610671833981519152546001600160a01b0316336001600160a01b0316141561015c5761015981610434565b50565b6101596100ed565b600080516020610671833981519152546001600160a01b0316336001600160a01b031614156102065761019683610434565b6000836001600160a01b031683836040516101b2929190610660565b600060405180830381855af49150503d80600081146101ed576040519150601f19603f3d011682016040523d82523d6000602084013e6101f2565b606091505b505090508061020057600080fd5b50505050565b61020e6100ed565b505050565b600061022b6000805160206106718339815191525490565b6001600160a01b0316336001600160a01b0316141561026a57507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b6102726100ed565b90565b600080516020610671833981519152546001600160a01b0316336001600160a01b0316141561015c576001600160a01b0381166103185760405162461bcd60e51b815260206004820152603660248201527f43616e6e6f74206368616e6765207468652061646d696e206f6620612070726f604482015275787920746f20746865207a65726f206164647265737360501b60648201526084015b60405180910390fd5b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f61034f6000805160206106718339815191525490565b604080516001600160a01b03928316815291841660208301520160405180910390a16101598160008051602061067183398151915255565b600061039f6000805160206106718339815191525490565b6001600160a01b0316336001600160a01b0316141561026a57506000805160206106718339815191525490565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061040057508115155b949350505050565b610125610474565b3660008037600080366000845af43d6000803e80801561042f573d6000f35b3d6000fd5b61043d81610500565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b600080516020610671833981519152546001600160a01b0316336001600160a01b031614156101255760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b606482015260840161030f565b610509816103cc565b61057b5760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e747261637420616464726573730000000000606482015260840161030f565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b80356001600160a01b03811681146105b657600080fd5b919050565b6000602082840312156105cd57600080fd5b6105d68261059f565b9392505050565b6000806000604084860312156105f257600080fd5b6105fb8461059f565b9250602084013567ffffffffffffffff8082111561061857600080fd5b818601915086601f83011261062c57600080fd5b81358181111561063b57600080fd5b87602082850101111561064d57600080fd5b6020830194508093505050509250925092565b818382376000910190815291905056feb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103a264697066735822122073e149d62b7d18f5879e16a606d8f03a344b1a413710116953bccd8202f9332764736f6c634300080a0033b53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct AdminUpgradeabilityProxy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AdminUpgradeabilityProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AdminUpgradeabilityProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AdminUpgradeabilityProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ADMINUPGRADEABILITYPROXY_ABI.clone(),
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
                ADMINUPGRADEABILITYPROXY_ABI.clone(),
                ADMINUPGRADEABILITYPROXY_BYTECODE.clone().into(),
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
        ) -> ethers::contract::builders::Event<M, AdminUpgradeabilityProxyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for AdminUpgradeabilityProxy<M>
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
    pub enum AdminUpgradeabilityProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for AdminUpgradeabilityProxyEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(AdminUpgradeabilityProxyEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(AdminUpgradeabilityProxyEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AdminUpgradeabilityProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AdminUpgradeabilityProxyEvents::AdminChangedFilter(element) => element.fmt(f),
                AdminUpgradeabilityProxyEvents::UpgradedFilter(element) => element.fmt(f),
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
    pub enum AdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        ChangeAdmin(ChangeAdminCall),
        Implementation(ImplementationCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for AdminUpgradeabilityProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminUpgradeabilityProxyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ChangeAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminUpgradeabilityProxyCalls::ChangeAdmin(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminUpgradeabilityProxyCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminUpgradeabilityProxyCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AdminUpgradeabilityProxyCalls::UpgradeToAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AdminUpgradeabilityProxyCalls::Admin(element) => element.encode(),
                AdminUpgradeabilityProxyCalls::ChangeAdmin(element) => element.encode(),
                AdminUpgradeabilityProxyCalls::Implementation(element) => element.encode(),
                AdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.encode(),
                AdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AdminUpgradeabilityProxyCalls::Admin(element) => element.fmt(f),
                AdminUpgradeabilityProxyCalls::ChangeAdmin(element) => element.fmt(f),
                AdminUpgradeabilityProxyCalls::Implementation(element) => element.fmt(f),
                AdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.fmt(f),
                AdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdminCall> for AdminUpgradeabilityProxyCalls {
        fn from(var: AdminCall) -> Self {
            AdminUpgradeabilityProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ChangeAdminCall> for AdminUpgradeabilityProxyCalls {
        fn from(var: ChangeAdminCall) -> Self {
            AdminUpgradeabilityProxyCalls::ChangeAdmin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for AdminUpgradeabilityProxyCalls {
        fn from(var: ImplementationCall) -> Self {
            AdminUpgradeabilityProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for AdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            AdminUpgradeabilityProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for AdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            AdminUpgradeabilityProxyCalls::UpgradeToAndCall(var)
        }
    }
}
