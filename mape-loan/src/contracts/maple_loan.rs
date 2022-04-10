pub use mapleloan_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleloan_mod {
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
    #[doc = "MapleLoan was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLELOAN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Upgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptBorrower\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralRequired\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"drawableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drawdownFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earlyFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"endingPrincipal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excessCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"excessCollateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fundsAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawdown_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAdditionalCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEarlyPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRefinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"proRataInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isProtocolPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"paused_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateInterestPremium\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrate\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextPaymentDueDate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentInterval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentsRemaining\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingBorrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingLender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalRequested\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeNewTerms\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newImplementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"skimmed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"superFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"superFactory_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLELOAN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50613720806100206000396000f3fe608060405234801561001057600080fd5b50600436106103425760003560e01c80637c3a00fd116101b8578063ba5d307811610104578063d05951a0116100a2578063d8dfeb451161007c578063d8dfeb4514610634578063dac885611461063c578063e44b387514610654578063e920b1e11461065c57600080fd5b8063d05951a0146105fb578063d41ddc961461060e578063d784d4261461062157600080fd5b8063c3fbb6fd116100de578063c3fbb6fd146105cd578063c45a015514610368578063cc32d176146105e0578063ccc04484146105e857600080fd5b8063ba5d3078146105ac578063ba83276b146105b4578063bcead63e146105bc57600080fd5b8063a97d116111610171578063b69410de1161014b578063b69410de1461058c578063b86a513e14610594578063b96b5c991461059c578063b9b1f4e3146105a457600080fd5b8063a97d116114610560578063aabaecd614610568578063acb522b41461057957600080fd5b80637c3a00fd146105115780637df1f1b91461051957806387accaf11461052a5780638ffc92151461053d5780639e10320b14610545578063a06db7dc1461055857600080fd5b806345755dd6116102925780635eeb53b411610230578063712b772f1161020a578063712b772f146104e657806375a20676146104f957806377b3c55c146105015780637a0e6fa11461050957600080fd5b80635eeb53b4146104bc57806369458ba7146104cd578063700f5006146104d557600080fd5b806350acb4ee1161026c57806350acb4ee1461045b57806350f2012f1461046e5780635114cb52146104815780635c60da1b146104b457600080fd5b806345755dd61461040d57806347350e9f146104205780634eac42351461044857600080fd5b8063267f4ac3116102ff578063390d6855116102d9578063390d6855146103ce57806339ba9f86146103e15780633b99bcee146103f25780634003f34d1461040557600080fd5b8063267f4ac3146103ab5780632ead1098146103be57806330fea1ce146103c657600080fd5b806301daa38f146103475780630895326f146103515780630d49b38c146103685780630fe3d9b7146103885780631cc1cf46146103905780631f3f19ab14610398575b600080fd5b61034f61066f565b005b6013545b6040519081526020015b60405180910390f35b610370610715565b6040516001600160a01b03909116815260200161035f565b61034f610724565b600754610355565b61034f6103a63660046131f1565b6107c7565b61034f6103b93660046131f1565b61086c565b600b54610355565b600954610355565b61034f6103dc3660046133f9565b610908565b6005546001600160a01b0316610370565b61034f61040036600461341e565b6109cb565b601254610355565b61035561041b3660046133c7565b610ac2565b61043361042e3660046131f1565b610b97565b6040805192835260208301919091520161035f565b6103556104563660046133c7565b610c63565b61034f610469366004613341565b610cab565b61035561047c3660046133c7565b610ecb565b61049461048f3660046133c7565b610f87565b60408051948552602085019390935291830152606082015260800161035f565b6103706110cc565b6003546001600160a01b0316610370565b6104946110d6565b6002546001600160a01b0316610370565b6103556104f436600461322b565b6110f2565b600c54610355565b600a54610355565b6103556112e9565b600854610355565b6000546001600160a01b0316610370565b61034f6105383660046132e5565b611325565b600d54610355565b6103556105533660046133c7565b61143a565b600654610355565b601654610355565b6004546001600160a01b0316610370565b61034f6105873660046132e5565b611464565b601754610355565b600e54610355565b610494611515565b600f54610355565b601454610355565b601554610355565b6001546001600160a01b0316610370565b61034f6105db366004613264565b611523565b601854610355565b6103556105f63660046133f9565b6115c8565b6104946106093660046133c7565b6116d7565b61034f61061c3660046133f9565b611810565b61034f61062f3660046131f1565b6118d1565b601154610355565b61064461195d565b604051901515815260200161035f565b601054610355565b61035561066a3660046132b9565b611a47565b6002546001600160a01b031633146106ce5760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a41423a4e4f545f50454e44494e475f424f52524f57455200000000000060448201526064015b60405180910390fd5b600280546001600160a01b031990811690915560008054339216821781556040517f29bac0ac2b15405bfcc160bb74b6ae7a559b7674ce33db80785ada73e38204d29190a2565b600061071f611bef565b905090565b6003546001600160a01b0316331461077e5760405162461bcd60e51b815260206004820152601860248201527f4d4c3a414c3a4e4f545f50454e44494e475f4c454e444552000000000000000060448201526064016106c5565b600380546001600160a01b031990811690915560018054339216821790556040517fd6165838d2e3db87aa1002b548048673fc6427eefbd1b914e100f3a0deae23e390600090a2565b6000546001600160a01b031633146108175760405162461bcd60e51b815260206004820152601360248201527226a61d29a8211d2727aa2fa127a92927aba2a960691b60448201526064016106c5565b600280546001600160a01b0319166001600160a01b0383169081179091556040519081527f10f06072822ef73860fedb88933f968d20bb4aadce8a8d360d1124cb6ce1e0b2906020015b60405180910390a150565b6001546001600160a01b031633146108ba5760405162461bcd60e51b815260206004820152601160248201527026a61d29a8261d2727aa2fa622a72222a960791b60448201526064016106c5565b600380546001600160a01b0319166001600160a01b0383169081179091556040519081527fa3ab02442c80a4102475683f16513c9139a89142be9db9804edfcfbb379fc49290602001610861565b61091061195d565b1561092d5760405162461bcd60e51b81526004016106c5906135ac565b6001546001600160a01b0316331461097a5760405162461bcd60e51b815260206004820152601060248201526f26a61d21a31d2727aa2fa622a72222a960811b60448201526064016106c5565b806001600160a01b03167f6bd56533ce1c8ea03f7b858ac441b5a86d140a793a7c9e3faecbbe517c2c8791836040516109b591815260200190565b60405180910390a26109c78282611c1e565b5050565b6000546001600160a01b03163314610a195760405162461bcd60e51b815260206004820152601160248201527026a61d2a9d2727aa2fa127a92927aba2a960791b60448201526064016106c5565b7faaaa7ee6b0c2f4ee1fa7312c7d5b3623a434da5a1a9ce3cb6e629caa23454ab6838383604051610a4c939291906135d8565b60405180910390a1610a5c611bef565b6001600160a01b031663fe69f7088484846040518463ffffffff1660e01b8152600401610a8b939291906135d8565b600060405180830381600087803b158015610aa557600080fd5b505af1158015610ab9573d6000803e3d6000fd5b50505050505050565b6000610acc61195d565b15610ae95760405162461bcd60e51b81526004016106c5906135ac565b811580610b095750600554610b09906001600160a01b0316333085611c91565b610b555760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a52463a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016106c5565b7f278e51d3323fbf18b9fb8df3f8b97e31b145bc1146c52e764cf4aa1bfc4ba17d610b7e611d08565b60405181815290925060200160405180910390a1919050565b600080610ba261195d565b15610bbf5760405162461bcd60e51b81526004016106c5906135ac565b6001546001600160a01b03163314610c0b5760405162461bcd60e51b815260206004820152600f60248201526e26a61d291d2727aa2fa622a72222a960891b60448201526064016106c5565b610c1483611d3e565b60408051838152602081018390529294509092506001600160a01b038516917f027e623aab0b174da270ff529cad1c54f09182651e07437d2ac557929b9e5b49910160405180910390a2915091565b600080610c8560145484600f54610c7a9190613692565b600d54600c54611eb3565b601154909150808211610c99576000610ca3565b610ca38183613692565b949350505050565b610cb361195d565b15610cd05760405162461bcd60e51b81526004016106c5906135ac565b6001546001600160a01b0316338114610d1f5760405162461bcd60e51b815260206004820152601160248201527026a61d20a72a1d2727aa2fa622a72222a960791b60448201526064016106c5565b6005546001600160a01b0316821580610d3f5750610d3f81333086611c91565b610d8b5760405162461bcd60e51b815260206004820152601b60248201527f4d4c3a414e543a5452414e534645525f46524f4d5f4641494c4544000000000060448201526064016106c5565b7f7150c332bd889236b6ab42cc34f0853631ceb58827f58a8697b682f13e390a8c610db888888888611eec565b88888888604051610dcd95949392919061357e565b60405180910390a17ffe9a32948c4b8ec5c8a8eddeacd3f3621458e8bde95b725b625e5c8f4f2cb54d601754601854604051610e13929190918252602082015260400190565b60405180910390a16000610e26826121eb565b90508015610ec157826001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f382604051610e6991815260200190565b60405180910390a2610e7c8284836122c5565b610ec15760405162461bcd60e51b815260206004820152601660248201527513530e9053950e9514905394d1915497d1905253115160521b60448201526064016106c5565b5050505050505050565b6000610ed561195d565b15610ef25760405162461bcd60e51b81526004016106c5906135ac565b811580610f125750600454610f12906001600160a01b0316333085611c91565b610f5e5760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a50433a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016106c5565b7f437d44b2c697fb69e2b2f25f57fd844e376c25ed28ed5a9c4be88aa1e5c87d12610b7e612302565b6000806000806000600f5490506000861480610fb65750600554610fb6906001600160a01b0316333089611c91565b6110025760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a4d503a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016106c5565b61100a61232e565b600054939850919650945092506001600160a01b031633148061102f575080600f5410155b61107b5760405162461bcd60e51b815260206004820152601960248201527f4d4c3a4d503a43414e4e4f545f5553455f4452415741424c450000000000000060448201526064016106c5565b6040805186815260208101869052908101849052606081018390527f95c4acf903eb698cf367efaaf79a8a58fb4554fcd8503b62af9f9c1b68b59e1e906080015b60405180910390a1509193509193565b600061071f612425565b6000806000806110e461244f565b929791965094509092509050565b60006110fc61195d565b156111195760405162461bcd60e51b81526004016106c5906135ac565b6000546001600160a01b031633148061113c57506001546001600160a01b031633145b6111765760405162461bcd60e51b815260206004820152600b60248201526a09874a6749c9ebe82aaa8960ab1b60448201526064016106c5565b6005546001600160a01b038481169116148015906111a257506004546001600160a01b03848116911614155b6111e25760405162461bcd60e51b8152602060048201526011602482015270261d299d24a72b20a624a22faa27a5a2a760791b60448201526064016106c5565b6040516370a0823160e01b81523060048201526001600160a01b0380841691908516907ff1f6a55e7ad487ac8dd8e1d4517348d3b410a7a0bc405ef87b09078dc51b23b69082906370a082319060240160206040518083038186803b15801561124a57600080fd5b505afa15801561125e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061128291906133e0565b60405181815290945060200160405180910390a36112a18383836122c5565b6112e35760405162461bcd60e51b8152602060048201526013602482015272130e94ce9514905394d1915497d19052531151606a1b60448201526064016106c5565b92915050565b600080611300601454600f54600d54600c54611eb3565b60115490915081811161131457600061131e565b61131e8282613692565b9250505090565b61132d61195d565b1561134a5760405162461bcd60e51b81526004016106c5906135ac565b6000546001600160a01b0316331461139a5760405162461bcd60e51b815260206004820152601360248201527226a61d28272a1d2727aa2fa127a92927aba2a960691b60448201526064016106c5565b428310156113ea5760405162461bcd60e51b815260206004820152601760248201527f4d4c3a504e543a494e56414c49445f444541444c494e4500000000000000000060448201526064016106c5565b7ff94d2f0322894aaf1bce14561461a8b8b6c9b11a77bbe80f20b804da8a95e4b7611417858585856124b6565b8585858560405161142c95949392919061357e565b60405180910390a150505050565b600061145d82600754601454600e54600854601354601254600a54600b546124de565b5092915050565b61146c61195d565b156114895760405162461bcd60e51b81526004016106c5906135ac565b6000546001600160a01b03163314806114ac57506001546001600160a01b031633145b6114e85760405162461bcd60e51b815260206004820152600d60248201526c09874a49ca8749c9ebe82aaa89609b1b60448201526064016106c5565b7f47244a449377da5fd10e98d86d118dee442e842fc34f05179c973cfcff6acba76114178585858561254d565b6000806000806110e46125bb565b61152b611bef565b6001600160a01b0316336001600160a01b03161461157e5760405162461bcd60e51b815260206004820152601060248201526f4d4c3a4d3a4e4f545f464143544f525960801b60448201526064016106c5565b611589838383612606565b6115c35760405162461bcd60e51b815260206004820152600b60248201526a13530e934e91905253115160aa1b60448201526064016106c5565b505050565b60006115d261195d565b156115ef5760405162461bcd60e51b81526004016106c5906135ac565b6000546001600160a01b0316331461163e5760405162461bcd60e51b815260206004820152601260248201527126a61d22231d2727aa2fa127a92927aba2a960711b60448201526064016106c5565b816001600160a01b03167f7578fe8c4d9f6fc38fdad20d219b0ce47d38bbf8a72bdb26867809f24119363d8460405161167991815260200190565b60405180910390a2600061168c84610c63565b905080156116cd576004546000906116ac906001600160a01b03166121eb565b90506116c98183116116bf576000610ecb565b61047c8284613692565b9250505b61145d848461267f565b6000806000806000600f54905060008614806117065750600554611706906001600160a01b0316333089611c91565b6117525760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a434c3a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016106c5565b61175a612746565b600054939850919650945092506001600160a01b031633148061177f575080600f5410155b6117cb5760405162461bcd60e51b815260206004820152601960248201527f4d4c3a434c3a43414e4e4f545f5553455f4452415741424c450000000000000060448201526064016106c5565b6040805186815260208101869052908101849052606081018390527f6d5b31efac20a15ed5b9e27e38cf9ebcc3ffb6d64feb827a35ef84a607e8dfaf906080016110bc565b61181861195d565b156118355760405162461bcd60e51b81526004016106c5906135ac565b6000546001600160a01b031633146118845760405162461bcd60e51b815260206004820152601260248201527126a61d29219d2727aa2fa127a92927aba2a960711b60448201526064016106c5565b806001600160a01b03167f97b446ee2df422b7273fe6d658674835f9de3319d131c229f9a2f8ed62a76193836040516118bf91815260200190565b60405180910390a26109c78282612834565b6118d9611bef565b6001600160a01b0316336001600160a01b03161461192d5760405162461bcd60e51b81526020600482015260116024820152704d4c3a53493a4e4f545f464143544f525960781b60448201526064016106c5565b6001600160a01b03167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc55565b50565b6000611967611bef565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b15801561199f57600080fd5b505afa1580156119b3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119d7919061320e565b6001600160a01b031663425fad586040518163ffffffff1660e01b815260040160206040518083038186803b158015611a0f57600080fd5b505afa158015611a23573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071f91906133a5565b6000611a5161195d565b15611a6e5760405162461bcd60e51b81526004016106c5906135ac565b6005546001600160a01b0316821580611a8e5750611a8e81333086611c91565b611ada5760405162461bcd60e51b815260206004820152601a60248201527f4d4c3a464c3a5452414e534645525f46524f4d5f4641494c454400000000000060448201526064016106c5565b601254611b3457836001600160a01b03167fcd909ec339185c4598a4096e174308fbdf136d117f230960f873a2f2e81f63af611b15866128fb565b6012546040805183815260208101929092529195500160405180910390a25b6000611b3f826121eb565b6001549091506001600160a01b03168115611be657806001600160a01b03167ff505854d1244de20a434e0eca67ec8de6d69504f7f85594c61102ab4d9a278f383604051611b8f91815260200190565b60405180910390a2611ba28382846122c5565b611be65760405162461bcd60e51b815260206004820152601560248201527413530e91930e9514905394d1915497d19052531151605a1b60448201526064016106c5565b50505092915050565b6000611c197f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15490565b919050565b8160106000828254611c309190613692565b9091555050600554611c4c906001600160a01b031682846122c5565b6109c75760405162461bcd60e51b81526020600482015260166024820152751353124e90d18e9514905394d1915497d1905253115160521b60448201526064016106c5565b6040516001600160a01b0380851660248301528316604482015260648101829052600090611cff9086906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152612a45565b95945050505050565b600554600090611d20906001600160a01b03166121eb565b905080600f6000828254611d349190613639565b9250508190555090565b60125460009081908015801590611d605750600654611d5d9082613639565b42115b611da35760405162461bcd60e51b81526020600482015260146024820152731353124e948e9393d517d25397d111519055531560621b60448201526064016106c5565b611dab612ae5565b600060118190556010819055600f8190556004546001600160a01b031690611dd2826121eb565b9450841480611de75750611de78186866122c5565b611e335760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a435f5452414e534645525f4641494c454400000000000000000060448201526064016106c5565b6005546001600160a01b03166000611e4a826121eb565b9450841480611e5f5750611e5f8187866122c5565b611eab5760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a465f5452414e534645525f4641494c454400000000000000000060448201526064016106c5565b505050915091565b600083851115611ee15782611ec88587613692565b611ed29084613673565b611edc9190613651565b611cff565b506000949350505050565b6000611efa85858585612b23565b90508060155414611f4d5760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a414e543a434f4d4d49544d454e545f4d49534d41544348000000000060448201526064016106c5565b6001600160a01b0385163b611fa45760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a494e56414c49445f524546494e414e43455200000000000060448201526064016106c5565b83421115611ff45760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a455850495245445f434f4d4d49544d454e5400000000000060448201526064016106c5565b6000600754905060008061201d4284601454600e54600854601354601254600a54600b546124de565b9150915081601660008282546120339190613639565b90915550506000601581905585905b818110156121135760008a6001600160a01b0316898984818110612068576120686136bf565b905060200281019061207a91906135f2565b60405161208892919061350b565b600060405180830381855af49150503d80600081146120c3576040519150601f19603f3d011682016040523d82523d6000602084013e6120c8565b606091505b505090508061210a5760405162461bcd60e51b815260206004820152600e60248201526d1353124e9053950e91905253115160921b60448201526064016106c5565b50600101612042565b506000601354856121249190613673565b836017546121329190613673565b61213c9190613651565b905060006013548661214e9190613673565b8460185461215c9190613673565b6121669190613651565b6007549091506121768142613639565b60125560145461218890828585612b5c565b612190612cba565b6121dc5760405162461bcd60e51b815260206004820152601f60248201527f4d4c493a414e543a494e53554646494349454e545f434f4c4c41544552414c0060448201526064016106c5565b50505050505050949350505050565b6005546000906001600160a01b0383811691161461220a57600061221a565b600f5460105461221a9190613639565b6004546001600160a01b0384811691161461223657600061223a565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561227957600080fd5b505afa15801561228d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122b191906133e0565b6122bb9190613692565b6112e39190613692565b6040516001600160a01b0383166024820152604481018290526000906122f890859063a9059cbb60e01b90606401611cc8565b90505b9392505050565b60045460009061231a906001600160a01b03166121eb565b90508060116000828254611d349190613639565b60008060008061233c6125bb565b60006016819055939750919550935091506123578486613639565b9050816123648483613639565b61236e9190613639565b600554612383906001600160a01b03166121eb565b600f546123909190613639565b61239a9190613692565b600f8190555080601060008282546123b29190613639565b909155506123c290508383612cda565b60135460018114156123db576123d6612ae5565b61241d565b600754601260008282546123ef9190613639565b9250508190555085601460008282546124089190613692565b909155506124199050600182613692565b6013555b505090919293565b6000611c197f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b600080600080601654670de0b6b3a76400006009546014549650866124749190613673565b61247e9190613651565b6124889190613639565b6013546017549194509061249d908290613673565b9250806018546124ad9190613673565b91505090919293565b6000816124c45760006124d0565b6124d085858585612b23565b601581905595945050505050565b600080846124ec8b8d613639565b10156124fd5750600090508061253f565b6125078a86613692565b612511908c613692565b9050612520898989848a612d4c565b925061253290508b8a89888888612e3f565b61253c9083613639565b91505b995099975050505050505050565b600061255b85858585612b23565b905080601554146125ae5760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a524e543a434f4d4d49544d454e545f4d49534d41544348000000000060448201526064016106c5565b6000601555949350505050565b6000806000806125e242601254600754601454600e54601354600854600a54600b54612ee5565b60165491955093506125f49084613639565b92506017549150601854905090919293565b6000833b806126195760009150506122fb565b846001600160a01b0316848460405161263392919061350b565b600060405180830381855af49150503d806000811461266e576040519150601f19603f3d011682016040523d82523d6000602084013e612673565b606091505b50909695505050505050565b81600f60008282546126919190613692565b90915550506005546126ad906001600160a01b031682846122c5565b6126f25760405162461bcd60e51b81526020600482015260166024820152751353124e91118e9514905394d1915497d1905253115160521b60448201526064016106c5565b6126fa612cba565b6109c75760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a44463a494e53554646494349454e545f434f4c4c41544552414c000060448201526064016106c5565b6000806000806012544211156127975760405162461bcd60e51b81526020600482015260166024820152754d4c493a434c3a5041594d454e545f49535f4c41544560501b60448201526064016106c5565b61279f61244f565b60006016819055939750919550935091506127ba8486613639565b9050816127c78483613639565b6127d19190613639565b6005546127e6906001600160a01b03166121eb565b600f546127f39190613639565b6127fd9190613692565b600f8190555080601060008282546128159190613639565b9091555061282590508383612cda565b61282d612ae5565b5090919293565b81601160008282546128469190613692565b9091555050600454612862906001600160a01b031682846122c5565b6128a75760405162461bcd60e51b81526020600482015260166024820152751353124e9490ce9514905394d1915497d1905253115160521b60448201526064016106c5565b6128af612cba565b6109c75760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a52433a494e53554646494349454e545f434f4c4c41544552414c000060448201526064016106c5565b60006001600160a01b03821661294b5760405162461bcd60e51b815260206004820152601560248201527426a6249d23261d24a72b20a624a22fa622a72222a960591b60448201526064016106c5565b60135460125415801561295d57508015155b61299e5760405162461bcd60e51b81526020600482015260126024820152714d4c493a464c3a4c4f414e5f41435449564560701b60448201526064016106c5565b600754600180546001600160a01b0319166001600160a01b0386161790556129c68142613639565b601255600d5460148190556005549093506001600160a01b0316836129ea826121eb565b1015612a385760405162461bcd60e51b815260206004820152601860248201527f4d4c493a464c3a57524f4e475f46554e445f414d4f554e54000000000000000060448201526064016106c5565b505050600f819055919050565b60006001600160a01b0383163b612a5e575060006112e3565b6060836001600160a01b031683604051612a78919061351b565b6000604051808303816000865af19150503d8060008114612ab5576040519150601f19603f3d011682016040523d82523d6000602084013e612aba565b606091505b509092509050818015610ca3575080511580610ca3575080806020019051810190610ca391906133a5565b60006006819055600781905560088190556009819055600a819055600b819055600e8190556012819055601381905560148190556017819055601855565b600084848484604051602001612b3c9493929190613556565b604051602081830303815290604052805190602001209050949350505050565b6000612b66612f34565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612ba957600080fd5b505afa158015612bbd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612be191906133e0565b612beb9089613673565b612bf59190613673565b612bff9190613651565b612c099190613639565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b158015612c5057600080fd5b505afa158015612c64573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612c8891906133e0565b612c929089613673565b612c9c9190613673565b612ca69190613651565b612cb09190613639565b6018555050505050565b6000612cd0601454600f54600d54600c54611eb3565b6011541015905090565b600154612cf8906001600160a01b0316634046af2b60e01b84612fae565b612d14578160106000828254612d0e9190613639565b90915550505b612d2d612d1f612f34565b63a5a2760560e01b83612fae565b6109c7578060106000828254612d439190613639565b90915550505050565b6000806000612d5b86866130bd565b90506000612d83612d7483670de0b6b3a7640000613639565b86670de0b6b3a76400006130d8565b9050670de0b6b3a76400008111612db65784612d9f898b613692565b612da99190613651565b6000935093505050612e35565b6000612dca670de0b6b3a764000083613692565b838a670de0b6b3a7640000612ddf868f613673565b612de99190613651565b612df39190613692565b612dfd9190613673565b612e079190613651565b9050612e148a898961313a565b935083811015612e25576000612e2f565b612e2f8482613692565b94505050505b9550959350505050565b6000838711612e5057506000612edb565b6000620151806001612e62878b613692565b612e6c9190613692565b612e769190613651565b612e81906001613639565b612e8e9062015180613673565b9050612ea487612e9e8589613639565b8361313a565b612eae9083613639565b9150670de0b6b3a7640000612ec38886613673565b612ecd9190613651565b612ed79083613639565b9150505b9695505050505050565b600080612ef58888878c8a612d4c565b909250905060018614612f085781612f0a565b875b9150612f1a8b89878d8888612e3f565b612f249082613639565b9050995099975050505050505050565b6000612f3e611bef565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612f7657600080fd5b505afa158015612f8a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071f919061320e565b600081612fbd575060016122fb565b60408051600481526024810182526020810180516001600160e01b03166001600160e01b03198716179052905160009182916001600160a01b038816916130039161351b565b6000604051808303816000865af19150503d8060008114613040576040519150601f19603f3d011682016040523d82523d6000602084013e613045565b606091505b509150915081158061305957506020815114155b15613069576000925050506122fb565b60008180602001905181019061307f919061320e565b90506001600160a01b03811661309b57600093505050506122fb565b6005546130b2906001600160a01b031682876122c5565b979650505050505050565b60006301e133806130ce8385613673565b6122fb9190613651565b6000600183166130e857816130ea565b835b90505b60019290921c9182156122fb57816131058580613673565b61310f9190613651565b93506001831661311e576130ed565b816131298583613673565b6131339190613651565b90506130ed565b6000670de0b6b3a764000061314f84846130bd565b6131599086613673565b6122f89190613651565b60008083601f84011261317557600080fd5b50813567ffffffffffffffff81111561318d57600080fd5b6020830191508360208260051b85010111156131a857600080fd5b9250929050565b60008083601f8401126131c157600080fd5b50813567ffffffffffffffff8111156131d957600080fd5b6020830191508360208285010111156131a857600080fd5b60006020828403121561320357600080fd5b81356122fb816136d5565b60006020828403121561322057600080fd5b81516122fb816136d5565b6000806040838503121561323e57600080fd5b8235613249816136d5565b91506020830135613259816136d5565b809150509250929050565b60008060006040848603121561327957600080fd5b8335613284816136d5565b9250602084013567ffffffffffffffff8111156132a057600080fd5b6132ac868287016131af565b9497909650939450505050565b600080604083850312156132cc57600080fd5b82356132d7816136d5565b946020939093013593505050565b600080600080606085870312156132fb57600080fd5b8435613306816136d5565b935060208501359250604085013567ffffffffffffffff81111561332957600080fd5b61333587828801613163565b95989497509550505050565b60008060008060006080868803121561335957600080fd5b8535613364816136d5565b945060208601359350604086013567ffffffffffffffff81111561338757600080fd5b61339388828901613163565b96999598509660600135949350505050565b6000602082840312156133b757600080fd5b815180151581146122fb57600080fd5b6000602082840312156133d957600080fd5b5035919050565b6000602082840312156133f257600080fd5b5051919050565b6000806040838503121561340c57600080fd5b823591506020830135613259816136d5565b60008060006040848603121561343357600080fd5b83359250602084013567ffffffffffffffff8111156132a057600080fd5b81835260006020808501808196508560051b810191508460005b878110156134d55782840389528135601e1988360301811261348c57600080fd5b8701803567ffffffffffffffff8111156134a557600080fd5b8036038913156134b457600080fd5b6134c186828985016134e2565b9a87019a955050509084019060010161346b565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8183823760009101908152919050565b6000825160005b8181101561353c5760208186018101518583015201613522565b8181111561354b576000828501525b509190910192915050565b60018060a01b0385168152836020820152606060408201526000612edb606083018486613451565b85815260018060a01b03851660208201528360408201526080606082015260006130b2608083018486613451565b60208082526012908201527113530e941493d513d0d3d317d4105554d15160721b604082015260600190565b838152604060208201526000611cff6040830184866134e2565b6000808335601e1984360301811261360957600080fd5b83018035915067ffffffffffffffff82111561362457600080fd5b6020019150368190038213156131a857600080fd5b6000821982111561364c5761364c6136a9565b500190565b60008261366e57634e487b7160e01b600052601260045260246000fd5b500490565b600081600019048311821515161561368d5761368d6136a9565b500290565b6000828210156136a4576136a46136a9565b500390565b634e487b7160e01b600052601160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6001600160a01b038116811461195a57600080fdfea26469706673582212204c4014e9c973b4d4656bc1c3f69a5ae6fcd7f9219aae24dc2fbb4d460f28135b64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MapleLoan<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleLoan<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleLoan<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleLoan))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleLoan<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAPLELOAN_ABI.clone(), client).into()
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
                MAPLELOAN_ABI.clone(),
                MAPLELOAN_BYTECODE.clone().into(),
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, MapleLoanEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MapleLoan<M> {
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
    pub enum MapleLoanEvents {
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
    impl ethers::contract::EthLogDecode for MapleLoanEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::BorrowerAcceptedFilter(decoded));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::CollateralPostedFilter(decoded));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::CollateralRemovedFilter(decoded));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(MapleLoanEvents::EstablishmentFeesSetFilter(decoded));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(MapleLoanEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::NewTermsAcceptedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::NewTermsProposedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::NewTermsRejectedFilter(decoded));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(MapleLoanEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(MapleLoanEvents::PendingBorrowerSetFilter(decoded));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(MapleLoanEvents::PendingLenderSetFilter(decoded));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::SkimmedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(MapleLoanEvents::UpgradedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MapleLoanEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                MapleLoanEvents::CollateralPostedFilter(element) => element.fmt(f),
                MapleLoanEvents::CollateralRemovedFilter(element) => element.fmt(f),
                MapleLoanEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                MapleLoanEvents::FundedFilter(element) => element.fmt(f),
                MapleLoanEvents::FundsClaimedFilter(element) => element.fmt(f),
                MapleLoanEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                MapleLoanEvents::FundsRedirectedFilter(element) => element.fmt(f),
                MapleLoanEvents::FundsReturnedFilter(element) => element.fmt(f),
                MapleLoanEvents::InitializedFilter(element) => element.fmt(f),
                MapleLoanEvents::LenderAcceptedFilter(element) => element.fmt(f),
                MapleLoanEvents::LoanClosedFilter(element) => element.fmt(f),
                MapleLoanEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                MapleLoanEvents::NewTermsProposedFilter(element) => element.fmt(f),
                MapleLoanEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                MapleLoanEvents::PaymentMadeFilter(element) => element.fmt(f),
                MapleLoanEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                MapleLoanEvents::PendingLenderSetFilter(element) => element.fmt(f),
                MapleLoanEvents::RepossessedFilter(element) => element.fmt(f),
                MapleLoanEvents::SkimmedFilter(element) => element.fmt(f),
                MapleLoanEvents::UpgradedFilter(element) => element.fmt(f),
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
    pub enum MapleLoanCalls {
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
    impl ethers::core::abi::AbiDecode for MapleLoanCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::AcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <AcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::AcceptLender(decoded));
            }
            if let Ok(decoded) =
                <AcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::AcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <BorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Borrower(decoded));
            }
            if let Ok(decoded) =
                <ClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::ClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::ClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <CloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::CloseLoan(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Collateral(decoded));
            }
            if let Ok(decoded) =
                <CollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::CollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <CollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::CollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <DelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::DelegateFee(decoded));
            }
            if let Ok(decoded) =
                <DrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::DrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <DrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::DrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <EarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::EarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <EndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::EndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <ExcessCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::ExcessCollateral(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <FundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::FundLoan(decoded));
            }
            if let Ok(decoded) =
                <FundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::FundsAsset(decoded));
            }
            if let Ok(decoded) =
                <GetAdditionalCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MapleLoanCalls::GetAdditionalCollateralRequiredFor(decoded));
            }
            if let Ok(decoded) =
                <GetEarlyPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MapleLoanCalls::GetEarlyPaymentBreakdown(decoded));
            }
            if let Ok(decoded) =
                <GetNextPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::GetNextPaymentBreakdown(decoded));
            }
            if let Ok(decoded) =
                <GetRefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::GetRefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::InterestRate(decoded));
            }
            if let Ok(decoded) =
                <IsProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::IsProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <LateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::LateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <LateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::LateInterestPremium(decoded));
            }
            if let Ok(decoded) = <LenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Lender(decoded));
            }
            if let Ok(decoded) =
                <MakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::MakePayment(decoded));
            }
            if let Ok(decoded) =
                <MigrateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Migrate(decoded));
            }
            if let Ok(decoded) =
                <NextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::NextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <PaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <PaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <PendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <PendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PendingLender(decoded));
            }
            if let Ok(decoded) =
                <PostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PostCollateral(decoded));
            }
            if let Ok(decoded) =
                <PrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Principal(decoded));
            }
            if let Ok(decoded) =
                <PrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::PrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::ProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::RefinanceCommitment(decoded));
            }
            if let Ok(decoded) =
                <RefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::RefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <RejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::RejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <RepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Repossess(decoded));
            }
            if let Ok(decoded) =
                <ReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::ReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::SetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::SetPendingLender(decoded));
            }
            if let Ok(decoded) = <SkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MapleLoanCalls::Skim(decoded));
            }
            if let Ok(decoded) =
                <SuperFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::SuperFactory(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::TreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanCalls::Upgrade(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleLoanCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleLoanCalls::AcceptBorrower(element) => element.encode(),
                MapleLoanCalls::AcceptLender(element) => element.encode(),
                MapleLoanCalls::AcceptNewTerms(element) => element.encode(),
                MapleLoanCalls::Borrower(element) => element.encode(),
                MapleLoanCalls::ClaimFunds(element) => element.encode(),
                MapleLoanCalls::ClaimableFunds(element) => element.encode(),
                MapleLoanCalls::CloseLoan(element) => element.encode(),
                MapleLoanCalls::Collateral(element) => element.encode(),
                MapleLoanCalls::CollateralAsset(element) => element.encode(),
                MapleLoanCalls::CollateralRequired(element) => element.encode(),
                MapleLoanCalls::DelegateFee(element) => element.encode(),
                MapleLoanCalls::DrawableFunds(element) => element.encode(),
                MapleLoanCalls::DrawdownFunds(element) => element.encode(),
                MapleLoanCalls::EarlyFeeRate(element) => element.encode(),
                MapleLoanCalls::EndingPrincipal(element) => element.encode(),
                MapleLoanCalls::ExcessCollateral(element) => element.encode(),
                MapleLoanCalls::Factory(element) => element.encode(),
                MapleLoanCalls::FundLoan(element) => element.encode(),
                MapleLoanCalls::FundsAsset(element) => element.encode(),
                MapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => element.encode(),
                MapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.encode(),
                MapleLoanCalls::GetNextPaymentBreakdown(element) => element.encode(),
                MapleLoanCalls::GetRefinanceInterest(element) => element.encode(),
                MapleLoanCalls::GracePeriod(element) => element.encode(),
                MapleLoanCalls::Implementation(element) => element.encode(),
                MapleLoanCalls::InterestRate(element) => element.encode(),
                MapleLoanCalls::IsProtocolPaused(element) => element.encode(),
                MapleLoanCalls::LateFeeRate(element) => element.encode(),
                MapleLoanCalls::LateInterestPremium(element) => element.encode(),
                MapleLoanCalls::Lender(element) => element.encode(),
                MapleLoanCalls::MakePayment(element) => element.encode(),
                MapleLoanCalls::Migrate(element) => element.encode(),
                MapleLoanCalls::NextPaymentDueDate(element) => element.encode(),
                MapleLoanCalls::PaymentInterval(element) => element.encode(),
                MapleLoanCalls::PaymentsRemaining(element) => element.encode(),
                MapleLoanCalls::PendingBorrower(element) => element.encode(),
                MapleLoanCalls::PendingLender(element) => element.encode(),
                MapleLoanCalls::PostCollateral(element) => element.encode(),
                MapleLoanCalls::Principal(element) => element.encode(),
                MapleLoanCalls::PrincipalRequested(element) => element.encode(),
                MapleLoanCalls::ProposeNewTerms(element) => element.encode(),
                MapleLoanCalls::RefinanceCommitment(element) => element.encode(),
                MapleLoanCalls::RefinanceInterest(element) => element.encode(),
                MapleLoanCalls::RejectNewTerms(element) => element.encode(),
                MapleLoanCalls::RemoveCollateral(element) => element.encode(),
                MapleLoanCalls::Repossess(element) => element.encode(),
                MapleLoanCalls::ReturnFunds(element) => element.encode(),
                MapleLoanCalls::SetImplementation(element) => element.encode(),
                MapleLoanCalls::SetPendingBorrower(element) => element.encode(),
                MapleLoanCalls::SetPendingLender(element) => element.encode(),
                MapleLoanCalls::Skim(element) => element.encode(),
                MapleLoanCalls::SuperFactory(element) => element.encode(),
                MapleLoanCalls::TreasuryFee(element) => element.encode(),
                MapleLoanCalls::Upgrade(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleLoanCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanCalls::AcceptBorrower(element) => element.fmt(f),
                MapleLoanCalls::AcceptLender(element) => element.fmt(f),
                MapleLoanCalls::AcceptNewTerms(element) => element.fmt(f),
                MapleLoanCalls::Borrower(element) => element.fmt(f),
                MapleLoanCalls::ClaimFunds(element) => element.fmt(f),
                MapleLoanCalls::ClaimableFunds(element) => element.fmt(f),
                MapleLoanCalls::CloseLoan(element) => element.fmt(f),
                MapleLoanCalls::Collateral(element) => element.fmt(f),
                MapleLoanCalls::CollateralAsset(element) => element.fmt(f),
                MapleLoanCalls::CollateralRequired(element) => element.fmt(f),
                MapleLoanCalls::DelegateFee(element) => element.fmt(f),
                MapleLoanCalls::DrawableFunds(element) => element.fmt(f),
                MapleLoanCalls::DrawdownFunds(element) => element.fmt(f),
                MapleLoanCalls::EarlyFeeRate(element) => element.fmt(f),
                MapleLoanCalls::EndingPrincipal(element) => element.fmt(f),
                MapleLoanCalls::ExcessCollateral(element) => element.fmt(f),
                MapleLoanCalls::Factory(element) => element.fmt(f),
                MapleLoanCalls::FundLoan(element) => element.fmt(f),
                MapleLoanCalls::FundsAsset(element) => element.fmt(f),
                MapleLoanCalls::GetAdditionalCollateralRequiredFor(element) => element.fmt(f),
                MapleLoanCalls::GetEarlyPaymentBreakdown(element) => element.fmt(f),
                MapleLoanCalls::GetNextPaymentBreakdown(element) => element.fmt(f),
                MapleLoanCalls::GetRefinanceInterest(element) => element.fmt(f),
                MapleLoanCalls::GracePeriod(element) => element.fmt(f),
                MapleLoanCalls::Implementation(element) => element.fmt(f),
                MapleLoanCalls::InterestRate(element) => element.fmt(f),
                MapleLoanCalls::IsProtocolPaused(element) => element.fmt(f),
                MapleLoanCalls::LateFeeRate(element) => element.fmt(f),
                MapleLoanCalls::LateInterestPremium(element) => element.fmt(f),
                MapleLoanCalls::Lender(element) => element.fmt(f),
                MapleLoanCalls::MakePayment(element) => element.fmt(f),
                MapleLoanCalls::Migrate(element) => element.fmt(f),
                MapleLoanCalls::NextPaymentDueDate(element) => element.fmt(f),
                MapleLoanCalls::PaymentInterval(element) => element.fmt(f),
                MapleLoanCalls::PaymentsRemaining(element) => element.fmt(f),
                MapleLoanCalls::PendingBorrower(element) => element.fmt(f),
                MapleLoanCalls::PendingLender(element) => element.fmt(f),
                MapleLoanCalls::PostCollateral(element) => element.fmt(f),
                MapleLoanCalls::Principal(element) => element.fmt(f),
                MapleLoanCalls::PrincipalRequested(element) => element.fmt(f),
                MapleLoanCalls::ProposeNewTerms(element) => element.fmt(f),
                MapleLoanCalls::RefinanceCommitment(element) => element.fmt(f),
                MapleLoanCalls::RefinanceInterest(element) => element.fmt(f),
                MapleLoanCalls::RejectNewTerms(element) => element.fmt(f),
                MapleLoanCalls::RemoveCollateral(element) => element.fmt(f),
                MapleLoanCalls::Repossess(element) => element.fmt(f),
                MapleLoanCalls::ReturnFunds(element) => element.fmt(f),
                MapleLoanCalls::SetImplementation(element) => element.fmt(f),
                MapleLoanCalls::SetPendingBorrower(element) => element.fmt(f),
                MapleLoanCalls::SetPendingLender(element) => element.fmt(f),
                MapleLoanCalls::Skim(element) => element.fmt(f),
                MapleLoanCalls::SuperFactory(element) => element.fmt(f),
                MapleLoanCalls::TreasuryFee(element) => element.fmt(f),
                MapleLoanCalls::Upgrade(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptBorrowerCall> for MapleLoanCalls {
        fn from(var: AcceptBorrowerCall) -> Self {
            MapleLoanCalls::AcceptBorrower(var)
        }
    }
    impl ::std::convert::From<AcceptLenderCall> for MapleLoanCalls {
        fn from(var: AcceptLenderCall) -> Self {
            MapleLoanCalls::AcceptLender(var)
        }
    }
    impl ::std::convert::From<AcceptNewTermsCall> for MapleLoanCalls {
        fn from(var: AcceptNewTermsCall) -> Self {
            MapleLoanCalls::AcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<BorrowerCall> for MapleLoanCalls {
        fn from(var: BorrowerCall) -> Self {
            MapleLoanCalls::Borrower(var)
        }
    }
    impl ::std::convert::From<ClaimFundsCall> for MapleLoanCalls {
        fn from(var: ClaimFundsCall) -> Self {
            MapleLoanCalls::ClaimFunds(var)
        }
    }
    impl ::std::convert::From<ClaimableFundsCall> for MapleLoanCalls {
        fn from(var: ClaimableFundsCall) -> Self {
            MapleLoanCalls::ClaimableFunds(var)
        }
    }
    impl ::std::convert::From<CloseLoanCall> for MapleLoanCalls {
        fn from(var: CloseLoanCall) -> Self {
            MapleLoanCalls::CloseLoan(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for MapleLoanCalls {
        fn from(var: CollateralCall) -> Self {
            MapleLoanCalls::Collateral(var)
        }
    }
    impl ::std::convert::From<CollateralAssetCall> for MapleLoanCalls {
        fn from(var: CollateralAssetCall) -> Self {
            MapleLoanCalls::CollateralAsset(var)
        }
    }
    impl ::std::convert::From<CollateralRequiredCall> for MapleLoanCalls {
        fn from(var: CollateralRequiredCall) -> Self {
            MapleLoanCalls::CollateralRequired(var)
        }
    }
    impl ::std::convert::From<DelegateFeeCall> for MapleLoanCalls {
        fn from(var: DelegateFeeCall) -> Self {
            MapleLoanCalls::DelegateFee(var)
        }
    }
    impl ::std::convert::From<DrawableFundsCall> for MapleLoanCalls {
        fn from(var: DrawableFundsCall) -> Self {
            MapleLoanCalls::DrawableFunds(var)
        }
    }
    impl ::std::convert::From<DrawdownFundsCall> for MapleLoanCalls {
        fn from(var: DrawdownFundsCall) -> Self {
            MapleLoanCalls::DrawdownFunds(var)
        }
    }
    impl ::std::convert::From<EarlyFeeRateCall> for MapleLoanCalls {
        fn from(var: EarlyFeeRateCall) -> Self {
            MapleLoanCalls::EarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<EndingPrincipalCall> for MapleLoanCalls {
        fn from(var: EndingPrincipalCall) -> Self {
            MapleLoanCalls::EndingPrincipal(var)
        }
    }
    impl ::std::convert::From<ExcessCollateralCall> for MapleLoanCalls {
        fn from(var: ExcessCollateralCall) -> Self {
            MapleLoanCalls::ExcessCollateral(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for MapleLoanCalls {
        fn from(var: FactoryCall) -> Self {
            MapleLoanCalls::Factory(var)
        }
    }
    impl ::std::convert::From<FundLoanCall> for MapleLoanCalls {
        fn from(var: FundLoanCall) -> Self {
            MapleLoanCalls::FundLoan(var)
        }
    }
    impl ::std::convert::From<FundsAssetCall> for MapleLoanCalls {
        fn from(var: FundsAssetCall) -> Self {
            MapleLoanCalls::FundsAsset(var)
        }
    }
    impl ::std::convert::From<GetAdditionalCollateralRequiredForCall> for MapleLoanCalls {
        fn from(var: GetAdditionalCollateralRequiredForCall) -> Self {
            MapleLoanCalls::GetAdditionalCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetEarlyPaymentBreakdownCall> for MapleLoanCalls {
        fn from(var: GetEarlyPaymentBreakdownCall) -> Self {
            MapleLoanCalls::GetEarlyPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetNextPaymentBreakdownCall> for MapleLoanCalls {
        fn from(var: GetNextPaymentBreakdownCall) -> Self {
            MapleLoanCalls::GetNextPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetRefinanceInterestCall> for MapleLoanCalls {
        fn from(var: GetRefinanceInterestCall) -> Self {
            MapleLoanCalls::GetRefinanceInterest(var)
        }
    }
    impl ::std::convert::From<GracePeriodCall> for MapleLoanCalls {
        fn from(var: GracePeriodCall) -> Self {
            MapleLoanCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for MapleLoanCalls {
        fn from(var: ImplementationCall) -> Self {
            MapleLoanCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InterestRateCall> for MapleLoanCalls {
        fn from(var: InterestRateCall) -> Self {
            MapleLoanCalls::InterestRate(var)
        }
    }
    impl ::std::convert::From<IsProtocolPausedCall> for MapleLoanCalls {
        fn from(var: IsProtocolPausedCall) -> Self {
            MapleLoanCalls::IsProtocolPaused(var)
        }
    }
    impl ::std::convert::From<LateFeeRateCall> for MapleLoanCalls {
        fn from(var: LateFeeRateCall) -> Self {
            MapleLoanCalls::LateFeeRate(var)
        }
    }
    impl ::std::convert::From<LateInterestPremiumCall> for MapleLoanCalls {
        fn from(var: LateInterestPremiumCall) -> Self {
            MapleLoanCalls::LateInterestPremium(var)
        }
    }
    impl ::std::convert::From<LenderCall> for MapleLoanCalls {
        fn from(var: LenderCall) -> Self {
            MapleLoanCalls::Lender(var)
        }
    }
    impl ::std::convert::From<MakePaymentCall> for MapleLoanCalls {
        fn from(var: MakePaymentCall) -> Self {
            MapleLoanCalls::MakePayment(var)
        }
    }
    impl ::std::convert::From<MigrateCall> for MapleLoanCalls {
        fn from(var: MigrateCall) -> Self {
            MapleLoanCalls::Migrate(var)
        }
    }
    impl ::std::convert::From<NextPaymentDueDateCall> for MapleLoanCalls {
        fn from(var: NextPaymentDueDateCall) -> Self {
            MapleLoanCalls::NextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<PaymentIntervalCall> for MapleLoanCalls {
        fn from(var: PaymentIntervalCall) -> Self {
            MapleLoanCalls::PaymentInterval(var)
        }
    }
    impl ::std::convert::From<PaymentsRemainingCall> for MapleLoanCalls {
        fn from(var: PaymentsRemainingCall) -> Self {
            MapleLoanCalls::PaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<PendingBorrowerCall> for MapleLoanCalls {
        fn from(var: PendingBorrowerCall) -> Self {
            MapleLoanCalls::PendingBorrower(var)
        }
    }
    impl ::std::convert::From<PendingLenderCall> for MapleLoanCalls {
        fn from(var: PendingLenderCall) -> Self {
            MapleLoanCalls::PendingLender(var)
        }
    }
    impl ::std::convert::From<PostCollateralCall> for MapleLoanCalls {
        fn from(var: PostCollateralCall) -> Self {
            MapleLoanCalls::PostCollateral(var)
        }
    }
    impl ::std::convert::From<PrincipalCall> for MapleLoanCalls {
        fn from(var: PrincipalCall) -> Self {
            MapleLoanCalls::Principal(var)
        }
    }
    impl ::std::convert::From<PrincipalRequestedCall> for MapleLoanCalls {
        fn from(var: PrincipalRequestedCall) -> Self {
            MapleLoanCalls::PrincipalRequested(var)
        }
    }
    impl ::std::convert::From<ProposeNewTermsCall> for MapleLoanCalls {
        fn from(var: ProposeNewTermsCall) -> Self {
            MapleLoanCalls::ProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<RefinanceCommitmentCall> for MapleLoanCalls {
        fn from(var: RefinanceCommitmentCall) -> Self {
            MapleLoanCalls::RefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<RefinanceInterestCall> for MapleLoanCalls {
        fn from(var: RefinanceInterestCall) -> Self {
            MapleLoanCalls::RefinanceInterest(var)
        }
    }
    impl ::std::convert::From<RejectNewTermsCall> for MapleLoanCalls {
        fn from(var: RejectNewTermsCall) -> Self {
            MapleLoanCalls::RejectNewTerms(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for MapleLoanCalls {
        fn from(var: RemoveCollateralCall) -> Self {
            MapleLoanCalls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepossessCall> for MapleLoanCalls {
        fn from(var: RepossessCall) -> Self {
            MapleLoanCalls::Repossess(var)
        }
    }
    impl ::std::convert::From<ReturnFundsCall> for MapleLoanCalls {
        fn from(var: ReturnFundsCall) -> Self {
            MapleLoanCalls::ReturnFunds(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for MapleLoanCalls {
        fn from(var: SetImplementationCall) -> Self {
            MapleLoanCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<SetPendingBorrowerCall> for MapleLoanCalls {
        fn from(var: SetPendingBorrowerCall) -> Self {
            MapleLoanCalls::SetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<SetPendingLenderCall> for MapleLoanCalls {
        fn from(var: SetPendingLenderCall) -> Self {
            MapleLoanCalls::SetPendingLender(var)
        }
    }
    impl ::std::convert::From<SkimCall> for MapleLoanCalls {
        fn from(var: SkimCall) -> Self {
            MapleLoanCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SuperFactoryCall> for MapleLoanCalls {
        fn from(var: SuperFactoryCall) -> Self {
            MapleLoanCalls::SuperFactory(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for MapleLoanCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            MapleLoanCalls::TreasuryFee(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for MapleLoanCalls {
        fn from(var: UpgradeCall) -> Self {
            MapleLoanCalls::Upgrade(var)
        }
    }
}
