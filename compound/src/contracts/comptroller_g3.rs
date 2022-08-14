pub use comptroller_g3::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod comptroller_g3 {
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
    #[doc = "ComptrollerG3 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERG3_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newSpeed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CompSpeedUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"compDelta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"compBorrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DistributedBorrowerComp\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"supplier\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"compDelta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"compSupplyIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DistributedSupplierComp\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketComped\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketEntered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketExited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketListed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCloseFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"oldCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCollateralFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldCompRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCompRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCompRate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLiquidationIncentive\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMaxAssets\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPauseGuardian\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"oldPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PriceOracle\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPriceOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addCompMarkets\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract Unitroller\",\"name\":\"unitroller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"compRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"compMarketsToAdd\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"otherMarketsToAdd\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_become\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"compRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"compMarketsToAdd\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"otherMarketsToAdd\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_becomeG3\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_dropCompMarket\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setBorrowPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCloseFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCollateralFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"compRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCompRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setLiquidationIncentive\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMaxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMintPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPauseGuardian\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"newOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPriceOracle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setSeizePaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setTransferPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_supportMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allMarkets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkMembership\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"holders\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"contract CToken[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"borrowers\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"suppliers\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compAccrued\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowerIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compClaimThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compInitialIndex\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"\",\"type\":\"uint224\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSpeeds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplierIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplyState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterMarkets\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllMarkets\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsIn\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCompAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenModify\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHypotheticalAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isComptroller\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidateCalculateSeizeTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isListed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualMintAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintVerify\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauseGuardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemVerify\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refreshCompSpeeds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowerIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"seizeGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transferGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferVerify\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERG3_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b03191633179055615e9980620000336000396000f3fe608060405234801561001057600080fd5b50600436106104275760003560e01c8063747026c91161022b578063bdcdc25811610130578063dce15449116100b8578063e875544611610087578063e875544614611311578063e9af029214611319578063eabe7d911461133f578063ede4edd014611375578063f851a4401461139b57610427565b8063dce15449146112a9578063dcfbc0c7146112d5578063e4028eee146112dd578063e6653f3d1461130957610427565b8063cc7ebdc4116100ff578063cc7ebdc414611149578063ce485c5e1461116f578063d02f735114611210578063d9226ced14611256578063da3d454c1461127357610427565b8063bdcdc25814610fef578063c29982381461102b578063c488847b146110cc578063ca0af0431461111b57610427565b80639d1b5a0a116101b3578063abfceffc11610182578063abfceffc14610f33578063ac0b0bb714610fa9578063b0772d0b14610fb1578063b21be7fd14610fb9578063bb82aa5e14610fe757610427565b80639d1b5a0a14610ed9578063a76b3fda14610ee1578063a7f0e23114610f07578063aa90075414610f2b57610427565b80638e8f294b116101fa5780638e8f294b14610d045780638ebf636414610d4c578063929fe9a114610d6b57806394b2294b14610d99578063992c529414610da157610427565b8063747026c914610cc65780637dc0d1d014610cce57806387f7630314610cd65780638c57804e14610cde57610427565b80634d8e5037116103315780635f5af1aa116102b95780636a56947e116102885780636a56947e14610baa5780636b79c38d14610be65780636d154ea514610c345780636d35bf9114610c5a578063731f0c2b14610ca057610427565b80635f5af1aa146109f55780635fc7e71e14610a1b5780636810dfa614610a615780636a49111214610b8d57610427565b806351dff9891161030057806351dff9891461091a57806352d84d1e1461095657806355ee1fe1146109735780635c778605146109995780635ec88c79146109cf57610427565b80634d8e5037146108655780634e79238f1461086d5780634ef4c3e1146108c75780634fd42e17146108fd57610427565b80632d70db78116103b45780633c94786f116103835780633c94786f146107c557806341c728b9146107cd57806342cbb15c1461080957806347ef3b3b146108115780634ada90af1461085d57610427565b80632d70db7814610735578063317b0b77146107545780633aa729b4146107715780633bcf7ec11461079757610427565b80631ededc91116103fb5780631ededc91146105615780632327c7df146105a357806324008a62146106cd57806324a3d62214610709578063267822471461072d57610427565b80627e3dd21461042c57806318c882a5146104485780631c3db2e0146104765780631d7b33d714610529575b600080fd5b6104346113a3565b604080519115158252519081900360200190f35b6104346004803603604081101561045e57600080fd5b506001600160a01b03813516906020013515156113a8565b6105276004803603604081101561048c57600080fd5b6001600160a01b038235169190810190604081016020820135600160201b8111156104b657600080fd5b8201836020820111156104c857600080fd5b803590602001918460208302840111600160201b831117156104e957600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550611548945050505050565b005b61054f6004803603602081101561053f57600080fd5b50356001600160a01b03166115aa565b60408051918252519081900360200190f35b610527600480360360a081101561057757600080fd5b506001600160a01b038135811691602081013582169160408201351690606081013590608001356115bc565b610527600480360360608110156105b957600080fd5b81359190810190604081016020820135600160201b8111156105da57600080fd5b8201836020820111156105ec57600080fd5b803590602001918460208302840111600160201b8311171561060d57600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295949360208101935035915050600160201b81111561065c57600080fd5b82018360208201111561066e57600080fd5b803590602001918460208302840111600160201b8311171561068f57600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295506115c3945050505050565b61054f600480360360808110156106e357600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135611689565b610711611752565b604080516001600160a01b039092168252519081900360200190f35b610711611761565b6104346004803603602081101561074b57600080fd5b50351515611770565b61054f6004803603602081101561076a57600080fd5b50356118aa565b6105276004803603602081101561078757600080fd5b50356001600160a01b03166119bb565b610434600480360360408110156107ad57600080fd5b506001600160a01b0381351690602001351515611aec565b610434611c87565b610527600480360360808110156107e357600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135611c97565b61054f611c9d565b610527600480360360c081101561082757600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060808101359060a00135611ca2565b61054f611caa565b610527611cb0565b6108a96004803603608081101561088357600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135612102565b60408051938452602084019290925282820152519081900360600190f35b61054f600480360360608110156108dd57600080fd5b506001600160a01b0381358116916020810135909116906040013561213c565b61054f6004803603602081101561091357600080fd5b50356121e7565b6105276004803603608081101561093057600080fd5b506001600160a01b038135811691602081013590911690604081013590606001356122db565b6107116004803603602081101561096c57600080fd5b503561232f565b61054f6004803603602081101561098957600080fd5b50356001600160a01b0316612356565b610527600480360360608110156109af57600080fd5b506001600160a01b038135811691602081013590911690604001356123dd565b6108a9600480360360208110156109e557600080fd5b50356001600160a01b03166123e2565b61054f60048036036020811015610a0b57600080fd5b50356001600160a01b0316612417565b61054f600480360360a0811015610a3157600080fd5b506001600160a01b038135811691602081013582169160408201358116916060810135909116906080013561249b565b61052760048036036080811015610a7757600080fd5b810190602081018135600160201b811115610a9157600080fd5b820183602082011115610aa357600080fd5b803590602001918460208302840111600160201b83111715610ac457600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295949360208101935035915050600160201b811115610b1357600080fd5b820183602082011115610b2557600080fd5b803590602001918460208302840111600160201b83111715610b4657600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550505050803515159150602001351515612622565b61052760048036036020811015610ba357600080fd5b50356127cb565b61052760048036036080811015610bc057600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135611c97565b610c0c60048036036020811015610bfc57600080fd5b50356001600160a01b031661286f565b604080516001600160e01b03909316835263ffffffff90911660208301528051918290030190f35b61043460048036036020811015610c4a57600080fd5b50356001600160a01b0316612899565b610527600480360360a0811015610c7057600080fd5b506001600160a01b03813581169160208101358216916040820135811691606081013590911690608001356115bc565b61043460048036036020811015610cb657600080fd5b50356001600160a01b03166128ae565b61054f6128c3565b6107116128ce565b6104346128dd565b610c0c60048036036020811015610cf457600080fd5b50356001600160a01b03166128ed565b610d2a60048036036020811015610d1a57600080fd5b50356001600160a01b0316612917565b6040805193151584526020840192909252151582820152519081900360600190f35b61043460048036036020811015610d6257600080fd5b5035151561293d565b61043460048036036040811015610d8157600080fd5b506001600160a01b0381358116916020013516612a76565b61054f612aa9565b61052760048036036080811015610db757600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b811115610de657600080fd5b820183602082011115610df857600080fd5b803590602001918460208302840111600160201b83111715610e1957600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295949360208101935035915050600160201b811115610e6857600080fd5b820183602082011115610e7a57600080fd5b803590602001918460208302840111600160201b83111715610e9b57600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550612aaf945050505050565b610711612cfb565b61054f60048036036020811015610ef757600080fd5b50356001600160a01b0316612d13565b610f0f612e70565b604080516001600160e01b039092168252519081900360200190f35b61054f612e83565b610f5960048036036020811015610f4957600080fd5b50356001600160a01b0316612e89565b60408051602080825283518183015283519192839290830191858101910280838360005b83811015610f95578181015183820152602001610f7d565b505050509050019250505060405180910390f35b610434612f12565b610f59612f22565b61054f60048036036040811015610fcf57600080fd5b506001600160a01b0381358116916020013516612f84565b610711612fa1565b61054f6004803603608081101561100557600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135612fb0565b610f596004803603602081101561104157600080fd5b810190602081018135600160201b81111561105b57600080fd5b82018360208201111561106d57600080fd5b803590602001918460208302840111600160201b8311171561108e57600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550613044945050505050565b611102600480360360608110156110e257600080fd5b506001600160a01b038135811691602081013590911690604001356130db565b6040805192835260208301919091528051918290030190f35b61054f6004803603604081101561113157600080fd5b506001600160a01b0381358116916020013516613350565b61054f6004803603602081101561115f57600080fd5b50356001600160a01b031661336d565b6105276004803603602081101561118557600080fd5b810190602081018135600160201b81111561119f57600080fd5b8201836020820111156111b157600080fd5b803590602001918460208302840111600160201b831117156111d257600080fd5b91908080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525092955061337f945050505050565b61054f600480360360a081101561122657600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060800135613414565b61054f6004803603602081101561126c57600080fd5b50356135cc565b61054f6004803603606081101561128957600080fd5b506001600160a01b03813581169160208101359091169060400135613635565b610711600480360360408110156112bf57600080fd5b506001600160a01b038135169060200135613922565b610711613957565b61054f600480360360408110156112f357600080fd5b506001600160a01b038135169060200135613966565b610434613b16565b61054f613b26565b6105276004803603602081101561132f57600080fd5b50356001600160a01b0316613b2c565b61054f6004803603606081101561135557600080fd5b506001600160a01b03813581169160208101359091169060400135613b90565b61054f6004803603602081101561138b57600080fd5b50356001600160a01b0316613bcd565b610711613ee0565b600181565b6001600160a01b03821660009081526009602052604081205460ff166113ff5760405162461bcd60e51b8152600401808060200182810382526028815260200180615daa6028913960400191505060405180910390fd5b600a546001600160a01b031633148061142257506000546001600160a01b031633145b61145d5760405162461bcd60e51b8152600401808060200182810382526027815260200180615dd26027913960400191505060405180910390fd5b6000546001600160a01b031633148061147857506001821515145b6114c2576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600c6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260069083015265426f72726f7760d01b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150805b92915050565b60408051600180825281830190925260609160208083019080388339019050509050828160008151811061157857fe5b60200260200101906001600160a01b031690816001600160a01b0316815250506115a58183600180612622565b505050565b600f6020526000908152604090205481565b5050505050565b6002546001600160a01b03163314611622576040805162461bcd60e51b815260206004820152601d60248201527f6f6e6c7920627261696e732063616e206265636f6d6520697473656c66000000604482015290519081900360640190fd5b60005b82518110156116525761164a83828151811061163d57fe5b6020026020010151613eef565b600101611625565b5060005b81518110156116765761166e82828151811061163d57fe5b600101611656565b50611680836127cb565b6115a58261337f565b6001600160a01b03841660009081526009602052604081205460ff166116b15750600961174a565b6116b9615cea565b6040518060200160405280876001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b1580156116fd57600080fd5b505afa158015611711573d6000803e3d6000fd5b505050506040513d602081101561172757600080fd5b5051905290506117378682613fcd565b6117448685836000614255565b60009150505b949350505050565b600a546001600160a01b031681565b6001546001600160a01b031681565b600a546000906001600160a01b031633148061179657506000546001600160a01b031633145b6117d15760405162461bcd60e51b8152600401808060200182810382526027815260200180615dd26027913960400191505060405180910390fd5b6000546001600160a01b03163314806117ec57506001821515145b611836576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b81b810260ff60b81b1990921691909117909155604080516020810192909252808252600582820152645365697a6560d81b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a150805b919050565b600080546001600160a01b031633146118d0576118c960016004614440565b90506118a5565b6118d8615cea565b5060408051602081019091528281526118ef615cea565b50604080516020810190915266b1a2bc2ec50000815261190f82826144a6565b156119285761191f600580614440565b925050506118a5565b611930615cea565b506040805160208101909152670c7d713b49da0000815261195181846144ae565b1561196b57611961600580614440565b93505050506118a5565b6005805490869055604080518281526020810188905281517f3b9670cf975d26958e754b57098eaa2ac914d8d2a31b83257997b9f346110fd9929181900390910190a160005b9695505050505050565b6000546001600160a01b03163314611a1a576040805162461bcd60e51b815260206004820152601f60248201527f6f6e6c792061646d696e2063616e2064726f7020636f6d70206d61726b657400604482015290519081900360640190fd5b6001600160a01b0381166000908152600960205260409020600381015460ff161515600114611a90576040805162461bcd60e51b815260206004820152601b60248201527f6d61726b6574206973206e6f74206120636f6d70206d61726b65740000000000604482015290519081900360640190fd5b60038101805460ff19169055604080516001600160a01b03841681526000602082015281517f93c1f3e36ed71139f466a4ce8c9751790e2e33f5afb2df0dcfb3aeabe55d5aa2929181900390910190a1611ae8611cb0565b5050565b6001600160a01b03821660009081526009602052604081205460ff16611b435760405162461bcd60e51b8152600401808060200182810382526028815260200180615daa6028913960400191505060405180910390fd5b600a546001600160a01b0316331480611b6657506000546001600160a01b031633145b611ba15760405162461bcd60e51b8152600401808060200182810382526027815260200180615dd26027913960400191505060405180910390fd5b6000546001600160a01b0316331480611bbc57506001821515145b611c06576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600b6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260049083015263135a5b9d60e21b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150919050565b600a54600160a01b900460ff1681565b50505050565b435b90565b505050505050565b60065481565b6060600d805480602002602001604051908101604052809291908181526020018280548015611d0857602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311611cea575b50939450600093505050505b8151811015611dce576000828281518110611d2b57fe5b60200260200101519050611d3d615cea565b6040518060200160405280836001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b158015611d8157600080fd5b505afa158015611d95573d6000803e3d6000fd5b505050506040513d6020811015611dab57600080fd5b505190529050611dba826144b5565b611dc48282613fcd565b5050600101611d14565b50611dd7615cea565b60405180602001604052806000815250905060608251604051908082528060200260200182016040528015611e2657816020015b611e13615cea565b815260200190600190039081611e0b5790505b50905060005b8351811015612039576000848281518110611e4357fe5b6020908102919091018101516001600160a01b0381166000908152600990925260409091206003015490915060ff161561203057611e7f615cea565b60408051602080820180845260045463fc57d4df60e01b9091526001600160a01b03868116602485015293519293849391169163fc57d4df916044808601929190818703018186803b158015611ed457600080fd5b505afa158015611ee8573d6000803e3d6000fd5b505050506040513d6020811015611efe57600080fd5b505190529050611f0c615cea565b611ff26040518060200160405280856001600160a01b031663f8f9da286040518163ffffffff1660e01b815260040160206040518083038186803b158015611f5357600080fd5b505afa158015611f67573d6000803e3d6000fd5b505050506040513d6020811015611f7d57600080fd5b50519052604080516308f7a6e360e31b815290516001600160a01b038716916347bd3718916004808301926020929190829003018186803b158015611fc157600080fd5b505afa158015611fd5573d6000803e3d6000fd5b505050506040513d6020811015611feb57600080fd5b5051614733565b9050611ffc615cea565b612006828461475d565b90508086868151811061201557fe5b602002602001018190525061202a878261479c565b96505050505b50600101611e2c565b5060005b8351811015611c97576000600d828154811061205557fe5b600091825260208220015485516001600160a01b03909116925061207a5760006120a2565b6120a2600e5461209d86868151811061208f57fe5b6020026020010151886147c1565b6147f4565b6001600160a01b0383166000818152600f60209081526040918290208490558151848152915193945091927f2ab93f65628379309f36cb125e90d7c902454a545c4f8b8cb0794af75c24b807929181900390910190a2505060010161203d565b6000806000806000806121178a8a8a8a61481c565b92509250925082601181111561212957fe5b95509093509150505b9450945094915050565b6001600160a01b0383166000908152600b602052604081205460ff161561219b576040805162461bcd60e51b815260206004820152600e60248201526d1b5a5b9d081a5cc81c185d5cd95960921b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff166121c55760095b90506121e0565b6121ce846144b5565b6121da84846000614c37565b60005b90505b9392505050565b600080546001600160a01b03163314612206576118c96001600b614440565b61220e615cea565b506040805160208101909152828152612225615cea565b506040805160208101909152670de0b6b3a7640000815261224682826144ae565b156122575761191f6007600c614440565b61225f615cea565b5060408051602081019091526714d1120d7b160000815261228081846144ae565b15612291576119616007600c614440565b6006805490869055604080518281526020810188905281517faeba5a6c40a8ac138134bff1aaa65debf25971188a58804bad717f82f0ec1316929181900390910190a160006119b1565b801580156122e95750600082115b15611c97576040805162461bcd60e51b815260206004820152601160248201527072656465656d546f6b656e73207a65726f60781b604482015290519081900360640190fd5b600d818154811061233c57fe5b6000918252602090912001546001600160a01b0316905081565b600080546001600160a01b03163314612375576118c960016010614440565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fd52b2b9b7e9ee655fcb95d2e5b9e0c9f69e7ef2b8e9d2d0ea78402d576d22e22929181900390910190a160009392505050565b6115a5565b6000806000806000806123f987600080600061481c565b92509250925082601181111561240b57fe5b97919650945092505050565b600080546001600160a01b03163314612436576118c960016013614440565b600a80546001600160a01b038481166001600160a01b0319831617928390556040805192821680845293909116602083015280517f0613b6ee6a04f0d09f390e4d9318894b9f6ac7fd83897cd8d18896ba579c401e9281900390910190a160006121e0565b6001600160a01b03851660009081526009602052604081205460ff1615806124dc57506001600160a01b03851660009081526009602052604090205460ff16155b156124eb5760095b9050612619565b6000806124f785614e2f565b9193509091506000905082601181111561250d57fe5b146125275781601181111561251e57fe5b92505050612619565b8061253357600361251e565b6000886001600160a01b03166395dd9193876040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b15801561258b57600080fd5b505afa15801561259f573d6000803e3d6000fd5b505050506040513d60208110156125b557600080fd5b50516040805160208101909152600554815290915060009081906125d99084614e4f565b909250905060008260038111156125ec57fe5b1461260057600b5b95505050505050612619565b8087111561260f5760116125f4565b6000955050505050505b95945050505050565b60005b83518110156115bc57600084828151811061263c57fe5b6020908102919091018101516001600160a01b0381166000908152600990925260409091205490915060ff166126b1576040805162461bcd60e51b81526020600482015260156024820152741b585c9ad95d081b5d5cdd081899481b1a5cdd1959605a1b604482015290519081900360640190fd5b60018415151415612779576126c4615cea565b6040518060200160405280836001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b15801561270857600080fd5b505afa15801561271c573d6000803e3d6000fd5b505050506040513d602081101561273257600080fd5b5051905290506127428282613fcd565b60005b87518110156127765761276e8389838151811061275e57fe5b6020026020010151846001614255565b600101612745565b50505b600183151514156127c25761278d816144b5565b60005b86518110156127c0576127b8828883815181106127a957fe5b60200260200101516001614c37565b600101612790565b505b50600101612625565b6127d3614ea3565b612824576040805162461bcd60e51b815260206004820152601f60248201527f6f6e6c792061646d696e2063616e206368616e676520636f6d70207261746500604482015290519081900360640190fd5b600e805490829055604080518281526020810184905281517fc227c9272633c3a307d9845bf2bc2509cefb20d655b5f3c1002d8e1e3f22c8b0929181900390910190a1611ae8611cb0565b6010602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b600c6020526000908152604090205460ff1681565b600b6020526000908152604090205460ff1681565b66038d7ea4c6800081565b6004546001600160a01b031681565b600a54600160b01b900460ff1681565b6011602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b60096020526000908152604090208054600182015460039092015460ff91821692911683565b600a546000906001600160a01b031633148061296357506000546001600160a01b031633145b61299e5760405162461bcd60e51b8152600401808060200182810382526027815260200180615dd26027913960400191505060405180910390fd5b6000546001600160a01b03163314806129b957506001821515145b612a03576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b01b810260ff60b01b1990921691909117909155604080516020810192909252808252600882820152672a3930b739b332b960c11b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a15090565b6001600160a01b038082166000908152600960209081526040808320938616835260029093019052205460ff1692915050565b60075481565b836001600160a01b031663f851a4406040518163ffffffff1660e01b815260040160206040518083038186803b158015612ae857600080fd5b505afa158015612afc573d6000803e3d6000fd5b505050506040513d6020811015612b1257600080fd5b50516001600160a01b03163314612b5a5760405162461bcd60e51b8152600401808060200182810382526027815260200180615e3e6027913960400191505060405180910390fd5b836001600160a01b031663c1e803346040518163ffffffff1660e01b8152600401602060405180830381600087803b158015612b9557600080fd5b505af1158015612ba9573d6000803e3d6000fd5b505050506040513d6020811015612bbf57600080fd5b505115612c0b576040805162461bcd60e51b815260206004820152601560248201527418da185b99d9481b9bdd08185d5d1a1bdc9a5e9959605a1b604482015290519081900360640190fd5b836001600160a01b0316632327c7df8484846040518463ffffffff1660e01b8152600401808481526020018060200180602001838103835285818151815260200191508051906020019060200280838360005b83811015612c76578181015183820152602001612c5e565b50505050905001838103825284818151815260200191508051906020019060200280838360005b83811015612cb5578181015183820152602001612c9d565b5050505090500195505050505050600060405180830381600087803b158015612cdd57600080fd5b505af1158015612cf1573d6000803e3d6000fd5b5050505050505050565b73c00e94cb662c3520282e6f5717214004a7f2688890565b600080546001600160a01b03163314612d32576118c960016012614440565b6001600160a01b03821660009081526009602052604090205460ff1615612d5f576118c9600a6011614440565b816001600160a01b031663fe9c44ae6040518163ffffffff1660e01b815260040160206040518083038186803b158015612d9857600080fd5b505afa158015612dac573d6000803e3d6000fd5b505050506040513d6020811015612dc257600080fd5b5050604080516060810182526001808252600060208381018281528486018381526001600160a01b03891684526009909252949091209251835490151560ff19918216178455935191830191909155516003909101805491151591909216179055612e2c82613eef565b604080516001600160a01b038416815290517fcf583bb0c569eb967f806b11601c4cb93c10310485c67add5f8362c2f212321f9181900360200190a1600092915050565b6ec097ce7bc90715b34b9f100000000081565b600e5481565b60608060086000846001600160a01b03166001600160a01b03168152602001908152602001600020805480602002602001604051908101604052809291908181526020018280548015612f0557602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311612ee7575b5093979650505050505050565b600a54600160b81b900460ff1681565b6060600d805480602002602001604051908101604052809291908181526020018280548015612f7a57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311612f5c575b5050505050905090565b601260209081526000928352604080842090915290825290205481565b6002546001600160a01b031681565b600a54600090600160b01b900460ff1615613007576040805162461bcd60e51b81526020600482015260126024820152711d1c985b9cd9995c881a5cc81c185d5cd95960721b604482015290519081900360640190fd5b6000613014868685614ecc565b9050801561302357905061174a565b61302c866144b5565b61303886866000614c37565b61174486856000614c37565b6060600082519050606081604051908082528060200260200182016040528015613078578160200160208202803883390190505b50905060005b828110156130d357600085828151811061309457fe5b602002602001015190506130a88133614f6f565b60118111156130b357fe5b8383815181106130bf57fe5b60209081029190910101525060010161307e565b509392505050565b600480546040805163fc57d4df60e01b81526001600160a01b038781169482019490945290516000938493849391169163fc57d4df91602480820192602092909190829003018186803b15801561313157600080fd5b505afa158015613145573d6000803e3d6000fd5b505050506040513d602081101561315b57600080fd5b5051600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051939450600093929091169163fc57d4df91602480820192602092909190829003018186803b1580156131b457600080fd5b505afa1580156131c8573d6000803e3d6000fd5b505050506040513d60208110156131de57600080fd5b505190508115806131ed575080155b1561320257600d935060009250613348915050565b6000866001600160a01b031663182df0f56040518163ffffffff1660e01b815260040160206040518083038186803b15801561323d57600080fd5b505afa158015613251573d6000803e3d6000fd5b505050506040513d602081101561326757600080fd5b505190506000613275615cea565b61327d615cea565b613285615cea565b600061329360065489615090565b9450905060008160038111156132a557fe5b146132c157600b5b995060009850613348975050505050505050565b6132cb8787615090565b9350905060008160038111156132dd57fe5b146132e957600b6132ad565b6132f384846150cb565b92509050600081600381111561330557fe5b1461331157600b6132ad565b61331b828c614e4f565b95509050600081600381111561332d57fe5b1461333957600b6132ad565b60009950939750505050505050505b935093915050565b601360209081526000928352604080842090915290825290205481565b60146020526000908152604090205481565b613387614ea3565b6133d8576040805162461bcd60e51b815260206004820152601e60248201527f6f6e6c792061646d696e2063616e2061646420636f6d70206d61726b65740000604482015290519081900360640190fd5b60005b8151811015613408576134008282815181106133f357fe5b60200260200101516150e3565b6001016133db565b50613411611cb0565b50565b600a54600090600160b81b900460ff1615613468576040805162461bcd60e51b815260206004820152600f60248201526e1cd95a5e99481a5cc81c185d5cd959608a1b604482015290519081900360640190fd5b6001600160a01b03861660009081526009602052604090205460ff1615806134a957506001600160a01b03851660009081526009602052604090205460ff16155b156134b55760096124e4565b846001600160a01b0316635fe3b5676040518163ffffffff1660e01b815260040160206040518083038186803b1580156134ee57600080fd5b505afa158015613502573d6000803e3d6000fd5b505050506040513d602081101561351857600080fd5b505160408051635fe3b56760e01b815290516001600160a01b0392831692891691635fe3b567916004808301926020929190829003018186803b15801561355e57600080fd5b505afa158015613572573d6000803e3d6000fd5b505050506040513d602081101561358857600080fd5b50516001600160a01b03161461359f5760026124e4565b6135a8866144b5565b6135b486846000614c37565b6135c086856000614c37565b60009695505050505050565b600080546001600160a01b031633146135eb576118c96001600d614440565b6007805490839055604080518281526020810185905281517f7093cf1eb653f749c3ff531d6df7f92764536a7fa0d13530cd26e070780c32ea929181900390910190a160006121e0565b6001600160a01b0383166000908152600c602052604081205460ff1615613696576040805162461bcd60e51b815260206004820152601060248201526f189bdc9c9bddc81a5cc81c185d5cd95960821b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff166136bd5760096121be565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff166137ad57336001600160a01b03851614613743576040805162461bcd60e51b815260206004820152601560248201527439b2b73232b91036bab9ba1031329031aa37b5b2b760591b604482015290519081900360640190fd5b600061374f3385614f6f565b9050600081601181111561375f57fe5b146137785780601181111561377057fe5b9150506121e0565b6001600160a01b038086166000908152600960209081526040808320938816835260029093019052205460ff166137ab57fe5b505b600480546040805163fc57d4df60e01b81526001600160a01b03888116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b1580156137fe57600080fd5b505afa158015613812573d6000803e3d6000fd5b505050506040513d602081101561382857600080fd5b505161383557600d6121be565b600080613845858760008761481c565b9193509091506000905082601181111561385b57fe5b146138755781601181111561386c57fe5b925050506121e0565b801561388257600461386c565b61388a615cea565b6040518060200160405280886001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b1580156138ce57600080fd5b505afa1580156138e2573d6000803e3d6000fd5b505050506040513d60208110156138f857600080fd5b5051905290506139088782613fcd565b6139158787836000614255565b6000979650505050505050565b6008602052816000526040600020818154811061393b57fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b600080546001600160a01b0316331461398c5761398560016006614440565b9050611542565b6001600160a01b0383166000908152600960205260409020805460ff166139c1576139b960096007614440565b915050611542565b6139c9615cea565b5060408051602081019091528381526139e0615cea565b506040805160208101909152670c7d713b49da00008152613a0181836144ae565b15613a1c57613a1260066008614440565b9350505050611542565b8415801590613aa55750600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b158015613a7757600080fd5b505afa158015613a8b573d6000803e3d6000fd5b505050506040513d6020811015613aa157600080fd5b5051155b15613ab657613a12600d6009614440565b60018301805490869055604080516001600160a01b03891681526020810183905280820188905290517f70483e6592cd5182d45ac970e05bc62cdcc90e9d8ef2c2dbe686cf383bcd7fc59181900360600190a16000979650505050505050565b600a54600160a81b900460ff1681565b60055481565b61341181600d805480602002602001604051908101604052809291908181526020018280548015613b8657602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613b68575b5050505050611548565b600080613b9e858585614ecc565b90508015613bad5790506121e0565b613bb6856144b5565b613bc285856000614c37565b600095945050505050565b6000808290506000806000836001600160a01b031663c37f68e2336040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b158015613c2e57600080fd5b505afa158015613c42573d6000803e3d6000fd5b505050506040513d6080811015613c5857600080fd5b508051602082015160409092015190945090925090508215613cab5760405162461bcd60e51b8152600401808060200182810382526025815260200180615df96025913960400191505060405180910390fd5b8015613cc857613cbd600c6002614440565b9450505050506118a5565b6000613cd5873385614ecc565b90508015613cf657613cea600e6003836153fe565b955050505050506118a5565b6001600160a01b0385166000908152600960209081526040808320338452600281019092529091205460ff16613d3557600096505050505050506118a5565b3360009081526002820160209081526040808320805460ff191690556008825291829020805483518184028101840190945280845260609392830182828015613da757602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613d89575b5050835193945083925060009150505b82811015613dfc57896001600160a01b0316848281518110613dd557fe5b60200260200101516001600160a01b03161415613df457809150613dfc565b600101613db7565b50818110613e0657fe5b336000908152600860205260409020805481906000198101908110613e2757fe5b9060005260206000200160009054906101000a90046001600160a01b0316818381548110613e5157fe5b600091825260209091200180546001600160a01b0319166001600160a01b03929092169190911790558054613e8a826000198301615cfd565b50604080516001600160a01b038c16815233602082015281517fe699a64c18b07ac5b7301aa273f36a2287239eb9501d81950672794afba29a0d929181900390910190a160009c9b505050505050505050505050565b6000546001600160a01b031681565b60005b600d54811015613f7a57816001600160a01b0316600d8281548110613f1357fe5b6000918252602090912001546001600160a01b03161415613f72576040805162461bcd60e51b81526020600482015260146024820152731b585c9ad95d08185b1c9958591e48185919195960621b604482015290519081900360640190fd5b600101613ef2565b50600d80546001810182556000919091527fd7b6990105719101dabeb77144f2a3385c8033acd3af97e9423a695e81ad1eb50180546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b0382166000908152601160209081526040808320600f9092528220549091613ffa611c9d565b835490915060009061401a908390600160e01b900463ffffffff16615464565b905060008111801561402c5750600083115b156141fb5760006140a1876001600160a01b03166347bd37186040518163ffffffff1660e01b815260040160206040518083038186803b15801561406f57600080fd5b505afa158015614083573d6000803e3d6000fd5b505050506040513d602081101561409957600080fd5b50518761549e565b905060006140af83866154bc565b90506140b9615cea565b600083116140d657604051806020016040528060008152506140e0565b6140e082846154fe565b90506140ea615cea565b604080516020810190915288546001600160e01b0316815261410c908361479c565b9050604051806040016040528061415c83600001516040518060400160405280601a81526020017f6e657720696e6465782065786365656473203232342062697473000000000000815250615533565b6001600160e01b03168152602001614197886040518060400160405280601c8152602001600080516020615e1e8339815191528152506155cd565b63ffffffff9081169091526001600160a01b038c166000908152601160209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b0319909416939093171691909117905550611ca292505050565b8015611ca25761422e826040518060400160405280601c8152602001600080516020615e1e8339815191528152506155cd565b845463ffffffff91909116600160e01b026001600160e01b03909116178455505050505050565b6001600160a01b0384166000908152601160205260409020614275615cea565b50604080516020810190915281546001600160e01b03168152614296615cea565b5060408051602080820183526001600160a01b03808a16600090815260138352848120918a1680825282845294812080548552865195909152915291909155805115614437576142e4615cea565b6142ee8383615622565b9050600061437d896001600160a01b03166395dd91938a6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b15801561434b57600080fd5b505afa15801561435f573d6000803e3d6000fd5b505050506040513d602081101561437557600080fd5b50518861549e565b9050600061438b8284615647565b6001600160a01b038a16600090815260146020526040812054919250906143b29083615667565b90506143d38a828a6143cb5766038d7ea4c680006143ce565b60005b61569d565b6001600160a01b03808c1660008181526014602090815260409182902094909455895181518781529485015280519193928f16927f1fc3ecc087d8d2d15e23d0032af5a47059c3892d003d8e139fdcb6bb327c99a6929081900390910190a3505050505b50505050505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa083601181111561446f57fe5b83601381111561447b57fe5b604080519283526020830191909152600082820152519081900360600190a18260118111156121e057fe5b519051111590565b5190511090565b6001600160a01b0381166000908152601060209081526040808320600f90925282205490916144e2611c9d565b8354909150600090614502908390600160e01b900463ffffffff16615464565b90506000811180156145145750600083115b156146da576000856001600160a01b03166318160ddd6040518163ffffffff1660e01b815260040160206040518083038186803b15801561455457600080fd5b505afa158015614568573d6000803e3d6000fd5b505050506040513d602081101561457e57600080fd5b50519050600061458e83866154bc565b9050614598615cea565b600083116145b557604051806020016040528060008152506145bf565b6145bf82846154fe565b90506145c9615cea565b604080516020810190915288546001600160e01b031681526145eb908361479c565b9050604051806040016040528061463b83600001516040518060400160405280601a81526020017f6e657720696e6465782065786365656473203232342062697473000000000000815250615533565b6001600160e01b03168152602001614676886040518060400160405280601c8152602001600080516020615e1e8339815191528152506155cd565b63ffffffff9081169091526001600160a01b038b166000908152601060209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b03199094169390931716919091179055506115bc92505050565b80156115bc5761470d826040518060400160405280601c8152602001600080516020615e1e8339815191528152506155cd565b845463ffffffff91909116600160e01b026001600160e01b039091161784555050505050565b61473b615cea565b60405180602001604052806147548560000151856154bc565b90529392505050565b614765615cea565b6040518060200160405280670de0b6b3a764000061478b866000015186600001516154bc565b8161479257fe5b0490529392505050565b6147a4615cea565b604051806020016040528061475485600001518560000151615667565b6147c9615cea565b60405180602001604052806147546147ed8660000151670de0b6b3a76400006154bc565b85516157e2565b6000670de0b6b3a764000061480d8484600001516154bc565b8161481457fe5b049392505050565b6000806000614829615d21565b6001600160a01b03881660009081526008602090815260408083208054825181850281018501909352808352849360609392919083018282801561489657602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311614878575b50939450600093505050505b8151811015614bf25760008282815181106148b957fe5b60200260200101519050806001600160a01b031663c37f68e28e6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b15801561491957600080fd5b505afa15801561492d573d6000803e3d6000fd5b505050506040513d608081101561494357600080fd5b508051602082015160408084015160609485015160808c0152938a019390935291880191909152945084156149895750600f975060009650869550612132945050505050565b60408051602080820183526001600160a01b0380851660008181526009845285902060010154845260c08b01939093528351808301855260808b0151815260e08b015260048054855163fc57d4df60e01b815291820194909452935192169263fc57d4df9260248083019392829003018186803b158015614a0957600080fd5b505afa158015614a1d573d6000803e3d6000fd5b505050506040513d6020811015614a3357600080fd5b505160a08701819052614a575750600d975060009650869550612132945050505050565b604080516020810190915260a08701518152610100870181905260c087015160e0880151614a8492615815565b61012088015293506000846003811115614a9a57fe5b14614ab65750600b975060009650869550612132945050505050565b614ace8661012001518760400151886000015161586d565b875293506000846003811115614ae057fe5b14614afc5750600b975060009650869550612132945050505050565b614b148661010001518760600151886020015161586d565b602088015293506000846003811115614b2957fe5b14614b455750600b975060009650869550612132945050505050565b8b6001600160a01b0316816001600160a01b03161415614be957614b738661012001518c886020015161586d565b602088015293506000846003811115614b8857fe5b14614ba45750600b975060009650869550612132945050505050565b614bb88661010001518b886020015161586d565b602088015293506000846003811115614bcd57fe5b14614be95750600b975060009650869550612132945050505050565b506001016148a2565b50602084015184511115614c19575050506020810151905160009450039150829050612132565b50508151602090920151600095508594509190910391506121329050565b6001600160a01b0383166000908152601060205260409020614c57615cea565b50604080516020810190915281546001600160e01b03168152614c78615cea565b5060408051602080820183526001600160a01b03808916600090815260128352848120918916808252828452948120805485528651959091529152919091558051158015614cc65750815115155b15614cde576ec097ce7bc90715b34b9f100000000081525b614ce6615cea565b614cf08383615622565b90506000876001600160a01b03166370a08231886040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b158015614d4a57600080fd5b505afa158015614d5e573d6000803e3d6000fd5b505050506040513d6020811015614d7457600080fd5b505190506000614d848284615647565b6001600160a01b03891660009081526014602052604081205491925090614dab9083615667565b9050614dc489828a6143cb5766038d7ea4c680006143ce565b6001600160a01b03808b1660008181526014602090815260409182902094909455895181518781529485015280519193928e16927f2caecd17d02f56fa897705dcc740da2d237c373f70686f4e0d9bd3bf0400ea7a929081900390910190a350505050505050505050565b6000806000614e4284600080600061481c565b9250925092509193909250565b6000806000614e5c615cea565b614e6686866158ba565b90925090506000826003811115614e7957fe5b14614e8a5750915060009050614e9c565b6000614e9582615922565b9350935050505b9250929050565b600080546001600160a01b0316331480614ec757506002546001600160a01b031633145b905090565b6001600160a01b03831660009081526009602052604081205460ff16614ef35760096121be565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff16614f2b5760006121be565b600080614f3b858786600061481c565b91935090915060009050826011811115614f5157fe5b14614f625781601181111561386c57fe5b80156135c057600461386c565b6001600160a01b0382166000908152600960205260408120805460ff16614f9a576009915050611542565b6001600160a01b038316600090815260028201602052604090205460ff16151560011415614fcc576000915050611542565b6007546001600160a01b03841660009081526008602052604090205410614ff7576010915050611542565b6001600160a01b0380841660008181526002840160209081526040808320805460ff19166001908117909155600883528184208054918201815584529282902090920180549489166001600160a01b031990951685179055815193845283019190915280517f3ab23ab0d51cccc0c3085aec51f99228625aa1a922b3a8ca89a26b0f2027a1a59281900390910190a15060009392505050565b600061509a615cea565b6150c0604051806020016040528086815250604051806020016040528086815250615931565b915091509250929050565b60006150d5615cea565b835183516150c09190615a1a565b6001600160a01b0381166000908152600960205260409020805460ff161515600114615156576040805162461bcd60e51b815260206004820152601960248201527f636f6d70206d61726b6574206973206e6f74206c697374656400000000000000604482015290519081900360640190fd5b600381015460ff16156151b0576040805162461bcd60e51b815260206004820152601960248201527f636f6d70206d61726b657420616c726561647920616464656400000000000000604482015290519081900360640190fd5b60038101805460ff19166001908117909155604080516001600160a01b0385168152602081019290925280517f93c1f3e36ed71139f466a4ce8c9751790e2e33f5afb2df0dcfb3aeabe55d5aa29281900390910190a16001600160a01b0382166000908152601060205260409020546001600160e01b031615801561525857506001600160a01b038216600090815260106020526040902054600160e01b900463ffffffff16155b156153155760405180604001604052806ec097ce7bc90715b34b9f10000000006001600160e01b031681526020016152ba615291611c9d565b6040518060400160405280601c8152602001600080516020615e1e8339815191528152506155cd565b63ffffffff9081169091526001600160a01b0384166000908152601060209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b031990941693909317169190911790555b6001600160a01b0382166000908152601160205260409020546001600160e01b031615801561536757506001600160a01b038216600090815260116020526040902054600160e01b900463ffffffff16155b15611ae85760405180604001604052806ec097ce7bc90715b34b9f10000000006001600160e01b031681526020016153a0615291611c9d565b63ffffffff9081169091526001600160a01b0384166000908152601160209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b031990941693909317169190911790555050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa084601181111561542d57fe5b84601381111561543957fe5b604080519283526020830191909152818101859052519081900360600190a18360118111156121dd57fe5b60006121e08383604051806040016040528060158152602001747375627472616374696f6e20756e646572666c6f7760581b815250615aca565b60006121e06154b584670de0b6b3a76400006154bc565b83516157e2565b60006121e083836040518060400160405280601781526020017f6d756c7469706c69636174696f6e206f766572666c6f77000000000000000000815250615b24565b615506615cea565b604051806020016040528061475461552d866ec097ce7bc90715b34b9f10000000006154bc565b856157e2565b600081600160e01b84106155c55760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561558a578181015183820152602001615572565b50505050905090810190601f1680156155b75780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b509192915050565b600081600160201b84106155c55760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561558a578181015183820152602001615572565b61562a615cea565b604051806020016040528061475485600001518560000151615464565b60006ec097ce7bc90715b34b9f100000000061480d8484600001516154bc565b60006121e08383604051806040016040528060118152602001706164646974696f6e206f766572666c6f7760781b815250615ba3565b60008183101580156156af5750600083115b156157da5760006156be612cfb565b604080516370a0823160e01b815230600482015290519192506000916001600160a01b038416916370a08231916024808301926020929190829003018186803b15801561570a57600080fd5b505afa15801561571e573d6000803e3d6000fd5b505050506040513d602081101561573457600080fd5b505190508085116157d757816001600160a01b031663a9059cbb87876040518363ffffffff1660e01b815260040180836001600160a01b03166001600160a01b0316815260200182815260200192505050602060405180830381600087803b15801561579f57600080fd5b505af11580156157b3573d6000803e3d6000fd5b505050506040513d60208110156157c957600080fd5b50600093506121e092505050565b50505b509092915050565b60006121e083836040518060400160405280600e81526020016d646976696465206279207a65726f60901b815250615bf8565b600061581f615cea565b6000615829615cea565b6158338787615931565b9092509050600082600381111561584657fe5b14615855579092509050613348565b61585f8186615931565b935093505050935093915050565b600080600061587a615cea565b61588487876158ba565b9092509050600082600381111561589757fe5b146158a85750915060009050613348565b61585f6158b482615922565b86615c5a565b60006158c4615cea565b6000806158d5866000015186615c80565b909250905060008260038111156158e857fe5b1461590757506040805160208101909152600081529092509050614e9c565b60408051602081019091529081526000969095509350505050565b51670de0b6b3a7640000900490565b600061593b615cea565b60008061595086600001518660000151615c80565b9092509050600082600381111561596357fe5b1461598257506040805160208101909152600081529092509050614e9c565b6000806159976706f05b59d3b2000084615c5a565b909250905060008260038111156159aa57fe5b146159cc57506040805160208101909152600081529094509250614e9c915050565b6000806159e183670de0b6b3a7640000615cbf565b909250905060008260038111156159f457fe5b146159fb57fe5b604080516020810190915290815260009a909950975050505050505050565b6000615a24615cea565b600080615a3986670de0b6b3a7640000615c80565b90925090506000826003811115615a4c57fe5b14615a6b57506040805160208101909152600081529092509050614e9c565b600080615a788388615cbf565b90925090506000826003811115615a8b57fe5b14615aad57506040805160208101909152600081529094509250614e9c915050565b604080516020810190915290815260009890975095505050505050565b60008184841115615b1c5760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561558a578181015183820152602001615572565b505050900390565b6000831580615b31575082155b15615b3e575060006121e0565b83830283858281615b4b57fe5b04148390615b9a5760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561558a578181015183820152602001615572565b50949350505050565b60008383018285821015615b9a5760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561558a578181015183820152602001615572565b60008183615c475760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561558a578181015183820152602001615572565b50828481615c5157fe5b04949350505050565b600080838301848110615c7257600092509050614e9c565b506002915060009050614e9c565b60008083615c9357506000905080614e9c565b83830283858281615ca057fe5b0414615cb457506002915060009050614e9c565b600092509050614e9c565b60008082615cd35750600190506000614e9c565b6000838581615cde57fe5b04915091509250929050565b6040518060200160405280600081525090565b8154818355818111156115a5576000838152602090206115a5918101908301615d8b565b604051806101400160405280600081526020016000815260200160008152602001600081526020016000815260200160008152602001615d5f615cea565b8152602001615d6c615cea565b8152602001615d79615cea565b8152602001615d86615cea565b905290565b611c9f91905b80821115615da55760008155600101615d91565b509056fe63616e6e6f742070617573652061206d61726b65742074686174206973206e6f74206c69737465646f6e6c7920706175736520677561726469616e20616e642061646d696e2063616e207061757365657869744d61726b65743a206765744163636f756e74536e617073686f74206661696c6564626c6f636b206e756d62657220657863656564732033322062697473000000006f6e6c7920756e6974726f6c6c65722061646d696e2063616e206368616e676520627261696e73a265627a7a723158200026382377608808a8c013ea8efd89730a739001f27ef8ac3aaeb7bb279f45cc64736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    pub struct ComptrollerG3<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ComptrollerG3<M> {
        fn clone(&self) -> Self {
            ComptrollerG3(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ComptrollerG3<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerG3<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerG3))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ComptrollerG3<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMPTROLLERG3_ABI.clone(), client)
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
                COMPTROLLERG3_ABI.clone(),
                COMPTROLLERG3_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_addCompMarkets` (0xce485c5e) function"]
        pub fn add_comp_markets(
            &self,
            c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 72, 92, 94], c_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_become` (0x992c5294) function"]
        pub fn become_(
            &self,
            unitroller: ethers::core::types::Address,
            comp_rate: ethers::core::types::U256,
            comp_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
            other_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 44, 82, 148],
                    (
                        unitroller,
                        comp_rate,
                        comp_markets_to_add,
                        other_markets_to_add,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_becomeG3` (0x2327c7df) function"]
        pub fn become_g3(
            &self,
            comp_rate: ethers::core::types::U256,
            comp_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
            other_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [35, 39, 199, 223],
                    (comp_rate, comp_markets_to_add, other_markets_to_add),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_borrowGuardianPaused` (0xe6653f3d) function"]
        pub fn _borrow_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 101, 63, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_dropCompMarket` (0x3aa729b4) function"]
        pub fn drop_comp_market(
            &self,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 167, 41, 180], c_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_mintGuardianPaused` (0x3c94786f) function"]
        pub fn _mint_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([60, 148, 120, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setBorrowPaused` (0x18c882a5) function"]
        pub fn set_borrow_paused(
            &self,
            c_token: ethers::core::types::Address,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 200, 130, 165], (c_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setCloseFactor` (0x317b0b77) function"]
        pub fn set_close_factor(
            &self,
            new_close_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([49, 123, 11, 119], new_close_factor_mantissa)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setCollateralFactor` (0xe4028eee) function"]
        pub fn set_collateral_factor(
            &self,
            c_token: ethers::core::types::Address,
            new_collateral_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [228, 2, 142, 238],
                    (c_token, new_collateral_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setCompRate` (0x6a491112) function"]
        pub fn set_comp_rate(
            &self,
            comp_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 73, 17, 18], comp_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setLiquidationIncentive` (0x4fd42e17) function"]
        pub fn set_liquidation_incentive(
            &self,
            new_liquidation_incentive_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([79, 212, 46, 23], new_liquidation_incentive_mantissa)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setMaxAssets` (0xd9226ced) function"]
        pub fn set_max_assets(
            &self,
            new_max_assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([217, 34, 108, 237], new_max_assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setMintPaused` (0x3bcf7ec1) function"]
        pub fn set_mint_paused(
            &self,
            c_token: ethers::core::types::Address,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 207, 126, 193], (c_token, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setPauseGuardian` (0x5f5af1aa) function"]
        pub fn set_pause_guardian(
            &self,
            new_pause_guardian: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 90, 241, 170], new_pause_guardian)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setPriceOracle` (0x55ee1fe1) function"]
        pub fn set_price_oracle(
            &self,
            new_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([85, 238, 31, 225], new_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setSeizePaused` (0x2d70db78) function"]
        pub fn set_seize_paused(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 112, 219, 120], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setTransferPaused` (0x8ebf6364) function"]
        pub fn set_transfer_paused(
            &self,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 191, 99, 100], state)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_supportMarket` (0xa76b3fda) function"]
        pub fn support_market(
            &self,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([167, 107, 63, 218], c_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accountAssets` (0xdce15449) function"]
        pub fn account_assets(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 225, 84, 73], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allMarkets` (0x52d84d1e) function"]
        pub fn all_markets(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([82, 216, 77, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowAllowed` (0xda3d454c) function"]
        pub fn borrow_allowed(
            &self,
            c_token: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            borrow_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([218, 61, 69, 76], (c_token, borrower, borrow_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowGuardianPaused` (0x6d154ea5) function"]
        pub fn borrow_guardian_paused(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 21, 78, 165], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowVerify` (0x5c778605) function"]
        pub fn borrow_verify(
            &self,
            c_token: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            borrow_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 119, 134, 5], (c_token, borrower, borrow_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkMembership` (0x929fe9a1) function"]
        pub fn check_membership(
            &self,
            account: ethers::core::types::Address,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 159, 233, 161], (account, c_token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimComp` (0x1c3db2e0) function"]
        pub fn claim_comp_with_c_tokens(
            &self,
            holder: ethers::core::types::Address,
            c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 61, 178, 224], (holder, c_tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimComp` (0x6810dfa6) function"]
        pub fn claim_comp_with_holders_and_c_tokens_and_borrowers_and_suppliers(
            &self,
            holders: ::std::vec::Vec<ethers::core::types::Address>,
            c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            borrowers: bool,
            suppliers: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [104, 16, 223, 166],
                    (holders, c_tokens, borrowers, suppliers),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimComp` (0xe9af0292) function"]
        pub fn claim_comp(
            &self,
            holder: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 175, 2, 146], holder)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeFactorMantissa` (0xe8755446) function"]
        pub fn close_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([232, 117, 84, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compAccrued` (0xcc7ebdc4) function"]
        pub fn comp_accrued(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([204, 126, 189, 196], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compBorrowState` (0x8c57804e) function"]
        pub fn comp_borrow_state(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([140, 87, 128, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compBorrowerIndex` (0xca0af043) function"]
        pub fn comp_borrower_index(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([202, 10, 240, 67], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compClaimThreshold` (0x747026c9) function"]
        pub fn comp_claim_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([116, 112, 38, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compInitialIndex` (0xa7f0e231) function"]
        pub fn comp_initial_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([167, 240, 226, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compRate` (0xaa900754) function"]
        pub fn comp_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 144, 7, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSpeeds` (0x1d7b33d7) function"]
        pub fn comp_speeds(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 123, 51, 215], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSupplierIndex` (0xb21be7fd) function"]
        pub fn comp_supplier_index(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([178, 27, 231, 253], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSupplyState` (0x6b79c38d) function"]
        pub fn comp_supply_state(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([107, 121, 195, 141], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `comptrollerImplementation` (0xbb82aa5e) function"]
        pub fn comptroller_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([187, 130, 170, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enterMarkets` (0xc2998238) function"]
        pub fn enter_markets(
            &self,
            c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([194, 153, 130, 56], c_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitMarket` (0xede4edd0) function"]
        pub fn exit_market(
            &self,
            c_token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([237, 228, 237, 208], c_token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountLiquidity` (0x5ec88c79) function"]
        pub fn get_account_liquidity(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([94, 200, 140, 121], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAllMarkets` (0xb0772d0b) function"]
        pub fn get_all_markets(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([176, 119, 45, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsIn` (0xabfceffc) function"]
        pub fn get_assets_in(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([171, 252, 239, 252], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockNumber` (0x42cbb15c) function"]
        pub fn get_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCompAddress` (0x9d1b5a0a) function"]
        pub fn get_comp_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([157, 27, 90, 10], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getHypotheticalAccountLiquidity` (0x4e79238f) function"]
        pub fn get_hypothetical_account_liquidity(
            &self,
            account: ethers::core::types::Address,
            c_token_modify: ethers::core::types::Address,
            redeem_tokens: ethers::core::types::U256,
            borrow_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [78, 121, 35, 143],
                    (account, c_token_modify, redeem_tokens, borrow_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isComptroller` (0x007e3dd2) function"]
        pub fn is_comptroller(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([0, 126, 61, 210], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateBorrowAllowed` (0x5fc7e71e) function"]
        pub fn liquidate_borrow_allowed(
            &self,
            c_token_borrowed: ethers::core::types::Address,
            c_token_collateral: ethers::core::types::Address,
            liquidator: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [95, 199, 231, 30],
                    (
                        c_token_borrowed,
                        c_token_collateral,
                        liquidator,
                        borrower,
                        repay_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateBorrowVerify` (0x47ef3b3b) function"]
        pub fn liquidate_borrow_verify(
            &self,
            c_token_borrowed: ethers::core::types::Address,
            c_token_collateral: ethers::core::types::Address,
            liquidator: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            actual_repay_amount: ethers::core::types::U256,
            seize_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [71, 239, 59, 59],
                    (
                        c_token_borrowed,
                        c_token_collateral,
                        liquidator,
                        borrower,
                        actual_repay_amount,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateCalculateSeizeTokens` (0xc488847b) function"]
        pub fn liquidate_calculate_seize_tokens(
            &self,
            c_token_borrowed: ethers::core::types::Address,
            c_token_collateral: ethers::core::types::Address,
            actual_repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [196, 136, 132, 123],
                    (c_token_borrowed, c_token_collateral, actual_repay_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationIncentiveMantissa` (0x4ada90af) function"]
        pub fn liquidation_incentive_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([74, 218, 144, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `markets` (0x8e8f294b) function"]
        pub fn markets(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash([142, 143, 41, 75], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxAssets` (0x94b2294b) function"]
        pub fn max_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 178, 41, 75], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintAllowed` (0x4ef4c3e1) function"]
        pub fn mint_allowed(
            &self,
            c_token: ethers::core::types::Address,
            minter: ethers::core::types::Address,
            mint_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([78, 244, 195, 225], (c_token, minter, mint_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintGuardianPaused` (0x731f0c2b) function"]
        pub fn mint_guardian_paused(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 31, 12, 43], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintVerify` (0x41c728b9) function"]
        pub fn mint_verify(
            &self,
            c_token: ethers::core::types::Address,
            minter: ethers::core::types::Address,
            actual_mint_amount: ethers::core::types::U256,
            mint_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [65, 199, 40, 185],
                    (c_token, minter, actual_mint_amount, mint_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pauseGuardian` (0x24a3d622) function"]
        pub fn pause_guardian(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 163, 214, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingAdmin` (0x26782247) function"]
        pub fn pending_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingComptrollerImplementation` (0xdcfbc0c7) function"]
        pub fn pending_comptroller_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 251, 192, 199], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemAllowed` (0xeabe7d91) function"]
        pub fn redeem_allowed(
            &self,
            c_token: ethers::core::types::Address,
            redeemer: ethers::core::types::Address,
            redeem_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([234, 190, 125, 145], (c_token, redeemer, redeem_tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemVerify` (0x51dff989) function"]
        pub fn redeem_verify(
            &self,
            c_token: ethers::core::types::Address,
            redeemer: ethers::core::types::Address,
            redeem_amount: ethers::core::types::U256,
            redeem_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [81, 223, 249, 137],
                    (c_token, redeemer, redeem_amount, redeem_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refreshCompSpeeds` (0x4d8e5037) function"]
        pub fn refresh_comp_speeds(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 142, 80, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrowAllowed` (0x24008a62) function"]
        pub fn repay_borrow_allowed(
            &self,
            c_token: ethers::core::types::Address,
            payer: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([36, 0, 138, 98], (c_token, payer, borrower, repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrowVerify` (0x1ededc91) function"]
        pub fn repay_borrow_verify(
            &self,
            c_token: ethers::core::types::Address,
            payer: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            actual_repay_amount: ethers::core::types::U256,
            borrower_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 222, 220, 145],
                    (
                        c_token,
                        payer,
                        borrower,
                        actual_repay_amount,
                        borrower_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seizeAllowed` (0xd02f7351) function"]
        pub fn seize_allowed(
            &self,
            c_token_collateral: ethers::core::types::Address,
            c_token_borrowed: ethers::core::types::Address,
            liquidator: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            seize_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [208, 47, 115, 81],
                    (
                        c_token_collateral,
                        c_token_borrowed,
                        liquidator,
                        borrower,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seizeGuardianPaused` (0xac0b0bb7) function"]
        pub fn seize_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 11, 11, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seizeVerify` (0x6d35bf91) function"]
        pub fn seize_verify(
            &self,
            c_token_collateral: ethers::core::types::Address,
            c_token_borrowed: ethers::core::types::Address,
            liquidator: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            seize_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [109, 53, 191, 145],
                    (
                        c_token_collateral,
                        c_token_borrowed,
                        liquidator,
                        borrower,
                        seize_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferAllowed` (0xbdcdc258) function"]
        pub fn transfer_allowed(
            &self,
            c_token: ethers::core::types::Address,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            transfer_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([189, 205, 194, 88], (c_token, src, dst, transfer_tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferGuardianPaused` (0x87f76303) function"]
        pub fn transfer_guardian_paused(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 247, 99, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferVerify` (0x6a56947e) function"]
        pub fn transfer_verify(
            &self,
            c_token: ethers::core::types::Address,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            transfer_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 86, 148, 126], (c_token, src, dst, transfer_tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ActionPaused` event"]
        pub fn action_paused_1_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ActionPaused1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ActionPaused` event"]
        pub fn action_paused_2_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ActionPaused2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CompSpeedUpdated` event"]
        pub fn comp_speed_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CompSpeedUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DistributedBorrowerComp` event"]
        pub fn distributed_borrower_comp_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DistributedBorrowerCompFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DistributedSupplierComp` event"]
        pub fn distributed_supplier_comp_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DistributedSupplierCompFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Failure` event"]
        pub fn failure_filter(&self) -> ethers::contract::builders::Event<M, FailureFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketComped` event"]
        pub fn market_comped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketCompedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketEntered` event"]
        pub fn market_entered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketEnteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketExited` event"]
        pub fn market_exited_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketExitedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketListed` event"]
        pub fn market_listed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketListedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCloseFactor` event"]
        pub fn new_close_factor_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCloseFactorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCollateralFactor` event"]
        pub fn new_collateral_factor_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCollateralFactorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCompRate` event"]
        pub fn new_comp_rate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCompRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewLiquidationIncentive` event"]
        pub fn new_liquidation_incentive_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewLiquidationIncentiveFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewMaxAssets` event"]
        pub fn new_max_assets_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewMaxAssetsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPauseGuardian` event"]
        pub fn new_pause_guardian_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPauseGuardianFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPriceOracle` event"]
        pub fn new_price_oracle_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPriceOracleFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ComptrollerG3Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ComptrollerG3<M> {
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
    #[ethevent(name = "ActionPaused", abi = "ActionPaused(string,bool)")]
    pub struct ActionPaused1Filter {
        pub action: String,
        pub pause_state: bool,
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
    #[ethevent(name = "ActionPaused", abi = "ActionPaused(address,string,bool)")]
    pub struct ActionPaused2Filter {
        pub c_token: ethers::core::types::Address,
        pub action: String,
        pub pause_state: bool,
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
    #[ethevent(name = "CompSpeedUpdated", abi = "CompSpeedUpdated(address,uint256)")]
    pub struct CompSpeedUpdatedFilter {
        #[ethevent(indexed)]
        pub c_token: ethers::core::types::Address,
        pub new_speed: ethers::core::types::U256,
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
        name = "DistributedBorrowerComp",
        abi = "DistributedBorrowerComp(address,address,uint256,uint256)"
    )]
    pub struct DistributedBorrowerCompFilter {
        #[ethevent(indexed)]
        pub c_token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        pub comp_delta: ethers::core::types::U256,
        pub comp_borrow_index: ethers::core::types::U256,
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
        name = "DistributedSupplierComp",
        abi = "DistributedSupplierComp(address,address,uint256,uint256)"
    )]
    pub struct DistributedSupplierCompFilter {
        #[ethevent(indexed)]
        pub c_token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub supplier: ethers::core::types::Address,
        pub comp_delta: ethers::core::types::U256,
        pub comp_supply_index: ethers::core::types::U256,
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
    #[ethevent(name = "Failure", abi = "Failure(uint256,uint256,uint256)")]
    pub struct FailureFilter {
        pub error: ethers::core::types::U256,
        pub info: ethers::core::types::U256,
        pub detail: ethers::core::types::U256,
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
    #[ethevent(name = "MarketComped", abi = "MarketComped(address,bool)")]
    pub struct MarketCompedFilter {
        pub c_token: ethers::core::types::Address,
        pub is_comped: bool,
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
    #[ethevent(name = "MarketEntered", abi = "MarketEntered(address,address)")]
    pub struct MarketEnteredFilter {
        pub c_token: ethers::core::types::Address,
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "MarketExited", abi = "MarketExited(address,address)")]
    pub struct MarketExitedFilter {
        pub c_token: ethers::core::types::Address,
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "MarketListed", abi = "MarketListed(address)")]
    pub struct MarketListedFilter {
        pub c_token: ethers::core::types::Address,
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
    #[ethevent(name = "NewCloseFactor", abi = "NewCloseFactor(uint256,uint256)")]
    pub struct NewCloseFactorFilter {
        pub old_close_factor_mantissa: ethers::core::types::U256,
        pub new_close_factor_mantissa: ethers::core::types::U256,
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
        name = "NewCollateralFactor",
        abi = "NewCollateralFactor(address,uint256,uint256)"
    )]
    pub struct NewCollateralFactorFilter {
        pub c_token: ethers::core::types::Address,
        pub old_collateral_factor_mantissa: ethers::core::types::U256,
        pub new_collateral_factor_mantissa: ethers::core::types::U256,
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
    #[ethevent(name = "NewCompRate", abi = "NewCompRate(uint256,uint256)")]
    pub struct NewCompRateFilter {
        pub old_comp_rate: ethers::core::types::U256,
        pub new_comp_rate: ethers::core::types::U256,
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
        name = "NewLiquidationIncentive",
        abi = "NewLiquidationIncentive(uint256,uint256)"
    )]
    pub struct NewLiquidationIncentiveFilter {
        pub old_liquidation_incentive_mantissa: ethers::core::types::U256,
        pub new_liquidation_incentive_mantissa: ethers::core::types::U256,
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
    #[ethevent(name = "NewMaxAssets", abi = "NewMaxAssets(uint256,uint256)")]
    pub struct NewMaxAssetsFilter {
        pub old_max_assets: ethers::core::types::U256,
        pub new_max_assets: ethers::core::types::U256,
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
    #[ethevent(name = "NewPauseGuardian", abi = "NewPauseGuardian(address,address)")]
    pub struct NewPauseGuardianFilter {
        pub old_pause_guardian: ethers::core::types::Address,
        pub new_pause_guardian: ethers::core::types::Address,
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
    #[ethevent(name = "NewPriceOracle", abi = "NewPriceOracle(address,address)")]
    pub struct NewPriceOracleFilter {
        pub old_price_oracle: ethers::core::types::Address,
        pub new_price_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ComptrollerG3Events {
        ActionPaused1Filter(ActionPaused1Filter),
        ActionPaused2Filter(ActionPaused2Filter),
        CompSpeedUpdatedFilter(CompSpeedUpdatedFilter),
        DistributedBorrowerCompFilter(DistributedBorrowerCompFilter),
        DistributedSupplierCompFilter(DistributedSupplierCompFilter),
        FailureFilter(FailureFilter),
        MarketCompedFilter(MarketCompedFilter),
        MarketEnteredFilter(MarketEnteredFilter),
        MarketExitedFilter(MarketExitedFilter),
        MarketListedFilter(MarketListedFilter),
        NewCloseFactorFilter(NewCloseFactorFilter),
        NewCollateralFactorFilter(NewCollateralFactorFilter),
        NewCompRateFilter(NewCompRateFilter),
        NewLiquidationIncentiveFilter(NewLiquidationIncentiveFilter),
        NewMaxAssetsFilter(NewMaxAssetsFilter),
        NewPauseGuardianFilter(NewPauseGuardianFilter),
        NewPriceOracleFilter(NewPriceOracleFilter),
    }
    impl ethers::contract::EthLogDecode for ComptrollerG3Events {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ActionPaused1Filter::decode_log(log) {
                return Ok(ComptrollerG3Events::ActionPaused1Filter(decoded));
            }
            if let Ok(decoded) = ActionPaused2Filter::decode_log(log) {
                return Ok(ComptrollerG3Events::ActionPaused2Filter(decoded));
            }
            if let Ok(decoded) = CompSpeedUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::CompSpeedUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DistributedBorrowerCompFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::DistributedBorrowerCompFilter(decoded));
            }
            if let Ok(decoded) = DistributedSupplierCompFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::DistributedSupplierCompFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::FailureFilter(decoded));
            }
            if let Ok(decoded) = MarketCompedFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::MarketCompedFilter(decoded));
            }
            if let Ok(decoded) = MarketEnteredFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::MarketEnteredFilter(decoded));
            }
            if let Ok(decoded) = MarketExitedFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::MarketExitedFilter(decoded));
            }
            if let Ok(decoded) = MarketListedFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::MarketListedFilter(decoded));
            }
            if let Ok(decoded) = NewCloseFactorFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewCloseFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCollateralFactorFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewCollateralFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCompRateFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewCompRateFilter(decoded));
            }
            if let Ok(decoded) = NewLiquidationIncentiveFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewLiquidationIncentiveFilter(decoded));
            }
            if let Ok(decoded) = NewMaxAssetsFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewMaxAssetsFilter(decoded));
            }
            if let Ok(decoded) = NewPauseGuardianFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewPauseGuardianFilter(decoded));
            }
            if let Ok(decoded) = NewPriceOracleFilter::decode_log(log) {
                return Ok(ComptrollerG3Events::NewPriceOracleFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ComptrollerG3Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerG3Events::ActionPaused1Filter(element) => element.fmt(f),
                ComptrollerG3Events::ActionPaused2Filter(element) => element.fmt(f),
                ComptrollerG3Events::CompSpeedUpdatedFilter(element) => element.fmt(f),
                ComptrollerG3Events::DistributedBorrowerCompFilter(element) => element.fmt(f),
                ComptrollerG3Events::DistributedSupplierCompFilter(element) => element.fmt(f),
                ComptrollerG3Events::FailureFilter(element) => element.fmt(f),
                ComptrollerG3Events::MarketCompedFilter(element) => element.fmt(f),
                ComptrollerG3Events::MarketEnteredFilter(element) => element.fmt(f),
                ComptrollerG3Events::MarketExitedFilter(element) => element.fmt(f),
                ComptrollerG3Events::MarketListedFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewCloseFactorFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewCollateralFactorFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewCompRateFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewLiquidationIncentiveFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewMaxAssetsFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewPauseGuardianFilter(element) => element.fmt(f),
                ComptrollerG3Events::NewPriceOracleFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_addCompMarkets` function with signature `_addCompMarkets(address[])` and selector `[206, 72, 92, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_addCompMarkets", abi = "_addCompMarkets(address[])")]
    pub struct AddCompMarketsCall {
        pub c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `_become` function with signature `_become(address,uint256,address[],address[])` and selector `[153, 44, 82, 148]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_become", abi = "_become(address,uint256,address[],address[])")]
    pub struct BecomeCall {
        pub unitroller: ethers::core::types::Address,
        pub comp_rate: ethers::core::types::U256,
        pub comp_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
        pub other_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `_becomeG3` function with signature `_becomeG3(uint256,address[],address[])` and selector `[35, 39, 199, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_becomeG3", abi = "_becomeG3(uint256,address[],address[])")]
    pub struct BecomeG3Call {
        pub comp_rate: ethers::core::types::U256,
        pub comp_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
        pub other_markets_to_add: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `_borrowGuardianPaused` function with signature `_borrowGuardianPaused()` and selector `[230, 101, 63, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_borrowGuardianPaused", abi = "_borrowGuardianPaused()")]
    pub struct _BorrowGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `_dropCompMarket` function with signature `_dropCompMarket(address)` and selector `[58, 167, 41, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_dropCompMarket", abi = "_dropCompMarket(address)")]
    pub struct DropCompMarketCall {
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_mintGuardianPaused` function with signature `_mintGuardianPaused()` and selector `[60, 148, 120, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_mintGuardianPaused", abi = "_mintGuardianPaused()")]
    pub struct _MintGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `_setBorrowPaused` function with signature `_setBorrowPaused(address,bool)` and selector `[24, 200, 130, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setBorrowPaused", abi = "_setBorrowPaused(address,bool)")]
    pub struct SetBorrowPausedCall {
        pub c_token: ethers::core::types::Address,
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `_setCloseFactor` function with signature `_setCloseFactor(uint256)` and selector `[49, 123, 11, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setCloseFactor", abi = "_setCloseFactor(uint256)")]
    pub struct SetCloseFactorCall {
        pub new_close_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_setCollateralFactor` function with signature `_setCollateralFactor(address,uint256)` and selector `[228, 2, 142, 238]`"]
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
        name = "_setCollateralFactor",
        abi = "_setCollateralFactor(address,uint256)"
    )]
    pub struct SetCollateralFactorCall {
        pub c_token: ethers::core::types::Address,
        pub new_collateral_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_setCompRate` function with signature `_setCompRate(uint256)` and selector `[106, 73, 17, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setCompRate", abi = "_setCompRate(uint256)")]
    pub struct SetCompRateCall {
        pub comp_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_setLiquidationIncentive` function with signature `_setLiquidationIncentive(uint256)` and selector `[79, 212, 46, 23]`"]
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
        name = "_setLiquidationIncentive",
        abi = "_setLiquidationIncentive(uint256)"
    )]
    pub struct SetLiquidationIncentiveCall {
        pub new_liquidation_incentive_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_setMaxAssets` function with signature `_setMaxAssets(uint256)` and selector `[217, 34, 108, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setMaxAssets", abi = "_setMaxAssets(uint256)")]
    pub struct SetMaxAssetsCall {
        pub new_max_assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_setMintPaused` function with signature `_setMintPaused(address,bool)` and selector `[59, 207, 126, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setMintPaused", abi = "_setMintPaused(address,bool)")]
    pub struct SetMintPausedCall {
        pub c_token: ethers::core::types::Address,
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `_setPauseGuardian` function with signature `_setPauseGuardian(address)` and selector `[95, 90, 241, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setPauseGuardian", abi = "_setPauseGuardian(address)")]
    pub struct SetPauseGuardianCall {
        pub new_pause_guardian: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setPriceOracle` function with signature `_setPriceOracle(address)` and selector `[85, 238, 31, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setPriceOracle", abi = "_setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub new_oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setSeizePaused` function with signature `_setSeizePaused(bool)` and selector `[45, 112, 219, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setSeizePaused", abi = "_setSeizePaused(bool)")]
    pub struct SetSeizePausedCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `_setTransferPaused` function with signature `_setTransferPaused(bool)` and selector `[142, 191, 99, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setTransferPaused", abi = "_setTransferPaused(bool)")]
    pub struct SetTransferPausedCall {
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `_supportMarket` function with signature `_supportMarket(address)` and selector `[167, 107, 63, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_supportMarket", abi = "_supportMarket(address)")]
    pub struct SupportMarketCall {
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `accountAssets` function with signature `accountAssets(address,uint256)` and selector `[220, 225, 84, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accountAssets", abi = "accountAssets(address,uint256)")]
    pub struct AccountAssetsCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `allMarkets` function with signature `allMarkets(uint256)` and selector `[82, 216, 77, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allMarkets", abi = "allMarkets(uint256)")]
    pub struct AllMarketsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `borrowAllowed` function with signature `borrowAllowed(address,address,uint256)` and selector `[218, 61, 69, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowAllowed", abi = "borrowAllowed(address,address,uint256)")]
    pub struct BorrowAllowedCall {
        pub c_token: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub borrow_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrowGuardianPaused` function with signature `borrowGuardianPaused(address)` and selector `[109, 21, 78, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowGuardianPaused", abi = "borrowGuardianPaused(address)")]
    pub struct BorrowGuardianPausedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `borrowVerify` function with signature `borrowVerify(address,address,uint256)` and selector `[92, 119, 134, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowVerify", abi = "borrowVerify(address,address,uint256)")]
    pub struct BorrowVerifyCall {
        pub c_token: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub borrow_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `checkMembership` function with signature `checkMembership(address,address)` and selector `[146, 159, 233, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "checkMembership", abi = "checkMembership(address,address)")]
    pub struct CheckMembershipCall {
        pub account: ethers::core::types::Address,
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimComp` function with signature `claimComp(address,address[])` and selector `[28, 61, 178, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimComp", abi = "claimComp(address,address[])")]
    pub struct ClaimCompWithCTokensCall {
        pub holder: ethers::core::types::Address,
        pub c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `claimComp` function with signature `claimComp(address[],address[],bool,bool)` and selector `[104, 16, 223, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimComp", abi = "claimComp(address[],address[],bool,bool)")]
    pub struct ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall {
        pub holders: ::std::vec::Vec<ethers::core::types::Address>,
        pub c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub borrowers: bool,
        pub suppliers: bool,
    }
    #[doc = "Container type for all input parameters for the `claimComp` function with signature `claimComp(address)` and selector `[233, 175, 2, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimComp", abi = "claimComp(address)")]
    pub struct ClaimCompCall {
        pub holder: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `closeFactorMantissa` function with signature `closeFactorMantissa()` and selector `[232, 117, 84, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "closeFactorMantissa", abi = "closeFactorMantissa()")]
    pub struct CloseFactorMantissaCall;
    #[doc = "Container type for all input parameters for the `compAccrued` function with signature `compAccrued(address)` and selector `[204, 126, 189, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compAccrued", abi = "compAccrued(address)")]
    pub struct CompAccruedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compBorrowState` function with signature `compBorrowState(address)` and selector `[140, 87, 128, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compBorrowState", abi = "compBorrowState(address)")]
    pub struct CompBorrowStateCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compBorrowerIndex` function with signature `compBorrowerIndex(address,address)` and selector `[202, 10, 240, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compBorrowerIndex", abi = "compBorrowerIndex(address,address)")]
    pub struct CompBorrowerIndexCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `compClaimThreshold` function with signature `compClaimThreshold()` and selector `[116, 112, 38, 201]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compClaimThreshold", abi = "compClaimThreshold()")]
    pub struct CompClaimThresholdCall;
    #[doc = "Container type for all input parameters for the `compInitialIndex` function with signature `compInitialIndex()` and selector `[167, 240, 226, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compInitialIndex", abi = "compInitialIndex()")]
    pub struct CompInitialIndexCall;
    #[doc = "Container type for all input parameters for the `compRate` function with signature `compRate()` and selector `[170, 144, 7, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compRate", abi = "compRate()")]
    pub struct CompRateCall;
    #[doc = "Container type for all input parameters for the `compSpeeds` function with signature `compSpeeds(address)` and selector `[29, 123, 51, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSpeeds", abi = "compSpeeds(address)")]
    pub struct CompSpeedsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compSupplierIndex` function with signature `compSupplierIndex(address,address)` and selector `[178, 27, 231, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSupplierIndex", abi = "compSupplierIndex(address,address)")]
    pub struct CompSupplierIndexCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `compSupplyState` function with signature `compSupplyState(address)` and selector `[107, 121, 195, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSupplyState", abi = "compSupplyState(address)")]
    pub struct CompSupplyStateCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `[187, 130, 170, 94]`"]
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
        name = "comptrollerImplementation",
        abi = "comptrollerImplementation()"
    )]
    pub struct ComptrollerImplementationCall;
    #[doc = "Container type for all input parameters for the `enterMarkets` function with signature `enterMarkets(address[])` and selector `[194, 153, 130, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "enterMarkets", abi = "enterMarkets(address[])")]
    pub struct EnterMarketsCall {
        pub c_tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `exitMarket` function with signature `exitMarket(address)` and selector `[237, 228, 237, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exitMarket", abi = "exitMarket(address)")]
    pub struct ExitMarketCall {
        pub c_token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAccountLiquidity` function with signature `getAccountLiquidity(address)` and selector `[94, 200, 140, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountLiquidity", abi = "getAccountLiquidity(address)")]
    pub struct GetAccountLiquidityCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAllMarkets` function with signature `getAllMarkets()` and selector `[176, 119, 45, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAllMarkets", abi = "getAllMarkets()")]
    pub struct GetAllMarketsCall;
    #[doc = "Container type for all input parameters for the `getAssetsIn` function with signature `getAssetsIn(address)` and selector `[171, 252, 239, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsIn", abi = "getAssetsIn(address)")]
    pub struct GetAssetsInCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    #[doc = "Container type for all input parameters for the `getCompAddress` function with signature `getCompAddress()` and selector `[157, 27, 90, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCompAddress", abi = "getCompAddress()")]
    pub struct GetCompAddressCall;
    #[doc = "Container type for all input parameters for the `getHypotheticalAccountLiquidity` function with signature `getHypotheticalAccountLiquidity(address,address,uint256,uint256)` and selector `[78, 121, 35, 143]`"]
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
        name = "getHypotheticalAccountLiquidity",
        abi = "getHypotheticalAccountLiquidity(address,address,uint256,uint256)"
    )]
    pub struct GetHypotheticalAccountLiquidityCall {
        pub account: ethers::core::types::Address,
        pub c_token_modify: ethers::core::types::Address,
        pub redeem_tokens: ethers::core::types::U256,
        pub borrow_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isComptroller` function with signature `isComptroller()` and selector `[0, 126, 61, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isComptroller", abi = "isComptroller()")]
    pub struct IsComptrollerCall;
    #[doc = "Container type for all input parameters for the `liquidateBorrowAllowed` function with signature `liquidateBorrowAllowed(address,address,address,address,uint256)` and selector `[95, 199, 231, 30]`"]
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
        name = "liquidateBorrowAllowed",
        abi = "liquidateBorrowAllowed(address,address,address,address,uint256)"
    )]
    pub struct LiquidateBorrowAllowedCall {
        pub c_token_borrowed: ethers::core::types::Address,
        pub c_token_collateral: ethers::core::types::Address,
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidateBorrowVerify` function with signature `liquidateBorrowVerify(address,address,address,address,uint256,uint256)` and selector `[71, 239, 59, 59]`"]
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
        name = "liquidateBorrowVerify",
        abi = "liquidateBorrowVerify(address,address,address,address,uint256,uint256)"
    )]
    pub struct LiquidateBorrowVerifyCall {
        pub c_token_borrowed: ethers::core::types::Address,
        pub c_token_collateral: ethers::core::types::Address,
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub actual_repay_amount: ethers::core::types::U256,
        pub seize_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidateCalculateSeizeTokens` function with signature `liquidateCalculateSeizeTokens(address,address,uint256)` and selector `[196, 136, 132, 123]`"]
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
        name = "liquidateCalculateSeizeTokens",
        abi = "liquidateCalculateSeizeTokens(address,address,uint256)"
    )]
    pub struct LiquidateCalculateSeizeTokensCall {
        pub c_token_borrowed: ethers::core::types::Address,
        pub c_token_collateral: ethers::core::types::Address,
        pub actual_repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `liquidationIncentiveMantissa` function with signature `liquidationIncentiveMantissa()` and selector `[74, 218, 144, 175]`"]
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
        name = "liquidationIncentiveMantissa",
        abi = "liquidationIncentiveMantissa()"
    )]
    pub struct LiquidationIncentiveMantissaCall;
    #[doc = "Container type for all input parameters for the `markets` function with signature `markets(address)` and selector `[142, 143, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "markets", abi = "markets(address)")]
    pub struct MarketsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `maxAssets` function with signature `maxAssets()` and selector `[148, 178, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxAssets", abi = "maxAssets()")]
    pub struct MaxAssetsCall;
    #[doc = "Container type for all input parameters for the `mintAllowed` function with signature `mintAllowed(address,address,uint256)` and selector `[78, 244, 195, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintAllowed", abi = "mintAllowed(address,address,uint256)")]
    pub struct MintAllowedCall {
        pub c_token: ethers::core::types::Address,
        pub minter: ethers::core::types::Address,
        pub mint_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintGuardianPaused` function with signature `mintGuardianPaused(address)` and selector `[115, 31, 12, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintGuardianPaused", abi = "mintGuardianPaused(address)")]
    pub struct MintGuardianPausedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `mintVerify` function with signature `mintVerify(address,address,uint256,uint256)` and selector `[65, 199, 40, 185]`"]
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
        name = "mintVerify",
        abi = "mintVerify(address,address,uint256,uint256)"
    )]
    pub struct MintVerifyCall {
        pub c_token: ethers::core::types::Address,
        pub minter: ethers::core::types::Address,
        pub actual_mint_amount: ethers::core::types::U256,
        pub mint_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    #[doc = "Container type for all input parameters for the `pauseGuardian` function with signature `pauseGuardian()` and selector `[36, 163, 214, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pauseGuardian", abi = "pauseGuardian()")]
    pub struct PauseGuardianCall;
    #[doc = "Container type for all input parameters for the `pendingAdmin` function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    #[doc = "Container type for all input parameters for the `pendingComptrollerImplementation` function with signature `pendingComptrollerImplementation()` and selector `[220, 251, 192, 199]`"]
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
        name = "pendingComptrollerImplementation",
        abi = "pendingComptrollerImplementation()"
    )]
    pub struct PendingComptrollerImplementationCall;
    #[doc = "Container type for all input parameters for the `redeemAllowed` function with signature `redeemAllowed(address,address,uint256)` and selector `[234, 190, 125, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "redeemAllowed", abi = "redeemAllowed(address,address,uint256)")]
    pub struct RedeemAllowedCall {
        pub c_token: ethers::core::types::Address,
        pub redeemer: ethers::core::types::Address,
        pub redeem_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `redeemVerify` function with signature `redeemVerify(address,address,uint256,uint256)` and selector `[81, 223, 249, 137]`"]
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
        name = "redeemVerify",
        abi = "redeemVerify(address,address,uint256,uint256)"
    )]
    pub struct RedeemVerifyCall {
        pub c_token: ethers::core::types::Address,
        pub redeemer: ethers::core::types::Address,
        pub redeem_amount: ethers::core::types::U256,
        pub redeem_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `refreshCompSpeeds` function with signature `refreshCompSpeeds()` and selector `[77, 142, 80, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "refreshCompSpeeds", abi = "refreshCompSpeeds()")]
    pub struct RefreshCompSpeedsCall;
    #[doc = "Container type for all input parameters for the `repayBorrowAllowed` function with signature `repayBorrowAllowed(address,address,address,uint256)` and selector `[36, 0, 138, 98]`"]
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
        name = "repayBorrowAllowed",
        abi = "repayBorrowAllowed(address,address,address,uint256)"
    )]
    pub struct RepayBorrowAllowedCall {
        pub c_token: ethers::core::types::Address,
        pub payer: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repayBorrowVerify` function with signature `repayBorrowVerify(address,address,address,uint256,uint256)` and selector `[30, 222, 220, 145]`"]
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
        name = "repayBorrowVerify",
        abi = "repayBorrowVerify(address,address,address,uint256,uint256)"
    )]
    pub struct RepayBorrowVerifyCall {
        pub c_token: ethers::core::types::Address,
        pub payer: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub actual_repay_amount: ethers::core::types::U256,
        pub borrower_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `seizeAllowed` function with signature `seizeAllowed(address,address,address,address,uint256)` and selector `[208, 47, 115, 81]`"]
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
        name = "seizeAllowed",
        abi = "seizeAllowed(address,address,address,address,uint256)"
    )]
    pub struct SeizeAllowedCall {
        pub c_token_collateral: ethers::core::types::Address,
        pub c_token_borrowed: ethers::core::types::Address,
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub seize_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `seizeGuardianPaused` function with signature `seizeGuardianPaused()` and selector `[172, 11, 11, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "seizeGuardianPaused", abi = "seizeGuardianPaused()")]
    pub struct SeizeGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `seizeVerify` function with signature `seizeVerify(address,address,address,address,uint256)` and selector `[109, 53, 191, 145]`"]
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
        name = "seizeVerify",
        abi = "seizeVerify(address,address,address,address,uint256)"
    )]
    pub struct SeizeVerifyCall {
        pub c_token_collateral: ethers::core::types::Address,
        pub c_token_borrowed: ethers::core::types::Address,
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub seize_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferAllowed` function with signature `transferAllowed(address,address,address,uint256)` and selector `[189, 205, 194, 88]`"]
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
        name = "transferAllowed",
        abi = "transferAllowed(address,address,address,uint256)"
    )]
    pub struct TransferAllowedCall {
        pub c_token: ethers::core::types::Address,
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub transfer_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferGuardianPaused` function with signature `transferGuardianPaused()` and selector `[135, 247, 99, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferGuardianPaused", abi = "transferGuardianPaused()")]
    pub struct TransferGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `transferVerify` function with signature `transferVerify(address,address,address,uint256)` and selector `[106, 86, 148, 126]`"]
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
        name = "transferVerify",
        abi = "transferVerify(address,address,address,uint256)"
    )]
    pub struct TransferVerifyCall {
        pub c_token: ethers::core::types::Address,
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub transfer_tokens: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ComptrollerG3Calls {
        AddCompMarkets(AddCompMarketsCall),
        Become(BecomeCall),
        BecomeG3(BecomeG3Call),
        _BorrowGuardianPaused(_BorrowGuardianPausedCall),
        DropCompMarket(DropCompMarketCall),
        _MintGuardianPaused(_MintGuardianPausedCall),
        SetBorrowPaused(SetBorrowPausedCall),
        SetCloseFactor(SetCloseFactorCall),
        SetCollateralFactor(SetCollateralFactorCall),
        SetCompRate(SetCompRateCall),
        SetLiquidationIncentive(SetLiquidationIncentiveCall),
        SetMaxAssets(SetMaxAssetsCall),
        SetMintPaused(SetMintPausedCall),
        SetPauseGuardian(SetPauseGuardianCall),
        SetPriceOracle(SetPriceOracleCall),
        SetSeizePaused(SetSeizePausedCall),
        SetTransferPaused(SetTransferPausedCall),
        SupportMarket(SupportMarketCall),
        AccountAssets(AccountAssetsCall),
        Admin(AdminCall),
        AllMarkets(AllMarketsCall),
        BorrowAllowed(BorrowAllowedCall),
        BorrowGuardianPaused(BorrowGuardianPausedCall),
        BorrowVerify(BorrowVerifyCall),
        CheckMembership(CheckMembershipCall),
        ClaimCompWithCTokens(ClaimCompWithCTokensCall),
        ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(
            ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall,
        ),
        ClaimComp(ClaimCompCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        CompAccrued(CompAccruedCall),
        CompBorrowState(CompBorrowStateCall),
        CompBorrowerIndex(CompBorrowerIndexCall),
        CompClaimThreshold(CompClaimThresholdCall),
        CompInitialIndex(CompInitialIndexCall),
        CompRate(CompRateCall),
        CompSpeeds(CompSpeedsCall),
        CompSupplierIndex(CompSupplierIndexCall),
        CompSupplyState(CompSupplyStateCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        EnterMarkets(EnterMarketsCall),
        ExitMarket(ExitMarketCall),
        GetAccountLiquidity(GetAccountLiquidityCall),
        GetAllMarkets(GetAllMarketsCall),
        GetAssetsIn(GetAssetsInCall),
        GetBlockNumber(GetBlockNumberCall),
        GetCompAddress(GetCompAddressCall),
        GetHypotheticalAccountLiquidity(GetHypotheticalAccountLiquidityCall),
        IsComptroller(IsComptrollerCall),
        LiquidateBorrowAllowed(LiquidateBorrowAllowedCall),
        LiquidateBorrowVerify(LiquidateBorrowVerifyCall),
        LiquidateCalculateSeizeTokens(LiquidateCalculateSeizeTokensCall),
        LiquidationIncentiveMantissa(LiquidationIncentiveMantissaCall),
        Markets(MarketsCall),
        MaxAssets(MaxAssetsCall),
        MintAllowed(MintAllowedCall),
        MintGuardianPaused(MintGuardianPausedCall),
        MintVerify(MintVerifyCall),
        Oracle(OracleCall),
        PauseGuardian(PauseGuardianCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
        RedeemAllowed(RedeemAllowedCall),
        RedeemVerify(RedeemVerifyCall),
        RefreshCompSpeeds(RefreshCompSpeedsCall),
        RepayBorrowAllowed(RepayBorrowAllowedCall),
        RepayBorrowVerify(RepayBorrowVerifyCall),
        SeizeAllowed(SeizeAllowedCall),
        SeizeGuardianPaused(SeizeGuardianPausedCall),
        SeizeVerify(SeizeVerifyCall),
        TransferAllowed(TransferAllowedCall),
        TransferGuardianPaused(TransferGuardianPausedCall),
        TransferVerify(TransferVerifyCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerG3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddCompMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::AddCompMarkets(decoded));
            }
            if let Ok(decoded) = <BecomeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::Become(decoded));
            }
            if let Ok(decoded) =
                <BecomeG3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::BecomeG3(decoded));
            }
            if let Ok(decoded) =
                <_BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <DropCompMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::DropCompMarket(decoded));
            }
            if let Ok(decoded) =
                <_MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetBorrowPaused(decoded));
            }
            if let Ok(decoded) =
                <SetCloseFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetCloseFactor(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetCollateralFactor(decoded));
            }
            if let Ok(decoded) =
                <SetCompRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetCompRate(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationIncentiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetLiquidationIncentive(decoded));
            }
            if let Ok(decoded) =
                <SetMaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetMaxAssets(decoded));
            }
            if let Ok(decoded) =
                <SetMintPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetMintPaused(decoded));
            }
            if let Ok(decoded) =
                <SetPauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetPauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetSeizePausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetSeizePaused(decoded));
            }
            if let Ok(decoded) =
                <SetTransferPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SetTransferPaused(decoded));
            }
            if let Ok(decoded) =
                <SupportMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SupportMarket(decoded));
            }
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::AllMarkets(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::BorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <BorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::BorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <CheckMembershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CheckMembership(decoded));
            }
            if let Ok(decoded) =
                <ClaimCompWithCTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::ClaimCompWithCTokens(decoded));
            }
            if let Ok (decoded) = < ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ComptrollerG3Calls :: ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers (decoded)) }
            if let Ok(decoded) =
                <ClaimCompCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::ClaimComp(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <CompAccruedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompAccrued(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompBorrowState(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowerIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompBorrowerIndex(decoded));
            }
            if let Ok(decoded) =
                <CompClaimThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompClaimThreshold(decoded));
            }
            if let Ok(decoded) =
                <CompInitialIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompInitialIndex(decoded));
            }
            if let Ok(decoded) =
                <CompRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompRate(decoded));
            }
            if let Ok(decoded) =
                <CompSpeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompSpeeds(decoded));
            }
            if let Ok(decoded) =
                <CompSupplierIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompSupplierIndex(decoded));
            }
            if let Ok(decoded) =
                <CompSupplyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::CompSupplyState(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG3Calls::ComptrollerImplementation(decoded));
            }
            if let Ok(decoded) =
                <EnterMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::EnterMarkets(decoded));
            }
            if let Ok(decoded) =
                <ExitMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::ExitMarket(decoded));
            }
            if let Ok(decoded) =
                <GetAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::GetAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetAllMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::GetAllMarkets(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::GetAssetsIn(decoded));
            }
            if let Ok(decoded) =
                <GetBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::GetBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetCompAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::GetCompAddress(decoded));
            }
            if let Ok(decoded) =
                <GetHypotheticalAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG3Calls::GetHypotheticalAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <IsComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::IsComptroller(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::LiquidateBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::LiquidateBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCalculateSeizeTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG3Calls::LiquidateCalculateSeizeTokens(decoded));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG3Calls::LiquidationIncentiveMantissa(decoded));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::Markets(decoded));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::MaxAssets(decoded));
            }
            if let Ok(decoded) =
                <MintAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::MintAllowed(decoded));
            }
            if let Ok(decoded) =
                <MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <MintVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::MintVerify(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::PauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG3Calls::PendingComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RedeemAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::RedeemAllowed(decoded));
            }
            if let Ok(decoded) =
                <RedeemVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::RedeemVerify(decoded));
            }
            if let Ok(decoded) =
                <RefreshCompSpeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::RefreshCompSpeeds(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::RepayBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::RepayBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <SeizeAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SeizeAllowed(decoded));
            }
            if let Ok(decoded) =
                <SeizeGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SeizeVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::SeizeVerify(decoded));
            }
            if let Ok(decoded) =
                <TransferAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::TransferAllowed(decoded));
            }
            if let Ok(decoded) =
                <TransferGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::TransferGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <TransferVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG3Calls::TransferVerify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerG3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerG3Calls::AddCompMarkets(element) => element.encode(),
                ComptrollerG3Calls::Become(element) => element.encode(),
                ComptrollerG3Calls::BecomeG3(element) => element.encode(),
                ComptrollerG3Calls::_BorrowGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::DropCompMarket(element) => element.encode(),
                ComptrollerG3Calls::_MintGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::SetBorrowPaused(element) => element.encode(),
                ComptrollerG3Calls::SetCloseFactor(element) => element.encode(),
                ComptrollerG3Calls::SetCollateralFactor(element) => element.encode(),
                ComptrollerG3Calls::SetCompRate(element) => element.encode(),
                ComptrollerG3Calls::SetLiquidationIncentive(element) => element.encode(),
                ComptrollerG3Calls::SetMaxAssets(element) => element.encode(),
                ComptrollerG3Calls::SetMintPaused(element) => element.encode(),
                ComptrollerG3Calls::SetPauseGuardian(element) => element.encode(),
                ComptrollerG3Calls::SetPriceOracle(element) => element.encode(),
                ComptrollerG3Calls::SetSeizePaused(element) => element.encode(),
                ComptrollerG3Calls::SetTransferPaused(element) => element.encode(),
                ComptrollerG3Calls::SupportMarket(element) => element.encode(),
                ComptrollerG3Calls::AccountAssets(element) => element.encode(),
                ComptrollerG3Calls::Admin(element) => element.encode(),
                ComptrollerG3Calls::AllMarkets(element) => element.encode(),
                ComptrollerG3Calls::BorrowAllowed(element) => element.encode(),
                ComptrollerG3Calls::BorrowGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::BorrowVerify(element) => element.encode(),
                ComptrollerG3Calls::CheckMembership(element) => element.encode(),
                ComptrollerG3Calls::ClaimCompWithCTokens(element) => element.encode(),
                ComptrollerG3Calls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(
                    element,
                ) => element.encode(),
                ComptrollerG3Calls::ClaimComp(element) => element.encode(),
                ComptrollerG3Calls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerG3Calls::CompAccrued(element) => element.encode(),
                ComptrollerG3Calls::CompBorrowState(element) => element.encode(),
                ComptrollerG3Calls::CompBorrowerIndex(element) => element.encode(),
                ComptrollerG3Calls::CompClaimThreshold(element) => element.encode(),
                ComptrollerG3Calls::CompInitialIndex(element) => element.encode(),
                ComptrollerG3Calls::CompRate(element) => element.encode(),
                ComptrollerG3Calls::CompSpeeds(element) => element.encode(),
                ComptrollerG3Calls::CompSupplierIndex(element) => element.encode(),
                ComptrollerG3Calls::CompSupplyState(element) => element.encode(),
                ComptrollerG3Calls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerG3Calls::EnterMarkets(element) => element.encode(),
                ComptrollerG3Calls::ExitMarket(element) => element.encode(),
                ComptrollerG3Calls::GetAccountLiquidity(element) => element.encode(),
                ComptrollerG3Calls::GetAllMarkets(element) => element.encode(),
                ComptrollerG3Calls::GetAssetsIn(element) => element.encode(),
                ComptrollerG3Calls::GetBlockNumber(element) => element.encode(),
                ComptrollerG3Calls::GetCompAddress(element) => element.encode(),
                ComptrollerG3Calls::GetHypotheticalAccountLiquidity(element) => element.encode(),
                ComptrollerG3Calls::IsComptroller(element) => element.encode(),
                ComptrollerG3Calls::LiquidateBorrowAllowed(element) => element.encode(),
                ComptrollerG3Calls::LiquidateBorrowVerify(element) => element.encode(),
                ComptrollerG3Calls::LiquidateCalculateSeizeTokens(element) => element.encode(),
                ComptrollerG3Calls::LiquidationIncentiveMantissa(element) => element.encode(),
                ComptrollerG3Calls::Markets(element) => element.encode(),
                ComptrollerG3Calls::MaxAssets(element) => element.encode(),
                ComptrollerG3Calls::MintAllowed(element) => element.encode(),
                ComptrollerG3Calls::MintGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::MintVerify(element) => element.encode(),
                ComptrollerG3Calls::Oracle(element) => element.encode(),
                ComptrollerG3Calls::PauseGuardian(element) => element.encode(),
                ComptrollerG3Calls::PendingAdmin(element) => element.encode(),
                ComptrollerG3Calls::PendingComptrollerImplementation(element) => element.encode(),
                ComptrollerG3Calls::RedeemAllowed(element) => element.encode(),
                ComptrollerG3Calls::RedeemVerify(element) => element.encode(),
                ComptrollerG3Calls::RefreshCompSpeeds(element) => element.encode(),
                ComptrollerG3Calls::RepayBorrowAllowed(element) => element.encode(),
                ComptrollerG3Calls::RepayBorrowVerify(element) => element.encode(),
                ComptrollerG3Calls::SeizeAllowed(element) => element.encode(),
                ComptrollerG3Calls::SeizeGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::SeizeVerify(element) => element.encode(),
                ComptrollerG3Calls::TransferAllowed(element) => element.encode(),
                ComptrollerG3Calls::TransferGuardianPaused(element) => element.encode(),
                ComptrollerG3Calls::TransferVerify(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerG3Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerG3Calls::AddCompMarkets(element) => element.fmt(f),
                ComptrollerG3Calls::Become(element) => element.fmt(f),
                ComptrollerG3Calls::BecomeG3(element) => element.fmt(f),
                ComptrollerG3Calls::_BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::DropCompMarket(element) => element.fmt(f),
                ComptrollerG3Calls::_MintGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::SetBorrowPaused(element) => element.fmt(f),
                ComptrollerG3Calls::SetCloseFactor(element) => element.fmt(f),
                ComptrollerG3Calls::SetCollateralFactor(element) => element.fmt(f),
                ComptrollerG3Calls::SetCompRate(element) => element.fmt(f),
                ComptrollerG3Calls::SetLiquidationIncentive(element) => element.fmt(f),
                ComptrollerG3Calls::SetMaxAssets(element) => element.fmt(f),
                ComptrollerG3Calls::SetMintPaused(element) => element.fmt(f),
                ComptrollerG3Calls::SetPauseGuardian(element) => element.fmt(f),
                ComptrollerG3Calls::SetPriceOracle(element) => element.fmt(f),
                ComptrollerG3Calls::SetSeizePaused(element) => element.fmt(f),
                ComptrollerG3Calls::SetTransferPaused(element) => element.fmt(f),
                ComptrollerG3Calls::SupportMarket(element) => element.fmt(f),
                ComptrollerG3Calls::AccountAssets(element) => element.fmt(f),
                ComptrollerG3Calls::Admin(element) => element.fmt(f),
                ComptrollerG3Calls::AllMarkets(element) => element.fmt(f),
                ComptrollerG3Calls::BorrowAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::BorrowVerify(element) => element.fmt(f),
                ComptrollerG3Calls::CheckMembership(element) => element.fmt(f),
                ComptrollerG3Calls::ClaimCompWithCTokens(element) => element.fmt(f),
                ComptrollerG3Calls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(
                    element,
                ) => element.fmt(f),
                ComptrollerG3Calls::ClaimComp(element) => element.fmt(f),
                ComptrollerG3Calls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerG3Calls::CompAccrued(element) => element.fmt(f),
                ComptrollerG3Calls::CompBorrowState(element) => element.fmt(f),
                ComptrollerG3Calls::CompBorrowerIndex(element) => element.fmt(f),
                ComptrollerG3Calls::CompClaimThreshold(element) => element.fmt(f),
                ComptrollerG3Calls::CompInitialIndex(element) => element.fmt(f),
                ComptrollerG3Calls::CompRate(element) => element.fmt(f),
                ComptrollerG3Calls::CompSpeeds(element) => element.fmt(f),
                ComptrollerG3Calls::CompSupplierIndex(element) => element.fmt(f),
                ComptrollerG3Calls::CompSupplyState(element) => element.fmt(f),
                ComptrollerG3Calls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerG3Calls::EnterMarkets(element) => element.fmt(f),
                ComptrollerG3Calls::ExitMarket(element) => element.fmt(f),
                ComptrollerG3Calls::GetAccountLiquidity(element) => element.fmt(f),
                ComptrollerG3Calls::GetAllMarkets(element) => element.fmt(f),
                ComptrollerG3Calls::GetAssetsIn(element) => element.fmt(f),
                ComptrollerG3Calls::GetBlockNumber(element) => element.fmt(f),
                ComptrollerG3Calls::GetCompAddress(element) => element.fmt(f),
                ComptrollerG3Calls::GetHypotheticalAccountLiquidity(element) => element.fmt(f),
                ComptrollerG3Calls::IsComptroller(element) => element.fmt(f),
                ComptrollerG3Calls::LiquidateBorrowAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::LiquidateBorrowVerify(element) => element.fmt(f),
                ComptrollerG3Calls::LiquidateCalculateSeizeTokens(element) => element.fmt(f),
                ComptrollerG3Calls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerG3Calls::Markets(element) => element.fmt(f),
                ComptrollerG3Calls::MaxAssets(element) => element.fmt(f),
                ComptrollerG3Calls::MintAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::MintGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::MintVerify(element) => element.fmt(f),
                ComptrollerG3Calls::Oracle(element) => element.fmt(f),
                ComptrollerG3Calls::PauseGuardian(element) => element.fmt(f),
                ComptrollerG3Calls::PendingAdmin(element) => element.fmt(f),
                ComptrollerG3Calls::PendingComptrollerImplementation(element) => element.fmt(f),
                ComptrollerG3Calls::RedeemAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::RedeemVerify(element) => element.fmt(f),
                ComptrollerG3Calls::RefreshCompSpeeds(element) => element.fmt(f),
                ComptrollerG3Calls::RepayBorrowAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::RepayBorrowVerify(element) => element.fmt(f),
                ComptrollerG3Calls::SeizeAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::SeizeGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::SeizeVerify(element) => element.fmt(f),
                ComptrollerG3Calls::TransferAllowed(element) => element.fmt(f),
                ComptrollerG3Calls::TransferGuardianPaused(element) => element.fmt(f),
                ComptrollerG3Calls::TransferVerify(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddCompMarketsCall> for ComptrollerG3Calls {
        fn from(var: AddCompMarketsCall) -> Self {
            ComptrollerG3Calls::AddCompMarkets(var)
        }
    }
    impl ::std::convert::From<BecomeCall> for ComptrollerG3Calls {
        fn from(var: BecomeCall) -> Self {
            ComptrollerG3Calls::Become(var)
        }
    }
    impl ::std::convert::From<BecomeG3Call> for ComptrollerG3Calls {
        fn from(var: BecomeG3Call) -> Self {
            ComptrollerG3Calls::BecomeG3(var)
        }
    }
    impl ::std::convert::From<_BorrowGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: _BorrowGuardianPausedCall) -> Self {
            ComptrollerG3Calls::_BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<DropCompMarketCall> for ComptrollerG3Calls {
        fn from(var: DropCompMarketCall) -> Self {
            ComptrollerG3Calls::DropCompMarket(var)
        }
    }
    impl ::std::convert::From<_MintGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: _MintGuardianPausedCall) -> Self {
            ComptrollerG3Calls::_MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SetBorrowPausedCall> for ComptrollerG3Calls {
        fn from(var: SetBorrowPausedCall) -> Self {
            ComptrollerG3Calls::SetBorrowPaused(var)
        }
    }
    impl ::std::convert::From<SetCloseFactorCall> for ComptrollerG3Calls {
        fn from(var: SetCloseFactorCall) -> Self {
            ComptrollerG3Calls::SetCloseFactor(var)
        }
    }
    impl ::std::convert::From<SetCollateralFactorCall> for ComptrollerG3Calls {
        fn from(var: SetCollateralFactorCall) -> Self {
            ComptrollerG3Calls::SetCollateralFactor(var)
        }
    }
    impl ::std::convert::From<SetCompRateCall> for ComptrollerG3Calls {
        fn from(var: SetCompRateCall) -> Self {
            ComptrollerG3Calls::SetCompRate(var)
        }
    }
    impl ::std::convert::From<SetLiquidationIncentiveCall> for ComptrollerG3Calls {
        fn from(var: SetLiquidationIncentiveCall) -> Self {
            ComptrollerG3Calls::SetLiquidationIncentive(var)
        }
    }
    impl ::std::convert::From<SetMaxAssetsCall> for ComptrollerG3Calls {
        fn from(var: SetMaxAssetsCall) -> Self {
            ComptrollerG3Calls::SetMaxAssets(var)
        }
    }
    impl ::std::convert::From<SetMintPausedCall> for ComptrollerG3Calls {
        fn from(var: SetMintPausedCall) -> Self {
            ComptrollerG3Calls::SetMintPaused(var)
        }
    }
    impl ::std::convert::From<SetPauseGuardianCall> for ComptrollerG3Calls {
        fn from(var: SetPauseGuardianCall) -> Self {
            ComptrollerG3Calls::SetPauseGuardian(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for ComptrollerG3Calls {
        fn from(var: SetPriceOracleCall) -> Self {
            ComptrollerG3Calls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetSeizePausedCall> for ComptrollerG3Calls {
        fn from(var: SetSeizePausedCall) -> Self {
            ComptrollerG3Calls::SetSeizePaused(var)
        }
    }
    impl ::std::convert::From<SetTransferPausedCall> for ComptrollerG3Calls {
        fn from(var: SetTransferPausedCall) -> Self {
            ComptrollerG3Calls::SetTransferPaused(var)
        }
    }
    impl ::std::convert::From<SupportMarketCall> for ComptrollerG3Calls {
        fn from(var: SupportMarketCall) -> Self {
            ComptrollerG3Calls::SupportMarket(var)
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerG3Calls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerG3Calls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerG3Calls {
        fn from(var: AdminCall) -> Self {
            ComptrollerG3Calls::Admin(var)
        }
    }
    impl ::std::convert::From<AllMarketsCall> for ComptrollerG3Calls {
        fn from(var: AllMarketsCall) -> Self {
            ComptrollerG3Calls::AllMarkets(var)
        }
    }
    impl ::std::convert::From<BorrowAllowedCall> for ComptrollerG3Calls {
        fn from(var: BorrowAllowedCall) -> Self {
            ComptrollerG3Calls::BorrowAllowed(var)
        }
    }
    impl ::std::convert::From<BorrowGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: BorrowGuardianPausedCall) -> Self {
            ComptrollerG3Calls::BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<BorrowVerifyCall> for ComptrollerG3Calls {
        fn from(var: BorrowVerifyCall) -> Self {
            ComptrollerG3Calls::BorrowVerify(var)
        }
    }
    impl ::std::convert::From<CheckMembershipCall> for ComptrollerG3Calls {
        fn from(var: CheckMembershipCall) -> Self {
            ComptrollerG3Calls::CheckMembership(var)
        }
    }
    impl ::std::convert::From<ClaimCompWithCTokensCall> for ComptrollerG3Calls {
        fn from(var: ClaimCompWithCTokensCall) -> Self {
            ComptrollerG3Calls::ClaimCompWithCTokens(var)
        }
    }
    impl ::std::convert::From<ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall>
        for ComptrollerG3Calls
    {
        fn from(var: ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall) -> Self {
            ComptrollerG3Calls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(var)
        }
    }
    impl ::std::convert::From<ClaimCompCall> for ComptrollerG3Calls {
        fn from(var: ClaimCompCall) -> Self {
            ComptrollerG3Calls::ClaimComp(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerG3Calls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerG3Calls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<CompAccruedCall> for ComptrollerG3Calls {
        fn from(var: CompAccruedCall) -> Self {
            ComptrollerG3Calls::CompAccrued(var)
        }
    }
    impl ::std::convert::From<CompBorrowStateCall> for ComptrollerG3Calls {
        fn from(var: CompBorrowStateCall) -> Self {
            ComptrollerG3Calls::CompBorrowState(var)
        }
    }
    impl ::std::convert::From<CompBorrowerIndexCall> for ComptrollerG3Calls {
        fn from(var: CompBorrowerIndexCall) -> Self {
            ComptrollerG3Calls::CompBorrowerIndex(var)
        }
    }
    impl ::std::convert::From<CompClaimThresholdCall> for ComptrollerG3Calls {
        fn from(var: CompClaimThresholdCall) -> Self {
            ComptrollerG3Calls::CompClaimThreshold(var)
        }
    }
    impl ::std::convert::From<CompInitialIndexCall> for ComptrollerG3Calls {
        fn from(var: CompInitialIndexCall) -> Self {
            ComptrollerG3Calls::CompInitialIndex(var)
        }
    }
    impl ::std::convert::From<CompRateCall> for ComptrollerG3Calls {
        fn from(var: CompRateCall) -> Self {
            ComptrollerG3Calls::CompRate(var)
        }
    }
    impl ::std::convert::From<CompSpeedsCall> for ComptrollerG3Calls {
        fn from(var: CompSpeedsCall) -> Self {
            ComptrollerG3Calls::CompSpeeds(var)
        }
    }
    impl ::std::convert::From<CompSupplierIndexCall> for ComptrollerG3Calls {
        fn from(var: CompSupplierIndexCall) -> Self {
            ComptrollerG3Calls::CompSupplierIndex(var)
        }
    }
    impl ::std::convert::From<CompSupplyStateCall> for ComptrollerG3Calls {
        fn from(var: CompSupplyStateCall) -> Self {
            ComptrollerG3Calls::CompSupplyState(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerG3Calls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerG3Calls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<EnterMarketsCall> for ComptrollerG3Calls {
        fn from(var: EnterMarketsCall) -> Self {
            ComptrollerG3Calls::EnterMarkets(var)
        }
    }
    impl ::std::convert::From<ExitMarketCall> for ComptrollerG3Calls {
        fn from(var: ExitMarketCall) -> Self {
            ComptrollerG3Calls::ExitMarket(var)
        }
    }
    impl ::std::convert::From<GetAccountLiquidityCall> for ComptrollerG3Calls {
        fn from(var: GetAccountLiquidityCall) -> Self {
            ComptrollerG3Calls::GetAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<GetAllMarketsCall> for ComptrollerG3Calls {
        fn from(var: GetAllMarketsCall) -> Self {
            ComptrollerG3Calls::GetAllMarkets(var)
        }
    }
    impl ::std::convert::From<GetAssetsInCall> for ComptrollerG3Calls {
        fn from(var: GetAssetsInCall) -> Self {
            ComptrollerG3Calls::GetAssetsIn(var)
        }
    }
    impl ::std::convert::From<GetBlockNumberCall> for ComptrollerG3Calls {
        fn from(var: GetBlockNumberCall) -> Self {
            ComptrollerG3Calls::GetBlockNumber(var)
        }
    }
    impl ::std::convert::From<GetCompAddressCall> for ComptrollerG3Calls {
        fn from(var: GetCompAddressCall) -> Self {
            ComptrollerG3Calls::GetCompAddress(var)
        }
    }
    impl ::std::convert::From<GetHypotheticalAccountLiquidityCall> for ComptrollerG3Calls {
        fn from(var: GetHypotheticalAccountLiquidityCall) -> Self {
            ComptrollerG3Calls::GetHypotheticalAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<IsComptrollerCall> for ComptrollerG3Calls {
        fn from(var: IsComptrollerCall) -> Self {
            ComptrollerG3Calls::IsComptroller(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowAllowedCall> for ComptrollerG3Calls {
        fn from(var: LiquidateBorrowAllowedCall) -> Self {
            ComptrollerG3Calls::LiquidateBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowVerifyCall> for ComptrollerG3Calls {
        fn from(var: LiquidateBorrowVerifyCall) -> Self {
            ComptrollerG3Calls::LiquidateBorrowVerify(var)
        }
    }
    impl ::std::convert::From<LiquidateCalculateSeizeTokensCall> for ComptrollerG3Calls {
        fn from(var: LiquidateCalculateSeizeTokensCall) -> Self {
            ComptrollerG3Calls::LiquidateCalculateSeizeTokens(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerG3Calls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerG3Calls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerG3Calls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerG3Calls::Markets(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerG3Calls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerG3Calls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<MintAllowedCall> for ComptrollerG3Calls {
        fn from(var: MintAllowedCall) -> Self {
            ComptrollerG3Calls::MintAllowed(var)
        }
    }
    impl ::std::convert::From<MintGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: MintGuardianPausedCall) -> Self {
            ComptrollerG3Calls::MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<MintVerifyCall> for ComptrollerG3Calls {
        fn from(var: MintVerifyCall) -> Self {
            ComptrollerG3Calls::MintVerify(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerG3Calls {
        fn from(var: OracleCall) -> Self {
            ComptrollerG3Calls::Oracle(var)
        }
    }
    impl ::std::convert::From<PauseGuardianCall> for ComptrollerG3Calls {
        fn from(var: PauseGuardianCall) -> Self {
            ComptrollerG3Calls::PauseGuardian(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerG3Calls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerG3Calls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerG3Calls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerG3Calls::PendingComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<RedeemAllowedCall> for ComptrollerG3Calls {
        fn from(var: RedeemAllowedCall) -> Self {
            ComptrollerG3Calls::RedeemAllowed(var)
        }
    }
    impl ::std::convert::From<RedeemVerifyCall> for ComptrollerG3Calls {
        fn from(var: RedeemVerifyCall) -> Self {
            ComptrollerG3Calls::RedeemVerify(var)
        }
    }
    impl ::std::convert::From<RefreshCompSpeedsCall> for ComptrollerG3Calls {
        fn from(var: RefreshCompSpeedsCall) -> Self {
            ComptrollerG3Calls::RefreshCompSpeeds(var)
        }
    }
    impl ::std::convert::From<RepayBorrowAllowedCall> for ComptrollerG3Calls {
        fn from(var: RepayBorrowAllowedCall) -> Self {
            ComptrollerG3Calls::RepayBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<RepayBorrowVerifyCall> for ComptrollerG3Calls {
        fn from(var: RepayBorrowVerifyCall) -> Self {
            ComptrollerG3Calls::RepayBorrowVerify(var)
        }
    }
    impl ::std::convert::From<SeizeAllowedCall> for ComptrollerG3Calls {
        fn from(var: SeizeAllowedCall) -> Self {
            ComptrollerG3Calls::SeizeAllowed(var)
        }
    }
    impl ::std::convert::From<SeizeGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: SeizeGuardianPausedCall) -> Self {
            ComptrollerG3Calls::SeizeGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SeizeVerifyCall> for ComptrollerG3Calls {
        fn from(var: SeizeVerifyCall) -> Self {
            ComptrollerG3Calls::SeizeVerify(var)
        }
    }
    impl ::std::convert::From<TransferAllowedCall> for ComptrollerG3Calls {
        fn from(var: TransferAllowedCall) -> Self {
            ComptrollerG3Calls::TransferAllowed(var)
        }
    }
    impl ::std::convert::From<TransferGuardianPausedCall> for ComptrollerG3Calls {
        fn from(var: TransferGuardianPausedCall) -> Self {
            ComptrollerG3Calls::TransferGuardianPaused(var)
        }
    }
    impl ::std::convert::From<TransferVerifyCall> for ComptrollerG3Calls {
        fn from(var: TransferVerifyCall) -> Self {
            ComptrollerG3Calls::TransferVerify(var)
        }
    }
    #[doc = "Container type for all return fields from the `_borrowGuardianPaused` function with signature `_borrowGuardianPaused()` and selector `[230, 101, 63, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct _BorrowGuardianPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_mintGuardianPaused` function with signature `_mintGuardianPaused()` and selector `[60, 148, 120, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct _MintGuardianPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_setBorrowPaused` function with signature `_setBorrowPaused(address,bool)` and selector `[24, 200, 130, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetBorrowPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_setCloseFactor` function with signature `_setCloseFactor(uint256)` and selector `[49, 123, 11, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetCloseFactorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setCollateralFactor` function with signature `_setCollateralFactor(address,uint256)` and selector `[228, 2, 142, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetCollateralFactorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setLiquidationIncentive` function with signature `_setLiquidationIncentive(uint256)` and selector `[79, 212, 46, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetLiquidationIncentiveReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setMaxAssets` function with signature `_setMaxAssets(uint256)` and selector `[217, 34, 108, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetMaxAssetsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setMintPaused` function with signature `_setMintPaused(address,bool)` and selector `[59, 207, 126, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetMintPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_setPauseGuardian` function with signature `_setPauseGuardian(address)` and selector `[95, 90, 241, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetPauseGuardianReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setPriceOracle` function with signature `_setPriceOracle(address)` and selector `[85, 238, 31, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetPriceOracleReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setSeizePaused` function with signature `_setSeizePaused(bool)` and selector `[45, 112, 219, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetSeizePausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_setTransferPaused` function with signature `_setTransferPaused(bool)` and selector `[142, 191, 99, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetTransferPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `_supportMarket` function with signature `_supportMarket(address)` and selector `[167, 107, 63, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportMarketReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `accountAssets` function with signature `accountAssets(address,uint256)` and selector `[220, 225, 84, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AccountAssetsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `allMarkets` function with signature `allMarkets(uint256)` and selector `[82, 216, 77, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllMarketsReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `borrowAllowed` function with signature `borrowAllowed(address,address,uint256)` and selector `[218, 61, 69, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowGuardianPaused` function with signature `borrowGuardianPaused(address)` and selector `[109, 21, 78, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowGuardianPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `checkMembership` function with signature `checkMembership(address,address)` and selector `[146, 159, 233, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CheckMembershipReturn(pub bool);
    #[doc = "Container type for all return fields from the `closeFactorMantissa` function with signature `closeFactorMantissa()` and selector `[232, 117, 84, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CloseFactorMantissaReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compAccrued` function with signature `compAccrued(address)` and selector `[204, 126, 189, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompAccruedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compBorrowState` function with signature `compBorrowState(address)` and selector `[140, 87, 128, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompBorrowStateReturn {
        pub index: ethers::core::types::U256,
        pub block: u32,
    }
    #[doc = "Container type for all return fields from the `compBorrowerIndex` function with signature `compBorrowerIndex(address,address)` and selector `[202, 10, 240, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompBorrowerIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compClaimThreshold` function with signature `compClaimThreshold()` and selector `[116, 112, 38, 201]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompClaimThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compInitialIndex` function with signature `compInitialIndex()` and selector `[167, 240, 226, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompInitialIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compRate` function with signature `compRate()` and selector `[170, 144, 7, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compSpeeds` function with signature `compSpeeds(address)` and selector `[29, 123, 51, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompSpeedsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compSupplierIndex` function with signature `compSupplierIndex(address,address)` and selector `[178, 27, 231, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompSupplierIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `compSupplyState` function with signature `compSupplyState(address)` and selector `[107, 121, 195, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CompSupplyStateReturn {
        pub index: ethers::core::types::U256,
        pub block: u32,
    }
    #[doc = "Container type for all return fields from the `comptrollerImplementation` function with signature `comptrollerImplementation()` and selector `[187, 130, 170, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ComptrollerImplementationReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `enterMarkets` function with signature `enterMarkets(address[])` and selector `[194, 153, 130, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EnterMarketsReturn(pub ::std::vec::Vec<ethers::core::types::U256>);
    #[doc = "Container type for all return fields from the `exitMarket` function with signature `exitMarket(address)` and selector `[237, 228, 237, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExitMarketReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAccountLiquidity` function with signature `getAccountLiquidity(address)` and selector `[94, 200, 140, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAccountLiquidityReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getAllMarkets` function with signature `getAllMarkets()` and selector `[176, 119, 45, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAllMarketsReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getAssetsIn` function with signature `getAssetsIn(address)` and selector `[171, 252, 239, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAssetsInReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBlockNumberReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getCompAddress` function with signature `getCompAddress()` and selector `[157, 27, 90, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCompAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getHypotheticalAccountLiquidity` function with signature `getHypotheticalAccountLiquidity(address,address,uint256,uint256)` and selector `[78, 121, 35, 143]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetHypotheticalAccountLiquidityReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `isComptroller` function with signature `isComptroller()` and selector `[0, 126, 61, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsComptrollerReturn(pub bool);
    #[doc = "Container type for all return fields from the `liquidateBorrowAllowed` function with signature `liquidateBorrowAllowed(address,address,address,address,uint256)` and selector `[95, 199, 231, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidateBorrowAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `liquidateCalculateSeizeTokens` function with signature `liquidateCalculateSeizeTokens(address,address,uint256)` and selector `[196, 136, 132, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidateCalculateSeizeTokensReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `liquidationIncentiveMantissa` function with signature `liquidationIncentiveMantissa()` and selector `[74, 218, 144, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidationIncentiveMantissaReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `markets` function with signature `markets(address)` and selector `[142, 143, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MarketsReturn {
        pub is_listed: bool,
        pub collateral_factor_mantissa: ethers::core::types::U256,
        pub is_comped: bool,
    }
    #[doc = "Container type for all return fields from the `maxAssets` function with signature `maxAssets()` and selector `[148, 178, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxAssetsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mintAllowed` function with signature `mintAllowed(address,address,uint256)` and selector `[78, 244, 195, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mintGuardianPaused` function with signature `mintGuardianPaused(address)` and selector `[115, 31, 12, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintGuardianPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `oracle` function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pauseGuardian` function with signature `pauseGuardian()` and selector `[36, 163, 214, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PauseGuardianReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pendingAdmin` function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PendingAdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pendingComptrollerImplementation` function with signature `pendingComptrollerImplementation()` and selector `[220, 251, 192, 199]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PendingComptrollerImplementationReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `redeemAllowed` function with signature `redeemAllowed(address,address,uint256)` and selector `[234, 190, 125, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RedeemAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repayBorrowAllowed` function with signature `repayBorrowAllowed(address,address,address,uint256)` and selector `[36, 0, 138, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayBorrowAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `seizeAllowed` function with signature `seizeAllowed(address,address,address,address,uint256)` and selector `[208, 47, 115, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SeizeAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `seizeGuardianPaused` function with signature `seizeGuardianPaused()` and selector `[172, 11, 11, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SeizeGuardianPausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferAllowed` function with signature `transferAllowed(address,address,address,uint256)` and selector `[189, 205, 194, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferAllowedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transferGuardianPaused` function with signature `transferGuardianPaused()` and selector `[135, 247, 99, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferGuardianPausedReturn(pub bool);
}
