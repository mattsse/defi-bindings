pub use manipulatablemapleloan_mod::*;
#[allow(clippy::too_many_arguments)]
mod manipulatablemapleloan_mod {
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
    #[doc = "ManipulatableMapleLoan was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MANIPULATABLEMAPLELOAN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setClaimableFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setCollateralAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setCollateralRequired\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setDrawableFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setEndingPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setFactory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setFundsAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setNextPaymentDueDate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPaymentInterval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPaymentsRemaining\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setPrincipalRequested\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"__setRefinanceCommitmentHash\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptBorrower\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralRequired\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"drawableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drawdownFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earlyFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"endingPrincipal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excessCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"excessCollateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fundsAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawdown_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAdditionalCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEarlyPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRefinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"proRataInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isProtocolPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateInterestPremium\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextPaymentDueDate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentInterval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentsRemaining\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingBorrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingLender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalRequested\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"skimmed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"superFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"superFactory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MANIPULATABLEMAPLELOAN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50613a32806100206000396000f3fe608060405234801561001057600080fd5b50600436106104285760003560e01c80637c3a00fd1161022b578063b9b1f4e311610130578063d41ddc96116100b8578063e44b387511610087578063e44b387514610910578063e920b1e114610918578063eda0807f1461092b578063f0b1c8d91461093e578063fe85235f1461096e57600080fd5b8063d41ddc96146108ca578063d784d426146108dd578063d8dfeb45146108f0578063dac88561146108f857600080fd5b8063c3fbb6fd116100ff578063c3fbb6fd14610889578063c45a015514610491578063cc32d1761461089c578063ccc04484146108a4578063d05951a0146108b757600080fd5b8063b9b1f4e314610860578063ba5d307814610868578063ba83276b14610870578063bcead63e1461087857600080fd5b8063a7cecb96116101b3578063acb522b411610182578063acb522b414610822578063b69410de14610835578063b86a513e1461083d578063b90d56c214610845578063b96b5c991461085857600080fd5b8063a7cecb96146107e3578063a93c8f8f146107f6578063a97d116114610809578063aabaecd61461081157600080fd5b806387accaf1116101fa57806387accaf11461077d5780638ffc9215146107905780639e10320b14610798578063a06db7dc146107ab578063a3782afa146107b357600080fd5b80637c3a00fd146107215780637df1f1b9146107295780637f7951d91461073a5780638252c1a61461076a57600080fd5b80634003f34d1161033157806362e8a976116102b9578063700f500611610288578063700f5006146106e5578063712b772f146106f657806375a206761461070957806377b3c55c146107115780637a0e6fa11461071957600080fd5b806362e8a9761461067c57806362f6fffa146106b757806369458ba7146106ca5780636b3e5646146106d257600080fd5b806350acb4ee1161030057806350acb4ee1461060a57806350f2012f1461061d5780635114cb52146106305780635c60da1b146106635780635eeb53b41461066b57600080fd5b80634003f34d146105b457806345755dd6146105bc57806347350e9f146105cf5780634eac4235146105f757600080fd5b80631f3f19ab116103b45780633653454e116103835780633653454e1461053a57806337e8b48e1461056a578063390d68551461057d57806339ba9f86146105905780633b99bcee146105a157600080fd5b80631f3f19ab14610504578063267f4ac3146105175780632ead10981461052a57806330fea1ce1461053257600080fd5b80630d49b38c116103fb5780630d49b38c146104915780630fe3d9b7146104b157806311e0f951146104b9578063149de2a0146104e95780631cc1cf46146104fc57600080fd5b806301daa38f1461042d5780630895326f1461043757806308bd94f51461044e5780630c90436614610461575b600080fd5b610435610981565b005b6013545b6040519081526020015b60405180910390f35b61043561045c3660046136d9565b600d55565b61043561046f366004613503565b600280546001600160a01b0319166001600160a01b0392909216919091179055565b610499610a27565b6040516001600160a01b039091168152602001610445565b610435610a36565b6104356104c7366004613503565b600380546001600160a01b0319166001600160a01b0392909216919091179055565b6104356104f73660046136d9565b601355565b60075461043b565b610435610512366004613503565b610ad9565b610435610525366004613503565b610b7e565b600b5461043b565b60095461043b565b610435610548366004613503565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6104356105783660046136d9565b601555565b61043561058b36600461370b565b610c1a565b6005546001600160a01b0316610499565b6104356105af366004613730565b610cdd565b60125461043b565b61043b6105ca3660046136d9565b610dd4565b6105e26105dd366004613503565b610ea9565b60408051928352602083019190915201610445565b61043b6106053660046136d9565b610f75565b610435610618366004613653565b610fbd565b61043b61062b3660046136d9565b6111dd565b61064361063e3660046136d9565b611299565b604080519485526020850193909352918301526060820152608001610445565b6104996113de565b6003546001600160a01b0316610499565b61043561068a366004613503565b6001600160a01b03167f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af155565b6104356106c53660046136d9565b600e55565b6106436113eb565b6104356106e03660046136d9565b601055565b6002546001600160a01b0316610499565b61043b61070436600461353d565b611407565b600c5461043b565b600a5461043b565b61043b6115fe565b60085461043b565b6000546001600160a01b0316610499565b610435610748366004613503565b600180546001600160a01b0319166001600160a01b0392909216919091179055565b6104356107783660046136d9565b601155565b61043561078b3660046135f7565b61163a565b600d5461043b565b61043b6107a63660046136d9565b61174f565b60065461043b565b6104356107c1366004613503565b600580546001600160a01b0319166001600160a01b0392909216919091179055565b6104356107f13660046136d9565b600755565b6104356108043660046136d9565b600f55565b60165461043b565b6004546001600160a01b0316610499565b6104356108303660046135f7565b611779565b60175461043b565b600e5461043b565b6104356108533660046136d9565b601455565b61064361182a565b600f5461043b565b60145461043b565b60155461043b565b6001546001600160a01b0316610499565b610435610897366004613576565b611838565b60185461043b565b61043b6108b236600461370b565b6118dd565b6106436108c53660046136d9565b6119ec565b6104356108d836600461370b565b611b25565b6104356108eb366004613503565b611be6565b60115461043b565b610900611c6f565b6040519015158152602001610445565b60105461043b565b61043b6109263660046135cb565b611d59565b6104356109393660046136d9565b601255565b61043561094c366004613503565b600480546001600160a01b0319166001600160a01b0392909216919091179055565b61043561097c3660046136d9565b600c55565b6002546001600160a01b031633146109e05760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a41423a4e4f545f50454e44494e475f424f52524f57455200000000000060448201526064015b60405180910390fd5b600280546001600160a01b031990811690915560008054339216821781556040517f29bac0ac2b15405bfcc160bb74b6ae7a559b7674ce33db80785ada73e38204d29190a2565b6000610a31611f01565b905090565b6003546001600160a01b03163314610a905760405162461bcd60e51b815260206004820152601860248201527f4d4c3a414c3a4e4f545f50454e44494e475f4c454e444552000000000000000060448201526064016109d7565b600380546001600160a01b031990811690915560018054339216821790556040517fd6165838d2e3db87aa1002b548048673fc6427eefbd1b914e100f3a0deae23e390600090a2565b6000546001600160a01b03163314610b295760405162461bcd60e51b815260206004820152601360248201527226a61d29a8211d2727aa2fa127a92927aba2a960691b60448201526064016109d7565b600280546001600160a01b0319166001600160a01b0383169081179091556040519081527f10f06072822ef73860fedb88933f968d20bb4aadce8a8d360d1124cb6ce1e0b2906020015b60405180910390a150565b6001546001600160a01b03163314610bcc5760405162461bcd60e51b815260206004820152601160248201527026a61d29a8261d2727aa2fa622a72222a960791b60448201526064016109d7565b600380546001600160a01b0319166001600160a01b0383169081179091556040519081527fa3ab02442c80a4102475683f16513c9139a89142be9db9804edfcfbb379fc49290602001610b73565b610c22611c6f565b15610c3f5760405162461bcd60e51b81526004016109d7906138be565b6001546001600160a01b03163314610c8c5760405162461bcd60e51b815260206004820152601060248201526f26a61d21a31d2727aa2fa622a72222a960811b60448201526064016109d7565b806001600160a01b03167f6bd56533ce1c8ea03f7b858ac441b5a86d140a793a7c9e3faecbbe517c2c879183604051610cc791815260200190565b60405180910390a2610cd98282611f30565b5050565b6000546001600160a01b03163314610d2b5760405162461bcd60e51b815260206004820152601160248201527026a61d2a9d2727aa2fa127a92927aba2a960791b60448201526064016109d7565b7faaaa7ee6b0c2f4ee1fa7312c7d5b3623a434da5a1a9ce3cb6e629caa23454ab6838383604051610d5e939291906138ea565b60405180910390a1610d6e611f01565b6001600160a01b031663fe69f7088484846040518463ffffffff1660e01b8152600401610d9d939291906138ea565b600060405180830381600087803b158015610db757600080fd5b505af1158015610dcb573d6000803e3d6000fd5b50505050505050565b6000610dde611c6f565b15610dfb5760405162461bcd60e51b81526004016109d7906138be565b811580610e1b5750600554610e1b906001600160a01b0316333085611fa3565b610e675760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a52463a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016109d7565b7f278e51d3323fbf18b9fb8df3f8b97e31b145bc1146c52e764cf4aa1bfc4ba17d610e9061201a565b60405181815290925060200160405180910390a1919050565b600080610eb4611c6f565b15610ed15760405162461bcd60e51b81526004016109d7906138be565b6001546001600160a01b03163314610f1d5760405162461bcd60e51b815260206004820152600f60248201526e26a61d291d2727aa2fa622a72222a960891b60448201526064016109d7565b610f2683612050565b60408051838152602081018390529294509092506001600160a01b038516917f027e623aab0b174da270ff529cad1c54f09182651e07437d2ac557929b9e5b49910160405180910390a2915091565b600080610f9760145484600f54610f8c91906139a4565b600d54600c546121c5565b601154909150808211610fab576000610fb5565b610fb581836139a4565b949350505050565b610fc5611c6f565b15610fe25760405162461bcd60e51b81526004016109d7906138be565b6001546001600160a01b03163381146110315760405162461bcd60e51b815260206004820152601160248201527026a61d20a72a1d2727aa2fa622a72222a960791b60448201526064016109d7565b6005546001600160a01b0316821580611051575061105181333086611fa3565b61109d5760405162461bcd60e51b815260206004820152601b60248201527f4d4c3a414e543a5452414e534645525f46524f4d5f4641494c4544000000000060448201526064016109d7565b7f7150c332bd889236b6ab42cc34f0853631ceb58827f58a8697b682f13e390a8c6110ca888888886121fe565b888888886040516110df959493929190613890565b60405180910390a17ffe9a32948c4b8ec5c8a8eddeacd3f3621458e8bde95b725b625e5c8f4f2cb54d601754601854604051611125929190918252602082015260400190565b60405180910390a16000611138826124fd565b905080156111d357826001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f38260405161117b91815260200190565b60405180910390a261118e8284836125d7565b6111d35760405162461bcd60e51b815260206004820152601660248201527513530e9053950e9514905394d1915497d1905253115160521b60448201526064016109d7565b5050505050505050565b60006111e7611c6f565b156112045760405162461bcd60e51b81526004016109d7906138be565b8115806112245750600454611224906001600160a01b0316333085611fa3565b6112705760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a50433a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016109d7565b7f437d44b2c697fb69e2b2f25f57fd844e376c25ed28ed5a9c4be88aa1e5c87d12610e90612614565b6000806000806000600f54905060008614806112c857506005546112c8906001600160a01b0316333089611fa3565b6113145760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a4d503a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016109d7565b61131c612640565b600054939850919650945092506001600160a01b0316331480611341575080600f5410155b61138d5760405162461bcd60e51b815260206004820152601960248201527f4d4c3a4d503a43414e4e4f545f5553455f4452415741424c450000000000000060448201526064016109d7565b6040805186815260208101869052908101849052606081018390527f95c4acf903eb698cf367efaaf79a8a58fb4554fcd8503b62af9f9c1b68b59e1e906080015b60405180910390a1509193509193565b6000610a31612737565b50565b6000806000806113f9612761565b929791965094509092509050565b6000611411611c6f565b1561142e5760405162461bcd60e51b81526004016109d7906138be565b6000546001600160a01b031633148061145157506001546001600160a01b031633145b61148b5760405162461bcd60e51b815260206004820152600b60248201526a09874a6749c9ebe82aaa8960ab1b60448201526064016109d7565b6005546001600160a01b038481169116148015906114b757506004546001600160a01b03848116911614155b6114f75760405162461bcd60e51b8152602060048201526011602482015270261d299d24a72b20a624a22faa27a5a2a760791b60448201526064016109d7565b6040516370a0823160e01b81523060048201526001600160a01b0380841691908516907ff1f6a55e7ad487ac8dd8e1d4517348d3b410a7a0bc405ef87b09078dc51b23b69082906370a082319060240160206040518083038186803b15801561155f57600080fd5b505afa158015611573573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061159791906136f2565b60405181815290945060200160405180910390a36115b68383836125d7565b6115f85760405162461bcd60e51b8152602060048201526013602482015272130e94ce9514905394d1915497d19052531151606a1b60448201526064016109d7565b92915050565b600080611615601454600f54600d54600c546121c5565b601154909150818111611629576000611633565b61163382826139a4565b9250505090565b611642611c6f565b1561165f5760405162461bcd60e51b81526004016109d7906138be565b6000546001600160a01b031633146116af5760405162461bcd60e51b815260206004820152601360248201527226a61d28272a1d2727aa2fa127a92927aba2a960691b60448201526064016109d7565b428310156116ff5760405162461bcd60e51b815260206004820152601760248201527f4d4c3a504e543a494e56414c49445f444541444c494e4500000000000000000060448201526064016109d7565b7ff94d2f0322894aaf1bce14561461a8b8b6c9b11a77bbe80f20b804da8a95e4b761172c858585856127c8565b85858585604051611741959493929190613890565b60405180910390a150505050565b600061177282600754601454600e54600854601354601254600a54600b546127f0565b5092915050565b611781611c6f565b1561179e5760405162461bcd60e51b81526004016109d7906138be565b6000546001600160a01b03163314806117c157506001546001600160a01b031633145b6117fd5760405162461bcd60e51b815260206004820152600d60248201526c09874a49ca8749c9ebe82aaa89609b1b60448201526064016109d7565b7f47244a449377da5fd10e98d86d118dee442e842fc34f05179c973cfcff6acba761172c8585858561285f565b6000806000806113f96128cd565b611840611f01565b6001600160a01b0316336001600160a01b0316146118935760405162461bcd60e51b815260206004820152601060248201526f4d4c3a4d3a4e4f545f464143544f525960801b60448201526064016109d7565b61189e838383612918565b6118d85760405162461bcd60e51b815260206004820152600b60248201526a13530e934e91905253115160aa1b60448201526064016109d7565b505050565b60006118e7611c6f565b156119045760405162461bcd60e51b81526004016109d7906138be565b6000546001600160a01b031633146119535760405162461bcd60e51b815260206004820152601260248201527126a61d22231d2727aa2fa127a92927aba2a960711b60448201526064016109d7565b816001600160a01b03167f7578fe8c4d9f6fc38fdad20d219b0ce47d38bbf8a72bdb26867809f24119363d8460405161198e91815260200190565b60405180910390a260006119a184610f75565b905080156119e2576004546000906119c1906001600160a01b03166124fd565b90506119de8183116119d45760006111dd565b61062b82846139a4565b9250505b6117728484612991565b6000806000806000600f5490506000861480611a1b5750600554611a1b906001600160a01b0316333089611fa3565b611a675760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a434c3a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016109d7565b611a6f612a58565b600054939850919650945092506001600160a01b0316331480611a94575080600f5410155b611ae05760405162461bcd60e51b815260206004820152601960248201527f4d4c3a434c3a43414e4e4f545f5553455f4452415741424c450000000000000060448201526064016109d7565b6040805186815260208101869052908101849052606081018390527f6d5b31efac20a15ed5b9e27e38cf9ebcc3ffb6d64feb827a35ef84a607e8dfaf906080016113ce565b611b2d611c6f565b15611b4a5760405162461bcd60e51b81526004016109d7906138be565b6000546001600160a01b03163314611b995760405162461bcd60e51b815260206004820152601260248201527126a61d29219d2727aa2fa127a92927aba2a960711b60448201526064016109d7565b806001600160a01b03167f97b446ee2df422b7273fe6d658674835f9de3319d131c229f9a2f8ed62a7619383604051611bd491815260200190565b60405180910390a2610cd98282612b46565b611bee611f01565b6001600160a01b0316336001600160a01b031614611c425760405162461bcd60e51b81526020600482015260116024820152704d4c3a53493a4e4f545f464143544f525960781b60448201526064016109d7565b6001600160a01b03167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b6000611c79611f01565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b158015611cb157600080fd5b505afa158015611cc5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ce99190613520565b6001600160a01b031663425fad586040518163ffffffff1660e01b815260040160206040518083038186803b158015611d2157600080fd5b505afa158015611d35573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a3191906136b7565b6000611d63611c6f565b15611d805760405162461bcd60e51b81526004016109d7906138be565b6005546001600160a01b0316821580611da05750611da081333086611fa3565b611dec5760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a464c3a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016109d7565b601254611e4657836001600160a01b03167fcd909ec339185c4598a4096e174308fbdf136d117f230960f873a2f2e81f63af611e2786612c0d565b6012546040805183815260208101929092529195500160405180910390a25b6000611e51826124fd565b6001549091506001600160a01b03168115611ef857806001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f383604051611ea191815260200190565b60405180910390a2611eb48382846125d7565b611ef85760405162461bcd60e51b815260206004820152601560248201527413530e91930e9514905394d1915497d19052531151605a1b60448201526064016109d7565b50505092915050565b6000611f2b7f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15490565b919050565b8160106000828254611f4291906139a4565b9091555050600554611f5e906001600160a01b031682846125d7565b610cd95760405162461bcd60e51b81526020600482015260166024820152751353124e90d18e9514905394d1915497d1905253115160521b60448201526064016109d7565b6040516001600160a01b03808516602483015283166044820152606481018290526000906120119086906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612d57565b95945050505050565b600554600090612032906001600160a01b03166124fd565b905080600f6000828254612046919061394b565b9250508190555090565b60125460009081908015801590612072575060065461206f908261394b565b42115b6120b55760405162461bcd60e51b81526020600482015260146024820152731353124e948e9393d517d25397d111519055531560621b60448201526064016109d7565b6120bd612df7565b600060118190556010819055600f8190556004546001600160a01b0316906120e4826124fd565b94508414806120f957506120f98186866125d7565b6121455760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a435f5452414e534645525f4641494c454400000000000000000060448201526064016109d7565b6005546001600160a01b0316600061215c826124fd565b945084148061217157506121718187866125d7565b6121bd5760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a465f5452414e534645525f4641494c454400000000000000000060448201526064016109d7565b505050915091565b6000838511156121f357826121da85876139a4565b6121e49084613985565b6121ee9190613963565b612011565b506000949350505050565b600061220c85858585612e35565b9050806015541461225f5760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a414e543a434f4d4d49544d454e545f4d49534d41544348000000000060448201526064016109d7565b6001600160a01b0385163b6122b65760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a494e56414c49445f524546494e414e43455200000000000060448201526064016109d7565b834211156123065760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a455850495245445f434f4d4d49544d454e5400000000000060448201526064016109d7565b6000600754905060008061232f4284601454600e54600854601354601254600a54600b546127f0565b915091508160166000828254612345919061394b565b90915550506000601581905585905b818110156124255760008a6001600160a01b031689898481811061237a5761237a6139d1565b905060200281019061238c9190613904565b60405161239a92919061381d565b600060405180830381855af49150503d80600081146123d5576040519150601f19603f3d011682016040523d82523d6000602084013e6123da565b606091505b505090508061241c5760405162461bcd60e51b815260206004820152600e60248201526d1353124e9053950e91905253115160921b60448201526064016109d7565b50600101612354565b506000601354856124369190613985565b836017546124449190613985565b61244e9190613963565b90506000601354866124609190613985565b8460185461246e9190613985565b6124789190613963565b600754909150612488814261394b565b60125560145461249a90828585612e6e565b6124a2612fcc565b6124ee5760405162461bcd60e51b815260206004820152601f60248201527f4d4c493a414e543a494e53554646494349454e545f434f4c4c41544552414c0060448201526064016109d7565b50505050505050949350505050565b6005546000906001600160a01b0383811691161461251c57600061252c565b600f5460105461252c919061394b565b6004546001600160a01b0384811691161461254857600061254c565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561258b57600080fd5b505afa15801561259f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125c391906136f2565b6125cd91906139a4565b6115f891906139a4565b6040516001600160a01b03831660248201526044810182905260009061260a90859063a9059cbb60e01b90606401611fda565b90505b9392505050565b60045460009061262c906001600160a01b03166124fd565b90508060116000828254612046919061394b565b60008060008061264e6128cd565b6000601681905593975091955093509150612669848661394b565b905081612676848361394b565b612680919061394b565b600554612695906001600160a01b03166124fd565b600f546126a2919061394b565b6126ac91906139a4565b600f8190555080601060008282546126c4919061394b565b909155506126d490508383612fec565b60135460018114156126ed576126e8612df7565b61272f565b60075460126000828254612701919061394b565b92505081905550856014600082825461271a91906139a4565b9091555061272b90506001826139a4565b6013555b505090919293565b6000611f2b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b600080600080601654670de0b6b3a76400006009546014549650866127869190613985565b6127909190613963565b61279a919061394b565b601354601754919450906127af908290613985565b9250806018546127bf9190613985565b91505090919293565b6000816127d65760006127e2565b6127e285858585612e35565b601581905595945050505050565b600080846127fe8b8d61394b565b101561280f57506000905080612851565b6128198a866139a4565b612823908c6139a4565b9050612832898989848a61305e565b925061284490508b8a89888888613151565b61284e908361394b565b91505b995099975050505050505050565b600061286d85858585612e35565b905080601554146128c05760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a524e543a434f4d4d49544d454e545f4d49534d41544348000000000060448201526064016109d7565b6000601555949350505050565b6000806000806128f442601254600754601454600e54601354600854600a54600b546131f7565b6016549195509350612906908461394b565b92506017549150601854905090919293565b6000833b8061292b57600091505061260d565b846001600160a01b0316848460405161294592919061381d565b600060405180830381855af49150503d8060008114612980576040519150601f19603f3d011682016040523d82523d6000602084013e612985565b606091505b50909695505050505050565b81600f60008282546129a391906139a4565b90915550506005546129bf906001600160a01b031682846125d7565b612a045760405162461bcd60e51b81526020600482015260166024820152751353124e91118e9514905394d1915497d1905253115160521b60448201526064016109d7565b612a0c612fcc565b610cd95760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a44463a494e53554646494349454e545f434f4c4c41544552414c000060448201526064016109d7565b600080600080601254421115612aa95760405162461bcd60e51b81526020600482015260166024820152754d4c493a434c3a5041594d454e545f49535f4c41544560501b60448201526064016109d7565b612ab1612761565b6000601681905593975091955093509150612acc848661394b565b905081612ad9848361394b565b612ae3919061394b565b600554612af8906001600160a01b03166124fd565b600f54612b05919061394b565b612b0f91906139a4565b600f819055508060106000828254612b27919061394b565b90915550612b3790508383612fec565b612b3f612df7565b5090919293565b8160116000828254612b5891906139a4565b9091555050600454612b74906001600160a01b031682846125d7565b612bb95760405162461bcd60e51b81526020600482015260166024820152751353124e9490ce9514905394d1915497d1905253115160521b60448201526064016109d7565b612bc1612fcc565b610cd95760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a52433a494e53554646494349454e545f434f4c4c41544552414c000060448201526064016109d7565b60006001600160a01b038216612c5d5760405162461bcd60e51b815260206004820152601560248201527426a6249d23261d24a72b20a624a22fa622a72222a960591b60448201526064016109d7565b601354601254158015612c6f57508015155b612cb05760405162461bcd60e51b81526020600482015260126024820152714d4c493a464c3a4c4f414e5f41435449564560701b60448201526064016109d7565b600754600180546001600160a01b0319166001600160a01b038616179055612cd8814261394b565b601255600d5460148190556005549093506001600160a01b031683612cfc826124fd565b1015612d4a5760405162461bcd60e51b815260206004820152601860248201527f4d4c493a464c3a57524f4e475f46554e445f414d4f554e54000000000000000060448201526064016109d7565b505050600f819055919050565b60006001600160a01b0383163b612d70575060006115f8565b6060836001600160a01b031683604051612d8a919061382d565b6000604051808303816000865af19150503d8060008114612dc7576040519150601f19603f3d011682016040523d82523d6000602084013e612dcc565b606091505b509092509050818015610fb5575080511580610fb5575080806020019051810190610fb591906136b7565b60006006819055600781905560088190556009819055600a819055600b819055600e8190556012819055601381905560148190556017819055601855565b600084848484604051602001612e4e9493929190613868565b604051602081830303815290604052805190602001209050949350505050565b6000612e78613246565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612ebb57600080fd5b505afa158015612ecf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ef391906136f2565b612efd9089613985565b612f079190613985565b612f119190613963565b612f1b919061394b565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b158015612f6257600080fd5b505afa158015612f76573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612f9a91906136f2565b612fa49089613985565b612fae9190613985565b612fb89190613963565b612fc2919061394b565b6018555050505050565b6000612fe2601454600f54600d54600c546121c5565b6011541015905090565b60015461300a906001600160a01b0316634046af2b60e01b846132c0565b613026578160106000828254613020919061394b565b90915550505b61303f613031613246565b63a5a2760560e01b836132c0565b610cd9578060106000828254613055919061394b565b90915550505050565b600080600061306d86866133cf565b9050600061309561308683670de0b6b3a764000061394b565b86670de0b6b3a76400006133ea565b9050670de0b6b3a764000081116130c857846130b1898b6139a4565b6130bb9190613963565b6000935093505050613147565b60006130dc670de0b6b3a7640000836139a4565b838a670de0b6b3a76400006130f1868f613985565b6130fb9190613963565b61310591906139a4565b61310f9190613985565b6131199190613963565b90506131268a898961344c565b935083811015613137576000613141565b61314184826139a4565b94505050505b9550959350505050565b6000838711613162575060006131ed565b6000620151806001613174878b6139a4565b61317e91906139a4565b6131889190613963565b61319390600161394b565b6131a09062015180613985565b90506131b6876131b0858961394b565b8361344c565b6131c0908361394b565b9150670de0b6b3a76400006131d58886613985565b6131df9190613963565b6131e9908361394b565b9150505b9695505050505050565b6000806132078888878c8a61305e565b90925090506001861461321a578161321c565b875b915061322c8b89878d8888613151565b613236908261394b565b9050995099975050505050505050565b6000613250611f01565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b15801561328857600080fd5b505afa15801561329c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a319190613520565b6000816132cf5750600161260d565b60408051600481526024810182526020810180516001600160e01b03166001600160e01b03198716179052905160009182916001600160a01b038816916133159161382d565b6000604051808303816000865af19150503d8060008114613352576040519150601f19603f3d011682016040523d82523d6000602084013e613357565b606091505b509150915081158061336b57506020815114155b1561337b5760009250505061260d565b6000818060200190518101906133919190613520565b90506001600160a01b0381166133ad576000935050505061260d565b6005546133c4906001600160a01b031682876125d7565b979650505050505050565b60006301e133806133e08385613985565b61260d9190613963565b6000600183166133fa57816133fc565b835b90505b60019290921c91821561260d57816134178580613985565b6134219190613963565b935060018316613430576133ff565b8161343b8583613985565b6134459190613963565b90506133ff565b6000670de0b6b3a764000061346184846133cf565b61346b9086613985565b61260a9190613963565b60008083601f84011261348757600080fd5b50813567ffffffffffffffff81111561349f57600080fd5b6020830191508360208260051b85010111156134ba57600080fd5b9250929050565b60008083601f8401126134d357600080fd5b50813567ffffffffffffffff8111156134eb57600080fd5b6020830191508360208285010111156134ba57600080fd5b60006020828403121561351557600080fd5b813561260d816139e7565b60006020828403121561353257600080fd5b815161260d816139e7565b6000806040838503121561355057600080fd5b823561355b816139e7565b9150602083013561356b816139e7565b809150509250929050565b60008060006040848603121561358b57600080fd5b8335613596816139e7565b9250602084013567ffffffffffffffff8111156135b257600080fd5b6135be868287016134c1565b9497909650939450505050565b600080604083850312156135de57600080fd5b82356135e9816139e7565b946020939093013593505050565b6000806000806060858703121561360d57600080fd5b8435613618816139e7565b935060208501359250604085013567ffffffffffffffff81111561363b57600080fd5b61364787828801613475565b95989497509550505050565b60008060008060006080868803121561366b57600080fd5b8535613676816139e7565b945060208601359350604086013567ffffffffffffffff81111561369957600080fd5b6136a588828901613475565b96999598509660600135949350505050565b6000602082840312156136c957600080fd5b8151801515811461260d57600080fd5b6000602082840312156136eb57600080fd5b5035919050565b60006020828403121561370457600080fd5b5051919050565b6000806040838503121561371e57600080fd5b82359150602083013561356b816139e7565b60008060006040848603121561374557600080fd5b83359250602084013567ffffffffffffffff8111156135b257600080fd5b81835260006020808501808196508560051b810191508460005b878110156137e75782840389528135601e1988360301811261379e57600080fd5b8701803567ffffffffffffffff8111156137b757600080fd5b8036038913156137c657600080fd5b6137d386828985016137f4565b9a87019a955050509084019060010161377d565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8183823760009101908152919050565b6000825160005b8181101561384e5760208186018101518583015201613834565b8181111561385d576000828501525b509190910192915050565b60018060a01b03851681528360208201526060604082015260006131ed606083018486613763565b85815260018060a01b03851660208201528360408201526080606082015260006133c4608083018486613763565b60208082526012908201527113530e941493d513d0d3d317d4105554d15160721b604082015260600190565b8381526040602082015260006120116040830184866137f4565b6000808335601e1984360301811261391b57600080fd5b83018035915067ffffffffffffffff82111561393657600080fd5b6020019150368190038213156134ba57600080fd5b6000821982111561395e5761395e6139bb565b500190565b60008261398057634e487b7160e01b600052601260045260246000fd5b500490565b600081600019048311821515161561399f5761399f6139bb565b500290565b6000828210156139b6576139b66139bb565b500390565b634e487b7160e01b600052601160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6001600160a01b03811681146113e857600080fdfea2646970667358221220cf3ad50a00c9ef705713d6ef5ef8ce0ee706924267153610fc2180e741b82d4f64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ManipulatableMapleLoan<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ManipulatableMapleLoan<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ManipulatableMapleLoan<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ManipulatableMapleLoan))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ManipulatableMapleLoan<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MANIPULATABLEMAPLELOAN_ABI.clone(),
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
                MANIPULATABLEMAPLELOAN_ABI.clone(),
                MANIPULATABLEMAPLELOAN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `__setBorrower` (0x3653454e) function"]
        pub fn set_borrower(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 83, 69, 78], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setClaimableFunds` (0x6b3e5646) function"]
        pub fn set_claimable_funds(
            &self,
            claimable_funds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 62, 86, 70], claimable_funds)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setCollateral` (0x8252c1a6) function"]
        pub fn set_collateral(
            &self,
            collateral: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 82, 193, 166], collateral)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setCollateralAsset` (0xf0b1c8d9) function"]
        pub fn set_collateral_asset(
            &self,
            collateral_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 177, 200, 217], collateral_asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setCollateralRequired` (0xfe85235f) function"]
        pub fn set_collateral_required(
            &self,
            collateral_required: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 133, 35, 95], collateral_required)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setDrawableFunds` (0xa93c8f8f) function"]
        pub fn set_drawable_funds(
            &self,
            drawable_funds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 60, 143, 143], drawable_funds)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setEndingPrincipal` (0x62f6fffa) function"]
        pub fn set_ending_principal(
            &self,
            ending_principal: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 246, 255, 250], ending_principal)
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `__setFundsAsset` (0xa3782afa) function"]
        pub fn set_funds_asset(
            &self,
            funds_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 120, 42, 250], funds_asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setLender` (0x7f7951d9) function"]
        pub fn set_lender(
            &self,
            lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 121, 81, 217], lender)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setNextPaymentDueDate` (0xeda0807f) function"]
        pub fn set_next_payment_due_date(
            &self,
            next_payment_due_date: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 160, 128, 127], next_payment_due_date)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPaymentInterval` (0xa7cecb96) function"]
        pub fn set_payment_interval(
            &self,
            payment_interval: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 206, 203, 150], payment_interval)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPaymentsRemaining` (0x149de2a0) function"]
        pub fn set_payments_remaining(
            &self,
            payments_remaining: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 157, 226, 160], payments_remaining)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPendingBorrower` (0x0c904366) function"]
        pub fn __set_pending_borrower(
            &self,
            pending_borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 144, 67, 102], pending_borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPendingLender` (0x11e0f951) function"]
        pub fn __set_pending_lender(
            &self,
            pending_lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 224, 249, 81], pending_lender)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPrincipal` (0xb90d56c2) function"]
        pub fn set_principal(
            &self,
            principal: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 13, 86, 194], principal)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setPrincipalRequested` (0x08bd94f5) function"]
        pub fn set_principal_requested(
            &self,
            principal_requested: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 189, 148, 245], principal_requested)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `__setRefinanceCommitmentHash` (0x37e8b48e) function"]
        pub fn set_refinance_commitment_hash(
            &self,
            refinance_commitment: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 232, 180, 142], refinance_commitment)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ManipulatableMapleLoanEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ManipulatableMapleLoan<M>
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
    pub enum ManipulatableMapleLoanEvents {
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
    impl ethers::contract::EthLogDecode for ManipulatableMapleLoanEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::BorrowerAcceptedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::CollateralPostedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::CollateralRemovedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::EstablishmentFeesSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::NewTermsAcceptedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::NewTermsProposedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::NewTermsRejectedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::PendingBorrowerSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::PendingLenderSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::SkimmedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ManipulatableMapleLoanEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ManipulatableMapleLoanEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ManipulatableMapleLoanEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::CollateralPostedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::CollateralRemovedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::FundedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::FundsClaimedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::FundsRedirectedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::FundsReturnedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::InitializedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::LenderAcceptedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::LoanClosedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::NewTermsProposedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::PaymentMadeFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::PendingLenderSetFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::RepossessedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::SkimmedFilter(element) => element.fmt(f),
                ManipulatableMapleLoanEvents::UpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `__setBorrower`function with signature `__setBorrower(address)` and selector `[54, 83, 69, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setBorrower", abi = "__setBorrower(address)")]
    pub struct SetBorrowerCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setClaimableFunds`function with signature `__setClaimableFunds(uint256)` and selector `[107, 62, 86, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setClaimableFunds", abi = "__setClaimableFunds(uint256)")]
    pub struct SetClaimableFundsCall {
        pub claimable_funds: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setCollateral`function with signature `__setCollateral(uint256)` and selector `[130, 82, 193, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setCollateral", abi = "__setCollateral(uint256)")]
    pub struct SetCollateralCall {
        pub collateral: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setCollateralAsset`function with signature `__setCollateralAsset(address)` and selector `[240, 177, 200, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setCollateralAsset", abi = "__setCollateralAsset(address)")]
    pub struct SetCollateralAssetCall {
        pub collateral_asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setCollateralRequired`function with signature `__setCollateralRequired(uint256)` and selector `[254, 133, 35, 95]`"]
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
        name = "__setCollateralRequired",
        abi = "__setCollateralRequired(uint256)"
    )]
    pub struct SetCollateralRequiredCall {
        pub collateral_required: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setDrawableFunds`function with signature `__setDrawableFunds(uint256)` and selector `[169, 60, 143, 143]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setDrawableFunds", abi = "__setDrawableFunds(uint256)")]
    pub struct SetDrawableFundsCall {
        pub drawable_funds: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setEndingPrincipal`function with signature `__setEndingPrincipal(uint256)` and selector `[98, 246, 255, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setEndingPrincipal", abi = "__setEndingPrincipal(uint256)")]
    pub struct SetEndingPrincipalCall {
        pub ending_principal: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `__setFundsAsset`function with signature `__setFundsAsset(address)` and selector `[163, 120, 42, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setFundsAsset", abi = "__setFundsAsset(address)")]
    pub struct SetFundsAssetCall {
        pub funds_asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setLender`function with signature `__setLender(address)` and selector `[127, 121, 81, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setLender", abi = "__setLender(address)")]
    pub struct SetLenderCall {
        pub lender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setNextPaymentDueDate`function with signature `__setNextPaymentDueDate(uint256)` and selector `[237, 160, 128, 127]`"]
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
        name = "__setNextPaymentDueDate",
        abi = "__setNextPaymentDueDate(uint256)"
    )]
    pub struct SetNextPaymentDueDateCall {
        pub next_payment_due_date: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setPaymentInterval`function with signature `__setPaymentInterval(uint256)` and selector `[167, 206, 203, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setPaymentInterval", abi = "__setPaymentInterval(uint256)")]
    pub struct SetPaymentIntervalCall {
        pub payment_interval: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setPaymentsRemaining`function with signature `__setPaymentsRemaining(uint256)` and selector `[20, 157, 226, 160]`"]
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
        name = "__setPaymentsRemaining",
        abi = "__setPaymentsRemaining(uint256)"
    )]
    pub struct SetPaymentsRemainingCall {
        pub payments_remaining: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setPendingBorrower`function with signature `__setPendingBorrower(address)` and selector `[12, 144, 67, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setPendingBorrower", abi = "__setPendingBorrower(address)")]
    pub struct __SetPendingBorrowerCall {
        pub pending_borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setPendingLender`function with signature `__setPendingLender(address)` and selector `[17, 224, 249, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setPendingLender", abi = "__setPendingLender(address)")]
    pub struct __SetPendingLenderCall {
        pub pending_lender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `__setPrincipal`function with signature `__setPrincipal(uint256)` and selector `[185, 13, 86, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "__setPrincipal", abi = "__setPrincipal(uint256)")]
    pub struct SetPrincipalCall {
        pub principal: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setPrincipalRequested`function with signature `__setPrincipalRequested(uint256)` and selector `[8, 189, 148, 245]`"]
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
        name = "__setPrincipalRequested",
        abi = "__setPrincipalRequested(uint256)"
    )]
    pub struct SetPrincipalRequestedCall {
        pub principal_requested: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `__setRefinanceCommitmentHash`function with signature `__setRefinanceCommitmentHash(bytes32)` and selector `[55, 232, 180, 142]`"]
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
        name = "__setRefinanceCommitmentHash",
        abi = "__setRefinanceCommitmentHash(bytes32)"
    )]
    pub struct SetRefinanceCommitmentHashCall {
        pub refinance_commitment: [u8; 32],
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
    pub enum ManipulatableMapleLoanCalls {
        SetBorrower(SetBorrowerCall),
        SetClaimableFunds(SetClaimableFundsCall),
        SetCollateral(SetCollateralCall),
        SetCollateralAsset(SetCollateralAssetCall),
        SetCollateralRequired(SetCollateralRequiredCall),
        SetDrawableFunds(SetDrawableFundsCall),
        SetEndingPrincipal(SetEndingPrincipalCall),
        SetFactory(SetFactoryCall),
        SetFundsAsset(SetFundsAssetCall),
        SetLender(SetLenderCall),
        SetNextPaymentDueDate(SetNextPaymentDueDateCall),
        SetPaymentInterval(SetPaymentIntervalCall),
        SetPaymentsRemaining(SetPaymentsRemainingCall),
        __SetPendingBorrower(__SetPendingBorrowerCall),
        __SetPendingLender(__SetPendingLenderCall),
        SetPrincipal(SetPrincipalCall),
        SetPrincipalRequested(SetPrincipalRequestedCall),
        SetRefinanceCommitmentHash(SetRefinanceCommitmentHashCall),
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
    impl ethers::core::abi::AbiDecode for ManipulatableMapleLoanCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SetBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetCollateral(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetCollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetCollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <SetDrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetDrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <SetEndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetEndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetFactory(decoded));
            }
            if let Ok(decoded) =
                <SetFundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetFundsAsset(decoded));
            }
            if let Ok(decoded) =
                <SetLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetLender(decoded));
            }
            if let Ok(decoded) =
                <SetNextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetNextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <__SetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::__SetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <__SetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::__SetPendingLender(decoded));
            }
            if let Ok(decoded) =
                <SetPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetPrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <SetRefinanceCommitmentHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ManipulatableMapleLoanCalls::SetRefinanceCommitmentHash(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::AcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <AcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::AcceptLender(decoded));
            }
            if let Ok(decoded) =
                <AcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::AcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <BorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Borrower(decoded));
            }
            if let Ok(decoded) =
                <ClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::ClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::ClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <CloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::CloseLoan(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Collateral(decoded));
            }
            if let Ok(decoded) =
                <CollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::CollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <CollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::CollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <DelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::DelegateFee(decoded));
            }
            if let Ok(decoded) =
                <DrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::DrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <DrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::DrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <EarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::EarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <EndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::EndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <ExcessCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::ExcessCollateral(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::FundLoan(decoded));
            }
            if let Ok(decoded) =
                <FundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::FundsAsset(decoded));
            }
            if let Ok(decoded) =
                <GetAdditionalCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    ManipulatableMapleLoanCalls::GetAdditionalCollateralRequiredFor(decoded),
                );
            }
            if let Ok(decoded) =
                <GetEarlyPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ManipulatableMapleLoanCalls::GetEarlyPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetNextPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::GetNextPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::GetRefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::InterestRate(decoded));
            }
            if let Ok(decoded) =
                <IsProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::IsProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <LateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::LateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <LateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::LateInterestPremium(decoded));
            }
            if let Ok(decoded) = <LenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Lender(decoded));
            }
            if let Ok(decoded) =
                <MakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::MakePayment(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <NextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::NextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <PaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <PaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <PendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <PendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PendingLender(decoded));
            }
            if let Ok(decoded) =
                <PostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PostCollateral(decoded));
            }
            if let Ok(decoded) =
                <PrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Principal(decoded));
            }
            if let Ok(decoded) =
                <PrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::PrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::ProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::RefinanceCommitment(decoded));
            }
            if let Ok(decoded) =
                <RefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::RefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <RejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::RejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <RepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Repossess(decoded));
            }
            if let Ok(decoded) =
                <ReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::ReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SetPendingLender(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ManipulatableMapleLoanCalls::Skim(decoded));
            }
            if let Ok(decoded) =
                <SuperFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::SuperFactory(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::TreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ManipulatableMapleLoanCalls::Upgrade(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ManipulatableMapleLoanCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ManipulatableMapleLoanCalls::SetBorrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetClaimableFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetCollateral(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetCollateralAsset(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetCollateralRequired(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetDrawableFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetEndingPrincipal(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetFactory(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetFundsAsset(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetLender(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetNextPaymentDueDate(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPaymentInterval(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPaymentsRemaining(element) => element.encode(),
                ManipulatableMapleLoanCalls::__SetPendingBorrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::__SetPendingLender(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPrincipal(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPrincipalRequested(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetRefinanceCommitmentHash(element) => {
                    element.encode()
                }
                ManipulatableMapleLoanCalls::AcceptBorrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::AcceptLender(element) => element.encode(),
                ManipulatableMapleLoanCalls::AcceptNewTerms(element) => element.encode(),
                ManipulatableMapleLoanCalls::Borrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::ClaimFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::ClaimableFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::CloseLoan(element) => element.encode(),
                ManipulatableMapleLoanCalls::Collateral(element) => element.encode(),
                ManipulatableMapleLoanCalls::CollateralAsset(element) => element.encode(),
                ManipulatableMapleLoanCalls::CollateralRequired(element) => element.encode(),
                ManipulatableMapleLoanCalls::DelegateFee(element) => element.encode(),
                ManipulatableMapleLoanCalls::DrawableFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::DrawdownFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::EarlyFeeRate(element) => element.encode(),
                ManipulatableMapleLoanCalls::EndingPrincipal(element) => element.encode(),
                ManipulatableMapleLoanCalls::ExcessCollateral(element) => element.encode(),
                ManipulatableMapleLoanCalls::Factory(element) => element.encode(),
                ManipulatableMapleLoanCalls::FundLoan(element) => element.encode(),
                ManipulatableMapleLoanCalls::FundsAsset(element) => element.encode(),
                ManipulatableMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => {
                    element.encode()
                }
                ManipulatableMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.encode(),
                ManipulatableMapleLoanCalls::GetNextPaymentBreakdown(element) => element.encode(),
                ManipulatableMapleLoanCalls::GetRefinanceInterest(element) => element.encode(),
                ManipulatableMapleLoanCalls::GracePeriod(element) => element.encode(),
                ManipulatableMapleLoanCalls::Implementation(element) => element.encode(),
                ManipulatableMapleLoanCalls::InterestRate(element) => element.encode(),
                ManipulatableMapleLoanCalls::IsProtocolPaused(element) => element.encode(),
                ManipulatableMapleLoanCalls::LateFeeRate(element) => element.encode(),
                ManipulatableMapleLoanCalls::LateInterestPremium(element) => element.encode(),
                ManipulatableMapleLoanCalls::Lender(element) => element.encode(),
                ManipulatableMapleLoanCalls::MakePayment(element) => element.encode(),
                ManipulatableMapleLoanCalls::Migrate(element) => element.encode(),
                ManipulatableMapleLoanCalls::NextPaymentDueDate(element) => element.encode(),
                ManipulatableMapleLoanCalls::PaymentInterval(element) => element.encode(),
                ManipulatableMapleLoanCalls::PaymentsRemaining(element) => element.encode(),
                ManipulatableMapleLoanCalls::PendingBorrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::PendingLender(element) => element.encode(),
                ManipulatableMapleLoanCalls::PostCollateral(element) => element.encode(),
                ManipulatableMapleLoanCalls::Principal(element) => element.encode(),
                ManipulatableMapleLoanCalls::PrincipalRequested(element) => element.encode(),
                ManipulatableMapleLoanCalls::ProposeNewTerms(element) => element.encode(),
                ManipulatableMapleLoanCalls::RefinanceCommitment(element) => element.encode(),
                ManipulatableMapleLoanCalls::RefinanceInterest(element) => element.encode(),
                ManipulatableMapleLoanCalls::RejectNewTerms(element) => element.encode(),
                ManipulatableMapleLoanCalls::RemoveCollateral(element) => element.encode(),
                ManipulatableMapleLoanCalls::Repossess(element) => element.encode(),
                ManipulatableMapleLoanCalls::ReturnFunds(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetImplementation(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPendingBorrower(element) => element.encode(),
                ManipulatableMapleLoanCalls::SetPendingLender(element) => element.encode(),
                ManipulatableMapleLoanCalls::Skim(element) => element.encode(),
                ManipulatableMapleLoanCalls::SuperFactory(element) => element.encode(),
                ManipulatableMapleLoanCalls::TreasuryFee(element) => element.encode(),
                ManipulatableMapleLoanCalls::Upgrade(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ManipulatableMapleLoanCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ManipulatableMapleLoanCalls::SetBorrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetClaimableFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetCollateral(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetCollateralAsset(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetCollateralRequired(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetDrawableFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetEndingPrincipal(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetFactory(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetFundsAsset(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetLender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetNextPaymentDueDate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPaymentInterval(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPaymentsRemaining(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::__SetPendingBorrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::__SetPendingLender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPrincipal(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPrincipalRequested(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetRefinanceCommitmentHash(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::AcceptBorrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::AcceptLender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::AcceptNewTerms(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Borrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::ClaimFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::ClaimableFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::CloseLoan(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Collateral(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::CollateralAsset(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::CollateralRequired(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::DelegateFee(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::DrawableFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::DrawdownFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::EarlyFeeRate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::EndingPrincipal(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::ExcessCollateral(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Factory(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::FundLoan(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::FundsAsset(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => {
                    element.fmt(f)
                }
                ManipulatableMapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::GetNextPaymentBreakdown(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::GetRefinanceInterest(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::GracePeriod(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Implementation(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::InterestRate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::IsProtocolPaused(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::LateFeeRate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::LateInterestPremium(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Lender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::MakePayment(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Migrate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::NextPaymentDueDate(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PaymentInterval(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PaymentsRemaining(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PendingBorrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PendingLender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PostCollateral(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Principal(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::PrincipalRequested(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::ProposeNewTerms(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::RefinanceCommitment(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::RefinanceInterest(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::RejectNewTerms(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::RemoveCollateral(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Repossess(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::ReturnFunds(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetImplementation(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPendingBorrower(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SetPendingLender(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Skim(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::SuperFactory(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::TreasuryFee(element) => element.fmt(f),
                ManipulatableMapleLoanCalls::Upgrade(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SetBorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetBorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::SetBorrower(var)
        }
    }
    impl ::std::convert::From<SetClaimableFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetClaimableFundsCall) -> Self {
            ManipulatableMapleLoanCalls::SetClaimableFunds(var)
        }
    }
    impl ::std::convert::From<SetCollateralCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetCollateralCall) -> Self {
            ManipulatableMapleLoanCalls::SetCollateral(var)
        }
    }
    impl ::std::convert::From<SetCollateralAssetCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetCollateralAssetCall) -> Self {
            ManipulatableMapleLoanCalls::SetCollateralAsset(var)
        }
    }
    impl ::std::convert::From<SetCollateralRequiredCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetCollateralRequiredCall) -> Self {
            ManipulatableMapleLoanCalls::SetCollateralRequired(var)
        }
    }
    impl ::std::convert::From<SetDrawableFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetDrawableFundsCall) -> Self {
            ManipulatableMapleLoanCalls::SetDrawableFunds(var)
        }
    }
    impl ::std::convert::From<SetEndingPrincipalCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetEndingPrincipalCall) -> Self {
            ManipulatableMapleLoanCalls::SetEndingPrincipal(var)
        }
    }
    impl ::std::convert::From<SetFactoryCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetFactoryCall) -> Self {
            ManipulatableMapleLoanCalls::SetFactory(var)
        }
    }
    impl ::std::convert::From<SetFundsAssetCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetFundsAssetCall) -> Self {
            ManipulatableMapleLoanCalls::SetFundsAsset(var)
        }
    }
    impl ::std::convert::From<SetLenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetLenderCall) -> Self {
            ManipulatableMapleLoanCalls::SetLender(var)
        }
    }
    impl ::std::convert::From<SetNextPaymentDueDateCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetNextPaymentDueDateCall) -> Self {
            ManipulatableMapleLoanCalls::SetNextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<SetPaymentIntervalCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPaymentIntervalCall) -> Self {
            ManipulatableMapleLoanCalls::SetPaymentInterval(var)
        }
    }
    impl ::std::convert::From<SetPaymentsRemainingCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPaymentsRemainingCall) -> Self {
            ManipulatableMapleLoanCalls::SetPaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<__SetPendingBorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: __SetPendingBorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::__SetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<__SetPendingLenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: __SetPendingLenderCall) -> Self {
            ManipulatableMapleLoanCalls::__SetPendingLender(var)
        }
    }
    impl ::std::convert::From<SetPrincipalCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPrincipalCall) -> Self {
            ManipulatableMapleLoanCalls::SetPrincipal(var)
        }
    }
    impl ::std::convert::From<SetPrincipalRequestedCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPrincipalRequestedCall) -> Self {
            ManipulatableMapleLoanCalls::SetPrincipalRequested(var)
        }
    }
    impl ::std::convert::From<SetRefinanceCommitmentHashCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetRefinanceCommitmentHashCall) -> Self {
            ManipulatableMapleLoanCalls::SetRefinanceCommitmentHash(var)
        }
    }
    impl ::std::convert::From<AcceptBorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: AcceptBorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::AcceptBorrower(var)
        }
    }
    impl ::std::convert::From<AcceptLenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: AcceptLenderCall) -> Self {
            ManipulatableMapleLoanCalls::AcceptLender(var)
        }
    }
    impl ::std::convert::From<AcceptNewTermsCall> for ManipulatableMapleLoanCalls {
        fn from(var: AcceptNewTermsCall) -> Self {
            ManipulatableMapleLoanCalls::AcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<BorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: BorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::Borrower(var)
        }
    }
    impl ::std::convert::From<ClaimFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: ClaimFundsCall) -> Self {
            ManipulatableMapleLoanCalls::ClaimFunds(var)
        }
    }
    impl ::std::convert::From<ClaimableFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: ClaimableFundsCall) -> Self {
            ManipulatableMapleLoanCalls::ClaimableFunds(var)
        }
    }
    impl ::std::convert::From<CloseLoanCall> for ManipulatableMapleLoanCalls {
        fn from(var: CloseLoanCall) -> Self {
            ManipulatableMapleLoanCalls::CloseLoan(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for ManipulatableMapleLoanCalls {
        fn from(var: CollateralCall) -> Self {
            ManipulatableMapleLoanCalls::Collateral(var)
        }
    }
    impl ::std::convert::From<CollateralAssetCall> for ManipulatableMapleLoanCalls {
        fn from(var: CollateralAssetCall) -> Self {
            ManipulatableMapleLoanCalls::CollateralAsset(var)
        }
    }
    impl ::std::convert::From<CollateralRequiredCall> for ManipulatableMapleLoanCalls {
        fn from(var: CollateralRequiredCall) -> Self {
            ManipulatableMapleLoanCalls::CollateralRequired(var)
        }
    }
    impl ::std::convert::From<DelegateFeeCall> for ManipulatableMapleLoanCalls {
        fn from(var: DelegateFeeCall) -> Self {
            ManipulatableMapleLoanCalls::DelegateFee(var)
        }
    }
    impl ::std::convert::From<DrawableFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: DrawableFundsCall) -> Self {
            ManipulatableMapleLoanCalls::DrawableFunds(var)
        }
    }
    impl ::std::convert::From<DrawdownFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: DrawdownFundsCall) -> Self {
            ManipulatableMapleLoanCalls::DrawdownFunds(var)
        }
    }
    impl ::std::convert::From<EarlyFeeRateCall> for ManipulatableMapleLoanCalls {
        fn from(var: EarlyFeeRateCall) -> Self {
            ManipulatableMapleLoanCalls::EarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<EndingPrincipalCall> for ManipulatableMapleLoanCalls {
        fn from(var: EndingPrincipalCall) -> Self {
            ManipulatableMapleLoanCalls::EndingPrincipal(var)
        }
    }
    impl ::std::convert::From<ExcessCollateralCall> for ManipulatableMapleLoanCalls {
        fn from(var: ExcessCollateralCall) -> Self {
            ManipulatableMapleLoanCalls::ExcessCollateral(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for ManipulatableMapleLoanCalls {
        fn from(var: FactoryCall) -> Self {
            ManipulatableMapleLoanCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FundLoanCall> for ManipulatableMapleLoanCalls {
        fn from(var: FundLoanCall) -> Self {
            ManipulatableMapleLoanCalls::FundLoan(var)
        }
    }
    impl ::std::convert::From<FundsAssetCall> for ManipulatableMapleLoanCalls {
        fn from(var: FundsAssetCall) -> Self {
            ManipulatableMapleLoanCalls::FundsAsset(var)
        }
    }
    impl ::std::convert::From<GetAdditionalCollateralRequiredForCall> for ManipulatableMapleLoanCalls {
        fn from(var: GetAdditionalCollateralRequiredForCall) -> Self {
            ManipulatableMapleLoanCalls::GetAdditionalCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetEarlyPaymentBreakdownCall> for ManipulatableMapleLoanCalls {
        fn from(var: GetEarlyPaymentBreakdownCall) -> Self {
            ManipulatableMapleLoanCalls::GetEarlyPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetNextPaymentBreakdownCall> for ManipulatableMapleLoanCalls {
        fn from(var: GetNextPaymentBreakdownCall) -> Self {
            ManipulatableMapleLoanCalls::GetNextPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetRefinanceInterestCall> for ManipulatableMapleLoanCalls {
        fn from(var: GetRefinanceInterestCall) -> Self {
            ManipulatableMapleLoanCalls::GetRefinanceInterest(var)
        }
    }
    impl ::std::convert::From<GracePeriodCall> for ManipulatableMapleLoanCalls {
        fn from(var: GracePeriodCall) -> Self {
            ManipulatableMapleLoanCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for ManipulatableMapleLoanCalls {
        fn from(var: ImplementationCall) -> Self {
            ManipulatableMapleLoanCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InterestRateCall> for ManipulatableMapleLoanCalls {
        fn from(var: InterestRateCall) -> Self {
            ManipulatableMapleLoanCalls::InterestRate(var)
        }
    }
    impl ::std::convert::From<IsProtocolPausedCall> for ManipulatableMapleLoanCalls {
        fn from(var: IsProtocolPausedCall) -> Self {
            ManipulatableMapleLoanCalls::IsProtocolPaused(var)
        }
    }
    impl ::std::convert::From<LateFeeRateCall> for ManipulatableMapleLoanCalls {
        fn from(var: LateFeeRateCall) -> Self {
            ManipulatableMapleLoanCalls::LateFeeRate(var)
        }
    }
    impl ::std::convert::From<LateInterestPremiumCall> for ManipulatableMapleLoanCalls {
        fn from(var: LateInterestPremiumCall) -> Self {
            ManipulatableMapleLoanCalls::LateInterestPremium(var)
        }
    }
    impl ::std::convert::From<LenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: LenderCall) -> Self {
            ManipulatableMapleLoanCalls::Lender(var)
        }
    }
    impl ::std::convert::From<MakePaymentCall> for ManipulatableMapleLoanCalls {
        fn from(var: MakePaymentCall) -> Self {
            ManipulatableMapleLoanCalls::MakePayment(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for ManipulatableMapleLoanCalls {
        fn from(var: MigrateCall) -> Self {
            ManipulatableMapleLoanCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<NextPaymentDueDateCall> for ManipulatableMapleLoanCalls {
        fn from(var: NextPaymentDueDateCall) -> Self {
            ManipulatableMapleLoanCalls::NextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<PaymentIntervalCall> for ManipulatableMapleLoanCalls {
        fn from(var: PaymentIntervalCall) -> Self {
            ManipulatableMapleLoanCalls::PaymentInterval(var)
        }
    }
    impl ::std::convert::From<PaymentsRemainingCall> for ManipulatableMapleLoanCalls {
        fn from(var: PaymentsRemainingCall) -> Self {
            ManipulatableMapleLoanCalls::PaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<PendingBorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: PendingBorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::PendingBorrower(var)
        }
    }
    impl ::std::convert::From<PendingLenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: PendingLenderCall) -> Self {
            ManipulatableMapleLoanCalls::PendingLender(var)
        }
    }
    impl ::std::convert::From<PostCollateralCall> for ManipulatableMapleLoanCalls {
        fn from(var: PostCollateralCall) -> Self {
            ManipulatableMapleLoanCalls::PostCollateral(var)
        }
    }
    impl ::std::convert::From<PrincipalCall> for ManipulatableMapleLoanCalls {
        fn from(var: PrincipalCall) -> Self {
            ManipulatableMapleLoanCalls::Principal(var)
        }
    }
    impl ::std::convert::From<PrincipalRequestedCall> for ManipulatableMapleLoanCalls {
        fn from(var: PrincipalRequestedCall) -> Self {
            ManipulatableMapleLoanCalls::PrincipalRequested(var)
        }
    }
    impl ::std::convert::From<ProposeNewTermsCall> for ManipulatableMapleLoanCalls {
        fn from(var: ProposeNewTermsCall) -> Self {
            ManipulatableMapleLoanCalls::ProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<RefinanceCommitmentCall> for ManipulatableMapleLoanCalls {
        fn from(var: RefinanceCommitmentCall) -> Self {
            ManipulatableMapleLoanCalls::RefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<RefinanceInterestCall> for ManipulatableMapleLoanCalls {
        fn from(var: RefinanceInterestCall) -> Self {
            ManipulatableMapleLoanCalls::RefinanceInterest(var)
        }
    }
    impl ::std::convert::From<RejectNewTermsCall> for ManipulatableMapleLoanCalls {
        fn from(var: RejectNewTermsCall) -> Self {
            ManipulatableMapleLoanCalls::RejectNewTerms(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for ManipulatableMapleLoanCalls {
        fn from(var: RemoveCollateralCall) -> Self {
            ManipulatableMapleLoanCalls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepossessCall> for ManipulatableMapleLoanCalls {
        fn from(var: RepossessCall) -> Self {
            ManipulatableMapleLoanCalls::Repossess(var)
        }
    }
    impl ::std::convert::From<ReturnFundsCall> for ManipulatableMapleLoanCalls {
        fn from(var: ReturnFundsCall) -> Self {
            ManipulatableMapleLoanCalls::ReturnFunds(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetImplementationCall) -> Self {
            ManipulatableMapleLoanCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<SetPendingBorrowerCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPendingBorrowerCall) -> Self {
            ManipulatableMapleLoanCalls::SetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<SetPendingLenderCall> for ManipulatableMapleLoanCalls {
        fn from(var: SetPendingLenderCall) -> Self {
            ManipulatableMapleLoanCalls::SetPendingLender(var)
        }
    }
    impl ::std::convert::From<SkimCall> for ManipulatableMapleLoanCalls {
        fn from(var: SkimCall) -> Self {
            ManipulatableMapleLoanCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SuperFactoryCall> for ManipulatableMapleLoanCalls {
        fn from(var: SuperFactoryCall) -> Self {
            ManipulatableMapleLoanCalls::SuperFactory(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for ManipulatableMapleLoanCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            ManipulatableMapleLoanCalls::TreasuryFee(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for ManipulatableMapleLoanCalls {
        fn from(var: UpgradeCall) -> Self {
            ManipulatableMapleLoanCalls::Upgrade(var)
        }
    }
}
