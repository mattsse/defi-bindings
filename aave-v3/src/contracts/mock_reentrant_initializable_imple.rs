pub use mock_reentrant_initializable_imple::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_reentrant_initializable_imple {
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
    #[doc = "MockReentrantInitializableImple was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKREENTRANTINITIALIZABLEIMPLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REVISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"value\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKREENTRANTINITIALIZABLEIMPLE_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60806040526000805534801561001457600080fd5b506101ce806100246000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80633fa4f24514610046578063dde43cba14610061578063fe4b84df14610069575b600080fd5b61004f60345481565b60405190815260200160405180910390f35b61004f600281565b61007c610077366004610159565b61007e565b005b60015460029060ff16806100915750303b155b8061009d575060005481115b6101045760405162461bcd60e51b815260206004820152602e60248201527f436f6e747261637420696e7374616e63652068617320616c726561647920626560448201526d195b881a5b9a5d1a585b1a5e995960921b606482015260840160405180910390fd5b60015460ff16158015610123576001805460ff19168117905560008290555b603483905560028310156101435761014360345460016100779190610172565b8015610154576001805460ff191690555b505050565b60006020828403121561016b57600080fd5b5035919050565b6000821982111561019357634e487b7160e01b600052601160045260246000fd5b50019056fea2646970667358221220bdefdad5e19b514c100e7bd894a86132696d3065c5dadcc67d295d1f7da5c48164736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    pub struct MockReentrantInitializableImple<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockReentrantInitializableImple<M> {
        fn clone(&self) -> Self {
            MockReentrantInitializableImple(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockReentrantInitializableImple<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockReentrantInitializableImple<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockReentrantInitializableImple))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockReentrantInitializableImple<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKREENTRANTINITIALIZABLEIMPLE_ABI.clone(),
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
                MOCKREENTRANTINITIALIZABLEIMPLE_ABI.clone(),
                MOCKREENTRANTINITIALIZABLEIMPLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `REVISION` (0xdde43cba) function"]
        pub fn revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 228, 60, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xfe4b84df) function"]
        pub fn initialize(
            &self,
            val: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 75, 132, 223], val)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `value` (0x3fa4f245) function"]
        pub fn value(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([63, 164, 242, 69], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockReentrantInitializableImple<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `REVISION` function with signature `REVISION()` and selector `[221, 228, 60, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "REVISION", abi = "REVISION()")]
    pub struct RevisionCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256)` and selector `[254, 75, 132, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256)")]
    pub struct InitializeCall {
        pub val: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `value` function with signature `value()` and selector `[63, 164, 242, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "value", abi = "value()")]
    pub struct ValueCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockReentrantInitializableImpleCalls {
        Revision(RevisionCall),
        Initialize(InitializeCall),
        Value(ValueCall),
    }
    impl ethers::core::abi::AbiDecode for MockReentrantInitializableImpleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <RevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReentrantInitializableImpleCalls::Revision(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReentrantInitializableImpleCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <ValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReentrantInitializableImpleCalls::Value(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockReentrantInitializableImpleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockReentrantInitializableImpleCalls::Revision(element) => element.encode(),
                MockReentrantInitializableImpleCalls::Initialize(element) => element.encode(),
                MockReentrantInitializableImpleCalls::Value(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockReentrantInitializableImpleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockReentrantInitializableImpleCalls::Revision(element) => element.fmt(f),
                MockReentrantInitializableImpleCalls::Initialize(element) => element.fmt(f),
                MockReentrantInitializableImpleCalls::Value(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<RevisionCall> for MockReentrantInitializableImpleCalls {
        fn from(var: RevisionCall) -> Self {
            MockReentrantInitializableImpleCalls::Revision(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockReentrantInitializableImpleCalls {
        fn from(var: InitializeCall) -> Self {
            MockReentrantInitializableImpleCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<ValueCall> for MockReentrantInitializableImpleCalls {
        fn from(var: ValueCall) -> Self {
            MockReentrantInitializableImpleCalls::Value(var)
        }
    }
    #[doc = "Container type for all return fields from the `REVISION` function with signature `REVISION()` and selector `[221, 228, 60, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RevisionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `value` function with signature `value()` and selector `[63, 164, 242, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ValueReturn(pub ethers::core::types::U256);
}
