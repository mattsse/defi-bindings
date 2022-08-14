pub use mock_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_pool {
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
    #[doc = "MockPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addReserveToReservesList\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReservesList\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKPOOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506102cb806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063c4d66de814610046578063d1946dbc14610078578063e636a4f414610096575b600080fd5b6100766100543660046101c3565b606480546001600160a01b0319166001600160a01b0392909216919091179055565b005b6100806100f6565b60405161008d91906101f3565b60405180910390f35b6100766100a43660046101c3565b606580546001810182556000919091527f8ff97419363ffd7000167f130ef7168fbea05faf9251824ca5043f113cc6a7c70180546001600160a01b0319166001600160a01b0392909216919091179055565b60655460609060009067ffffffffffffffff81111561011757610117610240565b604051908082528060200260200182016040528015610140578160200160208202803683370190505b50905060005b6065548110156101bd576065818154811061016357610163610256565b9060005260206000200160009054906101000a90046001600160a01b031682828151811061019357610193610256565b6001600160a01b0390921660209283029190910190910152806101b58161026c565b915050610146565b50919050565b6000602082840312156101d557600080fd5b81356001600160a01b03811681146101ec57600080fd5b9392505050565b6020808252825182820181905260009190848201906040850190845b818110156102345783516001600160a01b03168352928401929184019160010161020f565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b600060001982141561028e57634e487b7160e01b600052601160045260246000fd5b506001019056fea2646970667358221220e6534a7a155f715d740e4f7c87059de89c20117f48efbfeaa8b2d9768342128064736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockPool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockPool<M> {
        fn clone(&self) -> Self {
            MockPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKPOOL_ABI.clone(), client).into()
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
                MOCKPOOL_ABI.clone(),
                MOCKPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addReserveToReservesList` (0xe636a4f4) function"]
        pub fn add_reserve_to_reserves_list(
            &self,
            reserve: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 54, 164, 244], reserve)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReservesList` (0xd1946dbc) function"]
        pub fn get_reserves_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([209, 148, 109, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], provider)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MockPool<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addReserveToReservesList` function with signature `addReserveToReservesList(address)` and selector `[230, 54, 164, 244]`"]
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
        name = "addReserveToReservesList",
        abi = "addReserveToReservesList(address)"
    )]
    pub struct AddReserveToReservesListCall {
        pub reserve: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReservesList` function with signature `getReservesList()` and selector `[209, 148, 109, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReservesList", abi = "getReservesList()")]
    pub struct GetReservesListCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `[196, 214, 109, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub provider: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockPoolCalls {
        AddReserveToReservesList(AddReserveToReservesListCall),
        GetReservesList(GetReservesListCall),
        Initialize(InitializeCall),
    }
    impl ethers::core::abi::AbiDecode for MockPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddReserveToReservesListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockPoolCalls::AddReserveToReservesList(decoded));
            }
            if let Ok(decoded) =
                <GetReservesListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPoolCalls::GetReservesList(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPoolCalls::Initialize(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockPoolCalls::AddReserveToReservesList(element) => element.encode(),
                MockPoolCalls::GetReservesList(element) => element.encode(),
                MockPoolCalls::Initialize(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockPoolCalls::AddReserveToReservesList(element) => element.fmt(f),
                MockPoolCalls::GetReservesList(element) => element.fmt(f),
                MockPoolCalls::Initialize(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddReserveToReservesListCall> for MockPoolCalls {
        fn from(var: AddReserveToReservesListCall) -> Self {
            MockPoolCalls::AddReserveToReservesList(var)
        }
    }
    impl ::std::convert::From<GetReservesListCall> for MockPoolCalls {
        fn from(var: GetReservesListCall) -> Self {
            MockPoolCalls::GetReservesList(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockPoolCalls {
        fn from(var: InitializeCall) -> Self {
            MockPoolCalls::Initialize(var)
        }
    }
    #[doc = "Container type for all return fields from the `getReservesList` function with signature `getReservesList()` and selector `[209, 148, 109, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReservesListReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
}
