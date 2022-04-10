pub use comptroller_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptroller_mod {
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
    #[doc = "Comptroller was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newSpeed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CompSpeedUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"compDelta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"compBorrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DistributedBorrowerComp\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"supplier\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"compDelta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"compSupplyIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DistributedSupplierComp\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketComped\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketEntered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketExited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketListed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCloseFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"oldCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCollateralFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldCompRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCompRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCompRate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLiquidationIncentive\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMaxAssets\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPauseGuardian\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"oldPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PriceOracle\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPriceOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addCompMarkets\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract Unitroller\",\"name\":\"unitroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_become\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_dropCompMarket\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setBorrowPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCloseFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCollateralFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"compRate_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCompRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setLiquidationIncentive\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMaxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMintPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPauseGuardian\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"newOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPriceOracle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setSeizePaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setTransferPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_supportMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allMarkets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkMembership\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"holders\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"contract CToken[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"borrowers\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"suppliers\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"holder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compAccrued\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowerIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compClaimThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compInitialIndex\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"\",\"type\":\"uint224\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSpeeds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplierIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplyState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterMarkets\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllMarkets\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsIn\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCompAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenModify\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHypotheticalAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isComptroller\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidateCalculateSeizeTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isListed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualMintAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintVerify\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauseGuardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemVerify\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"refreshCompSpeeds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowerIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"seizeGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transferGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferVerify\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b03191633179055615a4980620000336000396000f3fe608060405234801561001057600080fd5b506004361061041c5760003560e01c8063731f0c2b1161022b578063bdcdc25811610130578063dce15449116100b8578063e875544611610087578063e8755446146110ca578063e9af0292146110d2578063eabe7d91146110f8578063ede4edd01461112e578063f851a440146111545761041c565b8063dce1544914611062578063dcfbc0c71461108e578063e4028eee14611096578063e6653f3d146110c25761041c565b8063cc7ebdc4116100ff578063cc7ebdc414610f02578063ce485c5e14610f28578063d02f735114610fc9578063d9226ced1461100f578063da3d454c1461102c5761041c565b8063bdcdc25814610da8578063c299823814610de4578063c488847b14610e85578063ca0af04314610ed45761041c565b80639d1b5a0a116101b3578063abfceffc11610182578063abfceffc14610cec578063ac0b0bb714610d62578063b0772d0b14610d6a578063b21be7fd14610d72578063bb82aa5e14610da05761041c565b80639d1b5a0a14610c92578063a76b3fda14610c9a578063a7f0e23114610cc0578063aa90075414610ce45761041c565b80638c57804e116101fa5780638c57804e14610bcf5780638e8f294b14610bf55780638ebf636414610c3d578063929fe9a114610c5c57806394b2294b14610c8a5761041c565b8063731f0c2b14610b91578063747026c914610bb75780637dc0d1d014610bbf57806387f7630314610bc75761041c565b80634ada90af116103315780635ec88c79116102b95780636a491112116102885780636a49111214610a7e5780636a56947e14610a9b5780636b79c38d14610ad75780636d154ea514610b255780636d35bf9114610b4b5761041c565b80635ec88c79146108c05780635f5af1aa146108e65780635fc7e71e1461090c5780636810dfa6146109525761041c565b80634fd42e17116103005780634fd42e17146107ee57806351dff9891461080b57806352d84d1e1461084757806355ee1fe1146108645780635c7786051461088a5761041c565b80634ada90af1461074e5780634d8e5037146107565780634e79238f1461075e5780634ef4c3e1146107b85761041c565b806326782247116103b45780633bcf7ec1116103835780633bcf7ec1146106885780633c94786f146106b657806341c728b9146106be57806342cbb15c146106fa57806347ef3b3b146107025761041c565b8063267822471461061e5780632d70db7814610626578063317b0b77146106455780633aa729b4146106625761041c565b80631d7b33d7116103f05780631d7b33d7146105445780631ededc911461057c57806324008a62146105be57806324a3d622146105fa5761041c565b80627e3dd21461042157806318c882a51461043d5780631c3db2e01461046b5780631d504dc61461051e575b600080fd5b61042961115c565b604080519115158252519081900360200190f35b6104296004803603604081101561045357600080fd5b506001600160a01b0381351690602001351515611161565b61051c6004803603604081101561048157600080fd5b6001600160a01b038235169190810190604081016020820135600160201b8111156104ab57600080fd5b8201836020820111156104bd57600080fd5b803590602001918460208302840111600160201b831117156104de57600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550611301945050505050565b005b61051c6004803603602081101561053457600080fd5b50356001600160a01b0316611363565b61056a6004803603602081101561055a57600080fd5b50356001600160a01b03166114c2565b60408051918252519081900360200190f35b61051c600480360360a081101561059257600080fd5b506001600160a01b038135811691602081013582169160408201351690606081013590608001356114d4565b61056a600480360360808110156105d457600080fd5b506001600160a01b038135811691602081013582169160408201351690606001356114db565b6106026115a4565b604080516001600160a01b039092168252519081900360200190f35b6106026115b3565b6104296004803603602081101561063c57600080fd5b503515156115c2565b61056a6004803603602081101561065b57600080fd5b50356116fc565b61051c6004803603602081101561067857600080fd5b50356001600160a01b031661180d565b6104296004803603604081101561069e57600080fd5b506001600160a01b038135169060200135151561193e565b610429611ad9565b61051c600480360360808110156106d457600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135611ae9565b61056a611aef565b61051c600480360360c081101561071857600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060808101359060a00135611af4565b61056a611afc565b61051c611b02565b61079a6004803603608081101561077457600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135611b4a565b60408051938452602084019290925282820152519081900360600190f35b61056a600480360360608110156107ce57600080fd5b506001600160a01b03813581169160208101359091169060400135611b84565b61056a6004803603602081101561080457600080fd5b5035611c2f565b61051c6004803603608081101561082157600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135611d23565b6106026004803603602081101561085d57600080fd5b5035611d77565b61056a6004803603602081101561087a57600080fd5b50356001600160a01b0316611d9e565b61051c600480360360608110156108a057600080fd5b506001600160a01b03813581169160208101359091169060400135611e25565b61079a600480360360208110156108d657600080fd5b50356001600160a01b0316611e2a565b61056a600480360360208110156108fc57600080fd5b50356001600160a01b0316611e5f565b61056a600480360360a081101561092257600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060800135611ee3565b61051c6004803603608081101561096857600080fd5b810190602081018135600160201b81111561098257600080fd5b82018360208201111561099457600080fd5b803590602001918460208302840111600160201b831117156109b557600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295949360208101935035915050600160201b811115610a0457600080fd5b820183602082011115610a1657600080fd5b803590602001918460208302840111600160201b83111715610a3757600080fd5b91908080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525092955050505080351515915060200135151561206a565b61051c60048036036020811015610a9457600080fd5b5035612213565b61051c60048036036080811015610ab157600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135611ae9565b610afd60048036036020811015610aed57600080fd5b50356001600160a01b03166122b7565b604080516001600160e01b03909316835263ffffffff90911660208301528051918290030190f35b61042960048036036020811015610b3b57600080fd5b50356001600160a01b03166122e1565b61051c600480360360a0811015610b6157600080fd5b506001600160a01b03813581169160208101358216916040820135811691606081013590911690608001356114d4565b61042960048036036020811015610ba757600080fd5b50356001600160a01b03166122f6565b61056a61230b565b610602612316565b610429612325565b610afd60048036036020811015610be557600080fd5b50356001600160a01b0316612335565b610c1b60048036036020811015610c0b57600080fd5b50356001600160a01b031661235f565b6040805193151584526020840192909252151582820152519081900360600190f35b61042960048036036020811015610c5357600080fd5b50351515612385565b61042960048036036040811015610c7257600080fd5b506001600160a01b03813581169160200135166124be565b61056a6124f1565b6106026124f7565b61056a60048036036020811015610cb057600080fd5b50356001600160a01b031661250f565b610cc861266c565b604080516001600160e01b039092168252519081900360200190f35b61056a61267f565b610d1260048036036020811015610d0257600080fd5b50356001600160a01b0316612685565b60408051602080825283518183015283519192839290830191858101910280838360005b83811015610d4e578181015183820152602001610d36565b505050509050019250505060405180910390f35b61042961270e565b610d1261271e565b61056a60048036036040811015610d8857600080fd5b506001600160a01b0381358116916020013516612780565b61060261279d565b61056a60048036036080811015610dbe57600080fd5b506001600160a01b038135811691602081013582169160408201351690606001356127ac565b610d1260048036036020811015610dfa57600080fd5b810190602081018135600160201b811115610e1457600080fd5b820183602082011115610e2657600080fd5b803590602001918460208302840111600160201b83111715610e4757600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550612840945050505050565b610ebb60048036036060811015610e9b57600080fd5b506001600160a01b038135811691602081013590911690604001356128d7565b6040805192835260208301919091528051918290030190f35b61056a60048036036040811015610eea57600080fd5b506001600160a01b0381358116916020013516612b4c565b61056a60048036036020811015610f1857600080fd5b50356001600160a01b0316612b69565b61051c60048036036020811015610f3e57600080fd5b810190602081018135600160201b811115610f5857600080fd5b820183602082011115610f6a57600080fd5b803590602001918460208302840111600160201b83111715610f8b57600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550612b7b945050505050565b61056a600480360360a0811015610fdf57600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060800135612c0d565b61056a6004803603602081101561102557600080fd5b5035612dc5565b61056a6004803603606081101561104257600080fd5b506001600160a01b03813581169160208101359091169060400135612e2e565b6106026004803603604081101561107857600080fd5b506001600160a01b03813516906020013561311b565b610602613150565b61056a600480360360408110156110ac57600080fd5b506001600160a01b03813516906020013561315f565b61042961330f565b61056a61331f565b61051c600480360360208110156110e857600080fd5b50356001600160a01b0316613325565b61056a6004803603606081101561110e57600080fd5b506001600160a01b03813581169160208101359091169060400135613389565b61056a6004803603602081101561114457600080fd5b50356001600160a01b03166133c6565b6106026136d9565b600181565b6001600160a01b03821660009081526009602052604081205460ff166111b85760405162461bcd60e51b81526004018080602001828103825260288152602001806159296028913960400191505060405180910390fd5b600a546001600160a01b03163314806111db57506000546001600160a01b031633145b6112165760405162461bcd60e51b81526004018080602001828103825260278152602001806159826027913960400191505060405180910390fd5b6000546001600160a01b031633148061123157506001821515145b61127b576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600c6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260069083015265426f72726f7760d01b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150805b92915050565b60408051600180825281830190925260609160208083019080388339019050509050828160008151811061133157fe5b60200260200101906001600160a01b031690816001600160a01b03168152505061135e818360018061206a565b505050565b806001600160a01b031663f851a4406040518163ffffffff1660e01b815260040160206040518083038186803b15801561139c57600080fd5b505afa1580156113b0573d6000803e3d6000fd5b505050506040513d60208110156113c657600080fd5b50516001600160a01b0316331461140e5760405162461bcd60e51b81526004018080602001828103825260278152602001806159ee6027913960400191505060405180910390fd5b806001600160a01b031663c1e803346040518163ffffffff1660e01b8152600401602060405180830381600087803b15801561144957600080fd5b505af115801561145d573d6000803e3d6000fd5b505050506040513d602081101561147357600080fd5b5051156114bf576040805162461bcd60e51b815260206004820152601560248201527418da185b99d9481b9bdd08185d5d1a1bdc9a5e9959605a1b604482015290519081900360640190fd5b50565b600f6020526000908152604090205481565b5050505050565b6001600160a01b03841660009081526009602052604081205460ff166115035750600961159c565b61150b615869565b6040518060200160405280876001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b15801561154f57600080fd5b505afa158015611563573d6000803e3d6000fd5b505050506040513d602081101561157957600080fd5b50519052905061158986826136e8565b6115968685836000613970565b60009150505b949350505050565b600a546001600160a01b031681565b6001546001600160a01b031681565b600a546000906001600160a01b03163314806115e857506000546001600160a01b031633145b6116235760405162461bcd60e51b81526004018080602001828103825260278152602001806159826027913960400191505060405180910390fd5b6000546001600160a01b031633148061163e57506001821515145b611688576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b81b810260ff60b81b1990921691909117909155604080516020810192909252808252600582820152645365697a6560d81b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a150805b919050565b600080546001600160a01b031633146117225761171b60016004613b5b565b90506116f7565b61172a615869565b506040805160208101909152828152611741615869565b50604080516020810190915266b1a2bc2ec5000081526117618282613bc1565b1561177a57611771600580613b5b565b925050506116f7565b611782615869565b506040805160208101909152670c7d713b49da000081526117a38184613bc9565b156117bd576117b3600580613b5b565b93505050506116f7565b6005805490869055604080518281526020810188905281517f3b9670cf975d26958e754b57098eaa2ac914d8d2a31b83257997b9f346110fd9929181900390910190a160005b9695505050505050565b6000546001600160a01b0316331461186c576040805162461bcd60e51b815260206004820152601f60248201527f6f6e6c792061646d696e2063616e2064726f7020636f6d70206d61726b657400604482015290519081900360640190fd5b6001600160a01b0381166000908152600960205260409020600381015460ff1615156001146118e2576040805162461bcd60e51b815260206004820152601b60248201527f6d61726b6574206973206e6f74206120636f6d70206d61726b65740000000000604482015290519081900360640190fd5b60038101805460ff19169055604080516001600160a01b03841681526000602082015281517f93c1f3e36ed71139f466a4ce8c9751790e2e33f5afb2df0dcfb3aeabe55d5aa2929181900390910190a161193a613bd0565b5050565b6001600160a01b03821660009081526009602052604081205460ff166119955760405162461bcd60e51b81526004018080602001828103825260288152602001806159296028913960400191505060405180910390fd5b600a546001600160a01b03163314806119b857506000546001600160a01b031633145b6119f35760405162461bcd60e51b81526004018080602001828103825260278152602001806159826027913960400191505060405180910390fd5b6000546001600160a01b0316331480611a0e57506001821515145b611a58576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600b6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260049083015263135a5b9d60e21b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150919050565b600a54600160a01b900460ff1681565b50505050565b435b90565b505050505050565b60065481565b333214611b405760405162461bcd60e51b81526004018080602001828103825260318152602001806159516031913960400191505060405180910390fd5b611b48613bd0565b565b600080600080600080611b5f8a8a8a8a613f95565b925092509250826011811115611b7157fe5b95509093509150505b9450945094915050565b6001600160a01b0383166000908152600b602052604081205460ff1615611be3576040805162461bcd60e51b815260206004820152600e60248201526d1b5a5b9d081a5cc81c185d5cd95960921b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff16611c0d5760095b9050611c28565b611c16846143b0565b611c228484600061462e565b60005b90505b9392505050565b600080546001600160a01b03163314611c4e5761171b6001600b613b5b565b611c56615869565b506040805160208101909152828152611c6d615869565b506040805160208101909152670de0b6b3a76400008152611c8e8282613bc9565b15611c9f576117716007600c613b5b565b611ca7615869565b5060408051602081019091526714d1120d7b1600008152611cc88184613bc9565b15611cd9576117b36007600c613b5b565b6006805490869055604080518281526020810188905281517faeba5a6c40a8ac138134bff1aaa65debf25971188a58804bad717f82f0ec1316929181900390910190a16000611803565b80158015611d315750600082115b15611ae9576040805162461bcd60e51b815260206004820152601160248201527072656465656d546f6b656e73207a65726f60781b604482015290519081900360640190fd5b600d8181548110611d8457fe5b6000918252602090912001546001600160a01b0316905081565b600080546001600160a01b03163314611dbd5761171b60016010613b5b565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fd52b2b9b7e9ee655fcb95d2e5b9e0c9f69e7ef2b8e9d2d0ea78402d576d22e22929181900390910190a160009392505050565b61135e565b600080600080600080611e41876000806000613f95565b925092509250826011811115611e5357fe5b97919650945092505050565b600080546001600160a01b03163314611e7e5761171b60016013613b5b565b600a80546001600160a01b038481166001600160a01b0319831617928390556040805192821680845293909116602083015280517f0613b6ee6a04f0d09f390e4d9318894b9f6ac7fd83897cd8d18896ba579c401e9281900390910190a16000611c28565b6001600160a01b03851660009081526009602052604081205460ff161580611f2457506001600160a01b03851660009081526009602052604090205460ff16155b15611f335760095b9050612061565b600080611f3f85614826565b91935090915060009050826011811115611f5557fe5b14611f6f57816011811115611f6657fe5b92505050612061565b80611f7b576003611f66565b6000886001600160a01b03166395dd9193876040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b158015611fd357600080fd5b505afa158015611fe7573d6000803e3d6000fd5b505050506040513d6020811015611ffd57600080fd5b50516040805160208101909152600554815290915060009081906120219084614846565b9092509050600082600381111561203457fe5b1461204857600b5b95505050505050612061565b8087111561205757601161203c565b6000955050505050505b95945050505050565b60005b83518110156114d457600084828151811061208457fe5b6020908102919091018101516001600160a01b0381166000908152600990925260409091205490915060ff166120f9576040805162461bcd60e51b81526020600482015260156024820152741b585c9ad95d081b5d5cdd081899481b1a5cdd1959605a1b604482015290519081900360640190fd5b600184151514156121c15761210c615869565b6040518060200160405280836001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b15801561215057600080fd5b505afa158015612164573d6000803e3d6000fd5b505050506040513d602081101561217a57600080fd5b50519052905061218a82826136e8565b60005b87518110156121be576121b6838983815181106121a657fe5b6020026020010151846001613970565b60010161218d565b50505b6001831515141561220a576121d5816143b0565b60005b865181101561220857612200828883815181106121f157fe5b6020026020010151600161462e565b6001016121d8565b505b5060010161206d565b61221b61489a565b61226c576040805162461bcd60e51b815260206004820152601f60248201527f6f6e6c792061646d696e2063616e206368616e676520636f6d70207261746500604482015290519081900360640190fd5b600e805490829055604080518281526020810184905281517fc227c9272633c3a307d9845bf2bc2509cefb20d655b5f3c1002d8e1e3f22c8b0929181900390910190a161193a613bd0565b6010602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b600c6020526000908152604090205460ff1681565b600b6020526000908152604090205460ff1681565b66038d7ea4c6800081565b6004546001600160a01b031681565b600a54600160b01b900460ff1681565b6011602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b60096020526000908152604090208054600182015460039092015460ff91821692911683565b600a546000906001600160a01b03163314806123ab57506000546001600160a01b031633145b6123e65760405162461bcd60e51b81526004018080602001828103825260278152602001806159826027913960400191505060405180910390fd5b6000546001600160a01b031633148061240157506001821515145b61244b576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b01b810260ff60b01b1990921691909117909155604080516020810192909252808252600882820152672a3930b739b332b960c11b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a15090565b6001600160a01b038082166000908152600960209081526040808320938616835260029093019052205460ff1692915050565b60075481565b73c00e94cb662c3520282e6f5717214004a7f2688890565b600080546001600160a01b0316331461252e5761171b60016012613b5b565b6001600160a01b03821660009081526009602052604090205460ff161561255b5761171b600a6011613b5b565b816001600160a01b031663fe9c44ae6040518163ffffffff1660e01b815260040160206040518083038186803b15801561259457600080fd5b505afa1580156125a8573d6000803e3d6000fd5b505050506040513d60208110156125be57600080fd5b5050604080516060810182526001808252600060208381018281528486018381526001600160a01b03891684526009909252949091209251835490151560ff19918216178455935191830191909155516003909101805491151591909216179055612628826148c3565b604080516001600160a01b038416815290517fcf583bb0c569eb967f806b11601c4cb93c10310485c67add5f8362c2f212321f9181900360200190a1600092915050565b6ec097ce7bc90715b34b9f100000000081565b600e5481565b60608060086000846001600160a01b03166001600160a01b0316815260200190815260200160002080548060200260200160405190810160405280929190818152602001828054801561270157602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116126e3575b5093979650505050505050565b600a54600160b81b900460ff1681565b6060600d80548060200260200160405190810160405280929190818152602001828054801561277657602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311612758575b5050505050905090565b601260209081526000928352604080842090915290825290205481565b6002546001600160a01b031681565b600a54600090600160b01b900460ff1615612803576040805162461bcd60e51b81526020600482015260126024820152711d1c985b9cd9995c881a5cc81c185d5cd95960721b604482015290519081900360640190fd5b60006128108686856149a1565b9050801561281f57905061159c565b612828866143b0565b6128348686600061462e565b6115968685600061462e565b6060600082519050606081604051908082528060200260200182016040528015612874578160200160208202803883390190505b50905060005b828110156128cf57600085828151811061289057fe5b602002602001015190506128a48133614a44565b60118111156128af57fe5b8383815181106128bb57fe5b60209081029190910101525060010161287a565b509392505050565b600480546040805163fc57d4df60e01b81526001600160a01b038781169482019490945290516000938493849391169163fc57d4df91602480820192602092909190829003018186803b15801561292d57600080fd5b505afa158015612941573d6000803e3d6000fd5b505050506040513d602081101561295757600080fd5b5051600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051939450600093929091169163fc57d4df91602480820192602092909190829003018186803b1580156129b057600080fd5b505afa1580156129c4573d6000803e3d6000fd5b505050506040513d60208110156129da57600080fd5b505190508115806129e9575080155b156129fe57600d935060009250612b44915050565b6000866001600160a01b031663182df0f56040518163ffffffff1660e01b815260040160206040518083038186803b158015612a3957600080fd5b505afa158015612a4d573d6000803e3d6000fd5b505050506040513d6020811015612a6357600080fd5b505190506000612a71615869565b612a79615869565b612a81615869565b6000612a8f60065489614b65565b945090506000816003811115612aa157fe5b14612abd57600b5b995060009850612b44975050505050505050565b612ac78787614b65565b935090506000816003811115612ad957fe5b14612ae557600b612aa9565b612aef8484614ba0565b925090506000816003811115612b0157fe5b14612b0d57600b612aa9565b612b17828c614846565b955090506000816003811115612b2957fe5b14612b3557600b612aa9565b60009950939750505050505050505b935093915050565b601360209081526000928352604080842090915290825290205481565b60146020526000908152604090205481565b612b8361489a565b612bd4576040805162461bcd60e51b815260206004820152601e60248201527f6f6e6c792061646d696e2063616e2061646420636f6d70206d61726b65740000604482015290519081900360640190fd5b60005b8151811015612c0457612bfc828281518110612bef57fe5b6020026020010151614bb8565b600101612bd7565b506114bf613bd0565b600a54600090600160b81b900460ff1615612c61576040805162461bcd60e51b815260206004820152600f60248201526e1cd95a5e99481a5cc81c185d5cd959608a1b604482015290519081900360640190fd5b6001600160a01b03861660009081526009602052604090205460ff161580612ca257506001600160a01b03851660009081526009602052604090205460ff16155b15612cae576009611f2c565b846001600160a01b0316635fe3b5676040518163ffffffff1660e01b815260040160206040518083038186803b158015612ce757600080fd5b505afa158015612cfb573d6000803e3d6000fd5b505050506040513d6020811015612d1157600080fd5b505160408051635fe3b56760e01b815290516001600160a01b0392831692891691635fe3b567916004808301926020929190829003018186803b158015612d5757600080fd5b505afa158015612d6b573d6000803e3d6000fd5b505050506040513d6020811015612d8157600080fd5b50516001600160a01b031614612d98576002611f2c565b612da1866143b0565b612dad8684600061462e565b612db98685600061462e565b60009695505050505050565b600080546001600160a01b03163314612de45761171b6001600d613b5b565b6007805490839055604080518281526020810185905281517f7093cf1eb653f749c3ff531d6df7f92764536a7fa0d13530cd26e070780c32ea929181900390910190a16000611c28565b6001600160a01b0383166000908152600c602052604081205460ff1615612e8f576040805162461bcd60e51b815260206004820152601060248201526f189bdc9c9bddc81a5cc81c185d5cd95960821b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff16612eb6576009611c06565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff16612fa657336001600160a01b03851614612f3c576040805162461bcd60e51b815260206004820152601560248201527439b2b73232b91036bab9ba1031329031aa37b5b2b760591b604482015290519081900360640190fd5b6000612f483385614a44565b90506000816011811115612f5857fe5b14612f7157806011811115612f6957fe5b915050611c28565b6001600160a01b038086166000908152600960209081526040808320938816835260029093019052205460ff16612fa457fe5b505b600480546040805163fc57d4df60e01b81526001600160a01b03888116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b158015612ff757600080fd5b505afa15801561300b573d6000803e3d6000fd5b505050506040513d602081101561302157600080fd5b505161302e57600d611c06565b60008061303e8587600087613f95565b9193509091506000905082601181111561305457fe5b1461306e5781601181111561306557fe5b92505050611c28565b801561307b576004613065565b613083615869565b6040518060200160405280886001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b1580156130c757600080fd5b505afa1580156130db573d6000803e3d6000fd5b505050506040513d60208110156130f157600080fd5b50519052905061310187826136e8565b61310e8787836000613970565b6000979650505050505050565b6008602052816000526040600020818154811061313457fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b600080546001600160a01b031633146131855761317e60016006613b5b565b90506112fb565b6001600160a01b0383166000908152600960205260409020805460ff166131ba576131b260096007613b5b565b9150506112fb565b6131c2615869565b5060408051602081019091528381526131d9615869565b506040805160208101909152670c7d713b49da000081526131fa8183613bc9565b156132155761320b60066008613b5b565b93505050506112fb565b841580159061329e5750600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b15801561327057600080fd5b505afa158015613284573d6000803e3d6000fd5b505050506040513d602081101561329a57600080fd5b5051155b156132af5761320b600d6009613b5b565b60018301805490869055604080516001600160a01b03891681526020810183905280820188905290517f70483e6592cd5182d45ac970e05bc62cdcc90e9d8ef2c2dbe686cf383bcd7fc59181900360600190a16000979650505050505050565b600a54600160a81b900460ff1681565b60055481565b6114bf81600d80548060200260200160405190810160405280929190818152602001828054801561337f57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613361575b5050505050611301565b6000806133978585856149a1565b905080156133a6579050611c28565b6133af856143b0565b6133bb8585600061462e565b600095945050505050565b6000808290506000806000836001600160a01b031663c37f68e2336040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b15801561342757600080fd5b505afa15801561343b573d6000803e3d6000fd5b505050506040513d608081101561345157600080fd5b5080516020820151604090920151909450909250905082156134a45760405162461bcd60e51b81526004018080602001828103825260258152602001806159a96025913960400191505060405180910390fd5b80156134c1576134b6600c6002613b5b565b9450505050506116f7565b60006134ce8733856149a1565b905080156134ef576134e3600e600383614ed3565b955050505050506116f7565b6001600160a01b0385166000908152600960209081526040808320338452600281019092529091205460ff1661352e57600096505050505050506116f7565b3360009081526002820160209081526040808320805460ff1916905560088252918290208054835181840281018401909452808452606093928301828280156135a057602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613582575b5050835193945083925060009150505b828110156135f557896001600160a01b03168482815181106135ce57fe5b60200260200101516001600160a01b031614156135ed578091506135f5565b6001016135b0565b508181106135ff57fe5b33600090815260086020526040902080548190600019810190811061362057fe5b9060005260206000200160009054906101000a90046001600160a01b031681838154811061364a57fe5b600091825260209091200180546001600160a01b0319166001600160a01b0392909216919091179055805461368382600019830161587c565b50604080516001600160a01b038c16815233602082015281517fe699a64c18b07ac5b7301aa273f36a2287239eb9501d81950672794afba29a0d929181900390910190a160009c9b505050505050505050505050565b6000546001600160a01b031681565b6001600160a01b0382166000908152601160209081526040808320600f9092528220549091613715611aef565b8354909150600090613735908390600160e01b900463ffffffff16614f39565b90506000811180156137475750600083115b156139165760006137bc876001600160a01b03166347bd37186040518163ffffffff1660e01b815260040160206040518083038186803b15801561378a57600080fd5b505afa15801561379e573d6000803e3d6000fd5b505050506040513d60208110156137b457600080fd5b505187614f73565b905060006137ca8386614f91565b90506137d4615869565b600083116137f157604051806020016040528060008152506137fb565b6137fb8284614fd3565b9050613805615869565b604080516020810190915288546001600160e01b031681526138279083615011565b9050604051806040016040528061387783600001516040518060400160405280601a81526020017f6e657720696e6465782065786365656473203232342062697473000000000000815250615036565b6001600160e01b031681526020016138b2886040518060400160405280601c81526020016000805160206159ce8339815191528152506150d0565b63ffffffff9081169091526001600160a01b038c166000908152601160209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b0319909416939093171691909117905550611af492505050565b8015611af457613949826040518060400160405280601c81526020016000805160206159ce8339815191528152506150d0565b845463ffffffff91909116600160e01b026001600160e01b03909116178455505050505050565b6001600160a01b0384166000908152601160205260409020613990615869565b50604080516020810190915281546001600160e01b031681526139b1615869565b5060408051602080820183526001600160a01b03808a16600090815260138352848120918a1680825282845294812080548552865195909152915291909155805115613b52576139ff615869565b613a098383615125565b90506000613a98896001600160a01b03166395dd91938a6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b158015613a6657600080fd5b505afa158015613a7a573d6000803e3d6000fd5b505050506040513d6020811015613a9057600080fd5b505188614f73565b90506000613aa6828461514a565b6001600160a01b038a1660009081526014602052604081205491925090613acd9083615179565b9050613aee8a828a613ae65766038d7ea4c68000613ae9565b60005b6151af565b6001600160a01b03808c1660008181526014602090815260409182902094909455895181518781529485015280519193928f16927f1fc3ecc087d8d2d15e23d0032af5a47059c3892d003d8e139fdcb6bb327c99a6929081900390910190a3505050505b50505050505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0836011811115613b8a57fe5b836013811115613b9657fe5b604080519283526020830191909152600082820152519081900360600190a1826011811115611c2857fe5b519051111590565b5190511090565b6060600d805480602002602001604051908101604052809291908181526020018280548015613c2857602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613c0a575b50939450600093505050505b8151811015613cee576000828281518110613c4b57fe5b60200260200101519050613c5d615869565b6040518060200160405280836001600160a01b031663aa5af0fd6040518163ffffffff1660e01b815260040160206040518083038186803b158015613ca157600080fd5b505afa158015613cb5573d6000803e3d6000fd5b505050506040513d6020811015613ccb57600080fd5b505190529050613cda826143b0565b613ce482826136e8565b5050600101613c34565b50613cf7615869565b60405180602001604052806000815250905060608251604051908082528060200260200182016040528015613d4657816020015b613d33615869565b815260200190600190039081613d2b5790505b50905060005b8351811015613ecc576000848281518110613d6357fe5b6020908102919091018101516001600160a01b0381166000908152600990925260409091206003015490915060ff1615613ec357613d9f615869565b60408051602080820180845260045463fc57d4df60e01b9091526001600160a01b03868116602485015293519293849391169163fc57d4df916044808601929190818703018186803b158015613df457600080fd5b505afa158015613e08573d6000803e3d6000fd5b505050506040513d6020811015613e1e57600080fd5b505190529050613e2c615869565b613e9a82846001600160a01b03166347bd37186040518163ffffffff1660e01b815260040160206040518083038186803b158015613e6957600080fd5b505afa158015613e7d573d6000803e3d6000fd5b505050506040513d6020811015613e9357600080fd5b50516152f4565b905080858581518110613ea957fe5b6020026020010181905250613ebe8682615011565b955050505b50600101613d4c565b5060005b8351811015611ae9576000600d8281548110613ee857fe5b600091825260208220015485516001600160a01b039091169250613f0d576000613f35565b613f35600e54613f30868681518110613f2257fe5b602002602001015188615315565b615348565b6001600160a01b0383166000818152600f60209081526040918290208490558151848152915193945091927f2ab93f65628379309f36cb125e90d7c902454a545c4f8b8cb0794af75c24b807929181900390910190a25050600101613ed0565b6000806000613fa26158a0565b6001600160a01b03881660009081526008602090815260408083208054825181850281018501909352808352849360609392919083018282801561400f57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311613ff1575b50939450600093505050505b815181101561436b57600082828151811061403257fe5b60200260200101519050806001600160a01b031663c37f68e28e6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b15801561409257600080fd5b505afa1580156140a6573d6000803e3d6000fd5b505050506040513d60808110156140bc57600080fd5b508051602082015160408084015160609485015160808c0152938a019390935291880191909152945084156141025750600f975060009650869550611b7a945050505050565b60408051602080820183526001600160a01b0380851660008181526009845285902060010154845260c08b01939093528351808301855260808b0151815260e08b015260048054855163fc57d4df60e01b815291820194909452935192169263fc57d4df9260248083019392829003018186803b15801561418257600080fd5b505afa158015614196573d6000803e3d6000fd5b505050506040513d60208110156141ac57600080fd5b505160a087018190526141d05750600d975060009650869550611b7a945050505050565b604080516020810190915260a08701518152610100870181905260c087015160e08801516141fd92615361565b6101208801529350600084600381111561421357fe5b1461422f5750600b975060009650869550611b7a945050505050565b614247866101200151876040015188600001516153b9565b87529350600084600381111561425957fe5b146142755750600b975060009650869550611b7a945050505050565b61428d866101000151876060015188602001516153b9565b6020880152935060008460038111156142a257fe5b146142be5750600b975060009650869550611b7a945050505050565b8b6001600160a01b0316816001600160a01b03161415614362576142ec8661012001518c88602001516153b9565b60208801529350600084600381111561430157fe5b1461431d5750600b975060009650869550611b7a945050505050565b6143318661010001518b88602001516153b9565b60208801529350600084600381111561434657fe5b146143625750600b975060009650869550611b7a945050505050565b5060010161401b565b50602084015184511115614392575050506020810151905160009450039150829050611b7a565b5050815160209092015160009550859450919091039150611b7a9050565b6001600160a01b0381166000908152601060209081526040808320600f90925282205490916143dd611aef565b83549091506000906143fd908390600160e01b900463ffffffff16614f39565b905060008111801561440f5750600083115b156145d5576000856001600160a01b03166318160ddd6040518163ffffffff1660e01b815260040160206040518083038186803b15801561444f57600080fd5b505afa158015614463573d6000803e3d6000fd5b505050506040513d602081101561447957600080fd5b5051905060006144898386614f91565b9050614493615869565b600083116144b057604051806020016040528060008152506144ba565b6144ba8284614fd3565b90506144c4615869565b604080516020810190915288546001600160e01b031681526144e69083615011565b9050604051806040016040528061453683600001516040518060400160405280601a81526020017f6e657720696e6465782065786365656473203232342062697473000000000000815250615036565b6001600160e01b03168152602001614571886040518060400160405280601c81526020016000805160206159ce8339815191528152506150d0565b63ffffffff9081169091526001600160a01b038b166000908152601060209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b03199094169390931716919091179055506114d492505050565b80156114d457614608826040518060400160405280601c81526020016000805160206159ce8339815191528152506150d0565b845463ffffffff91909116600160e01b026001600160e01b039091161784555050505050565b6001600160a01b038316600090815260106020526040902061464e615869565b50604080516020810190915281546001600160e01b0316815261466f615869565b5060408051602080820183526001600160a01b038089166000908152601283528481209189168082528284529481208054855286519590915291529190915580511580156146bd5750815115155b156146d5576ec097ce7bc90715b34b9f100000000081525b6146dd615869565b6146e78383615125565b90506000876001600160a01b03166370a08231886040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b15801561474157600080fd5b505afa158015614755573d6000803e3d6000fd5b505050506040513d602081101561476b57600080fd5b50519050600061477b828461514a565b6001600160a01b038916600090815260146020526040812054919250906147a29083615179565b90506147bb89828a613ae65766038d7ea4c68000613ae9565b6001600160a01b03808b1660008181526014602090815260409182902094909455895181518781529485015280519193928e16927f2caecd17d02f56fa897705dcc740da2d237c373f70686f4e0d9bd3bf0400ea7a929081900390910190a350505050505050505050565b6000806000614839846000806000613f95565b9250925092509193909250565b6000806000614853615869565b61485d8686615406565b9092509050600082600381111561487057fe5b146148815750915060009050614893565b600061488c8261546e565b9350935050505b9250929050565b600080546001600160a01b03163314806148be57506002546001600160a01b031633145b905090565b60005b600d5481101561494e57816001600160a01b0316600d82815481106148e757fe5b6000918252602090912001546001600160a01b03161415614946576040805162461bcd60e51b81526020600482015260146024820152731b585c9ad95d08185b1c9958591e48185919195960621b604482015290519081900360640190fd5b6001016148c6565b50600d80546001810182556000919091527fd7b6990105719101dabeb77144f2a3385c8033acd3af97e9423a695e81ad1eb50180546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b03831660009081526009602052604081205460ff166149c8576009611c06565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff16614a00576000611c06565b600080614a108587866000613f95565b91935090915060009050826011811115614a2657fe5b14614a375781601181111561306557fe5b8015612db9576004613065565b6001600160a01b0382166000908152600960205260408120805460ff16614a6f5760099150506112fb565b6001600160a01b038316600090815260028201602052604090205460ff16151560011415614aa15760009150506112fb565b6007546001600160a01b03841660009081526008602052604090205410614acc5760109150506112fb565b6001600160a01b0380841660008181526002840160209081526040808320805460ff19166001908117909155600883528184208054918201815584529282902090920180549489166001600160a01b031990951685179055815193845283019190915280517f3ab23ab0d51cccc0c3085aec51f99228625aa1a922b3a8ca89a26b0f2027a1a59281900390910190a15060009392505050565b6000614b6f615869565b614b9560405180602001604052808681525060405180602001604052808681525061547d565b915091509250929050565b6000614baa615869565b83518351614b959190615566565b6001600160a01b0381166000908152600960205260409020805460ff161515600114614c2b576040805162461bcd60e51b815260206004820152601960248201527f636f6d70206d61726b6574206973206e6f74206c697374656400000000000000604482015290519081900360640190fd5b600381015460ff1615614c85576040805162461bcd60e51b815260206004820152601960248201527f636f6d70206d61726b657420616c726561647920616464656400000000000000604482015290519081900360640190fd5b60038101805460ff19166001908117909155604080516001600160a01b0385168152602081019290925280517f93c1f3e36ed71139f466a4ce8c9751790e2e33f5afb2df0dcfb3aeabe55d5aa29281900390910190a16001600160a01b0382166000908152601060205260409020546001600160e01b0316158015614d2d57506001600160a01b038216600090815260106020526040902054600160e01b900463ffffffff16155b15614dea5760405180604001604052806ec097ce7bc90715b34b9f10000000006001600160e01b03168152602001614d8f614d66611aef565b6040518060400160405280601c81526020016000805160206159ce8339815191528152506150d0565b63ffffffff9081169091526001600160a01b0384166000908152601060209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b031990941693909317169190911790555b6001600160a01b0382166000908152601160205260409020546001600160e01b0316158015614e3c57506001600160a01b038216600090815260116020526040902054600160e01b900463ffffffff16155b1561193a5760405180604001604052806ec097ce7bc90715b34b9f10000000006001600160e01b03168152602001614e75614d66611aef565b63ffffffff9081169091526001600160a01b0384166000908152601160209081526040909120835181549490920151909216600160e01b026001600160e01b039182166001600160e01b031990941693909317169190911790555050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0846011811115614f0257fe5b846013811115614f0e57fe5b604080519283526020830191909152818101859052519081900360600190a1836011811115611c2557fe5b6000611c288383604051806040016040528060158152602001747375627472616374696f6e20756e646572666c6f7760581b815250615616565b6000611c28614f8a84670de0b6b3a7640000614f91565b8351615670565b6000611c2883836040518060400160405280601781526020017f6d756c7469706c69636174696f6e206f766572666c6f770000000000000000008152506156a3565b614fdb615869565b6040518060200160405280615008615002866ec097ce7bc90715b34b9f1000000000614f91565b85615670565b90529392505050565b615019615869565b604051806020016040528061500885600001518560000151615179565b600081600160e01b84106150c85760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561508d578181015183820152602001615075565b50505050905090810190601f1680156150ba5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b509192915050565b600081600160201b84106150c85760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561508d578181015183820152602001615075565b61512d615869565b604051806020016040528061500885600001518560000151614f39565b60006ec097ce7bc90715b34b9f100000000061516a848460000151614f91565b8161517157fe5b049392505050565b6000611c288383604051806040016040528060118152602001706164646974696f6e206f766572666c6f7760781b815250615722565b60008183101580156151c15750600083115b156152ec5760006151d06124f7565b604080516370a0823160e01b815230600482015290519192506000916001600160a01b038416916370a08231916024808301926020929190829003018186803b15801561521c57600080fd5b505afa158015615230573d6000803e3d6000fd5b505050506040513d602081101561524657600080fd5b505190508085116152e957816001600160a01b031663a9059cbb87876040518363ffffffff1660e01b815260040180836001600160a01b03166001600160a01b0316815260200182815260200192505050602060405180830381600087803b1580156152b157600080fd5b505af11580156152c5573d6000803e3d6000fd5b505050506040513d60208110156152db57600080fd5b5060009350611c2892505050565b50505b509092915050565b6152fc615869565b6040518060200160405280615008856000015185614f91565b61531d615869565b60405180602001604052806150086153418660000151670de0b6b3a7640000614f91565b8551615670565b6000670de0b6b3a764000061516a848460000151614f91565b600061536b615869565b6000615375615869565b61537f878761547d565b9092509050600082600381111561539257fe5b146153a1579092509050612b44565b6153ab818661547d565b935093505050935093915050565b60008060006153c6615869565b6153d08787615406565b909250905060008260038111156153e357fe5b146153f45750915060009050612b44565b6153ab6154008261546e565b86615777565b6000615410615869565b60008061542186600001518661579d565b9092509050600082600381111561543457fe5b1461545357506040805160208101909152600081529092509050614893565b60408051602081019091529081526000969095509350505050565b51670de0b6b3a7640000900490565b6000615487615869565b60008061549c8660000151866000015161579d565b909250905060008260038111156154af57fe5b146154ce57506040805160208101909152600081529092509050614893565b6000806154e36706f05b59d3b2000084615777565b909250905060008260038111156154f657fe5b1461551857506040805160208101909152600081529094509250614893915050565b60008061552d83670de0b6b3a76400006157dc565b9092509050600082600381111561554057fe5b1461554757fe5b604080516020810190915290815260009a909950975050505050505050565b6000615570615869565b60008061558586670de0b6b3a764000061579d565b9092509050600082600381111561559857fe5b146155b757506040805160208101909152600081529092509050614893565b6000806155c483886157dc565b909250905060008260038111156155d757fe5b146155f957506040805160208101909152600081529094509250614893915050565b604080516020810190915290815260009890975095505050505050565b600081848411156156685760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561508d578181015183820152602001615075565b505050900390565b6000611c2883836040518060400160405280600e81526020016d646976696465206279207a65726f60901b815250615807565b60008315806156b0575082155b156156bd57506000611c28565b838302838582816156ca57fe5b041483906157195760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561508d578181015183820152602001615075565b50949350505050565b600083830182858210156157195760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561508d578181015183820152602001615075565b60008083830184811061578f57600092509050614893565b506002915060009050614893565b600080836157b057506000905080614893565b838302838582816157bd57fe5b04146157d157506002915060009050614893565b600092509050614893565b600080826157f05750600190506000614893565b60008385816157fb57fe5b04915091509250929050565b600081836158565760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561508d578181015183820152602001615075565b5082848161586057fe5b04949350505050565b6040518060200160405280600081525090565b81548183558181111561135e5760008381526020902061135e91810190830161590a565b6040518061014001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016158de615869565b81526020016158eb615869565b81526020016158f8615869565b8152602001615905615869565b905290565b611af191905b808211156159245760008155600101615910565b509056fe63616e6e6f742070617573652061206d61726b65742074686174206973206e6f74206c69737465646f6e6c792065787465726e616c6c79206f776e6564206163636f756e7473206d61792072656672657368207370656564736f6e6c7920706175736520677561726469616e20616e642061646d696e2063616e207061757365657869744d61726b65743a206765744163636f756e74536e617073686f74206661696c6564626c6f636b206e756d62657220657863656564732033322062697473000000006f6e6c7920756e6974726f6c6c65722061646d696e2063616e206368616e676520627261696e73a265627a7a723158203858707cc51fbc491d6863de69ba1565278b668512120f3e8755d7488d8672ef64736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Comptroller<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Comptroller<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Comptroller<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Comptroller))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Comptroller<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMPTROLLER_ABI.clone(), client).into()
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
                COMPTROLLER_ABI.clone(),
                COMPTROLLER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `_become` (0x1d504dc6) function"]
        pub fn become_(
            &self,
            unitroller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 80, 77, 198], unitroller)
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
        pub fn action_paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ActionPausedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ComptrollerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Comptroller<M> {
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
    #[ethevent(name = "ActionPaused", abi = "ActionPaused(address,string,bool)")]
    pub struct ActionPausedFilter {
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
    pub enum ComptrollerEvents {
        ActionPausedFilter(ActionPausedFilter),
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
    impl ethers::contract::EthLogDecode for ComptrollerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ActionPausedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::ActionPausedFilter(decoded));
            }
            if let Ok(decoded) = ActionPausedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::ActionPausedFilter(decoded));
            }
            if let Ok(decoded) = CompSpeedUpdatedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::CompSpeedUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DistributedBorrowerCompFilter::decode_log(log) {
                return Ok(ComptrollerEvents::DistributedBorrowerCompFilter(decoded));
            }
            if let Ok(decoded) = DistributedSupplierCompFilter::decode_log(log) {
                return Ok(ComptrollerEvents::DistributedSupplierCompFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(ComptrollerEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = MarketCompedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketCompedFilter(decoded));
            }
            if let Ok(decoded) = MarketEnteredFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketEnteredFilter(decoded));
            }
            if let Ok(decoded) = MarketExitedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketExitedFilter(decoded));
            }
            if let Ok(decoded) = MarketListedFilter::decode_log(log) {
                return Ok(ComptrollerEvents::MarketListedFilter(decoded));
            }
            if let Ok(decoded) = NewCloseFactorFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewCloseFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCollateralFactorFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewCollateralFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCompRateFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewCompRateFilter(decoded));
            }
            if let Ok(decoded) = NewLiquidationIncentiveFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewLiquidationIncentiveFilter(decoded));
            }
            if let Ok(decoded) = NewMaxAssetsFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewMaxAssetsFilter(decoded));
            }
            if let Ok(decoded) = NewPauseGuardianFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewPauseGuardianFilter(decoded));
            }
            if let Ok(decoded) = NewPriceOracleFilter::decode_log(log) {
                return Ok(ComptrollerEvents::NewPriceOracleFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ComptrollerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerEvents::ActionPausedFilter(element) => element.fmt(f),
                ComptrollerEvents::CompSpeedUpdatedFilter(element) => element.fmt(f),
                ComptrollerEvents::DistributedBorrowerCompFilter(element) => element.fmt(f),
                ComptrollerEvents::DistributedSupplierCompFilter(element) => element.fmt(f),
                ComptrollerEvents::FailureFilter(element) => element.fmt(f),
                ComptrollerEvents::MarketCompedFilter(element) => element.fmt(f),
                ComptrollerEvents::MarketEnteredFilter(element) => element.fmt(f),
                ComptrollerEvents::MarketExitedFilter(element) => element.fmt(f),
                ComptrollerEvents::MarketListedFilter(element) => element.fmt(f),
                ComptrollerEvents::NewCloseFactorFilter(element) => element.fmt(f),
                ComptrollerEvents::NewCollateralFactorFilter(element) => element.fmt(f),
                ComptrollerEvents::NewCompRateFilter(element) => element.fmt(f),
                ComptrollerEvents::NewLiquidationIncentiveFilter(element) => element.fmt(f),
                ComptrollerEvents::NewMaxAssetsFilter(element) => element.fmt(f),
                ComptrollerEvents::NewPauseGuardianFilter(element) => element.fmt(f),
                ComptrollerEvents::NewPriceOracleFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_addCompMarkets`function with signature `_addCompMarkets(address[])` and selector `[206, 72, 92, 94]`"]
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
    #[doc = "Container type for all input parameters for the `_become`function with signature `_become(address)` and selector `[29, 80, 77, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_become", abi = "_become(address)")]
    pub struct BecomeCall {
        pub unitroller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_borrowGuardianPaused`function with signature `_borrowGuardianPaused()` and selector `[230, 101, 63, 61]`"]
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
    #[doc = "Container type for all input parameters for the `_dropCompMarket`function with signature `_dropCompMarket(address)` and selector `[58, 167, 41, 180]`"]
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
    #[doc = "Container type for all input parameters for the `_mintGuardianPaused`function with signature `_mintGuardianPaused()` and selector `[60, 148, 120, 111]`"]
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
    #[doc = "Container type for all input parameters for the `_setBorrowPaused`function with signature `_setBorrowPaused(address,bool)` and selector `[24, 200, 130, 165]`"]
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
    #[doc = "Container type for all input parameters for the `_setCloseFactor`function with signature `_setCloseFactor(uint256)` and selector `[49, 123, 11, 119]`"]
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
    #[doc = "Container type for all input parameters for the `_setCollateralFactor`function with signature `_setCollateralFactor(address,uint256)` and selector `[228, 2, 142, 238]`"]
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
    #[doc = "Container type for all input parameters for the `_setCompRate`function with signature `_setCompRate(uint256)` and selector `[106, 73, 17, 18]`"]
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
    #[doc = "Container type for all input parameters for the `_setLiquidationIncentive`function with signature `_setLiquidationIncentive(uint256)` and selector `[79, 212, 46, 23]`"]
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
    #[doc = "Container type for all input parameters for the `_setMaxAssets`function with signature `_setMaxAssets(uint256)` and selector `[217, 34, 108, 237]`"]
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
    #[doc = "Container type for all input parameters for the `_setMintPaused`function with signature `_setMintPaused(address,bool)` and selector `[59, 207, 126, 193]`"]
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
    #[doc = "Container type for all input parameters for the `_setPauseGuardian`function with signature `_setPauseGuardian(address)` and selector `[95, 90, 241, 170]`"]
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
    #[doc = "Container type for all input parameters for the `_setPriceOracle`function with signature `_setPriceOracle(address)` and selector `[85, 238, 31, 225]`"]
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
    #[doc = "Container type for all input parameters for the `_setSeizePaused`function with signature `_setSeizePaused(bool)` and selector `[45, 112, 219, 120]`"]
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
    #[doc = "Container type for all input parameters for the `_setTransferPaused`function with signature `_setTransferPaused(bool)` and selector `[142, 191, 99, 100]`"]
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
    #[doc = "Container type for all input parameters for the `_supportMarket`function with signature `_supportMarket(address)` and selector `[167, 107, 63, 218]`"]
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
    #[doc = "Container type for all input parameters for the `accountAssets`function with signature `accountAssets(address,uint256)` and selector `[220, 225, 84, 73]`"]
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
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
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
    #[doc = "Container type for all input parameters for the `allMarkets`function with signature `allMarkets(uint256)` and selector `[82, 216, 77, 30]`"]
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
    #[doc = "Container type for all input parameters for the `borrowAllowed`function with signature `borrowAllowed(address,address,uint256)` and selector `[218, 61, 69, 76]`"]
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
    #[doc = "Container type for all input parameters for the `borrowGuardianPaused`function with signature `borrowGuardianPaused(address)` and selector `[109, 21, 78, 165]`"]
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
    #[doc = "Container type for all input parameters for the `borrowVerify`function with signature `borrowVerify(address,address,uint256)` and selector `[92, 119, 134, 5]`"]
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
    #[doc = "Container type for all input parameters for the `checkMembership`function with signature `checkMembership(address,address)` and selector `[146, 159, 233, 161]`"]
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
    #[doc = "Container type for all input parameters for the `claimComp`function with signature `claimComp(address,address[])` and selector `[28, 61, 178, 224]`"]
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
    #[doc = "Container type for all input parameters for the `claimComp`function with signature `claimComp(address[],address[],bool,bool)` and selector `[104, 16, 223, 166]`"]
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
    #[doc = "Container type for all input parameters for the `claimComp`function with signature `claimComp(address)` and selector `[233, 175, 2, 146]`"]
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
    #[doc = "Container type for all input parameters for the `closeFactorMantissa`function with signature `closeFactorMantissa()` and selector `[232, 117, 84, 70]`"]
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
    #[doc = "Container type for all input parameters for the `compAccrued`function with signature `compAccrued(address)` and selector `[204, 126, 189, 196]`"]
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
    #[doc = "Container type for all input parameters for the `compBorrowState`function with signature `compBorrowState(address)` and selector `[140, 87, 128, 78]`"]
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
    #[doc = "Container type for all input parameters for the `compBorrowerIndex`function with signature `compBorrowerIndex(address,address)` and selector `[202, 10, 240, 67]`"]
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
    #[doc = "Container type for all input parameters for the `compClaimThreshold`function with signature `compClaimThreshold()` and selector `[116, 112, 38, 201]`"]
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
    #[doc = "Container type for all input parameters for the `compInitialIndex`function with signature `compInitialIndex()` and selector `[167, 240, 226, 49]`"]
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
    #[doc = "Container type for all input parameters for the `compRate`function with signature `compRate()` and selector `[170, 144, 7, 84]`"]
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
    #[doc = "Container type for all input parameters for the `compSpeeds`function with signature `compSpeeds(address)` and selector `[29, 123, 51, 215]`"]
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
    #[doc = "Container type for all input parameters for the `compSupplierIndex`function with signature `compSupplierIndex(address,address)` and selector `[178, 27, 231, 253]`"]
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
    #[doc = "Container type for all input parameters for the `compSupplyState`function with signature `compSupplyState(address)` and selector `[107, 121, 195, 141]`"]
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
    #[doc = "Container type for all input parameters for the `comptrollerImplementation`function with signature `comptrollerImplementation()` and selector `[187, 130, 170, 94]`"]
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
    #[doc = "Container type for all input parameters for the `enterMarkets`function with signature `enterMarkets(address[])` and selector `[194, 153, 130, 56]`"]
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
    #[doc = "Container type for all input parameters for the `exitMarket`function with signature `exitMarket(address)` and selector `[237, 228, 237, 208]`"]
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
    #[doc = "Container type for all input parameters for the `getAccountLiquidity`function with signature `getAccountLiquidity(address)` and selector `[94, 200, 140, 121]`"]
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
    #[doc = "Container type for all input parameters for the `getAllMarkets`function with signature `getAllMarkets()` and selector `[176, 119, 45, 11]`"]
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
    #[doc = "Container type for all input parameters for the `getAssetsIn`function with signature `getAssetsIn(address)` and selector `[171, 252, 239, 252]`"]
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
    #[doc = "Container type for all input parameters for the `getBlockNumber`function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
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
    #[doc = "Container type for all input parameters for the `getCompAddress`function with signature `getCompAddress()` and selector `[157, 27, 90, 10]`"]
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
    #[doc = "Container type for all input parameters for the `getHypotheticalAccountLiquidity`function with signature `getHypotheticalAccountLiquidity(address,address,uint256,uint256)` and selector `[78, 121, 35, 143]`"]
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
    #[doc = "Container type for all input parameters for the `isComptroller`function with signature `isComptroller()` and selector `[0, 126, 61, 210]`"]
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
    #[doc = "Container type for all input parameters for the `liquidateBorrowAllowed`function with signature `liquidateBorrowAllowed(address,address,address,address,uint256)` and selector `[95, 199, 231, 30]`"]
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
    #[doc = "Container type for all input parameters for the `liquidateBorrowVerify`function with signature `liquidateBorrowVerify(address,address,address,address,uint256,uint256)` and selector `[71, 239, 59, 59]`"]
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
    #[doc = "Container type for all input parameters for the `liquidateCalculateSeizeTokens`function with signature `liquidateCalculateSeizeTokens(address,address,uint256)` and selector `[196, 136, 132, 123]`"]
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
    #[doc = "Container type for all input parameters for the `liquidationIncentiveMantissa`function with signature `liquidationIncentiveMantissa()` and selector `[74, 218, 144, 175]`"]
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
    #[doc = "Container type for all input parameters for the `markets`function with signature `markets(address)` and selector `[142, 143, 41, 75]`"]
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
    #[doc = "Container type for all input parameters for the `maxAssets`function with signature `maxAssets()` and selector `[148, 178, 41, 75]`"]
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
    #[doc = "Container type for all input parameters for the `mintAllowed`function with signature `mintAllowed(address,address,uint256)` and selector `[78, 244, 195, 225]`"]
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
    #[doc = "Container type for all input parameters for the `mintGuardianPaused`function with signature `mintGuardianPaused(address)` and selector `[115, 31, 12, 43]`"]
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
    #[doc = "Container type for all input parameters for the `mintVerify`function with signature `mintVerify(address,address,uint256,uint256)` and selector `[65, 199, 40, 185]`"]
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
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
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
    #[doc = "Container type for all input parameters for the `pauseGuardian`function with signature `pauseGuardian()` and selector `[36, 163, 214, 34]`"]
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
    #[doc = "Container type for all input parameters for the `pendingAdmin`function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
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
    #[doc = "Container type for all input parameters for the `pendingComptrollerImplementation`function with signature `pendingComptrollerImplementation()` and selector `[220, 251, 192, 199]`"]
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
    #[doc = "Container type for all input parameters for the `redeemAllowed`function with signature `redeemAllowed(address,address,uint256)` and selector `[234, 190, 125, 145]`"]
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
    #[doc = "Container type for all input parameters for the `redeemVerify`function with signature `redeemVerify(address,address,uint256,uint256)` and selector `[81, 223, 249, 137]`"]
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
    #[doc = "Container type for all input parameters for the `refreshCompSpeeds`function with signature `refreshCompSpeeds()` and selector `[77, 142, 80, 55]`"]
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
    #[doc = "Container type for all input parameters for the `repayBorrowAllowed`function with signature `repayBorrowAllowed(address,address,address,uint256)` and selector `[36, 0, 138, 98]`"]
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
    #[doc = "Container type for all input parameters for the `repayBorrowVerify`function with signature `repayBorrowVerify(address,address,address,uint256,uint256)` and selector `[30, 222, 220, 145]`"]
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
    #[doc = "Container type for all input parameters for the `seizeAllowed`function with signature `seizeAllowed(address,address,address,address,uint256)` and selector `[208, 47, 115, 81]`"]
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
    #[doc = "Container type for all input parameters for the `seizeGuardianPaused`function with signature `seizeGuardianPaused()` and selector `[172, 11, 11, 183]`"]
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
    #[doc = "Container type for all input parameters for the `seizeVerify`function with signature `seizeVerify(address,address,address,address,uint256)` and selector `[109, 53, 191, 145]`"]
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
    #[doc = "Container type for all input parameters for the `transferAllowed`function with signature `transferAllowed(address,address,address,uint256)` and selector `[189, 205, 194, 88]`"]
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
    #[doc = "Container type for all input parameters for the `transferGuardianPaused`function with signature `transferGuardianPaused()` and selector `[135, 247, 99, 3]`"]
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
    #[doc = "Container type for all input parameters for the `transferVerify`function with signature `transferVerify(address,address,address,uint256)` and selector `[106, 86, 148, 126]`"]
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
    pub enum ComptrollerCalls {
        AddCompMarkets(AddCompMarketsCall),
        Become(BecomeCall),
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
    impl ethers::core::abi::AbiDecode for ComptrollerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddCompMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::AddCompMarkets(decoded));
            }
            if let Ok(decoded) = <BecomeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::Become(decoded));
            }
            if let Ok(decoded) =
                <_BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <DropCompMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::DropCompMarket(decoded));
            }
            if let Ok(decoded) =
                <_MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetBorrowPaused(decoded));
            }
            if let Ok(decoded) =
                <SetCloseFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetCloseFactor(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetCollateralFactor(decoded));
            }
            if let Ok(decoded) =
                <SetCompRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetCompRate(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationIncentiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetLiquidationIncentive(decoded));
            }
            if let Ok(decoded) =
                <SetMaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetMaxAssets(decoded));
            }
            if let Ok(decoded) =
                <SetMintPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetMintPaused(decoded));
            }
            if let Ok(decoded) =
                <SetPauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetPauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetSeizePausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetSeizePaused(decoded));
            }
            if let Ok(decoded) =
                <SetTransferPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SetTransferPaused(decoded));
            }
            if let Ok(decoded) =
                <SupportMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SupportMarket(decoded));
            }
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::AllMarkets(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::BorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <BorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::BorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <CheckMembershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CheckMembership(decoded));
            }
            if let Ok(decoded) =
                <ClaimCompWithCTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::ClaimCompWithCTokens(decoded));
            }
            if let Ok (decoded) = < ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ComptrollerCalls :: ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers (decoded)) }
            if let Ok(decoded) =
                <ClaimCompCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::ClaimComp(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <CompAccruedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompAccrued(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompBorrowState(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowerIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompBorrowerIndex(decoded));
            }
            if let Ok(decoded) =
                <CompClaimThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompClaimThreshold(decoded));
            }
            if let Ok(decoded) =
                <CompInitialIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompInitialIndex(decoded));
            }
            if let Ok(decoded) =
                <CompRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompRate(decoded));
            }
            if let Ok(decoded) =
                <CompSpeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompSpeeds(decoded));
            }
            if let Ok(decoded) =
                <CompSupplierIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompSupplierIndex(decoded));
            }
            if let Ok(decoded) =
                <CompSupplyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::CompSupplyState(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerCalls::ComptrollerImplementation(decoded));
            }
            if let Ok(decoded) =
                <EnterMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::EnterMarkets(decoded));
            }
            if let Ok(decoded) =
                <ExitMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::ExitMarket(decoded));
            }
            if let Ok(decoded) =
                <GetAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::GetAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetAllMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::GetAllMarkets(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::GetAssetsIn(decoded));
            }
            if let Ok(decoded) =
                <GetBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::GetBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetCompAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::GetCompAddress(decoded));
            }
            if let Ok(decoded) =
                <GetHypotheticalAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerCalls::GetHypotheticalAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <IsComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::IsComptroller(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::LiquidateBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::LiquidateBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCalculateSeizeTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerCalls::LiquidateCalculateSeizeTokens(decoded));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerCalls::LiquidationIncentiveMantissa(decoded));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::Markets(decoded));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::MaxAssets(decoded));
            }
            if let Ok(decoded) =
                <MintAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::MintAllowed(decoded));
            }
            if let Ok(decoded) =
                <MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <MintVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::MintVerify(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::PauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerCalls::PendingComptrollerImplementation(decoded));
            }
            if let Ok(decoded) =
                <RedeemAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::RedeemAllowed(decoded));
            }
            if let Ok(decoded) =
                <RedeemVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::RedeemVerify(decoded));
            }
            if let Ok(decoded) =
                <RefreshCompSpeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::RefreshCompSpeeds(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::RepayBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::RepayBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <SeizeAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SeizeAllowed(decoded));
            }
            if let Ok(decoded) =
                <SeizeGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SeizeVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::SeizeVerify(decoded));
            }
            if let Ok(decoded) =
                <TransferAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::TransferAllowed(decoded));
            }
            if let Ok(decoded) =
                <TransferGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::TransferGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <TransferVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerCalls::TransferVerify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerCalls::AddCompMarkets(element) => element.encode(),
                ComptrollerCalls::Become(element) => element.encode(),
                ComptrollerCalls::_BorrowGuardianPaused(element) => element.encode(),
                ComptrollerCalls::DropCompMarket(element) => element.encode(),
                ComptrollerCalls::_MintGuardianPaused(element) => element.encode(),
                ComptrollerCalls::SetBorrowPaused(element) => element.encode(),
                ComptrollerCalls::SetCloseFactor(element) => element.encode(),
                ComptrollerCalls::SetCollateralFactor(element) => element.encode(),
                ComptrollerCalls::SetCompRate(element) => element.encode(),
                ComptrollerCalls::SetLiquidationIncentive(element) => element.encode(),
                ComptrollerCalls::SetMaxAssets(element) => element.encode(),
                ComptrollerCalls::SetMintPaused(element) => element.encode(),
                ComptrollerCalls::SetPauseGuardian(element) => element.encode(),
                ComptrollerCalls::SetPriceOracle(element) => element.encode(),
                ComptrollerCalls::SetSeizePaused(element) => element.encode(),
                ComptrollerCalls::SetTransferPaused(element) => element.encode(),
                ComptrollerCalls::SupportMarket(element) => element.encode(),
                ComptrollerCalls::AccountAssets(element) => element.encode(),
                ComptrollerCalls::Admin(element) => element.encode(),
                ComptrollerCalls::AllMarkets(element) => element.encode(),
                ComptrollerCalls::BorrowAllowed(element) => element.encode(),
                ComptrollerCalls::BorrowGuardianPaused(element) => element.encode(),
                ComptrollerCalls::BorrowVerify(element) => element.encode(),
                ComptrollerCalls::CheckMembership(element) => element.encode(),
                ComptrollerCalls::ClaimCompWithCTokens(element) => element.encode(),
                ComptrollerCalls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(
                    element,
                ) => element.encode(),
                ComptrollerCalls::ClaimComp(element) => element.encode(),
                ComptrollerCalls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerCalls::CompAccrued(element) => element.encode(),
                ComptrollerCalls::CompBorrowState(element) => element.encode(),
                ComptrollerCalls::CompBorrowerIndex(element) => element.encode(),
                ComptrollerCalls::CompClaimThreshold(element) => element.encode(),
                ComptrollerCalls::CompInitialIndex(element) => element.encode(),
                ComptrollerCalls::CompRate(element) => element.encode(),
                ComptrollerCalls::CompSpeeds(element) => element.encode(),
                ComptrollerCalls::CompSupplierIndex(element) => element.encode(),
                ComptrollerCalls::CompSupplyState(element) => element.encode(),
                ComptrollerCalls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerCalls::EnterMarkets(element) => element.encode(),
                ComptrollerCalls::ExitMarket(element) => element.encode(),
                ComptrollerCalls::GetAccountLiquidity(element) => element.encode(),
                ComptrollerCalls::GetAllMarkets(element) => element.encode(),
                ComptrollerCalls::GetAssetsIn(element) => element.encode(),
                ComptrollerCalls::GetBlockNumber(element) => element.encode(),
                ComptrollerCalls::GetCompAddress(element) => element.encode(),
                ComptrollerCalls::GetHypotheticalAccountLiquidity(element) => element.encode(),
                ComptrollerCalls::IsComptroller(element) => element.encode(),
                ComptrollerCalls::LiquidateBorrowAllowed(element) => element.encode(),
                ComptrollerCalls::LiquidateBorrowVerify(element) => element.encode(),
                ComptrollerCalls::LiquidateCalculateSeizeTokens(element) => element.encode(),
                ComptrollerCalls::LiquidationIncentiveMantissa(element) => element.encode(),
                ComptrollerCalls::Markets(element) => element.encode(),
                ComptrollerCalls::MaxAssets(element) => element.encode(),
                ComptrollerCalls::MintAllowed(element) => element.encode(),
                ComptrollerCalls::MintGuardianPaused(element) => element.encode(),
                ComptrollerCalls::MintVerify(element) => element.encode(),
                ComptrollerCalls::Oracle(element) => element.encode(),
                ComptrollerCalls::PauseGuardian(element) => element.encode(),
                ComptrollerCalls::PendingAdmin(element) => element.encode(),
                ComptrollerCalls::PendingComptrollerImplementation(element) => element.encode(),
                ComptrollerCalls::RedeemAllowed(element) => element.encode(),
                ComptrollerCalls::RedeemVerify(element) => element.encode(),
                ComptrollerCalls::RefreshCompSpeeds(element) => element.encode(),
                ComptrollerCalls::RepayBorrowAllowed(element) => element.encode(),
                ComptrollerCalls::RepayBorrowVerify(element) => element.encode(),
                ComptrollerCalls::SeizeAllowed(element) => element.encode(),
                ComptrollerCalls::SeizeGuardianPaused(element) => element.encode(),
                ComptrollerCalls::SeizeVerify(element) => element.encode(),
                ComptrollerCalls::TransferAllowed(element) => element.encode(),
                ComptrollerCalls::TransferGuardianPaused(element) => element.encode(),
                ComptrollerCalls::TransferVerify(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerCalls::AddCompMarkets(element) => element.fmt(f),
                ComptrollerCalls::Become(element) => element.fmt(f),
                ComptrollerCalls::_BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::DropCompMarket(element) => element.fmt(f),
                ComptrollerCalls::_MintGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::SetBorrowPaused(element) => element.fmt(f),
                ComptrollerCalls::SetCloseFactor(element) => element.fmt(f),
                ComptrollerCalls::SetCollateralFactor(element) => element.fmt(f),
                ComptrollerCalls::SetCompRate(element) => element.fmt(f),
                ComptrollerCalls::SetLiquidationIncentive(element) => element.fmt(f),
                ComptrollerCalls::SetMaxAssets(element) => element.fmt(f),
                ComptrollerCalls::SetMintPaused(element) => element.fmt(f),
                ComptrollerCalls::SetPauseGuardian(element) => element.fmt(f),
                ComptrollerCalls::SetPriceOracle(element) => element.fmt(f),
                ComptrollerCalls::SetSeizePaused(element) => element.fmt(f),
                ComptrollerCalls::SetTransferPaused(element) => element.fmt(f),
                ComptrollerCalls::SupportMarket(element) => element.fmt(f),
                ComptrollerCalls::AccountAssets(element) => element.fmt(f),
                ComptrollerCalls::Admin(element) => element.fmt(f),
                ComptrollerCalls::AllMarkets(element) => element.fmt(f),
                ComptrollerCalls::BorrowAllowed(element) => element.fmt(f),
                ComptrollerCalls::BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::BorrowVerify(element) => element.fmt(f),
                ComptrollerCalls::CheckMembership(element) => element.fmt(f),
                ComptrollerCalls::ClaimCompWithCTokens(element) => element.fmt(f),
                ComptrollerCalls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(
                    element,
                ) => element.fmt(f),
                ComptrollerCalls::ClaimComp(element) => element.fmt(f),
                ComptrollerCalls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerCalls::CompAccrued(element) => element.fmt(f),
                ComptrollerCalls::CompBorrowState(element) => element.fmt(f),
                ComptrollerCalls::CompBorrowerIndex(element) => element.fmt(f),
                ComptrollerCalls::CompClaimThreshold(element) => element.fmt(f),
                ComptrollerCalls::CompInitialIndex(element) => element.fmt(f),
                ComptrollerCalls::CompRate(element) => element.fmt(f),
                ComptrollerCalls::CompSpeeds(element) => element.fmt(f),
                ComptrollerCalls::CompSupplierIndex(element) => element.fmt(f),
                ComptrollerCalls::CompSupplyState(element) => element.fmt(f),
                ComptrollerCalls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerCalls::EnterMarkets(element) => element.fmt(f),
                ComptrollerCalls::ExitMarket(element) => element.fmt(f),
                ComptrollerCalls::GetAccountLiquidity(element) => element.fmt(f),
                ComptrollerCalls::GetAllMarkets(element) => element.fmt(f),
                ComptrollerCalls::GetAssetsIn(element) => element.fmt(f),
                ComptrollerCalls::GetBlockNumber(element) => element.fmt(f),
                ComptrollerCalls::GetCompAddress(element) => element.fmt(f),
                ComptrollerCalls::GetHypotheticalAccountLiquidity(element) => element.fmt(f),
                ComptrollerCalls::IsComptroller(element) => element.fmt(f),
                ComptrollerCalls::LiquidateBorrowAllowed(element) => element.fmt(f),
                ComptrollerCalls::LiquidateBorrowVerify(element) => element.fmt(f),
                ComptrollerCalls::LiquidateCalculateSeizeTokens(element) => element.fmt(f),
                ComptrollerCalls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerCalls::Markets(element) => element.fmt(f),
                ComptrollerCalls::MaxAssets(element) => element.fmt(f),
                ComptrollerCalls::MintAllowed(element) => element.fmt(f),
                ComptrollerCalls::MintGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::MintVerify(element) => element.fmt(f),
                ComptrollerCalls::Oracle(element) => element.fmt(f),
                ComptrollerCalls::PauseGuardian(element) => element.fmt(f),
                ComptrollerCalls::PendingAdmin(element) => element.fmt(f),
                ComptrollerCalls::PendingComptrollerImplementation(element) => element.fmt(f),
                ComptrollerCalls::RedeemAllowed(element) => element.fmt(f),
                ComptrollerCalls::RedeemVerify(element) => element.fmt(f),
                ComptrollerCalls::RefreshCompSpeeds(element) => element.fmt(f),
                ComptrollerCalls::RepayBorrowAllowed(element) => element.fmt(f),
                ComptrollerCalls::RepayBorrowVerify(element) => element.fmt(f),
                ComptrollerCalls::SeizeAllowed(element) => element.fmt(f),
                ComptrollerCalls::SeizeGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::SeizeVerify(element) => element.fmt(f),
                ComptrollerCalls::TransferAllowed(element) => element.fmt(f),
                ComptrollerCalls::TransferGuardianPaused(element) => element.fmt(f),
                ComptrollerCalls::TransferVerify(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddCompMarketsCall> for ComptrollerCalls {
        fn from(var: AddCompMarketsCall) -> Self {
            ComptrollerCalls::AddCompMarkets(var)
        }
    }
    impl ::std::convert::From<BecomeCall> for ComptrollerCalls {
        fn from(var: BecomeCall) -> Self {
            ComptrollerCalls::Become(var)
        }
    }
    impl ::std::convert::From<_BorrowGuardianPausedCall> for ComptrollerCalls {
        fn from(var: _BorrowGuardianPausedCall) -> Self {
            ComptrollerCalls::_BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<DropCompMarketCall> for ComptrollerCalls {
        fn from(var: DropCompMarketCall) -> Self {
            ComptrollerCalls::DropCompMarket(var)
        }
    }
    impl ::std::convert::From<_MintGuardianPausedCall> for ComptrollerCalls {
        fn from(var: _MintGuardianPausedCall) -> Self {
            ComptrollerCalls::_MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SetBorrowPausedCall> for ComptrollerCalls {
        fn from(var: SetBorrowPausedCall) -> Self {
            ComptrollerCalls::SetBorrowPaused(var)
        }
    }
    impl ::std::convert::From<SetCloseFactorCall> for ComptrollerCalls {
        fn from(var: SetCloseFactorCall) -> Self {
            ComptrollerCalls::SetCloseFactor(var)
        }
    }
    impl ::std::convert::From<SetCollateralFactorCall> for ComptrollerCalls {
        fn from(var: SetCollateralFactorCall) -> Self {
            ComptrollerCalls::SetCollateralFactor(var)
        }
    }
    impl ::std::convert::From<SetCompRateCall> for ComptrollerCalls {
        fn from(var: SetCompRateCall) -> Self {
            ComptrollerCalls::SetCompRate(var)
        }
    }
    impl ::std::convert::From<SetLiquidationIncentiveCall> for ComptrollerCalls {
        fn from(var: SetLiquidationIncentiveCall) -> Self {
            ComptrollerCalls::SetLiquidationIncentive(var)
        }
    }
    impl ::std::convert::From<SetMaxAssetsCall> for ComptrollerCalls {
        fn from(var: SetMaxAssetsCall) -> Self {
            ComptrollerCalls::SetMaxAssets(var)
        }
    }
    impl ::std::convert::From<SetMintPausedCall> for ComptrollerCalls {
        fn from(var: SetMintPausedCall) -> Self {
            ComptrollerCalls::SetMintPaused(var)
        }
    }
    impl ::std::convert::From<SetPauseGuardianCall> for ComptrollerCalls {
        fn from(var: SetPauseGuardianCall) -> Self {
            ComptrollerCalls::SetPauseGuardian(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for ComptrollerCalls {
        fn from(var: SetPriceOracleCall) -> Self {
            ComptrollerCalls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetSeizePausedCall> for ComptrollerCalls {
        fn from(var: SetSeizePausedCall) -> Self {
            ComptrollerCalls::SetSeizePaused(var)
        }
    }
    impl ::std::convert::From<SetTransferPausedCall> for ComptrollerCalls {
        fn from(var: SetTransferPausedCall) -> Self {
            ComptrollerCalls::SetTransferPaused(var)
        }
    }
    impl ::std::convert::From<SupportMarketCall> for ComptrollerCalls {
        fn from(var: SupportMarketCall) -> Self {
            ComptrollerCalls::SupportMarket(var)
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerCalls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerCalls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerCalls {
        fn from(var: AdminCall) -> Self {
            ComptrollerCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllMarketsCall> for ComptrollerCalls {
        fn from(var: AllMarketsCall) -> Self {
            ComptrollerCalls::AllMarkets(var)
        }
    }
    impl ::std::convert::From<BorrowAllowedCall> for ComptrollerCalls {
        fn from(var: BorrowAllowedCall) -> Self {
            ComptrollerCalls::BorrowAllowed(var)
        }
    }
    impl ::std::convert::From<BorrowGuardianPausedCall> for ComptrollerCalls {
        fn from(var: BorrowGuardianPausedCall) -> Self {
            ComptrollerCalls::BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<BorrowVerifyCall> for ComptrollerCalls {
        fn from(var: BorrowVerifyCall) -> Self {
            ComptrollerCalls::BorrowVerify(var)
        }
    }
    impl ::std::convert::From<CheckMembershipCall> for ComptrollerCalls {
        fn from(var: CheckMembershipCall) -> Self {
            ComptrollerCalls::CheckMembership(var)
        }
    }
    impl ::std::convert::From<ClaimCompWithCTokensCall> for ComptrollerCalls {
        fn from(var: ClaimCompWithCTokensCall) -> Self {
            ComptrollerCalls::ClaimCompWithCTokens(var)
        }
    }
    impl ::std::convert::From<ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall>
        for ComptrollerCalls
    {
        fn from(var: ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliersCall) -> Self {
            ComptrollerCalls::ClaimCompWithHoldersAndCTokensAndBorrowersAndSuppliers(var)
        }
    }
    impl ::std::convert::From<ClaimCompCall> for ComptrollerCalls {
        fn from(var: ClaimCompCall) -> Self {
            ComptrollerCalls::ClaimComp(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerCalls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerCalls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<CompAccruedCall> for ComptrollerCalls {
        fn from(var: CompAccruedCall) -> Self {
            ComptrollerCalls::CompAccrued(var)
        }
    }
    impl ::std::convert::From<CompBorrowStateCall> for ComptrollerCalls {
        fn from(var: CompBorrowStateCall) -> Self {
            ComptrollerCalls::CompBorrowState(var)
        }
    }
    impl ::std::convert::From<CompBorrowerIndexCall> for ComptrollerCalls {
        fn from(var: CompBorrowerIndexCall) -> Self {
            ComptrollerCalls::CompBorrowerIndex(var)
        }
    }
    impl ::std::convert::From<CompClaimThresholdCall> for ComptrollerCalls {
        fn from(var: CompClaimThresholdCall) -> Self {
            ComptrollerCalls::CompClaimThreshold(var)
        }
    }
    impl ::std::convert::From<CompInitialIndexCall> for ComptrollerCalls {
        fn from(var: CompInitialIndexCall) -> Self {
            ComptrollerCalls::CompInitialIndex(var)
        }
    }
    impl ::std::convert::From<CompRateCall> for ComptrollerCalls {
        fn from(var: CompRateCall) -> Self {
            ComptrollerCalls::CompRate(var)
        }
    }
    impl ::std::convert::From<CompSpeedsCall> for ComptrollerCalls {
        fn from(var: CompSpeedsCall) -> Self {
            ComptrollerCalls::CompSpeeds(var)
        }
    }
    impl ::std::convert::From<CompSupplierIndexCall> for ComptrollerCalls {
        fn from(var: CompSupplierIndexCall) -> Self {
            ComptrollerCalls::CompSupplierIndex(var)
        }
    }
    impl ::std::convert::From<CompSupplyStateCall> for ComptrollerCalls {
        fn from(var: CompSupplyStateCall) -> Self {
            ComptrollerCalls::CompSupplyState(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerCalls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerCalls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<EnterMarketsCall> for ComptrollerCalls {
        fn from(var: EnterMarketsCall) -> Self {
            ComptrollerCalls::EnterMarkets(var)
        }
    }
    impl ::std::convert::From<ExitMarketCall> for ComptrollerCalls {
        fn from(var: ExitMarketCall) -> Self {
            ComptrollerCalls::ExitMarket(var)
        }
    }
    impl ::std::convert::From<GetAccountLiquidityCall> for ComptrollerCalls {
        fn from(var: GetAccountLiquidityCall) -> Self {
            ComptrollerCalls::GetAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<GetAllMarketsCall> for ComptrollerCalls {
        fn from(var: GetAllMarketsCall) -> Self {
            ComptrollerCalls::GetAllMarkets(var)
        }
    }
    impl ::std::convert::From<GetAssetsInCall> for ComptrollerCalls {
        fn from(var: GetAssetsInCall) -> Self {
            ComptrollerCalls::GetAssetsIn(var)
        }
    }
    impl ::std::convert::From<GetBlockNumberCall> for ComptrollerCalls {
        fn from(var: GetBlockNumberCall) -> Self {
            ComptrollerCalls::GetBlockNumber(var)
        }
    }
    impl ::std::convert::From<GetCompAddressCall> for ComptrollerCalls {
        fn from(var: GetCompAddressCall) -> Self {
            ComptrollerCalls::GetCompAddress(var)
        }
    }
    impl ::std::convert::From<GetHypotheticalAccountLiquidityCall> for ComptrollerCalls {
        fn from(var: GetHypotheticalAccountLiquidityCall) -> Self {
            ComptrollerCalls::GetHypotheticalAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<IsComptrollerCall> for ComptrollerCalls {
        fn from(var: IsComptrollerCall) -> Self {
            ComptrollerCalls::IsComptroller(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowAllowedCall> for ComptrollerCalls {
        fn from(var: LiquidateBorrowAllowedCall) -> Self {
            ComptrollerCalls::LiquidateBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowVerifyCall> for ComptrollerCalls {
        fn from(var: LiquidateBorrowVerifyCall) -> Self {
            ComptrollerCalls::LiquidateBorrowVerify(var)
        }
    }
    impl ::std::convert::From<LiquidateCalculateSeizeTokensCall> for ComptrollerCalls {
        fn from(var: LiquidateCalculateSeizeTokensCall) -> Self {
            ComptrollerCalls::LiquidateCalculateSeizeTokens(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerCalls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerCalls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerCalls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerCalls::Markets(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerCalls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerCalls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<MintAllowedCall> for ComptrollerCalls {
        fn from(var: MintAllowedCall) -> Self {
            ComptrollerCalls::MintAllowed(var)
        }
    }
    impl ::std::convert::From<MintGuardianPausedCall> for ComptrollerCalls {
        fn from(var: MintGuardianPausedCall) -> Self {
            ComptrollerCalls::MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<MintVerifyCall> for ComptrollerCalls {
        fn from(var: MintVerifyCall) -> Self {
            ComptrollerCalls::MintVerify(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerCalls {
        fn from(var: OracleCall) -> Self {
            ComptrollerCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PauseGuardianCall> for ComptrollerCalls {
        fn from(var: PauseGuardianCall) -> Self {
            ComptrollerCalls::PauseGuardian(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerCalls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerCalls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerCalls::PendingComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<RedeemAllowedCall> for ComptrollerCalls {
        fn from(var: RedeemAllowedCall) -> Self {
            ComptrollerCalls::RedeemAllowed(var)
        }
    }
    impl ::std::convert::From<RedeemVerifyCall> for ComptrollerCalls {
        fn from(var: RedeemVerifyCall) -> Self {
            ComptrollerCalls::RedeemVerify(var)
        }
    }
    impl ::std::convert::From<RefreshCompSpeedsCall> for ComptrollerCalls {
        fn from(var: RefreshCompSpeedsCall) -> Self {
            ComptrollerCalls::RefreshCompSpeeds(var)
        }
    }
    impl ::std::convert::From<RepayBorrowAllowedCall> for ComptrollerCalls {
        fn from(var: RepayBorrowAllowedCall) -> Self {
            ComptrollerCalls::RepayBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<RepayBorrowVerifyCall> for ComptrollerCalls {
        fn from(var: RepayBorrowVerifyCall) -> Self {
            ComptrollerCalls::RepayBorrowVerify(var)
        }
    }
    impl ::std::convert::From<SeizeAllowedCall> for ComptrollerCalls {
        fn from(var: SeizeAllowedCall) -> Self {
            ComptrollerCalls::SeizeAllowed(var)
        }
    }
    impl ::std::convert::From<SeizeGuardianPausedCall> for ComptrollerCalls {
        fn from(var: SeizeGuardianPausedCall) -> Self {
            ComptrollerCalls::SeizeGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SeizeVerifyCall> for ComptrollerCalls {
        fn from(var: SeizeVerifyCall) -> Self {
            ComptrollerCalls::SeizeVerify(var)
        }
    }
    impl ::std::convert::From<TransferAllowedCall> for ComptrollerCalls {
        fn from(var: TransferAllowedCall) -> Self {
            ComptrollerCalls::TransferAllowed(var)
        }
    }
    impl ::std::convert::From<TransferGuardianPausedCall> for ComptrollerCalls {
        fn from(var: TransferGuardianPausedCall) -> Self {
            ComptrollerCalls::TransferGuardianPaused(var)
        }
    }
    impl ::std::convert::From<TransferVerifyCall> for ComptrollerCalls {
        fn from(var: TransferVerifyCall) -> Self {
            ComptrollerCalls::TransferVerify(var)
        }
    }
}
