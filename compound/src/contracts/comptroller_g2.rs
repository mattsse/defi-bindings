pub use comptrollerg2_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerg2_mod {
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
    #[doc = "ComptrollerG2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERG2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"action\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"pauseState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ActionPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketEntered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketExited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketListed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCloseFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"oldCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCollateralFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLiquidationIncentive\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMaxAssets\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPauseGuardian\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"oldPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract PriceOracle\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPriceOracle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract Unitroller\",\"name\":\"unitroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_become\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setBorrowPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCloseFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCloseFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newCollateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setCollateralFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newLiquidationIncentiveMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setLiquidationIncentive\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newMaxAssets\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMaxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setMintPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPauseGuardian\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPauseGuardian\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"newOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPriceOracle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setSeizePaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setTransferPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_supportMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkMembership\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterMarkets\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsIn\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenModify\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHypotheticalAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isComptroller\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidateCalculateSeizeTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isListed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualMintAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintVerify\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauseGuardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"actualRepayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowerIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"seizeGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transferGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferVerify\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERG2_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b03191633179055613500806100326000396000f3fe608060405234801561001057600080fd5b50600436106102f05760003560e01c80636d35bf911161019d578063c2998238116100e9578063dcfbc0c7116100a2578063e87554461161007c578063e875544614610b52578063eabe7d9114610b5a578063ede4edd014610b90578063f851a44014610bb6576102f0565b8063dcfbc0c714610b16578063e4028eee14610b1e578063e6653f3d14610b4a576102f0565b8063c29982381461095f578063c488847b14610a02578063d02f735114610a51578063d9226ced14610a97578063da3d454c14610ab4578063dce1544914610aea576102f0565b8063929fe9a111610156578063abfceffc11610130578063abfceffc1461089d578063ac0b0bb714610913578063bb82aa5e1461091b578063bdcdc25814610923576102f0565b8063929fe9a11461084157806394b2294b1461086f578063a76b3fda14610877576102f0565b80636d35bf911461075e578063731f0c2b146107a45780637dc0d1d0146107ca57806387f76303146107d25780638e8f294b146107da5780638ebf636414610822576102f0565b806347ef3b3b1161025c57806355ee1fe1116102155780635f5af1aa116101ef5780635f5af1aa146106905780635fc7e71e146106b65780636a56947e146106fc5780636d154ea514610738576102f0565b806355ee1fe11461060e5780635c778605146106345780635ec88c791461066a576102f0565b806347ef3b3b146104d15780634ada90af1461051d5780634e79238f146105255780634ef4c3e11461057f5780634fd42e17146105b557806351dff989146105d2576102f0565b806326782247116102ae578063267822471461041b5780632d70db7814610423578063317b0b77146104425780633bcf7ec11461045f5780633c94786f1461048d57806341c728b914610495576102f0565b80627e3dd2146102f557806318c882a5146103115780631d504dc61461033f5780631ededc911461036757806324008a62146103a957806324a3d622146103f7575b600080fd5b6102fd610bbe565b604080519115158252519081900360200190f35b6102fd6004803603604081101561032757600080fd5b506001600160a01b0381351690602001351515610bc3565b6103656004803603602081101561035557600080fd5b50356001600160a01b0316610d63565b005b610365600480360360a081101561037d57600080fd5b506001600160a01b03813581169160208101358216916040820135169060608101359060800135610ec8565b6103e5600480360360808110156103bf57600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135610ecf565b60408051918252519081900360200190f35b6103ff610f05565b604080516001600160a01b039092168252519081900360200190f35b6103ff610f14565b6102fd6004803603602081101561043957600080fd5b50351515610f23565b6103e56004803603602081101561045857600080fd5b503561105d565b6102fd6004803603604081101561047557600080fd5b506001600160a01b038135169060200135151561116e565b6102fd611309565b610365600480360360808110156104ab57600080fd5b506001600160a01b03813581169160208101359091169060408101359060600135611319565b610365600480360360c08110156104e757600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060808101359060a0013561131f565b6103e5611327565b6105616004803603608081101561053b57600080fd5b506001600160a01b0381358116916020810135909116906040810135906060013561132d565b60408051938452602084019290925282820152519081900360600190f35b6103e56004803603606081101561059557600080fd5b506001600160a01b03813581169160208101359091169060400135611367565b6103e5600480360360208110156105cb57600080fd5b50356113fd565b610365600480360360808110156105e857600080fd5b506001600160a01b038135811691602081013590911690604081013590606001356114f1565b6103e56004803603602081101561062457600080fd5b50356001600160a01b0316611545565b6103656004803603606081101561064a57600080fd5b506001600160a01b038135811691602081013590911690604001356115cc565b6105616004803603602081101561068057600080fd5b50356001600160a01b03166115d1565b6103e5600480360360208110156106a657600080fd5b50356001600160a01b0316611606565b6103e5600480360360a08110156106cc57600080fd5b506001600160a01b038135811691602081013582169160408201358116916060810135909116906080013561168a565b6103656004803603608081101561071257600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135611319565b6102fd6004803603602081101561074e57600080fd5b50356001600160a01b0316611811565b610365600480360360a081101561077457600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060800135610ec8565b6102fd600480360360208110156107ba57600080fd5b50356001600160a01b0316611826565b6103ff61183b565b6102fd61184a565b610800600480360360208110156107f057600080fd5b50356001600160a01b031661185a565b6040805193151584526020840192909252151582820152519081900360600190f35b6102fd6004803603602081101561083857600080fd5b50351515611880565b6102fd6004803603604081101561085757600080fd5b506001600160a01b03813581169160200135166119b9565b6103e56119ec565b6103e56004803603602081101561088d57600080fd5b50356001600160a01b03166119f2565b6108c3600480360360208110156108b357600080fd5b50356001600160a01b0316611b40565b60408051602080825283518183015283519192839290830191858101910280838360005b838110156108ff5781810151838201526020016108e7565b505050509050019250505060405180910390f35b6102fd611bc9565b6103ff611bd9565b6103e56004803603608081101561093957600080fd5b506001600160a01b03813581169160208101358216916040820135169060600135611be8565b6108c36004803603602081101561097557600080fd5b81019060208101813564010000000081111561099057600080fd5b8201836020820111156109a257600080fd5b803590602001918460208302840111640100000000831117156109c457600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929550611c4a945050505050565b610a3860048036036060811015610a1857600080fd5b506001600160a01b03813581169160208101359091169060400135611ce1565b6040805192835260208301919091528051918290030190f35b6103e5600480360360a0811015610a6757600080fd5b506001600160a01b0381358116916020810135821691604082013581169160608101359091169060800135611f56565b6103e560048036036020811015610aad57600080fd5b50356120ed565b6103e560048036036060811015610aca57600080fd5b506001600160a01b03813581169160208101359091169060400135612156565b6103ff60048036036040811015610b0057600080fd5b506001600160a01b0381351690602001356123a3565b6103ff6123d8565b6103e560048036036040811015610b3457600080fd5b506001600160a01b0381351690602001356123e7565b6102fd612597565b6103e56125a7565b6103e560048036036060811015610b7057600080fd5b506001600160a01b038135811691602081013590911690604001356125ad565b6103e560048036036020811015610ba657600080fd5b50356001600160a01b03166125ba565b6103ff6128cd565b600181565b6001600160a01b03821660009081526009602052604081205460ff16610c1a5760405162461bcd60e51b81526004018080602001828103825260288152602001806134316028913960400191505060405180910390fd5b600a546001600160a01b0316331480610c3d57506000546001600160a01b031633145b610c785760405162461bcd60e51b81526004018080602001828103825260278152602001806134596027913960400191505060405180910390fd5b6000546001600160a01b0316331480610c9357506001821515145b610cdd576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600c6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260069083015265426f72726f7760d01b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150805b92915050565b806001600160a01b031663f851a4406040518163ffffffff1660e01b815260040160206040518083038186803b158015610d9c57600080fd5b505afa158015610db0573d6000803e3d6000fd5b505050506040513d6020811015610dc657600080fd5b50516001600160a01b03163314610e0e5760405162461bcd60e51b81526004018080602001828103825260278152602001806134a56027913960400191505060405180910390fd5b6000816001600160a01b031663c1e803346040518163ffffffff1660e01b8152600401602060405180830381600087803b158015610e4b57600080fd5b505af1158015610e5f573d6000803e3d6000fd5b505050506040513d6020811015610e7557600080fd5b505190508015610ec4576040805162461bcd60e51b815260206004820152601560248201527418da185b99d9481b9bdd08185d5d1a1bdc9a5e9959605a1b604482015290519081900360640190fd5b5050565b5050505050565b6001600160a01b03841660009081526009602052604081205460ff16610ef757506009610efd565b60005b90505b949350505050565b600a546001600160a01b031681565b6001546001600160a01b031681565b600a546000906001600160a01b0316331480610f4957506000546001600160a01b031633145b610f845760405162461bcd60e51b81526004018080602001828103825260278152602001806134596027913960400191505060405180910390fd5b6000546001600160a01b0316331480610f9f57506001821515145b610fe9576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b81b810260ff60b81b1990921691909117909155604080516020810192909252808252600582820152645365697a6560d81b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a150805b919050565b600080546001600160a01b031633146110835761107c600160046128dc565b9050611058565b61108b61336e565b5060408051602081019091528281526110a261336e565b50604080516020810190915266b1a2bc2ec5000081526110c28282612942565b156110db576110d26005806128dc565b92505050611058565b6110e361336e565b506040805160208101909152670c7d713b49da00008152611104818461294a565b1561111e576111146005806128dc565b9350505050611058565b6005805490869055604080518281526020810188905281517f3b9670cf975d26958e754b57098eaa2ac914d8d2a31b83257997b9f346110fd9929181900390910190a160005b9695505050505050565b6001600160a01b03821660009081526009602052604081205460ff166111c55760405162461bcd60e51b81526004018080602001828103825260288152602001806134316028913960400191505060405180910390fd5b600a546001600160a01b03163314806111e857506000546001600160a01b031633145b6112235760405162461bcd60e51b81526004018080602001828103825260278152602001806134596027913960400191505060405180910390fd5b6000546001600160a01b031633148061123e57506001821515145b611288576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b6001600160a01b0383166000818152600b6020908152604091829020805486151560ff199091168117909155825193845283830152606090830181905260049083015263135a5b9d60e21b6080830152517f71aec636243f9709bb0007ae15e9afb8150ab01716d75fd7573be5cc096e03b09181900360a00190a150919050565b600a54600160a01b900460ff1681565b50505050565b505050505050565b60065481565b6000806000806000806113428a8a8a8a612951565b92509250925082601181111561135457fe5b95509093509150505b9450945094915050565b6001600160a01b0383166000908152600b602052604081205460ff16156113c6576040805162461bcd60e51b815260206004820152600e60248201526d1b5a5b9d081a5cc81c185d5cd95960921b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff166113f05760095b90506113f6565b60005b90505b9392505050565b600080546001600160a01b0316331461141c5761107c6001600b6128dc565b61142461336e565b50604080516020810190915282815261143b61336e565b506040805160208101909152670de0b6b3a7640000815261145c828261294a565b1561146d576110d26007600c6128dc565b61147561336e565b5060408051602081019091526714d1120d7b1600008152611496818461294a565b156114a7576111146007600c6128dc565b6006805490869055604080518281526020810188905281517faeba5a6c40a8ac138134bff1aaa65debf25971188a58804bad717f82f0ec1316929181900390910190a16000611164565b801580156114ff5750600082115b15611319576040805162461bcd60e51b815260206004820152601160248201527072656465656d546f6b656e73207a65726f60781b604482015290519081900360640190fd5b600080546001600160a01b031633146115645761107c600160106128dc565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fd52b2b9b7e9ee655fcb95d2e5b9e0c9f69e7ef2b8e9d2d0ea78402d576d22e22929181900390910190a160009392505050565b505050565b6000806000806000806115e8876000806000612951565b9250925092508260118111156115fa57fe5b97919650945092505050565b600080546001600160a01b031633146116255761107c600160136128dc565b600a80546001600160a01b038481166001600160a01b0319831617928390556040805192821680845293909116602083015280517f0613b6ee6a04f0d09f390e4d9318894b9f6ac7fd83897cd8d18896ba579c401e9281900390910190a160006113f6565b6001600160a01b03851660009081526009602052604081205460ff1615806116cb57506001600160a01b03851660009081526009602052604090205460ff16155b156116da5760095b9050611808565b6000806116e685612d6c565b919350909150600090508260118111156116fc57fe5b146117165781601181111561170d57fe5b92505050611808565b8061172257600361170d565b6000886001600160a01b03166395dd9193876040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b15801561177a57600080fd5b505afa15801561178e573d6000803e3d6000fd5b505050506040513d60208110156117a457600080fd5b50516040805160208101909152600554815290915060009081906117c89084612d8c565b909250905060008260038111156117db57fe5b146117ef57600b5b95505050505050611808565b808711156117fe5760116117e3565b6000955050505050505b95945050505050565b600c6020526000908152604090205460ff1681565b600b6020526000908152604090205460ff1681565b6004546001600160a01b031681565b600a54600160b01b900460ff1681565b60096020526000908152604090208054600182015460039092015460ff91821692911683565b600a546000906001600160a01b03163314806118a657506000546001600160a01b031633145b6118e15760405162461bcd60e51b81526004018080602001828103825260278152602001806134596027913960400191505060405180910390fd5b6000546001600160a01b03163314806118fc57506001821515145b611946576040805162461bcd60e51b81526020600482015260166024820152756f6e6c792061646d696e2063616e20756e706175736560501b604482015290519081900360640190fd5b600a8054831515600160b01b810260ff60b01b1990921691909117909155604080516020810192909252808252600882820152672a3930b739b332b960c11b6060830152517fef159d9a32b2472e32b098f954f3ce62d232939f1c207070b584df1814de2de09181900360800190a15090565b6001600160a01b038082166000908152600960209081526040808320938616835260029093019052205460ff1692915050565b60075481565b600080546001600160a01b03163314611a115761107c600160126128dc565b6001600160a01b03821660009081526009602052604090205460ff1615611a3e5761107c600a60116128dc565b816001600160a01b031663fe9c44ae6040518163ffffffff1660e01b815260040160206040518083038186803b158015611a7757600080fd5b505afa158015611a8b573d6000803e3d6000fd5b505050506040513d6020811015611aa157600080fd5b5050604080516060810182526001808252600060208381018281528486018381526001600160a01b03891680855260098452938790209551865490151560ff199182161787559151948601949094559251600390940180549415159490931693909317909155825190815291517fcf583bb0c569eb967f806b11601c4cb93c10310485c67add5f8362c2f212321f9281900390910190a1600092915050565b60608060086000846001600160a01b03166001600160a01b03168152602001908152602001600020805480602002602001604051908101604052809291908181526020018280548015611bbc57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311611b9e575b5093979650505050505050565b600a54600160b81b900460ff1681565b6002546001600160a01b031681565b600a54600090600160b01b900460ff1615611c3f576040805162461bcd60e51b81526020600482015260126024820152711d1c985b9cd9995c881a5cc81c185d5cd95960721b604482015290519081900360640190fd5b610efa858584612de0565b6060600082519050606081604051908082528060200260200182016040528015611c7e578160200160208202803883390190505b50905060005b82811015611cd9576000858281518110611c9a57fe5b60200260200101519050611cae8133612e4f565b6011811115611cb957fe5b838381518110611cc557fe5b602090810291909101015250600101611c84565b509392505050565b600480546040805163fc57d4df60e01b81526001600160a01b038781169482019490945290516000938493849391169163fc57d4df91602480820192602092909190829003018186803b158015611d3757600080fd5b505afa158015611d4b573d6000803e3d6000fd5b505050506040513d6020811015611d6157600080fd5b5051600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051939450600093929091169163fc57d4df91602480820192602092909190829003018186803b158015611dba57600080fd5b505afa158015611dce573d6000803e3d6000fd5b505050506040513d6020811015611de457600080fd5b50519050811580611df3575080155b15611e0857600d935060009250611f4e915050565b6000866001600160a01b031663182df0f56040518163ffffffff1660e01b815260040160206040518083038186803b158015611e4357600080fd5b505afa158015611e57573d6000803e3d6000fd5b505050506040513d6020811015611e6d57600080fd5b505190506000611e7b61336e565b611e8361336e565b611e8b61336e565b6000611e9960065489612f70565b945090506000816003811115611eab57fe5b14611ec757600b5b995060009850611f4e975050505050505050565b611ed18787612f70565b935090506000816003811115611ee357fe5b14611eef57600b611eb3565b611ef98484612fab565b925090506000816003811115611f0b57fe5b14611f1757600b611eb3565b611f21828c612d8c565b955090506000816003811115611f3357fe5b14611f3f57600b611eb3565b60009950939750505050505050505b935093915050565b600a54600090600160b81b900460ff1615611faa576040805162461bcd60e51b815260206004820152600f60248201526e1cd95a5e99481a5cc81c185d5cd959608a1b604482015290519081900360640190fd5b6001600160a01b03861660009081526009602052604090205460ff161580611feb57506001600160a01b03851660009081526009602052604090205460ff16155b15611ff75760096116d3565b846001600160a01b0316635fe3b5676040518163ffffffff1660e01b815260040160206040518083038186803b15801561203057600080fd5b505afa158015612044573d6000803e3d6000fd5b505050506040513d602081101561205a57600080fd5b505160408051635fe3b56760e01b815290516001600160a01b0392831692891691635fe3b567916004808301926020929190829003018186803b1580156120a057600080fd5b505afa1580156120b4573d6000803e3d6000fd5b505050506040513d60208110156120ca57600080fd5b50516001600160a01b0316146120e15760026116d3565b60009695505050505050565b600080546001600160a01b0316331461210c5761107c6001600d6128dc565b6007805490839055604080518281526020810185905281517f7093cf1eb653f749c3ff531d6df7f92764536a7fa0d13530cd26e070780c32ea929181900390910190a160006113f6565b6001600160a01b0383166000908152600c602052604081205460ff16156121b7576040805162461bcd60e51b815260206004820152601060248201526f189bdc9c9bddc81a5cc81c185d5cd95960821b604482015290519081900360640190fd5b6001600160a01b03841660009081526009602052604090205460ff166121de5760096113e9565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff166122ce57336001600160a01b03851614612264576040805162461bcd60e51b815260206004820152601560248201527439b2b73232b91036bab9ba1031329031aa37b5b2b760591b604482015290519081900360640190fd5b60006122703385612e4f565b9050600081601181111561228057fe5b146122995780601181111561229157fe5b9150506113f6565b6001600160a01b038086166000908152600960209081526040808320938816835260029093019052205460ff166122cc57fe5b505b600480546040805163fc57d4df60e01b81526001600160a01b03888116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b15801561231f57600080fd5b505afa158015612333573d6000803e3d6000fd5b505050506040513d602081101561234957600080fd5b505161235657600d6113e9565b6000806123668587600087612951565b9193509091506000905082601181111561237c57fe5b146123965781601181111561238d57fe5b925050506113f6565b80156120e157600461238d565b600860205281600052604060002081815481106123bc57fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b600080546001600160a01b0316331461240d57612406600160066128dc565b9050610d5d565b6001600160a01b0383166000908152600960205260409020805460ff166124425761243a600960076128dc565b915050610d5d565b61244a61336e565b50604080516020810190915283815261246161336e565b506040805160208101909152670c7d713b49da00008152612482818361294a565b1561249d57612493600660086128dc565b9350505050610d5d565b84158015906125265750600480546040805163fc57d4df60e01b81526001600160a01b038a8116948201949094529051929091169163fc57d4df91602480820192602092909190829003018186803b1580156124f857600080fd5b505afa15801561250c573d6000803e3d6000fd5b505050506040513d602081101561252257600080fd5b5051155b1561253757612493600d60096128dc565b60018301805490869055604080516001600160a01b03891681526020810183905280820188905290517f70483e6592cd5182d45ac970e05bc62cdcc90e9d8ef2c2dbe686cf383bcd7fc59181900360600190a16000979650505050505050565b600a54600160a81b900460ff1681565b60055481565b60006113f3848484612de0565b6000808290506000806000836001600160a01b031663c37f68e2336040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b15801561261b57600080fd5b505afa15801561262f573d6000803e3d6000fd5b505050506040513d608081101561264557600080fd5b5080516020820151604090920151909450909250905082156126985760405162461bcd60e51b81526004018080602001828103825260258152602001806134806025913960400191505060405180910390fd5b80156126b5576126aa600c60026128dc565b945050505050611058565b60006126c2873385612de0565b905080156126e3576126d7600e600383612fc3565b95505050505050611058565b6001600160a01b0385166000908152600960209081526040808320338452600281019092529091205460ff166127225760009650505050505050611058565b3360009081526002820160209081526040808320805460ff19169055600882529182902080548351818402810184019094528084526060939283018282801561279457602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311612776575b5050835193945083925060009150505b828110156127e957896001600160a01b03168482815181106127c257fe5b60200260200101516001600160a01b031614156127e1578091506127e9565b6001016127a4565b508181106127f357fe5b33600090815260086020526040902080548190600019810190811061281457fe5b9060005260206000200160009054906101000a90046001600160a01b031681838154811061283e57fe5b600091825260209091200180546001600160a01b0319166001600160a01b03929092169190911790558054612877826000198301613381565b50604080516001600160a01b038c16815233602082015281517fe699a64c18b07ac5b7301aa273f36a2287239eb9501d81950672794afba29a0d929181900390910190a160009c9b505050505050505050505050565b6000546001600160a01b031681565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa083601181111561290b57fe5b83601381111561291757fe5b604080519283526020830191909152600082820152519081900360600190a18260118111156113f657fe5b519051111590565b5190511090565b600080600061295e6133a5565b6001600160a01b0388166000908152600860209081526040808320805482518185028101850190935280835284936060939291908301828280156129cb57602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116129ad575b50939450600093505050505b8151811015612d275760008282815181106129ee57fe5b60200260200101519050806001600160a01b031663c37f68e28e6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060806040518083038186803b158015612a4e57600080fd5b505afa158015612a62573d6000803e3d6000fd5b505050506040513d6080811015612a7857600080fd5b508051602082015160408084015160609485015160808c0152938a01939093529188019190915294508415612abe5750600f97506000965086955061135d945050505050565b60408051602080820183526001600160a01b0380851660008181526009845285902060010154845260c08b01939093528351808301855260808b0151815260e08b015260048054855163fc57d4df60e01b815291820194909452935192169263fc57d4df9260248083019392829003018186803b158015612b3e57600080fd5b505afa158015612b52573d6000803e3d6000fd5b505050506040513d6020811015612b6857600080fd5b505160a08701819052612b8c5750600d97506000965086955061135d945050505050565b604080516020810190915260a08701518152610100870181905260c087015160e0880151612bb992613029565b61012088015293506000846003811115612bcf57fe5b14612beb5750600b97506000965086955061135d945050505050565b612c0386610120015187604001518860000151613081565b875293506000846003811115612c1557fe5b14612c315750600b97506000965086955061135d945050505050565b612c4986610100015187606001518860200151613081565b602088015293506000846003811115612c5e57fe5b14612c7a5750600b97506000965086955061135d945050505050565b8b6001600160a01b0316816001600160a01b03161415612d1e57612ca88661012001518c8860200151613081565b602088015293506000846003811115612cbd57fe5b14612cd95750600b97506000965086955061135d945050505050565b612ced8661010001518b8860200151613081565b602088015293506000846003811115612d0257fe5b14612d1e5750600b97506000965086955061135d945050505050565b506001016129d7565b50602084015184511115612d4e57505050602081015190516000945003915082905061135d565b505081516020909201516000955085945091909103915061135d9050565b6000806000612d7f846000806000612951565b9250925092509193909250565b6000806000612d9961336e565b612da386866130ce565b90925090506000826003811115612db657fe5b14612dc75750915060009050612dd9565b6000612dd282613136565b9350935050505b9250929050565b6001600160a01b03831660009081526009602052604081205460ff16612e075760096113e9565b6001600160a01b038085166000908152600960209081526040808320938716835260029093019052205460ff16612e3f5760006113e9565b6000806123668587866000612951565b6001600160a01b0382166000908152600960205260408120805460ff16612e7a576009915050610d5d565b6001600160a01b038316600090815260028201602052604090205460ff16151560011415612eac576000915050610d5d565b6007546001600160a01b03841660009081526008602052604090205410612ed7576010915050610d5d565b6001600160a01b0380841660008181526002840160209081526040808320805460ff19166001908117909155600883528184208054918201815584529282902090920180549489166001600160a01b031990951685179055815193845283019190915280517f3ab23ab0d51cccc0c3085aec51f99228625aa1a922b3a8ca89a26b0f2027a1a59281900390910190a15060009392505050565b6000612f7a61336e565b612fa0604051806020016040528086815250604051806020016040528086815250613145565b915091509250929050565b6000612fb561336e565b83518351612fa0919061322e565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0846011811115612ff257fe5b846013811115612ffe57fe5b604080519283526020830191909152818101859052519081900360600190a18360118111156113f357fe5b600061303361336e565b600061303d61336e565b6130478787613145565b9092509050600082600381111561305a57fe5b14613069579092509050611f4e565b6130738186613145565b935093505050935093915050565b600080600061308e61336e565b61309887876130ce565b909250905060008260038111156130ab57fe5b146130bc5750915060009050611f4e565b6130736130c882613136565b866132de565b60006130d861336e565b6000806130e9866000015186613304565b909250905060008260038111156130fc57fe5b1461311b57506040805160208101909152600081529092509050612dd9565b60408051602081019091529081526000969095509350505050565b51670de0b6b3a7640000900490565b600061314f61336e565b60008061316486600001518660000151613304565b9092509050600082600381111561317757fe5b1461319657506040805160208101909152600081529092509050612dd9565b6000806131ab6706f05b59d3b20000846132de565b909250905060008260038111156131be57fe5b146131e057506040805160208101909152600081529094509250612dd9915050565b6000806131f583670de0b6b3a7640000613343565b9092509050600082600381111561320857fe5b1461320f57fe5b604080516020810190915290815260009a909950975050505050505050565b600061323861336e565b60008061324d86670de0b6b3a7640000613304565b9092509050600082600381111561326057fe5b1461327f57506040805160208101909152600081529092509050612dd9565b60008061328c8388613343565b9092509050600082600381111561329f57fe5b146132c157506040805160208101909152600081529094509250612dd9915050565b604080516020810190915290815260009890975095505050505050565b6000808383018481106132f657600092509050612dd9565b506002915060009050612dd9565b6000808361331757506000905080612dd9565b8383028385828161332457fe5b041461333857506002915060009050612dd9565b600092509050612dd9565b600080826133575750600190506000612dd9565b600083858161336257fe5b04915091509250929050565b6040518060200160405280600081525090565b8154818355818111156115cc576000838152602090206115cc91810190830161340f565b6040518061014001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016133e361336e565b81526020016133f061336e565b81526020016133fd61336e565b815260200161340a61336e565b905290565b61342d91905b808211156134295760008155600101613415565b5090565b9056fe63616e6e6f742070617573652061206d61726b65742074686174206973206e6f74206c69737465646f6e6c7920706175736520677561726469616e20616e642061646d696e2063616e207061757365657869744d61726b65743a206765744163636f756e74536e617073686f74206661696c65646f6e6c7920756e6974726f6c6c65722061646d696e2063616e206368616e676520627261696e73a265627a7a72315820b91eeead5e2da423f53ecd72916a27caf9f3c47c5394399c00316542b08edb6764736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ComptrollerG2<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerG2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerG2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerG2))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerG2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMPTROLLERG2_ABI.clone(), client)
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
                COMPTROLLERG2_ABI.clone(),
                COMPTROLLERG2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        #[doc = "Calls the contract's `closeFactorMantissa` (0xe8755446) function"]
        pub fn close_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([232, 117, 84, 70], ())
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
        #[doc = "Gets the contract's `Failure` event"]
        pub fn failure_filter(&self) -> ethers::contract::builders::Event<M, FailureFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ComptrollerG2Events> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ComptrollerG2<M> {
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
    pub enum ComptrollerG2Events {
        ActionPausedFilter(ActionPausedFilter),
        FailureFilter(FailureFilter),
        MarketEnteredFilter(MarketEnteredFilter),
        MarketExitedFilter(MarketExitedFilter),
        MarketListedFilter(MarketListedFilter),
        NewCloseFactorFilter(NewCloseFactorFilter),
        NewCollateralFactorFilter(NewCollateralFactorFilter),
        NewLiquidationIncentiveFilter(NewLiquidationIncentiveFilter),
        NewMaxAssetsFilter(NewMaxAssetsFilter),
        NewPauseGuardianFilter(NewPauseGuardianFilter),
        NewPriceOracleFilter(NewPriceOracleFilter),
    }
    impl ethers::contract::EthLogDecode for ComptrollerG2Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ActionPausedFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::ActionPausedFilter(decoded));
            }
            if let Ok(decoded) = ActionPausedFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::ActionPausedFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::FailureFilter(decoded));
            }
            if let Ok(decoded) = MarketEnteredFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::MarketEnteredFilter(decoded));
            }
            if let Ok(decoded) = MarketExitedFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::MarketExitedFilter(decoded));
            }
            if let Ok(decoded) = MarketListedFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::MarketListedFilter(decoded));
            }
            if let Ok(decoded) = NewCloseFactorFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewCloseFactorFilter(decoded));
            }
            if let Ok(decoded) = NewCollateralFactorFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewCollateralFactorFilter(decoded));
            }
            if let Ok(decoded) = NewLiquidationIncentiveFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewLiquidationIncentiveFilter(decoded));
            }
            if let Ok(decoded) = NewMaxAssetsFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewMaxAssetsFilter(decoded));
            }
            if let Ok(decoded) = NewPauseGuardianFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewPauseGuardianFilter(decoded));
            }
            if let Ok(decoded) = NewPriceOracleFilter::decode_log(log) {
                return Ok(ComptrollerG2Events::NewPriceOracleFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ComptrollerG2Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerG2Events::ActionPausedFilter(element) => element.fmt(f),
                ComptrollerG2Events::FailureFilter(element) => element.fmt(f),
                ComptrollerG2Events::MarketEnteredFilter(element) => element.fmt(f),
                ComptrollerG2Events::MarketExitedFilter(element) => element.fmt(f),
                ComptrollerG2Events::MarketListedFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewCloseFactorFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewCollateralFactorFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewLiquidationIncentiveFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewMaxAssetsFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewPauseGuardianFilter(element) => element.fmt(f),
                ComptrollerG2Events::NewPriceOracleFilter(element) => element.fmt(f),
            }
        }
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
    pub enum ComptrollerG2Calls {
        Become(BecomeCall),
        _BorrowGuardianPaused(_BorrowGuardianPausedCall),
        _MintGuardianPaused(_MintGuardianPausedCall),
        SetBorrowPaused(SetBorrowPausedCall),
        SetCloseFactor(SetCloseFactorCall),
        SetCollateralFactor(SetCollateralFactorCall),
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
        BorrowAllowed(BorrowAllowedCall),
        BorrowGuardianPaused(BorrowGuardianPausedCall),
        BorrowVerify(BorrowVerifyCall),
        CheckMembership(CheckMembershipCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        EnterMarkets(EnterMarketsCall),
        ExitMarket(ExitMarketCall),
        GetAccountLiquidity(GetAccountLiquidityCall),
        GetAssetsIn(GetAssetsInCall),
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
        RepayBorrowAllowed(RepayBorrowAllowedCall),
        RepayBorrowVerify(RepayBorrowVerifyCall),
        SeizeAllowed(SeizeAllowedCall),
        SeizeGuardianPaused(SeizeGuardianPausedCall),
        SeizeVerify(SeizeVerifyCall),
        TransferAllowed(TransferAllowedCall),
        TransferGuardianPaused(TransferGuardianPausedCall),
        TransferVerify(TransferVerifyCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerG2Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BecomeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::Become(decoded));
            }
            if let Ok(decoded) =
                <_BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <_MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetBorrowPaused(decoded));
            }
            if let Ok(decoded) =
                <SetCloseFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetCloseFactor(decoded));
            }
            if let Ok(decoded) =
                <SetCollateralFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetCollateralFactor(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationIncentiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetLiquidationIncentive(decoded));
            }
            if let Ok(decoded) =
                <SetMaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetMaxAssets(decoded));
            }
            if let Ok(decoded) =
                <SetMintPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetMintPaused(decoded));
            }
            if let Ok(decoded) =
                <SetPauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetPauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetSeizePausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetSeizePaused(decoded));
            }
            if let Ok(decoded) =
                <SetTransferPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SetTransferPaused(decoded));
            }
            if let Ok(decoded) =
                <SupportMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SupportMarket(decoded));
            }
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::BorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <BorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::BorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <CheckMembershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::CheckMembership(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG2Calls::ComptrollerImplementation(decoded));
            }
            if let Ok(decoded) =
                <EnterMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::EnterMarkets(decoded));
            }
            if let Ok(decoded) =
                <ExitMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::ExitMarket(decoded));
            }
            if let Ok(decoded) =
                <GetAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::GetAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::GetAssetsIn(decoded));
            }
            if let Ok(decoded) =
                <GetHypotheticalAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG2Calls::GetHypotheticalAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <IsComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::IsComptroller(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::LiquidateBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::LiquidateBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCalculateSeizeTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG2Calls::LiquidateCalculateSeizeTokens(decoded));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG2Calls::LiquidationIncentiveMantissa(decoded));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::Markets(decoded));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::MaxAssets(decoded));
            }
            if let Ok(decoded) =
                <MintAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::MintAllowed(decoded));
            }
            if let Ok(decoded) =
                <MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <MintVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::MintVerify(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::PauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerG2Calls::PendingComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RedeemAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::RedeemAllowed(decoded));
            }
            if let Ok(decoded) =
                <RedeemVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::RedeemVerify(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::RepayBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::RepayBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <SeizeAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SeizeAllowed(decoded));
            }
            if let Ok(decoded) =
                <SeizeGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <SeizeVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::SeizeVerify(decoded));
            }
            if let Ok(decoded) =
                <TransferAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::TransferAllowed(decoded));
            }
            if let Ok(decoded) =
                <TransferGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::TransferGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <TransferVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerG2Calls::TransferVerify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerG2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerG2Calls::Become(element) => element.encode(),
                ComptrollerG2Calls::_BorrowGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::_MintGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::SetBorrowPaused(element) => element.encode(),
                ComptrollerG2Calls::SetCloseFactor(element) => element.encode(),
                ComptrollerG2Calls::SetCollateralFactor(element) => element.encode(),
                ComptrollerG2Calls::SetLiquidationIncentive(element) => element.encode(),
                ComptrollerG2Calls::SetMaxAssets(element) => element.encode(),
                ComptrollerG2Calls::SetMintPaused(element) => element.encode(),
                ComptrollerG2Calls::SetPauseGuardian(element) => element.encode(),
                ComptrollerG2Calls::SetPriceOracle(element) => element.encode(),
                ComptrollerG2Calls::SetSeizePaused(element) => element.encode(),
                ComptrollerG2Calls::SetTransferPaused(element) => element.encode(),
                ComptrollerG2Calls::SupportMarket(element) => element.encode(),
                ComptrollerG2Calls::AccountAssets(element) => element.encode(),
                ComptrollerG2Calls::Admin(element) => element.encode(),
                ComptrollerG2Calls::BorrowAllowed(element) => element.encode(),
                ComptrollerG2Calls::BorrowGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::BorrowVerify(element) => element.encode(),
                ComptrollerG2Calls::CheckMembership(element) => element.encode(),
                ComptrollerG2Calls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerG2Calls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerG2Calls::EnterMarkets(element) => element.encode(),
                ComptrollerG2Calls::ExitMarket(element) => element.encode(),
                ComptrollerG2Calls::GetAccountLiquidity(element) => element.encode(),
                ComptrollerG2Calls::GetAssetsIn(element) => element.encode(),
                ComptrollerG2Calls::GetHypotheticalAccountLiquidity(element) => element.encode(),
                ComptrollerG2Calls::IsComptroller(element) => element.encode(),
                ComptrollerG2Calls::LiquidateBorrowAllowed(element) => element.encode(),
                ComptrollerG2Calls::LiquidateBorrowVerify(element) => element.encode(),
                ComptrollerG2Calls::LiquidateCalculateSeizeTokens(element) => element.encode(),
                ComptrollerG2Calls::LiquidationIncentiveMantissa(element) => element.encode(),
                ComptrollerG2Calls::Markets(element) => element.encode(),
                ComptrollerG2Calls::MaxAssets(element) => element.encode(),
                ComptrollerG2Calls::MintAllowed(element) => element.encode(),
                ComptrollerG2Calls::MintGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::MintVerify(element) => element.encode(),
                ComptrollerG2Calls::Oracle(element) => element.encode(),
                ComptrollerG2Calls::PauseGuardian(element) => element.encode(),
                ComptrollerG2Calls::PendingAdmin(element) => element.encode(),
                ComptrollerG2Calls::PendingComptrollerImplementation(element) => element.encode(),
                ComptrollerG2Calls::RedeemAllowed(element) => element.encode(),
                ComptrollerG2Calls::RedeemVerify(element) => element.encode(),
                ComptrollerG2Calls::RepayBorrowAllowed(element) => element.encode(),
                ComptrollerG2Calls::RepayBorrowVerify(element) => element.encode(),
                ComptrollerG2Calls::SeizeAllowed(element) => element.encode(),
                ComptrollerG2Calls::SeizeGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::SeizeVerify(element) => element.encode(),
                ComptrollerG2Calls::TransferAllowed(element) => element.encode(),
                ComptrollerG2Calls::TransferGuardianPaused(element) => element.encode(),
                ComptrollerG2Calls::TransferVerify(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerG2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerG2Calls::Become(element) => element.fmt(f),
                ComptrollerG2Calls::_BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::_MintGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::SetBorrowPaused(element) => element.fmt(f),
                ComptrollerG2Calls::SetCloseFactor(element) => element.fmt(f),
                ComptrollerG2Calls::SetCollateralFactor(element) => element.fmt(f),
                ComptrollerG2Calls::SetLiquidationIncentive(element) => element.fmt(f),
                ComptrollerG2Calls::SetMaxAssets(element) => element.fmt(f),
                ComptrollerG2Calls::SetMintPaused(element) => element.fmt(f),
                ComptrollerG2Calls::SetPauseGuardian(element) => element.fmt(f),
                ComptrollerG2Calls::SetPriceOracle(element) => element.fmt(f),
                ComptrollerG2Calls::SetSeizePaused(element) => element.fmt(f),
                ComptrollerG2Calls::SetTransferPaused(element) => element.fmt(f),
                ComptrollerG2Calls::SupportMarket(element) => element.fmt(f),
                ComptrollerG2Calls::AccountAssets(element) => element.fmt(f),
                ComptrollerG2Calls::Admin(element) => element.fmt(f),
                ComptrollerG2Calls::BorrowAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::BorrowVerify(element) => element.fmt(f),
                ComptrollerG2Calls::CheckMembership(element) => element.fmt(f),
                ComptrollerG2Calls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerG2Calls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerG2Calls::EnterMarkets(element) => element.fmt(f),
                ComptrollerG2Calls::ExitMarket(element) => element.fmt(f),
                ComptrollerG2Calls::GetAccountLiquidity(element) => element.fmt(f),
                ComptrollerG2Calls::GetAssetsIn(element) => element.fmt(f),
                ComptrollerG2Calls::GetHypotheticalAccountLiquidity(element) => element.fmt(f),
                ComptrollerG2Calls::IsComptroller(element) => element.fmt(f),
                ComptrollerG2Calls::LiquidateBorrowAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::LiquidateBorrowVerify(element) => element.fmt(f),
                ComptrollerG2Calls::LiquidateCalculateSeizeTokens(element) => element.fmt(f),
                ComptrollerG2Calls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerG2Calls::Markets(element) => element.fmt(f),
                ComptrollerG2Calls::MaxAssets(element) => element.fmt(f),
                ComptrollerG2Calls::MintAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::MintGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::MintVerify(element) => element.fmt(f),
                ComptrollerG2Calls::Oracle(element) => element.fmt(f),
                ComptrollerG2Calls::PauseGuardian(element) => element.fmt(f),
                ComptrollerG2Calls::PendingAdmin(element) => element.fmt(f),
                ComptrollerG2Calls::PendingComptrollerImplementation(element) => element.fmt(f),
                ComptrollerG2Calls::RedeemAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::RedeemVerify(element) => element.fmt(f),
                ComptrollerG2Calls::RepayBorrowAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::RepayBorrowVerify(element) => element.fmt(f),
                ComptrollerG2Calls::SeizeAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::SeizeGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::SeizeVerify(element) => element.fmt(f),
                ComptrollerG2Calls::TransferAllowed(element) => element.fmt(f),
                ComptrollerG2Calls::TransferGuardianPaused(element) => element.fmt(f),
                ComptrollerG2Calls::TransferVerify(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BecomeCall> for ComptrollerG2Calls {
        fn from(var: BecomeCall) -> Self {
            ComptrollerG2Calls::Become(var)
        }
    }
    impl ::std::convert::From<_BorrowGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: _BorrowGuardianPausedCall) -> Self {
            ComptrollerG2Calls::_BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<_MintGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: _MintGuardianPausedCall) -> Self {
            ComptrollerG2Calls::_MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SetBorrowPausedCall> for ComptrollerG2Calls {
        fn from(var: SetBorrowPausedCall) -> Self {
            ComptrollerG2Calls::SetBorrowPaused(var)
        }
    }
    impl ::std::convert::From<SetCloseFactorCall> for ComptrollerG2Calls {
        fn from(var: SetCloseFactorCall) -> Self {
            ComptrollerG2Calls::SetCloseFactor(var)
        }
    }
    impl ::std::convert::From<SetCollateralFactorCall> for ComptrollerG2Calls {
        fn from(var: SetCollateralFactorCall) -> Self {
            ComptrollerG2Calls::SetCollateralFactor(var)
        }
    }
    impl ::std::convert::From<SetLiquidationIncentiveCall> for ComptrollerG2Calls {
        fn from(var: SetLiquidationIncentiveCall) -> Self {
            ComptrollerG2Calls::SetLiquidationIncentive(var)
        }
    }
    impl ::std::convert::From<SetMaxAssetsCall> for ComptrollerG2Calls {
        fn from(var: SetMaxAssetsCall) -> Self {
            ComptrollerG2Calls::SetMaxAssets(var)
        }
    }
    impl ::std::convert::From<SetMintPausedCall> for ComptrollerG2Calls {
        fn from(var: SetMintPausedCall) -> Self {
            ComptrollerG2Calls::SetMintPaused(var)
        }
    }
    impl ::std::convert::From<SetPauseGuardianCall> for ComptrollerG2Calls {
        fn from(var: SetPauseGuardianCall) -> Self {
            ComptrollerG2Calls::SetPauseGuardian(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for ComptrollerG2Calls {
        fn from(var: SetPriceOracleCall) -> Self {
            ComptrollerG2Calls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetSeizePausedCall> for ComptrollerG2Calls {
        fn from(var: SetSeizePausedCall) -> Self {
            ComptrollerG2Calls::SetSeizePaused(var)
        }
    }
    impl ::std::convert::From<SetTransferPausedCall> for ComptrollerG2Calls {
        fn from(var: SetTransferPausedCall) -> Self {
            ComptrollerG2Calls::SetTransferPaused(var)
        }
    }
    impl ::std::convert::From<SupportMarketCall> for ComptrollerG2Calls {
        fn from(var: SupportMarketCall) -> Self {
            ComptrollerG2Calls::SupportMarket(var)
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerG2Calls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerG2Calls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerG2Calls {
        fn from(var: AdminCall) -> Self {
            ComptrollerG2Calls::Admin(var)
        }
    }
    impl ::std::convert::From<BorrowAllowedCall> for ComptrollerG2Calls {
        fn from(var: BorrowAllowedCall) -> Self {
            ComptrollerG2Calls::BorrowAllowed(var)
        }
    }
    impl ::std::convert::From<BorrowGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: BorrowGuardianPausedCall) -> Self {
            ComptrollerG2Calls::BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<BorrowVerifyCall> for ComptrollerG2Calls {
        fn from(var: BorrowVerifyCall) -> Self {
            ComptrollerG2Calls::BorrowVerify(var)
        }
    }
    impl ::std::convert::From<CheckMembershipCall> for ComptrollerG2Calls {
        fn from(var: CheckMembershipCall) -> Self {
            ComptrollerG2Calls::CheckMembership(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerG2Calls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerG2Calls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerG2Calls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerG2Calls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<EnterMarketsCall> for ComptrollerG2Calls {
        fn from(var: EnterMarketsCall) -> Self {
            ComptrollerG2Calls::EnterMarkets(var)
        }
    }
    impl ::std::convert::From<ExitMarketCall> for ComptrollerG2Calls {
        fn from(var: ExitMarketCall) -> Self {
            ComptrollerG2Calls::ExitMarket(var)
        }
    }
    impl ::std::convert::From<GetAccountLiquidityCall> for ComptrollerG2Calls {
        fn from(var: GetAccountLiquidityCall) -> Self {
            ComptrollerG2Calls::GetAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<GetAssetsInCall> for ComptrollerG2Calls {
        fn from(var: GetAssetsInCall) -> Self {
            ComptrollerG2Calls::GetAssetsIn(var)
        }
    }
    impl ::std::convert::From<GetHypotheticalAccountLiquidityCall> for ComptrollerG2Calls {
        fn from(var: GetHypotheticalAccountLiquidityCall) -> Self {
            ComptrollerG2Calls::GetHypotheticalAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<IsComptrollerCall> for ComptrollerG2Calls {
        fn from(var: IsComptrollerCall) -> Self {
            ComptrollerG2Calls::IsComptroller(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowAllowedCall> for ComptrollerG2Calls {
        fn from(var: LiquidateBorrowAllowedCall) -> Self {
            ComptrollerG2Calls::LiquidateBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowVerifyCall> for ComptrollerG2Calls {
        fn from(var: LiquidateBorrowVerifyCall) -> Self {
            ComptrollerG2Calls::LiquidateBorrowVerify(var)
        }
    }
    impl ::std::convert::From<LiquidateCalculateSeizeTokensCall> for ComptrollerG2Calls {
        fn from(var: LiquidateCalculateSeizeTokensCall) -> Self {
            ComptrollerG2Calls::LiquidateCalculateSeizeTokens(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerG2Calls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerG2Calls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerG2Calls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerG2Calls::Markets(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerG2Calls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerG2Calls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<MintAllowedCall> for ComptrollerG2Calls {
        fn from(var: MintAllowedCall) -> Self {
            ComptrollerG2Calls::MintAllowed(var)
        }
    }
    impl ::std::convert::From<MintGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: MintGuardianPausedCall) -> Self {
            ComptrollerG2Calls::MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<MintVerifyCall> for ComptrollerG2Calls {
        fn from(var: MintVerifyCall) -> Self {
            ComptrollerG2Calls::MintVerify(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerG2Calls {
        fn from(var: OracleCall) -> Self {
            ComptrollerG2Calls::Oracle(var)
        }
    }
    impl ::std::convert::From<PauseGuardianCall> for ComptrollerG2Calls {
        fn from(var: PauseGuardianCall) -> Self {
            ComptrollerG2Calls::PauseGuardian(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerG2Calls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerG2Calls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerG2Calls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerG2Calls::PendingComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<RedeemAllowedCall> for ComptrollerG2Calls {
        fn from(var: RedeemAllowedCall) -> Self {
            ComptrollerG2Calls::RedeemAllowed(var)
        }
    }
    impl ::std::convert::From<RedeemVerifyCall> for ComptrollerG2Calls {
        fn from(var: RedeemVerifyCall) -> Self {
            ComptrollerG2Calls::RedeemVerify(var)
        }
    }
    impl ::std::convert::From<RepayBorrowAllowedCall> for ComptrollerG2Calls {
        fn from(var: RepayBorrowAllowedCall) -> Self {
            ComptrollerG2Calls::RepayBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<RepayBorrowVerifyCall> for ComptrollerG2Calls {
        fn from(var: RepayBorrowVerifyCall) -> Self {
            ComptrollerG2Calls::RepayBorrowVerify(var)
        }
    }
    impl ::std::convert::From<SeizeAllowedCall> for ComptrollerG2Calls {
        fn from(var: SeizeAllowedCall) -> Self {
            ComptrollerG2Calls::SeizeAllowed(var)
        }
    }
    impl ::std::convert::From<SeizeGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: SeizeGuardianPausedCall) -> Self {
            ComptrollerG2Calls::SeizeGuardianPaused(var)
        }
    }
    impl ::std::convert::From<SeizeVerifyCall> for ComptrollerG2Calls {
        fn from(var: SeizeVerifyCall) -> Self {
            ComptrollerG2Calls::SeizeVerify(var)
        }
    }
    impl ::std::convert::From<TransferAllowedCall> for ComptrollerG2Calls {
        fn from(var: TransferAllowedCall) -> Self {
            ComptrollerG2Calls::TransferAllowed(var)
        }
    }
    impl ::std::convert::From<TransferGuardianPausedCall> for ComptrollerG2Calls {
        fn from(var: TransferGuardianPausedCall) -> Self {
            ComptrollerG2Calls::TransferGuardianPaused(var)
        }
    }
    impl ::std::convert::From<TransferVerifyCall> for ComptrollerG2Calls {
        fn from(var: TransferVerifyCall) -> Self {
            ComptrollerG2Calls::TransferVerify(var)
        }
    }
}
