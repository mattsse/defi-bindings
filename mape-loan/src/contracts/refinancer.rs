pub use refinancer_mod::*;
#[allow(clippy::too_many_arguments)]
mod refinancer_mod {
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
    #[doc = "Refinancer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static REFINANCER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralRequiredSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EarlyFeeRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EndingPrincipalSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GracePeriodSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InterestRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LateFeeRateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LateInterestPremiumSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentIntervalSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentsRemainingSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"increasedBy_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PrincipalIncreased\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increasePrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCollateralRequired\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEarlyFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndingPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGracePeriod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInterestRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateInterestPremium\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentInterval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentsRemaining\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static REFINANCER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061068b806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c8063b163ff9711610066578063b163ff9714610104578063d157f64514610117578063e94134d91461012a578063f2f659601461013d578063fe12afe91461015057600080fd5b80634764757e146100a35780635de1858c146100b85780635f84f302146100cb5780637febd92b146100de5780639c3c2ab1146100f1575b600080fd5b6100b66100b13660046105de565b610163565b005b6100b66100c63660046105de565b6101e5565b6100b66100d93660046105de565b61021a565b6100b66100ec3660046105de565b61024f565b6100b66100ff3660046105de565b610284565b6100b66101123660046105de565b6102b9565b6100b66101253660046105de565b610397565b6100b66101383660046105de565b61040d565b6100b661014b3660046105de565b610494565b6100b661015e3660046105de565b6104c9565b806101a95760405162461bcd60e51b8152602060048201526011602482015270148e94d4148e96915493d7d05353d55395607a1b60448201526064015b60405180910390fd5b60138190556040518181527f35ea55dd4eee968798b5f61efffeb9d5f88b42c05aee6151a1bb4836c082ad21906020015b60405180910390a150565b600c8190556040518181527fd08f964ebebbb23438d7327a9e0b4d3a6977b689a76fbbc5e3ff6cf2bd57c296906020016101da565b60088190556040518181527f532f252238b3b0d2b8c8a257b087fb3fdbdc775e3e0acca8e680a2f36aafa34b906020016101da565b600a8190556040518181527f901c1ec58c5f0467430dc60e75d29fc21f09505b87ae0f9add2ca0aa75f172b5906020016101da565b600b8190556040518181527fa1367e43892cb30dbdf580f60f215aae22c2575ecd8bfef69d87c3671ad79a3b906020016101da565b60055481906102d0906001600160a01b03166104fe565b101561031e5760405162461bcd60e51b815260206004820152601860248201527f523a49503a494e53554646494349454e545f414d4f554e54000000000000000060448201526064016101a0565b80601460008282546103309190610610565b9250508190555080600d60008282546103499190610610565b9250508190555080600f60008282546103629190610610565b90915550506040518181527fc8fcde6244e516452771097a17ecb4c9e6331f498c228f1a951518aedefcb5ee906020016101da565b806103d85760405162461bcd60e51b8152602060048201526011602482015270148e94d4124e96915493d7d05353d55395607a1b60448201526064016101a0565b60078190556040518181527f262b925b6c3983fb29f10dd4493cd3accbbf1fceec18a61b59b2e663b795a37a906020016101da565b60145481111561045f5760405162461bcd60e51b815260206004820152601d60248201527f523a5345503a41424f56455f43555252454e545f5052494e434950414c00000060448201526064016101a0565b600e8190556040518181527fbef1806a01fbfd8f94363f8c74073ad697b31d9278da8c01247366e6c8aa1678906020016101da565b60068190556040518181527f376aafccbf0af4f25bc38eb52182d4604f044d0d87e4cb26e1667b50e3a1de05906020016101da565b60098190556040518181527f6c16b95dcb84ecf131270f4ff8500490839e14db1d5fb8d8a8c89da8551806f8906020016101da565b6005546000906001600160a01b0383811691161461051d57600061052d565b600f5460105461052d9190610610565b6004546001600160a01b0384811691161461054957600061054d565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561058c57600080fd5b505afa1580156105a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105c491906105f7565b6105ce9190610628565b6105d89190610628565b92915050565b6000602082840312156105f057600080fd5b5035919050565b60006020828403121561060957600080fd5b5051919050565b600082198211156106235761062361063f565b500190565b60008282101561063a5761063a61063f565b500390565b634e487b7160e01b600052601160045260246000fdfea26469706673582212207ec41d97fd74277290ee65f2b3a186dff3b38b0278f194726215bf68f29423f364736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Refinancer<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Refinancer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Refinancer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Refinancer))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Refinancer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), REFINANCER_ABI.clone(), client).into()
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
                REFINANCER_ABI.clone(),
                REFINANCER_BYTECODE.clone().into(),
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, RefinancerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Refinancer<M> {
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
    pub enum RefinancerEvents {
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
    impl ethers::contract::EthLogDecode for RefinancerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CollateralRequiredSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::CollateralRequiredSetFilter(decoded));
            }
            if let Ok(decoded) = EarlyFeeRateSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::EarlyFeeRateSetFilter(decoded));
            }
            if let Ok(decoded) = EndingPrincipalSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::EndingPrincipalSetFilter(decoded));
            }
            if let Ok(decoded) = GracePeriodSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::GracePeriodSetFilter(decoded));
            }
            if let Ok(decoded) = InterestRateSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::InterestRateSetFilter(decoded));
            }
            if let Ok(decoded) = LateFeeRateSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::LateFeeRateSetFilter(decoded));
            }
            if let Ok(decoded) = LateInterestPremiumSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::LateInterestPremiumSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentIntervalSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::PaymentIntervalSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentsRemainingSetFilter::decode_log(log) {
                return Ok(RefinancerEvents::PaymentsRemainingSetFilter(decoded));
            }
            if let Ok(decoded) = PrincipalIncreasedFilter::decode_log(log) {
                return Ok(RefinancerEvents::PrincipalIncreasedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for RefinancerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RefinancerEvents::CollateralRequiredSetFilter(element) => element.fmt(f),
                RefinancerEvents::EarlyFeeRateSetFilter(element) => element.fmt(f),
                RefinancerEvents::EndingPrincipalSetFilter(element) => element.fmt(f),
                RefinancerEvents::GracePeriodSetFilter(element) => element.fmt(f),
                RefinancerEvents::InterestRateSetFilter(element) => element.fmt(f),
                RefinancerEvents::LateFeeRateSetFilter(element) => element.fmt(f),
                RefinancerEvents::LateInterestPremiumSetFilter(element) => element.fmt(f),
                RefinancerEvents::PaymentIntervalSetFilter(element) => element.fmt(f),
                RefinancerEvents::PaymentsRemainingSetFilter(element) => element.fmt(f),
                RefinancerEvents::PrincipalIncreasedFilter(element) => element.fmt(f),
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
    pub enum RefinancerCalls {
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
    impl ethers::core::abi::AbiDecode for RefinancerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IncreasePrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::IncreasePrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetCollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <SetEarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetEarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetEndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetEndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetInterestRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetLateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetLateInterestPremium(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetPaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RefinancerCalls::SetPaymentsRemaining(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for RefinancerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                RefinancerCalls::IncreasePrincipal(element) => element.encode(),
                RefinancerCalls::SetCollateralRequired(element) => element.encode(),
                RefinancerCalls::SetEarlyFeeRate(element) => element.encode(),
                RefinancerCalls::SetEndingPrincipal(element) => element.encode(),
                RefinancerCalls::SetGracePeriod(element) => element.encode(),
                RefinancerCalls::SetInterestRate(element) => element.encode(),
                RefinancerCalls::SetLateFeeRate(element) => element.encode(),
                RefinancerCalls::SetLateInterestPremium(element) => element.encode(),
                RefinancerCalls::SetPaymentInterval(element) => element.encode(),
                RefinancerCalls::SetPaymentsRemaining(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for RefinancerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RefinancerCalls::IncreasePrincipal(element) => element.fmt(f),
                RefinancerCalls::SetCollateralRequired(element) => element.fmt(f),
                RefinancerCalls::SetEarlyFeeRate(element) => element.fmt(f),
                RefinancerCalls::SetEndingPrincipal(element) => element.fmt(f),
                RefinancerCalls::SetGracePeriod(element) => element.fmt(f),
                RefinancerCalls::SetInterestRate(element) => element.fmt(f),
                RefinancerCalls::SetLateFeeRate(element) => element.fmt(f),
                RefinancerCalls::SetLateInterestPremium(element) => element.fmt(f),
                RefinancerCalls::SetPaymentInterval(element) => element.fmt(f),
                RefinancerCalls::SetPaymentsRemaining(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IncreasePrincipalCall> for RefinancerCalls {
        fn from(var: IncreasePrincipalCall) -> Self {
            RefinancerCalls::IncreasePrincipal(var)
        }
    }
    impl ::std::convert::From<SetCollateralRequiredCall> for RefinancerCalls {
        fn from(var: SetCollateralRequiredCall) -> Self {
            RefinancerCalls::SetCollateralRequired(var)
        }
    }
    impl ::std::convert::From<SetEarlyFeeRateCall> for RefinancerCalls {
        fn from(var: SetEarlyFeeRateCall) -> Self {
            RefinancerCalls::SetEarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<SetEndingPrincipalCall> for RefinancerCalls {
        fn from(var: SetEndingPrincipalCall) -> Self {
            RefinancerCalls::SetEndingPrincipal(var)
        }
    }
    impl ::std::convert::From<SetGracePeriodCall> for RefinancerCalls {
        fn from(var: SetGracePeriodCall) -> Self {
            RefinancerCalls::SetGracePeriod(var)
        }
    }
    impl ::std::convert::From<SetInterestRateCall> for RefinancerCalls {
        fn from(var: SetInterestRateCall) -> Self {
            RefinancerCalls::SetInterestRate(var)
        }
    }
    impl ::std::convert::From<SetLateFeeRateCall> for RefinancerCalls {
        fn from(var: SetLateFeeRateCall) -> Self {
            RefinancerCalls::SetLateFeeRate(var)
        }
    }
    impl ::std::convert::From<SetLateInterestPremiumCall> for RefinancerCalls {
        fn from(var: SetLateInterestPremiumCall) -> Self {
            RefinancerCalls::SetLateInterestPremium(var)
        }
    }
    impl ::std::convert::From<SetPaymentIntervalCall> for RefinancerCalls {
        fn from(var: SetPaymentIntervalCall) -> Self {
            RefinancerCalls::SetPaymentInterval(var)
        }
    }
    impl ::std::convert::From<SetPaymentsRemainingCall> for RefinancerCalls {
        fn from(var: SetPaymentsRemainingCall) -> Self {
            RefinancerCalls::SetPaymentsRemaining(var)
        }
    }
}
