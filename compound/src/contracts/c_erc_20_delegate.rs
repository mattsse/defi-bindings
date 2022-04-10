pub use cerc20delegate_mod::*;
#[allow(clippy::too_many_arguments)]
mod cerc20delegate_mod {
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
    #[doc = "CErc20Delegate was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CERC20DELEGATE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_becomeImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_resignImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract CTokenInterface\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CERC20DELEGATE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50615144806100206000396000f3fe608060405234801561001057600080fd5b50600436106102f15760003560e01c806373acee981161019d578063bd6d894d116100e9578063f2b3abbd116100a2578063f851a4401161007c578063f851a44014610b32578063f8f9da2814610b3a578063fca7820b14610b42578063fe9c44ae14610b5f576102f1565b8063f2b3abbd14610ace578063f3fdb15a14610af4578063f5e3c46214610afc576102f1565b8063bd6d894d14610a0a578063c37f68e214610a12578063c5ebeaec14610a5e578063db006a7514610a7b578063dd62ed3e14610a98578063e9c714f214610ac6576102f1565b8063a0712d6811610156578063aa5af0fd11610130578063aa5af0fd1461099e578063ae9d70b0146109a6578063b2a02ff1146109ae578063b71d1a0c146109e4576102f1565b8063a0712d681461094d578063a6afed951461096a578063a9059cbb14610972576102f1565b806373acee98146107a4578063852a12e3146107ac5780638f840ddd146107c957806395d89b41146107d157806395dd9193146107d957806399d8c1b4146107ff576102f1565b8063313ce5671161025c57806356e6772811610215578063601a0bf1116101ef578063601a0bf1146107515780636c540baf1461076e5780636f307dc31461077657806370a082311461077e576102f1565b806356e677281461069d5780635c60da1b146107415780635fe3b56714610749576102f1565b8063313ce567146106065780633af9e669146106245780633b1d21a21461064a5780633e941010146106525780634576b5db1461066f57806347bd371814610695576102f1565b806318160ddd116102ae57806318160ddd1461041a578063182df0f5146104225780631a31d4651461042a57806323b872dd146105805780632608f818146105b657806326782247146105e2576102f1565b806306fdde03146102f6578063095ea7b3146103735780630e752702146103b3578063153ab505146103e2578063173b9904146103ec57806317bfdfbc146103f4575b600080fd5b6102fe610b67565b6040805160208082528351818301528351919283929083019185019080838360005b83811015610338578181015183820152602001610320565b50505050905090810190601f1680156103655780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b61039f6004803603604081101561038957600080fd5b506001600160a01b038135169060200135610bf4565b604080519115158252519081900360200190f35b6103d0600480360360208110156103c957600080fd5b5035610c61565b60408051918252519081900360200190f35b6103ea610c77565b005b6103d0610cc7565b6103d06004803603602081101561040a57600080fd5b50356001600160a01b0316610ccd565b6103d0610d8d565b6103d0610d93565b6103ea600480360360e081101561044057600080fd5b6001600160a01b03823581169260208101358216926040820135909216916060820135919081019060a081016080820135600160201b81111561048257600080fd5b82018360208201111561049457600080fd5b803590602001918460018302840111600160201b831117156104b557600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561050757600080fd5b82018360208201111561051957600080fd5b803590602001918460018302840111600160201b8311171561053a57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff169150610df69050565b61039f6004803603606081101561059657600080fd5b506001600160a01b03813581169160208101359091169060400135610e95565b6103d0600480360360408110156105cc57600080fd5b506001600160a01b038135169060200135610f07565b6105ea610f1d565b604080516001600160a01b039092168252519081900360200190f35b61060e610f2c565b6040805160ff9092168252519081900360200190f35b6103d06004803603602081101561063a57600080fd5b50356001600160a01b0316610f35565b6103d0610feb565b6103d06004803603602081101561066857600080fd5b5035610ffa565b6103d06004803603602081101561068557600080fd5b50356001600160a01b0316611005565b6103d061115a565b6103ea600480360360208110156106b357600080fd5b810190602081018135600160201b8111156106cd57600080fd5b8201836020820111156106df57600080fd5b803590602001918460018302840111600160201b8311171561070057600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550611160945050505050565b6105ea6111b1565b6105ea6111c0565b6103d06004803603602081101561076757600080fd5b50356111cf565b6103d061126a565b6105ea611270565b6103d06004803603602081101561079457600080fd5b50356001600160a01b031661127f565b6103d061129a565b6103d0600480360360208110156107c257600080fd5b5035611350565b6103d061135b565b6102fe611361565b6103d0600480360360208110156107ef57600080fd5b50356001600160a01b03166113b9565b6103ea600480360360c081101561081557600080fd5b6001600160a01b03823581169260208101359091169160408201359190810190608081016060820135600160201b81111561084f57600080fd5b82018360208201111561086157600080fd5b803590602001918460018302840111600160201b8311171561088257600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b8111156108d457600080fd5b8201836020820111156108e657600080fd5b803590602001918460018302840111600160201b8311171561090757600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff1691506114169050565b6103d06004803603602081101561096357600080fd5b50356115fd565b6103d0611609565b61039f6004803603604081101561098857600080fd5b506001600160a01b038135169060200135611961565b6103d06119d2565b6103d06119d8565b6103d0600480360360608110156109c457600080fd5b506001600160a01b03813581169160208101359091169060400135611a77565b6103d0600480360360208110156109fa57600080fd5b50356001600160a01b0316611ae8565b6103d0611b74565b610a3860048036036020811015610a2857600080fd5b50356001600160a01b0316611c30565b604080519485526020850193909352838301919091526060830152519081900360800190f35b6103d060048036036020811015610a7457600080fd5b5035611cc5565b6103d060048036036020811015610a9157600080fd5b5035611cd0565b6103d060048036036040811015610aae57600080fd5b506001600160a01b0381358116916020013516611cdb565b6103d0611d06565b6103d060048036036020811015610ae457600080fd5b50356001600160a01b0316611e09565b6105ea611e43565b6103d060048036036060811015610b1257600080fd5b506001600160a01b03813581169160208101359160409091013516611e52565b6105ea611e6a565b6103d0611e7e565b6103d060048036036020811015610b5857600080fd5b5035611ee2565b61039f611f60565b60018054604080516020600284861615610100026000190190941693909304601f81018490048402820184019092528181529291830182828015610bec5780601f10610bc157610100808354040283529160200191610bec565b820191906000526020600020905b815481529060010190602001808311610bcf57829003601f168201915b505050505081565b336000818152600f602090815260408083206001600160a01b03871680855290835281842086905581518681529151939493909284927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929081900390910190a360019150505b92915050565b600080610c6d83611f65565b509150505b919050565b60035461010090046001600160a01b03163314610cc55760405162461bcd60e51b815260040180806020018281038252602d815260200180614ee1602d913960400191505060405180910390fd5b565b60085481565b6000805460ff16610d12576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610d24611609565b14610d6f576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b610d78826113b9565b90505b6000805460ff19166001179055919050565b600d5481565b6000806000610da061200e565b90925090506000826003811115610db357fe5b14610def5760405162461bcd60e51b815260040180806020018281038252603581526020018061502e6035913960400191505060405180910390fd5b9150505b90565b610e04868686868686611416565b601180546001600160a01b0319166001600160a01b038981169190911791829055604080516318160ddd60e01b8152905192909116916318160ddd91600480820192602092909190829003018186803b158015610e6057600080fd5b505afa158015610e74573d6000803e3d6000fd5b505050506040513d6020811015610e8a57600080fd5b505050505050505050565b6000805460ff16610eda576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610ef0338686866120bd565b1490506000805460ff191660011790559392505050565b600080610f1484846123cb565b50949350505050565b6004546001600160a01b031681565b60035460ff1681565b6000610f3f614cef565b6040518060200160405280610f52611b74565b90526001600160a01b0384166000908152600e6020526040812054919250908190610f7e908490612476565b90925090506000826003811115610f9157fe5b14610fe3576040805162461bcd60e51b815260206004820152601f60248201527f62616c616e636520636f756c64206e6f742062652063616c63756c6174656400604482015290519081900360640190fd5b949350505050565b6000610ff56124ca565b905090565b6000610c5b8261254a565b60035460009061010090046001600160a01b031633146110325761102b6001603f6125de565b9050610c72565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b15801561107757600080fd5b505afa15801561108b573d6000803e3d6000fd5b505050506040513d60208110156110a157600080fd5b50516110f4576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9392505050565b600b5481565b60035461010090046001600160a01b031633146111ae5760405162461bcd60e51b815260040180806020018281038252602d8152602001806150e3602d913960400191505060405180910390fd5b50565b6012546001600160a01b031681565b6005546001600160a01b031681565b6000805460ff16611214576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611226611609565b9050801561124c5761124481601081111561123d57fe5b60306125de565b915050610d7b565b61125583612644565b9150506000805460ff19166001179055919050565b60095481565b6011546001600160a01b031681565b6001600160a01b03166000908152600e602052604090205490565b6000805460ff166112df576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556112f1611609565b1461133c576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b50600b546000805460ff1916600117905590565b6000610c5b82612777565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f81018490048402820184019092528181529291830182828015610bec5780601f10610bc157610100808354040283529160200191610bec565b60008060006113c7846127f8565b909250905060008260038111156113da57fe5b146111535760405162461bcd60e51b8152600401808060200182810382526037815260200180614f396037913960400191505060405180910390fd5b60035461010090046001600160a01b031633146114645760405162461bcd60e51b8152600401808060200182810382526024815260200180614e486024913960400191505060405180910390fd5b6009541580156114745750600a54155b6114af5760405162461bcd60e51b8152600401808060200182810382526023815260200180614e6c6023913960400191505060405180910390fd5b6007849055836114f05760405162461bcd60e51b8152600401808060200182810382526030815260200180614e8f6030913960400191505060405180910390fd5b60006114fb87611005565b90508015611550576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b6115586128ac565b600955670de0b6b3a7640000600a55611570866128b0565b905080156115af5760405162461bcd60e51b8152600401808060200182810382526022815260200180614ebf6022913960400191505060405180910390fd5b83516115c2906001906020870190614d02565b5082516115d6906002906020860190614d02565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b600080610c6d83612a25565b6000806116146128ac565b6009549091508082141561162d57600092505050610df3565b60006116376124ca565b600b54600c54600a54600654604080516315f2405360e01b815260048101879052602481018690526044810185905290519596509394929391926000926001600160a01b03909216916315f24053916064808301926020929190829003018186803b1580156116a557600080fd5b505afa1580156116b9573d6000803e3d6000fd5b505050506040513d60208110156116cf57600080fd5b5051905065048c2739500081111561172e576040805162461bcd60e51b815260206004820152601c60248201527f626f72726f772072617465206973206162737572646c79206869676800000000604482015290519081900360640190fd5b60008061173b8989612aa6565b9092509050600082600381111561174e57fe5b146117a0576040805162461bcd60e51b815260206004820152601f60248201527f636f756c64206e6f742063616c63756c61746520626c6f636b2064656c746100604482015290519081900360640190fd5b6117a8614cef565b6000806000806117c660405180602001604052808a81525087612ac9565b909750945060008760038111156117d957fe5b1461180b576117f6600960068960038111156117f157fe5b612b31565b9e505050505050505050505050505050610df3565b611815858c612476565b9097509350600087600381111561182857fe5b14611840576117f6600960018960038111156117f157fe5b61184a848c612b97565b9097509250600087600381111561185d57fe5b14611875576117f6600960048960038111156117f157fe5b6118906040518060200160405280600854815250858c612bbd565b909750915060008760038111156118a357fe5b146118bb576117f6600960058960038111156117f157fe5b6118c6858a8b612bbd565b909750905060008760038111156118d957fe5b146118f1576117f6600960038960038111156117f157fe5b60098e9055600a819055600b839055600c829055604080518d8152602081018690528082018390526060810185905290517f4dec04e750ca11537cabcd8a9eab06494de08da3735bc8871cd41250e190bc049181900360800190a160009e50505050505050505050505050505090565b6000805460ff166119a6576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556119bc333386866120bd565b1490506000805460ff1916600117905592915050565b600a5481565b6006546000906001600160a01b031663b81688166119f46124ca565b600b54600c546008546040518563ffffffff1660e01b81526004018085815260200184815260200183815260200182815260200194505050505060206040518083038186803b158015611a4657600080fd5b505afa158015611a5a573d6000803e3d6000fd5b505050506040513d6020811015611a7057600080fd5b5051905090565b6000805460ff16611abc576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19169055611ad233858585612c19565b90506000805460ff191660011790559392505050565b60035460009061010090046001600160a01b03163314611b0e5761102b600160456125de565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a9929181900390910190a16000611153565b6000805460ff16611bb9576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611bcb611609565b14611c16576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b611c1e610d93565b90506000805460ff1916600117905590565b6001600160a01b0381166000908152600e6020526040812054819081908190818080611c5b896127f8565b935090506000816003811115611c6d57fe5b14611c8b5760095b975060009650869550859450611cbe9350505050565b611c9361200e565b925090506000816003811115611ca557fe5b14611cb1576009611c75565b5060009650919450925090505b9193509193565b6000610c5b82612e7f565b6000610c5b82612efe565b6001600160a01b039182166000908152600f6020908152604080832093909416825291909152205490565b6004546000906001600160a01b031633141580611d21575033155b15611d3957611d32600160006125de565b9050610df3565b60038054600480546001600160a01b03818116610100818102610100600160a81b0319871617968790556001600160a01b031990931690935560408051948390048216808652929095041660208401528351909391927ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc92908290030190a1600454604080516001600160a01b038085168252909216602083015280517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a99281900390910190a160009250505090565b600080611e14611609565b90508015611e3a57611e32816010811115611e2b57fe5b60406125de565b915050610c72565b611153836128b0565b6006546001600160a01b031681565b600080611e60858585612f78565b5095945050505050565b60035461010090046001600160a01b031681565b6006546000906001600160a01b03166315f24053611e9a6124ca565b600b54600c546040518463ffffffff1660e01b815260040180848152602001838152602001828152602001935050505060206040518083038186803b158015611a4657600080fd5b6000805460ff16611f27576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611f39611609565b90508015611f5757611244816010811115611f5057fe5b60466125de565b611255836130aa565b600181565b60008054819060ff16611fac576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611fbe611609565b90508015611fe957611fdc816010811115611fd557fe5b60366125de565b925060009150611ffa9050565b611ff4333386613152565b92509250505b6000805460ff191660011790559092909150565b600d54600090819080612029575050600754600091506120b9565b60006120336124ca565b9050600061203f614cef565b600061205084600b54600c54613537565b93509050600081600381111561206257fe5b14612077579550600094506120b99350505050565b6120818386613575565b92509050600081600381111561209357fe5b146120a8579550600094506120b99350505050565b50516000955093506120b992505050565b9091565b600554604080516317b9b84b60e31b81523060048201526001600160a01b03868116602483015285811660448301526064820185905291516000938493169163bdcdc25891608480830192602092919082900301818787803b15801561212257600080fd5b505af1158015612136573d6000803e3d6000fd5b505050506040513d602081101561214c57600080fd5b50519050801561216b576121636003604a83612b31565b915050610fe3565b836001600160a01b0316856001600160a01b03161415612191576121636002604b6125de565b60006001600160a01b0387811690871614156121b057506000196121d8565b506001600160a01b038086166000908152600f60209081526040808320938a16835292905220545b6000806000806121e88589612aa6565b909450925060008460038111156121fb57fe5b146122195761220c6009604b6125de565b9650505050505050610fe3565b6001600160a01b038a166000908152600e602052604090205461223c9089612aa6565b9094509150600084600381111561224f57fe5b146122605761220c6009604c6125de565b6001600160a01b0389166000908152600e60205260409020546122839089612b97565b9094509050600084600381111561229657fe5b146122a75761220c6009604d6125de565b6001600160a01b03808b166000908152600e6020526040808220859055918b1681522081905560001985146122ff576001600160a01b03808b166000908152600f60209081526040808320938f168352929052208390555b886001600160a01b03168a6001600160a01b0316600080516020614faa8339815191528a6040518082815260200191505060405180910390a36005546040805163352b4a3f60e11b81523060048201526001600160a01b038d811660248301528c81166044830152606482018c905291519190921691636a56947e91608480830192600092919082900301818387803b15801561239b57600080fd5b505af11580156123af573d6000803e3d6000fd5b50600092506123bc915050565b9b9a5050505050505050505050565b60008054819060ff16612412576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612424611609565b9050801561244f5761244281601081111561243b57fe5b60356125de565b9250600091506124609050565b61245a338686613152565b92509250505b6000805460ff1916600117905590939092509050565b6000806000612483614cef565b61248d8686612ac9565b909250905060008260038111156124a057fe5b146124b157509150600090506124c3565b60006124bc82613625565b9350935050505b9250929050565b601154604080516370a0823160e01b815230600482015290516000926001600160a01b03169182916370a0823191602480820192602092909190829003018186803b15801561251857600080fd5b505afa15801561252c573d6000803e3d6000fd5b505050506040513d602081101561254257600080fd5b505191505090565b6000805460ff1661258f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556125a1611609565b905080156125bf576112448160108111156125b857fe5b604e6125de565b6125c883613634565b509150506000805460ff19166001179055919050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa083601081111561260d57fe5b83605081111561261957fe5b604080519283526020830191909152600082820152519081900360600190a182601081111561115357fe5b600354600090819061010090046001600160a01b0316331461266c57611e32600160316125de565b6126746128ac565b6009541461268857611e32600a60336125de565b826126916124ca565b10156126a357611e32600e60326125de565b600c548311156126b957611e32600260346125de565b50600c54828103908111156126ff5760405162461bcd60e51b81526004018080602001828103825260248152602001806150bf6024913960400191505060405180910390fd5b600c81905560035461271f9061010090046001600160a01b03168461371c565b600354604080516101009092046001600160a01b0316825260208201859052818101839052517f3bad0c59cf2f06e7314077049f48a93578cd16f5ef92329f1dab1420a99c177e916060908290030190a16000611153565b6000805460ff166127bc576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556127ce611609565b905080156127ec576112448160108111156127e557fe5b60276125de565b61125533600085613813565b6001600160a01b03811660009081526010602052604081208054829182918291829161282f5750600094508493506128a792505050565b61283f8160000154600a54613cda565b9094509250600084600381111561285257fe5b146128675750919350600092506128a7915050565b612875838260010154613d19565b9094509150600084600381111561288857fe5b1461289d5750919350600092506128a7915050565b5060009450925050505b915091565b4390565b600354600090819061010090046001600160a01b031633146128d857611e32600160426125de565b6128e06128ac565b600954146128f457611e32600a60416125de565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b15801561294557600080fd5b505afa158015612959573d6000803e3d6000fd5b505050506040513d602081101561296f57600080fd5b50516129c2576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a16000611153565b60008054819060ff16612a6c576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612a7e611609565b90508015612a9c57611fdc816010811115612a9557fe5b601e6125de565b611ff43385613d44565b600080838311612abd5750600090508183036124c3565b506003905060006124c3565b6000612ad3614cef565b600080612ae4866000015186613cda565b90925090506000826003811115612af757fe5b14612b16575060408051602081019091526000815290925090506124c3565b60408051602081019091529081526000969095509350505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0846010811115612b6057fe5b846050811115612b6c57fe5b604080519283526020830191909152818101859052519081900360600190a1836010811115610fe357fe5b600080838301848110612baf576000925090506124c3565b5060029150600090506124c3565b6000806000612bca614cef565b612bd48787612ac9565b90925090506000826003811115612be757fe5b14612bf85750915060009050612c11565b612c0a612c0482613625565b86612b97565b9350935050505b935093915050565b6005546040805163d02f735160e01b81523060048201526001600160a01b038781166024830152868116604483015285811660648301526084820185905291516000938493169163d02f73519160a480830192602092919082900301818787803b158015612c8657600080fd5b505af1158015612c9a573d6000803e3d6000fd5b505050506040513d6020811015612cb057600080fd5b505190508015612cc7576121636003601b83612b31565b846001600160a01b0316846001600160a01b03161415612ced576121636006601c6125de565b6001600160a01b0384166000908152600e602052604081205481908190612d149087612aa6565b90935091506000836003811115612d2757fe5b14612d4a57612d3f6009601a8560038111156117f157fe5b945050505050610fe3565b6001600160a01b0388166000908152600e6020526040902054612d6d9087612b97565b90935090506000836003811115612d8057fe5b14612d9857612d3f600960198560038111156117f157fe5b6001600160a01b038088166000818152600e60209081526040808320879055938c168083529184902085905583518a815293519193600080516020614faa833981519152929081900390910190a360055460408051636d35bf9160e01b81523060048201526001600160a01b038c811660248301528b811660448301528a81166064830152608482018a905291519190921691636d35bf919160a480830192600092919082900301818387803b158015612e5157600080fd5b505af1158015612e65573d6000803e3d6000fd5b5060009250612e72915050565b9998505050505050505050565b6000805460ff16612ec4576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612ed6611609565b90508015612ef457611244816010811115612eed57fe5b60086125de565b61125533846141a3565b6000805460ff16612f43576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612f55611609565b90508015612f6c576112448160108111156127e557fe5b61125533846000613813565b60008054819060ff16612fbf576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612fd1611609565b90508015612ffc57612fef816010811115612fe857fe5b600f6125de565b9250600091506130939050565b836001600160a01b031663a6afed956040518163ffffffff1660e01b8152600401602060405180830381600087803b15801561303757600080fd5b505af115801561304b573d6000803e3d6000fd5b505050506040513d602081101561306157600080fd5b50519050801561308157612fef81601081111561307a57fe5b60106125de565b61308d338787876144b1565b92509250505b6000805460ff191660011790559094909350915050565b60035460009061010090046001600160a01b031633146130d05761102b600160476125de565b6130d86128ac565b600954146130ec5761102b600a60486125de565b670de0b6b3a76400008211156131085761102b600260496125de565b6008805490839055604080518281526020810185905281517faaa68312e2ea9d50e16af5068410ab56e1a1fd06037b1a35664812c30f821460929181900390910190a16000611153565b60055460408051631200453160e11b81523060048201526001600160a01b0386811660248301528581166044830152606482018590529151600093849384939116916324008a629160848082019260209290919082900301818787803b1580156131bb57600080fd5b505af11580156131cf573d6000803e3d6000fd5b505050506040513d60208110156131e557600080fd5b505190508015613209576131fc6003603883612b31565b925060009150612c119050565b6132116128ac565b60095414613225576131fc600a60396125de565b61322d614d80565b6001600160a01b0386166000908152601060205260409020600101546060820152613257866127f8565b608083018190526020830182600381111561326e57fe5b600381111561327957fe5b905250600090508160200151600381111561329057fe5b146132ba576132ac60096037836020015160038111156117f157fe5b935060009250612c11915050565b6000198514156132d357608081015160408201526132db565b604081018590525b6132e9878260400151614a34565b60e0820181905260808201516132fe91612aa6565b60a083018190526020830182600381111561331557fe5b600381111561332057fe5b905250600090508160200151600381111561333757fe5b146133735760405162461bcd60e51b815260040180806020018281038252603a815260200180614f70603a913960400191505060405180910390fd5b613383600b548260e00151612aa6565b60c083018190526020830182600381111561339a57fe5b60038111156133a557fe5b90525060009050816020015160038111156133bc57fe5b146133f85760405162461bcd60e51b8152600401808060200182810382526031815260200180614fca6031913960400191505060405180910390fd5b60a080820180516001600160a01b03808a16600081815260106020908152604091829020948555600a5460019095019490945560c0870151600b81905560e088015195518251948f16855294840192909252828101949094526060820192909252608081019190915290517f1a2a22cb034d26d1854bdc6666a5b91fe25efbbb5dcad3b0355478d6f5c362a1929181900390910190a160055460e0820151606083015160408051631ededc9160e01b81523060048201526001600160a01b038c811660248301528b8116604483015260648201949094526084810192909252519190921691631ededc919160a480830192600092919082900301818387803b15801561350357600080fd5b505af1158015613517573d6000803e3d6000fd5b5060009250613524915050565b8160e00151935093505050935093915050565b6000806000806135478787612b97565b9092509050600082600381111561355a57fe5b1461356b5750915060009050612c11565b612c0a8186612aa6565b600061357f614cef565b60008061359486670de0b6b3a7640000613cda565b909250905060008260038111156135a757fe5b146135c6575060408051602081019091526000815290925090506124c3565b6000806135d38388613d19565b909250905060008260038111156135e657fe5b14613608575060408051602081019091526000815290945092506124c3915050565b604080516020810190915290815260009890975095505050505050565b51670de0b6b3a7640000900490565b6000806000806136426128ac565b6009541461366157613656600a604f6125de565b935091506128a79050565b61366b3386614a34565b905080600c54019150600c548210156136cb576040805162461bcd60e51b815260206004820181905260248201527f61646420726573657276657320756e6578706563746564206f766572666c6f77604482015290519081900360640190fd5b600c829055604080513381526020810183905280820184905290517fa91e67c5ea634cd43a12c5a482724b03de01e85ca68702a53d0c2f45cb7c1dc59181900360600190a160009350915050915091565b6011546040805163a9059cbb60e01b81526001600160a01b0385811660048301526024820185905291519190921691829163a9059cbb9160448082019260009290919082900301818387803b15801561377457600080fd5b505af1158015613788573d6000803e3d6000fd5b5050505060003d600081146137a457602081146137ae57600080fd5b60001991506137ba565b60206000803e60005191505b508061380d576040805162461bcd60e51b815260206004820152601960248201527f544f4b454e5f5452414e534645525f4f55545f4641494c454400000000000000604482015290519081900360640190fd5b50505050565b6000821580613820575081155b61385b5760405162461bcd60e51b815260040180806020018281038252603481526020018061508b6034913960400191505060405180910390fd5b613863614dc6565b61386b61200e565b604083018190526020830182600381111561388257fe5b600381111561388d57fe5b90525060009050816020015160038111156138a457fe5b146138c8576138c06009602b836020015160038111156117f157fe5b915050611153565b83156139495760608101849052604080516020810182529082015181526138ef9085612476565b608083018190526020830182600381111561390657fe5b600381111561391157fe5b905250600090508160200151600381111561392857fe5b14613944576138c060096029836020015160038111156117f157fe5b6139c2565b6139658360405180602001604052808460400151815250614c7e565b606083018190526020830182600381111561397c57fe5b600381111561398757fe5b905250600090508160200151600381111561399e57fe5b146139ba576138c06009602a836020015160038111156117f157fe5b608081018390525b60055460608201516040805163eabe7d9160e01b81523060048201526001600160a01b03898116602483015260448201939093529051600093929092169163eabe7d919160648082019260209290919082900301818787803b158015613a2757600080fd5b505af1158015613a3b573d6000803e3d6000fd5b505050506040513d6020811015613a5157600080fd5b505190508015613a7157613a686003602883612b31565b92505050611153565b613a796128ac565b60095414613a8d57613a68600a602c6125de565b613a9d600d548360600151612aa6565b60a0840181905260208401826003811115613ab457fe5b6003811115613abf57fe5b9052506000905082602001516003811115613ad657fe5b14613af257613a686009602e846020015160038111156117f157fe5b6001600160a01b0386166000908152600e60205260409020546060830151613b1a9190612aa6565b60c0840181905260208401826003811115613b3157fe5b6003811115613b3c57fe5b9052506000905082602001516003811115613b5357fe5b14613b6f57613a686009602d846020015160038111156117f157fe5b8160800151613b7c6124ca565b1015613b8e57613a68600e602f6125de565b613b9c86836080015161371c565b60a0820151600d5560c08201516001600160a01b0387166000818152600e6020908152604091829020939093556060850151815190815290513093600080516020614faa833981519152928290030190a36080820151606080840151604080516001600160a01b038b168152602081019490945283810191909152517fe5b754fb1abb7f01b499791d0b820ae3b6af3424ac1c59768edb53f4ec31a9299281900390910190a160055460808301516060840151604080516351dff98960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916351dff98991608480830192600092919082900301818387803b158015613caf57600080fd5b505af1158015613cc3573d6000803e3d6000fd5b5060009250613cd0915050565b9695505050505050565b60008083613ced575060009050806124c3565b83830283858281613cfa57fe5b0414613d0e575060029150600090506124c3565b6000925090506124c3565b60008082613d2d57506001905060006124c3565b6000838581613d3857fe5b04915091509250929050565b60055460408051634ef4c3e160e01b81523060048201526001600160a01b03858116602483015260448201859052915160009384938493911691634ef4c3e19160648082019260209290919082900301818787803b158015613da557600080fd5b505af1158015613db9573d6000803e3d6000fd5b505050506040513d6020811015613dcf57600080fd5b505190508015613df357613de66003601f83612b31565b9250600091506124c39050565b613dfb6128ac565b60095414613e0f57613de6600a60226125de565b613e17614dc6565b613e1f61200e565b6040830181905260208301826003811115613e3657fe5b6003811115613e4157fe5b9052506000905081602001516003811115613e5857fe5b14613e8257613e7460096021836020015160038111156117f157fe5b9350600092506124c3915050565b613e8c8686614a34565b60c0820181905260408051602081018252908301518152613ead9190614c7e565b6060830181905260208301826003811115613ec457fe5b6003811115613ecf57fe5b9052506000905081602001516003811115613ee657fe5b14613f38576040805162461bcd60e51b815260206004820181905260248201527f4d494e545f45584348414e47455f43414c43554c4154494f4e5f4641494c4544604482015290519081900360640190fd5b613f48600d548260600151612b97565b6080830181905260208301826003811115613f5f57fe5b6003811115613f6a57fe5b9052506000905081602001516003811115613f8157fe5b14613fbd5760405162461bcd60e51b81526004018080602001828103825260288152602001806150636028913960400191505060405180910390fd5b6001600160a01b0386166000908152600e60205260409020546060820151613fe59190612b97565b60a0830181905260208301826003811115613ffc57fe5b600381111561400757fe5b905250600090508160200151600381111561401e57fe5b1461405a5760405162461bcd60e51b815260040180806020018281038252602b815260200180614f0e602b913960400191505060405180910390fd5b6080810151600d5560a08101516001600160a01b0387166000818152600e60209081526040918290209390935560c084015160608086015183519485529484019190915282820193909352517f4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f929181900390910190a1606081015160408051918252516001600160a01b038816913091600080516020614faa8339815191529181900360200190a360055460c08201516060830151604080516341c728b960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916341c728b991608480830192600092919082900301818387803b15801561417057600080fd5b505af1158015614184573d6000803e3d6000fd5b5060009250614191915050565b8160c001519350935050509250929050565b6005546040805163368f515360e21b81523060048201526001600160a01b0385811660248301526044820185905291516000938493169163da3d454c91606480830192602092919082900301818787803b15801561420057600080fd5b505af1158015614214573d6000803e3d6000fd5b505050506040513d602081101561422a57600080fd5b505190508015614249576142416003600e83612b31565b915050610c5b565b6142516128ac565b6009541461426457614241600a806125de565b8261426d6124ca565b101561427f57614241600e60096125de565b614287614e04565b614290856127f8565b60208301819052828260038111156142a457fe5b60038111156142af57fe5b90525060009050815160038111156142c357fe5b146142e8576142df60096007836000015160038111156117f157fe5b92505050610c5b565b6142f6816020015185612b97565b604083018190528282600381111561430a57fe5b600381111561431557fe5b905250600090508151600381111561432957fe5b14614345576142df6009600c836000015160038111156117f157fe5b614351600b5485612b97565b606083018190528282600381111561436557fe5b600381111561437057fe5b905250600090508151600381111561438457fe5b146143a0576142df6009600b836000015160038111156117f157fe5b6143aa858561371c565b604080820180516001600160a01b03881660008181526010602090815290859020928355600a54600190930192909255606080860151600b81905593518551928352928201899052818501929092529081019190915290517f13ed6866d4e1ee6da46f845c46d7e54120883d75c5ea9a2dacc1c4ca8984ab809181900360800190a160055460408051635c77860560e01b81523060048201526001600160a01b0388811660248301526044820188905291519190921691635c77860591606480830192600092919082900301818387803b15801561448757600080fd5b505af115801561449b573d6000803e3d6000fd5b50600092506144a8915050565b95945050505050565b60055460408051632fe3f38f60e11b81523060048201526001600160a01b0384811660248301528781166044830152868116606483015260848201869052915160009384938493911691635fc7e71e9160a48082019260209290919082900301818787803b15801561452257600080fd5b505af1158015614536573d6000803e3d6000fd5b505050506040513d602081101561454c57600080fd5b505190508015614570576145636003601283612b31565b925060009150614a2b9050565b6145786128ac565b6009541461458c57614563600a60166125de565b6145946128ac565b846001600160a01b0316636c540baf6040518163ffffffff1660e01b815260040160206040518083038186803b1580156145cd57600080fd5b505afa1580156145e1573d6000803e3d6000fd5b505050506040513d60208110156145f757600080fd5b50511461460a57614563600a60116125de565b866001600160a01b0316866001600160a01b0316141561463057614563600660176125de565b8461464157614563600760156125de565b60001985141561465757614563600760146125de565b600080614665898989613152565b909250905081156146955761468682601081111561467f57fe5b60186125de565b945060009350614a2b92505050565b6005546040805163c488847b60e01b81523060048201526001600160a01b038981166024830152604482018590528251600094859492169263c488847b926064808301939192829003018186803b1580156146ef57600080fd5b505afa158015614703573d6000803e3d6000fd5b505050506040513d604081101561471957600080fd5b508051602090910151909250905081156147645760405162461bcd60e51b8152600401808060200182810382526033815260200180614ffb6033913960400191505060405180910390fd5b80886001600160a01b03166370a082318c6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b1580156147bb57600080fd5b505afa1580156147cf573d6000803e3d6000fd5b505050506040513d60208110156147e557600080fd5b5051101561483a576040805162461bcd60e51b815260206004820152601860248201527f4c49515549444154455f5345495a455f544f4f5f4d5543480000000000000000604482015290519081900360640190fd5b60006001600160a01b03891630141561486057614859308d8d85612c19565b90506148ea565b6040805163b2a02ff160e01b81526001600160a01b038e811660048301528d81166024830152604482018590529151918b169163b2a02ff1916064808201926020929091908290030181600087803b1580156148bb57600080fd5b505af11580156148cf573d6000803e3d6000fd5b505050506040513d60208110156148e557600080fd5b505190505b8015614934576040805162461bcd60e51b81526020600482015260146024820152731d1bdad95b881cd95a5e9d5c994819985a5b195960621b604482015290519081900360640190fd5b604080516001600160a01b03808f168252808e1660208301528183018790528b1660608201526080810184905290517f298637f684da70674f26509b10f07ec2fbc77a335ab1e7d6215a4b2484d8bb529181900360a00190a1600554604080516347ef3b3b60e01b81523060048201526001600160a01b038c811660248301528f811660448301528e811660648301526084820188905260a48201869052915191909216916347ef3b3b9160c480830192600092919082900301818387803b1580156149ff57600080fd5b505af1158015614a13573d6000803e3d6000fd5b5060009250614a20915050565b975092955050505050505b94509492505050565b601154604080516370a0823160e01b815230600482015290516000926001600160a01b031691839183916370a08231916024808301926020929190829003018186803b158015614a8357600080fd5b505afa158015614a97573d6000803e3d6000fd5b505050506040513d6020811015614aad57600080fd5b5051604080516323b872dd60e01b81526001600160a01b038881166004830152306024830152604482018890529151929350908416916323b872dd9160648082019260009290919082900301818387803b158015614b0a57600080fd5b505af1158015614b1e573d6000803e3d6000fd5b5050505060003d60008114614b3a5760208114614b4457600080fd5b6000199150614b50565b60206000803e60005191505b5080614ba3576040805162461bcd60e51b815260206004820152601860248201527f544f4b454e5f5452414e534645525f494e5f4641494c45440000000000000000604482015290519081900360640190fd5b601154604080516370a0823160e01b815230600482015290516000926001600160a01b0316916370a08231916024808301926020929190829003018186803b158015614bee57600080fd5b505afa158015614c02573d6000803e3d6000fd5b505050506040513d6020811015614c1857600080fd5b5051905082811015614c71576040805162461bcd60e51b815260206004820152601a60248201527f544f4b454e5f5452414e534645525f494e5f4f564552464c4f57000000000000604482015290519081900360640190fd5b9190910395945050505050565b6000806000614c8b614cef565b61248d86866000614c9a614cef565b600080614caf670de0b6b3a764000087613cda565b90925090506000826003811115614cc257fe5b14614ce1575060408051602081019091526000815290925090506124c3565b6124bc818660000151613575565b6040518060200160405280600081525090565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f10614d4357805160ff1916838001178555614d70565b82800160010185558215614d70579182015b82811115614d70578251825591602001919060010190614d55565b50614d7c929150614e2d565b5090565b6040805161010081019091528060008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040805160e0810190915280600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604080516080810190915280600081526020016000815260200160008152602001600081525090565b610df391905b80821115614d7c5760008155600101614e3356fe6f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e6365696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c65646f6e6c79207468652061646d696e206d61792063616c6c205f72657369676e496d706c656d656e746174696f6e4d494e545f4e45575f4143434f554e545f42414c414e43455f43414c43554c4154494f4e5f4641494c4544626f72726f7742616c616e636553746f7265643a20626f72726f7742616c616e636553746f726564496e7465726e616c206661696c656452455041595f424f52524f575f4e45575f4143434f554e545f424f52524f575f42414c414e43455f43414c43554c4154494f4e5f4641494c4544ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef52455041595f424f52524f575f4e45575f544f54414c5f42414c414e43455f43414c43554c4154494f4e5f4641494c45444c49515549444154455f434f4d5054524f4c4c45525f43414c43554c4154455f414d4f554e545f5345495a455f4641494c454465786368616e67655261746553746f7265643a2065786368616e67655261746553746f726564496e7465726e616c206661696c65644d494e545f4e45575f544f54414c5f535550504c595f43414c43554c4154494f4e5f4641494c45446f6e65206f662072656465656d546f6b656e73496e206f722072656465656d416d6f756e74496e206d757374206265207a65726f72656475636520726573657276657320756e657870656374656420756e646572666c6f776f6e6c79207468652061646d696e206d61792063616c6c205f6265636f6d65496d706c656d656e746174696f6ea265627a7a72315820ebe008737c0e4ccf7764fafbed905dafd05f03fda148ed3be1a13bb43fdcaccf64736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CErc20Delegate<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CErc20Delegate<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CErc20Delegate<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CErc20Delegate))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CErc20Delegate<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CERC20DELEGATE_ABI.clone(), client)
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
                CERC20DELEGATE_ABI.clone(),
                CERC20DELEGATE_BYTECODE.clone().into(),
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CErc20DelegateEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CErc20Delegate<M> {
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
    pub enum CErc20DelegateEvents {
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
    impl ethers::contract::EthLogDecode for CErc20DelegateEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::NewMarketInterestRateModelFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CErc20DelegateEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CErc20DelegateEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20DelegateEvents::AccrueInterestFilter(element) => element.fmt(f),
                CErc20DelegateEvents::ApprovalFilter(element) => element.fmt(f),
                CErc20DelegateEvents::BorrowFilter(element) => element.fmt(f),
                CErc20DelegateEvents::FailureFilter(element) => element.fmt(f),
                CErc20DelegateEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CErc20DelegateEvents::MintFilter(element) => element.fmt(f),
                CErc20DelegateEvents::NewAdminFilter(element) => element.fmt(f),
                CErc20DelegateEvents::NewComptrollerFilter(element) => element.fmt(f),
                CErc20DelegateEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CErc20DelegateEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CErc20DelegateEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CErc20DelegateEvents::RedeemFilter(element) => element.fmt(f),
                CErc20DelegateEvents::RepayBorrowFilter(element) => element.fmt(f),
                CErc20DelegateEvents::ReservesAddedFilter(element) => element.fmt(f),
                CErc20DelegateEvents::ReservesReducedFilter(element) => element.fmt(f),
                CErc20DelegateEvents::TransferFilter(element) => element.fmt(f),
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CErc20DelegateCalls {
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
    }
    impl ethers::core::abi::AbiDecode for CErc20DelegateCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::AddReserves(decoded));
            }
            if let Ok(decoded) =
                <BecomeImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BecomeImplementation(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <ResignImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::ResignImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InitializeWithUnderlyingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CErc20DelegateCalls::InitializeWithUnderlying(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::IsCToken(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20DelegateCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20DelegateCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegateCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CErc20DelegateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CErc20DelegateCalls::AcceptAdmin(element) => element.encode(),
                CErc20DelegateCalls::AddReserves(element) => element.encode(),
                CErc20DelegateCalls::BecomeImplementation(element) => element.encode(),
                CErc20DelegateCalls::ReduceReserves(element) => element.encode(),
                CErc20DelegateCalls::ResignImplementation(element) => element.encode(),
                CErc20DelegateCalls::SetComptroller(element) => element.encode(),
                CErc20DelegateCalls::SetInterestRateModel(element) => element.encode(),
                CErc20DelegateCalls::SetPendingAdmin(element) => element.encode(),
                CErc20DelegateCalls::SetReserveFactor(element) => element.encode(),
                CErc20DelegateCalls::AccrualBlockNumber(element) => element.encode(),
                CErc20DelegateCalls::AccrueInterest(element) => element.encode(),
                CErc20DelegateCalls::Admin(element) => element.encode(),
                CErc20DelegateCalls::Allowance(element) => element.encode(),
                CErc20DelegateCalls::Approve(element) => element.encode(),
                CErc20DelegateCalls::BalanceOf(element) => element.encode(),
                CErc20DelegateCalls::BalanceOfUnderlying(element) => element.encode(),
                CErc20DelegateCalls::Borrow(element) => element.encode(),
                CErc20DelegateCalls::BorrowBalanceCurrent(element) => element.encode(),
                CErc20DelegateCalls::BorrowBalanceStored(element) => element.encode(),
                CErc20DelegateCalls::BorrowIndex(element) => element.encode(),
                CErc20DelegateCalls::BorrowRatePerBlock(element) => element.encode(),
                CErc20DelegateCalls::Comptroller(element) => element.encode(),
                CErc20DelegateCalls::Decimals(element) => element.encode(),
                CErc20DelegateCalls::ExchangeRateCurrent(element) => element.encode(),
                CErc20DelegateCalls::ExchangeRateStored(element) => element.encode(),
                CErc20DelegateCalls::GetAccountSnapshot(element) => element.encode(),
                CErc20DelegateCalls::GetCash(element) => element.encode(),
                CErc20DelegateCalls::Implementation(element) => element.encode(),
                CErc20DelegateCalls::InitializeWithUnderlying(element) => element.encode(),
                CErc20DelegateCalls::Initialize(element) => element.encode(),
                CErc20DelegateCalls::InterestRateModel(element) => element.encode(),
                CErc20DelegateCalls::IsCToken(element) => element.encode(),
                CErc20DelegateCalls::LiquidateBorrow(element) => element.encode(),
                CErc20DelegateCalls::Mint(element) => element.encode(),
                CErc20DelegateCalls::Name(element) => element.encode(),
                CErc20DelegateCalls::PendingAdmin(element) => element.encode(),
                CErc20DelegateCalls::Redeem(element) => element.encode(),
                CErc20DelegateCalls::RedeemUnderlying(element) => element.encode(),
                CErc20DelegateCalls::RepayBorrow(element) => element.encode(),
                CErc20DelegateCalls::RepayBorrowBehalf(element) => element.encode(),
                CErc20DelegateCalls::ReserveFactorMantissa(element) => element.encode(),
                CErc20DelegateCalls::Seize(element) => element.encode(),
                CErc20DelegateCalls::SupplyRatePerBlock(element) => element.encode(),
                CErc20DelegateCalls::Symbol(element) => element.encode(),
                CErc20DelegateCalls::TotalBorrows(element) => element.encode(),
                CErc20DelegateCalls::TotalBorrowsCurrent(element) => element.encode(),
                CErc20DelegateCalls::TotalReserves(element) => element.encode(),
                CErc20DelegateCalls::TotalSupply(element) => element.encode(),
                CErc20DelegateCalls::Transfer(element) => element.encode(),
                CErc20DelegateCalls::TransferFrom(element) => element.encode(),
                CErc20DelegateCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CErc20DelegateCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20DelegateCalls::AcceptAdmin(element) => element.fmt(f),
                CErc20DelegateCalls::AddReserves(element) => element.fmt(f),
                CErc20DelegateCalls::BecomeImplementation(element) => element.fmt(f),
                CErc20DelegateCalls::ReduceReserves(element) => element.fmt(f),
                CErc20DelegateCalls::ResignImplementation(element) => element.fmt(f),
                CErc20DelegateCalls::SetComptroller(element) => element.fmt(f),
                CErc20DelegateCalls::SetInterestRateModel(element) => element.fmt(f),
                CErc20DelegateCalls::SetPendingAdmin(element) => element.fmt(f),
                CErc20DelegateCalls::SetReserveFactor(element) => element.fmt(f),
                CErc20DelegateCalls::AccrualBlockNumber(element) => element.fmt(f),
                CErc20DelegateCalls::AccrueInterest(element) => element.fmt(f),
                CErc20DelegateCalls::Admin(element) => element.fmt(f),
                CErc20DelegateCalls::Allowance(element) => element.fmt(f),
                CErc20DelegateCalls::Approve(element) => element.fmt(f),
                CErc20DelegateCalls::BalanceOf(element) => element.fmt(f),
                CErc20DelegateCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CErc20DelegateCalls::Borrow(element) => element.fmt(f),
                CErc20DelegateCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CErc20DelegateCalls::BorrowBalanceStored(element) => element.fmt(f),
                CErc20DelegateCalls::BorrowIndex(element) => element.fmt(f),
                CErc20DelegateCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CErc20DelegateCalls::Comptroller(element) => element.fmt(f),
                CErc20DelegateCalls::Decimals(element) => element.fmt(f),
                CErc20DelegateCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CErc20DelegateCalls::ExchangeRateStored(element) => element.fmt(f),
                CErc20DelegateCalls::GetAccountSnapshot(element) => element.fmt(f),
                CErc20DelegateCalls::GetCash(element) => element.fmt(f),
                CErc20DelegateCalls::Implementation(element) => element.fmt(f),
                CErc20DelegateCalls::InitializeWithUnderlying(element) => element.fmt(f),
                CErc20DelegateCalls::Initialize(element) => element.fmt(f),
                CErc20DelegateCalls::InterestRateModel(element) => element.fmt(f),
                CErc20DelegateCalls::IsCToken(element) => element.fmt(f),
                CErc20DelegateCalls::LiquidateBorrow(element) => element.fmt(f),
                CErc20DelegateCalls::Mint(element) => element.fmt(f),
                CErc20DelegateCalls::Name(element) => element.fmt(f),
                CErc20DelegateCalls::PendingAdmin(element) => element.fmt(f),
                CErc20DelegateCalls::Redeem(element) => element.fmt(f),
                CErc20DelegateCalls::RedeemUnderlying(element) => element.fmt(f),
                CErc20DelegateCalls::RepayBorrow(element) => element.fmt(f),
                CErc20DelegateCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CErc20DelegateCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CErc20DelegateCalls::Seize(element) => element.fmt(f),
                CErc20DelegateCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CErc20DelegateCalls::Symbol(element) => element.fmt(f),
                CErc20DelegateCalls::TotalBorrows(element) => element.fmt(f),
                CErc20DelegateCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CErc20DelegateCalls::TotalReserves(element) => element.fmt(f),
                CErc20DelegateCalls::TotalSupply(element) => element.fmt(f),
                CErc20DelegateCalls::Transfer(element) => element.fmt(f),
                CErc20DelegateCalls::TransferFrom(element) => element.fmt(f),
                CErc20DelegateCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CErc20DelegateCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CErc20DelegateCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<AddReservesCall> for CErc20DelegateCalls {
        fn from(var: AddReservesCall) -> Self {
            CErc20DelegateCalls::AddReserves(var)
        }
    }
    impl ::std::convert::From<BecomeImplementationCall> for CErc20DelegateCalls {
        fn from(var: BecomeImplementationCall) -> Self {
            CErc20DelegateCalls::BecomeImplementation(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CErc20DelegateCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CErc20DelegateCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<ResignImplementationCall> for CErc20DelegateCalls {
        fn from(var: ResignImplementationCall) -> Self {
            CErc20DelegateCalls::ResignImplementation(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CErc20DelegateCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CErc20DelegateCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CErc20DelegateCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CErc20DelegateCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CErc20DelegateCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CErc20DelegateCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CErc20DelegateCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CErc20DelegateCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CErc20DelegateCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CErc20DelegateCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CErc20DelegateCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CErc20DelegateCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CErc20DelegateCalls {
        fn from(var: AdminCall) -> Self {
            CErc20DelegateCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CErc20DelegateCalls {
        fn from(var: AllowanceCall) -> Self {
            CErc20DelegateCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CErc20DelegateCalls {
        fn from(var: ApproveCall) -> Self {
            CErc20DelegateCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CErc20DelegateCalls {
        fn from(var: BalanceOfCall) -> Self {
            CErc20DelegateCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CErc20DelegateCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CErc20DelegateCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CErc20DelegateCalls {
        fn from(var: BorrowCall) -> Self {
            CErc20DelegateCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CErc20DelegateCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CErc20DelegateCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CErc20DelegateCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CErc20DelegateCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CErc20DelegateCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CErc20DelegateCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CErc20DelegateCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CErc20DelegateCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CErc20DelegateCalls {
        fn from(var: ComptrollerCall) -> Self {
            CErc20DelegateCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CErc20DelegateCalls {
        fn from(var: DecimalsCall) -> Self {
            CErc20DelegateCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CErc20DelegateCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CErc20DelegateCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CErc20DelegateCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CErc20DelegateCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CErc20DelegateCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CErc20DelegateCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CErc20DelegateCalls {
        fn from(var: GetCashCall) -> Self {
            CErc20DelegateCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for CErc20DelegateCalls {
        fn from(var: ImplementationCall) -> Self {
            CErc20DelegateCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InitializeWithUnderlyingCall> for CErc20DelegateCalls {
        fn from(var: InitializeWithUnderlyingCall) -> Self {
            CErc20DelegateCalls::InitializeWithUnderlying(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CErc20DelegateCalls {
        fn from(var: InitializeCall) -> Self {
            CErc20DelegateCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CErc20DelegateCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CErc20DelegateCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CErc20DelegateCalls {
        fn from(var: IsCTokenCall) -> Self {
            CErc20DelegateCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CErc20DelegateCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CErc20DelegateCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CErc20DelegateCalls {
        fn from(var: MintCall) -> Self {
            CErc20DelegateCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CErc20DelegateCalls {
        fn from(var: NameCall) -> Self {
            CErc20DelegateCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CErc20DelegateCalls {
        fn from(var: PendingAdminCall) -> Self {
            CErc20DelegateCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CErc20DelegateCalls {
        fn from(var: RedeemCall) -> Self {
            CErc20DelegateCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CErc20DelegateCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CErc20DelegateCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CErc20DelegateCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CErc20DelegateCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CErc20DelegateCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CErc20DelegateCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CErc20DelegateCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CErc20DelegateCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CErc20DelegateCalls {
        fn from(var: SeizeCall) -> Self {
            CErc20DelegateCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CErc20DelegateCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CErc20DelegateCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CErc20DelegateCalls {
        fn from(var: SymbolCall) -> Self {
            CErc20DelegateCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CErc20DelegateCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CErc20DelegateCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CErc20DelegateCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CErc20DelegateCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CErc20DelegateCalls {
        fn from(var: TotalReservesCall) -> Self {
            CErc20DelegateCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CErc20DelegateCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CErc20DelegateCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CErc20DelegateCalls {
        fn from(var: TransferCall) -> Self {
            CErc20DelegateCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CErc20DelegateCalls {
        fn from(var: TransferFromCall) -> Self {
            CErc20DelegateCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for CErc20DelegateCalls {
        fn from(var: UnderlyingCall) -> Self {
            CErc20DelegateCalls::Underlying(var)
        }
    }
}
