pub use imapleloan_mod::*;
#[allow(clippy::too_many_arguments)]
mod imapleloan_mod {
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
    #[doc = "IMapleLoan was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IMAPLELOAN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptBorrower\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralRequired\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"drawableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drawdownFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earlyFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"endingPrincipal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excessCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"excessCollateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fundsAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawdown_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAdditionalCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"additionalCollateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEarlyPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRefinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"proRataInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isProtocolPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateInterestPremium\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextPaymentDueDate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentInterval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentsRemaining\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingBorrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingLender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalRequested\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"skimmed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"superFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"superFactory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IMAPLELOAN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IMapleLoan<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IMapleLoan<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IMapleLoan<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMapleLoan))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IMapleLoan<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMAPLELOAN_ABI.clone(), client).into()
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
                IMAPLELOAN_ABI.clone(),
                IMAPLELOAN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `acceptBorrower` (0x01daa38f) function"]
        pub fn accept_borrower(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 218, 163, 143], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `acceptLender` (0x0fe3d9b7) function"]
        pub fn accept_lender(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 227, 217, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `acceptNewTerms` (0x50acb4ee) function"]
        pub fn accept_new_terms(
            &self,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 172, 180, 238], (refinancer, deadline, calls, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrower` (0x7df1f1b9) function"]
        pub fn borrower(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 241, 241, 185], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimFunds` (0x390d6855) function"]
        pub fn claim_funds(
            &self,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 13, 104, 85], (amount, destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimableFunds` (0xe44b3875) function"]
        pub fn claimable_funds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([228, 75, 56, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeLoan` (0xd05951a0) function"]
        pub fn close_loan(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([208, 89, 81, 160], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateral` (0xd8dfeb45) function"]
        pub fn collateral(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([216, 223, 235, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateralAsset` (0xaabaecd6) function"]
        pub fn collateral_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([170, 186, 236, 214], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collateralRequired` (0x75a20676) function"]
        pub fn collateral_required(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([117, 162, 6, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateFee` (0xb69410de) function"]
        pub fn delegate_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 148, 16, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drawableFunds` (0xb9b1f4e3) function"]
        pub fn drawable_funds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([185, 177, 244, 227], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drawdownFunds` (0xccc04484) function"]
        pub fn drawdown_funds(
            &self,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([204, 192, 68, 132], (amount, destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `earlyFeeRate` (0x30fea1ce) function"]
        pub fn early_fee_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([48, 254, 161, 206], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `endingPrincipal` (0xb86a513e) function"]
        pub fn ending_principal(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([184, 106, 81, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excessCollateral` (0x7a0e6fa1) function"]
        pub fn excess_collateral(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 14, 111, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fundLoan` (0xe920b1e1) function"]
        pub fn fund_loan(
            &self,
            lender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([233, 32, 177, 225], (lender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fundsAsset` (0x39ba9f86) function"]
        pub fn funds_asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([57, 186, 159, 134], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAdditionalCollateralRequiredFor` (0x4eac4235) function"]
        pub fn get_additional_collateral_required_for(
            &self,
            drawdown: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([78, 172, 66, 53], drawdown)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEarlyPaymentBreakdown` (0x69458ba7) function"]
        pub fn get_early_payment_breakdown(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([105, 69, 139, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextPaymentBreakdown` (0xb96b5c99) function"]
        pub fn get_next_payment_breakdown(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([185, 107, 92, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRefinanceInterest` (0x9e10320b) function"]
        pub fn get_refinance_interest(
            &self,
            timestamp: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([158, 16, 50, 11], timestamp)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gracePeriod` (0xa06db7dc) function"]
        pub fn grace_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([160, 109, 183, 220], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementation` (0x5c60da1b) function"]
        pub fn implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `interestRate` (0x7c3a00fd) function"]
        pub fn interest_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([124, 58, 0, 253], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isProtocolPaused` (0xdac88561) function"]
        pub fn is_protocol_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([218, 200, 133, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lateFeeRate` (0x77b3c55c) function"]
        pub fn late_fee_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([119, 179, 197, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lateInterestPremium` (0x2ead1098) function"]
        pub fn late_interest_premium(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([46, 173, 16, 152], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lender` (0xbcead63e) function"]
        pub fn lender(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([188, 234, 214, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makePayment` (0x5114cb52) function"]
        pub fn make_payment(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([81, 20, 203, 82], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrate` (0xc3fbb6fd) function"]
        pub fn migrate(
            &self,
            migrator: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 251, 182, 253], (migrator, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextPaymentDueDate` (0x4003f34d) function"]
        pub fn next_payment_due_date(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([64, 3, 243, 77], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paymentInterval` (0x1cc1cf46) function"]
        pub fn payment_interval(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 193, 207, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paymentsRemaining` (0x0895326f) function"]
        pub fn payments_remaining(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([8, 149, 50, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingBorrower` (0x700f5006) function"]
        pub fn pending_borrower(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([112, 15, 80, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingLender` (0x5eeb53b4) function"]
        pub fn pending_lender(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 235, 83, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `postCollateral` (0x50f2012f) function"]
        pub fn post_collateral(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([80, 242, 1, 47], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `principal` (0xba5d3078) function"]
        pub fn principal(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 93, 48, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `principalRequested` (0x8ffc9215) function"]
        pub fn principal_requested(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([143, 252, 146, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposeNewTerms` (0x87accaf1) function"]
        pub fn propose_new_terms(
            &self,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 172, 202, 241], (refinancer, deadline, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refinanceCommitment` (0xba83276b) function"]
        pub fn refinance_commitment(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 131, 39, 107], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refinanceInterest` (0xa97d1161) function"]
        pub fn refinance_interest(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([169, 125, 17, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rejectNewTerms` (0xacb522b4) function"]
        pub fn reject_new_terms(
            &self,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 181, 34, 180], (refinancer, deadline, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeCollateral` (0xd41ddc96) function"]
        pub fn remove_collateral(
            &self,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 29, 220, 150], (amount, destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repossess` (0x47350e9f) function"]
        pub fn repossess(
            &self,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([71, 53, 14, 159], destination)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `returnFunds` (0x45755dd6) function"]
        pub fn return_funds(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([69, 117, 93, 214], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setImplementation` (0xd784d426) function"]
        pub fn set_implementation(
            &self,
            new_implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 132, 212, 38], new_implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPendingBorrower` (0x1f3f19ab) function"]
        pub fn set_pending_borrower(
            &self,
            pending_borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 63, 25, 171], pending_borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPendingLender` (0x267f4ac3) function"]
        pub fn set_pending_lender(
            &self,
            pending_lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 127, 74, 195], pending_lender)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `skim` (0x712b772f) function"]
        pub fn skim(
            &self,
            token: ethers::core::types::Address,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([113, 43, 119, 47], (token, destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `superFactory` (0x0d49b38c) function"]
        pub fn super_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 73, 179, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `treasuryFee` (0xcc32d176) function"]
        pub fn treasury_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([204, 50, 209, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgrade` (0x3b99bcee) function"]
        pub fn upgrade(
            &self,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 153, 188, 238], (to_version, arguments))
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
        #[doc = "Gets the contract's `Upgraded` event"]
        pub fn upgraded_filter(&self) -> ethers::contract::builders::Event<M, UpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IMapleLoanEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IMapleLoan<M> {
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
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(uint256,bytes)")]
    pub struct UpgradedFilter {
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleLoanEvents {
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
        UpgradedFilter(UpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for IMapleLoanEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::BorrowerAcceptedFilter(decoded));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::CollateralPostedFilter(decoded));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::CollateralRemovedFilter(decoded));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::EstablishmentFeesSetFilter(decoded));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::NewTermsAcceptedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::NewTermsProposedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::NewTermsRejectedFilter(decoded));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::PendingBorrowerSetFilter(decoded));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::PendingLenderSetFilter(decoded));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::SkimmedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(IMapleLoanEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IMapleLoanEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleLoanEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                IMapleLoanEvents::CollateralPostedFilter(element) => element.fmt(f),
                IMapleLoanEvents::CollateralRemovedFilter(element) => element.fmt(f),
                IMapleLoanEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                IMapleLoanEvents::FundedFilter(element) => element.fmt(f),
                IMapleLoanEvents::FundsClaimedFilter(element) => element.fmt(f),
                IMapleLoanEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                IMapleLoanEvents::FundsRedirectedFilter(element) => element.fmt(f),
                IMapleLoanEvents::FundsReturnedFilter(element) => element.fmt(f),
                IMapleLoanEvents::InitializedFilter(element) => element.fmt(f),
                IMapleLoanEvents::LenderAcceptedFilter(element) => element.fmt(f),
                IMapleLoanEvents::LoanClosedFilter(element) => element.fmt(f),
                IMapleLoanEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                IMapleLoanEvents::NewTermsProposedFilter(element) => element.fmt(f),
                IMapleLoanEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                IMapleLoanEvents::PaymentMadeFilter(element) => element.fmt(f),
                IMapleLoanEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                IMapleLoanEvents::PendingLenderSetFilter(element) => element.fmt(f),
                IMapleLoanEvents::RepossessedFilter(element) => element.fmt(f),
                IMapleLoanEvents::SkimmedFilter(element) => element.fmt(f),
                IMapleLoanEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptBorrower`function with signature `acceptBorrower()` and selector `[1, 218, 163, 143]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptBorrower", abi = "acceptBorrower()")]
    pub struct AcceptBorrowerCall;
    #[doc = "Container type for all input parameters for the `acceptLender`function with signature `acceptLender()` and selector `[15, 227, 217, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptLender", abi = "acceptLender()")]
    pub struct AcceptLenderCall;
    #[doc = "Container type for all input parameters for the `acceptNewTerms`function with signature `acceptNewTerms(address,uint256,bytes[],uint256)` and selector `[80, 172, 180, 238]`"]
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
        name = "acceptNewTerms",
        abi = "acceptNewTerms(address,uint256,bytes[],uint256)"
    )]
    pub struct AcceptNewTermsCall {
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrower`function with signature `borrower()` and selector `[125, 241, 241, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrower", abi = "borrower()")]
    pub struct BorrowerCall;
    #[doc = "Container type for all input parameters for the `claimFunds`function with signature `claimFunds(uint256,address)` and selector `[57, 13, 104, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimFunds", abi = "claimFunds(uint256,address)")]
    pub struct ClaimFundsCall {
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimableFunds`function with signature `claimableFunds()` and selector `[228, 75, 56, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimableFunds", abi = "claimableFunds()")]
    pub struct ClaimableFundsCall;
    #[doc = "Container type for all input parameters for the `closeLoan`function with signature `closeLoan(uint256)` and selector `[208, 89, 81, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "closeLoan", abi = "closeLoan(uint256)")]
    pub struct CloseLoanCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `collateral`function with signature `collateral()` and selector `[216, 223, 235, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collateral", abi = "collateral()")]
    pub struct CollateralCall;
    #[doc = "Container type for all input parameters for the `collateralAsset`function with signature `collateralAsset()` and selector `[170, 186, 236, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collateralAsset", abi = "collateralAsset()")]
    pub struct CollateralAssetCall;
    #[doc = "Container type for all input parameters for the `collateralRequired`function with signature `collateralRequired()` and selector `[117, 162, 6, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "collateralRequired", abi = "collateralRequired()")]
    pub struct CollateralRequiredCall;
    #[doc = "Container type for all input parameters for the `delegateFee`function with signature `delegateFee()` and selector `[182, 148, 16, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delegateFee", abi = "delegateFee()")]
    pub struct DelegateFeeCall;
    #[doc = "Container type for all input parameters for the `drawableFunds`function with signature `drawableFunds()` and selector `[185, 177, 244, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "drawableFunds", abi = "drawableFunds()")]
    pub struct DrawableFundsCall;
    #[doc = "Container type for all input parameters for the `drawdownFunds`function with signature `drawdownFunds(uint256,address)` and selector `[204, 192, 68, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "drawdownFunds", abi = "drawdownFunds(uint256,address)")]
    pub struct DrawdownFundsCall {
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `earlyFeeRate`function with signature `earlyFeeRate()` and selector `[48, 254, 161, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "earlyFeeRate", abi = "earlyFeeRate()")]
    pub struct EarlyFeeRateCall;
    #[doc = "Container type for all input parameters for the `endingPrincipal`function with signature `endingPrincipal()` and selector `[184, 106, 81, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "endingPrincipal", abi = "endingPrincipal()")]
    pub struct EndingPrincipalCall;
    #[doc = "Container type for all input parameters for the `excessCollateral`function with signature `excessCollateral()` and selector `[122, 14, 111, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "excessCollateral", abi = "excessCollateral()")]
    pub struct ExcessCollateralCall;
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `fundLoan`function with signature `fundLoan(address,uint256)` and selector `[233, 32, 177, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fundLoan", abi = "fundLoan(address,uint256)")]
    pub struct FundLoanCall {
        pub lender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fundsAsset`function with signature `fundsAsset()` and selector `[57, 186, 159, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fundsAsset", abi = "fundsAsset()")]
    pub struct FundsAssetCall;
    #[doc = "Container type for all input parameters for the `getAdditionalCollateralRequiredFor`function with signature `getAdditionalCollateralRequiredFor(uint256)` and selector `[78, 172, 66, 53]`"]
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
        name = "getAdditionalCollateralRequiredFor",
        abi = "getAdditionalCollateralRequiredFor(uint256)"
    )]
    pub struct GetAdditionalCollateralRequiredForCall {
        pub drawdown: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getEarlyPaymentBreakdown`function with signature `getEarlyPaymentBreakdown()` and selector `[105, 69, 139, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEarlyPaymentBreakdown", abi = "getEarlyPaymentBreakdown()")]
    pub struct GetEarlyPaymentBreakdownCall;
    #[doc = "Container type for all input parameters for the `getNextPaymentBreakdown`function with signature `getNextPaymentBreakdown()` and selector `[185, 107, 92, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNextPaymentBreakdown", abi = "getNextPaymentBreakdown()")]
    pub struct GetNextPaymentBreakdownCall;
    #[doc = "Container type for all input parameters for the `getRefinanceInterest`function with signature `getRefinanceInterest(uint256)` and selector `[158, 16, 50, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRefinanceInterest", abi = "getRefinanceInterest(uint256)")]
    pub struct GetRefinanceInterestCall {
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `gracePeriod`function with signature `gracePeriod()` and selector `[160, 109, 183, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gracePeriod", abi = "gracePeriod()")]
    pub struct GracePeriodCall;
    #[doc = "Container type for all input parameters for the `implementation`function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    #[doc = "Container type for all input parameters for the `interestRate`function with signature `interestRate()` and selector `[124, 58, 0, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "interestRate", abi = "interestRate()")]
    pub struct InterestRateCall;
    #[doc = "Container type for all input parameters for the `isProtocolPaused`function with signature `isProtocolPaused()` and selector `[218, 200, 133, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isProtocolPaused", abi = "isProtocolPaused()")]
    pub struct IsProtocolPausedCall;
    #[doc = "Container type for all input parameters for the `lateFeeRate`function with signature `lateFeeRate()` and selector `[119, 179, 197, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lateFeeRate", abi = "lateFeeRate()")]
    pub struct LateFeeRateCall;
    #[doc = "Container type for all input parameters for the `lateInterestPremium`function with signature `lateInterestPremium()` and selector `[46, 173, 16, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lateInterestPremium", abi = "lateInterestPremium()")]
    pub struct LateInterestPremiumCall;
    #[doc = "Container type for all input parameters for the `lender`function with signature `lender()` and selector `[188, 234, 214, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lender", abi = "lender()")]
    pub struct LenderCall;
    #[doc = "Container type for all input parameters for the `makePayment`function with signature `makePayment(uint256)` and selector `[81, 20, 203, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "makePayment", abi = "makePayment(uint256)")]
    pub struct MakePaymentCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `migrate`function with signature `migrate(address,bytes)` and selector `[195, 251, 182, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "migrate", abi = "migrate(address,bytes)")]
    pub struct MigrateCall {
        pub migrator: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `nextPaymentDueDate`function with signature `nextPaymentDueDate()` and selector `[64, 3, 243, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nextPaymentDueDate", abi = "nextPaymentDueDate()")]
    pub struct NextPaymentDueDateCall;
    #[doc = "Container type for all input parameters for the `paymentInterval`function with signature `paymentInterval()` and selector `[28, 193, 207, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paymentInterval", abi = "paymentInterval()")]
    pub struct PaymentIntervalCall;
    #[doc = "Container type for all input parameters for the `paymentsRemaining`function with signature `paymentsRemaining()` and selector `[8, 149, 50, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paymentsRemaining", abi = "paymentsRemaining()")]
    pub struct PaymentsRemainingCall;
    #[doc = "Container type for all input parameters for the `pendingBorrower`function with signature `pendingBorrower()` and selector `[112, 15, 80, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingBorrower", abi = "pendingBorrower()")]
    pub struct PendingBorrowerCall;
    #[doc = "Container type for all input parameters for the `pendingLender`function with signature `pendingLender()` and selector `[94, 235, 83, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingLender", abi = "pendingLender()")]
    pub struct PendingLenderCall;
    #[doc = "Container type for all input parameters for the `postCollateral`function with signature `postCollateral(uint256)` and selector `[80, 242, 1, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "postCollateral", abi = "postCollateral(uint256)")]
    pub struct PostCollateralCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `principal`function with signature `principal()` and selector `[186, 93, 48, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "principal", abi = "principal()")]
    pub struct PrincipalCall;
    #[doc = "Container type for all input parameters for the `principalRequested`function with signature `principalRequested()` and selector `[143, 252, 146, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "principalRequested", abi = "principalRequested()")]
    pub struct PrincipalRequestedCall;
    #[doc = "Container type for all input parameters for the `proposeNewTerms`function with signature `proposeNewTerms(address,uint256,bytes[])` and selector `[135, 172, 202, 241]`"]
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
        name = "proposeNewTerms",
        abi = "proposeNewTerms(address,uint256,bytes[])"
    )]
    pub struct ProposeNewTermsCall {
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `refinanceCommitment`function with signature `refinanceCommitment()` and selector `[186, 131, 39, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "refinanceCommitment", abi = "refinanceCommitment()")]
    pub struct RefinanceCommitmentCall;
    #[doc = "Container type for all input parameters for the `refinanceInterest`function with signature `refinanceInterest()` and selector `[169, 125, 17, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "refinanceInterest", abi = "refinanceInterest()")]
    pub struct RefinanceInterestCall;
    #[doc = "Container type for all input parameters for the `rejectNewTerms`function with signature `rejectNewTerms(address,uint256,bytes[])` and selector `[172, 181, 34, 180]`"]
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
        name = "rejectNewTerms",
        abi = "rejectNewTerms(address,uint256,bytes[])"
    )]
    pub struct RejectNewTermsCall {
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `removeCollateral`function with signature `removeCollateral(uint256,address)` and selector `[212, 29, 220, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeCollateral", abi = "removeCollateral(uint256,address)")]
    pub struct RemoveCollateralCall {
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repossess`function with signature `repossess(address)` and selector `[71, 53, 14, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repossess", abi = "repossess(address)")]
    pub struct RepossessCall {
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `returnFunds`function with signature `returnFunds(uint256)` and selector `[69, 117, 93, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "returnFunds", abi = "returnFunds(uint256)")]
    pub struct ReturnFundsCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setImplementation`function with signature `setImplementation(address)` and selector `[215, 132, 212, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setImplementation", abi = "setImplementation(address)")]
    pub struct SetImplementationCall {
        pub new_implementation: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPendingBorrower`function with signature `setPendingBorrower(address)` and selector `[31, 63, 25, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPendingBorrower", abi = "setPendingBorrower(address)")]
    pub struct SetPendingBorrowerCall {
        pub pending_borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPendingLender`function with signature `setPendingLender(address)` and selector `[38, 127, 74, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPendingLender", abi = "setPendingLender(address)")]
    pub struct SetPendingLenderCall {
        pub pending_lender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `skim`function with signature `skim(address,address)` and selector `[113, 43, 119, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "skim", abi = "skim(address,address)")]
    pub struct SkimCall {
        pub token: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `superFactory`function with signature `superFactory()` and selector `[13, 73, 179, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "superFactory", abi = "superFactory()")]
    pub struct SuperFactoryCall;
    #[doc = "Container type for all input parameters for the `treasuryFee`function with signature `treasuryFee()` and selector `[204, 50, 209, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "treasuryFee", abi = "treasuryFee()")]
    pub struct TreasuryFeeCall;
    #[doc = "Container type for all input parameters for the `upgrade`function with signature `upgrade(uint256,bytes)` and selector `[59, 153, 188, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(uint256,bytes)")]
    pub struct UpgradeCall {
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMapleLoanCalls {
        AcceptBorrower(AcceptBorrowerCall),
        AcceptLender(AcceptLenderCall),
        AcceptNewTerms(AcceptNewTermsCall),
        Borrower(BorrowerCall),
        ClaimFunds(ClaimFundsCall),
        ClaimableFunds(ClaimableFundsCall),
        CloseLoan(CloseLoanCall),
        Collateral(CollateralCall),
        CollateralAsset(CollateralAssetCall),
        CollateralRequired(CollateralRequiredCall),
        DelegateFee(DelegateFeeCall),
        DrawableFunds(DrawableFundsCall),
        DrawdownFunds(DrawdownFundsCall),
        EarlyFeeRate(EarlyFeeRateCall),
        EndingPrincipal(EndingPrincipalCall),
        ExcessCollateral(ExcessCollateralCall),
        Factory(FactoryCall),
        FundLoan(FundLoanCall),
        FundsAsset(FundsAssetCall),
        GetAdditionalCollateralRequiredFor(GetAdditionalCollateralRequiredForCall),
        GetEarlyPaymentBreakdown(GetEarlyPaymentBreakdownCall),
        GetNextPaymentBreakdown(GetNextPaymentBreakdownCall),
        GetRefinanceInterest(GetRefinanceInterestCall),
        GracePeriod(GracePeriodCall),
        Implementation(ImplementationCall),
        InterestRate(InterestRateCall),
        IsProtocolPaused(IsProtocolPausedCall),
        LateFeeRate(LateFeeRateCall),
        LateInterestPremium(LateInterestPremiumCall),
        Lender(LenderCall),
        MakePayment(MakePaymentCall),
        Migrate(MigrateCall),
        NextPaymentDueDate(NextPaymentDueDateCall),
        PaymentInterval(PaymentIntervalCall),
        PaymentsRemaining(PaymentsRemainingCall),
        PendingBorrower(PendingBorrowerCall),
        PendingLender(PendingLenderCall),
        PostCollateral(PostCollateralCall),
        Principal(PrincipalCall),
        PrincipalRequested(PrincipalRequestedCall),
        ProposeNewTerms(ProposeNewTermsCall),
        RefinanceCommitment(RefinanceCommitmentCall),
        RefinanceInterest(RefinanceInterestCall),
        RejectNewTerms(RejectNewTermsCall),
        RemoveCollateral(RemoveCollateralCall),
        Repossess(RepossessCall),
        ReturnFunds(ReturnFundsCall),
        SetImplementation(SetImplementationCall),
        SetPendingBorrower(SetPendingBorrowerCall),
        SetPendingLender(SetPendingLenderCall),
        Skim(SkimCall),
        SuperFactory(SuperFactoryCall),
        TreasuryFee(TreasuryFeeCall),
        Upgrade(UpgradeCall),
    }
    impl ethers::core::abi::AbiDecode for IMapleLoanCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::AcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <AcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::AcceptLender(decoded));
            }
            if let Ok(decoded) =
                <AcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::AcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <BorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Borrower(decoded));
            }
            if let Ok(decoded) =
                <ClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::ClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::ClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <CloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::CloseLoan(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Collateral(decoded));
            }
            if let Ok(decoded) =
                <CollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::CollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <CollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::CollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <DelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::DelegateFee(decoded));
            }
            if let Ok(decoded) =
                <DrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::DrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <DrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::DrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <EarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::EarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <EndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::EndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <ExcessCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::ExcessCollateral(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::FundLoan(decoded));
            }
            if let Ok(decoded) =
                <FundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::FundsAsset(decoded));
            }
            if let Ok(decoded) =
                <GetAdditionalCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMapleLoanCalls::GetAdditionalCollateralRequiredFor(decoded));
            }
            if let Ok(decoded) =
                <GetEarlyPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IMapleLoanCalls::GetEarlyPaymentBreakdown(decoded));
            }
            if let Ok(decoded) =
                <GetNextPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::GetNextPaymentBreakdown(decoded));
            }
            if let Ok(decoded) =
                <GetRefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::GetRefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::InterestRate(decoded));
            }
            if let Ok(decoded) =
                <IsProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::IsProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <LateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::LateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <LateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::LateInterestPremium(decoded));
            }
            if let Ok(decoded) = <LenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Lender(decoded));
            }
            if let Ok(decoded) =
                <MakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::MakePayment(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <NextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::NextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <PaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <PaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <PendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <PendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PendingLender(decoded));
            }
            if let Ok(decoded) =
                <PostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PostCollateral(decoded));
            }
            if let Ok(decoded) =
                <PrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Principal(decoded));
            }
            if let Ok(decoded) =
                <PrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::PrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::ProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::RefinanceCommitment(decoded));
            }
            if let Ok(decoded) =
                <RefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::RefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <RejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::RejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <RepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Repossess(decoded));
            }
            if let Ok(decoded) =
                <ReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::ReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::SetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::SetPendingLender(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMapleLoanCalls::Skim(decoded));
            }
            if let Ok(decoded) =
                <SuperFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::SuperFactory(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::TreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMapleLoanCalls::Upgrade(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMapleLoanCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMapleLoanCalls::AcceptBorrower(element) => element.encode(),
                IMapleLoanCalls::AcceptLender(element) => element.encode(),
                IMapleLoanCalls::AcceptNewTerms(element) => element.encode(),
                IMapleLoanCalls::Borrower(element) => element.encode(),
                IMapleLoanCalls::ClaimFunds(element) => element.encode(),
                IMapleLoanCalls::ClaimableFunds(element) => element.encode(),
                IMapleLoanCalls::CloseLoan(element) => element.encode(),
                IMapleLoanCalls::Collateral(element) => element.encode(),
                IMapleLoanCalls::CollateralAsset(element) => element.encode(),
                IMapleLoanCalls::CollateralRequired(element) => element.encode(),
                IMapleLoanCalls::DelegateFee(element) => element.encode(),
                IMapleLoanCalls::DrawableFunds(element) => element.encode(),
                IMapleLoanCalls::DrawdownFunds(element) => element.encode(),
                IMapleLoanCalls::EarlyFeeRate(element) => element.encode(),
                IMapleLoanCalls::EndingPrincipal(element) => element.encode(),
                IMapleLoanCalls::ExcessCollateral(element) => element.encode(),
                IMapleLoanCalls::Factory(element) => element.encode(),
                IMapleLoanCalls::FundLoan(element) => element.encode(),
                IMapleLoanCalls::FundsAsset(element) => element.encode(),
                IMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => element.encode(),
                IMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.encode(),
                IMapleLoanCalls::GetNextPaymentBreakdown(element) => element.encode(),
                IMapleLoanCalls::GetRefinanceInterest(element) => element.encode(),
                IMapleLoanCalls::GracePeriod(element) => element.encode(),
                IMapleLoanCalls::Implementation(element) => element.encode(),
                IMapleLoanCalls::InterestRate(element) => element.encode(),
                IMapleLoanCalls::IsProtocolPaused(element) => element.encode(),
                IMapleLoanCalls::LateFeeRate(element) => element.encode(),
                IMapleLoanCalls::LateInterestPremium(element) => element.encode(),
                IMapleLoanCalls::Lender(element) => element.encode(),
                IMapleLoanCalls::MakePayment(element) => element.encode(),
                IMapleLoanCalls::Migrate(element) => element.encode(),
                IMapleLoanCalls::NextPaymentDueDate(element) => element.encode(),
                IMapleLoanCalls::PaymentInterval(element) => element.encode(),
                IMapleLoanCalls::PaymentsRemaining(element) => element.encode(),
                IMapleLoanCalls::PendingBorrower(element) => element.encode(),
                IMapleLoanCalls::PendingLender(element) => element.encode(),
                IMapleLoanCalls::PostCollateral(element) => element.encode(),
                IMapleLoanCalls::Principal(element) => element.encode(),
                IMapleLoanCalls::PrincipalRequested(element) => element.encode(),
                IMapleLoanCalls::ProposeNewTerms(element) => element.encode(),
                IMapleLoanCalls::RefinanceCommitment(element) => element.encode(),
                IMapleLoanCalls::RefinanceInterest(element) => element.encode(),
                IMapleLoanCalls::RejectNewTerms(element) => element.encode(),
                IMapleLoanCalls::RemoveCollateral(element) => element.encode(),
                IMapleLoanCalls::Repossess(element) => element.encode(),
                IMapleLoanCalls::ReturnFunds(element) => element.encode(),
                IMapleLoanCalls::SetImplementation(element) => element.encode(),
                IMapleLoanCalls::SetPendingBorrower(element) => element.encode(),
                IMapleLoanCalls::SetPendingLender(element) => element.encode(),
                IMapleLoanCalls::Skim(element) => element.encode(),
                IMapleLoanCalls::SuperFactory(element) => element.encode(),
                IMapleLoanCalls::TreasuryFee(element) => element.encode(),
                IMapleLoanCalls::Upgrade(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMapleLoanCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMapleLoanCalls::AcceptBorrower(element) => element.fmt(f),
                IMapleLoanCalls::AcceptLender(element) => element.fmt(f),
                IMapleLoanCalls::AcceptNewTerms(element) => element.fmt(f),
                IMapleLoanCalls::Borrower(element) => element.fmt(f),
                IMapleLoanCalls::ClaimFunds(element) => element.fmt(f),
                IMapleLoanCalls::ClaimableFunds(element) => element.fmt(f),
                IMapleLoanCalls::CloseLoan(element) => element.fmt(f),
                IMapleLoanCalls::Collateral(element) => element.fmt(f),
                IMapleLoanCalls::CollateralAsset(element) => element.fmt(f),
                IMapleLoanCalls::CollateralRequired(element) => element.fmt(f),
                IMapleLoanCalls::DelegateFee(element) => element.fmt(f),
                IMapleLoanCalls::DrawableFunds(element) => element.fmt(f),
                IMapleLoanCalls::DrawdownFunds(element) => element.fmt(f),
                IMapleLoanCalls::EarlyFeeRate(element) => element.fmt(f),
                IMapleLoanCalls::EndingPrincipal(element) => element.fmt(f),
                IMapleLoanCalls::ExcessCollateral(element) => element.fmt(f),
                IMapleLoanCalls::Factory(element) => element.fmt(f),
                IMapleLoanCalls::FundLoan(element) => element.fmt(f),
                IMapleLoanCalls::FundsAsset(element) => element.fmt(f),
                IMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => element.fmt(f),
                IMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.fmt(f),
                IMapleLoanCalls::GetNextPaymentBreakdown(element) => element.fmt(f),
                IMapleLoanCalls::GetRefinanceInterest(element) => element.fmt(f),
                IMapleLoanCalls::GracePeriod(element) => element.fmt(f),
                IMapleLoanCalls::Implementation(element) => element.fmt(f),
                IMapleLoanCalls::InterestRate(element) => element.fmt(f),
                IMapleLoanCalls::IsProtocolPaused(element) => element.fmt(f),
                IMapleLoanCalls::LateFeeRate(element) => element.fmt(f),
                IMapleLoanCalls::LateInterestPremium(element) => element.fmt(f),
                IMapleLoanCalls::Lender(element) => element.fmt(f),
                IMapleLoanCalls::MakePayment(element) => element.fmt(f),
                IMapleLoanCalls::Migrate(element) => element.fmt(f),
                IMapleLoanCalls::NextPaymentDueDate(element) => element.fmt(f),
                IMapleLoanCalls::PaymentInterval(element) => element.fmt(f),
                IMapleLoanCalls::PaymentsRemaining(element) => element.fmt(f),
                IMapleLoanCalls::PendingBorrower(element) => element.fmt(f),
                IMapleLoanCalls::PendingLender(element) => element.fmt(f),
                IMapleLoanCalls::PostCollateral(element) => element.fmt(f),
                IMapleLoanCalls::Principal(element) => element.fmt(f),
                IMapleLoanCalls::PrincipalRequested(element) => element.fmt(f),
                IMapleLoanCalls::ProposeNewTerms(element) => element.fmt(f),
                IMapleLoanCalls::RefinanceCommitment(element) => element.fmt(f),
                IMapleLoanCalls::RefinanceInterest(element) => element.fmt(f),
                IMapleLoanCalls::RejectNewTerms(element) => element.fmt(f),
                IMapleLoanCalls::RemoveCollateral(element) => element.fmt(f),
                IMapleLoanCalls::Repossess(element) => element.fmt(f),
                IMapleLoanCalls::ReturnFunds(element) => element.fmt(f),
                IMapleLoanCalls::SetImplementation(element) => element.fmt(f),
                IMapleLoanCalls::SetPendingBorrower(element) => element.fmt(f),
                IMapleLoanCalls::SetPendingLender(element) => element.fmt(f),
                IMapleLoanCalls::Skim(element) => element.fmt(f),
                IMapleLoanCalls::SuperFactory(element) => element.fmt(f),
                IMapleLoanCalls::TreasuryFee(element) => element.fmt(f),
                IMapleLoanCalls::Upgrade(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptBorrowerCall> for IMapleLoanCalls {
        fn from(var: AcceptBorrowerCall) -> Self {
            IMapleLoanCalls::AcceptBorrower(var)
        }
    }
    impl ::std::convert::From<AcceptLenderCall> for IMapleLoanCalls {
        fn from(var: AcceptLenderCall) -> Self {
            IMapleLoanCalls::AcceptLender(var)
        }
    }
    impl ::std::convert::From<AcceptNewTermsCall> for IMapleLoanCalls {
        fn from(var: AcceptNewTermsCall) -> Self {
            IMapleLoanCalls::AcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<BorrowerCall> for IMapleLoanCalls {
        fn from(var: BorrowerCall) -> Self {
            IMapleLoanCalls::Borrower(var)
        }
    }
    impl ::std::convert::From<ClaimFundsCall> for IMapleLoanCalls {
        fn from(var: ClaimFundsCall) -> Self {
            IMapleLoanCalls::ClaimFunds(var)
        }
    }
    impl ::std::convert::From<ClaimableFundsCall> for IMapleLoanCalls {
        fn from(var: ClaimableFundsCall) -> Self {
            IMapleLoanCalls::ClaimableFunds(var)
        }
    }
    impl ::std::convert::From<CloseLoanCall> for IMapleLoanCalls {
        fn from(var: CloseLoanCall) -> Self {
            IMapleLoanCalls::CloseLoan(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for IMapleLoanCalls {
        fn from(var: CollateralCall) -> Self {
            IMapleLoanCalls::Collateral(var)
        }
    }
    impl ::std::convert::From<CollateralAssetCall> for IMapleLoanCalls {
        fn from(var: CollateralAssetCall) -> Self {
            IMapleLoanCalls::CollateralAsset(var)
        }
    }
    impl ::std::convert::From<CollateralRequiredCall> for IMapleLoanCalls {
        fn from(var: CollateralRequiredCall) -> Self {
            IMapleLoanCalls::CollateralRequired(var)
        }
    }
    impl ::std::convert::From<DelegateFeeCall> for IMapleLoanCalls {
        fn from(var: DelegateFeeCall) -> Self {
            IMapleLoanCalls::DelegateFee(var)
        }
    }
    impl ::std::convert::From<DrawableFundsCall> for IMapleLoanCalls {
        fn from(var: DrawableFundsCall) -> Self {
            IMapleLoanCalls::DrawableFunds(var)
        }
    }
    impl ::std::convert::From<DrawdownFundsCall> for IMapleLoanCalls {
        fn from(var: DrawdownFundsCall) -> Self {
            IMapleLoanCalls::DrawdownFunds(var)
        }
    }
    impl ::std::convert::From<EarlyFeeRateCall> for IMapleLoanCalls {
        fn from(var: EarlyFeeRateCall) -> Self {
            IMapleLoanCalls::EarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<EndingPrincipalCall> for IMapleLoanCalls {
        fn from(var: EndingPrincipalCall) -> Self {
            IMapleLoanCalls::EndingPrincipal(var)
        }
    }
    impl ::std::convert::From<ExcessCollateralCall> for IMapleLoanCalls {
        fn from(var: ExcessCollateralCall) -> Self {
            IMapleLoanCalls::ExcessCollateral(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IMapleLoanCalls {
        fn from(var: FactoryCall) -> Self {
            IMapleLoanCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FundLoanCall> for IMapleLoanCalls {
        fn from(var: FundLoanCall) -> Self {
            IMapleLoanCalls::FundLoan(var)
        }
    }
    impl ::std::convert::From<FundsAssetCall> for IMapleLoanCalls {
        fn from(var: FundsAssetCall) -> Self {
            IMapleLoanCalls::FundsAsset(var)
        }
    }
    impl ::std::convert::From<GetAdditionalCollateralRequiredForCall> for IMapleLoanCalls {
        fn from(var: GetAdditionalCollateralRequiredForCall) -> Self {
            IMapleLoanCalls::GetAdditionalCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetEarlyPaymentBreakdownCall> for IMapleLoanCalls {
        fn from(var: GetEarlyPaymentBreakdownCall) -> Self {
            IMapleLoanCalls::GetEarlyPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetNextPaymentBreakdownCall> for IMapleLoanCalls {
        fn from(var: GetNextPaymentBreakdownCall) -> Self {
            IMapleLoanCalls::GetNextPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetRefinanceInterestCall> for IMapleLoanCalls {
        fn from(var: GetRefinanceInterestCall) -> Self {
            IMapleLoanCalls::GetRefinanceInterest(var)
        }
    }
    impl ::std::convert::From<GracePeriodCall> for IMapleLoanCalls {
        fn from(var: GracePeriodCall) -> Self {
            IMapleLoanCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for IMapleLoanCalls {
        fn from(var: ImplementationCall) -> Self {
            IMapleLoanCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InterestRateCall> for IMapleLoanCalls {
        fn from(var: InterestRateCall) -> Self {
            IMapleLoanCalls::InterestRate(var)
        }
    }
    impl ::std::convert::From<IsProtocolPausedCall> for IMapleLoanCalls {
        fn from(var: IsProtocolPausedCall) -> Self {
            IMapleLoanCalls::IsProtocolPaused(var)
        }
    }
    impl ::std::convert::From<LateFeeRateCall> for IMapleLoanCalls {
        fn from(var: LateFeeRateCall) -> Self {
            IMapleLoanCalls::LateFeeRate(var)
        }
    }
    impl ::std::convert::From<LateInterestPremiumCall> for IMapleLoanCalls {
        fn from(var: LateInterestPremiumCall) -> Self {
            IMapleLoanCalls::LateInterestPremium(var)
        }
    }
    impl ::std::convert::From<LenderCall> for IMapleLoanCalls {
        fn from(var: LenderCall) -> Self {
            IMapleLoanCalls::Lender(var)
        }
    }
    impl ::std::convert::From<MakePaymentCall> for IMapleLoanCalls {
        fn from(var: MakePaymentCall) -> Self {
            IMapleLoanCalls::MakePayment(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for IMapleLoanCalls {
        fn from(var: MigrateCall) -> Self {
            IMapleLoanCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<NextPaymentDueDateCall> for IMapleLoanCalls {
        fn from(var: NextPaymentDueDateCall) -> Self {
            IMapleLoanCalls::NextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<PaymentIntervalCall> for IMapleLoanCalls {
        fn from(var: PaymentIntervalCall) -> Self {
            IMapleLoanCalls::PaymentInterval(var)
        }
    }
    impl ::std::convert::From<PaymentsRemainingCall> for IMapleLoanCalls {
        fn from(var: PaymentsRemainingCall) -> Self {
            IMapleLoanCalls::PaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<PendingBorrowerCall> for IMapleLoanCalls {
        fn from(var: PendingBorrowerCall) -> Self {
            IMapleLoanCalls::PendingBorrower(var)
        }
    }
    impl ::std::convert::From<PendingLenderCall> for IMapleLoanCalls {
        fn from(var: PendingLenderCall) -> Self {
            IMapleLoanCalls::PendingLender(var)
        }
    }
    impl ::std::convert::From<PostCollateralCall> for IMapleLoanCalls {
        fn from(var: PostCollateralCall) -> Self {
            IMapleLoanCalls::PostCollateral(var)
        }
    }
    impl ::std::convert::From<PrincipalCall> for IMapleLoanCalls {
        fn from(var: PrincipalCall) -> Self {
            IMapleLoanCalls::Principal(var)
        }
    }
    impl ::std::convert::From<PrincipalRequestedCall> for IMapleLoanCalls {
        fn from(var: PrincipalRequestedCall) -> Self {
            IMapleLoanCalls::PrincipalRequested(var)
        }
    }
    impl ::std::convert::From<ProposeNewTermsCall> for IMapleLoanCalls {
        fn from(var: ProposeNewTermsCall) -> Self {
            IMapleLoanCalls::ProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<RefinanceCommitmentCall> for IMapleLoanCalls {
        fn from(var: RefinanceCommitmentCall) -> Self {
            IMapleLoanCalls::RefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<RefinanceInterestCall> for IMapleLoanCalls {
        fn from(var: RefinanceInterestCall) -> Self {
            IMapleLoanCalls::RefinanceInterest(var)
        }
    }
    impl ::std::convert::From<RejectNewTermsCall> for IMapleLoanCalls {
        fn from(var: RejectNewTermsCall) -> Self {
            IMapleLoanCalls::RejectNewTerms(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for IMapleLoanCalls {
        fn from(var: RemoveCollateralCall) -> Self {
            IMapleLoanCalls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepossessCall> for IMapleLoanCalls {
        fn from(var: RepossessCall) -> Self {
            IMapleLoanCalls::Repossess(var)
        }
    }
    impl ::std::convert::From<ReturnFundsCall> for IMapleLoanCalls {
        fn from(var: ReturnFundsCall) -> Self {
            IMapleLoanCalls::ReturnFunds(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for IMapleLoanCalls {
        fn from(var: SetImplementationCall) -> Self {
            IMapleLoanCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<SetPendingBorrowerCall> for IMapleLoanCalls {
        fn from(var: SetPendingBorrowerCall) -> Self {
            IMapleLoanCalls::SetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<SetPendingLenderCall> for IMapleLoanCalls {
        fn from(var: SetPendingLenderCall) -> Self {
            IMapleLoanCalls::SetPendingLender(var)
        }
    }
    impl ::std::convert::From<SkimCall> for IMapleLoanCalls {
        fn from(var: SkimCall) -> Self {
            IMapleLoanCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SuperFactoryCall> for IMapleLoanCalls {
        fn from(var: SuperFactoryCall) -> Self {
            IMapleLoanCalls::SuperFactory(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for IMapleLoanCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            IMapleLoanCalls::TreasuryFee(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for IMapleLoanCalls {
        fn from(var: UpgradeCall) -> Self {
            IMapleLoanCalls::Upgrade(var)
        }
    }
}
