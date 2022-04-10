pub use juglike_mod::*;
#[allow(clippy::too_many_arguments)]
mod juglike_mod {
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
    #[doc = "JugLike was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static JUGLIKE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"base\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ilks\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"duty\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rho\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static JUGLIKE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060d68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c80635001f3b5146037578063d9638d3614604f575b600080fd5b603d6082565b60408051918252519081900360200190f35b606960048036036020811015606357600080fd5b50356088565b6040805192835260208301919091528051918290030190f35b60015481565b600060208190529081526040902080546001909101548256fea265627a7a7231582088305a8005f65d20b95a58304a814d1e6c275d96198ed2d30b83e8d89629bb2364736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct JugLike<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for JugLike<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for JugLike<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(JugLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> JugLike<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), JUGLIKE_ABI.clone(), client).into()
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
                JUGLIKE_ABI.clone(),
                JUGLIKE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `base` (0x5001f3b5) function"]
        pub fn base(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([80, 1, 243, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ilks` (0xd9638d36) function"]
        pub fn ilks(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([217, 99, 141, 54], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for JugLike<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `base`function with signature `base()` and selector `[80, 1, 243, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "base", abi = "base()")]
    pub struct BaseCall;
    #[doc = "Container type for all input parameters for the `ilks`function with signature `ilks(bytes32)` and selector `[217, 99, 141, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ilks", abi = "ilks(bytes32)")]
    pub struct IlksCall(pub [u8; 32]);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum JugLikeCalls {
        Base(BaseCall),
        Ilks(IlksCall),
    }
    impl ethers::core::abi::AbiDecode for JugLikeCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BaseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(JugLikeCalls::Base(decoded));
            }
            if let Ok(decoded) = <IlksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(JugLikeCalls::Ilks(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for JugLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                JugLikeCalls::Base(element) => element.encode(),
                JugLikeCalls::Ilks(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for JugLikeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                JugLikeCalls::Base(element) => element.fmt(f),
                JugLikeCalls::Ilks(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BaseCall> for JugLikeCalls {
        fn from(var: BaseCall) -> Self {
            JugLikeCalls::Base(var)
        }
    }
    impl ::std::convert::From<IlksCall> for JugLikeCalls {
        fn from(var: IlksCall) -> Self {
            JugLikeCalls::Ilks(var)
        }
    }
}
