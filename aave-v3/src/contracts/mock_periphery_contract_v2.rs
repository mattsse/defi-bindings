pub use mock_periphery_contract_v2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_periphery_contract_v2 {
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
    #[doc = "MockPeripheryContractV2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKPERIPHERYCONTRACTV2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProvider\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setManager\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKPERIPHERYCONTRACTV2_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610153806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063c4d66de814610051578063d0ebdbe714610083578063d5009584146100b3578063fe65acfe146100dc575b600080fd5b61008161005f3660046100ed565b600280546001600160a01b0319166001600160a01b0392909216919091179055565b005b6100816100913660046100ed565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6000546001600160a01b03165b6040516001600160a01b03909116815260200160405180910390f35b6002546001600160a01b03166100c0565b6000602082840312156100ff57600080fd5b81356001600160a01b038116811461011657600080fd5b939250505056fea26469706673582212204824046b61401b9b0b38344a17ff4a2735c150052c979babf19e8a3801bbe43464736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    pub struct MockPeripheryContractV2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockPeripheryContractV2<M> {
        fn clone(&self) -> Self {
            MockPeripheryContractV2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockPeripheryContractV2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockPeripheryContractV2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockPeripheryContractV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockPeripheryContractV2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKPERIPHERYCONTRACTV2_ABI.clone(),
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
                MOCKPERIPHERYCONTRACTV2_ABI.clone(),
                MOCKPERIPHERYCONTRACTV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getAddressesProvider` (0xfe65acfe) function"]
        pub fn get_addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([254, 101, 172, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getManager` (0xd5009584) function"]
        pub fn get_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([213, 0, 149, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            addresses_provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], addresses_provider)
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
        for MockPeripheryContractV2<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getAddressesProvider` function with signature `getAddressesProvider()` and selector `[254, 101, 172, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAddressesProvider", abi = "getAddressesProvider()")]
    pub struct GetAddressesProviderCall;
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
        pub addresses_provider: ethers::core::types::Address,
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
    pub enum MockPeripheryContractV2Calls {
        GetAddressesProvider(GetAddressesProviderCall),
        GetManager(GetManagerCall),
        Initialize(InitializeCall),
        SetManager(SetManagerCall),
    }
    impl ethers::core::abi::AbiDecode for MockPeripheryContractV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV2Calls::GetAddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <GetManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV2Calls::GetManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV2Calls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SetManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockPeripheryContractV2Calls::SetManager(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockPeripheryContractV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockPeripheryContractV2Calls::GetAddressesProvider(element) => element.encode(),
                MockPeripheryContractV2Calls::GetManager(element) => element.encode(),
                MockPeripheryContractV2Calls::Initialize(element) => element.encode(),
                MockPeripheryContractV2Calls::SetManager(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockPeripheryContractV2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockPeripheryContractV2Calls::GetAddressesProvider(element) => element.fmt(f),
                MockPeripheryContractV2Calls::GetManager(element) => element.fmt(f),
                MockPeripheryContractV2Calls::Initialize(element) => element.fmt(f),
                MockPeripheryContractV2Calls::SetManager(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAddressesProviderCall> for MockPeripheryContractV2Calls {
        fn from(var: GetAddressesProviderCall) -> Self {
            MockPeripheryContractV2Calls::GetAddressesProvider(var)
        }
    }
    impl ::std::convert::From<GetManagerCall> for MockPeripheryContractV2Calls {
        fn from(var: GetManagerCall) -> Self {
            MockPeripheryContractV2Calls::GetManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockPeripheryContractV2Calls {
        fn from(var: InitializeCall) -> Self {
            MockPeripheryContractV2Calls::Initialize(var)
        }
    }
    impl ::std::convert::From<SetManagerCall> for MockPeripheryContractV2Calls {
        fn from(var: SetManagerCall) -> Self {
            MockPeripheryContractV2Calls::SetManager(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAddressesProvider` function with signature `getAddressesProvider()` and selector `[254, 101, 172, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressesProviderReturn(pub ethers::core::types::Address);
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
