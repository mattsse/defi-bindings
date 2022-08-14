pub use initializable_admin_upgradeability_proxy::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod initializable_admin_upgradeability_proxy {
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
    #[doc = "InitializableAdminUpgradeabilityProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"logic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_logic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    #[doc = r" Bytecode of the #name contract"]
    pub static INITIALIZABLEADMINUPGRADEABILITYPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610a41806100206000396000f3fe6080604052600436106100705760003560e01c80638f2839701161004e5780638f283970146100de578063cf7a1d77146100fe578063d1f5789414610111578063f851a4401461012457610070565b80633659cfe61461007a5780634f1ef2861461009a5780635c60da1b146100ad575b610078610139565b005b34801561008657600080fd5b50610078610095366004610751565b610161565b6100786100a8366004610773565b61019e565b3480156100b957600080fd5b506100c261024d565b6040516001600160a01b03909116815260200160405180910390f35b3480156100ea57600080fd5b506100786100f9366004610751565b61029d565b61007861010c366004610899565b6103af565b61007861011f3660046108f7565b610441565b34801561013057600080fd5b506100c261052f565b610141610574565b61015f61015a6000805160206109ec8339815191525490565b61057c565b565b6000805160206109cc833981519152546001600160a01b0316336001600160a01b0316141561019657610193816105a0565b50565b610193610139565b6000805160206109cc833981519152546001600160a01b0316336001600160a01b03161415610240576101d0836105a0565b6000836001600160a01b031683836040516101ec929190610945565b600060405180830381855af49150503d8060008114610227576040519150601f19603f3d011682016040523d82523d6000602084013e61022c565b606091505b505090508061023a57600080fd5b50505050565b610248610139565b505050565b60006102656000805160206109cc8339815191525490565b6001600160a01b0316336001600160a01b0316141561029257506000805160206109ec8339815191525490565b61029a610139565b90565b6000805160206109cc833981519152546001600160a01b0316336001600160a01b03161415610196576001600160a01b0381166103405760405162461bcd60e51b815260206004820152603660248201527f43616e6e6f74206368616e6765207468652061646d696e206f6620612070726f604482015275787920746f20746865207a65726f206164647265737360501b60648201526084015b60405180910390fd5b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f6103776000805160206109cc8339815191525490565b604080516001600160a01b03928316815291841660208301520160405180910390a1610193816000805160206109cc83398151915255565b60006103c76000805160206109ec8339815191525490565b6001600160a01b0316146103da57600080fd5b6103e48382610441565b61040f60017fb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6104610955565b6000805160206109cc8339815191521461042b5761042b61097a565b610248826000805160206109cc83398151915255565b60006104596000805160206109ec8339815191525490565b6001600160a01b03161461046c57600080fd5b61049760017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd610955565b6000805160206109ec833981519152146104b3576104b361097a565b6104bc826105e0565b80511561052b576000826001600160a01b0316826040516104dd9190610990565b600060405180830381855af49150503d8060008114610518576040519150601f19603f3d011682016040523d82523d6000602084013e61051d565b606091505b505090508061024857600080fd5b5050565b60006105476000805160206109cc8339815191525490565b6001600160a01b0316336001600160a01b0316141561029257506000805160206109cc8339815191525490565b61015f61066d565b3660008037600080366000845af43d6000803e80801561059b573d6000f35b3d6000fd5b6105a9816105e0565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b6105e9816106f9565b61065b5760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e7472616374206164647265737300000000006064820152608401610337565b6000805160206109ec83398151915255565b6000805160206109cc833981519152546001600160a01b0316336001600160a01b0316141561015f5760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b6064820152608401610337565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061072d57508115155b949350505050565b80356001600160a01b038116811461074c57600080fd5b919050565b60006020828403121561076357600080fd5b61076c82610735565b9392505050565b60008060006040848603121561078857600080fd5b61079184610735565b9250602084013567ffffffffffffffff808211156107ae57600080fd5b818601915086601f8301126107c257600080fd5b8135818111156107d157600080fd5b8760208285010111156107e357600080fd5b6020830194508093505050509250925092565b634e487b7160e01b600052604160045260246000fd5b600082601f83011261081d57600080fd5b813567ffffffffffffffff80821115610838576108386107f6565b604051601f8301601f19908116603f01168101908282118183101715610860576108606107f6565b8160405283815286602085880101111561087957600080fd5b836020870160208301376000602085830101528094505050505092915050565b6000806000606084860312156108ae57600080fd5b6108b784610735565b92506108c560208501610735565b9150604084013567ffffffffffffffff8111156108e157600080fd5b6108ed8682870161080c565b9150509250925092565b6000806040838503121561090a57600080fd5b61091383610735565b9150602083013567ffffffffffffffff81111561092f57600080fd5b61093b8582860161080c565b9150509250929050565b8183823760009101908152919050565b60008282101561097557634e487b7160e01b600052601160045260246000fd5b500390565b634e487b7160e01b600052600160045260246000fd5b6000825160005b818110156109b15760208186018101518583015201610997565b818111156109c0576000828501525b50919091019291505056feb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca2646970667358221220aa1dede92c7f8d383b93b013fc50b64f9b2576fb415a24eaba946acd842f9b6264736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    pub struct InitializableAdminUpgradeabilityProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for InitializableAdminUpgradeabilityProxy<M> {
        fn clone(&self) -> Self {
            InitializableAdminUpgradeabilityProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for InitializableAdminUpgradeabilityProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for InitializableAdminUpgradeabilityProxy<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(InitializableAdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> InitializableAdminUpgradeabilityProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                INITIALIZABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
                INITIALIZABLEADMINUPGRADEABILITYPROXY_BYTECODE
                    .clone()
                    .into(),
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
        #[doc = "Calls the contract's `initialize` (0xcf7a1d77) function"]
        pub fn initialize_with_logic_and_admin_and_data(
            &self,
            logic: ethers::core::types::Address,
            admin: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 122, 29, 119], (logic, admin, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xd1f57894) function"]
        pub fn initialize(
            &self,
            logic: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 245, 120, 148], (logic, data))
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
        ) -> ethers::contract::builders::Event<M, InitializableAdminUpgradeabilityProxyEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for InitializableAdminUpgradeabilityProxy<M>
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
    pub enum InitializableAdminUpgradeabilityProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for InitializableAdminUpgradeabilityProxyEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(InitializableAdminUpgradeabilityProxyEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(InitializableAdminUpgradeabilityProxyEvents::UpgradedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for InitializableAdminUpgradeabilityProxyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                InitializableAdminUpgradeabilityProxyEvents::AdminChangedFilter(element) => {
                    element.fmt(f)
                }
                InitializableAdminUpgradeabilityProxyEvents::UpgradedFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
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
    #[doc = "Container type for all input parameters for the `changeAdmin` function with signature `changeAdmin(address)` and selector `[143, 40, 57, 112]`"]
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,bytes)` and selector `[207, 122, 29, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address,bytes)")]
    pub struct InitializeWithLogicAndAdminAndDataCall {
        pub logic: ethers::core::types::Address,
        pub admin: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,bytes)` and selector `[209, 245, 120, 148]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,bytes)")]
    pub struct InitializeCall {
        pub logic: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address)` and selector `[54, 89, 207, 230]`"]
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
    #[doc = "Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `[79, 30, 242, 134]`"]
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
    pub enum InitializableAdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        ChangeAdmin(ChangeAdminCall),
        Implementation(ImplementationCall),
        InitializeWithLogicAndAdminAndData(InitializeWithLogicAndAdminAndDataCall),
        Initialize(InitializeCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for InitializableAdminUpgradeabilityProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ChangeAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::ChangeAdmin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::Implementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitializeWithLogicAndAdminAndDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    InitializableAdminUpgradeabilityProxyCalls::InitializeWithLogicAndAdminAndData(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::Initialize(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::UpgradeTo(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for InitializableAdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                InitializableAdminUpgradeabilityProxyCalls::Admin(element) => element.encode(),
                InitializableAdminUpgradeabilityProxyCalls::ChangeAdmin(element) => {
                    element.encode()
                }
                InitializableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.encode()
                }
                InitializableAdminUpgradeabilityProxyCalls::InitializeWithLogicAndAdminAndData(
                    element,
                ) => element.encode(),
                InitializableAdminUpgradeabilityProxyCalls::Initialize(element) => element.encode(),
                InitializableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.encode(),
                InitializableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for InitializableAdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                InitializableAdminUpgradeabilityProxyCalls::Admin(element) => element.fmt(f),
                InitializableAdminUpgradeabilityProxyCalls::ChangeAdmin(element) => element.fmt(f),
                InitializableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.fmt(f)
                }
                InitializableAdminUpgradeabilityProxyCalls::InitializeWithLogicAndAdminAndData(
                    element,
                ) => element.fmt(f),
                InitializableAdminUpgradeabilityProxyCalls::Initialize(element) => element.fmt(f),
                InitializableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.fmt(f),
                InitializableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AdminCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: AdminCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ChangeAdminCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: ChangeAdminCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::ChangeAdmin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: ImplementationCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InitializeWithLogicAndAdminAndDataCall>
        for InitializableAdminUpgradeabilityProxyCalls
    {
        fn from(var: InitializeWithLogicAndAdminAndDataCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::InitializeWithLogicAndAdminAndData(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: InitializeCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for InitializableAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            InitializableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(var)
        }
    }
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
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
