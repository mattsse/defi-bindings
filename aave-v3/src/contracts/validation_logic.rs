pub use validation_logic::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod validation_logic {
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
    #[doc = "ValidationLogic was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VALIDATIONLOGIC_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HEALTH_FACTOR_LIQUIDATION_THRESHOLD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static VALIDATIONLOGIC_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60b8610039600b82828239805160001a60731461002c57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe730000000000000000000000000000000000000000301460806040526004361060475760003560e01c8063561cbec914604c578063abfcc86a14606c578063c3525c28146074575b600080fd5b605a670d2f13f7789f000081565b60405190815260200160405180910390f35b605a61232881565b605a670de0b6b3a76400008156fea26469706673582212202b4015d9c0001e4626f5bee8dbdedb2cd0530cddaf0abc20dd105084252b5cc264736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ValidationLogic<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ValidationLogic<M> {
        fn clone(&self) -> Self {
            ValidationLogic(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ValidationLogic<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ValidationLogic<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ValidationLogic))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ValidationLogic<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VALIDATIONLOGIC_ABI.clone(), client)
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
                VALIDATIONLOGIC_ABI.clone(),
                VALIDATIONLOGIC_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` (0xc3525c28) function"]
        pub fn health_factor_liquidation_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([195, 82, 92, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` (0x561cbec9) function"]
        pub fn minimum_health_factor_liquidation_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([86, 28, 190, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` (0xabfcc86a) function"]
        pub fn rebalance_up_liquidity_rate_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([171, 252, 200, 106], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ValidationLogic<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `[195, 82, 92, 40]`"]
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
        name = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
        abi = "HEALTH_FACTOR_LIQUIDATION_THRESHOLD()"
    )]
    pub struct HealthFactorLiquidationThresholdCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `[86, 28, 190, 201]`"]
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
        name = "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD",
        abi = "MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()"
    )]
    pub struct MinimumHealthFactorLiquidationThresholdCall;
    #[doc = "Container type for all input parameters for the `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` function with signature `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()` and selector `[171, 252, 200, 106]`"]
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
        name = "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD",
        abi = "REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()"
    )]
    pub struct RebalanceUpLiquidityRateThresholdCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ValidationLogicCalls {
        HealthFactorLiquidationThreshold(HealthFactorLiquidationThresholdCall),
        MinimumHealthFactorLiquidationThreshold(MinimumHealthFactorLiquidationThresholdCall),
        RebalanceUpLiquidityRateThreshold(RebalanceUpLiquidityRateThresholdCall),
    }
    impl ethers::core::abi::AbiDecode for ValidationLogicCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <HealthFactorLiquidationThresholdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ValidationLogicCalls::HealthFactorLiquidationThreshold(
                    decoded,
                ));
            }
            if let Ok (decoded) = < MinimumHealthFactorLiquidationThresholdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ValidationLogicCalls :: MinimumHealthFactorLiquidationThreshold (decoded)) }
            if let Ok(decoded) =
                <RebalanceUpLiquidityRateThresholdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ValidationLogicCalls::RebalanceUpLiquidityRateThreshold(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ValidationLogicCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ValidationLogicCalls::HealthFactorLiquidationThreshold(element) => element.encode(),
                ValidationLogicCalls::MinimumHealthFactorLiquidationThreshold(element) => {
                    element.encode()
                }
                ValidationLogicCalls::RebalanceUpLiquidityRateThreshold(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for ValidationLogicCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ValidationLogicCalls::HealthFactorLiquidationThreshold(element) => element.fmt(f),
                ValidationLogicCalls::MinimumHealthFactorLiquidationThreshold(element) => {
                    element.fmt(f)
                }
                ValidationLogicCalls::RebalanceUpLiquidityRateThreshold(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<HealthFactorLiquidationThresholdCall> for ValidationLogicCalls {
        fn from(var: HealthFactorLiquidationThresholdCall) -> Self {
            ValidationLogicCalls::HealthFactorLiquidationThreshold(var)
        }
    }
    impl ::std::convert::From<MinimumHealthFactorLiquidationThresholdCall> for ValidationLogicCalls {
        fn from(var: MinimumHealthFactorLiquidationThresholdCall) -> Self {
            ValidationLogicCalls::MinimumHealthFactorLiquidationThreshold(var)
        }
    }
    impl ::std::convert::From<RebalanceUpLiquidityRateThresholdCall> for ValidationLogicCalls {
        fn from(var: RebalanceUpLiquidityRateThresholdCall) -> Self {
            ValidationLogicCalls::RebalanceUpLiquidityRateThreshold(var)
        }
    }
    #[doc = "Container type for all return fields from the `HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `[195, 82, 92, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HealthFactorLiquidationThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD` function with signature `MINIMUM_HEALTH_FACTOR_LIQUIDATION_THRESHOLD()` and selector `[86, 28, 190, 201]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MinimumHealthFactorLiquidationThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD` function with signature `REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD()` and selector `[171, 252, 200, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RebalanceUpLiquidityRateThresholdReturn(pub ethers::core::types::U256);
}
