pub use c_erc_20_immutable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod c_erc_20_immutable {
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
    #[doc = "CErc20Immutable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CERC20IMMUTABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract CTokenInterface\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CERC20IMMUTABLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620058eb380380620058eb83398181016040526101008110156200003857600080fd5b81516020830151604080850151606086015160808701805193519597949692959194919392820192846401000000008211156200007457600080fd5b9083019060208201858111156200008a57600080fd5b8251640100000000811182820188101715620000a557600080fd5b82525081516020918201929091019080838360005b83811015620000d4578181015183820152602001620000ba565b50505050905090810190601f168015620001025780820380516001836020036101000a031916815260200191505b50604052602001805160405193929190846401000000008211156200012657600080fd5b9083019060208201858111156200013c57600080fd5b82516401000000008111828201881017156200015757600080fd5b82525081516020918201929091019080838360005b83811015620001865781810151838201526020016200016c565b50505050905090810190601f168015620001b45780820380516001836020036101000a031916815260200191505b506040908152602082015191015160038054610100600160a81b03191633610100021790559092509050620001ef8888888888888862000223565b600380546001600160a01b0390921661010002610100600160a81b0319909216919091179055506200090d95505050505050565b6200023e868686868686620002d260201b620012611760201c565b601180546001600160a01b0319166001600160a01b038981169190911791829055604080516318160ddd60e01b8152905192909116916318160ddd91600480820192602092909190829003018186803b1580156200029b57600080fd5b505afa158015620002b0573d6000803e3d6000fd5b505050506040513d6020811015620002c757600080fd5b505050505050505050565b60035461010090046001600160a01b03163314620003225760405162461bcd60e51b8152600401808060200182810382526024815260200180620058526024913960400191505060405180910390fd5b600954158015620003335750600a54155b620003705760405162461bcd60e51b8152600401808060200182810382526023815260200180620058766023913960400191505060405180910390fd5b600784905583620003b35760405162461bcd60e51b8152600401808060200182810382526030815260200180620058996030913960400191505060405180910390fd5b6000620003c9876001600160e01b03620004e816565b905080156200041f576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b620004326001600160e01b036200065016565b600955670de0b6b3a7640000600a5562000455866001600160e01b036200065516565b90508015620004965760405162461bcd60e51b8152600401808060200182810382526022815260200180620058c96022913960400191505060405180910390fd5b8351620004ab9060019060208701906200086b565b508251620004c19060029060208601906200086b565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b60035460009061010090046001600160a01b0316331462000522576200051a6001603f6001600160e01b03620007fb16565b90506200064b565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b1580156200056857600080fd5b505afa1580156200057d573d6000803e3d6000fd5b505050506040513d60208110156200059457600080fd5b5051620005e8576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9150505b919050565b435b90565b600354600090819061010090046001600160a01b03163314620006925762000689600160426001600160e01b03620007fb16565b9150506200064b565b620006a56001600160e01b036200065016565b60095414620006c55762000689600a60416001600160e01b03620007fb16565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b1580156200071757600080fd5b505afa1580156200072c573d6000803e3d6000fd5b505050506040513d60208110156200074357600080fd5b505162000797576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a1600062000647565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa08360108111156200082b57fe5b8360508111156200083857fe5b604080519283526020830191909152600082820152519081900360600190a18260108111156200086457fe5b9392505050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f10620008ae57805160ff1916838001178555620008de565b82800160010185558215620008de579182015b82811115620008de578251825591602001919060010190620008c1565b50620008ec929150620008f0565b5090565b6200065291905b80821115620008ec5760008155600101620008f7565b614f35806200091d6000396000f3fe608060405234801561001057600080fd5b50600436106102a05760003560e01c80638f840ddd11610167578063c37f68e2116100ce578063f3fdb15a11610087578063f3fdb15a146109ef578063f5e3c462146109f7578063f851a44014610a2d578063f8f9da2814610a35578063fca7820b14610a3d578063fe9c44ae14610a5a576102a0565b8063c37f68e21461090d578063c5ebeaec14610959578063db006a7514610976578063dd62ed3e14610993578063e9c714f2146109c1578063f2b3abbd146109c9576102a0565b8063a9059cbb11610120578063a9059cbb1461086d578063aa5af0fd14610899578063ae9d70b0146108a1578063b2a02ff1146108a9578063b71d1a0c146108df578063bd6d894d14610905576102a0565b80638f840ddd146106c457806395d89b41146106cc57806395dd9193146106d457806399d8c1b4146106fa578063a0712d6814610848578063a6afed9514610865576102a0565b80633af9e6691161020b578063601a0bf1116101c4578063601a0bf11461064c5780636c540baf146106695780636f307dc31461067157806370a082311461067957806373acee981461069f578063852a12e3146106a7576102a0565b80633af9e669146105cb5780633b1d21a2146105f15780633e941010146105f95780634576b5db1461061657806347bd37181461063c5780635fe3b56714610644576102a0565b8063182df0f51161025d578063182df0f5146103c75780631a31d465146103cf57806323b872dd146105275780632608f8181461055d5780632678224714610589578063313ce567146105ad576102a0565b806306fdde03146102a5578063095ea7b3146103225780630e75270214610362578063173b99041461039157806317bfdfbc1461039957806318160ddd146103bf575b600080fd5b6102ad610a62565b6040805160208082528351818301528351919283929083019185019080838360005b838110156102e75781810151838201526020016102cf565b50505050905090810190601f1680156103145780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b61034e6004803603604081101561033857600080fd5b506001600160a01b038135169060200135610aef565b604080519115158252519081900360200190f35b61037f6004803603602081101561037857600080fd5b5035610b5c565b60408051918252519081900360200190f35b61037f610b72565b61037f600480360360208110156103af57600080fd5b50356001600160a01b0316610b78565b61037f610c38565b61037f610c3e565b610525600480360360e08110156103e557600080fd5b6001600160a01b03823581169260208101358216926040820135909216916060820135919081019060a081016080820135600160201b81111561042757600080fd5b82018360208201111561043957600080fd5b803590602001918460018302840111600160201b8311171561045a57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b8111156104ac57600080fd5b8201836020820111156104be57600080fd5b803590602001918460018302840111600160201b831117156104df57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff169150610ca19050565b005b61034e6004803603606081101561053d57600080fd5b506001600160a01b03813581169160208101359091169060400135610d40565b61037f6004803603604081101561057357600080fd5b506001600160a01b038135169060200135610db2565b610591610dc8565b604080516001600160a01b039092168252519081900360200190f35b6105b5610dd7565b6040805160ff9092168252519081900360200190f35b61037f600480360360208110156105e157600080fd5b50356001600160a01b0316610de0565b61037f610e96565b61037f6004803603602081101561060f57600080fd5b5035610ea5565b61037f6004803603602081101561062c57600080fd5b50356001600160a01b0316610eb0565b61037f611005565b61059161100b565b61037f6004803603602081101561066257600080fd5b503561101a565b61037f6110b5565b6105916110bb565b61037f6004803603602081101561068f57600080fd5b50356001600160a01b03166110ca565b61037f6110e5565b61037f600480360360208110156106bd57600080fd5b503561119b565b61037f6111a6565b6102ad6111ac565b61037f600480360360208110156106ea57600080fd5b50356001600160a01b0316611204565b610525600480360360c081101561071057600080fd5b6001600160a01b03823581169260208101359091169160408201359190810190608081016060820135600160201b81111561074a57600080fd5b82018360208201111561075c57600080fd5b803590602001918460018302840111600160201b8311171561077d57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b8111156107cf57600080fd5b8201836020820111156107e157600080fd5b803590602001918460018302840111600160201b8311171561080257600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff1691506112619050565b61037f6004803603602081101561085e57600080fd5b5035611448565b61037f611454565b61034e6004803603604081101561088357600080fd5b506001600160a01b0381351690602001356117ac565b61037f61181d565b61037f611823565b61037f600480360360608110156108bf57600080fd5b506001600160a01b038135811691602081013590911690604001356118c2565b61037f600480360360208110156108f557600080fd5b50356001600160a01b0316611933565b61037f6119bf565b6109336004803603602081101561092357600080fd5b50356001600160a01b0316611a7b565b604080519485526020850193909352838301919091526060830152519081900360800190f35b61037f6004803603602081101561096f57600080fd5b5035611b10565b61037f6004803603602081101561098c57600080fd5b5035611b1b565b61037f600480360360408110156109a957600080fd5b506001600160a01b0381358116916020013516611b26565b61037f611b51565b61037f600480360360208110156109df57600080fd5b50356001600160a01b0316611c54565b610591611c8e565b61037f60048036036060811015610a0d57600080fd5b506001600160a01b03813581169160208101359160409091013516611c9d565b610591611cb5565b61037f611cc9565b61037f60048036036020811015610a5357600080fd5b5035611d2d565b61034e611dab565b60018054604080516020600284861615610100026000190190941693909304601f81018490048402820184019092528181529291830182828015610ae75780601f10610abc57610100808354040283529160200191610ae7565b820191906000526020600020905b815481529060010190602001808311610aca57829003601f168201915b505050505081565b336000818152600f602090815260408083206001600160a01b03871680855290835281842086905581518681529151939493909284927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929081900390910190a360019150505b92915050565b600080610b6883611db0565b509150505b919050565b60085481565b6000805460ff16610bbd576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610bcf611454565b14610c1a576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b610c2382611204565b90505b6000805460ff19166001179055919050565b600d5481565b6000806000610c4b611e59565b90925090506000826003811115610c5e57fe5b14610c9a5760405162461bcd60e51b8152600401808060200182810382526035815260200180614e4c6035913960400191505060405180910390fd5b9150505b90565b610caf868686868686611261565b601180546001600160a01b0319166001600160a01b038981169190911791829055604080516318160ddd60e01b8152905192909116916318160ddd91600480820192602092909190829003018186803b158015610d0b57600080fd5b505afa158015610d1f573d6000803e3d6000fd5b505050506040513d6020811015610d3557600080fd5b505050505050505050565b6000805460ff16610d85576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610d9b33868686611f08565b1490506000805460ff191660011790559392505050565b600080610dbf8484612216565b50949350505050565b6004546001600160a01b031681565b60035460ff1681565b6000610dea614b3a565b6040518060200160405280610dfd6119bf565b90526001600160a01b0384166000908152600e6020526040812054919250908190610e299084906122c1565b90925090506000826003811115610e3c57fe5b14610e8e576040805162461bcd60e51b815260206004820152601f60248201527f62616c616e636520636f756c64206e6f742062652063616c63756c6174656400604482015290519081900360640190fd5b949350505050565b6000610ea0612315565b905090565b6000610b5682612395565b60035460009061010090046001600160a01b03163314610edd57610ed66001603f612429565b9050610b6d565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b158015610f2257600080fd5b505afa158015610f36573d6000803e3d6000fd5b505050506040513d6020811015610f4c57600080fd5b5051610f9f576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9392505050565b600b5481565b6005546001600160a01b031681565b6000805460ff1661105f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611071611454565b905080156110975761108f81601081111561108857fe5b6030612429565b915050610c26565b6110a08361248f565b9150506000805460ff19166001179055919050565b60095481565b6011546001600160a01b031681565b6001600160a01b03166000908152600e602052604090205490565b6000805460ff1661112a576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916815561113c611454565b14611187576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b50600b546000805460ff1916600117905590565b6000610b56826125c2565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f81018490048402820184019092528181529291830182828015610ae75780601f10610abc57610100808354040283529160200191610ae7565b600080600061121284612643565b9092509050600082600381111561122557fe5b14610ffe5760405162461bcd60e51b8152600401808060200182810382526037815260200180614d576037913960400191505060405180910390fd5b60035461010090046001600160a01b031633146112af5760405162461bcd60e51b8152600401808060200182810382526024815260200180614c936024913960400191505060405180910390fd5b6009541580156112bf5750600a54155b6112fa5760405162461bcd60e51b8152600401808060200182810382526023815260200180614cb76023913960400191505060405180910390fd5b60078490558361133b5760405162461bcd60e51b8152600401808060200182810382526030815260200180614cda6030913960400191505060405180910390fd5b600061134687610eb0565b9050801561139b576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b6113a36126f7565b600955670de0b6b3a7640000600a556113bb866126fb565b905080156113fa5760405162461bcd60e51b8152600401808060200182810382526022815260200180614d0a6022913960400191505060405180910390fd5b835161140d906001906020870190614b4d565b508251611421906002906020860190614b4d565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b600080610b6883612870565b60008061145f6126f7565b6009549091508082141561147857600092505050610c9e565b6000611482612315565b600b54600c54600a54600654604080516315f2405360e01b815260048101879052602481018690526044810185905290519596509394929391926000926001600160a01b03909216916315f24053916064808301926020929190829003018186803b1580156114f057600080fd5b505afa158015611504573d6000803e3d6000fd5b505050506040513d602081101561151a57600080fd5b5051905065048c27395000811115611579576040805162461bcd60e51b815260206004820152601c60248201527f626f72726f772072617465206973206162737572646c79206869676800000000604482015290519081900360640190fd5b60008061158689896128f1565b9092509050600082600381111561159957fe5b146115eb576040805162461bcd60e51b815260206004820152601f60248201527f636f756c64206e6f742063616c63756c61746520626c6f636b2064656c746100604482015290519081900360640190fd5b6115f3614b3a565b60008060008061161160405180602001604052808a81525087612914565b9097509450600087600381111561162457fe5b14611656576116416009600689600381111561163c57fe5b61297c565b9e505050505050505050505050505050610c9e565b611660858c6122c1565b9097509350600087600381111561167357fe5b1461168b576116416009600189600381111561163c57fe5b611695848c6129e2565b909750925060008760038111156116a857fe5b146116c0576116416009600489600381111561163c57fe5b6116db6040518060200160405280600854815250858c612a08565b909750915060008760038111156116ee57fe5b14611706576116416009600589600381111561163c57fe5b611711858a8b612a08565b9097509050600087600381111561172457fe5b1461173c576116416009600389600381111561163c57fe5b60098e9055600a819055600b839055600c829055604080518d8152602081018690528082018390526060810185905290517f4dec04e750ca11537cabcd8a9eab06494de08da3735bc8871cd41250e190bc049181900360800190a160009e50505050505050505050505050505090565b6000805460ff166117f1576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916815561180733338686611f08565b1490506000805460ff1916600117905592915050565b600a5481565b6006546000906001600160a01b031663b816881661183f612315565b600b54600c546008546040518563ffffffff1660e01b81526004018085815260200184815260200183815260200182815260200194505050505060206040518083038186803b15801561189157600080fd5b505afa1580156118a5573d6000803e3d6000fd5b505050506040513d60208110156118bb57600080fd5b5051905090565b6000805460ff16611907576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916905561191d33858585612a64565b90506000805460ff191660011790559392505050565b60035460009061010090046001600160a01b0316331461195957610ed660016045612429565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a9929181900390910190a16000610ffe565b6000805460ff16611a04576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611a16611454565b14611a61576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b611a69610c3e565b90506000805460ff1916600117905590565b6001600160a01b0381166000908152600e6020526040812054819081908190818080611aa689612643565b935090506000816003811115611ab857fe5b14611ad65760095b975060009650869550859450611b099350505050565b611ade611e59565b925090506000816003811115611af057fe5b14611afc576009611ac0565b5060009650919450925090505b9193509193565b6000610b5682612cca565b6000610b5682612d49565b6001600160a01b039182166000908152600f6020908152604080832093909416825291909152205490565b6004546000906001600160a01b031633141580611b6c575033155b15611b8457611b7d60016000612429565b9050610c9e565b60038054600480546001600160a01b03818116610100818102610100600160a81b0319871617968790556001600160a01b031990931690935560408051948390048216808652929095041660208401528351909391927ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc92908290030190a1600454604080516001600160a01b038085168252909216602083015280517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a99281900390910190a160009250505090565b600080611c5f611454565b90508015611c8557611c7d816010811115611c7657fe5b6040612429565b915050610b6d565b610ffe836126fb565b6006546001600160a01b031681565b600080611cab858585612dc3565b5095945050505050565b60035461010090046001600160a01b031681565b6006546000906001600160a01b03166315f24053611ce5612315565b600b54600c546040518463ffffffff1660e01b815260040180848152602001838152602001828152602001935050505060206040518083038186803b15801561189157600080fd5b6000805460ff16611d72576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611d84611454565b90508015611da25761108f816010811115611d9b57fe5b6046612429565b6110a083612ef5565b600181565b60008054819060ff16611df7576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611e09611454565b90508015611e3457611e27816010811115611e2057fe5b6036612429565b925060009150611e459050565b611e3f333386612f9d565b92509250505b6000805460ff191660011790559092909150565b600d54600090819080611e7457505060075460009150611f04565b6000611e7e612315565b90506000611e8a614b3a565b6000611e9b84600b54600c54613382565b935090506000816003811115611ead57fe5b14611ec257955060009450611f049350505050565b611ecc83866133c0565b925090506000816003811115611ede57fe5b14611ef357955060009450611f049350505050565b5051600095509350611f0492505050565b9091565b600554604080516317b9b84b60e31b81523060048201526001600160a01b03868116602483015285811660448301526064820185905291516000938493169163bdcdc25891608480830192602092919082900301818787803b158015611f6d57600080fd5b505af1158015611f81573d6000803e3d6000fd5b505050506040513d6020811015611f9757600080fd5b505190508015611fb657611fae6003604a8361297c565b915050610e8e565b836001600160a01b0316856001600160a01b03161415611fdc57611fae6002604b612429565b60006001600160a01b038781169087161415611ffb5750600019612023565b506001600160a01b038086166000908152600f60209081526040808320938a16835292905220545b60008060008061203385896128f1565b9094509250600084600381111561204657fe5b14612064576120576009604b612429565b9650505050505050610e8e565b6001600160a01b038a166000908152600e602052604090205461208790896128f1565b9094509150600084600381111561209a57fe5b146120ab576120576009604c612429565b6001600160a01b0389166000908152600e60205260409020546120ce90896129e2565b909450905060008460038111156120e157fe5b146120f2576120576009604d612429565b6001600160a01b03808b166000908152600e6020526040808220859055918b16815220819055600019851461214a576001600160a01b03808b166000908152600f60209081526040808320938f168352929052208390555b886001600160a01b03168a6001600160a01b0316600080516020614dc88339815191528a6040518082815260200191505060405180910390a36005546040805163352b4a3f60e11b81523060048201526001600160a01b038d811660248301528c81166044830152606482018c905291519190921691636a56947e91608480830192600092919082900301818387803b1580156121e657600080fd5b505af11580156121fa573d6000803e3d6000fd5b5060009250612207915050565b9b9a5050505050505050505050565b60008054819060ff1661225d576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916815561226f611454565b9050801561229a5761228d81601081111561228657fe5b6035612429565b9250600091506122ab9050565b6122a5338686612f9d565b92509250505b6000805460ff1916600117905590939092509050565b60008060006122ce614b3a565b6122d88686612914565b909250905060008260038111156122eb57fe5b146122fc575091506000905061230e565b600061230782613470565b9350935050505b9250929050565b601154604080516370a0823160e01b815230600482015290516000926001600160a01b03169182916370a0823191602480820192602092909190829003018186803b15801561236357600080fd5b505afa158015612377573d6000803e3d6000fd5b505050506040513d602081101561238d57600080fd5b505191505090565b6000805460ff166123da576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556123ec611454565b9050801561240a5761108f81601081111561240357fe5b604e612429565b6124138361347f565b509150506000805460ff19166001179055919050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa083601081111561245857fe5b83605081111561246457fe5b604080519283526020830191909152600082820152519081900360600190a1826010811115610ffe57fe5b600354600090819061010090046001600160a01b031633146124b757611c7d60016031612429565b6124bf6126f7565b600954146124d357611c7d600a6033612429565b826124dc612315565b10156124ee57611c7d600e6032612429565b600c5483111561250457611c7d60026034612429565b50600c548281039081111561254a5760405162461bcd60e51b8152600401808060200182810382526024815260200180614edd6024913960400191505060405180910390fd5b600c81905560035461256a9061010090046001600160a01b031684613567565b600354604080516101009092046001600160a01b0316825260208201859052818101839052517f3bad0c59cf2f06e7314077049f48a93578cd16f5ef92329f1dab1420a99c177e916060908290030190a16000610ffe565b6000805460ff16612607576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612619611454565b905080156126375761108f81601081111561263057fe5b6027612429565b6110a03360008561365e565b6001600160a01b03811660009081526010602052604081208054829182918291829161267a5750600094508493506126f292505050565b61268a8160000154600a54613b25565b9094509250600084600381111561269d57fe5b146126b25750919350600092506126f2915050565b6126c0838260010154613b64565b909450915060008460038111156126d357fe5b146126e85750919350600092506126f2915050565b5060009450925050505b915091565b4390565b600354600090819061010090046001600160a01b0316331461272357611c7d60016042612429565b61272b6126f7565b6009541461273f57611c7d600a6041612429565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b15801561279057600080fd5b505afa1580156127a4573d6000803e3d6000fd5b505050506040513d60208110156127ba57600080fd5b505161280d576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a16000610ffe565b60008054819060ff166128b7576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556128c9611454565b905080156128e757611e278160108111156128e057fe5b601e612429565b611e3f3385613b8f565b60008083831161290857506000905081830361230e565b5060039050600061230e565b600061291e614b3a565b60008061292f866000015186613b25565b9092509050600082600381111561294257fe5b146129615750604080516020810190915260008152909250905061230e565b60408051602081019091529081526000969095509350505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa08460108111156129ab57fe5b8460508111156129b757fe5b604080519283526020830191909152818101859052519081900360600190a1836010811115610e8e57fe5b6000808383018481106129fa5760009250905061230e565b50600291506000905061230e565b6000806000612a15614b3a565b612a1f8787612914565b90925090506000826003811115612a3257fe5b14612a435750915060009050612a5c565b612a55612a4f82613470565b866129e2565b9350935050505b935093915050565b6005546040805163d02f735160e01b81523060048201526001600160a01b038781166024830152868116604483015285811660648301526084820185905291516000938493169163d02f73519160a480830192602092919082900301818787803b158015612ad157600080fd5b505af1158015612ae5573d6000803e3d6000fd5b505050506040513d6020811015612afb57600080fd5b505190508015612b1257611fae6003601b8361297c565b846001600160a01b0316846001600160a01b03161415612b3857611fae6006601c612429565b6001600160a01b0384166000908152600e602052604081205481908190612b5f90876128f1565b90935091506000836003811115612b7257fe5b14612b9557612b8a6009601a85600381111561163c57fe5b945050505050610e8e565b6001600160a01b0388166000908152600e6020526040902054612bb890876129e2565b90935090506000836003811115612bcb57fe5b14612be357612b8a6009601985600381111561163c57fe5b6001600160a01b038088166000818152600e60209081526040808320879055938c168083529184902085905583518a815293519193600080516020614dc8833981519152929081900390910190a360055460408051636d35bf9160e01b81523060048201526001600160a01b038c811660248301528b811660448301528a81166064830152608482018a905291519190921691636d35bf919160a480830192600092919082900301818387803b158015612c9c57600080fd5b505af1158015612cb0573d6000803e3d6000fd5b5060009250612cbd915050565b9998505050505050505050565b6000805460ff16612d0f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612d21611454565b90508015612d3f5761108f816010811115612d3857fe5b6008612429565b6110a03384613fee565b6000805460ff16612d8e576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612da0611454565b90508015612db75761108f81601081111561263057fe5b6110a03384600061365e565b60008054819060ff16612e0a576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612e1c611454565b90508015612e4757612e3a816010811115612e3357fe5b600f612429565b925060009150612ede9050565b836001600160a01b031663a6afed956040518163ffffffff1660e01b8152600401602060405180830381600087803b158015612e8257600080fd5b505af1158015612e96573d6000803e3d6000fd5b505050506040513d6020811015612eac57600080fd5b505190508015612ecc57612e3a816010811115612ec557fe5b6010612429565b612ed8338787876142fc565b92509250505b6000805460ff191660011790559094909350915050565b60035460009061010090046001600160a01b03163314612f1b57610ed660016047612429565b612f236126f7565b60095414612f3757610ed6600a6048612429565b670de0b6b3a7640000821115612f5357610ed660026049612429565b6008805490839055604080518281526020810185905281517faaa68312e2ea9d50e16af5068410ab56e1a1fd06037b1a35664812c30f821460929181900390910190a16000610ffe565b60055460408051631200453160e11b81523060048201526001600160a01b0386811660248301528581166044830152606482018590529151600093849384939116916324008a629160848082019260209290919082900301818787803b15801561300657600080fd5b505af115801561301a573d6000803e3d6000fd5b505050506040513d602081101561303057600080fd5b50519050801561305457613047600360388361297c565b925060009150612a5c9050565b61305c6126f7565b6009541461307057613047600a6039612429565b613078614bcb565b6001600160a01b03861660009081526010602052604090206001015460608201526130a286612643565b60808301819052602083018260038111156130b957fe5b60038111156130c457fe5b90525060009050816020015160038111156130db57fe5b14613105576130f7600960378360200151600381111561163c57fe5b935060009250612a5c915050565b60001985141561311e5760808101516040820152613126565b604081018590525b61313487826040015161487f565b60e082018190526080820151613149916128f1565b60a083018190526020830182600381111561316057fe5b600381111561316b57fe5b905250600090508160200151600381111561318257fe5b146131be5760405162461bcd60e51b815260040180806020018281038252603a815260200180614d8e603a913960400191505060405180910390fd5b6131ce600b548260e001516128f1565b60c08301819052602083018260038111156131e557fe5b60038111156131f057fe5b905250600090508160200151600381111561320757fe5b146132435760405162461bcd60e51b8152600401808060200182810382526031815260200180614de86031913960400191505060405180910390fd5b60a080820180516001600160a01b03808a16600081815260106020908152604091829020948555600a5460019095019490945560c0870151600b81905560e088015195518251948f16855294840192909252828101949094526060820192909252608081019190915290517f1a2a22cb034d26d1854bdc6666a5b91fe25efbbb5dcad3b0355478d6f5c362a1929181900390910190a160055460e0820151606083015160408051631ededc9160e01b81523060048201526001600160a01b038c811660248301528b8116604483015260648201949094526084810192909252519190921691631ededc919160a480830192600092919082900301818387803b15801561334e57600080fd5b505af1158015613362573d6000803e3d6000fd5b506000925061336f915050565b8160e00151935093505050935093915050565b60008060008061339287876129e2565b909250905060008260038111156133a557fe5b146133b65750915060009050612a5c565b612a5581866128f1565b60006133ca614b3a565b6000806133df86670de0b6b3a7640000613b25565b909250905060008260038111156133f257fe5b146134115750604080516020810190915260008152909250905061230e565b60008061341e8388613b64565b9092509050600082600381111561343157fe5b146134535750604080516020810190915260008152909450925061230e915050565b604080516020810190915290815260009890975095505050505050565b51670de0b6b3a7640000900490565b60008060008061348d6126f7565b600954146134ac576134a1600a604f612429565b935091506126f29050565b6134b6338661487f565b905080600c54019150600c54821015613516576040805162461bcd60e51b815260206004820181905260248201527f61646420726573657276657320756e6578706563746564206f766572666c6f77604482015290519081900360640190fd5b600c829055604080513381526020810183905280820184905290517fa91e67c5ea634cd43a12c5a482724b03de01e85ca68702a53d0c2f45cb7c1dc59181900360600190a160009350915050915091565b6011546040805163a9059cbb60e01b81526001600160a01b0385811660048301526024820185905291519190921691829163a9059cbb9160448082019260009290919082900301818387803b1580156135bf57600080fd5b505af11580156135d3573d6000803e3d6000fd5b5050505060003d600081146135ef57602081146135f957600080fd5b6000199150613605565b60206000803e60005191505b5080613658576040805162461bcd60e51b815260206004820152601960248201527f544f4b454e5f5452414e534645525f4f55545f4641494c454400000000000000604482015290519081900360640190fd5b50505050565b600082158061366b575081155b6136a65760405162461bcd60e51b8152600401808060200182810382526034815260200180614ea96034913960400191505060405180910390fd5b6136ae614c11565b6136b6611e59565b60408301819052602083018260038111156136cd57fe5b60038111156136d857fe5b90525060009050816020015160038111156136ef57fe5b146137135761370b6009602b8360200151600381111561163c57fe5b915050610ffe565b831561379457606081018490526040805160208101825290820151815261373a90856122c1565b608083018190526020830182600381111561375157fe5b600381111561375c57fe5b905250600090508160200151600381111561377357fe5b1461378f5761370b600960298360200151600381111561163c57fe5b61380d565b6137b08360405180602001604052808460400151815250614ac9565b60608301819052602083018260038111156137c757fe5b60038111156137d257fe5b90525060009050816020015160038111156137e957fe5b146138055761370b6009602a8360200151600381111561163c57fe5b608081018390525b60055460608201516040805163eabe7d9160e01b81523060048201526001600160a01b03898116602483015260448201939093529051600093929092169163eabe7d919160648082019260209290919082900301818787803b15801561387257600080fd5b505af1158015613886573d6000803e3d6000fd5b505050506040513d602081101561389c57600080fd5b5051905080156138bc576138b3600360288361297c565b92505050610ffe565b6138c46126f7565b600954146138d8576138b3600a602c612429565b6138e8600d5483606001516128f1565b60a08401819052602084018260038111156138ff57fe5b600381111561390a57fe5b905250600090508260200151600381111561392157fe5b1461393d576138b36009602e8460200151600381111561163c57fe5b6001600160a01b0386166000908152600e6020526040902054606083015161396591906128f1565b60c084018190526020840182600381111561397c57fe5b600381111561398757fe5b905250600090508260200151600381111561399e57fe5b146139ba576138b36009602d8460200151600381111561163c57fe5b81608001516139c7612315565b10156139d9576138b3600e602f612429565b6139e7868360800151613567565b60a0820151600d5560c08201516001600160a01b0387166000818152600e6020908152604091829020939093556060850151815190815290513093600080516020614dc8833981519152928290030190a36080820151606080840151604080516001600160a01b038b168152602081019490945283810191909152517fe5b754fb1abb7f01b499791d0b820ae3b6af3424ac1c59768edb53f4ec31a9299281900390910190a160055460808301516060840151604080516351dff98960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916351dff98991608480830192600092919082900301818387803b158015613afa57600080fd5b505af1158015613b0e573d6000803e3d6000fd5b5060009250613b1b915050565b9695505050505050565b60008083613b385750600090508061230e565b83830283858281613b4557fe5b0414613b595750600291506000905061230e565b60009250905061230e565b60008082613b78575060019050600061230e565b6000838581613b8357fe5b04915091509250929050565b60055460408051634ef4c3e160e01b81523060048201526001600160a01b03858116602483015260448201859052915160009384938493911691634ef4c3e19160648082019260209290919082900301818787803b158015613bf057600080fd5b505af1158015613c04573d6000803e3d6000fd5b505050506040513d6020811015613c1a57600080fd5b505190508015613c3e57613c316003601f8361297c565b92506000915061230e9050565b613c466126f7565b60095414613c5a57613c31600a6022612429565b613c62614c11565b613c6a611e59565b6040830181905260208301826003811115613c8157fe5b6003811115613c8c57fe5b9052506000905081602001516003811115613ca357fe5b14613ccd57613cbf600960218360200151600381111561163c57fe5b93506000925061230e915050565b613cd7868661487f565b60c0820181905260408051602081018252908301518152613cf89190614ac9565b6060830181905260208301826003811115613d0f57fe5b6003811115613d1a57fe5b9052506000905081602001516003811115613d3157fe5b14613d83576040805162461bcd60e51b815260206004820181905260248201527f4d494e545f45584348414e47455f43414c43554c4154494f4e5f4641494c4544604482015290519081900360640190fd5b613d93600d5482606001516129e2565b6080830181905260208301826003811115613daa57fe5b6003811115613db557fe5b9052506000905081602001516003811115613dcc57fe5b14613e085760405162461bcd60e51b8152600401808060200182810382526028815260200180614e816028913960400191505060405180910390fd5b6001600160a01b0386166000908152600e60205260409020546060820151613e3091906129e2565b60a0830181905260208301826003811115613e4757fe5b6003811115613e5257fe5b9052506000905081602001516003811115613e6957fe5b14613ea55760405162461bcd60e51b815260040180806020018281038252602b815260200180614d2c602b913960400191505060405180910390fd5b6080810151600d5560a08101516001600160a01b0387166000818152600e60209081526040918290209390935560c084015160608086015183519485529484019190915282820193909352517f4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f929181900390910190a1606081015160408051918252516001600160a01b038816913091600080516020614dc88339815191529181900360200190a360055460c08201516060830151604080516341c728b960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916341c728b991608480830192600092919082900301818387803b158015613fbb57600080fd5b505af1158015613fcf573d6000803e3d6000fd5b5060009250613fdc915050565b8160c001519350935050509250929050565b6005546040805163368f515360e21b81523060048201526001600160a01b0385811660248301526044820185905291516000938493169163da3d454c91606480830192602092919082900301818787803b15801561404b57600080fd5b505af115801561405f573d6000803e3d6000fd5b505050506040513d602081101561407557600080fd5b5051905080156140945761408c6003600e8361297c565b915050610b56565b61409c6126f7565b600954146140af5761408c600a80612429565b826140b8612315565b10156140ca5761408c600e6009612429565b6140d2614c4f565b6140db85612643565b60208301819052828260038111156140ef57fe5b60038111156140fa57fe5b905250600090508151600381111561410e57fe5b146141335761412a600960078360000151600381111561163c57fe5b92505050610b56565b6141418160200151856129e2565b604083018190528282600381111561415557fe5b600381111561416057fe5b905250600090508151600381111561417457fe5b146141905761412a6009600c8360000151600381111561163c57fe5b61419c600b54856129e2565b60608301819052828260038111156141b057fe5b60038111156141bb57fe5b90525060009050815160038111156141cf57fe5b146141eb5761412a6009600b8360000151600381111561163c57fe5b6141f58585613567565b604080820180516001600160a01b03881660008181526010602090815290859020928355600a54600190930192909255606080860151600b81905593518551928352928201899052818501929092529081019190915290517f13ed6866d4e1ee6da46f845c46d7e54120883d75c5ea9a2dacc1c4ca8984ab809181900360800190a160055460408051635c77860560e01b81523060048201526001600160a01b0388811660248301526044820188905291519190921691635c77860591606480830192600092919082900301818387803b1580156142d257600080fd5b505af11580156142e6573d6000803e3d6000fd5b50600092506142f3915050565b95945050505050565b60055460408051632fe3f38f60e11b81523060048201526001600160a01b0384811660248301528781166044830152868116606483015260848201869052915160009384938493911691635fc7e71e9160a48082019260209290919082900301818787803b15801561436d57600080fd5b505af1158015614381573d6000803e3d6000fd5b505050506040513d602081101561439757600080fd5b5051905080156143bb576143ae600360128361297c565b9250600091506148769050565b6143c36126f7565b600954146143d7576143ae600a6016612429565b6143df6126f7565b846001600160a01b0316636c540baf6040518163ffffffff1660e01b815260040160206040518083038186803b15801561441857600080fd5b505afa15801561442c573d6000803e3d6000fd5b505050506040513d602081101561444257600080fd5b505114614455576143ae600a6011612429565b866001600160a01b0316866001600160a01b0316141561447b576143ae60066017612429565b8461448c576143ae60076015612429565b6000198514156144a2576143ae60076014612429565b6000806144b0898989612f9d565b909250905081156144e0576144d18260108111156144ca57fe5b6018612429565b94506000935061487692505050565b6005546040805163c488847b60e01b81523060048201526001600160a01b038981166024830152604482018590528251600094859492169263c488847b926064808301939192829003018186803b15801561453a57600080fd5b505afa15801561454e573d6000803e3d6000fd5b505050506040513d604081101561456457600080fd5b508051602090910151909250905081156145af5760405162461bcd60e51b8152600401808060200182810382526033815260200180614e196033913960400191505060405180910390fd5b80886001600160a01b03166370a082318c6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b15801561460657600080fd5b505afa15801561461a573d6000803e3d6000fd5b505050506040513d602081101561463057600080fd5b50511015614685576040805162461bcd60e51b815260206004820152601860248201527f4c49515549444154455f5345495a455f544f4f5f4d5543480000000000000000604482015290519081900360640190fd5b60006001600160a01b0389163014156146ab576146a4308d8d85612a64565b9050614735565b6040805163b2a02ff160e01b81526001600160a01b038e811660048301528d81166024830152604482018590529151918b169163b2a02ff1916064808201926020929091908290030181600087803b15801561470657600080fd5b505af115801561471a573d6000803e3d6000fd5b505050506040513d602081101561473057600080fd5b505190505b801561477f576040805162461bcd60e51b81526020600482015260146024820152731d1bdad95b881cd95a5e9d5c994819985a5b195960621b604482015290519081900360640190fd5b604080516001600160a01b03808f168252808e1660208301528183018790528b1660608201526080810184905290517f298637f684da70674f26509b10f07ec2fbc77a335ab1e7d6215a4b2484d8bb529181900360a00190a1600554604080516347ef3b3b60e01b81523060048201526001600160a01b038c811660248301528f811660448301528e811660648301526084820188905260a48201869052915191909216916347ef3b3b9160c480830192600092919082900301818387803b15801561484a57600080fd5b505af115801561485e573d6000803e3d6000fd5b506000925061486b915050565b975092955050505050505b94509492505050565b601154604080516370a0823160e01b815230600482015290516000926001600160a01b031691839183916370a08231916024808301926020929190829003018186803b1580156148ce57600080fd5b505afa1580156148e2573d6000803e3d6000fd5b505050506040513d60208110156148f857600080fd5b5051604080516323b872dd60e01b81526001600160a01b038881166004830152306024830152604482018890529151929350908416916323b872dd9160648082019260009290919082900301818387803b15801561495557600080fd5b505af1158015614969573d6000803e3d6000fd5b5050505060003d60008114614985576020811461498f57600080fd5b600019915061499b565b60206000803e60005191505b50806149ee576040805162461bcd60e51b815260206004820152601860248201527f544f4b454e5f5452414e534645525f494e5f4641494c45440000000000000000604482015290519081900360640190fd5b601154604080516370a0823160e01b815230600482015290516000926001600160a01b0316916370a08231916024808301926020929190829003018186803b158015614a3957600080fd5b505afa158015614a4d573d6000803e3d6000fd5b505050506040513d6020811015614a6357600080fd5b5051905082811015614abc576040805162461bcd60e51b815260206004820152601a60248201527f544f4b454e5f5452414e534645525f494e5f4f564552464c4f57000000000000604482015290519081900360640190fd5b9190910395945050505050565b6000806000614ad6614b3a565b6122d886866000614ae5614b3a565b600080614afa670de0b6b3a764000087613b25565b90925090506000826003811115614b0d57fe5b14614b2c5750604080516020810190915260008152909250905061230e565b6123078186600001516133c0565b6040518060200160405280600081525090565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f10614b8e57805160ff1916838001178555614bbb565b82800160010185558215614bbb579182015b82811115614bbb578251825591602001919060010190614ba0565b50614bc7929150614c78565b5090565b6040805161010081019091528060008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040805160e0810190915280600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604080516080810190915280600081526020016000815260200160008152602001600081525090565b610c9e91905b80821115614bc75760008155600101614c7e56fe6f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e6365696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c65644d494e545f4e45575f4143434f554e545f42414c414e43455f43414c43554c4154494f4e5f4641494c4544626f72726f7742616c616e636553746f7265643a20626f72726f7742616c616e636553746f726564496e7465726e616c206661696c656452455041595f424f52524f575f4e45575f4143434f554e545f424f52524f575f42414c414e43455f43414c43554c4154494f4e5f4641494c4544ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef52455041595f424f52524f575f4e45575f544f54414c5f42414c414e43455f43414c43554c4154494f4e5f4641494c45444c49515549444154455f434f4d5054524f4c4c45525f43414c43554c4154455f414d4f554e545f5345495a455f4641494c454465786368616e67655261746553746f7265643a2065786368616e67655261746553746f726564496e7465726e616c206661696c65644d494e545f4e45575f544f54414c5f535550504c595f43414c43554c4154494f4e5f4641494c45446f6e65206f662072656465656d546f6b656e73496e206f722072656465656d416d6f756e74496e206d757374206265207a65726f72656475636520726573657276657320756e657870656374656420756e646572666c6f77a265627a7a72315820ead775909838aeea29cf2981b343afca3d1b25fc293b9186e947cc9017207ce364736f6c634300051100326f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e6365696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c6564" . parse () . expect ("invalid bytecode")
        });
    pub struct CErc20Immutable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CErc20Immutable<M> {
        fn clone(&self) -> Self {
            CErc20Immutable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CErc20Immutable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CErc20Immutable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CErc20Immutable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CErc20Immutable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CERC20IMMUTABLE_ABI.clone(), client)
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
                CERC20IMMUTABLE_ABI.clone(),
                CERC20IMMUTABLE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `_reduceReserves` (0x601a0bf1) function"]
        pub fn reduce_reserves(
            &self,
            reduce_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([96, 26, 11, 241], reduce_amount)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CErc20ImmutableEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CErc20Immutable<M> {
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
    pub enum CErc20ImmutableEvents {
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
    impl ethers::contract::EthLogDecode for CErc20ImmutableEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::NewMarketInterestRateModelFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CErc20ImmutableEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CErc20ImmutableEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20ImmutableEvents::AccrueInterestFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::ApprovalFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::BorrowFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::FailureFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::MintFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::NewAdminFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::NewComptrollerFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::RedeemFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::RepayBorrowFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::ReservesAddedFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::ReservesReducedFilter(element) => element.fmt(f),
                CErc20ImmutableEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_acceptAdmin` function with signature `_acceptAdmin()` and selector `[233, 199, 20, 242]`"]
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
    #[doc = "Container type for all input parameters for the `_addReserves` function with signature `_addReserves(uint256)` and selector `[62, 148, 16, 16]`"]
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
    #[doc = "Container type for all input parameters for the `_reduceReserves` function with signature `_reduceReserves(uint256)` and selector `[96, 26, 11, 241]`"]
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
    #[doc = "Container type for all input parameters for the `_setComptroller` function with signature `_setComptroller(address)` and selector `[69, 118, 181, 219]`"]
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
    #[doc = "Container type for all input parameters for the `_setInterestRateModel` function with signature `_setInterestRateModel(address)` and selector `[242, 179, 171, 189]`"]
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
    #[doc = "Container type for all input parameters for the `_setPendingAdmin` function with signature `_setPendingAdmin(address)` and selector `[183, 29, 26, 12]`"]
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
    #[doc = "Container type for all input parameters for the `_setReserveFactor` function with signature `_setReserveFactor(uint256)` and selector `[252, 167, 130, 11]`"]
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
    #[doc = "Container type for all input parameters for the `accrualBlockNumber` function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
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
    #[doc = "Container type for all input parameters for the `accrueInterest` function with signature `accrueInterest()` and selector `[166, 175, 237, 149]`"]
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
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOfUnderlying` function with signature `balanceOfUnderlying(address)` and selector `[58, 249, 230, 105]`"]
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
    #[doc = "Container type for all input parameters for the `borrow` function with signature `borrow(uint256)` and selector `[197, 235, 234, 236]`"]
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
    #[doc = "Container type for all input parameters for the `borrowBalanceCurrent` function with signature `borrowBalanceCurrent(address)` and selector `[23, 191, 223, 188]`"]
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
    #[doc = "Container type for all input parameters for the `borrowBalanceStored` function with signature `borrowBalanceStored(address)` and selector `[149, 221, 145, 147]`"]
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
    #[doc = "Container type for all input parameters for the `borrowIndex` function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
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
    #[doc = "Container type for all input parameters for the `borrowRatePerBlock` function with signature `borrowRatePerBlock()` and selector `[248, 249, 218, 40]`"]
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
    #[doc = "Container type for all input parameters for the `comptroller` function with signature `comptroller()` and selector `[95, 227, 181, 103]`"]
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
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[doc = "Container type for all input parameters for the `exchangeRateCurrent` function with signature `exchangeRateCurrent()` and selector `[189, 109, 137, 77]`"]
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
    #[doc = "Container type for all input parameters for the `exchangeRateStored` function with signature `exchangeRateStored()` and selector `[24, 45, 240, 245]`"]
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
    #[doc = "Container type for all input parameters for the `getAccountSnapshot` function with signature `getAccountSnapshot(address)` and selector `[195, 127, 104, 226]`"]
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
    #[doc = "Container type for all input parameters for the `getCash` function with signature `getCash()` and selector `[59, 29, 33, 162]`"]
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint256,string,string,uint8)` and selector `[26, 49, 212, 101]`"]
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,string,string,uint8)` and selector `[153, 216, 193, 180]`"]
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
    #[doc = "Container type for all input parameters for the `interestRateModel` function with signature `interestRateModel()` and selector `[243, 253, 177, 90]`"]
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
    #[doc = "Container type for all input parameters for the `isCToken` function with signature `isCToken()` and selector `[254, 156, 68, 174]`"]
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
    #[doc = "Container type for all input parameters for the `liquidateBorrow` function with signature `liquidateBorrow(address,uint256,address)` and selector `[245, 227, 196, 98]`"]
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
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(uint256)` and selector `[160, 113, 45, 104]`"]
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
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[doc = "Container type for all input parameters for the `redeem` function with signature `redeem(uint256)` and selector `[219, 0, 106, 117]`"]
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
    #[doc = "Container type for all input parameters for the `redeemUnderlying` function with signature `redeemUnderlying(uint256)` and selector `[133, 42, 18, 227]`"]
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
    #[doc = "Container type for all input parameters for the `repayBorrow` function with signature `repayBorrow(uint256)` and selector `[14, 117, 39, 2]`"]
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
    #[doc = "Container type for all input parameters for the `repayBorrowBehalf` function with signature `repayBorrowBehalf(address,uint256)` and selector `[38, 8, 248, 24]`"]
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
    #[doc = "Container type for all input parameters for the `reserveFactorMantissa` function with signature `reserveFactorMantissa()` and selector `[23, 59, 153, 4]`"]
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
    #[doc = "Container type for all input parameters for the `seize` function with signature `seize(address,address,uint256)` and selector `[178, 160, 47, 241]`"]
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
    #[doc = "Container type for all input parameters for the `supplyRatePerBlock` function with signature `supplyRatePerBlock()` and selector `[174, 157, 112, 176]`"]
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
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[doc = "Container type for all input parameters for the `totalBorrows` function with signature `totalBorrows()` and selector `[71, 189, 55, 24]`"]
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
    #[doc = "Container type for all input parameters for the `totalBorrowsCurrent` function with signature `totalBorrowsCurrent()` and selector `[115, 172, 238, 152]`"]
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
    #[doc = "Container type for all input parameters for the `totalReserves` function with signature `totalReserves()` and selector `[143, 132, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
    #[doc = "Container type for all input parameters for the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
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
    pub enum CErc20ImmutableCalls {
        AcceptAdmin(AcceptAdminCall),
        AddReserves(AddReservesCall),
        ReduceReserves(ReduceReservesCall),
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
    impl ethers::core::abi::AbiDecode for CErc20ImmutableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::AddReserves(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <InitializeWithUnderlyingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CErc20ImmutableCalls::InitializeWithUnderlying(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::IsCToken(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20ImmutableCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20ImmutableCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20ImmutableCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CErc20ImmutableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CErc20ImmutableCalls::AcceptAdmin(element) => element.encode(),
                CErc20ImmutableCalls::AddReserves(element) => element.encode(),
                CErc20ImmutableCalls::ReduceReserves(element) => element.encode(),
                CErc20ImmutableCalls::SetComptroller(element) => element.encode(),
                CErc20ImmutableCalls::SetInterestRateModel(element) => element.encode(),
                CErc20ImmutableCalls::SetPendingAdmin(element) => element.encode(),
                CErc20ImmutableCalls::SetReserveFactor(element) => element.encode(),
                CErc20ImmutableCalls::AccrualBlockNumber(element) => element.encode(),
                CErc20ImmutableCalls::AccrueInterest(element) => element.encode(),
                CErc20ImmutableCalls::Admin(element) => element.encode(),
                CErc20ImmutableCalls::Allowance(element) => element.encode(),
                CErc20ImmutableCalls::Approve(element) => element.encode(),
                CErc20ImmutableCalls::BalanceOf(element) => element.encode(),
                CErc20ImmutableCalls::BalanceOfUnderlying(element) => element.encode(),
                CErc20ImmutableCalls::Borrow(element) => element.encode(),
                CErc20ImmutableCalls::BorrowBalanceCurrent(element) => element.encode(),
                CErc20ImmutableCalls::BorrowBalanceStored(element) => element.encode(),
                CErc20ImmutableCalls::BorrowIndex(element) => element.encode(),
                CErc20ImmutableCalls::BorrowRatePerBlock(element) => element.encode(),
                CErc20ImmutableCalls::Comptroller(element) => element.encode(),
                CErc20ImmutableCalls::Decimals(element) => element.encode(),
                CErc20ImmutableCalls::ExchangeRateCurrent(element) => element.encode(),
                CErc20ImmutableCalls::ExchangeRateStored(element) => element.encode(),
                CErc20ImmutableCalls::GetAccountSnapshot(element) => element.encode(),
                CErc20ImmutableCalls::GetCash(element) => element.encode(),
                CErc20ImmutableCalls::InitializeWithUnderlying(element) => element.encode(),
                CErc20ImmutableCalls::Initialize(element) => element.encode(),
                CErc20ImmutableCalls::InterestRateModel(element) => element.encode(),
                CErc20ImmutableCalls::IsCToken(element) => element.encode(),
                CErc20ImmutableCalls::LiquidateBorrow(element) => element.encode(),
                CErc20ImmutableCalls::Mint(element) => element.encode(),
                CErc20ImmutableCalls::Name(element) => element.encode(),
                CErc20ImmutableCalls::PendingAdmin(element) => element.encode(),
                CErc20ImmutableCalls::Redeem(element) => element.encode(),
                CErc20ImmutableCalls::RedeemUnderlying(element) => element.encode(),
                CErc20ImmutableCalls::RepayBorrow(element) => element.encode(),
                CErc20ImmutableCalls::RepayBorrowBehalf(element) => element.encode(),
                CErc20ImmutableCalls::ReserveFactorMantissa(element) => element.encode(),
                CErc20ImmutableCalls::Seize(element) => element.encode(),
                CErc20ImmutableCalls::SupplyRatePerBlock(element) => element.encode(),
                CErc20ImmutableCalls::Symbol(element) => element.encode(),
                CErc20ImmutableCalls::TotalBorrows(element) => element.encode(),
                CErc20ImmutableCalls::TotalBorrowsCurrent(element) => element.encode(),
                CErc20ImmutableCalls::TotalReserves(element) => element.encode(),
                CErc20ImmutableCalls::TotalSupply(element) => element.encode(),
                CErc20ImmutableCalls::Transfer(element) => element.encode(),
                CErc20ImmutableCalls::TransferFrom(element) => element.encode(),
                CErc20ImmutableCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CErc20ImmutableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20ImmutableCalls::AcceptAdmin(element) => element.fmt(f),
                CErc20ImmutableCalls::AddReserves(element) => element.fmt(f),
                CErc20ImmutableCalls::ReduceReserves(element) => element.fmt(f),
                CErc20ImmutableCalls::SetComptroller(element) => element.fmt(f),
                CErc20ImmutableCalls::SetInterestRateModel(element) => element.fmt(f),
                CErc20ImmutableCalls::SetPendingAdmin(element) => element.fmt(f),
                CErc20ImmutableCalls::SetReserveFactor(element) => element.fmt(f),
                CErc20ImmutableCalls::AccrualBlockNumber(element) => element.fmt(f),
                CErc20ImmutableCalls::AccrueInterest(element) => element.fmt(f),
                CErc20ImmutableCalls::Admin(element) => element.fmt(f),
                CErc20ImmutableCalls::Allowance(element) => element.fmt(f),
                CErc20ImmutableCalls::Approve(element) => element.fmt(f),
                CErc20ImmutableCalls::BalanceOf(element) => element.fmt(f),
                CErc20ImmutableCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CErc20ImmutableCalls::Borrow(element) => element.fmt(f),
                CErc20ImmutableCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CErc20ImmutableCalls::BorrowBalanceStored(element) => element.fmt(f),
                CErc20ImmutableCalls::BorrowIndex(element) => element.fmt(f),
                CErc20ImmutableCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CErc20ImmutableCalls::Comptroller(element) => element.fmt(f),
                CErc20ImmutableCalls::Decimals(element) => element.fmt(f),
                CErc20ImmutableCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CErc20ImmutableCalls::ExchangeRateStored(element) => element.fmt(f),
                CErc20ImmutableCalls::GetAccountSnapshot(element) => element.fmt(f),
                CErc20ImmutableCalls::GetCash(element) => element.fmt(f),
                CErc20ImmutableCalls::InitializeWithUnderlying(element) => element.fmt(f),
                CErc20ImmutableCalls::Initialize(element) => element.fmt(f),
                CErc20ImmutableCalls::InterestRateModel(element) => element.fmt(f),
                CErc20ImmutableCalls::IsCToken(element) => element.fmt(f),
                CErc20ImmutableCalls::LiquidateBorrow(element) => element.fmt(f),
                CErc20ImmutableCalls::Mint(element) => element.fmt(f),
                CErc20ImmutableCalls::Name(element) => element.fmt(f),
                CErc20ImmutableCalls::PendingAdmin(element) => element.fmt(f),
                CErc20ImmutableCalls::Redeem(element) => element.fmt(f),
                CErc20ImmutableCalls::RedeemUnderlying(element) => element.fmt(f),
                CErc20ImmutableCalls::RepayBorrow(element) => element.fmt(f),
                CErc20ImmutableCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CErc20ImmutableCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CErc20ImmutableCalls::Seize(element) => element.fmt(f),
                CErc20ImmutableCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CErc20ImmutableCalls::Symbol(element) => element.fmt(f),
                CErc20ImmutableCalls::TotalBorrows(element) => element.fmt(f),
                CErc20ImmutableCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CErc20ImmutableCalls::TotalReserves(element) => element.fmt(f),
                CErc20ImmutableCalls::TotalSupply(element) => element.fmt(f),
                CErc20ImmutableCalls::Transfer(element) => element.fmt(f),
                CErc20ImmutableCalls::TransferFrom(element) => element.fmt(f),
                CErc20ImmutableCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CErc20ImmutableCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CErc20ImmutableCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<AddReservesCall> for CErc20ImmutableCalls {
        fn from(var: AddReservesCall) -> Self {
            CErc20ImmutableCalls::AddReserves(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CErc20ImmutableCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CErc20ImmutableCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CErc20ImmutableCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CErc20ImmutableCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CErc20ImmutableCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CErc20ImmutableCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CErc20ImmutableCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CErc20ImmutableCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CErc20ImmutableCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CErc20ImmutableCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CErc20ImmutableCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CErc20ImmutableCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CErc20ImmutableCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CErc20ImmutableCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CErc20ImmutableCalls {
        fn from(var: AdminCall) -> Self {
            CErc20ImmutableCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CErc20ImmutableCalls {
        fn from(var: AllowanceCall) -> Self {
            CErc20ImmutableCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CErc20ImmutableCalls {
        fn from(var: ApproveCall) -> Self {
            CErc20ImmutableCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CErc20ImmutableCalls {
        fn from(var: BalanceOfCall) -> Self {
            CErc20ImmutableCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CErc20ImmutableCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CErc20ImmutableCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CErc20ImmutableCalls {
        fn from(var: BorrowCall) -> Self {
            CErc20ImmutableCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CErc20ImmutableCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CErc20ImmutableCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CErc20ImmutableCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CErc20ImmutableCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CErc20ImmutableCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CErc20ImmutableCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CErc20ImmutableCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CErc20ImmutableCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CErc20ImmutableCalls {
        fn from(var: ComptrollerCall) -> Self {
            CErc20ImmutableCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CErc20ImmutableCalls {
        fn from(var: DecimalsCall) -> Self {
            CErc20ImmutableCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CErc20ImmutableCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CErc20ImmutableCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CErc20ImmutableCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CErc20ImmutableCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CErc20ImmutableCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CErc20ImmutableCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CErc20ImmutableCalls {
        fn from(var: GetCashCall) -> Self {
            CErc20ImmutableCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<InitializeWithUnderlyingCall> for CErc20ImmutableCalls {
        fn from(var: InitializeWithUnderlyingCall) -> Self {
            CErc20ImmutableCalls::InitializeWithUnderlying(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CErc20ImmutableCalls {
        fn from(var: InitializeCall) -> Self {
            CErc20ImmutableCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CErc20ImmutableCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CErc20ImmutableCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CErc20ImmutableCalls {
        fn from(var: IsCTokenCall) -> Self {
            CErc20ImmutableCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CErc20ImmutableCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CErc20ImmutableCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CErc20ImmutableCalls {
        fn from(var: MintCall) -> Self {
            CErc20ImmutableCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CErc20ImmutableCalls {
        fn from(var: NameCall) -> Self {
            CErc20ImmutableCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CErc20ImmutableCalls {
        fn from(var: PendingAdminCall) -> Self {
            CErc20ImmutableCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CErc20ImmutableCalls {
        fn from(var: RedeemCall) -> Self {
            CErc20ImmutableCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CErc20ImmutableCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CErc20ImmutableCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CErc20ImmutableCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CErc20ImmutableCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CErc20ImmutableCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CErc20ImmutableCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CErc20ImmutableCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CErc20ImmutableCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CErc20ImmutableCalls {
        fn from(var: SeizeCall) -> Self {
            CErc20ImmutableCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CErc20ImmutableCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CErc20ImmutableCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CErc20ImmutableCalls {
        fn from(var: SymbolCall) -> Self {
            CErc20ImmutableCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CErc20ImmutableCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CErc20ImmutableCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CErc20ImmutableCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CErc20ImmutableCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CErc20ImmutableCalls {
        fn from(var: TotalReservesCall) -> Self {
            CErc20ImmutableCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CErc20ImmutableCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CErc20ImmutableCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CErc20ImmutableCalls {
        fn from(var: TransferCall) -> Self {
            CErc20ImmutableCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CErc20ImmutableCalls {
        fn from(var: TransferFromCall) -> Self {
            CErc20ImmutableCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for CErc20ImmutableCalls {
        fn from(var: UnderlyingCall) -> Self {
            CErc20ImmutableCalls::Underlying(var)
        }
    }
    #[doc = "Container type for all return fields from the `_acceptAdmin` function with signature `_acceptAdmin()` and selector `[233, 199, 20, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AcceptAdminReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_addReserves` function with signature `_addReserves(uint256)` and selector `[62, 148, 16, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddReservesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_reduceReserves` function with signature `_reduceReserves(uint256)` and selector `[96, 26, 11, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ReduceReservesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setComptroller` function with signature `_setComptroller(address)` and selector `[69, 118, 181, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetComptrollerReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setInterestRateModel` function with signature `_setInterestRateModel(address)` and selector `[242, 179, 171, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetInterestRateModelReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setPendingAdmin` function with signature `_setPendingAdmin(address)` and selector `[183, 29, 26, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetPendingAdminReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `_setReserveFactor` function with signature `_setReserveFactor(uint256)` and selector `[252, 167, 130, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetReserveFactorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `accrualBlockNumber` function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AccrualBlockNumberReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `accrueInterest` function with signature `accrueInterest()` and selector `[166, 175, 237, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AccrueInterestReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `balanceOfUnderlying` function with signature `balanceOfUnderlying(address)` and selector `[58, 249, 230, 105]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfUnderlyingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrow` function with signature `borrow(uint256)` and selector `[197, 235, 234, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowBalanceCurrent` function with signature `borrowBalanceCurrent(address)` and selector `[23, 191, 223, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowBalanceCurrentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowBalanceStored` function with signature `borrowBalanceStored(address)` and selector `[149, 221, 145, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowBalanceStoredReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowIndex` function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowRatePerBlock` function with signature `borrowRatePerBlock()` and selector `[248, 249, 218, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `comptroller` function with signature `comptroller()` and selector `[95, 227, 181, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ComptrollerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `exchangeRateCurrent` function with signature `exchangeRateCurrent()` and selector `[189, 109, 137, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExchangeRateCurrentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `exchangeRateStored` function with signature `exchangeRateStored()` and selector `[24, 45, 240, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExchangeRateStoredReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAccountSnapshot` function with signature `getAccountSnapshot(address)` and selector `[195, 127, 104, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAccountSnapshotReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getCash` function with signature `getCash()` and selector `[59, 29, 33, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCashReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `interestRateModel` function with signature `interestRateModel()` and selector `[243, 253, 177, 90]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct InterestRateModelReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isCToken` function with signature `isCToken()` and selector `[254, 156, 68, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsCTokenReturn(pub bool);
    #[doc = "Container type for all return fields from the `liquidateBorrow` function with signature `liquidateBorrow(address,uint256,address)` and selector `[245, 227, 196, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LiquidateBorrowReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(uint256)` and selector `[160, 113, 45, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NameReturn(pub String);
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
    #[doc = "Container type for all return fields from the `redeem` function with signature `redeem(uint256)` and selector `[219, 0, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RedeemReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `redeemUnderlying` function with signature `redeemUnderlying(uint256)` and selector `[133, 42, 18, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RedeemUnderlyingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repayBorrow` function with signature `repayBorrow(uint256)` and selector `[14, 117, 39, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayBorrowReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repayBorrowBehalf` function with signature `repayBorrowBehalf(address,uint256)` and selector `[38, 8, 248, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayBorrowBehalfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `reserveFactorMantissa` function with signature `reserveFactorMantissa()` and selector `[23, 59, 153, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ReserveFactorMantissaReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `seize` function with signature `seize(address,address,uint256)` and selector `[178, 160, 47, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SeizeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `supplyRatePerBlock` function with signature `supplyRatePerBlock()` and selector `[174, 157, 112, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupplyRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalBorrows` function with signature `totalBorrows()` and selector `[71, 189, 55, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalBorrowsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `totalBorrowsCurrent` function with signature `totalBorrowsCurrent()` and selector `[115, 172, 238, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalBorrowsCurrentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `totalReserves` function with signature `totalReserves()` and selector `[143, 132, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalReservesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferFromReturn(pub bool);
    #[doc = "Container type for all return fields from the `underlying` function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UnderlyingReturn(pub ethers::core::types::Address);
}
