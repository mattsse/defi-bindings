pub use interestratemodel_mod::*;
#[allow(clippy::too_many_arguments)]
mod interestratemodel_mod {
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
    #[doc = "InterestRateModel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static INTERESTRATEMODEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInterestRateModel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static INTERESTRATEMODEL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct InterestRateModel<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for InterestRateModel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for InterestRateModel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(InterestRateModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> InterestRateModel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), INTERESTRATEMODEL_ABI.clone(), client)
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
                INTERESTRATEMODEL_ABI.clone(),
                INTERESTRATEMODEL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getBorrowRate` (0x15f24053) function"]
        pub fn get_borrow_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 242, 64, 83], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyRate` (0xb8168816) function"]
        pub fn get_supply_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
            reserve_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [184, 22, 136, 22],
                    (cash, borrows, reserves, reserve_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInterestRateModel` (0x2191f92a) function"]
        pub fn is_interest_rate_model(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([33, 145, 249, 42], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for InterestRateModel<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getBorrowRate`function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowRate", abi = "getBorrowRate(uint256,uint256,uint256)")]
    pub struct GetBorrowRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSupplyRate`function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
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
        name = "getSupplyRate",
        abi = "getSupplyRate(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetSupplyRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
        pub reserve_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isInterestRateModel`function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isInterestRateModel", abi = "isInterestRateModel()")]
    pub struct IsInterestRateModelCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum InterestRateModelCalls {
        GetBorrowRate(GetBorrowRateCall),
        GetSupplyRate(GetSupplyRateCall),
        IsInterestRateModel(IsInterestRateModelCall),
    }
    impl ethers::core::abi::AbiDecode for InterestRateModelCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::GetBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::GetSupplyRate(decoded));
            }
            if let Ok(decoded) =
                <IsInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(InterestRateModelCalls::IsInterestRateModel(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for InterestRateModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                InterestRateModelCalls::GetBorrowRate(element) => element.encode(),
                InterestRateModelCalls::GetSupplyRate(element) => element.encode(),
                InterestRateModelCalls::IsInterestRateModel(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for InterestRateModelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                InterestRateModelCalls::GetBorrowRate(element) => element.fmt(f),
                InterestRateModelCalls::GetSupplyRate(element) => element.fmt(f),
                InterestRateModelCalls::IsInterestRateModel(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetBorrowRateCall> for InterestRateModelCalls {
        fn from(var: GetBorrowRateCall) -> Self {
            InterestRateModelCalls::GetBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyRateCall> for InterestRateModelCalls {
        fn from(var: GetSupplyRateCall) -> Self {
            InterestRateModelCalls::GetSupplyRate(var)
        }
    }
    impl ::std::convert::From<IsInterestRateModelCall> for InterestRateModelCalls {
        fn from(var: IsInterestRateModelCall) -> Self {
            InterestRateModelCalls::IsInterestRateModel(var)
        }
    }
}
