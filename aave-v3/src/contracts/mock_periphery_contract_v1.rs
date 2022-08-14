pub use mock_periphery_contract_v1::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_periphery_contract_v1 {
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
    #[doc = "MockPeripheryContractV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKPERIPHERYCONTRACTV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"manager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setManager\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKPERIPHERYCONTRACTV1_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610169806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063cd6dc68714610046578063d0ebdbe71461007c578063d5009584146100ac575b600080fd5b61007a6100543660046100e7565b600080546001600160a01b0319166001600160a01b039390931692909217909155600155565b005b61007a61008a366004610111565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b600054604080516001600160a01b039092168252519081900360200190f35b80356001600160a01b03811681146100e257600080fd5b919050565b600080604083850312156100fa57600080fd5b610103836100cb565b946020939093013593505050565b60006020828403121561012357600080fd5b61012c826100cb565b939250505056fea2646970667358221220699f5f34e4b471f86378d8b29bcc4e35661eab11fb8586535a87c931fe9576ae64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    pub struct MockPeripheryContractV1<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockPeripheryContractV1<M> {
        fn clone(&self) -> Self {
            MockPeripheryContractV1(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockPeripheryContractV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockPeripheryContractV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockPeripheryContractV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockPeripheryContractV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKPERIPHERYCONTRACTV1_ABI.clone(),
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
                MOCKPERIPHERYCONTRACTV1_ABI.clone(),
                MOCKPERIPHERYCONTRACTV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getManager` (0xd5009584) function"]
        pub fn get_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([213, 0, 149, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xcd6dc687) function"]
        pub fn initialize(
            &self,
            manager: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 109, 198, 135], (manager, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setManager` (0xd0ebdbe7) function"]
        pub fn set_manager(
            &self,
            new_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 235, 219, 231], new_manager)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockPeripheryContractV1<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getManager` function with signature `getManager()` and selector `[213, 0, 149, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getManager", abi = "getManager()")]
    pub struct GetManagerCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,uint256)` and selector `[205, 109, 198, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,uint256)")]
    pub struct InitializeCall {
        pub manager: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setManager` function with signature `setManager(address)` and selector `[208, 235, 219, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setManager", abi = "setManager(address)")]
    pub struct SetManagerCall {
        pub new_manager: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockPeripheryContractV1Calls {
        GetManager(GetManagerCall),
        Initialize(InitializeCall),
        SetManager(SetManagerCall),
    }
    impl ethers::core::abi::AbiDecode for MockPeripheryContractV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV1Calls::GetManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV1Calls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SetManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV1Calls::SetManager(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockPeripheryContractV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockPeripheryContractV1Calls::GetManager(element) => element.encode(),
                MockPeripheryContractV1Calls::Initialize(element) => element.encode(),
                MockPeripheryContractV1Calls::SetManager(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockPeripheryContractV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockPeripheryContractV1Calls::GetManager(element) => element.fmt(f),
                MockPeripheryContractV1Calls::Initialize(element) => element.fmt(f),
                MockPeripheryContractV1Calls::SetManager(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetManagerCall> for MockPeripheryContractV1Calls {
        fn from(var: GetManagerCall) -> Self {
            MockPeripheryContractV1Calls::GetManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockPeripheryContractV1Calls {
        fn from(var: InitializeCall) -> Self {
            MockPeripheryContractV1Calls::Initialize(var)
        }
    }
    impl ::std::convert::From<SetManagerCall> for MockPeripheryContractV1Calls {
        fn from(var: SetManagerCall) -> Self {
            MockPeripheryContractV1Calls::SetManager(var)
        }
    }
    #[doc = "Container type for all return fields from the `getManager` function with signature `getManager()` and selector `[213, 0, 149, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetManagerReturn(pub ethers::core::types::Address);
}
