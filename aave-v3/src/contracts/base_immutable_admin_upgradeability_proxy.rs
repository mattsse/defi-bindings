pub use baseimmutableadminupgradeabilityproxy_mod::*;
#[allow(clippy::too_many_arguments)]
mod baseimmutableadminupgradeabilityproxy_mod {
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
    #[doc = "BaseImmutableAdminUpgradeabilityProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BASEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"upgradeToAndCall\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    #[doc = r" Bytecode of the #name contract"]
    pub static BASEIMMUTABLEADMINUPGRADEABILITYPROXY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60a060405234801561001057600080fd5b5060405161064138038061064183398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b6080516105936100ae600039600081816101070152818161014c01528181610205015281816102690152818161029201526102bf01526105936000f3fe60806040526004361061003f5760003560e01c80633659cfe6146100495780634f1ef286146100695780635c60da1b1461007c578063f851a440146100ad575b6100476100c2565b005b34801561005557600080fd5b506100476100643660046104a8565b6100fc565b6100476100773660046104ca565b610141565b34801561008857600080fd5b506100916101f8565b6040516001600160a01b03909116815260200160405180910390f35b3480156100b957600080fd5b5061009161025c565b6100ca6102b4565b6100fa6100f57f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b61034d565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101395761013681610371565b50565b6101366100c2565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101eb5761017b83610371565b6000836001600160a01b0316838360405161019792919061054d565b600060405180830381855af49150503d80600081146101d2576040519150601f19603f3d011682016040523d82523d6000602084013e6101d7565b606091505b50509050806101e557600080fd5b50505050565b6101f36100c2565b505050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561025157507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b6102596100c2565b90565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561025157507f000000000000000000000000000000000000000000000000000000000000000090565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156100fa5760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b60648201526084015b60405180910390fd5b3660008037600080366000845af43d6000803e80801561036c573d6000f35b3d6000fd5b61037a816103b1565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b6103ba81610450565b61042c5760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e7472616374206164647265737300000000006064820152608401610344565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061048457508115155b949350505050565b80356001600160a01b03811681146104a357600080fd5b919050565b6000602082840312156104ba57600080fd5b6104c38261048c565b9392505050565b6000806000604084860312156104df57600080fd5b6104e88461048c565b9250602084013567ffffffffffffffff8082111561050557600080fd5b818601915086601f83011261051957600080fd5b81358181111561052857600080fd5b87602082850101111561053a57600080fd5b6020830194508093505050509250925092565b818382376000910190815291905056fea26469706673582212206ca3b390132d074d3e611c80b4df50af95ef7ddf1f329acb1ab6e97c156124a564736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct BaseImmutableAdminUpgradeabilityProxy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BaseImmutableAdminUpgradeabilityProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for BaseImmutableAdminUpgradeabilityProxy<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BaseImmutableAdminUpgradeabilityProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BaseImmutableAdminUpgradeabilityProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                BASEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
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
                BASEIMMUTABLEADMINUPGRADEABILITYPROXY_ABI.clone(),
                BASEIMMUTABLEADMINUPGRADEABILITYPROXY_BYTECODE
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
        for BaseImmutableAdminUpgradeabilityProxy<M>
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
    pub enum BaseImmutableAdminUpgradeabilityProxyCalls {
        Admin(AdminCall),
        Implementation(ImplementationCall),
        UpgradeTo(UpgradeToCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
    }
    impl ethers::core::abi::AbiDecode for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseImmutableAdminUpgradeabilityProxyCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseImmutableAdminUpgradeabilityProxyCalls::Implementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpgradeToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpgradeToAndCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BaseImmutableAdminUpgradeabilityProxyCalls::Admin(element) => element.encode(),
                BaseImmutableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.encode()
                }
                BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.encode(),
                BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseImmutableAdminUpgradeabilityProxyCalls::Admin(element) => element.fmt(f),
                BaseImmutableAdminUpgradeabilityProxyCalls::Implementation(element) => {
                    element.fmt(f)
                }
                BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(element) => element.fmt(f),
                BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AdminCall> for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: AdminCall) -> Self {
            BaseImmutableAdminUpgradeabilityProxyCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: ImplementationCall) -> Self {
            BaseImmutableAdminUpgradeabilityProxyCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<UpgradeToCall> for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToCall) -> Self {
            BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeTo(var)
        }
    }
    impl ::std::convert::From<UpgradeToAndCallCall> for BaseImmutableAdminUpgradeabilityProxyCalls {
        fn from(var: UpgradeToAndCallCall) -> Self {
            BaseImmutableAdminUpgradeabilityProxyCalls::UpgradeToAndCall(var)
        }
    }
}
