pub use initializableimmutableadminupgradeabilityproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod initializableimmutableadminupgradeabilityproxy_mod {
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
    #[doc = "InitializableImmutableAdminUpgradeabilityProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static INITIALIZABLEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_logic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    #[doc = r" Bytecode of the #name contract"]
    pub static INITIALIZABLEIMMUTABLEADMINUPGRADEABILITYPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60a060405234801561001057600080fd5b5060405161088d38038061088d83398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b6080516107df6100ae600039600081816101130152818161015801528181610211015281816103510152818161037a01526104a501526107df6000f3fe60806040526004361061004a5760003560e01c80633659cfe6146100545780634f1ef286146100745780635c60da1b14610087578063d1f57894146100b8578063f851a440146100cb575b6100526100e0565b005b34801561006057600080fd5b5061005261006f366004610586565b610108565b6100526100823660046105a8565b61014d565b34801561009357600080fd5b5061009c610204565b6040516001600160a01b03909116815260200160405180910390f35b6100526100c6366004610641565b610256565b3480156100d757600080fd5b5061009c610344565b6100e861039c565b61010661010160008051602061078a8339815191525490565b6103a4565b565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561014557610142816103c8565b50565b6101426100e0565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101f757610187836103c8565b6000836001600160a01b031683836040516101a3929190610703565b600060405180830381855af49150503d80600081146101de576040519150601f19603f3d011682016040523d82523d6000602084013e6101e3565b606091505b50509050806101f157600080fd5b50505050565b6101ff6100e0565b505050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561024b575060008051602061078a8339815191525490565b6102536100e0565b90565b600061026e60008051602061078a8339815191525490565b6001600160a01b03161461028157600080fd5b6102ac60017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd610713565b60008051602061078a833981519152146102c8576102c8610738565b6102d182610408565b805115610340576000826001600160a01b0316826040516102f2919061074e565b600060405180830381855af49150503d806000811461032d576040519150601f19603f3d011682016040523d82523d6000602084013e610332565b606091505b50509050806101ff57600080fd5b5050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561024b57507f000000000000000000000000000000000000000000000000000000000000000090565b61010661049a565b3660008037600080366000845af43d6000803e8080156103c3573d6000f35b3d6000fd5b6103d181610408565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b6104118161052e565b6104885760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e74726163742061646472657373000000000060648201526084015b60405180910390fd5b60008051602061078a83398151915255565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101065760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b606482015260840161047f565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061056257508115155b949350505050565b80356001600160a01b038116811461058157600080fd5b919050565b60006020828403121561059857600080fd5b6105a18261056a565b9392505050565b6000806000604084860312156105bd57600080fd5b6105c68461056a565b9250602084013567ffffffffffffffff808211156105e357600080fd5b818601915086601f8301126105f757600080fd5b81358181111561060657600080fd5b87602082850101111561061857600080fd5b6020830194508093505050509250925092565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561065457600080fd5b61065d8361056a565b9150602083013567ffffffffffffffff8082111561067a57600080fd5b818501915085601f83011261068e57600080fd5b8135818111156106a0576106a061062b565b604051601f8201601f19908116603f011681019083821181831017156106c8576106c861062b565b816040528281528860208487010111156106e157600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b8183823760009101908152919050565b60008282101561073357634e487b7160e01b600052601160045260246000fd5b500390565b634e487b7160e01b600052600160045260246000fd5b6000825160005b8181101561076f5760208186018101518583015201610755565b8181111561077e576000828501525b50919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca264697066735822122035ff0e387ac5cbedddb5f9eb27906f62e214188bc377e7c565d3483bff5ea83364736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct InitializableImmutableAdminUpgradeabilityProxy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for InitializableImmutableAdminUpgradeabilityProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for InitializableImmutableAdminUpgradeabilityProxy<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(InitializableImmutableAdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> InitializableImmutableAdminUpgradeabilityProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                INITIALIZABLEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
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
                INITIALIZABLEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
                INITIALIZABLEIMMUTABLEADMINUPGRADEABILITYPROXY_BYTECODE
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
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
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
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for InitializableImmutableAdminUpgradeabilityProxy<M>
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,bytes)` and selector `[209, 245, 120, 148]`"]
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
    pub enum InitializableImmutableAdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        Implementation(ImplementationCall),
        Initialize(InitializeCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableImmutableAdminUpgradeabilityProxyCalls::Admin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(
                    InitializableImmutableAdminUpgradeabilityProxyCalls::Implementation(decoded),
                );
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(
                    InitializableImmutableAdminUpgradeabilityProxyCalls::Initialize(decoded),
                );
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(decoded));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(
                    InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                InitializableImmutableAdminUpgradeabilityProxyCalls::Admin(element) => {
                    element.encode()
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.encode()
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::Initialize(element) => {
                    element.encode()
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => {
                    element.encode()
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                InitializableImmutableAdminUpgradeabilityProxyCalls::Admin(element) => {
                    element.fmt(f)
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.fmt(f)
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::Initialize(element) => {
                    element.fmt(f)
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => {
                    element.fmt(f)
                }
                InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AdminCall> for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: AdminCall) -> Self {
            InitializableImmutableAdminUpgradeabilityProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall>
        for InitializableImmutableAdminUpgradeabilityProxyCalls
    {
        fn from(var: ImplementationCall) -> Self {
            InitializableImmutableAdminUpgradeabilityProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: InitializeCall) -> Self {
            InitializableImmutableAdminUpgradeabilityProxyCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for InitializableImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall>
        for InitializableImmutableAdminUpgradeabilityProxyCalls
    {
        fn from(var: UpgradeToAndCallCall) -> Self {
            InitializableImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(var)
        }
    }
}
