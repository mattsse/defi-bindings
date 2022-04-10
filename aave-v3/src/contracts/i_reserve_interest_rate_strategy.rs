pub use ireserveinterestratestrategy_mod::*;
#[allow(clippy::too_many_arguments)]
mod ireserveinterestratestrategy_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IReserveInterestRateStrategy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IRESERVEINTERESTRATESTRATEGY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct DataTypes.CalculateInterestRatesParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"unbacked\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityAdded\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityTaken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalStableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalVariableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"averageStableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aToken\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateInterestRates\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBaseVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IRESERVEINTERESTRATESTRATEGY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IReserveInterestRateStrategy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IReserveInterestRateStrategy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IReserveInterestRateStrategy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IReserveInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IReserveInterestRateStrategy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IRESERVEINTERESTRATESTRATEGY_ABI.clone(),
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
                IRESERVEINTERESTRATESTRATEGY_ABI.clone(),
                IRESERVEINTERESTRATESTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `calculateInterestRates` (0xa5898709) function"]
        pub fn calculate_interest_rates(
            &self,
            params: CalculateInterestRatesParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBaseVariableBorrowRate` (0x34762ca5) function"]
        pub fn get_base_variable_borrow_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 118, 44, 165], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMaxVariableBorrowRate` (0x80031e37) function"]
        pub fn get_max_variable_borrow_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([128, 3, 30, 55], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IReserveInterestRateStrategy<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `calculateInterestRates`function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `[165, 137, 135, 9]`"]
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
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    #[doc = "Container type for all input parameters for the `getBaseVariableBorrowRate`function with signature `getBaseVariableBorrowRate()` and selector `[52, 118, 44, 165]`"]
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
        name = "getBaseVariableBorrowRate",
        abi = "getBaseVariableBorrowRate()"
    )]
    pub struct GetBaseVariableBorrowRateCall;
    #[doc = "Container type for all input parameters for the `getMaxVariableBorrowRate`function with signature `getMaxVariableBorrowRate()` and selector `[128, 3, 30, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMaxVariableBorrowRate", abi = "getMaxVariableBorrowRate()")]
    pub struct GetMaxVariableBorrowRateCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IReserveInterestRateStrategyCalls {
        CalculateInterestRates(CalculateInterestRatesCall),
        GetBaseVariableBorrowRate(GetBaseVariableBorrowRateCall),
        GetMaxVariableBorrowRate(GetMaxVariableBorrowRateCall),
    }
    impl ethers::core::abi::AbiDecode for IReserveInterestRateStrategyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CalculateInterestRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IReserveInterestRateStrategyCalls::CalculateInterestRates(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetBaseVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetMaxVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IReserveInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.encode()
                }
                IReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.encode()
                }
                IReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IReserveInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.fmt(f)
                }
                IReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.fmt(f)
                }
                IReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<CalculateInterestRatesCall> for IReserveInterestRateStrategyCalls {
        fn from(var: CalculateInterestRatesCall) -> Self {
            IReserveInterestRateStrategyCalls::CalculateInterestRates(var)
        }
    }
    impl ::std::convert::From<GetBaseVariableBorrowRateCall> for IReserveInterestRateStrategyCalls {
        fn from(var: GetBaseVariableBorrowRateCall) -> Self {
            IReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetMaxVariableBorrowRateCall> for IReserveInterestRateStrategyCalls {
        fn from(var: GetMaxVariableBorrowRateCall) -> Self {
            IReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(var)
        }
    }
}
