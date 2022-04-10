pub use mapleloaninternalsharness_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleloaninternalsharness_mod {
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
    #[doc = "MapleLoanInternalsHarness was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLELOANINTERNALSHARNESS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptNewTerms\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrower\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"clearLoanAccounting\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"collateralRequired\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"drawableFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drawdownFunds\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earlyFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"endingPrincipal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fundsAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getCollateralRequiredFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEarlyPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalPayments_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getInstallment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalAmount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestAmount_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"currentTime_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getPaymentBreakdown\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalAmount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestAmount_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getPeriodicInterestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"periodicInterestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRefinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUnaccountedAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"unaccountedAmount_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCollateralMaintained\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isMaintained_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateFeeRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lateInterestPremium\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextPaymentDueDate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentInterval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentsRemaining\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalRequested\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposeNewTerms\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceCommitment\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refinanceInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"base_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"exponent_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"one_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"scaledExponent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"claimableFunds_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setClaimableFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateral_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCollateralAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRequired_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCollateralRequired\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDelegateFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"drawableFunds_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDrawableFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"earlyFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEarlyFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"endingPrincipal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEndingPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFactory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fundsAsset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFundsAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gracePeriod_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGracePeriod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"interestRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInterestRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateFeeRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateFeeRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lateInterestPremium_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLateInterestPremium\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNextPaymentDueDate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentInterval_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentInterval\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentsRemaining_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentsRemaining\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPrincipal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalRequested_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPrincipalRequested\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRefinanceCommitment\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"refinanceInterest_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRefinanceInterest\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTreasuryFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLELOANINTERNALSHARNESS_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b506127bd806100206000396000f3fe608060405234801561001057600080fd5b50600436106104285760003560e01c80639931c34c1161022b578063c762d5f711610130578063e268255d116100b8578063eb7462c711610087578063eb7462c7146108f6578063f2f659601461090e578063f68dee9014610921578063fe12afe914610934578063ffb23a561461094757600080fd5b8063e268255d146108b5578063e44b3875146108c8578063e48f6faf146108d0578063e94134d9146108e357600080fd5b8063d157f645116100ff578063d157f6451461084f578063d41ddc9614610862578063d82d842114610875578063d8d79700146108a5578063d8dfeb45146108ad57600080fd5b8063c762d5f7146107f1578063c8b8c43e14610821578063cc32d17614610834578063ccc044841461083c57600080fd5b8063b86a513e116101b3578063ba5d307811610182578063ba5d3078146107aa578063ba83276b146107b2578063bcead63e146107ba578063bdef476b146107cb578063c58d45a9146107de57600080fd5b8063b86a513e1461077f578063b96b5c9914610787578063b9b1f4e31461078f578063b9d842d11461079757600080fd5b8063aabaecd6116101fa578063aabaecd61461072d578063ab8355d41461073e578063acb522b414610751578063b69410de14610764578063b75941921461076c57600080fd5b80639931c34c146106f75780639c3c2ab11461070a578063a06db7dc1461071d578063a97d11611461072557600080fd5b80634b65a86d1161033157806375a20676116102b95780637df1f1b9116102885780637df1f1b9146106b05780637febd92b146106c157806387accaf1146106d45780638c6862bd146106e75780638ffc9215146106ef57600080fd5b806375a206761461068557806377b3c55c1461068d57806377e741c7146106955780637c3a00fd146106a857600080fd5b80635de1858c116103005780635de1858c146106315780635f84f302146106445780636174b2721461065757806369458ba71461066a57806370a10c891461067257600080fd5b80634b65a86d146105a057806350b903a9146105b35780635260781c146105e35780635bb47808146105f657600080fd5b8063390d6855116103b457806341fc71061161038357806341fc71061461050f57806342af07501461052257806346e368d41461053557806347350e9f146105655780634764757e1461058d57600080fd5b8063390d6855146104bc57806339ba9f86146104cf5780633fabc6b7146104f45780634003f34d1461050757600080fd5b806321c3cbc2116103fb57806321c3cbc214610467578063232fa7331461046f578063258d0e8a146104975780632ead1098146104ac57806330fea1ce146104b457600080fd5b80630895326f1461042d5780631cc1cf46146104445780631df5b4d21461044c5780631eb5ea2e1461045f575b600080fd5b6013545b6040519081526020015b60405180910390f35b600754610431565b61043161045a36600461242d565b61095a565b610431610971565b610431610980565b61047761098a565b60408051948552602085019390935291830152606082015260800161043b565b6104aa6104a53660046123a9565b601255565b005b600b54610431565b600954610431565b6104aa6104ca3660046123db565b6109a6565b6005546001600160a01b03165b6040516001600160a01b03909116815260200161043b565b6104aa6105023660046123a9565b601155565b601254610431565b6104aa61051d3660046123a9565b601555565b6104316105303660046122fb565b6109b4565b6104aa610543366004612206565b600180546001600160a01b0319166001600160a01b0392909216919091179055565b610578610573366004612206565b6109cb565b6040805192835260208301919091520161043b565b6104aa61059b3660046123a9565b601355565b6104aa6105ae366004612240565b6109e0565b6104aa6105c1366004612206565b600580546001600160a01b0319166001600160a01b0392909216919091179055565b6105786105f13660046124c6565b6109f4565b6104aa610604366004612206565b6001600160a01b03167f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af155565b6104aa61063f3660046123a9565b600c55565b6104aa6106523660046123a9565b600855565b610431610665366004612206565b610a1b565b610477610a2c565b6104316106803660046122fb565b610a3a565b600c54610431565b600a54610431565b6104aa6106a33660046123a9565b601855565b600854610431565b6000546001600160a01b03166104dc565b6104aa6106cf3660046123a9565b600a55565b6104316106e23660046122fb565b610a48565b6104aa610a56565b600d54610431565b6104aa6107053660046123a9565b601755565b6104aa6107183660046123a9565b600b55565b600654610431565b601654610431565b6004546001600160a01b03166104dc565b6104aa61074c3660046123a9565b601655565b6104aa61075f3660046122fb565b610a60565b601754610431565b6104aa61077a3660046123a9565b600f55565b600e54610431565b610477610a6c565b600f54610431565b6104316107a536600461240b565b610a7a565b601454610431565b601554610431565b6001546001600160a01b03166104dc565b6104aa6107d93660046123a9565b601455565b6104316107ec36600461242d565b610a86565b6104aa6107ff366004612206565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b61057861082f36600461248b565b610a93565b601854610431565b6104aa61084a3660046123db565b610ab2565b6104aa61085d3660046123a9565b600755565b6104aa6108703660046123db565b610abc565b6104aa610883366004612206565b600480546001600160a01b0319166001600160a01b0392909216919091179055565b610477610ac6565b601154610431565b6104316108c3366004612459565b610ad4565b601054610431565b6104316108de366004612206565b610ae2565b6104aa6108f13660046123a9565b600e55565b6108fe610aed565b604051901515815260200161043b565b6104aa61091c3660046123a9565b600655565b6104aa61092f3660046123a9565b601055565b6104aa6109423660046123a9565b600955565b6104aa6109553660046123a9565b600d55565b6000610967848484610af7565b90505b9392505050565b600061097b610b20565b905090565b600061097b610b56565b600080600080610998610b82565b935093509350935090919293565b6109b08282610c75565b5050565b60006109c285858585610ce8565b95945050505050565b6000806109d783610d21565b91509150915091565b6109ed8585858585610e96565b5050505050565b600080610a088b8b8b8b8b8b8b8b8b61102b565b915091505b995099975050505050505050565b6000610a268261107a565b92915050565b600080600080610998611154565b60006109c2858585856111bb565b60006109c2858585856114ba565b610a5e6114e2565b565b6109ed84848484611520565b60008060008061099861158e565b600061096a83836115d9565b60006109678484846115f4565b600080610aa38787878787611656565b915091505b9550959350505050565b6109b08282611748565b6109b0828261180f565b6000806000806109986118d6565b60006109c2858585856119cd565b6000610a2682611a06565b600061097b611b50565b6000670de0b6b3a7640000610b0c84846115d9565b610b16908661270d565b61096791906126eb565b600554600090610b38906001600160a01b031661107a565b905080600f6000828254610b4c91906126d3565b9250508190555090565b600454600090610b6e906001600160a01b031661107a565b90508060116000828254610b4c91906126d3565b600080600080601254421115610bd85760405162461bcd60e51b81526020600482015260166024820152754d4c493a434c3a5041594d454e545f49535f4c41544560501b60448201526064015b60405180910390fd5b610be0611154565b6000601681905593975091955093509150610bfb84866126d3565b905081610c0884836126d3565b610c1291906126d3565b600554610c27906001600160a01b031661107a565b600f54610c3491906126d3565b610c3e919061272c565b600f819055508060106000828254610c5691906126d3565b90915550610c6690508383611b70565b610c6e6114e2565b5090919293565b8160106000828254610c87919061272c565b9091555050600554610ca3906001600160a01b03168284611be2565b6109b05760405162461bcd60e51b81526020600482015260166024820152751353124e90d18e9514905394d1915497d1905253115160521b6044820152606401610bcf565b600084848484604051602001610d019493929190612599565b604051602081830303815290604052805190602001209050949350505050565b60125460009081908015801590610d435750600654610d4090826126d3565b42115b610d865760405162461bcd60e51b81526020600482015260146024820152731353124e948e9393d517d25397d111519055531560621b6044820152606401610bcf565b610d8e6114e2565b600060118190556010819055600f8190556004546001600160a01b031690610db58261107a565b9450841480610dca5750610dca818686611be2565b610e165760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a435f5452414e534645525f4641494c45440000000000000000006044820152606401610bcf565b6005546001600160a01b03166000610e2d8261107a565b9450841480610e425750610e42818786611be2565b610e8e5760405162461bcd60e51b815260206004820152601760248201527f4d4c493a523a465f5452414e534645525f4641494c45440000000000000000006044820152606401610bcf565b505050915091565b6020820151610ee75760405162461bcd60e51b815260206004820152601760248201527f4d4c493a493a494e56414c49445f5052494e434950414c0000000000000000006044820152606401610bcf565b602082015160408301511115610f3f5760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a493a494e56414c49445f454e44494e475f5052494e434950414c00006044820152606401610bcf565b600080546001600160a01b0319166001600160a01b038716908117909155610fa25760405162461bcd60e51b815260206004820152601660248201527526a6249d249d24a72b20a624a22fa127a92927aba2a960511b6044820152606401610bcf565b8351600480546001600160a01b03199081166001600160a01b039384161790915560208087015160058054909316931692909217905583516006558381015160078190556040808601516013558451600c5584830151600d81905585820151600e55845160085592840151600955830151600a556060830151600b556109ed9190600080611c37565b60008061103b8888878c8a611656565b90925090506001861461104e5781611050565b875b91506110608b89878d8888611d95565b61106a90826126d3565b9050995099975050505050505050565b6005546000906001600160a01b038381169116146110995760006110a9565b600f546010546110a991906126d3565b6004546001600160a01b038481169116146110c55760006110c9565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561110857600080fd5b505afa15801561111c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061114091906123c2565b61114a919061272c565b610a26919061272c565b600080600080601654670de0b6b3a7640000600954601454965086611179919061270d565b61118391906126eb565b61118d91906126d3565b601354601754919450906111a290829061270d565b9250806018546111b2919061270d565b91505090919293565b60006111c985858585610ce8565b9050806015541461121c5760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a414e543a434f4d4d49544d454e545f4d49534d4154434800000000006044820152606401610bcf565b6001600160a01b0385163b6112735760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a494e56414c49445f524546494e414e4345520000000000006044820152606401610bcf565b834211156112c35760405162461bcd60e51b815260206004820152601a60248201527f4d4c493a414e543a455850495245445f434f4d4d49544d454e540000000000006044820152606401610bcf565b600060075490506000806112ec4284601454600e54600854601354601254600a54600b54611e3b565b91509150816016600082825461130291906126d3565b90915550506000601581905585905b818110156113e25760008a6001600160a01b031689898481811061133757611337612759565b9050602002810190611349919061264e565b60405161135792919061254e565b600060405180830381855af49150503d8060008114611392576040519150601f19603f3d011682016040523d82523d6000602084013e611397565b606091505b50509050806113d95760405162461bcd60e51b815260206004820152600e60248201526d1353124e9053950e91905253115160921b6044820152606401610bcf565b50600101611311565b506000601354856113f3919061270d565b83601754611401919061270d565b61140b91906126eb565b905060006013548661141d919061270d565b8460185461142b919061270d565b61143591906126eb565b60075490915061144581426126d3565b60125560145461145790828585611c37565b61145f611b50565b6114ab5760405162461bcd60e51b815260206004820152601f60248201527f4d4c493a414e543a494e53554646494349454e545f434f4c4c41544552414c006044820152606401610bcf565b50505050505050949350505050565b6000816114c85760006114d4565b6114d485858585610ce8565b601581905595945050505050565b60006006819055600781905560088190556009819055600a819055600b819055600e8190556012819055601381905560148190556017819055601855565b600061152e85858585610ce8565b905080601554146115815760405162461bcd60e51b815260206004820152601b60248201527f4d4c493a524e543a434f4d4d49544d454e545f4d49534d4154434800000000006044820152606401610bcf565b6000601555949350505050565b6000806000806115b542601254600754601454600e54601354600854600a54600b5461102b565b60165491955093506115c790846126d3565b92506017549150601854905090919293565b60006301e133806115ea838561270d565b61096a91906126eb565b6000600183166116045781611606565b835b90505b60019290921c91821561096a5781611621858061270d565b61162b91906126eb565b93506001831661163a57611609565b81611645858361270d565b61164f91906126eb565b9050611609565b600080600061166586866115d9565b9050600061168d61167e83670de0b6b3a76400006126d3565b86670de0b6b3a76400006115f4565b9050670de0b6b3a764000081116116c057846116a9898b61272c565b6116b391906126eb565b6000935093505050610aa8565b60006116d4670de0b6b3a76400008361272c565b838a670de0b6b3a76400006116e9868f61270d565b6116f391906126eb565b6116fd919061272c565b611707919061270d565b61171191906126eb565b905061171e8a8989610af7565b93508381101561172f576000611739565b611739848261272c565b94505050509550959350505050565b81600f600082825461175a919061272c565b9091555050600554611776906001600160a01b03168284611be2565b6117bb5760405162461bcd60e51b81526020600482015260166024820152751353124e91118e9514905394d1915497d1905253115160521b6044820152606401610bcf565b6117c3611b50565b6109b05760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a44463a494e53554646494349454e545f434f4c4c41544552414c00006044820152606401610bcf565b8160116000828254611821919061272c565b909155505060045461183d906001600160a01b03168284611be2565b6118825760405162461bcd60e51b81526020600482015260166024820152751353124e9490ce9514905394d1915497d1905253115160521b6044820152606401610bcf565b61188a611b50565b6109b05760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a52433a494e53554646494349454e545f434f4c4c41544552414c00006044820152606401610bcf565b6000806000806118e461158e565b60006016819055939750919550935091506118ff84866126d3565b90508161190c84836126d3565b61191691906126d3565b60055461192b906001600160a01b031661107a565b600f5461193891906126d3565b611942919061272c565b600f81905550806010600082825461195a91906126d3565b9091555061196a90508383611b70565b60135460018114156119835761197e6114e2565b6119c5565b6007546012600082825461199791906126d3565b9250508190555085601460008282546119b0919061272c565b909155506119c1905060018261272c565b6013555b505090919293565b6000838511156119fb57826119e2858761272c565b6119ec908461270d565b6119f691906126eb565b6109c2565b506000949350505050565b60006001600160a01b038216611a565760405162461bcd60e51b815260206004820152601560248201527426a6249d23261d24a72b20a624a22fa622a72222a960591b6044820152606401610bcf565b601354601254158015611a6857508015155b611aa95760405162461bcd60e51b81526020600482015260126024820152714d4c493a464c3a4c4f414e5f41435449564560701b6044820152606401610bcf565b600754600180546001600160a01b0319166001600160a01b038616179055611ad181426126d3565b601255600d5460148190556005549093506001600160a01b031683611af58261107a565b1015611b435760405162461bcd60e51b815260206004820152601860248201527f4d4c493a464c3a57524f4e475f46554e445f414d4f554e5400000000000000006044820152606401610bcf565b505050600f819055919050565b6000611b66601454600f54600d54600c546119cd565b6011541015905090565b600154611b8e906001600160a01b0316634046af2b60e01b84611ea9565b611baa578160106000828254611ba491906126d3565b90915550505b611bc3611bb5611fb8565b63a5a2760560e01b83611ea9565b6109b0578060106000828254611bd991906126d3565b90915550505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052600090610967908590612032565b6000611c41611fb8565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b158015611c8457600080fd5b505afa158015611c98573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611cbc91906123c2565b611cc6908961270d565b611cd0919061270d565b611cda91906126eb565b611ce491906126d3565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b158015611d2b57600080fd5b505afa158015611d3f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611d6391906123c2565b611d6d908961270d565b611d77919061270d565b611d8191906126eb565b611d8b91906126d3565b6018555050505050565b6000838711611da657506000611e31565b6000620151806001611db8878b61272c565b611dc2919061272c565b611dcc91906126eb565b611dd79060016126d3565b611de4906201518061270d565b9050611dfa87611df485896126d3565b83610af7565b611e0490836126d3565b9150670de0b6b3a7640000611e19888661270d565b611e2391906126eb565b611e2d90836126d3565b9150505b9695505050505050565b60008084611e498b8d6126d3565b1015611e5a57506000905080610a0d565b611e648a8661272c565b611e6e908c61272c565b9050611e7d898989848a611656565b9250611e8f90508b8a89888888611d95565b611e9990836126d3565b9150995099975050505050505050565b600081611eb85750600161096a565b60408051600481526024810182526020810180516001600160e01b03166001600160e01b03198716179052905160009182916001600160a01b03881691611efe9161255e565b6000604051808303816000865af19150503d8060008114611f3b576040519150601f19603f3d011682016040523d82523d6000602084013e611f40565b606091505b5091509150811580611f5457506020815114155b15611f645760009250505061096a565b600081806020019051810190611f7a9190612223565b90506001600160a01b038116611f96576000935050505061096a565b600554611fad906001600160a01b03168287611be2565b979650505050505050565b6000611fc26120da565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b158015611ffa57600080fd5b505afa15801561200e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061097b9190612223565b60006001600160a01b0383163b61204b57506000610a26565b6060836001600160a01b031683604051612065919061255e565b6000604051808303816000865af19150503d80600081146120a2576040519150601f19603f3d011682016040523d82523d6000602084013e6120a7565b606091505b5090925090508180156120d25750805115806120d25750808060200190518101906120d29190612387565b949350505050565b60006121047f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15490565b919050565b600082601f83011261211a57600080fd5b6040516060810181811067ffffffffffffffff8211171561214b57634e487b7160e01b600052604160045260246000fd5b60405280836060810186101561216057600080fd5b60005b6003811015612182578135835260209283019290910190600101612163565b509195945050505050565b600082601f83011261219e57600080fd5b6040516080810181811067ffffffffffffffff821117156121cf57634e487b7160e01b600052604160045260246000fd5b6040528083608081018610156121e457600080fd5b60005b60048110156121825781358352602092830192909101906001016121e7565b60006020828403121561221857600080fd5b813561096a8161276f565b60006020828403121561223557600080fd5b815161096a8161276f565b60008060008060006101a0868803121561225957600080fd5b85356122648161276f565b94506020603f8701881361227757600080fd5b61227f61269c565b8082890160608a018b81111561229457600080fd5b60005b60028110156122bd5782356122ab8161276f565b85529385019391850191600101612297565b508298506122cb8c82612109565b975050505050506122df8760c08801612109565b91506122ef87610120880161218d565b90509295509295909350565b6000806000806060858703121561231157600080fd5b843561231c8161276f565b935060208501359250604085013567ffffffffffffffff8082111561234057600080fd5b818701915087601f83011261235457600080fd5b81358181111561236357600080fd5b8860208260051b850101111561237857600080fd5b95989497505060200194505050565b60006020828403121561239957600080fd5b8151801515811461096a57600080fd5b6000602082840312156123bb57600080fd5b5035919050565b6000602082840312156123d457600080fd5b5051919050565b600080604083850312156123ee57600080fd5b8235915060208301356124008161276f565b809150509250929050565b6000806040838503121561241e57600080fd5b50508035926020909101359150565b60008060006060848603121561244257600080fd5b505081359360208301359350604090920135919050565b6000806000806080858703121561246f57600080fd5b5050823594602084013594506040840135936060013592509050565b600080600080600060a086880312156124a357600080fd5b505083359560208501359550604085013594606081013594506080013592509050565b60008060008060008060008060006101208a8c0312156124e557600080fd5b505087359960208901359950604089013598606081013598506080810135975060a0810135965060c0810135955060e08101359450610100013592509050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b8183823760009101908152919050565b6000825160005b8181101561257f5760208186018101518583015201612565565b8181111561258e576000828501525b509190910192915050565b6001600160a01b0385168152602080820185905260606040830181905282018390526000906080600585901b840181019190840186845b8781101561263f57868503607f190183528135368a9003601e190181126125f657600080fd5b8901803567ffffffffffffffff81111561260f57600080fd5b8036038b131561261e57600080fd5b61262b8782888501612525565b9650505091830191908301906001016125d0565b50929998505050505050505050565b6000808335601e1984360301811261266557600080fd5b83018035915067ffffffffffffffff82111561268057600080fd5b60200191503681900382131561269557600080fd5b9250929050565b6040805190810167ffffffffffffffff811182821017156126cd57634e487b7160e01b600052604160045260246000fd5b60405290565b600082198211156126e6576126e6612743565b500190565b60008261270857634e487b7160e01b600052601260045260246000fd5b500490565b600081600019048311821515161561272757612727612743565b500290565b60008282101561273e5761273e612743565b500390565b634e487b7160e01b600052601160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6001600160a01b038116811461278457600080fd5b5056fea26469706673582212208abdaa82dec3ac41eb2afe5e1426d93e21b83d062cc6e2030d3c0980c690cb1464736f6c63430008070033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct MapleLoanInternalsHarness<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleLoanInternalsHarness<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleLoanInternalsHarness<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleLoanInternalsHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleLoanInternalsHarness<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MAPLELOANINTERNALSHARNESS_ABI.clone(),
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
                MAPLELOANINTERNALSHARNESS_ABI.clone(),
                MAPLELOANINTERNALSHARNESS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `acceptNewTerms` (0x70a10c89) function"]
        pub fn accept_new_terms(
            &self,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([112, 161, 12, 137], (refinancer, deadline, calls))
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
        #[doc = "Calls the contract's `clearLoanAccounting` (0x8c6862bd) function"]
        pub fn clear_loan_accounting(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 104, 98, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeLoan` (0x232fa733) function"]
        pub fn close_loan(
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
                .method_hash([35, 47, 167, 51], ())
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
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Calls the contract's `fundLoan` (0xe48f6faf) function"]
        pub fn fund_loan(
            &self,
            lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([228, 143, 111, 175], lender)
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
        #[doc = "Calls the contract's `getInstallment` (0xc8b8c43e) function"]
        pub fn get_installment(
            &self,
            principal: ethers::core::types::U256,
            ending_principal: ethers::core::types::U256,
            interest_rate: ethers::core::types::U256,
            payment_interval: ethers::core::types::U256,
            total_payments: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [200, 184, 196, 62],
                    (
                        principal,
                        ending_principal,
                        interest_rate,
                        payment_interval,
                        total_payments,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInterest` (0x1df5b4d2) function"]
        pub fn get_interest(
            &self,
            principal: ethers::core::types::U256,
            interest_rate: ethers::core::types::U256,
            interval: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 245, 180, 210], (principal, interest_rate, interval))
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
        #[doc = "Calls the contract's `getPaymentBreakdown` (0x5260781c) function"]
        pub fn get_payment_breakdown(
            &self,
            current_time: ethers::core::types::U256,
            next_payment_due_date: ethers::core::types::U256,
            payment_interval: ethers::core::types::U256,
            principal: ethers::core::types::U256,
            ending_principal: ethers::core::types::U256,
            payments_remaining: ethers::core::types::U256,
            interest_rate: ethers::core::types::U256,
            late_fee_rate: ethers::core::types::U256,
            late_interest_premium: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [82, 96, 120, 28],
                    (
                        current_time,
                        next_payment_due_date,
                        payment_interval,
                        principal,
                        ending_principal,
                        payments_remaining,
                        interest_rate,
                        late_fee_rate,
                        late_interest_premium,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPeriodicInterestRate` (0xb9d842d1) function"]
        pub fn get_periodic_interest_rate(
            &self,
            interest_rate: ethers::core::types::U256,
            interval: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([185, 216, 66, 209], (interest_rate, interval))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRefinanceCommitment` (0x42af0750) function"]
        pub fn get_refinance_commitment(
            &self,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([66, 175, 7, 80], (refinancer, deadline, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUnaccountedAmount` (0x6174b272) function"]
        pub fn get_unaccounted_amount(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([97, 116, 178, 114], asset)
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
        #[doc = "Calls the contract's `initialize` (0x4b65a86d) function"]
        pub fn initialize(
            &self,
            borrower: ethers::core::types::Address,
            assets: [ethers::core::types::Address; 2usize],
            term_details: [ethers::core::types::U256; 3usize],
            amounts: [ethers::core::types::U256; 3usize],
            rates: [ethers::core::types::U256; 4usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [75, 101, 168, 109],
                    (borrower, assets, term_details, amounts, rates),
                )
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
        #[doc = "Calls the contract's `isCollateralMaintained` (0xeb7462c7) function"]
        pub fn is_collateral_maintained(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 116, 98, 199], ())
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
        #[doc = "Calls the contract's `makePayment` (0xd8d79700) function"]
        pub fn make_payment(
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
                .method_hash([216, 215, 151, 0], ())
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
        #[doc = "Calls the contract's `postCollateral` (0x21c3cbc2) function"]
        pub fn post_collateral(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([33, 195, 203, 194], ())
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
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
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
        #[doc = "Calls the contract's `returnFunds` (0x1eb5ea2e) function"]
        pub fn return_funds(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([30, 181, 234, 46], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledExponent` (0xc58d45a9) function"]
        pub fn scaled_exponent(
            &self,
            base: ethers::core::types::U256,
            exponent: ethers::core::types::U256,
            one: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 141, 69, 169], (base, exponent, one))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrower` (0xc762d5f7) function"]
        pub fn set_borrower(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 98, 213, 247], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setClaimableFunds` (0xf68dee90) function"]
        pub fn set_claimable_funds(
            &self,
            claimable_funds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 141, 238, 144], claimable_funds)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCollateral` (0x3fabc6b7) function"]
        pub fn set_collateral(
            &self,
            collateral: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 171, 198, 183], collateral)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCollateralAsset` (0xd82d8421) function"]
        pub fn set_collateral_asset(
            &self,
            collateral_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 45, 132, 33], collateral_asset)
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
        #[doc = "Calls the contract's `setDelegateFee` (0x9931c34c) function"]
        pub fn set_delegate_fee(
            &self,
            delegate_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 49, 195, 76], delegate_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDrawableFunds` (0xb7594192) function"]
        pub fn set_drawable_funds(
            &self,
            drawable_funds: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 89, 65, 146], drawable_funds)
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
        #[doc = "Calls the contract's `setFactory` (0x5bb47808) function"]
        pub fn set_factory(
            &self,
            factory: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 180, 120, 8], factory)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFundsAsset` (0x50b903a9) function"]
        pub fn set_funds_asset(
            &self,
            funds_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 185, 3, 169], funds_asset)
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
        #[doc = "Calls the contract's `setLender` (0x46e368d4) function"]
        pub fn set_lender(
            &self,
            lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 227, 104, 212], lender)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNextPaymentDueDate` (0x258d0e8a) function"]
        pub fn set_next_payment_due_date(
            &self,
            next_payment_due_date: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 141, 14, 138], next_payment_due_date)
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
        #[doc = "Calls the contract's `setPrincipal` (0xbdef476b) function"]
        pub fn set_principal(
            &self,
            principal: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 239, 71, 107], principal)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPrincipalRequested` (0xffb23a56) function"]
        pub fn set_principal_requested(
            &self,
            principal_requested: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 178, 58, 86], principal_requested)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRefinanceCommitment` (0x41fc7106) function"]
        pub fn set_refinance_commitment(
            &self,
            refinance_commitment: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 252, 113, 6], refinance_commitment)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRefinanceInterest` (0xab8355d4) function"]
        pub fn set_refinance_interest(
            &self,
            refinance_interest: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 131, 85, 212], refinance_interest)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTreasuryFee` (0x77e741c7) function"]
        pub fn set_treasury_fee(
            &self,
            treasury_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 231, 65, 199], treasury_fee)
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MapleLoanInternalsHarness<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `acceptNewTerms`function with signature `acceptNewTerms(address,uint256,bytes[])` and selector `[112, 161, 12, 137]`"]
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
        abi = "acceptNewTerms(address,uint256,bytes[])"
    )]
    pub struct AcceptNewTermsCall {
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
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
    #[doc = "Container type for all input parameters for the `clearLoanAccounting`function with signature `clearLoanAccounting()` and selector `[140, 104, 98, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "clearLoanAccounting", abi = "clearLoanAccounting()")]
    pub struct ClearLoanAccountingCall;
    #[doc = "Container type for all input parameters for the `closeLoan`function with signature `closeLoan()` and selector `[35, 47, 167, 51]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "closeLoan", abi = "closeLoan()")]
    pub struct CloseLoanCall;
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
    #[doc = "Container type for all input parameters for the `fundLoan`function with signature `fundLoan(address)` and selector `[228, 143, 111, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fundLoan", abi = "fundLoan(address)")]
    pub struct FundLoanCall {
        pub lender: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `getInstallment`function with signature `getInstallment(uint256,uint256,uint256,uint256,uint256)` and selector `[200, 184, 196, 62]`"]
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
        name = "getInstallment",
        abi = "getInstallment(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct GetInstallmentCall {
        pub principal: ethers::core::types::U256,
        pub ending_principal: ethers::core::types::U256,
        pub interest_rate: ethers::core::types::U256,
        pub payment_interval: ethers::core::types::U256,
        pub total_payments: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getInterest`function with signature `getInterest(uint256,uint256,uint256)` and selector `[29, 245, 180, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getInterest", abi = "getInterest(uint256,uint256,uint256)")]
    pub struct GetInterestCall {
        pub principal: ethers::core::types::U256,
        pub interest_rate: ethers::core::types::U256,
        pub interval: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `getPaymentBreakdown`function with signature `getPaymentBreakdown(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)` and selector `[82, 96, 120, 28]`"]
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
        name = "getPaymentBreakdown",
        abi = "getPaymentBreakdown(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct GetPaymentBreakdownCall {
        pub current_time: ethers::core::types::U256,
        pub next_payment_due_date: ethers::core::types::U256,
        pub payment_interval: ethers::core::types::U256,
        pub principal: ethers::core::types::U256,
        pub ending_principal: ethers::core::types::U256,
        pub payments_remaining: ethers::core::types::U256,
        pub interest_rate: ethers::core::types::U256,
        pub late_fee_rate: ethers::core::types::U256,
        pub late_interest_premium: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPeriodicInterestRate`function with signature `getPeriodicInterestRate(uint256,uint256)` and selector `[185, 216, 66, 209]`"]
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
        name = "getPeriodicInterestRate",
        abi = "getPeriodicInterestRate(uint256,uint256)"
    )]
    pub struct GetPeriodicInterestRateCall {
        pub interest_rate: ethers::core::types::U256,
        pub interval: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRefinanceCommitment`function with signature `getRefinanceCommitment(address,uint256,bytes[])` and selector `[66, 175, 7, 80]`"]
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
        name = "getRefinanceCommitment",
        abi = "getRefinanceCommitment(address,uint256,bytes[])"
    )]
    pub struct GetRefinanceCommitmentCall {
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `getUnaccountedAmount`function with signature `getUnaccountedAmount(address)` and selector `[97, 116, 178, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUnaccountedAmount", abi = "getUnaccountedAmount(address)")]
    pub struct GetUnaccountedAmountCall {
        pub asset: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address[2],uint256[3],uint256[3],uint256[4])` and selector `[75, 101, 168, 109]`"]
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
        name = "initialize",
        abi = "initialize(address,address[2],uint256[3],uint256[3],uint256[4])"
    )]
    pub struct InitializeCall {
        pub borrower: ethers::core::types::Address,
        pub assets: [ethers::core::types::Address; 2usize],
        pub term_details: [ethers::core::types::U256; 3usize],
        pub amounts: [ethers::core::types::U256; 3usize],
        pub rates: [ethers::core::types::U256; 4usize],
    }
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
    #[doc = "Container type for all input parameters for the `isCollateralMaintained`function with signature `isCollateralMaintained()` and selector `[235, 116, 98, 199]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCollateralMaintained", abi = "isCollateralMaintained()")]
    pub struct IsCollateralMaintainedCall;
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
    #[doc = "Container type for all input parameters for the `makePayment`function with signature `makePayment()` and selector `[216, 215, 151, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "makePayment", abi = "makePayment()")]
    pub struct MakePaymentCall;
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
    #[doc = "Container type for all input parameters for the `postCollateral`function with signature `postCollateral()` and selector `[33, 195, 203, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "postCollateral", abi = "postCollateral()")]
    pub struct PostCollateralCall;
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
    #[doc = "Container type for all input parameters for the `returnFunds`function with signature `returnFunds()` and selector `[30, 181, 234, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "returnFunds", abi = "returnFunds()")]
    pub struct ReturnFundsCall;
    #[doc = "Container type for all input parameters for the `scaledExponent`function with signature `scaledExponent(uint256,uint256,uint256)` and selector `[197, 141, 69, 169]`"]
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
        name = "scaledExponent",
        abi = "scaledExponent(uint256,uint256,uint256)"
    )]
    pub struct ScaledExponentCall {
        pub base: ethers::core::types::U256,
        pub exponent: ethers::core::types::U256,
        pub one: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setBorrower`function with signature `setBorrower(address)` and selector `[199, 98, 213, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBorrower", abi = "setBorrower(address)")]
    pub struct SetBorrowerCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setClaimableFunds`function with signature `setClaimableFunds(uint256)` and selector `[246, 141, 238, 144]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setClaimableFunds", abi = "setClaimableFunds(uint256)")]
    pub struct SetClaimableFundsCall {
        pub claimable_funds: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setCollateral`function with signature `setCollateral(uint256)` and selector `[63, 171, 198, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setCollateral", abi = "setCollateral(uint256)")]
    pub struct SetCollateralCall {
        pub collateral: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setCollateralAsset`function with signature `setCollateralAsset(address)` and selector `[216, 45, 132, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setCollateralAsset", abi = "setCollateralAsset(address)")]
    pub struct SetCollateralAssetCall {
        pub collateral_asset: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `setDelegateFee`function with signature `setDelegateFee(uint256)` and selector `[153, 49, 195, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDelegateFee", abi = "setDelegateFee(uint256)")]
    pub struct SetDelegateFeeCall {
        pub delegate_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setDrawableFunds`function with signature `setDrawableFunds(uint256)` and selector `[183, 89, 65, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDrawableFunds", abi = "setDrawableFunds(uint256)")]
    pub struct SetDrawableFundsCall {
        pub drawable_funds: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setFactory`function with signature `setFactory(address)` and selector `[91, 180, 120, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFactory", abi = "setFactory(address)")]
    pub struct SetFactoryCall {
        pub factory: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFundsAsset`function with signature `setFundsAsset(address)` and selector `[80, 185, 3, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFundsAsset", abi = "setFundsAsset(address)")]
    pub struct SetFundsAssetCall {
        pub funds_asset: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `setLender`function with signature `setLender(address)` and selector `[70, 227, 104, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLender", abi = "setLender(address)")]
    pub struct SetLenderCall {
        pub lender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setNextPaymentDueDate`function with signature `setNextPaymentDueDate(uint256)` and selector `[37, 141, 14, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setNextPaymentDueDate", abi = "setNextPaymentDueDate(uint256)")]
    pub struct SetNextPaymentDueDateCall {
        pub next_payment_due_date: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setPrincipal`function with signature `setPrincipal(uint256)` and selector `[189, 239, 71, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPrincipal", abi = "setPrincipal(uint256)")]
    pub struct SetPrincipalCall {
        pub principal: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPrincipalRequested`function with signature `setPrincipalRequested(uint256)` and selector `[255, 178, 58, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPrincipalRequested", abi = "setPrincipalRequested(uint256)")]
    pub struct SetPrincipalRequestedCall {
        pub principal_requested: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setRefinanceCommitment`function with signature `setRefinanceCommitment(bytes32)` and selector `[65, 252, 113, 6]`"]
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
        name = "setRefinanceCommitment",
        abi = "setRefinanceCommitment(bytes32)"
    )]
    pub struct SetRefinanceCommitmentCall {
        pub refinance_commitment: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setRefinanceInterest`function with signature `setRefinanceInterest(uint256)` and selector `[171, 131, 85, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setRefinanceInterest", abi = "setRefinanceInterest(uint256)")]
    pub struct SetRefinanceInterestCall {
        pub refinance_interest: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTreasuryFee`function with signature `setTreasuryFee(uint256)` and selector `[119, 231, 65, 199]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTreasuryFee", abi = "setTreasuryFee(uint256)")]
    pub struct SetTreasuryFeeCall {
        pub treasury_fee: ethers::core::types::U256,
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleLoanInternalsHarnessCalls {
        AcceptNewTerms(AcceptNewTermsCall),
        Borrower(BorrowerCall),
        ClaimFunds(ClaimFundsCall),
        ClaimableFunds(ClaimableFundsCall),
        ClearLoanAccounting(ClearLoanAccountingCall),
        CloseLoan(CloseLoanCall),
        Collateral(CollateralCall),
        CollateralAsset(CollateralAssetCall),
        CollateralRequired(CollateralRequiredCall),
        DelegateFee(DelegateFeeCall),
        DrawableFunds(DrawableFundsCall),
        DrawdownFunds(DrawdownFundsCall),
        EarlyFeeRate(EarlyFeeRateCall),
        EndingPrincipal(EndingPrincipalCall),
        FundLoan(FundLoanCall),
        FundsAsset(FundsAssetCall),
        GetCollateralRequiredFor(GetCollateralRequiredForCall),
        GetEarlyPaymentBreakdown(GetEarlyPaymentBreakdownCall),
        GetInstallment(GetInstallmentCall),
        GetInterest(GetInterestCall),
        GetNextPaymentBreakdown(GetNextPaymentBreakdownCall),
        GetPaymentBreakdown(GetPaymentBreakdownCall),
        GetPeriodicInterestRate(GetPeriodicInterestRateCall),
        GetRefinanceCommitment(GetRefinanceCommitmentCall),
        GetUnaccountedAmount(GetUnaccountedAmountCall),
        GracePeriod(GracePeriodCall),
        Initialize(InitializeCall),
        InterestRate(InterestRateCall),
        IsCollateralMaintained(IsCollateralMaintainedCall),
        LateFeeRate(LateFeeRateCall),
        LateInterestPremium(LateInterestPremiumCall),
        Lender(LenderCall),
        MakePayment(MakePaymentCall),
        NextPaymentDueDate(NextPaymentDueDateCall),
        PaymentInterval(PaymentIntervalCall),
        PaymentsRemaining(PaymentsRemainingCall),
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
        ScaledExponent(ScaledExponentCall),
        SetBorrower(SetBorrowerCall),
        SetClaimableFunds(SetClaimableFundsCall),
        SetCollateral(SetCollateralCall),
        SetCollateralAsset(SetCollateralAssetCall),
        SetCollateralRequired(SetCollateralRequiredCall),
        SetDelegateFee(SetDelegateFeeCall),
        SetDrawableFunds(SetDrawableFundsCall),
        SetEarlyFeeRate(SetEarlyFeeRateCall),
        SetEndingPrincipal(SetEndingPrincipalCall),
        SetFactory(SetFactoryCall),
        SetFundsAsset(SetFundsAssetCall),
        SetGracePeriod(SetGracePeriodCall),
        SetInterestRate(SetInterestRateCall),
        SetLateFeeRate(SetLateFeeRateCall),
        SetLateInterestPremium(SetLateInterestPremiumCall),
        SetLender(SetLenderCall),
        SetNextPaymentDueDate(SetNextPaymentDueDateCall),
        SetPaymentInterval(SetPaymentIntervalCall),
        SetPaymentsRemaining(SetPaymentsRemainingCall),
        SetPrincipal(SetPrincipalCall),
        SetPrincipalRequested(SetPrincipalRequestedCall),
        SetRefinanceCommitment(SetRefinanceCommitmentCall),
        SetRefinanceInterest(SetRefinanceInterestCall),
        SetTreasuryFee(SetTreasuryFeeCall),
        TreasuryFee(TreasuryFeeCall),
    }
    impl ethers::core::abi::AbiDecode for MapleLoanInternalsHarnessCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::AcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <BorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Borrower(decoded));
            }
            if let Ok(decoded) =
                <ClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <ClearLoanAccountingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ClearLoanAccounting(decoded));
            }
            if let Ok(decoded) =
                <CloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::CloseLoan(decoded));
            }
            if let Ok(decoded) =
                <CollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Collateral(decoded));
            }
            if let Ok(decoded) =
                <CollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::CollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <CollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::CollateralRequired(decoded));
            }
            if let Ok(decoded) =
                <DelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::DelegateFee(decoded));
            }
            if let Ok(decoded) =
                <DrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::DrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <DrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::DrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <EarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::EarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <EndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::EndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <FundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::FundLoan(decoded));
            }
            if let Ok(decoded) =
                <FundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::FundsAsset(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralRequiredForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetCollateralRequiredFor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetEarlyPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetEarlyPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetInstallmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetInstallment(decoded));
            }
            if let Ok(decoded) =
                <GetInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetInterest(decoded));
            }
            if let Ok(decoded) =
                <GetNextPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetNextPaymentBreakdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetPaymentBreakdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetPaymentBreakdown(decoded));
            }
            if let Ok(decoded) =
                <GetPeriodicInterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetPeriodicInterestRate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetRefinanceCommitment(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetUnaccountedAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GetUnaccountedAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::InterestRate(decoded));
            }
            if let Ok(decoded) =
                <IsCollateralMaintainedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::IsCollateralMaintained(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::LateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <LateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::LateInterestPremium(decoded));
            }
            if let Ok(decoded) = <LenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Lender(decoded));
            }
            if let Ok(decoded) =
                <MakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::MakePayment(decoded));
            }
            if let Ok(decoded) =
                <NextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::NextPaymentDueDate(decoded));
            }
            if let Ok(decoded) =
                <PaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::PaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <PaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::PaymentsRemaining(decoded));
            }
            if let Ok(decoded) =
                <PostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::PostCollateral(decoded));
            }
            if let Ok(decoded) =
                <PrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Principal(decoded));
            }
            if let Ok(decoded) =
                <PrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::PrincipalRequested(decoded));
            }
            if let Ok(decoded) =
                <ProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::RefinanceCommitment(decoded));
            }
            if let Ok(decoded) =
                <RefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::RefinanceInterest(decoded));
            }
            if let Ok(decoded) =
                <RejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::RejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <RemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::RemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <RepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::Repossess(decoded));
            }
            if let Ok(decoded) =
                <ReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <ScaledExponentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::ScaledExponent(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetBorrower(decoded));
            }
            if let Ok(decoded) =
                <SetClaimableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetClaimableFunds(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetCollateral(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetCollateralAsset(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralRequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetCollateralRequired(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetDelegateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetDelegateFee(decoded));
            }
            if let Ok(decoded) =
                <SetDrawableFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetDrawableFunds(decoded));
            }
            if let Ok(decoded) =
                <SetEarlyFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetEarlyFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetEndingPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetEndingPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetFactory(decoded));
            }
            if let Ok(decoded) =
                <SetFundsAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetFundsAsset(decoded));
            }
            if let Ok(decoded) =
                <SetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetInterestRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateFeeRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetLateFeeRate(decoded));
            }
            if let Ok(decoded) =
                <SetLateInterestPremiumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetLateInterestPremium(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetLender(decoded));
            }
            if let Ok(decoded) =
                <SetNextPaymentDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetNextPaymentDueDate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetPaymentIntervalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetPaymentInterval(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentsRemainingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetPaymentsRemaining(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetPrincipalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetPrincipal(decoded));
            }
            if let Ok(decoded) =
                <SetPrincipalRequestedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetPrincipalRequested(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetRefinanceCommitmentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetRefinanceCommitment(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetRefinanceInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetRefinanceInterest(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetTreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::SetTreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInternalsHarnessCalls::TreasuryFee(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleLoanInternalsHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleLoanInternalsHarnessCalls::AcceptNewTerms(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Borrower(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ClaimFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ClaimableFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ClearLoanAccounting(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::CloseLoan(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Collateral(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::CollateralAsset(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::CollateralRequired(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::DelegateFee(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::DrawableFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::DrawdownFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::EarlyFeeRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::EndingPrincipal(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::FundLoan(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::FundsAsset(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GetCollateralRequiredFor(element) => {
                    element.encode()
                }
                MapleLoanInternalsHarnessCalls::GetEarlyPaymentBreakdown(element) => {
                    element.encode()
                }
                MapleLoanInternalsHarnessCalls::GetInstallment(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GetInterest(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GetNextPaymentBreakdown(element) => {
                    element.encode()
                }
                MapleLoanInternalsHarnessCalls::GetPaymentBreakdown(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GetPeriodicInterestRate(element) => {
                    element.encode()
                }
                MapleLoanInternalsHarnessCalls::GetRefinanceCommitment(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GetUnaccountedAmount(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::GracePeriod(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Initialize(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::InterestRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::IsCollateralMaintained(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::LateFeeRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::LateInterestPremium(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Lender(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::MakePayment(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::NextPaymentDueDate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::PaymentInterval(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::PaymentsRemaining(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::PostCollateral(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Principal(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::PrincipalRequested(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ProposeNewTerms(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::RefinanceCommitment(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::RefinanceInterest(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::RejectNewTerms(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::RemoveCollateral(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::Repossess(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ReturnFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::ScaledExponent(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetBorrower(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetClaimableFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetCollateral(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetCollateralAsset(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetCollateralRequired(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetDelegateFee(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetDrawableFunds(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetEarlyFeeRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetEndingPrincipal(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetFactory(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetFundsAsset(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetGracePeriod(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetInterestRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetLateFeeRate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetLateInterestPremium(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetLender(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetNextPaymentDueDate(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetPaymentInterval(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetPaymentsRemaining(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetPrincipal(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetPrincipalRequested(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetRefinanceCommitment(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetRefinanceInterest(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::SetTreasuryFee(element) => element.encode(),
                MapleLoanInternalsHarnessCalls::TreasuryFee(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleLoanInternalsHarnessCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanInternalsHarnessCalls::AcceptNewTerms(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Borrower(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ClaimFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ClaimableFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ClearLoanAccounting(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::CloseLoan(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Collateral(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::CollateralAsset(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::CollateralRequired(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::DelegateFee(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::DrawableFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::DrawdownFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::EarlyFeeRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::EndingPrincipal(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::FundLoan(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::FundsAsset(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetCollateralRequiredFor(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetEarlyPaymentBreakdown(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetInstallment(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetInterest(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetNextPaymentBreakdown(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetPaymentBreakdown(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetPeriodicInterestRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetRefinanceCommitment(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GetUnaccountedAmount(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::GracePeriod(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Initialize(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::InterestRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::IsCollateralMaintained(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::LateFeeRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::LateInterestPremium(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Lender(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::MakePayment(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::NextPaymentDueDate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::PaymentInterval(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::PaymentsRemaining(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::PostCollateral(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Principal(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::PrincipalRequested(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ProposeNewTerms(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::RefinanceCommitment(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::RefinanceInterest(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::RejectNewTerms(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::RemoveCollateral(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::Repossess(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ReturnFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::ScaledExponent(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetBorrower(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetClaimableFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetCollateral(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetCollateralAsset(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetCollateralRequired(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetDelegateFee(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetDrawableFunds(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetEarlyFeeRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetEndingPrincipal(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetFactory(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetFundsAsset(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetGracePeriod(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetInterestRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetLateFeeRate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetLateInterestPremium(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetLender(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetNextPaymentDueDate(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetPaymentInterval(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetPaymentsRemaining(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetPrincipal(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetPrincipalRequested(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetRefinanceCommitment(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetRefinanceInterest(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::SetTreasuryFee(element) => element.fmt(f),
                MapleLoanInternalsHarnessCalls::TreasuryFee(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptNewTermsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: AcceptNewTermsCall) -> Self {
            MapleLoanInternalsHarnessCalls::AcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<BorrowerCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: BorrowerCall) -> Self {
            MapleLoanInternalsHarnessCalls::Borrower(var)
        }
    }
    impl ::std::convert::From<ClaimFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ClaimFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::ClaimFunds(var)
        }
    }
    impl ::std::convert::From<ClaimableFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ClaimableFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::ClaimableFunds(var)
        }
    }
    impl ::std::convert::From<ClearLoanAccountingCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ClearLoanAccountingCall) -> Self {
            MapleLoanInternalsHarnessCalls::ClearLoanAccounting(var)
        }
    }
    impl ::std::convert::From<CloseLoanCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: CloseLoanCall) -> Self {
            MapleLoanInternalsHarnessCalls::CloseLoan(var)
        }
    }
    impl ::std::convert::From<CollateralCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: CollateralCall) -> Self {
            MapleLoanInternalsHarnessCalls::Collateral(var)
        }
    }
    impl ::std::convert::From<CollateralAssetCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: CollateralAssetCall) -> Self {
            MapleLoanInternalsHarnessCalls::CollateralAsset(var)
        }
    }
    impl ::std::convert::From<CollateralRequiredCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: CollateralRequiredCall) -> Self {
            MapleLoanInternalsHarnessCalls::CollateralRequired(var)
        }
    }
    impl ::std::convert::From<DelegateFeeCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: DelegateFeeCall) -> Self {
            MapleLoanInternalsHarnessCalls::DelegateFee(var)
        }
    }
    impl ::std::convert::From<DrawableFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: DrawableFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::DrawableFunds(var)
        }
    }
    impl ::std::convert::From<DrawdownFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: DrawdownFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::DrawdownFunds(var)
        }
    }
    impl ::std::convert::From<EarlyFeeRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: EarlyFeeRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::EarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<EndingPrincipalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: EndingPrincipalCall) -> Self {
            MapleLoanInternalsHarnessCalls::EndingPrincipal(var)
        }
    }
    impl ::std::convert::From<FundLoanCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: FundLoanCall) -> Self {
            MapleLoanInternalsHarnessCalls::FundLoan(var)
        }
    }
    impl ::std::convert::From<FundsAssetCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: FundsAssetCall) -> Self {
            MapleLoanInternalsHarnessCalls::FundsAsset(var)
        }
    }
    impl ::std::convert::From<GetCollateralRequiredForCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetCollateralRequiredForCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetCollateralRequiredFor(var)
        }
    }
    impl ::std::convert::From<GetEarlyPaymentBreakdownCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetEarlyPaymentBreakdownCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetEarlyPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetInstallmentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetInstallmentCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetInstallment(var)
        }
    }
    impl ::std::convert::From<GetInterestCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetInterestCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetInterest(var)
        }
    }
    impl ::std::convert::From<GetNextPaymentBreakdownCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetNextPaymentBreakdownCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetNextPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetPaymentBreakdownCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetPaymentBreakdownCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetPaymentBreakdown(var)
        }
    }
    impl ::std::convert::From<GetPeriodicInterestRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetPeriodicInterestRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetPeriodicInterestRate(var)
        }
    }
    impl ::std::convert::From<GetRefinanceCommitmentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetRefinanceCommitmentCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetRefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<GetUnaccountedAmountCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GetUnaccountedAmountCall) -> Self {
            MapleLoanInternalsHarnessCalls::GetUnaccountedAmount(var)
        }
    }
    impl ::std::convert::From<GracePeriodCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: GracePeriodCall) -> Self {
            MapleLoanInternalsHarnessCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: InitializeCall) -> Self {
            MapleLoanInternalsHarnessCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: InterestRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::InterestRate(var)
        }
    }
    impl ::std::convert::From<IsCollateralMaintainedCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: IsCollateralMaintainedCall) -> Self {
            MapleLoanInternalsHarnessCalls::IsCollateralMaintained(var)
        }
    }
    impl ::std::convert::From<LateFeeRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: LateFeeRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::LateFeeRate(var)
        }
    }
    impl ::std::convert::From<LateInterestPremiumCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: LateInterestPremiumCall) -> Self {
            MapleLoanInternalsHarnessCalls::LateInterestPremium(var)
        }
    }
    impl ::std::convert::From<LenderCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: LenderCall) -> Self {
            MapleLoanInternalsHarnessCalls::Lender(var)
        }
    }
    impl ::std::convert::From<MakePaymentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: MakePaymentCall) -> Self {
            MapleLoanInternalsHarnessCalls::MakePayment(var)
        }
    }
    impl ::std::convert::From<NextPaymentDueDateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: NextPaymentDueDateCall) -> Self {
            MapleLoanInternalsHarnessCalls::NextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<PaymentIntervalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: PaymentIntervalCall) -> Self {
            MapleLoanInternalsHarnessCalls::PaymentInterval(var)
        }
    }
    impl ::std::convert::From<PaymentsRemainingCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: PaymentsRemainingCall) -> Self {
            MapleLoanInternalsHarnessCalls::PaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<PostCollateralCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: PostCollateralCall) -> Self {
            MapleLoanInternalsHarnessCalls::PostCollateral(var)
        }
    }
    impl ::std::convert::From<PrincipalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: PrincipalCall) -> Self {
            MapleLoanInternalsHarnessCalls::Principal(var)
        }
    }
    impl ::std::convert::From<PrincipalRequestedCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: PrincipalRequestedCall) -> Self {
            MapleLoanInternalsHarnessCalls::PrincipalRequested(var)
        }
    }
    impl ::std::convert::From<ProposeNewTermsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ProposeNewTermsCall) -> Self {
            MapleLoanInternalsHarnessCalls::ProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<RefinanceCommitmentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: RefinanceCommitmentCall) -> Self {
            MapleLoanInternalsHarnessCalls::RefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<RefinanceInterestCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: RefinanceInterestCall) -> Self {
            MapleLoanInternalsHarnessCalls::RefinanceInterest(var)
        }
    }
    impl ::std::convert::From<RejectNewTermsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: RejectNewTermsCall) -> Self {
            MapleLoanInternalsHarnessCalls::RejectNewTerms(var)
        }
    }
    impl ::std::convert::From<RemoveCollateralCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: RemoveCollateralCall) -> Self {
            MapleLoanInternalsHarnessCalls::RemoveCollateral(var)
        }
    }
    impl ::std::convert::From<RepossessCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: RepossessCall) -> Self {
            MapleLoanInternalsHarnessCalls::Repossess(var)
        }
    }
    impl ::std::convert::From<ReturnFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ReturnFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::ReturnFunds(var)
        }
    }
    impl ::std::convert::From<ScaledExponentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: ScaledExponentCall) -> Self {
            MapleLoanInternalsHarnessCalls::ScaledExponent(var)
        }
    }
    impl ::std::convert::From<SetBorrowerCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetBorrowerCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetBorrower(var)
        }
    }
    impl ::std::convert::From<SetClaimableFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetClaimableFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetClaimableFunds(var)
        }
    }
    impl ::std::convert::From<SetCollateralCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetCollateralCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetCollateral(var)
        }
    }
    impl ::std::convert::From<SetCollateralAssetCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetCollateralAssetCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetCollateralAsset(var)
        }
    }
    impl ::std::convert::From<SetCollateralRequiredCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetCollateralRequiredCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetCollateralRequired(var)
        }
    }
    impl ::std::convert::From<SetDelegateFeeCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetDelegateFeeCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetDelegateFee(var)
        }
    }
    impl ::std::convert::From<SetDrawableFundsCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetDrawableFundsCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetDrawableFunds(var)
        }
    }
    impl ::std::convert::From<SetEarlyFeeRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetEarlyFeeRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetEarlyFeeRate(var)
        }
    }
    impl ::std::convert::From<SetEndingPrincipalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetEndingPrincipalCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetEndingPrincipal(var)
        }
    }
    impl ::std::convert::From<SetFactoryCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetFactoryCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetFactory(var)
        }
    }
    impl ::std::convert::From<SetFundsAssetCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetFundsAssetCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetFundsAsset(var)
        }
    }
    impl ::std::convert::From<SetGracePeriodCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetGracePeriodCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetGracePeriod(var)
        }
    }
    impl ::std::convert::From<SetInterestRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetInterestRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetInterestRate(var)
        }
    }
    impl ::std::convert::From<SetLateFeeRateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetLateFeeRateCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetLateFeeRate(var)
        }
    }
    impl ::std::convert::From<SetLateInterestPremiumCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetLateInterestPremiumCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetLateInterestPremium(var)
        }
    }
    impl ::std::convert::From<SetLenderCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetLenderCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetLender(var)
        }
    }
    impl ::std::convert::From<SetNextPaymentDueDateCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetNextPaymentDueDateCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetNextPaymentDueDate(var)
        }
    }
    impl ::std::convert::From<SetPaymentIntervalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetPaymentIntervalCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetPaymentInterval(var)
        }
    }
    impl ::std::convert::From<SetPaymentsRemainingCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetPaymentsRemainingCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetPaymentsRemaining(var)
        }
    }
    impl ::std::convert::From<SetPrincipalCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetPrincipalCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetPrincipal(var)
        }
    }
    impl ::std::convert::From<SetPrincipalRequestedCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetPrincipalRequestedCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetPrincipalRequested(var)
        }
    }
    impl ::std::convert::From<SetRefinanceCommitmentCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetRefinanceCommitmentCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetRefinanceCommitment(var)
        }
    }
    impl ::std::convert::From<SetRefinanceInterestCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetRefinanceInterestCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetRefinanceInterest(var)
        }
    }
    impl ::std::convert::From<SetTreasuryFeeCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: SetTreasuryFeeCall) -> Self {
            MapleLoanInternalsHarnessCalls::SetTreasuryFee(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for MapleLoanInternalsHarnessCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            MapleLoanInternalsHarnessCalls::TreasuryFee(var)
        }
    }
}
