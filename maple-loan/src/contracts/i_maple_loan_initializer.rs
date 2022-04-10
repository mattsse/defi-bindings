pub use imapleloaninitializer_mod::*;
#[allow(clippy::too_many_arguments)]
mod imapleloaninitializer_mod {
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
    #[doc = "IMapleLoanInitializer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IMAPLELOANINITIALIZER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"encodedArguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decodeArguments\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"encodeArguments\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"encodedArguments_\",\"type\":\"bytes\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IMAPLELOANINITIALIZER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IMapleLoanInitializer<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IMapleLoanInitializer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IMapleLoanInitializer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMapleLoanInitializer))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IMapleLoanInitializer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IMAPLELOANINITIALIZER_ABI.clone(),
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
                IMAPLELOANINITIALIZER_ABI.clone(),
                IMAPLELOANINITIALIZER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `decodeArguments` (0x1725fa1d) function"]
        pub fn decode_arguments(
            &self,
            encoded_arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                [ethers::core::types::Address; 2usize],
                [ethers::core::types::U256; 3usize],
                [ethers::core::types::U256; 3usize],
                [ethers::core::types::U256; 4usize],
            ),
        > {
            self.0
                .method_hash([23, 37, 250, 29], encoded_arguments)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeArguments` (0x169acd1e) function"]
        pub fn encode_arguments(
            &self,
            borrower: ethers::core::types::Address,
            assets: [ethers::core::types::Address; 2usize],
            term_details: [ethers::core::types::U256; 3usize],
            amounts: [ethers::core::types::U256; 3usize],
            rates: [ethers::core::types::U256; 4usize],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [22, 154, 205, 30],
                    (borrower, assets, term_details, amounts, rates),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BorrowerAccepted` event"]
        pub fn borrower_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowerAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralPosted` event"]
        pub fn collateral_posted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralPostedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralRemoved` event"]
        pub fn collateral_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EstablishmentFeesSet` event"]
        pub fn establishment_fees_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EstablishmentFeesSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Funded` event"]
        pub fn funded_filter(&self) -> ethers::contract::builders::Event<M, FundedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsClaimed` event"]
        pub fn funds_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsDrawnDown` event"]
        pub fn funds_drawn_down_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsDrawnDownFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsRedirected` event"]
        pub fn funds_redirected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsRedirectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsReturned` event"]
        pub fn funds_returned_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsReturnedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LenderAccepted` event"]
        pub fn lender_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LenderAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LoanClosed` event"]
        pub fn loan_closed_filter(&self) -> ethers::contract::builders::Event<M, LoanClosedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsAccepted` event"]
        pub fn new_terms_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsProposed` event"]
        pub fn new_terms_proposed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsProposedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsRejected` event"]
        pub fn new_terms_rejected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsRejectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentMade` event"]
        pub fn payment_made_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentMadeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PendingBorrowerSet` event"]
        pub fn pending_borrower_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PendingBorrowerSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PendingLenderSet` event"]
        pub fn pending_lender_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PendingLenderSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repossessed` event"]
        pub fn repossessed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RepossessedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Skimmed` event"]
        pub fn skimmed_filter(&self) -> ethers::contract::builders::Event<M, SkimmedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IMapleLoanInitializerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IMapleLoanInitializer<M>
    {
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
    #[ethevent(name = "BorrowerAccepted", abi = "BorrowerAccepted(address)")]
    pub struct BorrowerAcceptedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
    #[ethevent(name = "CollateralPosted", abi = "CollateralPosted(uint256)")]
    pub struct CollateralPostedFilter {
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "CollateralRemoved", abi = "CollateralRemoved(uint256,address)")]
    pub struct CollateralRemovedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
        name = "EstablishmentFeesSet",
        abi = "EstablishmentFeesSet(uint256,uint256)"
    )]
    pub struct EstablishmentFeesSetFilter {
        pub delegate_fee: ethers::core::types::U256,
        pub treasury_fee: ethers::core::types::U256,
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
    #[ethevent(name = "Funded", abi = "Funded(address,uint256,uint256)")]
    pub struct FundedFilter {
        #[ethevent(indexed)]
        pub lender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub next_payment_due_date: ethers::core::types::U256,
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
    #[ethevent(name = "FundsClaimed", abi = "FundsClaimed(uint256,address)")]
    pub struct FundsClaimedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsDrawnDown", abi = "FundsDrawnDown(uint256,address)")]
    pub struct FundsDrawnDownFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsRedirected", abi = "FundsRedirected(uint256,address)")]
    pub struct FundsRedirectedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsReturned", abi = "FundsReturned(uint256)")]
    pub struct FundsReturnedFilter {
        pub amount: ethers::core::types::U256,
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
        name = "Initialized",
        abi = "Initialized(address,address[2],uint256[3],uint256[3],uint256[4])"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        pub assets: [ethers::core::types::Address; 2],
        pub term_details: [ethers::core::types::U256; 3],
        pub amounts: [ethers::core::types::U256; 3],
        pub rates: [ethers::core::types::U256; 4],
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
    #[ethevent(name = "LenderAccepted", abi = "LenderAccepted(address)")]
    pub struct LenderAcceptedFilter {
        #[ethevent(indexed)]
        pub lender: ethers::core::types::Address,
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
        name = "LoanClosed",
        abi = "LoanClosed(uint256,uint256,uint256,uint256)"
    )]
    pub struct LoanClosedFilter {
        pub principal_paid: ethers::core::types::U256,
        pub interest_paid: ethers::core::types::U256,
        pub delegate_fee_paid: ethers::core::types::U256,
        pub treasury_fee_paid: ethers::core::types::U256,
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
        name = "NewTermsAccepted",
        abi = "NewTermsAccepted(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsAcceptedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "NewTermsProposed",
        abi = "NewTermsProposed(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsProposedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "NewTermsRejected",
        abi = "NewTermsRejected(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsRejectedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "PaymentMade",
        abi = "PaymentMade(uint256,uint256,uint256,uint256)"
    )]
    pub struct PaymentMadeFilter {
        pub principal_paid: ethers::core::types::U256,
        pub interest_paid: ethers::core::types::U256,
        pub delegate_fee_paid: ethers::core::types::U256,
        pub treasury_fee_paid: ethers::core::types::U256,
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
    #[ethevent(name = "PendingBorrowerSet", abi = "PendingBorrowerSet(address)")]
    pub struct PendingBorrowerSetFilter {
        pub pending_borrower: ethers::core::types::Address,
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
    #[ethevent(name = "PendingLenderSet", abi = "PendingLenderSet(address)")]
    pub struct PendingLenderSetFilter {
        pub pending_lender: ethers::core::types::Address,
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
    #[ethevent(name = "Repossessed", abi = "Repossessed(uint256,uint256,address)")]
    pub struct RepossessedFilter {
        pub collateral_repossessed: ethers::core::types::U256,
        pub funds_repossessed: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "Skimmed", abi = "Skimmed(address,uint256,address)")]
    pub struct SkimmedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleLoanInitializerEvents {
        BorrowerAcceptedFilter(BorrowerAcceptedFilter),
        CollateralPostedFilter(CollateralPostedFilter),
        CollateralRemovedFilter(CollateralRemovedFilter),
        EstablishmentFeesSetFilter(EstablishmentFeesSetFilter),
        FundedFilter(FundedFilter),
        FundsClaimedFilter(FundsClaimedFilter),
        FundsDrawnDownFilter(FundsDrawnDownFilter),
        FundsRedirectedFilter(FundsRedirectedFilter),
        FundsReturnedFilter(FundsReturnedFilter),
        InitializedFilter(InitializedFilter),
        LenderAcceptedFilter(LenderAcceptedFilter),
        LoanClosedFilter(LoanClosedFilter),
        NewTermsAcceptedFilter(NewTermsAcceptedFilter),
        NewTermsProposedFilter(NewTermsProposedFilter),
        NewTermsRejectedFilter(NewTermsRejectedFilter),
        PaymentMadeFilter(PaymentMadeFilter),
        PendingBorrowerSetFilter(PendingBorrowerSetFilter),
        PendingLenderSetFilter(PendingLenderSetFilter),
        RepossessedFilter(RepossessedFilter),
        SkimmedFilter(SkimmedFilter),
    }
    impl ethers::contract::EthLogDecode for IMapleLoanInitializerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::BorrowerAcceptedFilter(decoded));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::CollateralPostedFilter(decoded));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::CollateralRemovedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::EstablishmentFeesSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::NewTermsAcceptedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::NewTermsProposedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::NewTermsRejectedFilter(decoded));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::PendingBorrowerSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::PendingLenderSetFilter(decoded));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(IMapleLoanInitializerEvents::SkimmedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IMapleLoanInitializerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleLoanInitializerEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::CollateralPostedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::CollateralRemovedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::FundedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::FundsClaimedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::FundsRedirectedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::FundsReturnedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::InitializedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::LenderAcceptedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::LoanClosedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::NewTermsProposedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::PaymentMadeFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::PendingLenderSetFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::RepossessedFilter(element) => element.fmt(f),
                IMapleLoanInitializerEvents::SkimmedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `decodeArguments`function with signature `decodeArguments(bytes)` and selector `[23, 37, 250, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decodeArguments", abi = "decodeArguments(bytes)")]
    pub struct DecodeArgumentsCall {
        pub encoded_arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `encodeArguments`function with signature `encodeArguments(address,address[2],uint256[3],uint256[3],uint256[4])` and selector `[22, 154, 205, 30]`"]
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
        name = "encodeArguments",
        abi = "encodeArguments(address,address[2],uint256[3],uint256[3],uint256[4])"
    )]
    pub struct EncodeArgumentsCall {
        pub borrower: ethers::core::types::Address,
        pub assets: [ethers::core::types::Address; 2usize],
        pub term_details: [ethers::core::types::U256; 3usize],
        pub amounts: [ethers::core::types::U256; 3usize],
        pub rates: [ethers::core::types::U256; 4usize],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleLoanInitializerCalls {
        DecodeArguments(DecodeArgumentsCall),
        EncodeArguments(EncodeArgumentsCall),
    }
    impl ethers::core::abi::AbiDecode for IMapleLoanInitializerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DecodeArgumentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanInitializerCalls::DecodeArguments(decoded));
            }
            if let Ok(decoded) =
                <EncodeArgumentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanInitializerCalls::EncodeArguments(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMapleLoanInitializerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMapleLoanInitializerCalls::DecodeArguments(element) => element.encode(),
                IMapleLoanInitializerCalls::EncodeArguments(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMapleLoanInitializerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleLoanInitializerCalls::DecodeArguments(element) => element.fmt(f),
                IMapleLoanInitializerCalls::EncodeArguments(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DecodeArgumentsCall> for IMapleLoanInitializerCalls {
        fn from(var: DecodeArgumentsCall) -> Self {
            IMapleLoanInitializerCalls::DecodeArguments(var)
        }
    }
    impl ::std::convert::From<EncodeArgumentsCall> for IMapleLoanInitializerCalls {
        fn from(var: EncodeArgumentsCall) -> Self {
            IMapleLoanInitializerCalls::EncodeArguments(var)
        }
    }
}
