pub use constructablemapleloan_mod::*;
#[allow(clippy::too_many_arguments)]
mod constructablemapleloan_mod {
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
    #[doc = "ConstructableMapleLoan was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONSTRUCTABLEMAPLELOAN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setFactory\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptBorrower\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralRequired\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"drawableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drawdownFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earlyFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"endingPrincipal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excessCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"excessCollateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fundsAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawdown_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAdditionalCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEarlyPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRefinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"proRataInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isProtocolPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateInterestPremium\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextPaymentDueDate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentInterval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentsRemaining\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingBorrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingLender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalRequested\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"skimmed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"superFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"superFactory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONSTRUCTABLEMAPLELOAN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162003f3138038062003f3183398101604081905262000034916200057d565b62000055866001600160a01b031660008051602062003f1183398151915255565b62000064858585858562000070565b5050505050506200071d565b6020820151620000c75760405162461bcd60e51b815260206004820152601760248201527f4d4c493a493a494e56414c49445f5052494e434950414c00000000000000000060448201526064015b60405180910390fd5b602082015160408301511115620001215760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a493a494e56414c49445f454e44494e475f5052494e434950414c00006044820152606401620000be565b600080546001600160a01b0319166001600160a01b0387169081179091556200018d5760405162461bcd60e51b815260206004820152601660248201527f4d4c493a493a494e56414c49445f424f52524f574552000000000000000000006044820152606401620000be565b8351600480546001600160a01b03199081166001600160a01b039384161790915560208087015160058054909316931692909217905583516006558381015160078190556040808601516013558451600c5584830151600d81905585820151600e55845160085592840151600955830151600a556060830151600b556200021891906000806200021f565b5050505050565b60006200022b62000397565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b1580156200026f57600080fd5b505afa15801562000284573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002aa919062000656565b620002b69089620006e5565b620002c29190620006e5565b620002ce9190620006c2565b620002da9190620006a7565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b1580156200032257600080fd5b505afa15801562000337573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200035d919062000656565b620003699089620006e5565b620003759190620006e5565b620003819190620006c2565b6200038d9190620006a7565b6018555050505050565b6000620003a36200041c565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b158015620003dc57600080fd5b505afa158015620003f1573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000417919062000558565b905090565b60006200043660008051602062003f118339815191525490565b919050565b80516001600160a01b03811681146200043657600080fd5b600082601f8301126200046557600080fd5b604051606081016001600160401b03811182821017156200049657634e487b7160e01b600052604160045260246000fd5b604052808360608101861015620004ac57600080fd5b60005b6003811015620004d0578151835260209283019290910190600101620004af565b509195945050505050565b600082601f830112620004ed57600080fd5b604051608081016001600160401b03811182821017156200051e57634e487b7160e01b600052604160045260246000fd5b6040528083608081018610156200053457600080fd5b60005b6004811015620004d057815183526020928301929091019060010162000537565b6000602082840312156200056b57600080fd5b62000576826200043b565b9392505050565b6000806000806000806101c087890312156200059857600080fd5b620005a3876200043b565b95506020620005b48189016200043b565b955088605f890112620005c657600080fd5b620005d062000670565b8060408a0160808b018c811115620005e757600080fd5b60005b60028110156200061257620005ff836200043b565b85529385019391850191600101620005ea565b50829850620006228d8262000453565b97505050505050620006388860e0890162000453565b91506200064a886101408901620004db565b90509295509295509295565b6000602082840312156200066957600080fd5b5051919050565b604080519081016001600160401b0381118282101715620006a157634e487b7160e01b600052604160045260246000fd5b60405290565b60008219821115620006bd57620006bd62000707565b500190565b600082620006e057634e487b7160e01b600052601260045260246000fd5b500490565b600081600019048311821515161562000702576200070262000707565b500290565b634e487b7160e01b600052601160045260246000fd5b6137e4806200072d6000396000f3fe608060405234801561001057600080fd5b50600436106103785760003560e01c80637c3a00fd116101d3578063ba83276b11610104578063d41ddc96116100a2578063dac885611161007c578063dac88561146106ad578063e268255d146106c5578063e44b3875146106d8578063e920b1e1146106e057600080fd5b8063d41ddc961461067f578063d784d42614610692578063d8dfeb45146106a557600080fd5b8063c45a0155116100de578063c45a01551461039e578063cc32d17614610651578063ccc0448414610659578063d05951a01461066c57600080fd5b8063ba83276b14610625578063bcead63e1461062d578063c3fbb6fd1461063e57600080fd5b8063aabaecd611610171578063b86a513e1161014b578063b86a513e14610605578063b96b5c991461060d578063b9b1f4e314610615578063ba5d30781461061d57600080fd5b8063aabaecd6146105d9578063acb522b4146105ea578063b69410de146105fd57600080fd5b80638ffc9215116101ad5780638ffc9215146105ae5780639e10320b146105b6578063a06db7dc146105c9578063a97d1161146105d157600080fd5b80637c3a00fd146105825780637df1f1b91461058a57806387accaf11461059b57600080fd5b806347350e9f116102ad57806362e8a9761161024b578063712b772f11610225578063712b772f1461055757806375a206761461056a57806377b3c55c146105725780637a0e6fa11461057a57600080fd5b806362e8a9761461050357806369458ba71461053e578063700f50061461054657600080fd5b806350f2012f1161028757806350f2012f146104a45780635114cb52146104b75780635c60da1b146104ea5780635eeb53b4146104f257600080fd5b806347350e9f146104565780634eac42351461047e57806350acb4ee1461049157600080fd5b80632ead10981161031a57806339ba9f86116102f457806339ba9f86146104175780633b99bcee146104285780634003f34d1461043b57806345755dd61461044357600080fd5b80632ead1098146103f457806330fea1ce146103fc578063390d68551461040457600080fd5b80630fe3d9b7116103565780630fe3d9b7146103be5780631cc1cf46146103c65780631f3f19ab146103ce578063267f4ac3146103e157600080fd5b806301daa38f1461037d5780630895326f146103875780630d49b38c1461039e575b600080fd5b6103856106f3565b005b6013545b6040519081526020015b60405180910390f35b6103a6610799565b6040516001600160a01b039091168152602001610395565b6103856107a8565b60075461038b565b6103856103dc366004613283565b61084b565b6103856103ef366004613283565b6108f0565b600b5461038b565b60095461038b565b61038561041236600461348b565b61098c565b6005546001600160a01b03166103a6565b6103856104363660046134b0565b610a4f565b60125461038b565b61038b610451366004613459565b610b46565b610469610464366004613283565b610c1b565b60408051928352602083019190915201610395565b61038b61048c366004613459565b610ce7565b61038561049f3660046133d3565b610d2f565b61038b6104b2366004613459565b610f4f565b6104ca6104c5366004613459565b61100b565b604080519485526020850193909352918301526060820152608001610395565b6103a6611150565b6003546001600160a01b03166103a6565b610385610511366004613283565b6001600160a01b03167f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af155565b6104ca61115d565b6002546001600160a01b03166103a6565b61038b6105653660046132bd565b611179565b600c5461038b565b600a5461038b565b61038b611370565b60085461038b565b6000546001600160a01b03166103a6565b6103856105a9366004613377565b6113ac565b600d5461038b565b61038b6105c4366004613459565b6114c1565b60065461038b565b60165461038b565b6004546001600160a01b03166103a6565b6103856105f8366004613377565b6114eb565b60175461038b565b600e5461038b565b6104ca61159c565b600f5461038b565b60145461038b565b60155461038b565b6001546001600160a01b03166103a6565b61038561064c3660046132f6565b6115aa565b60185461038b565b61038b61066736600461348b565b61164f565b6104ca61067a366004613459565b61175e565b61038561068d36600461348b565b611897565b6103856106a0366004613283565b611958565b60115461038b565b6106b56119e1565b6040519015158152602001610395565b61038b6106d33660046134e3565b611acb565b60105461038b565b61038b6106ee36600461334b565b611ae2565b6002546001600160a01b031633146107525760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a41423a4e4f545f50454e44494e475f424f52524f57455200000000000060448201526064015b60405180910390fd5b600280546001600160a01b031990811690915560008054339216821781556040517f29bac0ac2b15405bfcc160bb74b6ae7a559b7674ce33db80785ada73e38204d29190a2565b60006107a3611c8a565b905090565b6003546001600160a01b031633146108025760405162461bcd60e51b815260206004820152601860248201527f4d4c3a414c3a4e4f545f50454e44494e475f4c454e44455200000000000000006044820152606401610749565b600380546001600160a01b031990811690915560018054339216821790556040517fd6165838d2e3db87aa1002b548048673fc6427eefbd1b914e100f3a0deae23e390600090a2565b6000546001600160a01b0316331461089b5760405162461bcd60e51b815260206004820152601360248201527226a61d29a8211d2727aa2fa127a92927aba2a960691b6044820152606401610749565b600280546001600160a01b0319166001600160a01b0383169081179091556040519081527f10f06072822ef73860fedb88933f968d20bb4aadce8a8d360d1124cb6ce1e0b2906020015b60405180910390a150565b6001546001600160a01b0316331461093e5760405162461bcd60e51b815260206004820152601160248201527026a61d29a8261d2727aa2fa622a72222a960791b6044820152606401610749565b600380546001600160a01b0319166001600160a01b0383169081179091556040519081527fa3ab02442c80a4102475683f16513c9139a89142be9db9804edfcfbb379fc492906020016108e5565b6109946119e1565b156109b15760405162461bcd60e51b815260040161074990613670565b6001546001600160a01b031633146109fe5760405162461bcd60e51b815260206004820152601060248201526f26a61d21a31d2727aa2fa622a72222a960811b6044820152606401610749565b806001600160a01b03167f6bd56533ce1c8ea03f7b858ac441b5a86d140a793a7c9e3faecbbe517c2c879183604051610a3991815260200190565b60405180910390a2610a4b8282611cb9565b5050565b6000546001600160a01b03163314610a9d5760405162461bcd60e51b815260206004820152601160248201527026a61d2a9d2727aa2fa127a92927aba2a960791b6044820152606401610749565b7faaaa7ee6b0c2f4ee1fa7312c7d5b3623a434da5a1a9ce3cb6e629caa23454ab6838383604051610ad09392919061369c565b60405180910390a1610ae0611c8a565b6001600160a01b031663fe69f7088484846040518463ffffffff1660e01b8152600401610b0f9392919061369c565b600060405180830381600087803b158015610b2957600080fd5b505af1158015610b3d573d6000803e3d6000fd5b50505050505050565b6000610b506119e1565b15610b6d5760405162461bcd60e51b815260040161074990613670565b811580610b8d5750600554610b8d906001600160a01b0316333085611d2c565b610bd95760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a52463a5452414e534645525f46524f4d5f4641494c45440000000000006044820152606401610749565b7f278e51d3323fbf18b9fb8df3f8b97e31b145bc1146c52e764cf4aa1bfc4ba17d610c02611d9a565b60405181815290925060200160405180910390a1919050565b600080610c266119e1565b15610c435760405162461bcd60e51b815260040161074990613670565b6001546001600160a01b03163314610c8f5760405162461bcd60e51b815260206004820152600f60248201526e26a61d291d2727aa2fa622a72222a960891b6044820152606401610749565b610c9883611dd0565b60408051838152602081018390529294509092506001600160a01b038516917f027e623aab0b174da270ff529cad1c54f09182651e07437d2ac557929b9e5b49910160405180910390a2915091565b600080610d0960145484600f54610cfe9190613756565b600d54600c54611f45565b601154909150808211610d1d576000610d27565b610d278183613756565b949350505050565b610d376119e1565b15610d545760405162461bcd60e51b815260040161074990613670565b6001546001600160a01b0316338114610da35760405162461bcd60e51b815260206004820152601160248201527026a61d20a72a1d2727aa2fa622a72222a960791b6044820152606401610749565b6005546001600160a01b0316821580610dc35750610dc381333086611d2c565b610e0f5760405162461bcd60e51b815260206004820152601b60248201527f4d4c3a414e543a5452414e534645525f46524f4d5f4641494c454400000000006044820152606401610749565b7f7150c332bd889236b6ab42cc34f0853631ceb58827f58a8697b682f13e390a8c610e3c88888888611f7e565b88888888604051610e51959493929190613642565b60405180910390a17ffe9a32948c4b8ec5c8a8eddeacd3f3621458e8bde95b725b625e5c8f4f2cb54d601754601854604051610e97929190918252602082015260400190565b60405180910390a16000610eaa8261227d565b90508015610f4557826001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f382604051610eed91815260200190565b60405180910390a2610f00828483612357565b610f455760405162461bcd60e51b815260206004820152601660248201527513530e9053950e9514905394d1915497d1905253115160521b6044820152606401610749565b5050505050505050565b6000610f596119e1565b15610f765760405162461bcd60e51b815260040161074990613670565b811580610f965750600454610f96906001600160a01b0316333085611d2c565b610fe25760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a50433a5452414e534645525f46524f4d5f4641494c45440000000000006044820152606401610749565b7f437d44b2c697fb69e2b2f25f57fd844e376c25ed28ed5a9c4be88aa1e5c87d12610c02612394565b6000806000806000600f549050600086148061103a575060055461103a906001600160a01b0316333089611d2c565b6110865760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a4d503a5452414e534645525f46524f4d5f4641494c45440000000000006044820152606401610749565b61108e6123c0565b600054939850919650945092506001600160a01b03163314806110b3575080600f5410155b6110ff5760405162461bcd60e51b815260206004820152601960248201527f4d4c3a4d503a43414e4e4f545f5553455f4452415741424c45000000000000006044820152606401610749565b6040805186815260208101869052908101849052606081018390527f95c4acf903eb698cf367efaaf79a8a58fb4554fcd8503b62af9f9c1b68b59e1e906080015b60405180910390a1509193509193565b60006107a36124b7565b50565b60008060008061116b6124e1565b929791965094509092509050565b60006111836119e1565b156111a05760405162461bcd60e51b815260040161074990613670565b6000546001600160a01b03163314806111c357506001546001600160a01b031633145b6111fd5760405162461bcd60e51b815260206004820152600b60248201526a09874a6749c9ebe82aaa8960ab1b6044820152606401610749565b6005546001600160a01b0384811691161480159061122957506004546001600160a01b03848116911614155b6112695760405162461bcd60e51b8152602060048201526011602482015270261d299d24a72b20a624a22faa27a5a2a760791b6044820152606401610749565b6040516370a0823160e01b81523060048201526001600160a01b0380841691908516907ff1f6a55e7ad487ac8dd8e1d4517348d3b410a7a0bc405ef87b09078dc51b23b69082906370a082319060240160206040518083038186803b1580156112d157600080fd5b505afa1580156112e5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113099190613472565b60405181815290945060200160405180910390a3611328838383612357565b61136a5760405162461bcd60e51b8152602060048201526013602482015272130e94ce9514905394d1915497d19052531151606a1b6044820152606401610749565b92915050565b600080611387601454600f54600d54600c54611f45565b60115490915081811161139b5760006113a5565b6113a58282613756565b9250505090565b6113b46119e1565b156113d15760405162461bcd60e51b815260040161074990613670565b6000546001600160a01b031633146114215760405162461bcd60e51b815260206004820152601360248201527226a61d28272a1d2727aa2fa127a92927aba2a960691b6044820152606401610749565b428310156114715760405162461bcd60e51b815260206004820152601760248201527f4d4c3a504e543a494e56414c49445f444541444c494e450000000000000000006044820152606401610749565b7ff94d2f0322894aaf1bce14561461a8b8b6c9b11a77bbe80f20b804da8a95e4b761149e85858585612548565b858585856040516114b3959493929190613642565b60405180910390a150505050565b60006114e482600754601454600e54600854601354601254600a54600b54612570565b5092915050565b6114f36119e1565b156115105760405162461bcd60e51b815260040161074990613670565b6000546001600160a01b031633148061153357506001546001600160a01b031633145b61156f5760405162461bcd60e51b815260206004820152600d60248201526c09874a49ca8749c9ebe82aaa89609b1b6044820152606401610749565b7f47244a449377da5fd10e98d86d118dee442e842fc34f05179c973cfcff6acba761149e858585856125df565b60008060008061116b61264d565b6115b2611c8a565b6001600160a01b0316336001600160a01b0316146116055760405162461bcd60e51b815260206004820152601060248201526f4d4c3a4d3a4e4f545f464143544f525960801b6044820152606401610749565b611610838383612698565b61164a5760405162461bcd60e51b815260206004820152600b60248201526a13530e934e91905253115160aa1b6044820152606401610749565b505050565b60006116596119e1565b156116765760405162461bcd60e51b815260040161074990613670565b6000546001600160a01b031633146116c55760405162461bcd60e51b815260206004820152601260248201527126a61d22231d2727aa2fa127a92927aba2a960711b6044820152606401610749565b816001600160a01b03167f7578fe8c4d9f6fc38fdad20d219b0ce47d38bbf8a72bdb26867809f24119363d8460405161170091815260200190565b60405180910390a2600061171384610ce7565b9050801561175457600454600090611733906001600160a01b031661227d565b9050611750818311611746576000610f4f565b6104b28284613756565b9250505b6114e48484612711565b6000806000806000600f549050600086148061178d575060055461178d906001600160a01b0316333089611d2c565b6117d95760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a434c3a5452414e534645525f46524f4d5f4641494c45440000000000006044820152606401610749565b6117e16127d8565b600054939850919650945092506001600160a01b0316331480611806575080600f5410155b6118525760405162461bcd60e51b815260206004820152601960248201527f4d4c3a434c3a43414e4e4f545f5553455f4452415741424c45000000000000006044820152606401610749565b6040805186815260208101869052908101849052606081018390527f6d5b31efac20a15ed5b9e27e38cf9ebcc3ffb6d64feb827a35ef84a607e8dfaf90608001611140565b61189f6119e1565b156118bc5760405162461bcd60e51b815260040161074990613670565b6000546001600160a01b0316331461190b5760405162461bcd60e51b815260206004820152601260248201527126a61d29219d2727aa2fa127a92927aba2a960711b6044820152606401610749565b806001600160a01b03167f97b446ee2df422b7273fe6d658674835f9de3319d131c229f9a2f8ed62a761938360405161194691815260200190565b60405180910390a2610a4b82826128c6565b611960611c8a565b6001600160a01b0316336001600160a01b0316146119b45760405162461bcd60e51b81526020600482015260116024820152704d4c3a53493a4e4f545f464143544f525960781b6044820152606401610749565b6001600160a01b03167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b60006119eb611c8a565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b158015611a2357600080fd5b505afa158015611a37573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a5b91906132a0565b6001600160a01b031663425fad586040518163ffffffff1660e01b815260040160206040518083038186803b158015611a9357600080fd5b505afa158015611aa7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107a39190613437565b6000611ad985858585611f45565b95945050505050565b6000611aec6119e1565b15611b095760405162461bcd60e51b815260040161074990613670565b6005546001600160a01b0316821580611b295750611b2981333086611d2c565b611b755760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a464c3a5452414e534645525f46524f4d5f4641494c45440000000000006044820152606401610749565b601254611bcf57836001600160a01b03167fcd909ec339185c4598a4096e174308fbdf136d117f230960f873a2f2e81f63af611bb08661298d565b6012546040805183815260208101929092529195500160405180910390a25b6000611bda8261227d565b6001549091506001600160a01b03168115611c8157806001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f383604051611c2a91815260200190565b60405180910390a2611c3d838284612357565b611c815760405162461bcd60e51b815260206004820152601560248201527413530e91930e9514905394d1915497d19052531151605a1b6044820152606401610749565b50505092915050565b6000611cb47f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15490565b919050565b8160106000828254611ccb9190613756565b9091555050600554611ce7906001600160a01b03168284612357565b610a4b5760405162461bcd60e51b81526020600482015260166024820152751353124e90d18e9514905394d1915497d1905253115160521b6044820152606401610749565b6040516001600160a01b0380851660248301528316604482015260648101829052600090611ad99086906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612ad7565b600554600090611db2906001600160a01b031661227d565b905080600f6000828254611dc691906136fd565b9250508190555090565b60125460009081908015801590611df25750600654611def90826136fd565b42115b611e355760405162461bcd60e51b81526020600482015260146024820152731353124e948e9393d517d25397d111519055531560621b6044820152606401610749565b611e3d612b77565b600060118190556010819055600f8190556004546001600160a01b031690611e648261227d565b9450841480611e795750611e79818686612357565b611ec55760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a435f5452414e534645525f4641494c45440000000000000000006044820152606401610749565b6005546001600160a01b03166000611edc8261227d565b9450841480611ef15750611ef1818786612357565b611f3d5760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a465f5452414e534645525f4641494c45440000000000000000006044820152606401610749565b505050915091565b600083851115611f735782611f5a8587613756565b611f649084613737565b611f6e9190613715565b611ad9565b506000949350505050565b6000611f8c85858585612bb5565b90508060155414611fdf5760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a414e543a434f4d4d49544d454e545f4d49534d4154434800000000006044820152606401610749565b6001600160a01b0385163b6120365760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a494e56414c49445f524546494e414e4345520000000000006044820152606401610749565b834211156120865760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a455850495245445f434f4d4d49544d454e540000000000006044820152606401610749565b600060075490506000806120af4284601454600e54600854601354601254600a54600b54612570565b9150915081601660008282546120c591906136fd565b90915550506000601581905585905b818110156121a55760008a6001600160a01b03168989848181106120fa576120fa613783565b905060200281019061210c91906136b6565b60405161211a9291906135cf565b600060405180830381855af49150503d8060008114612155576040519150601f19603f3d011682016040523d82523d6000602084013e61215a565b606091505b505090508061219c5760405162461bcd60e51b815260206004820152600e60248201526d1353124e9053950e91905253115160921b6044820152606401610749565b506001016120d4565b506000601354856121b69190613737565b836017546121c49190613737565b6121ce9190613715565b90506000601354866121e09190613737565b846018546121ee9190613737565b6121f89190613715565b60075490915061220881426136fd565b60125560145461221a90828585612bee565b612222612d4c565b61226e5760405162461bcd60e51b815260206004820152601f60248201527f4d4c493a414e543a494e53554646494349454e545f434f4c4c41544552414c006044820152606401610749565b50505050505050949350505050565b6005546000906001600160a01b0383811691161461229c5760006122ac565b600f546010546122ac91906136fd565b6004546001600160a01b038481169116146122c85760006122cc565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561230b57600080fd5b505afa15801561231f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123439190613472565b61234d9190613756565b61136a9190613756565b6040516001600160a01b03831660248201526044810182905260009061238a90859063a9059cbb60e01b90606401611d63565b90505b9392505050565b6004546000906123ac906001600160a01b031661227d565b90508060116000828254611dc691906136fd565b6000806000806123ce61264d565b60006016819055939750919550935091506123e984866136fd565b9050816123f684836136fd565b61240091906136fd565b600554612415906001600160a01b031661227d565b600f5461242291906136fd565b61242c9190613756565b600f81905550806010600082825461244491906136fd565b9091555061245490508383612d6c565b601354600181141561246d57612468612b77565b6124af565b6007546012600082825461248191906136fd565b92505081905550856014600082825461249a9190613756565b909155506124ab9050600182613756565b6013555b505090919293565b6000611cb47f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b600080600080601654670de0b6b3a76400006009546014549650866125069190613737565b6125109190613715565b61251a91906136fd565b6013546017549194509061252f908290613737565b92508060185461253f9190613737565b91505090919293565b600081612556576000612562565b61256285858585612bb5565b601581905595945050505050565b6000808461257e8b8d6136fd565b101561258f575060009050806125d1565b6125998a86613756565b6125a3908c613756565b90506125b2898989848a612dde565b92506125c490508b8a89888888612ed1565b6125ce90836136fd565b91505b995099975050505050505050565b60006125ed85858585612bb5565b905080601554146126405760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a524e543a434f4d4d49544d454e545f4d49534d4154434800000000006044820152606401610749565b6000601555949350505050565b60008060008061267442601254600754601454600e54601354600854600a54600b54612f77565b601654919550935061268690846136fd565b92506017549150601854905090919293565b6000833b806126ab57600091505061238d565b846001600160a01b031684846040516126c59291906135cf565b600060405180830381855af49150503d8060008114612700576040519150601f19603f3d011682016040523d82523d6000602084013e612705565b606091505b50909695505050505050565b81600f60008282546127239190613756565b909155505060055461273f906001600160a01b03168284612357565b6127845760405162461bcd60e51b81526020600482015260166024820152751353124e91118e9514905394d1915497d1905253115160521b6044820152606401610749565b61278c612d4c565b610a4b5760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a44463a494e53554646494349454e545f434f4c4c41544552414c00006044820152606401610749565b6000806000806012544211156128295760405162461bcd60e51b81526020600482015260166024820152754d4c493a434c3a5041594d454e545f49535f4c41544560501b6044820152606401610749565b6128316124e1565b600060168190559397509195509350915061284c84866136fd565b90508161285984836136fd565b61286391906136fd565b600554612878906001600160a01b031661227d565b600f5461288591906136fd565b61288f9190613756565b600f8190555080601060008282546128a791906136fd565b909155506128b790508383612d6c565b6128bf612b77565b5090919293565b81601160008282546128d89190613756565b90915550506004546128f4906001600160a01b03168284612357565b6129395760405162461bcd60e51b81526020600482015260166024820152751353124e9490ce9514905394d1915497d1905253115160521b6044820152606401610749565b612941612d4c565b610a4b5760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a52433a494e53554646494349454e545f434f4c4c41544552414c00006044820152606401610749565b60006001600160a01b0382166129dd5760405162461bcd60e51b815260206004820152601560248201527426a6249d23261d24a72b20a624a22fa622a72222a960591b6044820152606401610749565b6013546012541580156129ef57508015155b612a305760405162461bcd60e51b81526020600482015260126024820152714d4c493a464c3a4c4f414e5f41435449564560701b6044820152606401610749565b600754600180546001600160a01b0319166001600160a01b038616179055612a5881426136fd565b601255600d5460148190556005549093506001600160a01b031683612a7c8261227d565b1015612aca5760405162461bcd60e51b815260206004820152601860248201527f4d4c493a464c3a57524f4e475f46554e445f414d4f554e5400000000000000006044820152606401610749565b505050600f819055919050565b60006001600160a01b0383163b612af05750600061136a565b6060836001600160a01b031683604051612b0a91906135df565b6000604051808303816000865af19150503d8060008114612b47576040519150601f19603f3d011682016040523d82523d6000602084013e612b4c565b606091505b509092509050818015610d27575080511580610d27575080806020019051810190610d279190613437565b60006006819055600781905560088190556009819055600a819055600b819055600e8190556012819055601381905560148190556017819055601855565b600084848484604051602001612bce949392919061361a565b604051602081830303815290604052805190602001209050949350505050565b6000612bf8612fc6565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612c3b57600080fd5b505afa158015612c4f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c739190613472565b612c7d9089613737565b612c879190613737565b612c919190613715565b612c9b91906136fd565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b158015612ce257600080fd5b505afa158015612cf6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d1a9190613472565b612d249089613737565b612d2e9190613737565b612d389190613715565b612d4291906136fd565b6018555050505050565b6000612d62601454600f54600d54600c54611f45565b6011541015905090565b600154612d8a906001600160a01b0316634046af2b60e01b84613040565b612da6578160106000828254612da091906136fd565b90915550505b612dbf612db1612fc6565b63a5a2760560e01b83613040565b610a4b578060106000828254612dd591906136fd565b90915550505050565b6000806000612ded868661314f565b90506000612e15612e0683670de0b6b3a76400006136fd565b86670de0b6b3a764000061316a565b9050670de0b6b3a76400008111612e485784612e31898b613756565b612e3b9190613715565b6000935093505050612ec7565b6000612e5c670de0b6b3a764000083613756565b838a670de0b6b3a7640000612e71868f613737565b612e7b9190613715565b612e859190613756565b612e8f9190613737565b612e999190613715565b9050612ea68a89896131cc565b935083811015612eb7576000612ec1565b612ec18482613756565b94505050505b9550959350505050565b6000838711612ee257506000612f6d565b6000620151806001612ef4878b613756565b612efe9190613756565b612f089190613715565b612f139060016136fd565b612f209062015180613737565b9050612f3687612f3085896136fd565b836131cc565b612f4090836136fd565b9150670de0b6b3a7640000612f558886613737565b612f5f9190613715565b612f6990836136fd565b9150505b9695505050505050565b600080612f878888878c8a612dde565b909250905060018614612f9a5781612f9c565b875b9150612fac8b89878d8888612ed1565b612fb690826136fd565b9050995099975050505050505050565b6000612fd0611c8a565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b15801561300857600080fd5b505afa15801561301c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107a391906132a0565b60008161304f5750600161238d565b60408051600481526024810182526020810180516001600160e01b03166001600160e01b03198716179052905160009182916001600160a01b03881691613095916135df565b6000604051808303816000865af19150503d80600081146130d2576040519150601f19603f3d011682016040523d82523d6000602084013e6130d7565b606091505b50915091508115806130eb57506020815114155b156130fb5760009250505061238d565b60008180602001905181019061311191906132a0565b90506001600160a01b03811661312d576000935050505061238d565b600554613144906001600160a01b03168287612357565b979650505050505050565b60006301e133806131608385613737565b61238d9190613715565b60006001831661317a578161317c565b835b90505b60019290921c91821561238d57816131978580613737565b6131a19190613715565b9350600183166131b05761317f565b816131bb8583613737565b6131c59190613715565b905061317f565b6000670de0b6b3a76400006131e1848461314f565b6131eb9086613737565b61238a9190613715565b60008083601f84011261320757600080fd5b50813567ffffffffffffffff81111561321f57600080fd5b6020830191508360208260051b850101111561323a57600080fd5b9250929050565b60008083601f84011261325357600080fd5b50813567ffffffffffffffff81111561326b57600080fd5b60208301915083602082850101111561323a57600080fd5b60006020828403121561329557600080fd5b813561238d81613799565b6000602082840312156132b257600080fd5b815161238d81613799565b600080604083850312156132d057600080fd5b82356132db81613799565b915060208301356132eb81613799565b809150509250929050565b60008060006040848603121561330b57600080fd5b833561331681613799565b9250602084013567ffffffffffffffff81111561333257600080fd5b61333e86828701613241565b9497909650939450505050565b6000806040838503121561335e57600080fd5b823561336981613799565b946020939093013593505050565b6000806000806060858703121561338d57600080fd5b843561339881613799565b935060208501359250604085013567ffffffffffffffff8111156133bb57600080fd5b6133c7878288016131f5565b95989497509550505050565b6000806000806000608086880312156133eb57600080fd5b85356133f681613799565b945060208601359350604086013567ffffffffffffffff81111561341957600080fd5b613425888289016131f5565b96999598509660600135949350505050565b60006020828403121561344957600080fd5b8151801515811461238d57600080fd5b60006020828403121561346b57600080fd5b5035919050565b60006020828403121561348457600080fd5b5051919050565b6000806040838503121561349e57600080fd5b8235915060208301356132eb81613799565b6000806000604084860312156134c557600080fd5b83359250602084013567ffffffffffffffff81111561333257600080fd5b600080600080608085870312156134f957600080fd5b5050823594602084013594506040840135936060013592509050565b81835260006020808501808196508560051b810191508460005b878110156135995782840389528135601e1988360301811261355057600080fd5b8701803567ffffffffffffffff81111561356957600080fd5b80360389131561357857600080fd5b61358586828985016135a6565b9a87019a955050509084019060010161352f565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8183823760009101908152919050565b6000825160005b8181101561360057602081860181015185830152016135e6565b8181111561360f576000828501525b509190910192915050565b60018060a01b0385168152836020820152606060408201526000612f6d606083018486613515565b85815260018060a01b0385166020820152836040820152608060608201526000613144608083018486613515565b60208082526012908201527113530e941493d513d0d3d317d4105554d15160721b604082015260600190565b838152604060208201526000611ad96040830184866135a6565b6000808335601e198436030181126136cd57600080fd5b83018035915067ffffffffffffffff8211156136e857600080fd5b60200191503681900382131561323a57600080fd5b600082198211156137105761371061376d565b500190565b60008261373257634e487b7160e01b600052601260045260246000fd5b500490565b60008160001904831182151516156137515761375161376d565b500290565b6000828210156137685761376861376d565b500390565b634e487b7160e01b600052601160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6001600160a01b038116811461115a57600080fdfea2646970667358221220faa510bec224c5abb8cb6a7ce679fde5fc0ddbc7e41c537f2f038e1950bbf3f464736f6c634300080700337a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af1" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ConstructableMapleLoan<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ConstructableMapleLoan<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConstructableMapleLoan<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConstructableMapleLoan))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ConstructableMapleLoan<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CONSTRUCTABLEMAPLELOAN_ABI.clone(),
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
                CONSTRUCTABLEMAPLELOAN_ABI.clone(),
                CONSTRUCTABLEMAPLELOAN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `__setFactory` (0x62e8a976) function"]
        pub fn set_factory(
            &self,
            factory: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 232, 169, 118], factory)
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `getCollateralRequiredFor` (0xe268255d) function"]
        pub fn get_collateral_required_for(
            &self,
            principal: ethers::core::types::U256,
            drawable_funds: ethers::core::types::U256,
            principal_requested: ethers::core::types::U256,
            collateral_required: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [226, 104, 37, 93],
                    (
                        principal,
                        drawable_funds,
                        principal_requested,
                        collateral_required,
                    ),
                )
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ConstructableMapleLoanEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ConstructableMapleLoan<M>
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
    pub enum ConstructableMapleLoanEvents {
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
    impl ethers::contract::EthLogDecode for ConstructableMapleLoanEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::BorrowerAcceptedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::CollateralPostedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::CollateralRemovedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::EstablishmentFeesSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::NewTermsAcceptedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::NewTermsProposedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::NewTermsRejectedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::PendingBorrowerSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::PendingLenderSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::SkimmedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ConstructableMapleLoanEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ConstructableMapleLoanEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConstructableMapleLoanEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::CollateralPostedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::CollateralRemovedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::FundedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::FundsClaimedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::FundsRedirectedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::FundsReturnedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::InitializedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::LenderAcceptedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::LoanClosedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::NewTermsProposedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::PaymentMadeFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::PendingLenderSetFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::RepossessedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::SkimmedFilter(element) => element.fmt(f),
                ConstructableMapleLoanEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `__setFactory`function with signature `__setFactory(address)` and selector `[98, 232, 169, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setFactory", abi = "__setFactory(address)")]
    pub struct SetFactoryCall {
        pub factory: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `getCollateralRequiredFor`function with signature `getCollateralRequiredFor(uint256,uint256,uint256,uint256)` and selector `[226, 104, 37, 93]`"]
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
        name = "getCollateralRequiredFor",
        abi = "getCollateralRequiredFor(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetCollateralRequiredForCall {
        pub principal: ethers::core::types::U256,
        pub drawable_funds: ethers::core::types::U256,
        pub principal_requested: ethers::core::types::U256,
        pub collateral_required: ethers::core::types::U256,
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
    pub enum ConstructableMapleLoanCalls {
        SetFactory(SetFactoryCall),
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
        GetCollateralRequiredFor(GetCollateralRequiredForCall),
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
    impl ethers::core::abi::AbiDecode for ConstructableMapleLoanCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SetFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::SetFactory(decoded));
            }
            if let Ok(decoded) =
                <AcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::AcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <AcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::AcceptLender(decoded));
            }
            if let Ok(decoded) =
                <AcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::AcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <BorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Borrower(decoded));
            }
            if let Ok(decoded) =
                <ClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::ClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::ClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <CloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::CloseLoan(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Collateral(decoded));
            }
            if let Ok(decoded) =
                <CollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::CollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <CollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::CollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <DelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::DelegateFee(decoded));
            }
            if let Ok(decoded) =
                <DrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::DrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <DrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::DrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <EarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::EarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <EndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::EndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <ExcessCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::ExcessCollateral(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::FundLoan(decoded));
            }
            if let Ok(decoded) =
                <FundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::FundsAsset(decoded));
            }
            if let Ok(decoded) =
                <GetAdditionalCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ConstructableMapleLoanCalls::GetAdditionalCollateralRequiredFor(decoded),
                );
            }
            if let Ok(decoded) =
                <GetCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConstructableMapleLoanCalls::GetCollateralRequiredFor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetEarlyPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConstructableMapleLoanCalls::GetEarlyPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNextPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::GetNextPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::GetRefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::InterestRate(decoded));
            }
            if let Ok(decoded) =
                <IsProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::IsProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <LateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::LateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <LateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::LateInterestPremium(decoded));
            }
            if let Ok(decoded) = <LenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Lender(decoded));
            }
            if let Ok(decoded) =
                <MakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::MakePayment(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <NextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::NextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <PaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <PaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <PendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <PendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PendingLender(decoded));
            }
            if let Ok(decoded) =
                <PostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PostCollateral(decoded));
            }
            if let Ok(decoded) =
                <PrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Principal(decoded));
            }
            if let Ok(decoded) =
                <PrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::PrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::ProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::RefinanceCommitment(decoded));
            }
            if let Ok(decoded) =
                <RefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::RefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <RejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::RejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <RepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Repossess(decoded));
            }
            if let Ok(decoded) =
                <ReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::ReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::SetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::SetPendingLender(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ConstructableMapleLoanCalls::Skim(decoded));
            }
            if let Ok(decoded) =
                <SuperFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::SuperFactory(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::TreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConstructableMapleLoanCalls::Upgrade(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConstructableMapleLoanCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConstructableMapleLoanCalls::SetFactory(element) => element.encode(),
                ConstructableMapleLoanCalls::AcceptBorrower(element) => element.encode(),
                ConstructableMapleLoanCalls::AcceptLender(element) => element.encode(),
                ConstructableMapleLoanCalls::AcceptNewTerms(element) => element.encode(),
                ConstructableMapleLoanCalls::Borrower(element) => element.encode(),
                ConstructableMapleLoanCalls::ClaimFunds(element) => element.encode(),
                ConstructableMapleLoanCalls::ClaimableFunds(element) => element.encode(),
                ConstructableMapleLoanCalls::CloseLoan(element) => element.encode(),
                ConstructableMapleLoanCalls::Collateral(element) => element.encode(),
                ConstructableMapleLoanCalls::CollateralAsset(element) => element.encode(),
                ConstructableMapleLoanCalls::CollateralRequired(element) => element.encode(),
                ConstructableMapleLoanCalls::DelegateFee(element) => element.encode(),
                ConstructableMapleLoanCalls::DrawableFunds(element) => element.encode(),
                ConstructableMapleLoanCalls::DrawdownFunds(element) => element.encode(),
                ConstructableMapleLoanCalls::EarlyFeeRate(element) => element.encode(),
                ConstructableMapleLoanCalls::EndingPrincipal(element) => element.encode(),
                ConstructableMapleLoanCalls::ExcessCollateral(element) => element.encode(),
                ConstructableMapleLoanCalls::Factory(element) => element.encode(),
                ConstructableMapleLoanCalls::FundLoan(element) => element.encode(),
                ConstructableMapleLoanCalls::FundsAsset(element) => element.encode(),
                ConstructableMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => {
                    element.encode()
                }
                ConstructableMapleLoanCalls::GetCollateralRequiredFor(element) => element.encode(),
                ConstructableMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.encode(),
                ConstructableMapleLoanCalls::GetNextPaymentBreakdown(element) => element.encode(),
                ConstructableMapleLoanCalls::GetRefinanceInterest(element) => element.encode(),
                ConstructableMapleLoanCalls::GracePeriod(element) => element.encode(),
                ConstructableMapleLoanCalls::Implementation(element) => element.encode(),
                ConstructableMapleLoanCalls::InterestRate(element) => element.encode(),
                ConstructableMapleLoanCalls::IsProtocolPaused(element) => element.encode(),
                ConstructableMapleLoanCalls::LateFeeRate(element) => element.encode(),
                ConstructableMapleLoanCalls::LateInterestPremium(element) => element.encode(),
                ConstructableMapleLoanCalls::Lender(element) => element.encode(),
                ConstructableMapleLoanCalls::MakePayment(element) => element.encode(),
                ConstructableMapleLoanCalls::Migrate(element) => element.encode(),
                ConstructableMapleLoanCalls::NextPaymentDueDate(element) => element.encode(),
                ConstructableMapleLoanCalls::PaymentInterval(element) => element.encode(),
                ConstructableMapleLoanCalls::PaymentsRemaining(element) => element.encode(),
                ConstructableMapleLoanCalls::PendingBorrower(element) => element.encode(),
                ConstructableMapleLoanCalls::PendingLender(element) => element.encode(),
                ConstructableMapleLoanCalls::PostCollateral(element) => element.encode(),
                ConstructableMapleLoanCalls::Principal(element) => element.encode(),
                ConstructableMapleLoanCalls::PrincipalRequested(element) => element.encode(),
                ConstructableMapleLoanCalls::ProposeNewTerms(element) => element.encode(),
                ConstructableMapleLoanCalls::RefinanceCommitment(element) => element.encode(),
                ConstructableMapleLoanCalls::RefinanceInterest(element) => element.encode(),
                ConstructableMapleLoanCalls::RejectNewTerms(element) => element.encode(),
                ConstructableMapleLoanCalls::RemoveCollateral(element) => element.encode(),
                ConstructableMapleLoanCalls::Repossess(element) => element.encode(),
                ConstructableMapleLoanCalls::ReturnFunds(element) => element.encode(),
                ConstructableMapleLoanCalls::SetImplementation(element) => element.encode(),
                ConstructableMapleLoanCalls::SetPendingBorrower(element) => element.encode(),
                ConstructableMapleLoanCalls::SetPendingLender(element) => element.encode(),
                ConstructableMapleLoanCalls::Skim(element) => element.encode(),
                ConstructableMapleLoanCalls::SuperFactory(element) => element.encode(),
                ConstructableMapleLoanCalls::TreasuryFee(element) => element.encode(),
                ConstructableMapleLoanCalls::Upgrade(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConstructableMapleLoanCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConstructableMapleLoanCalls::SetFactory(element) => element.fmt(f),
                ConstructableMapleLoanCalls::AcceptBorrower(element) => element.fmt(f),
                ConstructableMapleLoanCalls::AcceptLender(element) => element.fmt(f),
                ConstructableMapleLoanCalls::AcceptNewTerms(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Borrower(element) => element.fmt(f),
                ConstructableMapleLoanCalls::ClaimFunds(element) => element.fmt(f),
                ConstructableMapleLoanCalls::ClaimableFunds(element) => element.fmt(f),
                ConstructableMapleLoanCalls::CloseLoan(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Collateral(element) => element.fmt(f),
                ConstructableMapleLoanCalls::CollateralAsset(element) => element.fmt(f),
                ConstructableMapleLoanCalls::CollateralRequired(element) => element.fmt(f),
                ConstructableMapleLoanCalls::DelegateFee(element) => element.fmt(f),
                ConstructableMapleLoanCalls::DrawableFunds(element) => element.fmt(f),
                ConstructableMapleLoanCalls::DrawdownFunds(element) => element.fmt(f),
                ConstructableMapleLoanCalls::EarlyFeeRate(element) => element.fmt(f),
                ConstructableMapleLoanCalls::EndingPrincipal(element) => element.fmt(f),
                ConstructableMapleLoanCalls::ExcessCollateral(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Factory(element) => element.fmt(f),
                ConstructableMapleLoanCalls::FundLoan(element) => element.fmt(f),
                ConstructableMapleLoanCalls::FundsAsset(element) => element.fmt(f),
                ConstructableMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => {
                    element.fmt(f)
                }
                ConstructableMapleLoanCalls::GetCollateralRequiredFor(element) => element.fmt(f),
                ConstructableMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.fmt(f),
                ConstructableMapleLoanCalls::GetNextPaymentBreakdown(element) => element.fmt(f),
                ConstructableMapleLoanCalls::GetRefinanceInterest(element) => element.fmt(f),
                ConstructableMapleLoanCalls::GracePeriod(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Implementation(element) => element.fmt(f),
                ConstructableMapleLoanCalls::InterestRate(element) => element.fmt(f),
                ConstructableMapleLoanCalls::IsProtocolPaused(element) => element.fmt(f),
                ConstructableMapleLoanCalls::LateFeeRate(element) => element.fmt(f),
                ConstructableMapleLoanCalls::LateInterestPremium(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Lender(element) => element.fmt(f),
                ConstructableMapleLoanCalls::MakePayment(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Migrate(element) => element.fmt(f),
                ConstructableMapleLoanCalls::NextPaymentDueDate(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PaymentInterval(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PaymentsRemaining(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PendingBorrower(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PendingLender(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PostCollateral(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Principal(element) => element.fmt(f),
                ConstructableMapleLoanCalls::PrincipalRequested(element) => element.fmt(f),
                ConstructableMapleLoanCalls::ProposeNewTerms(element) => element.fmt(f),
                ConstructableMapleLoanCalls::RefinanceCommitment(element) => element.fmt(f),
                ConstructableMapleLoanCalls::RefinanceInterest(element) => element.fmt(f),
                ConstructableMapleLoanCalls::RejectNewTerms(element) => element.fmt(f),
                ConstructableMapleLoanCalls::RemoveCollateral(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Repossess(element) => element.fmt(f),
                ConstructableMapleLoanCalls::ReturnFunds(element) => element.fmt(f),
                ConstructableMapleLoanCalls::SetImplementation(element) => element.fmt(f),
                ConstructableMapleLoanCalls::SetPendingBorrower(element) => element.fmt(f),
                ConstructableMapleLoanCalls::SetPendingLender(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Skim(element) => element.fmt(f),
                ConstructableMapleLoanCalls::SuperFactory(element) => element.fmt(f),
                ConstructableMapleLoanCalls::TreasuryFee(element) => element.fmt(f),
                ConstructableMapleLoanCalls::Upgrade(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SetFactoryCall> for ConstructableMapleLoanCalls {
        fn from(var: SetFactoryCall) -> Self {
            ConstructableMapleLoanCalls::SetFactory(var)
        }
    }
    impl ::std::convert::From<AcceptBorrowerCall> for ConstructableMapleLoanCalls {
        fn from(var: AcceptBorrowerCall) -> Self {
            ConstructableMapleLoanCalls::AcceptBorrower(var)
        }
    }
    impl ::std::convert::From<AcceptLenderCall> for ConstructableMapleLoanCalls {
        fn from(var: AcceptLenderCall) -> Self {
            ConstructableMapleLoanCalls::AcceptLender(var)
        }
    }
    impl ::std::convert::From<AcceptNewTermsCall> for ConstructableMapleLoanCalls {
        fn from(var: AcceptNewTermsCall) -> Self {
            ConstructableMapleLoanCalls::AcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<BorrowerCall> for ConstructableMapleLoanCalls {
        fn from(var: BorrowerCall) -> Self {
            ConstructableMapleLoanCalls::Borrower(var)
        }
    }
    impl ::std::convert::From<ClaimFundsCall> for ConstructableMapleLoanCalls {
        fn from(var: ClaimFundsCall) -> Self {
            ConstructableMapleLoanCalls::ClaimFunds(var)
        }
    }
    impl ::std::convert::From<ClaimableFundsCall> for ConstructableMapleLoanCalls {
        fn from(var: ClaimableFundsCall) -> Self {
            ConstructableMapleLoanCalls::ClaimableFunds(var)
        }
    }
    impl ::std::convert::From<CloseLoanCall> for ConstructableMapleLoanCalls {
        fn from(var: CloseLoanCall) -> Self {
            ConstructableMapleLoanCalls::CloseLoan(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for ConstructableMapleLoanCalls {
        fn from(var: CollateralCall) -> Self {
            ConstructableMapleLoanCalls::Collateral(var)
        }
    }
    impl ::std::convert::From<CollateralAssetCall> for ConstructableMapleLoanCalls {
        fn from(var: CollateralAssetCall) -> Self {
            ConstructableMapleLoanCalls::CollateralAsset(var)
        }
    }
    impl ::std::convert::From<CollateralRequiredCall> for ConstructableMapleLoanCalls {
        fn from(var: CollateralRequiredCall) -> Self {
            ConstructableMapleLoanCalls::CollateralRequired(var)
        }
    }
    impl ::std::convert::From<DelegateFeeCall> for ConstructableMapleLoanCalls {
        fn from(var: DelegateFeeCall) -> Self {
            ConstructableMapleLoanCalls::DelegateFee(var)
        }
    }
    impl ::std::convert::From<DrawableFundsCall> for ConstructableMapleLoanCalls {
        fn from(var: DrawableFundsCall) -> Self {
            ConstructableMapleLoanCalls::DrawableFunds(var)
        }
    }
    impl ::std::convert::From<DrawdownFundsCall> for ConstructableMapleLoanCalls {
        fn from(var: DrawdownFundsCall) -> Self {
            ConstructableMapleLoanCalls::DrawdownFunds(var)
        }
    }
    impl ::std::convert::From<EarlyFeeRateCall> for ConstructableMapleLoanCalls {
        fn from(var: EarlyFeeRateCall) -> Self {
            ConstructableMapleLoanCalls::EarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<EndingPrincipalCall> for ConstructableMapleLoanCalls {
        fn from(var: EndingPrincipalCall) -> Self {
            ConstructableMapleLoanCalls::EndingPrincipal(var)
        }
    }
    impl ::std::convert::From<ExcessCollateralCall> for ConstructableMapleLoanCalls {
        fn from(var: ExcessCollateralCall) -> Self {
            ConstructableMapleLoanCalls::ExcessCollateral(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for ConstructableMapleLoanCalls {
        fn from(var: FactoryCall) -> Self {
            ConstructableMapleLoanCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FundLoanCall> for ConstructableMapleLoanCalls {
        fn from(var: FundLoanCall) -> Self {
            ConstructableMapleLoanCalls::FundLoan(var)
        }
    }
    impl ::std::convert::From<FundsAssetCall> for ConstructableMapleLoanCalls {
        fn from(var: FundsAssetCall) -> Self {
            ConstructableMapleLoanCalls::FundsAsset(var)
        }
    }
    impl ::std::convert::From<GetAdditionalCollateralRequiredForCall> for ConstructableMapleLoanCalls {
        fn from(var: GetAdditionalCollateralRequiredForCall) -> Self {
            ConstructableMapleLoanCalls::GetAdditionalCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetCollateralRequiredForCall> for ConstructableMapleLoanCalls {
        fn from(var: GetCollateralRequiredForCall) -> Self {
            ConstructableMapleLoanCalls::GetCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetEarlyPaymentBreakdownCall> for ConstructableMapleLoanCalls {
        fn from(var: GetEarlyPaymentBreakdownCall) -> Self {
            ConstructableMapleLoanCalls::GetEarlyPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetNextPaymentBreakdownCall> for ConstructableMapleLoanCalls {
        fn from(var: GetNextPaymentBreakdownCall) -> Self {
            ConstructableMapleLoanCalls::GetNextPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetRefinanceInterestCall> for ConstructableMapleLoanCalls {
        fn from(var: GetRefinanceInterestCall) -> Self {
            ConstructableMapleLoanCalls::GetRefinanceInterest(var)
        }
    }
    impl ::std::convert::From<GracePeriodCall> for ConstructableMapleLoanCalls {
        fn from(var: GracePeriodCall) -> Self {
            ConstructableMapleLoanCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for ConstructableMapleLoanCalls {
        fn from(var: ImplementationCall) -> Self {
            ConstructableMapleLoanCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InterestRateCall> for ConstructableMapleLoanCalls {
        fn from(var: InterestRateCall) -> Self {
            ConstructableMapleLoanCalls::InterestRate(var)
        }
    }
    impl ::std::convert::From<IsProtocolPausedCall> for ConstructableMapleLoanCalls {
        fn from(var: IsProtocolPausedCall) -> Self {
            ConstructableMapleLoanCalls::IsProtocolPaused(var)
        }
    }
    impl ::std::convert::From<LateFeeRateCall> for ConstructableMapleLoanCalls {
        fn from(var: LateFeeRateCall) -> Self {
            ConstructableMapleLoanCalls::LateFeeRate(var)
        }
    }
    impl ::std::convert::From<LateInterestPremiumCall> for ConstructableMapleLoanCalls {
        fn from(var: LateInterestPremiumCall) -> Self {
            ConstructableMapleLoanCalls::LateInterestPremium(var)
        }
    }
    impl ::std::convert::From<LenderCall> for ConstructableMapleLoanCalls {
        fn from(var: LenderCall) -> Self {
            ConstructableMapleLoanCalls::Lender(var)
        }
    }
    impl ::std::convert::From<MakePaymentCall> for ConstructableMapleLoanCalls {
        fn from(var: MakePaymentCall) -> Self {
            ConstructableMapleLoanCalls::MakePayment(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for ConstructableMapleLoanCalls {
        fn from(var: MigrateCall) -> Self {
            ConstructableMapleLoanCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<NextPaymentDueDateCall> for ConstructableMapleLoanCalls {
        fn from(var: NextPaymentDueDateCall) -> Self {
            ConstructableMapleLoanCalls::NextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<PaymentIntervalCall> for ConstructableMapleLoanCalls {
        fn from(var: PaymentIntervalCall) -> Self {
            ConstructableMapleLoanCalls::PaymentInterval(var)
        }
    }
    impl ::std::convert::From<PaymentsRemainingCall> for ConstructableMapleLoanCalls {
        fn from(var: PaymentsRemainingCall) -> Self {
            ConstructableMapleLoanCalls::PaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<PendingBorrowerCall> for ConstructableMapleLoanCalls {
        fn from(var: PendingBorrowerCall) -> Self {
            ConstructableMapleLoanCalls::PendingBorrower(var)
        }
    }
    impl ::std::convert::From<PendingLenderCall> for ConstructableMapleLoanCalls {
        fn from(var: PendingLenderCall) -> Self {
            ConstructableMapleLoanCalls::PendingLender(var)
        }
    }
    impl ::std::convert::From<PostCollateralCall> for ConstructableMapleLoanCalls {
        fn from(var: PostCollateralCall) -> Self {
            ConstructableMapleLoanCalls::PostCollateral(var)
        }
    }
    impl ::std::convert::From<PrincipalCall> for ConstructableMapleLoanCalls {
        fn from(var: PrincipalCall) -> Self {
            ConstructableMapleLoanCalls::Principal(var)
        }
    }
    impl ::std::convert::From<PrincipalRequestedCall> for ConstructableMapleLoanCalls {
        fn from(var: PrincipalRequestedCall) -> Self {
            ConstructableMapleLoanCalls::PrincipalRequested(var)
        }
    }
    impl ::std::convert::From<ProposeNewTermsCall> for ConstructableMapleLoanCalls {
        fn from(var: ProposeNewTermsCall) -> Self {
            ConstructableMapleLoanCalls::ProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<RefinanceCommitmentCall> for ConstructableMapleLoanCalls {
        fn from(var: RefinanceCommitmentCall) -> Self {
            ConstructableMapleLoanCalls::RefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<RefinanceInterestCall> for ConstructableMapleLoanCalls {
        fn from(var: RefinanceInterestCall) -> Self {
            ConstructableMapleLoanCalls::RefinanceInterest(var)
        }
    }
    impl ::std::convert::From<RejectNewTermsCall> for ConstructableMapleLoanCalls {
        fn from(var: RejectNewTermsCall) -> Self {
            ConstructableMapleLoanCalls::RejectNewTerms(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for ConstructableMapleLoanCalls {
        fn from(var: RemoveCollateralCall) -> Self {
            ConstructableMapleLoanCalls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepossessCall> for ConstructableMapleLoanCalls {
        fn from(var: RepossessCall) -> Self {
            ConstructableMapleLoanCalls::Repossess(var)
        }
    }
    impl ::std::convert::From<ReturnFundsCall> for ConstructableMapleLoanCalls {
        fn from(var: ReturnFundsCall) -> Self {
            ConstructableMapleLoanCalls::ReturnFunds(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for ConstructableMapleLoanCalls {
        fn from(var: SetImplementationCall) -> Self {
            ConstructableMapleLoanCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<SetPendingBorrowerCall> for ConstructableMapleLoanCalls {
        fn from(var: SetPendingBorrowerCall) -> Self {
            ConstructableMapleLoanCalls::SetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<SetPendingLenderCall> for ConstructableMapleLoanCalls {
        fn from(var: SetPendingLenderCall) -> Self {
            ConstructableMapleLoanCalls::SetPendingLender(var)
        }
    }
    impl ::std::convert::From<SkimCall> for ConstructableMapleLoanCalls {
        fn from(var: SkimCall) -> Self {
            ConstructableMapleLoanCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SuperFactoryCall> for ConstructableMapleLoanCalls {
        fn from(var: SuperFactoryCall) -> Self {
            ConstructableMapleLoanCalls::SuperFactory(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for ConstructableMapleLoanCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            ConstructableMapleLoanCalls::TreasuryFee(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for ConstructableMapleLoanCalls {
        fn from(var: UpgradeCall) -> Self {
            ConstructableMapleLoanCalls::Upgrade(var)
        }
    }
}
