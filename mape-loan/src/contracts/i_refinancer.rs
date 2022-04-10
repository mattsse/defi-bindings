pub use irefinancer_mod::*;
#[allow(clippy::too_many_arguments)]
mod irefinancer_mod {
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
    #[doc = "IRefinancer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IREFINANCER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralRequiredSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EarlyFeeRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EndingPrincipalSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GracePeriodSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InterestRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LateFeeRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LateInterestPremiumSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentIntervalSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentsRemainingSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"increasedBy_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PrincipalIncreased\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increasePrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCollateralRequired\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEarlyFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndingPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGracePeriod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInterestRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateInterestPremium\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentInterval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentsRemaining\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IREFINANCER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IRefinancer<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IRefinancer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRefinancer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRefinancer))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IRefinancer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IREFINANCER_ABI.clone(), client).into()
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
                IREFINANCER_ABI.clone(),
                IREFINANCER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `increasePrincipal` (0xb163ff97) function"]
        pub fn increase_principal(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 99, 255, 151], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCollateralRequired` (0x5de1858c) function"]
        pub fn set_collateral_required(
            &self,
            collateral_required: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 225, 133, 140], collateral_required)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEarlyFeeRate` (0xfe12afe9) function"]
        pub fn set_early_fee_rate(
            &self,
            early_fee_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 18, 175, 233], early_fee_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEndingPrincipal` (0xe94134d9) function"]
        pub fn set_ending_principal(
            &self,
            ending_principal: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 65, 52, 217], ending_principal)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGracePeriod` (0xf2f65960) function"]
        pub fn set_grace_period(
            &self,
            grace_period: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 246, 89, 96], grace_period)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setInterestRate` (0x5f84f302) function"]
        pub fn set_interest_rate(
            &self,
            interest_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 132, 243, 2], interest_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLateFeeRate` (0x7febd92b) function"]
        pub fn set_late_fee_rate(
            &self,
            late_fee_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 235, 217, 43], late_fee_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLateInterestPremium` (0x9c3c2ab1) function"]
        pub fn set_late_interest_premium(
            &self,
            late_interest_premium: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 60, 42, 177], late_interest_premium)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPaymentInterval` (0xd157f645) function"]
        pub fn set_payment_interval(
            &self,
            payment_interval: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 87, 246, 69], payment_interval)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPaymentsRemaining` (0x4764757e) function"]
        pub fn set_payments_remaining(
            &self,
            payments_remaining: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 100, 117, 126], payments_remaining)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CollateralRequiredSet` event"]
        pub fn collateral_required_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralRequiredSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EarlyFeeRateSet` event"]
        pub fn early_fee_rate_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EarlyFeeRateSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EndingPrincipalSet` event"]
        pub fn ending_principal_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EndingPrincipalSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GracePeriodSet` event"]
        pub fn grace_period_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GracePeriodSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InterestRateSet` event"]
        pub fn interest_rate_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InterestRateSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LateFeeRateSet` event"]
        pub fn late_fee_rate_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LateFeeRateSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LateInterestPremiumSet` event"]
        pub fn late_interest_premium_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LateInterestPremiumSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentIntervalSet` event"]
        pub fn payment_interval_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentIntervalSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentsRemainingSet` event"]
        pub fn payments_remaining_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentsRemainingSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PrincipalIncreased` event"]
        pub fn principal_increased_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PrincipalIncreasedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IRefinancerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRefinancer<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "CollateralRequiredSet", abi = "CollateralRequiredSet(uint256)")]
    pub struct CollateralRequiredSetFilter {
        pub collateral_required: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "EarlyFeeRateSet", abi = "EarlyFeeRateSet(uint256)")]
    pub struct EarlyFeeRateSetFilter {
        pub early_fee_rate: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "EndingPrincipalSet", abi = "EndingPrincipalSet(uint256)")]
    pub struct EndingPrincipalSetFilter {
        pub ending_principal: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "GracePeriodSet", abi = "GracePeriodSet(uint256)")]
    pub struct GracePeriodSetFilter {
        pub grace_period: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "InterestRateSet", abi = "InterestRateSet(uint256)")]
    pub struct InterestRateSetFilter {
        pub interest_rate: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "LateFeeRateSet", abi = "LateFeeRateSet(uint256)")]
    pub struct LateFeeRateSetFilter {
        pub late_fee_rate: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "LateInterestPremiumSet",
        abi = "LateInterestPremiumSet(uint256)"
    )]
    pub struct LateInterestPremiumSetFilter {
        pub late_interest_premium: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "PaymentIntervalSet", abi = "PaymentIntervalSet(uint256)")]
    pub struct PaymentIntervalSetFilter {
        pub payment_interval: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "PaymentsRemainingSet", abi = "PaymentsRemainingSet(uint256)")]
    pub struct PaymentsRemainingSetFilter {
        pub payments_remaining: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "PrincipalIncreased", abi = "PrincipalIncreased(uint256)")]
    pub struct PrincipalIncreasedFilter {
        pub increased_by: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IRefinancerEvents {
        CollateralRequiredSetFilter(CollateralRequiredSetFilter),
        EarlyFeeRateSetFilter(EarlyFeeRateSetFilter),
        EndingPrincipalSetFilter(EndingPrincipalSetFilter),
        GracePeriodSetFilter(GracePeriodSetFilter),
        InterestRateSetFilter(InterestRateSetFilter),
        LateFeeRateSetFilter(LateFeeRateSetFilter),
        LateInterestPremiumSetFilter(LateInterestPremiumSetFilter),
        PaymentIntervalSetFilter(PaymentIntervalSetFilter),
        PaymentsRemainingSetFilter(PaymentsRemainingSetFilter),
        PrincipalIncreasedFilter(PrincipalIncreasedFilter),
    }
    impl ethers::contract::EthLogDecode for IRefinancerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CollateralRequiredSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::CollateralRequiredSetFilter(decoded));
            }
            if let Ok(decoded) = EarlyFeeRateSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::EarlyFeeRateSetFilter(decoded));
            }
            if let Ok(decoded) = EndingPrincipalSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::EndingPrincipalSetFilter(decoded));
            }
            if let Ok(decoded) = GracePeriodSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::GracePeriodSetFilter(decoded));
            }
            if let Ok(decoded) = InterestRateSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::InterestRateSetFilter(decoded));
            }
            if let Ok(decoded) = LateFeeRateSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::LateFeeRateSetFilter(decoded));
            }
            if let Ok(decoded) = LateInterestPremiumSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::LateInterestPremiumSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentIntervalSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::PaymentIntervalSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentsRemainingSetFilter::decode_log(log) {
                return Ok(IRefinancerEvents::PaymentsRemainingSetFilter(decoded));
            }
            if let Ok(decoded) = PrincipalIncreasedFilter::decode_log(log) {
                return Ok(IRefinancerEvents::PrincipalIncreasedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IRefinancerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRefinancerEvents::CollateralRequiredSetFilter(element) => element.fmt(f),
                IRefinancerEvents::EarlyFeeRateSetFilter(element) => element.fmt(f),
                IRefinancerEvents::EndingPrincipalSetFilter(element) => element.fmt(f),
                IRefinancerEvents::GracePeriodSetFilter(element) => element.fmt(f),
                IRefinancerEvents::InterestRateSetFilter(element) => element.fmt(f),
                IRefinancerEvents::LateFeeRateSetFilter(element) => element.fmt(f),
                IRefinancerEvents::LateInterestPremiumSetFilter(element) => element.fmt(f),
                IRefinancerEvents::PaymentIntervalSetFilter(element) => element.fmt(f),
                IRefinancerEvents::PaymentsRemainingSetFilter(element) => element.fmt(f),
                IRefinancerEvents::PrincipalIncreasedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `increasePrincipal`function with signature `increasePrincipal(uint256)` and selector `[177, 99, 255, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increasePrincipal", abi = "increasePrincipal(uint256)")]
    pub struct IncreasePrincipalCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setCollateralRequired`function with signature `setCollateralRequired(uint256)` and selector `[93, 225, 133, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setCollateralRequired", abi = "setCollateralRequired(uint256)")]
    pub struct SetCollateralRequiredCall {
        pub collateral_required: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setEarlyFeeRate`function with signature `setEarlyFeeRate(uint256)` and selector `[254, 18, 175, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setEarlyFeeRate", abi = "setEarlyFeeRate(uint256)")]
    pub struct SetEarlyFeeRateCall {
        pub early_fee_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setEndingPrincipal`function with signature `setEndingPrincipal(uint256)` and selector `[233, 65, 52, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setEndingPrincipal", abi = "setEndingPrincipal(uint256)")]
    pub struct SetEndingPrincipalCall {
        pub ending_principal: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setGracePeriod`function with signature `setGracePeriod(uint256)` and selector `[242, 246, 89, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGracePeriod", abi = "setGracePeriod(uint256)")]
    pub struct SetGracePeriodCall {
        pub grace_period: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setInterestRate`function with signature `setInterestRate(uint256)` and selector `[95, 132, 243, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setInterestRate", abi = "setInterestRate(uint256)")]
    pub struct SetInterestRateCall {
        pub interest_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLateFeeRate`function with signature `setLateFeeRate(uint256)` and selector `[127, 235, 217, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLateFeeRate", abi = "setLateFeeRate(uint256)")]
    pub struct SetLateFeeRateCall {
        pub late_fee_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLateInterestPremium`function with signature `setLateInterestPremium(uint256)` and selector `[156, 60, 42, 177]`"]
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
        name = "setLateInterestPremium",
        abi = "setLateInterestPremium(uint256)"
    )]
    pub struct SetLateInterestPremiumCall {
        pub late_interest_premium: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPaymentInterval`function with signature `setPaymentInterval(uint256)` and selector `[209, 87, 246, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPaymentInterval", abi = "setPaymentInterval(uint256)")]
    pub struct SetPaymentIntervalCall {
        pub payment_interval: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPaymentsRemaining`function with signature `setPaymentsRemaining(uint256)` and selector `[71, 100, 117, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPaymentsRemaining", abi = "setPaymentsRemaining(uint256)")]
    pub struct SetPaymentsRemainingCall {
        pub payments_remaining: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IRefinancerCalls {
        IncreasePrincipal(IncreasePrincipalCall),
        SetCollateralRequired(SetCollateralRequiredCall),
        SetEarlyFeeRate(SetEarlyFeeRateCall),
        SetEndingPrincipal(SetEndingPrincipalCall),
        SetGracePeriod(SetGracePeriodCall),
        SetInterestRate(SetInterestRateCall),
        SetLateFeeRate(SetLateFeeRateCall),
        SetLateInterestPremium(SetLateInterestPremiumCall),
        SetPaymentInterval(SetPaymentIntervalCall),
        SetPaymentsRemaining(SetPaymentsRemainingCall),
    }
    impl ethers::core::abi::AbiDecode for IRefinancerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IncreasePrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::IncreasePrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetCollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <SetEarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetEarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetEndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetEndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetInterestRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetLateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetLateInterestPremium(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetPaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRefinancerCalls::SetPaymentsRemaining(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRefinancerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRefinancerCalls::IncreasePrincipal(element) => element.encode(),
                IRefinancerCalls::SetCollateralRequired(element) => element.encode(),
                IRefinancerCalls::SetEarlyFeeRate(element) => element.encode(),
                IRefinancerCalls::SetEndingPrincipal(element) => element.encode(),
                IRefinancerCalls::SetGracePeriod(element) => element.encode(),
                IRefinancerCalls::SetInterestRate(element) => element.encode(),
                IRefinancerCalls::SetLateFeeRate(element) => element.encode(),
                IRefinancerCalls::SetLateInterestPremium(element) => element.encode(),
                IRefinancerCalls::SetPaymentInterval(element) => element.encode(),
                IRefinancerCalls::SetPaymentsRemaining(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRefinancerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRefinancerCalls::IncreasePrincipal(element) => element.fmt(f),
                IRefinancerCalls::SetCollateralRequired(element) => element.fmt(f),
                IRefinancerCalls::SetEarlyFeeRate(element) => element.fmt(f),
                IRefinancerCalls::SetEndingPrincipal(element) => element.fmt(f),
                IRefinancerCalls::SetGracePeriod(element) => element.fmt(f),
                IRefinancerCalls::SetInterestRate(element) => element.fmt(f),
                IRefinancerCalls::SetLateFeeRate(element) => element.fmt(f),
                IRefinancerCalls::SetLateInterestPremium(element) => element.fmt(f),
                IRefinancerCalls::SetPaymentInterval(element) => element.fmt(f),
                IRefinancerCalls::SetPaymentsRemaining(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IncreasePrincipalCall> for IRefinancerCalls {
        fn from(var: IncreasePrincipalCall) -> Self {
            IRefinancerCalls::IncreasePrincipal(var)
        }
    }
    impl ::std::convert::From<SetCollateralRequiredCall> for IRefinancerCalls {
        fn from(var: SetCollateralRequiredCall) -> Self {
            IRefinancerCalls::SetCollateralRequired(var)
        }
    }
    impl ::std::convert::From<SetEarlyFeeRateCall> for IRefinancerCalls {
        fn from(var: SetEarlyFeeRateCall) -> Self {
            IRefinancerCalls::SetEarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<SetEndingPrincipalCall> for IRefinancerCalls {
        fn from(var: SetEndingPrincipalCall) -> Self {
            IRefinancerCalls::SetEndingPrincipal(var)
        }
    }
    impl ::std::convert::From<SetGracePeriodCall> for IRefinancerCalls {
        fn from(var: SetGracePeriodCall) -> Self {
            IRefinancerCalls::SetGracePeriod(var)
        }
    }
    impl ::std::convert::From<SetInterestRateCall> for IRefinancerCalls {
        fn from(var: SetInterestRateCall) -> Self {
            IRefinancerCalls::SetInterestRate(var)
        }
    }
    impl ::std::convert::From<SetLateFeeRateCall> for IRefinancerCalls {
        fn from(var: SetLateFeeRateCall) -> Self {
            IRefinancerCalls::SetLateFeeRate(var)
        }
    }
    impl ::std::convert::From<SetLateInterestPremiumCall> for IRefinancerCalls {
        fn from(var: SetLateInterestPremiumCall) -> Self {
            IRefinancerCalls::SetLateInterestPremium(var)
        }
    }
    impl ::std::convert::From<SetPaymentIntervalCall> for IRefinancerCalls {
        fn from(var: SetPaymentIntervalCall) -> Self {
            IRefinancerCalls::SetPaymentInterval(var)
        }
    }
    impl ::std::convert::From<SetPaymentsRemainingCall> for IRefinancerCalls {
        fn from(var: SetPaymentsRemainingCall) -> Self {
            IRefinancerCalls::SetPaymentsRemaining(var)
        }
    }
}
