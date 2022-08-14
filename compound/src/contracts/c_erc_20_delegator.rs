pub use c_erc_20_delegator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod c_erc_20_delegator {
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
    #[doc = "CErc20Delegator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CERC20DELEGATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlying_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"becomeImplementationData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldImplementation\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newImplementation\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewImplementation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowResign\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"becomeImplementationData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegateToImplementation\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegateToViewImplementation\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract CTokenInterface\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CERC20DELEGATOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620019eb380380620019eb83398181016040526101408110156200003857600080fd5b81516020830151604080850151606086015160808701805193519597949692959194919392820192846401000000008211156200007457600080fd5b9083019060208201858111156200008a57600080fd5b8251640100000000811182820188101715620000a557600080fd5b82525081516020918201929091019080838360005b83811015620000d4578181015183820152602001620000ba565b50505050905090810190601f168015620001025780820380516001836020036101000a031916815260200191505b50604052602001805160405193929190846401000000008211156200012657600080fd5b9083019060208201858111156200013c57600080fd5b82516401000000008111828201881017156200015757600080fd5b82525081516020918201929091019080838360005b83811015620001865781810151838201526020016200016c565b50505050905090810190601f168015620001b45780820380516001836020036101000a031916815260200191505b50604081815260208301519083015160608401516080909401805192969195919284640100000000821115620001e957600080fd5b908301906020820185811115620001ff57600080fd5b82516401000000008111828201881017156200021a57600080fd5b82525081516020918201929091019080838360005b83811015620002495781810151838201526020016200022f565b50505050905090810190601f168015620002775780820380516001836020036101000a031916815260200191505b5060405250505033600360016101000a8154816001600160a01b0302191690836001600160a01b0316021790555062000424828b8b8b8b8b8b8b60405160240180886001600160a01b03166001600160a01b03168152602001876001600160a01b03166001600160a01b03168152602001866001600160a01b03166001600160a01b0316815260200185815260200180602001806020018460ff1660ff168152602001838103835286818151815260200191508051906020019080838360005b838110156200035157818101518382015260200162000337565b50505050905090810190601f1680156200037f5780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b83811015620003b45781810151838201526020016200039a565b50505050905090810190601f168015620003e25780820380516001836020036101000a031916815260200191505b5060408051601f198184030181529190526020810180516001600160e01b03908116631a31d46560e01b17909152909a50620004721698505050505050505050565b506200043c826000836001600160e01b036200053916565b5050600380546001600160a01b0390921661010002610100600160a81b0319909216919091179055506200071a95505050505050565b606060006060846001600160a01b0316846040518082805190602001908083835b60208310620004b45780518252601f19909201916020918201910162000493565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855af49150503d806000811462000516576040519150601f19603f3d011682016040523d82523d6000602084013e6200051b565b606091505b5091509150600082141562000531573d60208201fd5b949350505050565b60035461010090046001600160a01b03163314620005895760405162461bcd60e51b8152600401808060200182810382526039815260200180620019b26039913960400191505060405180910390fd5b8115620005cb576040805160048152602481019091526020810180516001600160e01b0390811663153ab50560e01b17909152620005c99190620006f016565b505b601280546001600160a01b038581166001600160a01b03198316179092556040516020602482018181528551604484015285519490931693620006a1938693909283926064909201919085019080838360005b83811015620006385781810151838201526020016200061e565b50505050905090810190601f168015620006665780820380516001836020036101000a031916815260200191505b5060408051601f198184030181529190526020810180516001600160e01b03908116630adccee560e31b17909152909350620006f016915050565b50601254604080516001600160a01b038085168252909216602083015280517fd604de94d45953f9138079ec1b82d533cb2160c906d1076d1f7ed54befbca97a9281900390910190a150505050565b60125460609062000714906001600160a01b0316836001600160e01b036200047216565b92915050565b611288806200072a6000396000f3fe6080604052600436106102c95760003560e01c806373acee9811610175578063bd6d894d116100dc578063f2b3abbd11610095578063f851a4401161006f578063f851a44014610983578063f8f9da2814610532578063fca7820b14610499578063fe9c44ae14610998576102c9565b8063f2b3abbd146104ea578063f3fdb15a1461092b578063f5e3c46214610940576102c9565b8063bd6d894d14610820578063c37f68e214610897578063c5ebeaec14610499578063db006a7514610499578063dd62ed3e146108f0578063e9c714f214610820576102c9565b8063a6afed951161012e578063a6afed9514610820578063a9059cbb1461044c578063aa5af0fd1461085f578063ae9d70b014610532578063b2a02ff114610874578063b71d1a0c146104ea576102c9565b806373acee9814610820578063852a12e3146104995780638f840ddd1461083557806395d89b411461084a57806395dd9193146107ed578063a0712d6814610499576102c9565b80633af9e66911610234578063555bcc40116101ed578063601a0bf1116101c7578063601a0bf1146104995780636c540baf146107c35780636f307dc3146107d857806370a08231146107ed576102c9565b8063555bcc40146106cf5780635c60da1b146107995780635fe3b567146107ae576102c9565b80633af9e669146104ea5780633b1d21a2146105325780633e941010146104995780634487152f146106095780634576b5db146104ea57806347bd3718146106ba576102c9565b806318160ddd1161028657806318160ddd1461051d578063182df0f51461053257806323b872dd146105475780632608f8181461058a57806326782247146105ad578063313ce567146105de576102c9565b806306fdde03146103115780630933c1ed1461039b578063095ea7b31461044c5780630e75270214610499578063173b9904146104d557806317bfdfbc146104ea575b34156103065760405162461bcd60e51b81526004018080602001828103825260378152602001806111e46037913960400191505060405180910390fd5b61030e6109ad565b50005b34801561031d57600080fd5b50610326610a35565b6040805160208082528351818301528351919283929083019185019080838360005b83811015610360578181015183820152602001610348565b50505050905090810190601f16801561038d5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b3480156103a757600080fd5b50610326600480360360208110156103be57600080fd5b810190602081018135600160201b8111156103d857600080fd5b8201836020820111156103ea57600080fd5b803590602001918460018302840111600160201b8311171561040b57600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550610ac2945050505050565b34801561045857600080fd5b506104856004803603604081101561046f57600080fd5b506001600160a01b038135169060200135610ae1565b604080519115158252519081900360200190f35b3480156104a557600080fd5b506104c3600480360360208110156104bc57600080fd5b5035610af2565b60408051918252519081900360200190f35b3480156104e157600080fd5b506104c3610b02565b3480156104f657600080fd5b506104c36004803603602081101561050d57600080fd5b50356001600160a01b0316610af2565b34801561052957600080fd5b506104c3610b08565b34801561053e57600080fd5b506104c3610b0e565b34801561055357600080fd5b506104856004803603606081101561056a57600080fd5b506001600160a01b03813581169160208101359091169060400135610b1c565b34801561059657600080fd5b506104c36004803603604081101561046f57600080fd5b3480156105b957600080fd5b506105c2610b2e565b604080516001600160a01b039092168252519081900360200190f35b3480156105ea57600080fd5b506105f3610b3d565b6040805160ff9092168252519081900360200190f35b34801561061557600080fd5b506103266004803603602081101561062c57600080fd5b810190602081018135600160201b81111561064657600080fd5b82018360208201111561065857600080fd5b803590602001918460018302840111600160201b8311171561067957600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550610b46945050505050565b3480156106c657600080fd5b506104c3610d65565b3480156106db57600080fd5b50610797600480360360608110156106f257600080fd5b6001600160a01b03823516916020810135151591810190606081016040820135600160201b81111561072357600080fd5b82018360208201111561073557600080fd5b803590602001918460018302840111600160201b8311171561075657600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550610d6b945050505050565b005b3480156107a557600080fd5b506105c2610f0e565b3480156107ba57600080fd5b506105c2610f1d565b3480156107cf57600080fd5b506104c3610f2c565b3480156107e457600080fd5b506105c2610f32565b3480156107f957600080fd5b506104c36004803603602081101561081057600080fd5b50356001600160a01b0316610f41565b34801561082c57600080fd5b506104c3610f4b565b34801561084157600080fd5b506104c3610f55565b34801561085657600080fd5b50610326610f5b565b34801561086b57600080fd5b506104c3610fb3565b34801561088057600080fd5b506104c36004803603606081101561056a57600080fd5b3480156108a357600080fd5b506108ca600480360360208110156108ba57600080fd5b50356001600160a01b0316610fb9565b604080519485526020850193909352838301919091526060830152519081900360800190f35b3480156108fc57600080fd5b506104c36004803603604081101561091357600080fd5b506001600160a01b0381358116916020013516610fcf565b34801561093757600080fd5b506105c2610fd9565b34801561094c57600080fd5b506104c36004803603606081101561096357600080fd5b506001600160a01b03813581169160208101359160409091013516610b1c565b34801561098f57600080fd5b506105c2610fe8565b3480156109a457600080fd5b50610485610ffc565b6012546040516060916000916001600160a01b0390911690829036908083838082843760405192019450600093509091505080830381855af49150503d8060008114610a15576040519150601f19603f3d011682016040523d82523d6000602084013e610a1a565b606091505b505090506040513d6000823e818015610a31573d82f35b3d82fd5b60018054604080516020600284861615610100026000190190941693909304601f81018490048402820184019092528181529291830182828015610aba5780601f10610a8f57610100808354040283529160200191610aba565b820191906000526020600020905b815481529060010190602001808311610a9d57829003601f168201915b505050505081565b601254606090610adb906001600160a01b031683611001565b92915050565b6000610aeb6109ad565b5092915050565b6000610afc6109ad565b50919050565b60085481565b600d5481565b6000610b186110c3565b5090565b6000610b266109ad565b509392505050565b6004546001600160a01b031681565b60035460ff1681565b606060006060306001600160a01b0316846040516024018080602001828103825283818151815260200191508051906020019080838360005b83811015610b97578181015183820152602001610b7f565b50505050905090810190601f168015610bc45780820380516001836020036101000a031916815260200191505b5060408051601f198184030181529181526020820180516001600160e01b0316630933c1ed60e01b178152905182519295509350839250908083835b60208310610c1f5780518252601f199092019160209182019101610c00565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855afa9150503d8060008114610c7f576040519150601f19603f3d011682016040523d82523d6000602084013e610c84565b606091505b50915091506000821415610c99573d60208201fd5b808060200190516020811015610cae57600080fd5b8101908080516040519392919084600160201b821115610ccd57600080fd5b908301906020820185811115610ce257600080fd5b8251600160201b811182820188101715610cfb57600080fd5b82525081516020918201929091019080838360005b83811015610d28578181015183820152602001610d10565b50505050905090810190601f168015610d555780820380516001836020036101000a031916815260200191505b5060405250505092505050919050565b600b5481565b60035461010090046001600160a01b03163314610db95760405162461bcd60e51b815260040180806020018281038252603981526020018061121b6039913960400191505060405180910390fd5b8115610df3576040805160048152602481019091526020810180516001600160e01b031663153ab50560e01b179052610df190610ac2565b505b601280546001600160a01b038581166001600160a01b03198316179092556040516020602482018181528551604484015285519490931693610ebf938693909283926064909201919085019080838360005b83811015610e5d578181015183820152602001610e45565b50505050905090810190601f168015610e8a5780820380516001836020036101000a031916815260200191505b5060408051601f198184030181529190526020810180516001600160e01b0316630adccee560e31b1790529250610ac2915050565b50601254604080516001600160a01b038085168252909216602083015280517fd604de94d45953f9138079ec1b82d533cb2160c906d1076d1f7ed54befbca97a9281900390910190a150505050565b6012546001600160a01b031681565b6005546001600160a01b031681565b60095481565b6011546001600160a01b031681565b6000610afc6110c3565b6000610b186109ad565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f81018490048402820184019092528181529291830182828015610aba5780601f10610a8f57610100808354040283529160200191610aba565b600a5481565b600080600080610fc76110c3565b509193509193565b6000610aeb6110c3565b6006546001600160a01b031681565b60035461010090046001600160a01b031681565b600181565b606060006060846001600160a01b0316846040518082805190602001908083835b602083106110415780518252601f199092019160209182019101611022565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855af49150503d80600081146110a1576040519150601f19603f3d011682016040523d82523d6000602084013e6110a6565b606091505b509150915060008214156110bb573d60208201fd5b949350505050565b60606000306001600160a01b03166000366040516024018080602001828103825284848281815260200192508082843760008382015260408051601f909201601f1990811690940182810390940182529283526020810180516001600160e01b0316630933c1ed60e01b17815292518151919750955085945091925081905083835b602083106111645780518252601f199092019160209182019101611145565b6001836020036101000a038019825116818451168082178552505050505050905001915050600060405180830381855afa9150503d80600081146111c4576040519150601f19603f3d011682016040523d82523d6000602084013e6111c9565b606091505b505090506040513d6000823e818015610a31573d60408301f3fe43457263323044656c656761746f723a66616c6c6261636b3a2063616e6e6f742073656e642076616c756520746f2066616c6c6261636b43457263323044656c656761746f723a3a5f736574496d706c656d656e746174696f6e3a2043616c6c6572206d7573742062652061646d696ea265627a7a72315820c91b6a11cfecfabc39414967ee20e39dc786aa9742d6556778dd29f21db96dee64736f6c6343000511003243457263323044656c656761746f723a3a5f736574496d706c656d656e746174696f6e3a2043616c6c6572206d7573742062652061646d696e" . parse () . expect ("invalid bytecode")
        });
    pub struct CErc20Delegator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CErc20Delegator<M> {
        fn clone(&self) -> Self {
            CErc20Delegator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CErc20Delegator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CErc20Delegator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CErc20Delegator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CErc20Delegator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CERC20DELEGATOR_ABI.clone(), client)
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
                CERC20DELEGATOR_ABI.clone(),
                CERC20DELEGATOR_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `_setImplementation` (0x555bcc40) function"]
        pub fn set_implementation(
            &self,
            implementation: ethers::core::types::Address,
            allow_resign: bool,
            become_implementation_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [85, 91, 204, 64],
                    (implementation, allow_resign, become_implementation_data),
                )
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
        #[doc = "Calls the contract's `delegateToImplementation` (0x0933c1ed) function"]
        pub fn delegate_to_implementation(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([9, 51, 193, 237], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateToViewImplementation` (0x4487152f) function"]
        pub fn delegate_to_view_implementation(
            &self,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([68, 135, 21, 47], data)
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
        #[doc = "Gets the contract's `NewImplementation` event"]
        pub fn new_implementation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewImplementationFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CErc20DelegatorEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CErc20Delegator<M> {
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
    #[ethevent(name = "NewImplementation", abi = "NewImplementation(address,address)")]
    pub struct NewImplementationFilter {
        pub old_implementation: ethers::core::types::Address,
        pub new_implementation: ethers::core::types::Address,
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
    pub enum CErc20DelegatorEvents {
        AccrueInterestFilter(AccrueInterestFilter),
        ApprovalFilter(ApprovalFilter),
        BorrowFilter(BorrowFilter),
        FailureFilter(FailureFilter),
        LiquidateBorrowFilter(LiquidateBorrowFilter),
        MintFilter(MintFilter),
        NewAdminFilter(NewAdminFilter),
        NewComptrollerFilter(NewComptrollerFilter),
        NewImplementationFilter(NewImplementationFilter),
        NewMarketInterestRateModelFilter(NewMarketInterestRateModelFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        NewReserveFactorFilter(NewReserveFactorFilter),
        RedeemFilter(RedeemFilter),
        RepayBorrowFilter(RepayBorrowFilter),
        ReservesAddedFilter(ReservesAddedFilter),
        ReservesReducedFilter(ReservesReducedFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for CErc20DelegatorEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewImplementationFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewImplementationFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewMarketInterestRateModelFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CErc20DelegatorEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CErc20DelegatorEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20DelegatorEvents::AccrueInterestFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::ApprovalFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::BorrowFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::FailureFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::MintFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewAdminFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewComptrollerFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewImplementationFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::RedeemFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::RepayBorrowFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::ReservesAddedFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::ReservesReducedFilter(element) => element.fmt(f),
                CErc20DelegatorEvents::TransferFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `_setImplementation` function with signature `_setImplementation(address,bool,bytes)` and selector `[85, 91, 204, 64]`"]
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
        name = "_setImplementation",
        abi = "_setImplementation(address,bool,bytes)"
    )]
    pub struct SetImplementationCall {
        pub implementation: ethers::core::types::Address,
        pub allow_resign: bool,
        pub become_implementation_data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `delegateToImplementation` function with signature `delegateToImplementation(bytes)` and selector `[9, 51, 193, 237]`"]
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
        name = "delegateToImplementation",
        abi = "delegateToImplementation(bytes)"
    )]
    pub struct DelegateToImplementationCall {
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `delegateToViewImplementation` function with signature `delegateToViewImplementation(bytes)` and selector `[68, 135, 21, 47]`"]
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
        name = "delegateToViewImplementation",
        abi = "delegateToViewImplementation(bytes)"
    )]
    pub struct DelegateToViewImplementationCall {
        pub data: ethers::core::types::Bytes,
    }
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
    #[doc = "Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
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
    pub enum CErc20DelegatorCalls {
        AcceptAdmin(AcceptAdminCall),
        AddReserves(AddReservesCall),
        ReduceReserves(ReduceReservesCall),
        SetComptroller(SetComptrollerCall),
        SetImplementation(SetImplementationCall),
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
        DelegateToImplementation(DelegateToImplementationCall),
        DelegateToViewImplementation(DelegateToViewImplementationCall),
        ExchangeRateCurrent(ExchangeRateCurrentCall),
        ExchangeRateStored(ExchangeRateStoredCall),
        GetAccountSnapshot(GetAccountSnapshotCall),
        GetCash(GetCashCall),
        Implementation(ImplementationCall),
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
    impl ethers::core::abi::AbiDecode for CErc20DelegatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::AddReserves(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SetImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DelegateToImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CErc20DelegatorCalls::DelegateToImplementation(decoded));
            }
            if let Ok(decoded) =
                <DelegateToViewImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CErc20DelegatorCalls::DelegateToViewImplementation(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <ImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Implementation(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::IsCToken(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20DelegatorCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20DelegatorCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20DelegatorCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CErc20DelegatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CErc20DelegatorCalls::AcceptAdmin(element) => element.encode(),
                CErc20DelegatorCalls::AddReserves(element) => element.encode(),
                CErc20DelegatorCalls::ReduceReserves(element) => element.encode(),
                CErc20DelegatorCalls::SetComptroller(element) => element.encode(),
                CErc20DelegatorCalls::SetImplementation(element) => element.encode(),
                CErc20DelegatorCalls::SetInterestRateModel(element) => element.encode(),
                CErc20DelegatorCalls::SetPendingAdmin(element) => element.encode(),
                CErc20DelegatorCalls::SetReserveFactor(element) => element.encode(),
                CErc20DelegatorCalls::AccrualBlockNumber(element) => element.encode(),
                CErc20DelegatorCalls::AccrueInterest(element) => element.encode(),
                CErc20DelegatorCalls::Admin(element) => element.encode(),
                CErc20DelegatorCalls::Allowance(element) => element.encode(),
                CErc20DelegatorCalls::Approve(element) => element.encode(),
                CErc20DelegatorCalls::BalanceOf(element) => element.encode(),
                CErc20DelegatorCalls::BalanceOfUnderlying(element) => element.encode(),
                CErc20DelegatorCalls::Borrow(element) => element.encode(),
                CErc20DelegatorCalls::BorrowBalanceCurrent(element) => element.encode(),
                CErc20DelegatorCalls::BorrowBalanceStored(element) => element.encode(),
                CErc20DelegatorCalls::BorrowIndex(element) => element.encode(),
                CErc20DelegatorCalls::BorrowRatePerBlock(element) => element.encode(),
                CErc20DelegatorCalls::Comptroller(element) => element.encode(),
                CErc20DelegatorCalls::Decimals(element) => element.encode(),
                CErc20DelegatorCalls::DelegateToImplementation(element) => element.encode(),
                CErc20DelegatorCalls::DelegateToViewImplementation(element) => element.encode(),
                CErc20DelegatorCalls::ExchangeRateCurrent(element) => element.encode(),
                CErc20DelegatorCalls::ExchangeRateStored(element) => element.encode(),
                CErc20DelegatorCalls::GetAccountSnapshot(element) => element.encode(),
                CErc20DelegatorCalls::GetCash(element) => element.encode(),
                CErc20DelegatorCalls::Implementation(element) => element.encode(),
                CErc20DelegatorCalls::InterestRateModel(element) => element.encode(),
                CErc20DelegatorCalls::IsCToken(element) => element.encode(),
                CErc20DelegatorCalls::LiquidateBorrow(element) => element.encode(),
                CErc20DelegatorCalls::Mint(element) => element.encode(),
                CErc20DelegatorCalls::Name(element) => element.encode(),
                CErc20DelegatorCalls::PendingAdmin(element) => element.encode(),
                CErc20DelegatorCalls::Redeem(element) => element.encode(),
                CErc20DelegatorCalls::RedeemUnderlying(element) => element.encode(),
                CErc20DelegatorCalls::RepayBorrow(element) => element.encode(),
                CErc20DelegatorCalls::RepayBorrowBehalf(element) => element.encode(),
                CErc20DelegatorCalls::ReserveFactorMantissa(element) => element.encode(),
                CErc20DelegatorCalls::Seize(element) => element.encode(),
                CErc20DelegatorCalls::SupplyRatePerBlock(element) => element.encode(),
                CErc20DelegatorCalls::Symbol(element) => element.encode(),
                CErc20DelegatorCalls::TotalBorrows(element) => element.encode(),
                CErc20DelegatorCalls::TotalBorrowsCurrent(element) => element.encode(),
                CErc20DelegatorCalls::TotalReserves(element) => element.encode(),
                CErc20DelegatorCalls::TotalSupply(element) => element.encode(),
                CErc20DelegatorCalls::Transfer(element) => element.encode(),
                CErc20DelegatorCalls::TransferFrom(element) => element.encode(),
                CErc20DelegatorCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CErc20DelegatorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20DelegatorCalls::AcceptAdmin(element) => element.fmt(f),
                CErc20DelegatorCalls::AddReserves(element) => element.fmt(f),
                CErc20DelegatorCalls::ReduceReserves(element) => element.fmt(f),
                CErc20DelegatorCalls::SetComptroller(element) => element.fmt(f),
                CErc20DelegatorCalls::SetImplementation(element) => element.fmt(f),
                CErc20DelegatorCalls::SetInterestRateModel(element) => element.fmt(f),
                CErc20DelegatorCalls::SetPendingAdmin(element) => element.fmt(f),
                CErc20DelegatorCalls::SetReserveFactor(element) => element.fmt(f),
                CErc20DelegatorCalls::AccrualBlockNumber(element) => element.fmt(f),
                CErc20DelegatorCalls::AccrueInterest(element) => element.fmt(f),
                CErc20DelegatorCalls::Admin(element) => element.fmt(f),
                CErc20DelegatorCalls::Allowance(element) => element.fmt(f),
                CErc20DelegatorCalls::Approve(element) => element.fmt(f),
                CErc20DelegatorCalls::BalanceOf(element) => element.fmt(f),
                CErc20DelegatorCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CErc20DelegatorCalls::Borrow(element) => element.fmt(f),
                CErc20DelegatorCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CErc20DelegatorCalls::BorrowBalanceStored(element) => element.fmt(f),
                CErc20DelegatorCalls::BorrowIndex(element) => element.fmt(f),
                CErc20DelegatorCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CErc20DelegatorCalls::Comptroller(element) => element.fmt(f),
                CErc20DelegatorCalls::Decimals(element) => element.fmt(f),
                CErc20DelegatorCalls::DelegateToImplementation(element) => element.fmt(f),
                CErc20DelegatorCalls::DelegateToViewImplementation(element) => element.fmt(f),
                CErc20DelegatorCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CErc20DelegatorCalls::ExchangeRateStored(element) => element.fmt(f),
                CErc20DelegatorCalls::GetAccountSnapshot(element) => element.fmt(f),
                CErc20DelegatorCalls::GetCash(element) => element.fmt(f),
                CErc20DelegatorCalls::Implementation(element) => element.fmt(f),
                CErc20DelegatorCalls::InterestRateModel(element) => element.fmt(f),
                CErc20DelegatorCalls::IsCToken(element) => element.fmt(f),
                CErc20DelegatorCalls::LiquidateBorrow(element) => element.fmt(f),
                CErc20DelegatorCalls::Mint(element) => element.fmt(f),
                CErc20DelegatorCalls::Name(element) => element.fmt(f),
                CErc20DelegatorCalls::PendingAdmin(element) => element.fmt(f),
                CErc20DelegatorCalls::Redeem(element) => element.fmt(f),
                CErc20DelegatorCalls::RedeemUnderlying(element) => element.fmt(f),
                CErc20DelegatorCalls::RepayBorrow(element) => element.fmt(f),
                CErc20DelegatorCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CErc20DelegatorCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CErc20DelegatorCalls::Seize(element) => element.fmt(f),
                CErc20DelegatorCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CErc20DelegatorCalls::Symbol(element) => element.fmt(f),
                CErc20DelegatorCalls::TotalBorrows(element) => element.fmt(f),
                CErc20DelegatorCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CErc20DelegatorCalls::TotalReserves(element) => element.fmt(f),
                CErc20DelegatorCalls::TotalSupply(element) => element.fmt(f),
                CErc20DelegatorCalls::Transfer(element) => element.fmt(f),
                CErc20DelegatorCalls::TransferFrom(element) => element.fmt(f),
                CErc20DelegatorCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CErc20DelegatorCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CErc20DelegatorCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<AddReservesCall> for CErc20DelegatorCalls {
        fn from(var: AddReservesCall) -> Self {
            CErc20DelegatorCalls::AddReserves(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CErc20DelegatorCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CErc20DelegatorCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CErc20DelegatorCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CErc20DelegatorCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetImplementationCall> for CErc20DelegatorCalls {
        fn from(var: SetImplementationCall) -> Self {
            CErc20DelegatorCalls::SetImplementation(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CErc20DelegatorCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CErc20DelegatorCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CErc20DelegatorCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CErc20DelegatorCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CErc20DelegatorCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CErc20DelegatorCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CErc20DelegatorCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CErc20DelegatorCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CErc20DelegatorCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CErc20DelegatorCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CErc20DelegatorCalls {
        fn from(var: AdminCall) -> Self {
            CErc20DelegatorCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CErc20DelegatorCalls {
        fn from(var: AllowanceCall) -> Self {
            CErc20DelegatorCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CErc20DelegatorCalls {
        fn from(var: ApproveCall) -> Self {
            CErc20DelegatorCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CErc20DelegatorCalls {
        fn from(var: BalanceOfCall) -> Self {
            CErc20DelegatorCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CErc20DelegatorCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CErc20DelegatorCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CErc20DelegatorCalls {
        fn from(var: BorrowCall) -> Self {
            CErc20DelegatorCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CErc20DelegatorCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CErc20DelegatorCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CErc20DelegatorCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CErc20DelegatorCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CErc20DelegatorCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CErc20DelegatorCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CErc20DelegatorCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CErc20DelegatorCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CErc20DelegatorCalls {
        fn from(var: ComptrollerCall) -> Self {
            CErc20DelegatorCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CErc20DelegatorCalls {
        fn from(var: DecimalsCall) -> Self {
            CErc20DelegatorCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DelegateToImplementationCall> for CErc20DelegatorCalls {
        fn from(var: DelegateToImplementationCall) -> Self {
            CErc20DelegatorCalls::DelegateToImplementation(var)
        }
    }
    impl ::std::convert::From<DelegateToViewImplementationCall> for CErc20DelegatorCalls {
        fn from(var: DelegateToViewImplementationCall) -> Self {
            CErc20DelegatorCalls::DelegateToViewImplementation(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CErc20DelegatorCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CErc20DelegatorCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CErc20DelegatorCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CErc20DelegatorCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CErc20DelegatorCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CErc20DelegatorCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CErc20DelegatorCalls {
        fn from(var: GetCashCall) -> Self {
            CErc20DelegatorCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<ImplementationCall> for CErc20DelegatorCalls {
        fn from(var: ImplementationCall) -> Self {
            CErc20DelegatorCalls::Implementation(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CErc20DelegatorCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CErc20DelegatorCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CErc20DelegatorCalls {
        fn from(var: IsCTokenCall) -> Self {
            CErc20DelegatorCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CErc20DelegatorCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CErc20DelegatorCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CErc20DelegatorCalls {
        fn from(var: MintCall) -> Self {
            CErc20DelegatorCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CErc20DelegatorCalls {
        fn from(var: NameCall) -> Self {
            CErc20DelegatorCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CErc20DelegatorCalls {
        fn from(var: PendingAdminCall) -> Self {
            CErc20DelegatorCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CErc20DelegatorCalls {
        fn from(var: RedeemCall) -> Self {
            CErc20DelegatorCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CErc20DelegatorCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CErc20DelegatorCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CErc20DelegatorCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CErc20DelegatorCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CErc20DelegatorCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CErc20DelegatorCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CErc20DelegatorCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CErc20DelegatorCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CErc20DelegatorCalls {
        fn from(var: SeizeCall) -> Self {
            CErc20DelegatorCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CErc20DelegatorCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CErc20DelegatorCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CErc20DelegatorCalls {
        fn from(var: SymbolCall) -> Self {
            CErc20DelegatorCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CErc20DelegatorCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CErc20DelegatorCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CErc20DelegatorCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CErc20DelegatorCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CErc20DelegatorCalls {
        fn from(var: TotalReservesCall) -> Self {
            CErc20DelegatorCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CErc20DelegatorCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CErc20DelegatorCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CErc20DelegatorCalls {
        fn from(var: TransferCall) -> Self {
            CErc20DelegatorCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CErc20DelegatorCalls {
        fn from(var: TransferFromCall) -> Self {
            CErc20DelegatorCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for CErc20DelegatorCalls {
        fn from(var: UnderlyingCall) -> Self {
            CErc20DelegatorCalls::Underlying(var)
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
    #[doc = "Container type for all return fields from the `delegateToImplementation` function with signature `delegateToImplementation(bytes)` and selector `[9, 51, 193, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DelegateToImplementationReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `delegateToViewImplementation` function with signature `delegateToViewImplementation(bytes)` and selector `[68, 135, 21, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DelegateToViewImplementationReturn(pub ethers::core::types::Bytes);
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
    #[doc = "Container type for all return fields from the `implementation` function with signature `implementation()` and selector `[92, 96, 218, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ImplementationReturn(pub ethers::core::types::Address);
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
