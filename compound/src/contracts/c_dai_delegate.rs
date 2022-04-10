pub use cdaidelegate_mod::*;
#[allow(clippy::too_many_arguments)]
mod cdaidelegate_mod {
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
    #[doc = "CDaiDelegate was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CDAIDELEGATE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_becomeImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_resignImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"daiJoinAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract CTokenInterface\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"potAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vatAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CDAIDELEGATE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052615aab806100136000396000f3fe608060405234801561001057600080fd5b50600436106103425760003560e01c8063852a12e3116101b8578063b71d1a0c11610104578063f2b3abbd116100a2578063f851a4401161007c578063f851a44014610b9b578063f8f9da2814610ba3578063fca7820b14610bab578063fe9c44ae14610bc857610342565b8063f2b3abbd14610b37578063f3fdb15a14610b5d578063f5e3c46214610b6557610342565b8063c5ebeaec116100de578063c5ebeaec14610ac7578063db006a7514610ae4578063dd62ed3e14610b01578063e9c714f214610b2f57610342565b8063b71d1a0c14610a4d578063bd6d894d14610a73578063c37f68e214610a7b57610342565b806399d8c1b411610171578063a9059cbb1161014b578063a9059cbb146109db578063aa5af0fd14610a07578063ae9d70b014610a0f578063b2a02ff114610a1757610342565b806399d8c1b414610868578063a0712d68146109b6578063a6afed95146109d357610342565b8063852a12e3146108055780638a8df2e6146108225780638d925ccd1461082a5780638f840ddd1461083257806395d89b411461083a57806395dd91931461084257610342565b80633af9e669116102925780635fe3b567116102305780636c540baf1161020a5780636c540baf146107c75780636f307dc3146107cf57806370a08231146107d757806373acee98146107fd57610342565b80635fe3b5671461079a578063601a0bf1146107a257806366ced602146107bf57610342565b80634576b5db1161026c5780634576b5db146106c057806347bd3718146106e657806356e67728146106ee5780635c60da1b1461079257610342565b80633af9e669146106755780633b1d21a21461069b5780633e941010146106a357610342565b806318160ddd116102ff57806323b872dd116102d957806323b872dd146105d15780632608f818146106075780632678224714610633578063313ce5671461065757610342565b806318160ddd1461046b578063182df0f5146104735780631a31d4651461047b57610342565b806306fdde0314610347578063095ea7b3146103c45780630e75270214610404578063153ab50514610433578063173b99041461043d57806317bfdfbc14610445575b600080fd5b61034f610bd0565b6040805160208082528351818301528351919283929083019185019080838360005b83811015610389578181015183820152602001610371565b50505050905090810190601f1680156103b65780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6103f0600480360360408110156103da57600080fd5b506001600160a01b038135169060200135610c5d565b604080519115158252519081900360200190f35b6104216004803603602081101561041a57600080fd5b5035610cca565b60408051918252519081900360200190f35b61043b610ce0565b005b610421610f7e565b6104216004803603602081101561045b57600080fd5b50356001600160a01b0316610f84565b610421611044565b61042161104a565b61043b600480360360e081101561049157600080fd5b6001600160a01b03823581169260208101358216926040820135909216916060820135919081019060a081016080820135600160201b8111156104d357600080fd5b8201836020820111156104e557600080fd5b803590602001918460018302840111600160201b8311171561050657600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561055857600080fd5b82018360208201111561056a57600080fd5b803590602001918460018302840111600160201b8311171561058b57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff1691506110ad9050565b6103f0600480360360608110156105e757600080fd5b506001600160a01b03813581169160208101359091169060400135611141565b6104216004803603604081101561061d57600080fd5b506001600160a01b0381351690602001356111b3565b61063b6111c9565b604080516001600160a01b039092168252519081900360200190f35b61065f6111d8565b6040805160ff9092168252519081900360200190f35b6104216004803603602081101561068b57600080fd5b50356001600160a01b03166111e1565b610421611297565b610421600480360360208110156106b957600080fd5b50356112a6565b610421600480360360208110156106d657600080fd5b50356001600160a01b03166112b1565b610421611406565b61043b6004803603602081101561070457600080fd5b810190602081018135600160201b81111561071e57600080fd5b82018360208201111561073057600080fd5b803590602001918460018302840111600160201b8311171561075157600080fd5b91908080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525092955061140c945050505050565b61063b61148f565b61063b61149e565b610421600480360360208110156107b857600080fd5b50356114ad565b61063b611548565b610421611557565b61063b61155d565b610421600480360360208110156107ed57600080fd5b50356001600160a01b031661156c565b610421611587565b6104216004803603602081101561081b57600080fd5b503561163d565b61063b611648565b61063b611657565b610421611666565b61034f61166c565b6104216004803603602081101561085857600080fd5b50356001600160a01b03166116c4565b61043b600480360360c081101561087e57600080fd5b6001600160a01b03823581169260208101359091169160408201359190810190608081016060820135600160201b8111156108b857600080fd5b8201836020820111156108ca57600080fd5b803590602001918460018302840111600160201b831117156108eb57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561093d57600080fd5b82018360208201111561094f57600080fd5b803590602001918460018302840111600160201b8311171561097057600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff1691506117219050565b610421600480360360208110156109cc57600080fd5b5035611908565b610421611914565b6103f0600480360360408110156109f157600080fd5b506001600160a01b03813516906020013561198f565b610421611a00565b610421611a06565b61042160048036036060811015610a2d57600080fd5b506001600160a01b03813581169160208101359091169060400135611aa5565b61042160048036036020811015610a6357600080fd5b50356001600160a01b0316611b16565b610421611ba2565b610aa160048036036020811015610a9157600080fd5b50356001600160a01b0316611c5e565b604080519485526020850193909352838301919091526060830152519081900360800190f35b61042160048036036020811015610add57600080fd5b5035611cf3565b61042160048036036020811015610afa57600080fd5b5035611cfe565b61042160048036036040811015610b1757600080fd5b506001600160a01b0381358116916020013516611d09565b610421611d34565b61042160048036036020811015610b4d57600080fd5b50356001600160a01b0316611e37565b61063b611e71565b61042160048036036060811015610b7b57600080fd5b506001600160a01b03813581169160208101359160409091013516611e80565b61063b611e98565b610421611eac565b61042160048036036020811015610bc157600080fd5b5035611f10565b6103f0611f8e565b60018054604080516020600284861615610100026000190190941693909304601f81018490048402820184019092528181529291830182828015610c555780601f10610c2a57610100808354040283529160200191610c55565b820191906000526020600020905b815481529060010190602001808311610c3857829003601f168201915b505050505081565b336000818152600f602090815260408083206001600160a01b03871680855290835281842086905581518681529151939493909284927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929081900390910190a360019150505b92915050565b600080610cd683611f93565b509150505b919050565b60035461010090046001600160a01b03163314610d2e5760405162461bcd60e51b815260040180806020018281038252602d81526020018061582f602d913960400191505060405180910390fd5b60135460145460155460408051634fb3c66560e11b815290516001600160a01b039485169493841693909216918391639f678cca9160048083019260209291908290030181600087803b158015610d8457600080fd5b505af1158015610d98573d6000803e3d6000fd5b505050506040513d6020811015610dae57600080fd5b5050604080516305f5d64360e11b815230600482015290516000916001600160a01b03851691630bebac8691602480820192602092909190829003018186803b158015610dfa57600080fd5b505afa158015610e0e573d6000803e3d6000fd5b505050506040513d6020811015610e2457600080fd5b505160408051637f8661a160e01b81526004810183905290519192506001600160a01b03851691637f8661a19160248082019260009290919082900301818387803b158015610e7257600080fd5b505af1158015610e86573d6000803e3d6000fd5b505060408051633612d9a360e11b81523060048201529051600093506001600160a01b0386169250636c25b34691602480820192602092909190829003018186803b158015610ed457600080fd5b505afa158015610ee8573d6000803e3d6000fd5b505050506040513d6020811015610efe57600080fd5b50516040805163ef693bed60e01b81523060048201526b033b2e3c9fd0803ce80000008304602482015290519192506001600160a01b0387169163ef693bed9160448082019260009290919082900301818387803b158015610f5f57600080fd5b505af1158015610f73573d6000803e3d6000fd5b505050505050505050565b60085481565b6000805460ff16610fc9576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610fdb611914565b14611026576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b61102f826116c4565b90505b6000805460ff19166001179055919050565b600d5481565b600080600061105761203c565b9092509050600082600381111561106a57fe5b146110a65760405162461bcd60e51b81526004018080602001828103825260358152602001806159c26035913960400191505060405180910390fd5b9150505b90565b6110bb868686868686611721565b601180546001600160a01b0319166001600160a01b038981169190911791829055604080516318160ddd60e01b8152905192909116916318160ddd91600480820192602092909190829003018186803b15801561111757600080fd5b505afa15801561112b573d6000803e3d6000fd5b505050506040513d6020811015610f7357600080fd5b6000805460ff16611186576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916815561119c338686866120eb565b1490506000805460ff191660011790559392505050565b6000806111c084846123f9565b50949350505050565b6004546001600160a01b031681565b60035460ff1681565b60006111eb61560d565b60405180602001604052806111fe611ba2565b90526001600160a01b0384166000908152600e602052604081205491925090819061122a9084906124a4565b9092509050600082600381111561123d57fe5b1461128f576040805162461bcd60e51b815260206004820152601f60248201527f62616c616e636520636f756c64206e6f742062652063616c63756c6174656400604482015290519081900360640190fd5b949350505050565b60006112a16124f8565b905090565b6000610cc48261260a565b60035460009061010090046001600160a01b031633146112de576112d76001603f61269e565b9050610cdb565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b15801561132357600080fd5b505afa158015611337573d6000803e3d6000fd5b505050506040513d602081101561134d57600080fd5b50516113a0576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9392505050565b600b5481565b60035461010090046001600160a01b0316331461145a5760405162461bcd60e51b81526004018080602001828103825260308152602001806157ad6030913960400191505060405180910390fd5b60008082806020019051604081101561147257600080fd5b508051602090910151909250905061148a8282612704565b505050565b6012546001600160a01b031681565b6005546001600160a01b031681565b6000805460ff166114f2576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611504611914565b9050801561152a5761152281601081111561151b57fe5b603061269e565b915050611032565b61153383612a1f565b9150506000805460ff19166001179055919050565b6014546001600160a01b031681565b60095481565b6011546001600160a01b031681565b6001600160a01b03166000908152600e602052604090205490565b6000805460ff166115cc576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556115de611914565b14611629576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b50600b546000805460ff1916600117905590565b6000610cc482612b52565b6013546001600160a01b031681565b6015546001600160a01b031681565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f81018490048402820184019092528181529291830182828015610c555780601f10610c2a57610100808354040283529160200191610c55565b60008060006116d284612bd3565b909250905060008260038111156116e557fe5b146113ff5760405162461bcd60e51b81526004018080602001828103825260378152602001806158876037913960400191505060405180910390fd5b60035461010090046001600160a01b0316331461176f5760405162461bcd60e51b81526004018080602001828103825260248152602001806157666024913960400191505060405180910390fd5b60095415801561177f5750600a54155b6117ba5760405162461bcd60e51b815260040180806020018281038252602381526020018061578a6023913960400191505060405180910390fd5b6007849055836117fb5760405162461bcd60e51b81526004018080602001828103825260308152602001806157dd6030913960400191505060405180910390fd5b6000611806876112b1565b9050801561185b576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b611863612c87565b600955670de0b6b3a7640000600a5561187b86612c8b565b905080156118ba5760405162461bcd60e51b815260040180806020018281038252602281526020018061580d6022913960400191505060405180910390fd5b83516118cd906001906020870190615620565b5082516118e1906002906020860190615620565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b600080610cd683612e00565b60145460408051634fb3c66560e11b815290516000926001600160a01b031691639f678cca91600480830192602092919082900301818787803b15801561195a57600080fd5b505af115801561196e573d6000803e3d6000fd5b505050506040513d602081101561198457600080fd5b506112a19050612e81565b6000805460ff166119d4576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556119ea333386866120eb565b1490506000805460ff1916600117905592915050565b600a5481565b6006546000906001600160a01b031663b8168816611a226124f8565b600b54600c546008546040518563ffffffff1660e01b81526004018085815260200184815260200183815260200182815260200194505050505060206040518083038186803b158015611a7457600080fd5b505afa158015611a88573d6000803e3d6000fd5b505050506040513d6020811015611a9e57600080fd5b5051905090565b6000805460ff16611aea576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19169055611b00338585856131d9565b90506000805460ff191660011790559392505050565b60035460009061010090046001600160a01b03163314611b3c576112d76001604561269e565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a9929181900390910190a160006113ff565b6000805460ff16611be7576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611bf9611914565b14611c44576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b611c4c61104a565b90506000805460ff1916600117905590565b6001600160a01b0381166000908152600e6020526040812054819081908190818080611c8989612bd3565b935090506000816003811115611c9b57fe5b14611cb95760095b975060009650869550859450611cec9350505050565b611cc161203c565b925090506000816003811115611cd357fe5b14611cdf576009611ca3565b5060009650919450925090505b9193509193565b6000610cc48261343f565b6000610cc4826134be565b6001600160a01b039182166000908152600f6020908152604080832093909416825291909152205490565b6004546000906001600160a01b031633141580611d4f575033155b15611d6757611d606001600061269e565b90506110aa565b60038054600480546001600160a01b03818116610100818102610100600160a81b0319871617968790556001600160a01b031990931690935560408051948390048216808652929095041660208401528351909391927ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc92908290030190a1600454604080516001600160a01b038085168252909216602083015280517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a99281900390910190a160009250505090565b600080611e42611914565b90508015611e6857611e60816010811115611e5957fe5b604061269e565b915050610cdb565b6113ff83612c8b565b6006546001600160a01b031681565b600080611e8e858585613538565b5095945050505050565b60035461010090046001600160a01b031681565b6006546000906001600160a01b03166315f24053611ec86124f8565b600b54600c546040518463ffffffff1660e01b815260040180848152602001838152602001828152602001935050505060206040518083038186803b158015611a7457600080fd5b6000805460ff16611f55576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611f67611914565b90508015611f8557611522816010811115611f7e57fe5b604661269e565b6115338361366a565b600181565b60008054819060ff16611fda576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611fec611914565b905080156120175761200a81601081111561200357fe5b603661269e565b9250600091506120289050565b612022333386613712565b92509250505b6000805460ff191660011790559092909150565b600d54600090819080612057575050600754600091506120e7565b60006120616124f8565b9050600061206d61560d565b600061207e84600b54600c54613af8565b93509050600081600381111561209057fe5b146120a5579550600094506120e79350505050565b6120af8386613b44565b9250905060008160038111156120c157fe5b146120d6579550600094506120e79350505050565b50516000955093506120e792505050565b9091565b600554604080516317b9b84b60e31b81523060048201526001600160a01b03868116602483015285811660448301526064820185905291516000938493169163bdcdc25891608480830192602092919082900301818787803b15801561215057600080fd5b505af1158015612164573d6000803e3d6000fd5b505050506040513d602081101561217a57600080fd5b505190508015612199576121916003604a83613bf4565b91505061128f565b836001600160a01b0316856001600160a01b031614156121bf576121916002604b61269e565b60006001600160a01b0387811690871614156121de5750600019612206565b506001600160a01b038086166000908152600f60209081526040808320938a16835292905220545b6000806000806122168589613c5a565b9094509250600084600381111561222957fe5b146122475761223a6009604b61269e565b965050505050505061128f565b6001600160a01b038a166000908152600e602052604090205461226a9089613c5a565b9094509150600084600381111561227d57fe5b1461228e5761223a6009604c61269e565b6001600160a01b0389166000908152600e60205260409020546122b19089613c7d565b909450905060008460038111156122c457fe5b146122d55761223a6009604d61269e565b6001600160a01b03808b166000908152600e6020526040808220859055918b16815220819055600019851461232d576001600160a01b03808b166000908152600f60209081526040808320938f168352929052208390555b886001600160a01b03168a6001600160a01b031660008051602061591a8339815191528a6040518082815260200191505060405180910390a36005546040805163352b4a3f60e11b81523060048201526001600160a01b038d811660248301528c81166044830152606482018c905291519190921691636a56947e91608480830192600092919082900301818387803b1580156123c957600080fd5b505af11580156123dd573d6000803e3d6000fd5b50600092506123ea915050565b9b9a5050505050505050505050565b60008054819060ff16612440576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612452611914565b9050801561247d5761247081601081111561246957fe5b603561269e565b92506000915061248e9050565b612488338686613712565b92509250505b6000805460ff1916600117905590939092509050565b60008060006124b161560d565b6124bb8686613ca3565b909250905060008260038111156124ce57fe5b146124df57509150600090506124f1565b60006124ea82613d0b565b9350935050505b9250929050565b601454604080516305f5d64360e11b815230600482015290516000926001600160a01b03169183918391630bebac86916024808301926020929190829003018186803b15801561254757600080fd5b505afa15801561255b573d6000803e3d6000fd5b505050506040513d602081101561257157600080fd5b50516040805163324abb3160e21b815290519192506b033b2e3c9fd0803ce8000000916125fb916001600160a01b0386169163c92aecc491600480820192602092909190829003018186803b1580156125c957600080fd5b505afa1580156125dd573d6000803e3d6000fd5b505050506040513d60208110156125f357600080fd5b505183613d1a565b8161260257fe5b049250505090565b6000805460ff1661264f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612661611914565b9050801561267f5761152281601081111561267857fe5b604e61269e565b61268883613d75565b509150506000805460ff19166001179055919050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa08360108111156126cd57fe5b8360508111156126d957fe5b604080519283526020830191909152600082820152519081900360600190a18260108111156113ff57fe5b600082905060008290506000826001600160a01b031663f4b9fa756040518163ffffffff1660e01b8152600401602060405180830381600087803b15801561274b57600080fd5b505af115801561275f573d6000803e3d6000fd5b505050506040513d602081101561277557600080fd5b5051604080516336569e7760e01b815290519192506000916001600160a01b038616916336569e7791600480830192602092919082900301818787803b1580156127be57600080fd5b505af11580156127d2573d6000803e3d6000fd5b505050506040513d60208110156127e857600080fd5b50516011549091506001600160a01b038381169116146128395760405162461bcd60e51b81526004018080602001828103825260228152602001806158be6022913960400191505060405180910390fd5b601380546001600160a01b038089166001600160a01b0319928316179283905560148054898316908416179055601580548583169316929092179091556040805163095ea7b360e01b815292821660048401526000196024840152519084169163095ea7b391604480830192600092919082900301818387803b1580156128bf57600080fd5b505af11580156128d3573d6000803e3d6000fd5b5050601454604080516328ec8bf160e21b81526001600160a01b0392831660048201529051918516935063a3b22fc4925060248082019260009290919082900301818387803b15801561292557600080fd5b505af1158015612939573d6000803e3d6000fd5b5050601354604080516328ec8bf160e21b81526001600160a01b0392831660048201529051918516935063a3b22fc4925060248082019260009290919082900301818387803b15801561298b57600080fd5b505af115801561299f573d6000803e3d6000fd5b50505050826001600160a01b0316639f678cca6040518163ffffffff1660e01b8152600401602060405180830381600087803b1580156129de57600080fd5b505af11580156129f2573d6000803e3d6000fd5b505050506040513d6020811015612a0857600080fd5b50612a169050306000613e5d565b50505050505050565b600354600090819061010090046001600160a01b03163314612a4757611e606001603161269e565b612a4f612c87565b60095414612a6357611e60600a603361269e565b82612a6c6124f8565b1015612a7e57611e60600e603261269e565b600c54831115612a9457611e606002603461269e565b50600c5482810390811115612ada5760405162461bcd60e51b8152600401808060200182810382526024815260200180615a536024913960400191505060405180910390fd5b600c819055600354612afa9061010090046001600160a01b031684614182565b600354604080516101009092046001600160a01b0316825260208201859052818101839052517f3bad0c59cf2f06e7314077049f48a93578cd16f5ef92329f1dab1420a99c177e916060908290030190a160006113ff565b6000805460ff16612b97576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612ba9611914565b90508015612bc757611522816010811115612bc057fe5b602761269e565b611533336000856142e7565b6001600160a01b038116600090815260106020526040812080548291829182918291612c0a575060009450849350612c8292505050565b612c1a8160000154600a546147ae565b90945092506000846003811115612c2d57fe5b14612c42575091935060009250612c82915050565b612c508382600101546147ed565b90945091506000846003811115612c6357fe5b14612c78575091935060009250612c82915050565b5060009450925050505b915091565b4390565b600354600090819061010090046001600160a01b03163314612cb357611e606001604261269e565b612cbb612c87565b60095414612ccf57611e60600a604161269e565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612d2057600080fd5b505afa158015612d34573d6000803e3d6000fd5b505050506040513d6020811015612d4a57600080fd5b5051612d9d576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a160006113ff565b60008054819060ff16612e47576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612e59611914565b90508015612e775761200a816010811115612e7057fe5b601e61269e565b6120223385614818565b600080612e8c612c87565b60095490915080821415612ea5576000925050506110aa565b6000612eaf6124f8565b600b54600c54600a54600654604080516315f2405360e01b815260048101879052602481018690526044810185905290519596509394929391926000926001600160a01b03909216916315f24053916064808301926020929190829003018186803b158015612f1d57600080fd5b505afa158015612f31573d6000803e3d6000fd5b505050506040513d6020811015612f4757600080fd5b5051905065048c27395000811115612fa6576040805162461bcd60e51b815260206004820152601c60248201527f626f72726f772072617465206973206162737572646c79206869676800000000604482015290519081900360640190fd5b600080612fb38989613c5a565b90925090506000826003811115612fc657fe5b14613018576040805162461bcd60e51b815260206004820152601f60248201527f636f756c64206e6f742063616c63756c61746520626c6f636b2064656c746100604482015290519081900360640190fd5b61302061560d565b60008060008061303e60405180602001604052808a81525087613ca3565b9097509450600087600381111561305157fe5b146130835761306e6009600689600381111561306957fe5b613bf4565b9e5050505050505050505050505050506110aa565b61308d858c6124a4565b909750935060008760038111156130a057fe5b146130b85761306e6009600189600381111561306957fe5b6130c2848c613c7d565b909750925060008760038111156130d557fe5b146130ed5761306e6009600489600381111561306957fe5b6131086040518060200160405280600854815250858c614c77565b9097509150600087600381111561311b57fe5b146131335761306e6009600589600381111561306957fe5b61313e858a8b614c77565b9097509050600087600381111561315157fe5b146131695761306e6009600389600381111561306957fe5b60098e9055600a819055600b839055600c829055604080518d8152602081018690528082018390526060810185905290517f4dec04e750ca11537cabcd8a9eab06494de08da3735bc8871cd41250e190bc049181900360800190a160009e50505050505050505050505050505090565b6005546040805163d02f735160e01b81523060048201526001600160a01b038781166024830152868116604483015285811660648301526084820185905291516000938493169163d02f73519160a480830192602092919082900301818787803b15801561324657600080fd5b505af115801561325a573d6000803e3d6000fd5b505050506040513d602081101561327057600080fd5b505190508015613287576121916003601b83613bf4565b846001600160a01b0316846001600160a01b031614156132ad576121916006601c61269e565b6001600160a01b0384166000908152600e6020526040812054819081906132d49087613c5a565b909350915060008360038111156132e757fe5b1461330a576132ff6009601a85600381111561306957fe5b94505050505061128f565b6001600160a01b0388166000908152600e602052604090205461332d9087613c7d565b9093509050600083600381111561334057fe5b14613358576132ff6009601985600381111561306957fe5b6001600160a01b038088166000818152600e60209081526040808320879055938c168083529184902085905583518a81529351919360008051602061591a833981519152929081900390910190a360055460408051636d35bf9160e01b81523060048201526001600160a01b038c811660248301528b811660448301528a81166064830152608482018a905291519190921691636d35bf919160a480830192600092919082900301818387803b15801561341157600080fd5b505af1158015613425573d6000803e3d6000fd5b5060009250613432915050565b9998505050505050505050565b6000805460ff16613484576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155613496611914565b905080156134b4576115228160108111156134ad57fe5b600861269e565b6115333384614cc4565b6000805460ff16613503576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155613515611914565b9050801561352c57611522816010811115612bc057fe5b611533338460006142e7565b60008054819060ff1661357f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155613591611914565b905080156135bc576135af8160108111156135a857fe5b600f61269e565b9250600091506136539050565b836001600160a01b031663a6afed956040518163ffffffff1660e01b8152600401602060405180830381600087803b1580156135f757600080fd5b505af115801561360b573d6000803e3d6000fd5b505050506040513d602081101561362157600080fd5b505190508015613641576135af81601081111561363a57fe5b601061269e565b61364d33878787614fd2565b92509250505b6000805460ff191660011790559094909350915050565b60035460009061010090046001600160a01b03163314613690576112d76001604761269e565b613698612c87565b600954146136ac576112d7600a604861269e565b670de0b6b3a76400008211156136c8576112d76002604961269e565b6008805490839055604080518281526020810185905281517faaa68312e2ea9d50e16af5068410ab56e1a1fd06037b1a35664812c30f821460929181900390910190a160006113ff565b60055460408051631200453160e11b81523060048201526001600160a01b0386811660248301528581166044830152606482018590529151600093849384939116916324008a629160848082019260209290919082900301818787803b15801561377b57600080fd5b505af115801561378f573d6000803e3d6000fd5b505050506040513d60208110156137a557600080fd5b5051905080156137c9576137bc6003603883613bf4565b925060009150613af09050565b6137d1612c87565b600954146137e5576137bc600a603961269e565b6137ed61569e565b6001600160a01b038616600090815260106020526040902060010154606082015261381786612bd3565b608083018190526020830182600381111561382e57fe5b600381111561383957fe5b905250600090508160200151600381111561385057fe5b1461387a5761386c600960378360200151600381111561306957fe5b935060009250613af0915050565b600019851415613893576080810151604082015261389b565b604081018590525b6138a9878260400151613e5d565b60e0820181905260808201516138be91613c5a565b60a08301819052602083018260038111156138d557fe5b60038111156138e057fe5b90525060009050816020015160038111156138f757fe5b146139335760405162461bcd60e51b815260040180806020018281038252603a8152602001806158e0603a913960400191505060405180910390fd5b613943600b548260e00151613c5a565b60c083018190526020830182600381111561395a57fe5b600381111561396557fe5b905250600090508160200151600381111561397c57fe5b146139b85760405162461bcd60e51b815260040180806020018281038252603181526020018061593a6031913960400191505060405180910390fd5b60a080820180516001600160a01b03808a16600081815260106020908152604091829020948555600a5460019095019490945560c0870151600b81905560e088015195518251948f16855294840192909252828101949094526060820192909252608081019190915290517f1a2a22cb034d26d1854bdc6666a5b91fe25efbbb5dcad3b0355478d6f5c362a1929181900390910190a160055460e0820151606083015160408051631ededc9160e01b81523060048201526001600160a01b038c811660248301528b8116604483015260648201949094526084810192909252519190921691631ededc919160a480830192600092919082900301818387803b158015613ac357600080fd5b505af1158015613ad7573d6000803e3d6000fd5b5060009250613ae4915050565b8160e001519350935050505b935093915050565b600080600080613b088787613c7d565b90925090506000826003811115613b1b57fe5b14613b2c5750915060009050613af0565b613b368186613c5a565b935093505050935093915050565b6000613b4e61560d565b600080613b6386670de0b6b3a76400006147ae565b90925090506000826003811115613b7657fe5b14613b95575060408051602081019091526000815290925090506124f1565b600080613ba283886147ed565b90925090506000826003811115613bb557fe5b14613bd7575060408051602081019091526000815290945092506124f1915050565b604080516020810190915290815260009890975095505050505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0846010811115613c2357fe5b846050811115613c2f57fe5b604080519283526020830191909152818101859052519081900360600190a183601081111561128f57fe5b600080838311613c715750600090508183036124f1565b506003905060006124f1565b600080838301848110613c95576000925090506124f1565b5060029150600090506124f1565b6000613cad61560d565b600080613cbe8660000151866147ae565b90925090506000826003811115613cd157fe5b14613cf0575060408051602081019091526000815290925090506124f1565b60408051602081019091529081526000969095509350505050565b51670de0b6b3a7640000900490565b6000811580613d3557505080820282828281613d3257fe5b04145b610cc4576040805162461bcd60e51b815260206004820152600c60248201526b6d756c2d6f766572666c6f7760a01b604482015290519081900360640190fd5b600080600080613d83612c87565b60095414613da257613d97600a604f61269e565b93509150612c829050565b613dac3386613e5d565b905080600c54019150600c54821015613e0c576040805162461bcd60e51b815260206004820181905260248201527f61646420726573657276657320756e6578706563746564206f766572666c6f77604482015290519081900360640190fd5b600c829055604080513381526020810183905280820184905290517fa91e67c5ea634cd43a12c5a482724b03de01e85ca68702a53d0c2f45cb7c1dc59181900360600190a160009350915050915091565b601154604080516323b872dd60e01b81526001600160a01b038581166004830152306024830152604482018590529151600093929092169182916323b872dd91606480830192602092919082900301818887803b158015613ebd57600080fd5b505af1158015613ed1573d6000803e3d6000fd5b505050506040513d6020811015613ee757600080fd5b5051613f245760405162461bcd60e51b815260040180806020018281038252602481526020018061599e6024913960400191505060405180910390fd5b601354601154601454601554604080516370a0823160e01b8152306004820181905291516001600160a01b03968716969586169594851694909316928692633b4da69f92909187916370a08231916024808301926020929190829003018186803b158015613f9157600080fd5b505afa158015613fa5573d6000803e3d6000fd5b505050506040513d6020811015613fbb57600080fd5b5051604080516001600160e01b031960e086901b1681526001600160a01b039093166004840152602483019190915251604480830192600092919082900301818387803b15801561400b57600080fd5b505af115801561401f573d6000803e3d6000fd5b505060408051633612d9a360e11b81523060048201529051600093506001600160a01b0385169250636c25b34691602480820192602092909190829003018186803b15801561406d57600080fd5b505afa158015614081573d6000803e3d6000fd5b505050506040513d602081101561409757600080fd5b50516040805163324abb3160e21b815290519192506000916001600160a01b0386169163c92aecc4916004808301926020929190829003018186803b1580156140df57600080fd5b505afa1580156140f3573d6000803e3d6000fd5b505050506040513d602081101561410957600080fd5b5051828161411357fe5b049050836001600160a01b031663049878f3826040518263ffffffff1660e01b815260040180828152602001915050600060405180830381600087803b15801561415c57600080fd5b505af1158015614170573d6000803e3d6000fd5b509a9c9b505050505050505050505050565b6013546014546040805163324abb3160e21b815290516001600160a01b03938416939092169160009161422791849163c92aecc4916004808301926020929190829003018186803b1580156141d657600080fd5b505afa1580156141ea573d6000803e3d6000fd5b505050506040513d602081101561420057600080fd5b5051614218866b033b2e3c9fd0803ce8000000613d1a565b8161421f57fe5b046001615555565b9050816001600160a01b0316637f8661a1826040518263ffffffff1660e01b815260040180828152602001915050600060405180830381600087803b15801561426f57600080fd5b505af1158015614283573d6000803e3d6000fd5b50505050826001600160a01b031663ef693bed86866040518363ffffffff1660e01b815260040180836001600160a01b03166001600160a01b0316815260200182815260200192505050600060405180830381600087803b158015610f5f57600080fd5b60008215806142f4575081155b61432f5760405162461bcd60e51b8152600401808060200182810382526034815260200180615a1f6034913960400191505060405180910390fd5b6143376156e4565b61433f61203c565b604083018190526020830182600381111561435657fe5b600381111561436157fe5b905250600090508160200151600381111561437857fe5b1461439c576143946009602b8360200151600381111561306957fe5b9150506113ff565b831561441d5760608101849052604080516020810182529082015181526143c390856124a4565b60808301819052602083018260038111156143da57fe5b60038111156143e557fe5b90525060009050816020015160038111156143fc57fe5b1461441857614394600960298360200151600381111561306957fe5b614496565b614439836040518060200160405280846040015181525061559c565b606083018190526020830182600381111561445057fe5b600381111561445b57fe5b905250600090508160200151600381111561447257fe5b1461448e576143946009602a8360200151600381111561306957fe5b608081018390525b60055460608201516040805163eabe7d9160e01b81523060048201526001600160a01b03898116602483015260448201939093529051600093929092169163eabe7d919160648082019260209290919082900301818787803b1580156144fb57600080fd5b505af115801561450f573d6000803e3d6000fd5b505050506040513d602081101561452557600080fd5b5051905080156145455761453c6003602883613bf4565b925050506113ff565b61454d612c87565b600954146145615761453c600a602c61269e565b614571600d548360600151613c5a565b60a084018190526020840182600381111561458857fe5b600381111561459357fe5b90525060009050826020015160038111156145aa57fe5b146145c65761453c6009602e8460200151600381111561306957fe5b6001600160a01b0386166000908152600e602052604090205460608301516145ee9190613c5a565b60c084018190526020840182600381111561460557fe5b600381111561461057fe5b905250600090508260200151600381111561462757fe5b146146435761453c6009602d8460200151600381111561306957fe5b81608001516146506124f8565b10156146625761453c600e602f61269e565b614670868360800151614182565b60a0820151600d5560c08201516001600160a01b0387166000818152600e602090815260409182902093909355606085015181519081529051309360008051602061591a833981519152928290030190a36080820151606080840151604080516001600160a01b038b168152602081019490945283810191909152517fe5b754fb1abb7f01b499791d0b820ae3b6af3424ac1c59768edb53f4ec31a9299281900390910190a160055460808301516060840151604080516351dff98960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916351dff98991608480830192600092919082900301818387803b15801561478357600080fd5b505af1158015614797573d6000803e3d6000fd5b50600092506147a4915050565b9695505050505050565b600080836147c1575060009050806124f1565b838302838582816147ce57fe5b04146147e2575060029150600090506124f1565b6000925090506124f1565b6000808261480157506001905060006124f1565b600083858161480c57fe5b04915091509250929050565b60055460408051634ef4c3e160e01b81523060048201526001600160a01b03858116602483015260448201859052915160009384938493911691634ef4c3e19160648082019260209290919082900301818787803b15801561487957600080fd5b505af115801561488d573d6000803e3d6000fd5b505050506040513d60208110156148a357600080fd5b5051905080156148c7576148ba6003601f83613bf4565b9250600091506124f19050565b6148cf612c87565b600954146148e3576148ba600a602261269e565b6148eb6156e4565b6148f361203c565b604083018190526020830182600381111561490a57fe5b600381111561491557fe5b905250600090508160200151600381111561492c57fe5b1461495657614948600960218360200151600381111561306957fe5b9350600092506124f1915050565b6149608686613e5d565b60c0820181905260408051602081018252908301518152614981919061559c565b606083018190526020830182600381111561499857fe5b60038111156149a357fe5b90525060009050816020015160038111156149ba57fe5b14614a0c576040805162461bcd60e51b815260206004820181905260248201527f4d494e545f45584348414e47455f43414c43554c4154494f4e5f4641494c4544604482015290519081900360640190fd5b614a1c600d548260600151613c7d565b6080830181905260208301826003811115614a3357fe5b6003811115614a3e57fe5b9052506000905081602001516003811115614a5557fe5b14614a915760405162461bcd60e51b81526004018080602001828103825260288152602001806159f76028913960400191505060405180910390fd5b6001600160a01b0386166000908152600e60205260409020546060820151614ab99190613c7d565b60a0830181905260208301826003811115614ad057fe5b6003811115614adb57fe5b9052506000905081602001516003811115614af257fe5b14614b2e5760405162461bcd60e51b815260040180806020018281038252602b81526020018061585c602b913960400191505060405180910390fd5b6080810151600d5560a08101516001600160a01b0387166000818152600e60209081526040918290209390935560c084015160608086015183519485529484019190915282820193909352517f4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f929181900390910190a1606081015160408051918252516001600160a01b03881691309160008051602061591a8339815191529181900360200190a360055460c08201516060830151604080516341c728b960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916341c728b991608480830192600092919082900301818387803b158015614c4457600080fd5b505af1158015614c58573d6000803e3d6000fd5b5060009250614c65915050565b8160c001519350935050509250929050565b6000806000614c8461560d565b614c8e8787613ca3565b90925090506000826003811115614ca157fe5b14614cb25750915060009050613af0565b613b36614cbe82613d0b565b86613c7d565b6005546040805163368f515360e21b81523060048201526001600160a01b0385811660248301526044820185905291516000938493169163da3d454c91606480830192602092919082900301818787803b158015614d2157600080fd5b505af1158015614d35573d6000803e3d6000fd5b505050506040513d6020811015614d4b57600080fd5b505190508015614d6a57614d626003600e83613bf4565b915050610cc4565b614d72612c87565b60095414614d8557614d62600a8061269e565b82614d8e6124f8565b1015614da057614d62600e600961269e565b614da8615722565b614db185612bd3565b6020830181905282826003811115614dc557fe5b6003811115614dd057fe5b9052506000905081516003811115614de457fe5b14614e0957614e00600960078360000151600381111561306957fe5b92505050610cc4565b614e17816020015185613c7d565b6040830181905282826003811115614e2b57fe5b6003811115614e3657fe5b9052506000905081516003811115614e4a57fe5b14614e6657614e006009600c8360000151600381111561306957fe5b614e72600b5485613c7d565b6060830181905282826003811115614e8657fe5b6003811115614e9157fe5b9052506000905081516003811115614ea557fe5b14614ec157614e006009600b8360000151600381111561306957fe5b614ecb8585614182565b604080820180516001600160a01b03881660008181526010602090815290859020928355600a54600190930192909255606080860151600b81905593518551928352928201899052818501929092529081019190915290517f13ed6866d4e1ee6da46f845c46d7e54120883d75c5ea9a2dacc1c4ca8984ab809181900360800190a160055460408051635c77860560e01b81523060048201526001600160a01b0388811660248301526044820188905291519190921691635c77860591606480830192600092919082900301818387803b158015614fa857600080fd5b505af1158015614fbc573d6000803e3d6000fd5b5060009250614fc9915050565b95945050505050565b60055460408051632fe3f38f60e11b81523060048201526001600160a01b0384811660248301528781166044830152868116606483015260848201869052915160009384938493911691635fc7e71e9160a48082019260209290919082900301818787803b15801561504357600080fd5b505af1158015615057573d6000803e3d6000fd5b505050506040513d602081101561506d57600080fd5b505190508015615091576150846003601283613bf4565b92506000915061554c9050565b615099612c87565b600954146150ad57615084600a601661269e565b6150b5612c87565b846001600160a01b0316636c540baf6040518163ffffffff1660e01b815260040160206040518083038186803b1580156150ee57600080fd5b505afa158015615102573d6000803e3d6000fd5b505050506040513d602081101561511857600080fd5b50511461512b57615084600a601161269e565b866001600160a01b0316866001600160a01b03161415615151576150846006601761269e565b84615162576150846007601561269e565b600019851415615178576150846007601461269e565b600080615186898989613712565b909250905081156151b6576151a78260108111156151a057fe5b601861269e565b94506000935061554c92505050565b6005546040805163c488847b60e01b81523060048201526001600160a01b038981166024830152604482018590528251600094859492169263c488847b926064808301939192829003018186803b15801561521057600080fd5b505afa158015615224573d6000803e3d6000fd5b505050506040513d604081101561523a57600080fd5b508051602090910151909250905081156152855760405162461bcd60e51b815260040180806020018281038252603381526020018061596b6033913960400191505060405180910390fd5b80886001600160a01b03166370a082318c6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b1580156152dc57600080fd5b505afa1580156152f0573d6000803e3d6000fd5b505050506040513d602081101561530657600080fd5b5051101561535b576040805162461bcd60e51b815260206004820152601860248201527f4c49515549444154455f5345495a455f544f4f5f4d5543480000000000000000604482015290519081900360640190fd5b60006001600160a01b0389163014156153815761537a308d8d856131d9565b905061540b565b6040805163b2a02ff160e01b81526001600160a01b038e811660048301528d81166024830152604482018590529151918b169163b2a02ff1916064808201926020929091908290030181600087803b1580156153dc57600080fd5b505af11580156153f0573d6000803e3d6000fd5b505050506040513d602081101561540657600080fd5b505190505b8015615455576040805162461bcd60e51b81526020600482015260146024820152731d1bdad95b881cd95a5e9d5c994819985a5b195960621b604482015290519081900360640190fd5b604080516001600160a01b03808f168252808e1660208301528183018790528b1660608201526080810184905290517f298637f684da70674f26509b10f07ec2fbc77a335ab1e7d6215a4b2484d8bb529181900360a00190a1600554604080516347ef3b3b60e01b81523060048201526001600160a01b038c811660248301528f811660448301528e811660648301526084820188905260a48201869052915191909216916347ef3b3b9160c480830192600092919082900301818387803b15801561552057600080fd5b505af1158015615534573d6000803e3d6000fd5b5060009250615541915050565b975092955050505050505b94509492505050565b80820182811015610cc4576040805162461bcd60e51b815260206004820152600c60248201526b6164642d6f766572666c6f7760a01b604482015290519081900360640190fd5b60008060006155a961560d565b6124bb868660006155b861560d565b6000806155cd670de0b6b3a7640000876147ae565b909250905060008260038111156155e057fe5b146155ff575060408051602081019091526000815290925090506124f1565b6124ea818660000151613b44565b6040518060200160405280600081525090565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1061566157805160ff191683800117855561568e565b8280016001018555821561568e579182015b8281111561568e578251825591602001919060010190615673565b5061569a92915061574b565b5090565b6040805161010081019091528060008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040805160e0810190915280600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604080516080810190915280600081526020016000815260200160008152602001600081525090565b6110aa91905b8082111561569a576000815560010161575156fe6f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e63656f6e6c79207468652061646d696e206d617920696e697469616c697a652074686520696d706c656d656e746174696f6e696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c65646f6e6c79207468652061646d696e206d6179206162616e646f6e2074686520696d706c656d656e746174696f6e4d494e545f4e45575f4143434f554e545f42414c414e43455f43414c43554c4154494f4e5f4641494c4544626f72726f7742616c616e636553746f7265643a20626f72726f7742616c616e636553746f726564496e7465726e616c206661696c6564444149206d757374206265207468652073616d6520617320756e6465726c79696e6752455041595f424f52524f575f4e45575f4143434f554e545f424f52524f575f42414c414e43455f43414c43554c4154494f4e5f4641494c4544ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef52455041595f424f52524f575f4e45575f544f54414c5f42414c414e43455f43414c43554c4154494f4e5f4641494c45444c49515549444154455f434f4d5054524f4c4c45525f43414c43554c4154455f414d4f554e545f5345495a455f4641494c4544756e6578706563746564204549502d3230207472616e7366657220696e2072657475726e65786368616e67655261746553746f7265643a2065786368616e67655261746553746f726564496e7465726e616c206661696c65644d494e545f4e45575f544f54414c5f535550504c595f43414c43554c4154494f4e5f4641494c45446f6e65206f662072656465656d546f6b656e73496e206f722072656465656d416d6f756e74496e206d757374206265207a65726f72656475636520726573657276657320756e657870656374656420756e646572666c6f77a265627a7a72315820967ea3df48bbb8573cea217a2fef6f884ff93c9fc820fd29e03f6680175fb65464736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CDaiDelegate<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CDaiDelegate<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CDaiDelegate<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CDaiDelegate))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CDaiDelegate<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CDAIDELEGATE_ABI.clone(), client).into()
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
                CDAIDELEGATE_ABI.clone(),
                CDAIDELEGATE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_acceptAdmin` (0xe9c714f2) function"]
        pub fn accept_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([233, 199, 20, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_addReserves` (0x3e941010) function"]
        pub fn add_reserves(
            &self,
            add_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([62, 148, 16, 16], add_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_becomeImplementation` (0x56e67728) function"]
        pub fn become_implementation(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 230, 119, 40], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_reduceReserves` (0x601a0bf1) function"]
        pub fn reduce_reserves(
            &self,
            reduce_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([96, 26, 11, 241], reduce_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_resignImplementation` (0x153ab505) function"]
        pub fn resign_implementation(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 58, 181, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setComptroller` (0x4576b5db) function"]
        pub fn set_comptroller(
            &self,
            new_comptroller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([69, 118, 181, 219], new_comptroller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setInterestRateModel` (0xf2b3abbd) function"]
        pub fn set_interest_rate_model(
            &self,
            new_interest_rate_model: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 179, 171, 189], new_interest_rate_model)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setPendingAdmin` (0xb71d1a0c) function"]
        pub fn set_pending_admin(
            &self,
            new_pending_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([183, 29, 26, 12], new_pending_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_setReserveFactor` (0xfca7820b) function"]
        pub fn set_reserve_factor(
            &self,
            new_reserve_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 167, 130, 11], new_reserve_factor_mantissa)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrualBlockNumber` (0x6c540baf) function"]
        pub fn accrual_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 84, 11, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrueInterest` (0xa6afed95) function"]
        pub fn accrue_interest(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([166, 175, 237, 149], ())
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
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOfUnderlying` (0x3af9e669) function"]
        pub fn balance_of_underlying(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([58, 249, 230, 105], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0xc5ebeaec) function"]
        pub fn borrow(
            &self,
            borrow_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 235, 234, 236], borrow_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowBalanceCurrent` (0x17bfdfbc) function"]
        pub fn borrow_balance_current(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 191, 223, 188], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowBalanceStored` (0x95dd9193) function"]
        pub fn borrow_balance_stored(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 221, 145, 147], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowIndex` (0xaa5af0fd) function"]
        pub fn borrow_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 90, 240, 253], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowRatePerBlock` (0xf8f9da28) function"]
        pub fn borrow_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 249, 218, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `comptroller` (0x5fe3b567) function"]
        pub fn comptroller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([95, 227, 181, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `daiJoinAddress` (0x8a8df2e6) function"]
        pub fn dai_join_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([138, 141, 242, 230], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchangeRateCurrent` (0xbd6d894d) function"]
        pub fn exchange_rate_current(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([189, 109, 137, 77], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchangeRateStored` (0x182df0f5) function"]
        pub fn exchange_rate_stored(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 45, 240, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountSnapshot` (0xc37f68e2) function"]
        pub fn get_account_snapshot(
            &self,
            account: ethers::core::types::Address,
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
                .method_hash([195, 127, 104, 226], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCash` (0x3b1d21a2) function"]
        pub fn get_cash(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([59, 29, 33, 162], ())
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
        #[doc = "Calls the contract's `initialize` (0x1a31d465) function"]
        pub fn initialize_with_underlying(
            &self,
            underlying: ethers::core::types::Address,
            comptroller: ethers::core::types::Address,
            interest_rate_model: ethers::core::types::Address,
            initial_exchange_rate_mantissa: ethers::core::types::U256,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 49, 212, 101],
                    (
                        underlying,
                        comptroller,
                        interest_rate_model,
                        initial_exchange_rate_mantissa,
                        name,
                        symbol,
                        decimals,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x99d8c1b4) function"]
        pub fn initialize(
            &self,
            comptroller: ethers::core::types::Address,
            interest_rate_model: ethers::core::types::Address,
            initial_exchange_rate_mantissa: ethers::core::types::U256,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 216, 193, 180],
                    (
                        comptroller,
                        interest_rate_model,
                        initial_exchange_rate_mantissa,
                        name,
                        symbol,
                        decimals,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `interestRateModel` (0xf3fdb15a) function"]
        pub fn interest_rate_model(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([243, 253, 177, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCToken` (0xfe9c44ae) function"]
        pub fn is_c_token(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([254, 156, 68, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateBorrow` (0xf5e3c462) function"]
        pub fn liquidate_borrow(
            &self,
            borrower: ethers::core::types::Address,
            repay_amount: ethers::core::types::U256,
            c_token_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [245, 227, 196, 98],
                    (borrower, repay_amount, c_token_collateral),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xa0712d68) function"]
        pub fn mint(
            &self,
            mint_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([160, 113, 45, 104], mint_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `potAddress` (0x66ced602) function"]
        pub fn pot_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([102, 206, 214, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeem` (0xdb006a75) function"]
        pub fn redeem(
            &self,
            redeem_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 0, 106, 117], redeem_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemUnderlying` (0x852a12e3) function"]
        pub fn redeem_underlying(
            &self,
            redeem_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 42, 18, 227], redeem_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrow` (0x0e752702) function"]
        pub fn repay_borrow(
            &self,
            repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([14, 117, 39, 2], repay_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrowBehalf` (0x2608f818) function"]
        pub fn repay_borrow_behalf(
            &self,
            borrower: ethers::core::types::Address,
            repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([38, 8, 248, 24], (borrower, repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reserveFactorMantissa` (0x173b9904) function"]
        pub fn reserve_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 59, 153, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seize` (0xb2a02ff1) function"]
        pub fn seize(
            &self,
            liquidator: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            seize_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([178, 160, 47, 241], (liquidator, borrower, seize_tokens))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supplyRatePerBlock` (0xae9d70b0) function"]
        pub fn supply_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([174, 157, 112, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrows` (0x47bd3718) function"]
        pub fn total_borrows(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([71, 189, 55, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrowsCurrent` (0x73acee98) function"]
        pub fn total_borrows_current(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([115, 172, 238, 152], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalReserves` (0x8f840ddd) function"]
        pub fn total_reserves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([143, 132, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            dst: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vatAddress` (0x8d925ccd) function"]
        pub fn vat_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 146, 92, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AccrueInterest` event"]
        pub fn accrue_interest_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccrueInterestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers::contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Failure` event"]
        pub fn failure_filter(&self) -> ethers::contract::builders::Event<M, FailureFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidateBorrow` event"]
        pub fn liquidate_borrow_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidateBorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewAdmin` event"]
        pub fn new_admin_filter(&self) -> ethers::contract::builders::Event<M, NewAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewComptroller` event"]
        pub fn new_comptroller_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewComptrollerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewMarketInterestRateModel` event"]
        pub fn new_market_interest_rate_model_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewMarketInterestRateModelFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPendingAdmin` event"]
        pub fn new_pending_admin_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPendingAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewReserveFactor` event"]
        pub fn new_reserve_factor_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewReserveFactorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Redeem` event"]
        pub fn redeem_filter(&self) -> ethers::contract::builders::Event<M, RedeemFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RepayBorrow` event"]
        pub fn repay_borrow_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RepayBorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReservesAdded` event"]
        pub fn reserves_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReservesAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReservesReduced` event"]
        pub fn reserves_reduced_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReservesReducedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CDaiDelegateEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CDaiDelegate<M> {
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
    #[ethevent(
        name = "AccrueInterest",
        abi = "AccrueInterest(uint256,uint256,uint256,uint256)"
    )]
    pub struct AccrueInterestFilter {
        pub cash_prior: ethers::core::types::U256,
        pub interest_accumulated: ethers::core::types::U256,
        pub borrow_index: ethers::core::types::U256,
        pub total_borrows: ethers::core::types::U256,
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
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
    #[ethevent(name = "Borrow", abi = "Borrow(address,uint256,uint256,uint256)")]
    pub struct BorrowFilter {
        pub borrower: ethers::core::types::Address,
        pub borrow_amount: ethers::core::types::U256,
        pub account_borrows: ethers::core::types::U256,
        pub total_borrows: ethers::core::types::U256,
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
    #[ethevent(
        name = "LiquidateBorrow",
        abi = "LiquidateBorrow(address,address,uint256,address,uint256)"
    )]
    pub struct LiquidateBorrowFilter {
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
        pub c_token_collateral: ethers::core::types::Address,
        pub seize_tokens: ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        pub minter: ethers::core::types::Address,
        pub mint_amount: ethers::core::types::U256,
        pub mint_tokens: ethers::core::types::U256,
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
    #[ethevent(name = "NewAdmin", abi = "NewAdmin(address,address)")]
    pub struct NewAdminFilter {
        pub old_admin: ethers::core::types::Address,
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "NewComptroller", abi = "NewComptroller(address,address)")]
    pub struct NewComptrollerFilter {
        pub old_comptroller: ethers::core::types::Address,
        pub new_comptroller: ethers::core::types::Address,
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
        name = "NewMarketInterestRateModel",
        abi = "NewMarketInterestRateModel(address,address)"
    )]
    pub struct NewMarketInterestRateModelFilter {
        pub old_interest_rate_model: ethers::core::types::Address,
        pub new_interest_rate_model: ethers::core::types::Address,
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
    #[ethevent(name = "NewPendingAdmin", abi = "NewPendingAdmin(address,address)")]
    pub struct NewPendingAdminFilter {
        pub old_pending_admin: ethers::core::types::Address,
        pub new_pending_admin: ethers::core::types::Address,
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
    #[ethevent(name = "NewReserveFactor", abi = "NewReserveFactor(uint256,uint256)")]
    pub struct NewReserveFactorFilter {
        pub old_reserve_factor_mantissa: ethers::core::types::U256,
        pub new_reserve_factor_mantissa: ethers::core::types::U256,
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
    #[ethevent(name = "Redeem", abi = "Redeem(address,uint256,uint256)")]
    pub struct RedeemFilter {
        pub redeemer: ethers::core::types::Address,
        pub redeem_amount: ethers::core::types::U256,
        pub redeem_tokens: ethers::core::types::U256,
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
        name = "RepayBorrow",
        abi = "RepayBorrow(address,address,uint256,uint256,uint256)"
    )]
    pub struct RepayBorrowFilter {
        pub payer: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
        pub account_borrows: ethers::core::types::U256,
        pub total_borrows: ethers::core::types::U256,
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
    #[ethevent(name = "ReservesAdded", abi = "ReservesAdded(address,uint256,uint256)")]
    pub struct ReservesAddedFilter {
        pub benefactor: ethers::core::types::Address,
        pub add_amount: ethers::core::types::U256,
        pub new_total_reserves: ethers::core::types::U256,
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
        name = "ReservesReduced",
        abi = "ReservesReduced(address,uint256,uint256)"
    )]
    pub struct ReservesReducedFilter {
        pub admin: ethers::core::types::Address,
        pub reduce_amount: ethers::core::types::U256,
        pub new_total_reserves: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CDaiDelegateEvents {
        AccrueInterestFilter(AccrueInterestFilter),
        ApprovalFilter(ApprovalFilter),
        BorrowFilter(BorrowFilter),
        FailureFilter(FailureFilter),
        LiquidateBorrowFilter(LiquidateBorrowFilter),
        MintFilter(MintFilter),
        NewAdminFilter(NewAdminFilter),
        NewComptrollerFilter(NewComptrollerFilter),
        NewMarketInterestRateModelFilter(NewMarketInterestRateModelFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        NewReserveFactorFilter(NewReserveFactorFilter),
        RedeemFilter(RedeemFilter),
        RepayBorrowFilter(RepayBorrowFilter),
        ReservesAddedFilter(ReservesAddedFilter),
        ReservesReducedFilter(ReservesReducedFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for CDaiDelegateEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::NewMarketInterestRateModelFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CDaiDelegateEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CDaiDelegateEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CDaiDelegateEvents::AccrueInterestFilter(element) => element.fmt(f),
                CDaiDelegateEvents::ApprovalFilter(element) => element.fmt(f),
                CDaiDelegateEvents::BorrowFilter(element) => element.fmt(f),
                CDaiDelegateEvents::FailureFilter(element) => element.fmt(f),
                CDaiDelegateEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CDaiDelegateEvents::MintFilter(element) => element.fmt(f),
                CDaiDelegateEvents::NewAdminFilter(element) => element.fmt(f),
                CDaiDelegateEvents::NewComptrollerFilter(element) => element.fmt(f),
                CDaiDelegateEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CDaiDelegateEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CDaiDelegateEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CDaiDelegateEvents::RedeemFilter(element) => element.fmt(f),
                CDaiDelegateEvents::RepayBorrowFilter(element) => element.fmt(f),
                CDaiDelegateEvents::ReservesAddedFilter(element) => element.fmt(f),
                CDaiDelegateEvents::ReservesReducedFilter(element) => element.fmt(f),
                CDaiDelegateEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_acceptAdmin`function with signature `_acceptAdmin()` and selector `[233, 199, 20, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_acceptAdmin", abi = "_acceptAdmin()")]
    pub struct AcceptAdminCall;
    #[doc = "Container type for all input parameters for the `_addReserves`function with signature `_addReserves(uint256)` and selector `[62, 148, 16, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_addReserves", abi = "_addReserves(uint256)")]
    pub struct AddReservesCall {
        pub add_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_becomeImplementation`function with signature `_becomeImplementation(bytes)` and selector `[86, 230, 119, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_becomeImplementation", abi = "_becomeImplementation(bytes)")]
    pub struct BecomeImplementationCall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `_reduceReserves`function with signature `_reduceReserves(uint256)` and selector `[96, 26, 11, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_reduceReserves", abi = "_reduceReserves(uint256)")]
    pub struct ReduceReservesCall {
        pub reduce_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_resignImplementation`function with signature `_resignImplementation()` and selector `[21, 58, 181, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_resignImplementation", abi = "_resignImplementation()")]
    pub struct ResignImplementationCall;
    #[doc = "Container type for all input parameters for the `_setComptroller`function with signature `_setComptroller(address)` and selector `[69, 118, 181, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setComptroller", abi = "_setComptroller(address)")]
    pub struct SetComptrollerCall {
        pub new_comptroller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setInterestRateModel`function with signature `_setInterestRateModel(address)` and selector `[242, 179, 171, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setInterestRateModel", abi = "_setInterestRateModel(address)")]
    pub struct SetInterestRateModelCall {
        pub new_interest_rate_model: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setPendingAdmin`function with signature `_setPendingAdmin(address)` and selector `[183, 29, 26, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setPendingAdmin", abi = "_setPendingAdmin(address)")]
    pub struct SetPendingAdminCall {
        pub new_pending_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_setReserveFactor`function with signature `_setReserveFactor(uint256)` and selector `[252, 167, 130, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_setReserveFactor", abi = "_setReserveFactor(uint256)")]
    pub struct SetReserveFactorCall {
        pub new_reserve_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `accrualBlockNumber`function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accrualBlockNumber", abi = "accrualBlockNumber()")]
    pub struct AccrualBlockNumberCall;
    #[doc = "Container type for all input parameters for the `accrueInterest`function with signature `accrueInterest()` and selector `[166, 175, 237, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accrueInterest", abi = "accrueInterest()")]
    pub struct AccrueInterestCall;
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
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `balanceOfUnderlying`function with signature `balanceOfUnderlying(address)` and selector `[58, 249, 230, 105]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOfUnderlying", abi = "balanceOfUnderlying(address)")]
    pub struct BalanceOfUnderlyingCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrow`function with signature `borrow(uint256)` and selector `[197, 235, 234, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrow", abi = "borrow(uint256)")]
    pub struct BorrowCall {
        pub borrow_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrowBalanceCurrent`function with signature `borrowBalanceCurrent(address)` and selector `[23, 191, 223, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowBalanceCurrent", abi = "borrowBalanceCurrent(address)")]
    pub struct BorrowBalanceCurrentCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowBalanceStored`function with signature `borrowBalanceStored(address)` and selector `[149, 221, 145, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowBalanceStored", abi = "borrowBalanceStored(address)")]
    pub struct BorrowBalanceStoredCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowIndex`function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowIndex", abi = "borrowIndex()")]
    pub struct BorrowIndexCall;
    #[doc = "Container type for all input parameters for the `borrowRatePerBlock`function with signature `borrowRatePerBlock()` and selector `[248, 249, 218, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowRatePerBlock", abi = "borrowRatePerBlock()")]
    pub struct BorrowRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `comptroller`function with signature `comptroller()` and selector `[95, 227, 181, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "comptroller", abi = "comptroller()")]
    pub struct ComptrollerCall;
    #[doc = "Container type for all input parameters for the `daiJoinAddress`function with signature `daiJoinAddress()` and selector `[138, 141, 242, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "daiJoinAddress", abi = "daiJoinAddress()")]
    pub struct DaiJoinAddressCall;
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `exchangeRateCurrent`function with signature `exchangeRateCurrent()` and selector `[189, 109, 137, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exchangeRateCurrent", abi = "exchangeRateCurrent()")]
    pub struct ExchangeRateCurrentCall;
    #[doc = "Container type for all input parameters for the `exchangeRateStored`function with signature `exchangeRateStored()` and selector `[24, 45, 240, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exchangeRateStored", abi = "exchangeRateStored()")]
    pub struct ExchangeRateStoredCall;
    #[doc = "Container type for all input parameters for the `getAccountSnapshot`function with signature `getAccountSnapshot(address)` and selector `[195, 127, 104, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountSnapshot", abi = "getAccountSnapshot(address)")]
    pub struct GetAccountSnapshotCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCash`function with signature `getCash()` and selector `[59, 29, 33, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCash", abi = "getCash()")]
    pub struct GetCashCall;
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
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address,address,uint256,string,string,uint8)` and selector `[26, 49, 212, 101]`"]
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
        abi = "initialize(address,address,address,uint256,string,string,uint8)"
    )]
    pub struct InitializeWithUnderlyingCall {
        pub underlying: ethers::core::types::Address,
        pub comptroller: ethers::core::types::Address,
        pub interest_rate_model: ethers::core::types::Address,
        pub initial_exchange_rate_mantissa: ethers::core::types::U256,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address,uint256,string,string,uint8)` and selector `[153, 216, 193, 180]`"]
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
        abi = "initialize(address,address,uint256,string,string,uint8)"
    )]
    pub struct InitializeCall {
        pub comptroller: ethers::core::types::Address,
        pub interest_rate_model: ethers::core::types::Address,
        pub initial_exchange_rate_mantissa: ethers::core::types::U256,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `interestRateModel`function with signature `interestRateModel()` and selector `[243, 253, 177, 90]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "interestRateModel", abi = "interestRateModel()")]
    pub struct InterestRateModelCall;
    #[doc = "Container type for all input parameters for the `isCToken`function with signature `isCToken()` and selector `[254, 156, 68, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCToken", abi = "isCToken()")]
    pub struct IsCTokenCall;
    #[doc = "Container type for all input parameters for the `liquidateBorrow`function with signature `liquidateBorrow(address,uint256,address)` and selector `[245, 227, 196, 98]`"]
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
        name = "liquidateBorrow",
        abi = "liquidateBorrow(address,uint256,address)"
    )]
    pub struct LiquidateBorrowCall {
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
        pub c_token_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256)` and selector `[160, 113, 45, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(uint256)")]
    pub struct MintCall {
        pub mint_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    #[doc = "Container type for all input parameters for the `potAddress`function with signature `potAddress()` and selector `[102, 206, 214, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "potAddress", abi = "potAddress()")]
    pub struct PotAddressCall;
    #[doc = "Container type for all input parameters for the `redeem`function with signature `redeem(uint256)` and selector `[219, 0, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "redeem", abi = "redeem(uint256)")]
    pub struct RedeemCall {
        pub redeem_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `redeemUnderlying`function with signature `redeemUnderlying(uint256)` and selector `[133, 42, 18, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "redeemUnderlying", abi = "redeemUnderlying(uint256)")]
    pub struct RedeemUnderlyingCall {
        pub redeem_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repayBorrow`function with signature `repayBorrow(uint256)` and selector `[14, 117, 39, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayBorrow", abi = "repayBorrow(uint256)")]
    pub struct RepayBorrowCall {
        pub repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repayBorrowBehalf`function with signature `repayBorrowBehalf(address,uint256)` and selector `[38, 8, 248, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayBorrowBehalf", abi = "repayBorrowBehalf(address,uint256)")]
    pub struct RepayBorrowBehalfCall {
        pub borrower: ethers::core::types::Address,
        pub repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `reserveFactorMantissa`function with signature `reserveFactorMantissa()` and selector `[23, 59, 153, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "reserveFactorMantissa", abi = "reserveFactorMantissa()")]
    pub struct ReserveFactorMantissaCall;
    #[doc = "Container type for all input parameters for the `seize`function with signature `seize(address,address,uint256)` and selector `[178, 160, 47, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "seize", abi = "seize(address,address,uint256)")]
    pub struct SeizeCall {
        pub liquidator: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub seize_tokens: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `supplyRatePerBlock`function with signature `supplyRatePerBlock()` and selector `[174, 157, 112, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supplyRatePerBlock", abi = "supplyRatePerBlock()")]
    pub struct SupplyRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalBorrows`function with signature `totalBorrows()` and selector `[71, 189, 55, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalBorrows", abi = "totalBorrows()")]
    pub struct TotalBorrowsCall;
    #[doc = "Container type for all input parameters for the `totalBorrowsCurrent`function with signature `totalBorrowsCurrent()` and selector `[115, 172, 238, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalBorrowsCurrent", abi = "totalBorrowsCurrent()")]
    pub struct TotalBorrowsCurrentCall;
    #[doc = "Container type for all input parameters for the `totalReserves`function with signature `totalReserves()` and selector `[143, 132, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalReserves", abi = "totalReserves()")]
    pub struct TotalReservesCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub dst: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `underlying`function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    #[doc = "Container type for all input parameters for the `vatAddress`function with signature `vatAddress()` and selector `[141, 146, 92, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vatAddress", abi = "vatAddress()")]
    pub struct VatAddressCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CDaiDelegateCalls {
        AcceptAdmin(AcceptAdminCall),
        AddReserves(AddReservesCall),
        BecomeImplementation(BecomeImplementationCall),
        ReduceReserves(ReduceReservesCall),
        ResignImplementation(ResignImplementationCall),
        SetComptroller(SetComptrollerCall),
        SetInterestRateModel(SetInterestRateModelCall),
        SetPendingAdmin(SetPendingAdminCall),
        SetReserveFactor(SetReserveFactorCall),
        AccrualBlockNumber(AccrualBlockNumberCall),
        AccrueInterest(AccrueInterestCall),
        Admin(AdminCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BalanceOfUnderlying(BalanceOfUnderlyingCall),
        Borrow(BorrowCall),
        BorrowBalanceCurrent(BorrowBalanceCurrentCall),
        BorrowBalanceStored(BorrowBalanceStoredCall),
        BorrowIndex(BorrowIndexCall),
        BorrowRatePerBlock(BorrowRatePerBlockCall),
        Comptroller(ComptrollerCall),
        DaiJoinAddress(DaiJoinAddressCall),
        Decimals(DecimalsCall),
        ExchangeRateCurrent(ExchangeRateCurrentCall),
        ExchangeRateStored(ExchangeRateStoredCall),
        GetAccountSnapshot(GetAccountSnapshotCall),
        GetCash(GetCashCall),
        Implementation(ImplementationCall),
        InitializeWithUnderlying(InitializeWithUnderlyingCall),
        Initialize(InitializeCall),
        InterestRateModel(InterestRateModelCall),
        IsCToken(IsCTokenCall),
        LiquidateBorrow(LiquidateBorrowCall),
        Mint(MintCall),
        Name(NameCall),
        PendingAdmin(PendingAdminCall),
        PotAddress(PotAddressCall),
        Redeem(RedeemCall),
        RedeemUnderlying(RedeemUnderlyingCall),
        RepayBorrow(RepayBorrowCall),
        RepayBorrowBehalf(RepayBorrowBehalfCall),
        ReserveFactorMantissa(ReserveFactorMantissaCall),
        Seize(SeizeCall),
        SupplyRatePerBlock(SupplyRatePerBlockCall),
        Symbol(SymbolCall),
        TotalBorrows(TotalBorrowsCall),
        TotalBorrowsCurrent(TotalBorrowsCurrentCall),
        TotalReserves(TotalReservesCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Underlying(UnderlyingCall),
        VatAddress(VatAddressCall),
    }
    impl ethers::core::abi::AbiDecode for CDaiDelegateCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::AddReserves(decoded));
            }
            if let Ok(decoded) =
                <BecomeImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BecomeImplementation(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <ResignImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::ResignImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DaiJoinAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::DaiJoinAddress(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InitializeWithUnderlyingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CDaiDelegateCalls::InitializeWithUnderlying(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::IsCToken(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CDaiDelegateCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CDaiDelegateCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PotAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::PotAddress(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VatAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CDaiDelegateCalls::VatAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CDaiDelegateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CDaiDelegateCalls::AcceptAdmin(element) => element.encode(),
                CDaiDelegateCalls::AddReserves(element) => element.encode(),
                CDaiDelegateCalls::BecomeImplementation(element) => element.encode(),
                CDaiDelegateCalls::ReduceReserves(element) => element.encode(),
                CDaiDelegateCalls::ResignImplementation(element) => element.encode(),
                CDaiDelegateCalls::SetComptroller(element) => element.encode(),
                CDaiDelegateCalls::SetInterestRateModel(element) => element.encode(),
                CDaiDelegateCalls::SetPendingAdmin(element) => element.encode(),
                CDaiDelegateCalls::SetReserveFactor(element) => element.encode(),
                CDaiDelegateCalls::AccrualBlockNumber(element) => element.encode(),
                CDaiDelegateCalls::AccrueInterest(element) => element.encode(),
                CDaiDelegateCalls::Admin(element) => element.encode(),
                CDaiDelegateCalls::Allowance(element) => element.encode(),
                CDaiDelegateCalls::Approve(element) => element.encode(),
                CDaiDelegateCalls::BalanceOf(element) => element.encode(),
                CDaiDelegateCalls::BalanceOfUnderlying(element) => element.encode(),
                CDaiDelegateCalls::Borrow(element) => element.encode(),
                CDaiDelegateCalls::BorrowBalanceCurrent(element) => element.encode(),
                CDaiDelegateCalls::BorrowBalanceStored(element) => element.encode(),
                CDaiDelegateCalls::BorrowIndex(element) => element.encode(),
                CDaiDelegateCalls::BorrowRatePerBlock(element) => element.encode(),
                CDaiDelegateCalls::Comptroller(element) => element.encode(),
                CDaiDelegateCalls::DaiJoinAddress(element) => element.encode(),
                CDaiDelegateCalls::Decimals(element) => element.encode(),
                CDaiDelegateCalls::ExchangeRateCurrent(element) => element.encode(),
                CDaiDelegateCalls::ExchangeRateStored(element) => element.encode(),
                CDaiDelegateCalls::GetAccountSnapshot(element) => element.encode(),
                CDaiDelegateCalls::GetCash(element) => element.encode(),
                CDaiDelegateCalls::Implementation(element) => element.encode(),
                CDaiDelegateCalls::InitializeWithUnderlying(element) => element.encode(),
                CDaiDelegateCalls::Initialize(element) => element.encode(),
                CDaiDelegateCalls::InterestRateModel(element) => element.encode(),
                CDaiDelegateCalls::IsCToken(element) => element.encode(),
                CDaiDelegateCalls::LiquidateBorrow(element) => element.encode(),
                CDaiDelegateCalls::Mint(element) => element.encode(),
                CDaiDelegateCalls::Name(element) => element.encode(),
                CDaiDelegateCalls::PendingAdmin(element) => element.encode(),
                CDaiDelegateCalls::PotAddress(element) => element.encode(),
                CDaiDelegateCalls::Redeem(element) => element.encode(),
                CDaiDelegateCalls::RedeemUnderlying(element) => element.encode(),
                CDaiDelegateCalls::RepayBorrow(element) => element.encode(),
                CDaiDelegateCalls::RepayBorrowBehalf(element) => element.encode(),
                CDaiDelegateCalls::ReserveFactorMantissa(element) => element.encode(),
                CDaiDelegateCalls::Seize(element) => element.encode(),
                CDaiDelegateCalls::SupplyRatePerBlock(element) => element.encode(),
                CDaiDelegateCalls::Symbol(element) => element.encode(),
                CDaiDelegateCalls::TotalBorrows(element) => element.encode(),
                CDaiDelegateCalls::TotalBorrowsCurrent(element) => element.encode(),
                CDaiDelegateCalls::TotalReserves(element) => element.encode(),
                CDaiDelegateCalls::TotalSupply(element) => element.encode(),
                CDaiDelegateCalls::Transfer(element) => element.encode(),
                CDaiDelegateCalls::TransferFrom(element) => element.encode(),
                CDaiDelegateCalls::Underlying(element) => element.encode(),
                CDaiDelegateCalls::VatAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CDaiDelegateCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CDaiDelegateCalls::AcceptAdmin(element) => element.fmt(f),
                CDaiDelegateCalls::AddReserves(element) => element.fmt(f),
                CDaiDelegateCalls::BecomeImplementation(element) => element.fmt(f),
                CDaiDelegateCalls::ReduceReserves(element) => element.fmt(f),
                CDaiDelegateCalls::ResignImplementation(element) => element.fmt(f),
                CDaiDelegateCalls::SetComptroller(element) => element.fmt(f),
                CDaiDelegateCalls::SetInterestRateModel(element) => element.fmt(f),
                CDaiDelegateCalls::SetPendingAdmin(element) => element.fmt(f),
                CDaiDelegateCalls::SetReserveFactor(element) => element.fmt(f),
                CDaiDelegateCalls::AccrualBlockNumber(element) => element.fmt(f),
                CDaiDelegateCalls::AccrueInterest(element) => element.fmt(f),
                CDaiDelegateCalls::Admin(element) => element.fmt(f),
                CDaiDelegateCalls::Allowance(element) => element.fmt(f),
                CDaiDelegateCalls::Approve(element) => element.fmt(f),
                CDaiDelegateCalls::BalanceOf(element) => element.fmt(f),
                CDaiDelegateCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CDaiDelegateCalls::Borrow(element) => element.fmt(f),
                CDaiDelegateCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CDaiDelegateCalls::BorrowBalanceStored(element) => element.fmt(f),
                CDaiDelegateCalls::BorrowIndex(element) => element.fmt(f),
                CDaiDelegateCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CDaiDelegateCalls::Comptroller(element) => element.fmt(f),
                CDaiDelegateCalls::DaiJoinAddress(element) => element.fmt(f),
                CDaiDelegateCalls::Decimals(element) => element.fmt(f),
                CDaiDelegateCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CDaiDelegateCalls::ExchangeRateStored(element) => element.fmt(f),
                CDaiDelegateCalls::GetAccountSnapshot(element) => element.fmt(f),
                CDaiDelegateCalls::GetCash(element) => element.fmt(f),
                CDaiDelegateCalls::Implementation(element) => element.fmt(f),
                CDaiDelegateCalls::InitializeWithUnderlying(element) => element.fmt(f),
                CDaiDelegateCalls::Initialize(element) => element.fmt(f),
                CDaiDelegateCalls::InterestRateModel(element) => element.fmt(f),
                CDaiDelegateCalls::IsCToken(element) => element.fmt(f),
                CDaiDelegateCalls::LiquidateBorrow(element) => element.fmt(f),
                CDaiDelegateCalls::Mint(element) => element.fmt(f),
                CDaiDelegateCalls::Name(element) => element.fmt(f),
                CDaiDelegateCalls::PendingAdmin(element) => element.fmt(f),
                CDaiDelegateCalls::PotAddress(element) => element.fmt(f),
                CDaiDelegateCalls::Redeem(element) => element.fmt(f),
                CDaiDelegateCalls::RedeemUnderlying(element) => element.fmt(f),
                CDaiDelegateCalls::RepayBorrow(element) => element.fmt(f),
                CDaiDelegateCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CDaiDelegateCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CDaiDelegateCalls::Seize(element) => element.fmt(f),
                CDaiDelegateCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CDaiDelegateCalls::Symbol(element) => element.fmt(f),
                CDaiDelegateCalls::TotalBorrows(element) => element.fmt(f),
                CDaiDelegateCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CDaiDelegateCalls::TotalReserves(element) => element.fmt(f),
                CDaiDelegateCalls::TotalSupply(element) => element.fmt(f),
                CDaiDelegateCalls::Transfer(element) => element.fmt(f),
                CDaiDelegateCalls::TransferFrom(element) => element.fmt(f),
                CDaiDelegateCalls::Underlying(element) => element.fmt(f),
                CDaiDelegateCalls::VatAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CDaiDelegateCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CDaiDelegateCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<AddReservesCall> for CDaiDelegateCalls {
        fn from(var: AddReservesCall) -> Self {
            CDaiDelegateCalls::AddReserves(var)
        }
    }
    impl ::std::convert::From<BecomeImplementationCall> for CDaiDelegateCalls {
        fn from(var: BecomeImplementationCall) -> Self {
            CDaiDelegateCalls::BecomeImplementation(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CDaiDelegateCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CDaiDelegateCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<ResignImplementationCall> for CDaiDelegateCalls {
        fn from(var: ResignImplementationCall) -> Self {
            CDaiDelegateCalls::ResignImplementation(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CDaiDelegateCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CDaiDelegateCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CDaiDelegateCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CDaiDelegateCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CDaiDelegateCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CDaiDelegateCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CDaiDelegateCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CDaiDelegateCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CDaiDelegateCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CDaiDelegateCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CDaiDelegateCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CDaiDelegateCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CDaiDelegateCalls {
        fn from(var: AdminCall) -> Self {
            CDaiDelegateCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CDaiDelegateCalls {
        fn from(var: AllowanceCall) -> Self {
            CDaiDelegateCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CDaiDelegateCalls {
        fn from(var: ApproveCall) -> Self {
            CDaiDelegateCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CDaiDelegateCalls {
        fn from(var: BalanceOfCall) -> Self {
            CDaiDelegateCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CDaiDelegateCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CDaiDelegateCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CDaiDelegateCalls {
        fn from(var: BorrowCall) -> Self {
            CDaiDelegateCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CDaiDelegateCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CDaiDelegateCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CDaiDelegateCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CDaiDelegateCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CDaiDelegateCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CDaiDelegateCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CDaiDelegateCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CDaiDelegateCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CDaiDelegateCalls {
        fn from(var: ComptrollerCall) -> Self {
            CDaiDelegateCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DaiJoinAddressCall> for CDaiDelegateCalls {
        fn from(var: DaiJoinAddressCall) -> Self {
            CDaiDelegateCalls::DaiJoinAddress(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CDaiDelegateCalls {
        fn from(var: DecimalsCall) -> Self {
            CDaiDelegateCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CDaiDelegateCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CDaiDelegateCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CDaiDelegateCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CDaiDelegateCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CDaiDelegateCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CDaiDelegateCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CDaiDelegateCalls {
        fn from(var: GetCashCall) -> Self {
            CDaiDelegateCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for CDaiDelegateCalls {
        fn from(var: ImplementationCall) -> Self {
            CDaiDelegateCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InitializeWithUnderlyingCall> for CDaiDelegateCalls {
        fn from(var: InitializeWithUnderlyingCall) -> Self {
            CDaiDelegateCalls::InitializeWithUnderlying(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CDaiDelegateCalls {
        fn from(var: InitializeCall) -> Self {
            CDaiDelegateCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CDaiDelegateCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CDaiDelegateCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CDaiDelegateCalls {
        fn from(var: IsCTokenCall) -> Self {
            CDaiDelegateCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CDaiDelegateCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CDaiDelegateCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CDaiDelegateCalls {
        fn from(var: MintCall) -> Self {
            CDaiDelegateCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CDaiDelegateCalls {
        fn from(var: NameCall) -> Self {
            CDaiDelegateCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CDaiDelegateCalls {
        fn from(var: PendingAdminCall) -> Self {
            CDaiDelegateCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PotAddressCall> for CDaiDelegateCalls {
        fn from(var: PotAddressCall) -> Self {
            CDaiDelegateCalls::PotAddress(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CDaiDelegateCalls {
        fn from(var: RedeemCall) -> Self {
            CDaiDelegateCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CDaiDelegateCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CDaiDelegateCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CDaiDelegateCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CDaiDelegateCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CDaiDelegateCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CDaiDelegateCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CDaiDelegateCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CDaiDelegateCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CDaiDelegateCalls {
        fn from(var: SeizeCall) -> Self {
            CDaiDelegateCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CDaiDelegateCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CDaiDelegateCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CDaiDelegateCalls {
        fn from(var: SymbolCall) -> Self {
            CDaiDelegateCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CDaiDelegateCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CDaiDelegateCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CDaiDelegateCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CDaiDelegateCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CDaiDelegateCalls {
        fn from(var: TotalReservesCall) -> Self {
            CDaiDelegateCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CDaiDelegateCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CDaiDelegateCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CDaiDelegateCalls {
        fn from(var: TransferCall) -> Self {
            CDaiDelegateCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CDaiDelegateCalls {
        fn from(var: TransferFromCall) -> Self {
            CDaiDelegateCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for CDaiDelegateCalls {
        fn from(var: UnderlyingCall) -> Self {
            CDaiDelegateCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VatAddressCall> for CDaiDelegateCalls {
        fn from(var: VatAddressCall) -> Self {
            CDaiDelegateCalls::VatAddress(var)
        }
    }
}
