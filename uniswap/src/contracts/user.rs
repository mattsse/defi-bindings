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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_greeter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"gm\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"greeting\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"greet\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static USER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516102fd3803806102fd83398101604081905261002f91610054565b600080546001600160a01b0319166001600160a01b0392909216919091179055610084565b60006020828403121561006657600080fd5b81516001600160a01b038116811461007d57600080fd5b9392505050565b61026a806100936000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063c0129d431461003b578063ead710c414610045575b600080fd5b610043610058565b005b61004361005336600461012e565b6100b3565b600080546040805163c0129d4360e01b815290516001600160a01b039092169263c0129d439260048084019382900301818387803b15801561009957600080fd5b505af11580156100ad573d6000803e3d6000fd5b50505050565b600054604051633ab5c43160e21b81526001600160a01b039091169063ead710c4906100e39084906004016101df565b600060405180830381600087803b1580156100fd57600080fd5b505af1158015610111573d6000803e3d6000fd5b5050505050565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561014057600080fd5b813567ffffffffffffffff8082111561015857600080fd5b818401915084601f83011261016c57600080fd5b81358181111561017e5761017e610118565b604051601f8201601f19908116603f011681019083821181831017156101a6576101a6610118565b816040528281528760208487010111156101bf57600080fd5b826020860160208301376000928101602001929092525095945050505050565b600060208083528351808285015260005b8181101561020c578581018301518582016040015282016101f0565b8181111561021e576000604083870101525b50601f01601f191692909201604001939250505056fea264697066735822122049ff2658e52a98cbd947f398cfa42dfa7d701f70b871200a7a1ffbbb8af77a7b64736f6c634300080d0033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `gm` (0xc0129d43) function"]
        pub fn gm(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 18, 157, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `greet` (0xead710c4) function"]
        pub fn greet(&self, greeting: String) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 215, 16, 196], greeting)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for User<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `gm`function with signature `gm()` and selector `[192, 18, 157, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gm", abi = "gm()")]
    pub struct GmCall;
    #[doc = "Container type for all input parameters for the `greet`function with signature `greet(string)` and selector `[234, 215, 16, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "greet", abi = "greet(string)")]
    pub struct GreetCall {
        pub greeting: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UserCalls {
        Gm(GmCall),
        Greet(GreetCall),
    }
    impl ethers::core::abi::AbiDecode for UserCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <GmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UserCalls::Gm(decoded));
            }
            if let Ok(decoded) = <GreetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UserCalls::Greet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UserCalls::Gm(element) => element.encode(),
                UserCalls::Greet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UserCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UserCalls::Gm(element) => element.fmt(f),
                UserCalls::Greet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GmCall> for UserCalls {
        fn from(var: GmCall) -> Self {
            UserCalls::Gm(var)
        }
    }
    impl ::std::convert::From<GreetCall> for UserCalls {
        fn from(var: GreetCall) -> Self {
            UserCalls::Greet(var)
        }
    }
}
