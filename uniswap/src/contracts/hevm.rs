pub use hevm_mod::*;
#[allow(clippy::too_many_arguments)]
mod hevm_mod {
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
    #[doc = "Hevm was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static HEVM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string[]\",\"name\":\"\",\"type\":\"string[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ffi\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"roll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"c\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"loc\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"store\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"warp\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HEVM_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct Hevm<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Hevm<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Hevm<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Hevm))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Hevm<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), HEVM_ABI.clone(), client).into()
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
                HEVM_ABI.clone(),
                HEVM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ffi` (0x89160467) function"]
        pub fn ffi(
            &self,
            p0: ::std::vec::Vec<String>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([137, 22, 4, 103], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roll` (0x1f7b4f30) function"]
        pub fn roll(
            &self,
            x: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 123, 79, 48], x)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `store` (0x70ca10bb) function"]
        pub fn store(
            &self,
            c: ethers::core::types::Address,
            loc: [u8; 32],
            val: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 202, 16, 187], (c, loc, val))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `warp` (0xe5d6bf02) function"]
        pub fn warp(
            &self,
            x: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 214, 191, 2], x)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Hevm<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ffi`function with signature `ffi(string[])` and selector `[137, 22, 4, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ffi", abi = "ffi(string[])")]
    pub struct FfiCall(pub ::std::vec::Vec<String>);
    #[doc = "Container type for all input parameters for the `roll`function with signature `roll(uint256)` and selector `[31, 123, 79, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roll", abi = "roll(uint256)")]
    pub struct RollCall {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `store`function with signature `store(address,bytes32,bytes32)` and selector `[112, 202, 16, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "store", abi = "store(address,bytes32,bytes32)")]
    pub struct StoreCall {
        pub c: ethers::core::types::Address,
        pub loc: [u8; 32],
        pub val: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `warp`function with signature `warp(uint256)` and selector `[229, 214, 191, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "warp", abi = "warp(uint256)")]
    pub struct WarpCall {
        pub x: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum HevmCalls {
        Ffi(FfiCall),
        Roll(RollCall),
        Store(StoreCall),
        Warp(WarpCall),
    }
    impl ethers::core::abi::AbiDecode for HevmCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <FfiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(HevmCalls::Ffi(decoded));
            }
            if let Ok(decoded) = <RollCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(HevmCalls::Roll(decoded));
            }
            if let Ok(decoded) = <StoreCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HevmCalls::Store(decoded));
            }
            if let Ok(decoded) = <WarpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(HevmCalls::Warp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for HevmCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                HevmCalls::Ffi(element) => element.encode(),
                HevmCalls::Roll(element) => element.encode(),
                HevmCalls::Store(element) => element.encode(),
                HevmCalls::Warp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HevmCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HevmCalls::Ffi(element) => element.fmt(f),
                HevmCalls::Roll(element) => element.fmt(f),
                HevmCalls::Store(element) => element.fmt(f),
                HevmCalls::Warp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FfiCall> for HevmCalls {
        fn from(var: FfiCall) -> Self {
            HevmCalls::Ffi(var)
        }
    }
    impl ::std::convert::From<RollCall> for HevmCalls {
        fn from(var: RollCall) -> Self {
            HevmCalls::Roll(var)
        }
    }
    impl ::std::convert::From<StoreCall> for HevmCalls {
        fn from(var: StoreCall) -> Self {
            HevmCalls::Store(var)
        }
    }
    impl ::std::convert::From<WarpCall> for HevmCalls {
        fn from(var: WarpCall) -> Self {
            HevmCalls::Warp(var)
        }
    }
}
