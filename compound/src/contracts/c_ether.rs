pub use cether_mod::*;
#[allow(clippy::too_many_arguments)]
mod cether_mod {
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
    #[doc = "CEther was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CETHER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CToken\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CETHER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b506040516200567e3803806200567e833981810160405260e08110156200003757600080fd5b8151602083015160408085015160608601805192519496939591949391820192846401000000008211156200006b57600080fd5b9083019060208201858111156200008157600080fd5b82516401000000008111828201881017156200009c57600080fd5b82525081516020918201929091019080838360005b83811015620000cb578181015183820152602001620000b1565b50505050905090810190601f168015620000f95780820380516001836020036101000a031916815260200191505b50604052602001805160405193929190846401000000008211156200011d57600080fd5b9083019060208201858111156200013357600080fd5b82516401000000008111828201881017156200014e57600080fd5b82525081516020918201929091019080838360005b838110156200017d57818101518382015260200162000163565b50505050905090810190601f168015620001ab5780820380516001836020036101000a031916815260200191505b506040908152602082015191015160038054610100600160a81b03191633610100021790559092509050620001e587878787878762000218565b600380546001600160a01b0390921661010002610100600160a81b03199092169190911790555062000853945050505050565b60035461010090046001600160a01b03163314620002685760405162461bcd60e51b8152600401808060200182810382526024815260200180620055e56024913960400191505060405180910390fd5b600954158015620002795750600a54155b620002b65760405162461bcd60e51b8152600401808060200182810382526023815260200180620056096023913960400191505060405180910390fd5b600784905583620002f95760405162461bcd60e51b81526004018080602001828103825260308152602001806200562c6030913960400191505060405180910390fd5b60006200030f876001600160e01b036200042e16565b9050801562000365576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b620003786001600160e01b036200059616565b600955670de0b6b3a7640000600a556200039b866001600160e01b036200059b16565b90508015620003dc5760405162461bcd60e51b81526004018080602001828103825260228152602001806200565c6022913960400191505060405180910390fd5b8351620003f1906001906020870190620007b1565b50825162000407906002906020860190620007b1565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b60035460009061010090046001600160a01b031633146200046857620004606001603f6001600160e01b036200074116565b905062000591565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b158015620004ae57600080fd5b505afa158015620004c3573d6000803e3d6000fd5b505050506040513d6020811015620004da57600080fd5b50516200052e576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9150505b919050565b435b90565b600354600090819061010090046001600160a01b03163314620005d857620005cf600160426001600160e01b036200074116565b91505062000591565b620005eb6001600160e01b036200059616565b600954146200060b57620005cf600a60416001600160e01b036200074116565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b1580156200065d57600080fd5b505afa15801562000672573d6000803e3d6000fd5b505050506040513d60208110156200068957600080fd5b5051620006dd576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a160006200058d565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa08360108111156200077157fe5b8360508111156200077e57fe5b604080519283526020830191909152600082820152519081900360600190a1826010811115620007aa57fe5b9392505050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f10620007f457805160ff191683800117855562000824565b8280016001018555821562000824579182015b828111156200082457825182559160200191906001019062000807565b506200083292915062000836565b5090565b6200059891905b808211156200083257600081556001016200083d565b614d8280620008636000396000f3fe6080604052600436106102725760003560e01c806395d89b411161014f578063c37f68e2116100c1578063f2b3abbd1161007a578063f2b3abbd14610a22578063f3fdb15a14610a55578063f851a44014610a6a578063f8f9da2814610a7f578063fca7820b14610a94578063fe9c44ae14610abe57610272565b8063c37f68e2146108ff578063c5ebeaec14610958578063db006a7514610982578063dd62ed3e146109ac578063e5974619146109e7578063e9c714f214610a0d57610272565b8063aa5af0fd11610113578063aa5af0fd1461081c578063aae40a2a14610831578063ae9d70b01461085f578063b2a02ff114610874578063b71d1a0c146108b7578063bd6d894d146108ea57610272565b806395d89b411461062757806395dd91931461063c57806399d8c1b41461066f578063a6afed95146107ce578063a9059cbb146107e357610272565b80633b1d21a2116101e8578063601a0bf1116101ac578063601a0bf1146105615780636c540baf1461058b57806370a08231146105a057806373acee98146105d3578063852a12e3146105e85780638f840ddd1461061257610272565b80633b1d21a2146104e75780634576b5db146104fc57806347bd37181461052f5780634e4d9fea146105445780635fe3b5671461054c57610272565b806318160ddd1161023a57806318160ddd146103eb578063182df0f51461040057806323b872dd146104155780632678224714610458578063313ce567146104895780633af9e669146104b457610272565b806306fdde03146102b0578063095ea7b31461033a5780631249c58b14610387578063173b99041461039157806317bfdfbc146103b8575b600061027d34610ad3565b5090506102ad816040518060400160405280600b81526020016a1b5a5b9d0819985a5b195960aa1b815250610b7b565b50005b3480156102bc57600080fd5b506102c5610d7b565b6040805160208082528351818301528351919283929083019185019080838360005b838110156102ff5781810151838201526020016102e7565b50505050905090810190601f16801561032c5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b34801561034657600080fd5b506103736004803603604081101561035d57600080fd5b506001600160a01b038135169060200135610e08565b604080519115158252519081900360200190f35b61038f610e75565b005b34801561039d57600080fd5b506103a6610eb3565b60408051918252519081900360200190f35b3480156103c457600080fd5b506103a6600480360360208110156103db57600080fd5b50356001600160a01b0316610eb9565b3480156103f757600080fd5b506103a6610f79565b34801561040c57600080fd5b506103a6610f7f565b34801561042157600080fd5b506103736004803603606081101561043857600080fd5b506001600160a01b03813581169160208101359091169060400135610fe2565b34801561046457600080fd5b5061046d611054565b604080516001600160a01b039092168252519081900360200190f35b34801561049557600080fd5b5061049e611063565b6040805160ff9092168252519081900360200190f35b3480156104c057600080fd5b506103a6600480360360208110156104d757600080fd5b50356001600160a01b031661106c565b3480156104f357600080fd5b506103a6611124565b34801561050857600080fd5b506103a66004803603602081101561051f57600080fd5b50356001600160a01b0316611133565b34801561053b57600080fd5b506103a6611288565b61038f61128e565b34801561055857600080fd5b5061046d6112d0565b34801561056d57600080fd5b506103a66004803603602081101561058457600080fd5b50356112df565b34801561059757600080fd5b506103a661137a565b3480156105ac57600080fd5b506103a6600480360360208110156105c357600080fd5b50356001600160a01b0316611380565b3480156105df57600080fd5b506103a661139b565b3480156105f457600080fd5b506103a66004803603602081101561060b57600080fd5b5035611451565b34801561061e57600080fd5b506103a661145c565b34801561063357600080fd5b506102c5611462565b34801561064857600080fd5b506103a66004803603602081101561065f57600080fd5b50356001600160a01b03166114ba565b34801561067b57600080fd5b5061038f600480360360c081101561069257600080fd5b6001600160a01b038235811692602081013590911691604082013591908101906080810160608201356401000000008111156106cd57600080fd5b8201836020820111156106df57600080fd5b8035906020019184600183028401116401000000008311171561070157600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929594936020810193503591505064010000000081111561075457600080fd5b82018360208201111561076657600080fd5b8035906020019184600183028401116401000000008311171561078857600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505050903560ff1691506115179050565b3480156107da57600080fd5b506103a66116fe565b3480156107ef57600080fd5b506103736004803603604081101561080657600080fd5b506001600160a01b038135169060200135611a56565b34801561082857600080fd5b506103a6611ac7565b61038f6004803603604081101561084757600080fd5b506001600160a01b0381358116916020013516611acd565b34801561086b57600080fd5b506103a6611b1a565b34801561088057600080fd5b506103a66004803603606081101561089757600080fd5b506001600160a01b03813581169160208101359091169060400135611bb9565b3480156108c357600080fd5b506103a6600480360360208110156108da57600080fd5b50356001600160a01b0316611c2a565b3480156108f657600080fd5b506103a6611cb6565b34801561090b57600080fd5b506109326004803603602081101561092257600080fd5b50356001600160a01b0316611d72565b604080519485526020850193909352838301919091526060830152519081900360800190f35b34801561096457600080fd5b506103a66004803603602081101561097b57600080fd5b5035611e07565b34801561098e57600080fd5b506103a6600480360360208110156109a557600080fd5b5035611e12565b3480156109b857600080fd5b506103a6600480360360408110156109cf57600080fd5b506001600160a01b0381358116916020013516611e1d565b61038f600480360360208110156109fd57600080fd5b50356001600160a01b0316611e48565b348015610a1957600080fd5b506103a6611e96565b348015610a2e57600080fd5b506103a660048036036020811015610a4557600080fd5b50356001600160a01b0316611f99565b348015610a6157600080fd5b5061046d611fd3565b348015610a7657600080fd5b5061046d611fe2565b348015610a8b57600080fd5b506103a6611ff6565b348015610aa057600080fd5b506103a660048036036020811015610ab757600080fd5b503561205a565b348015610aca57600080fd5b506103736120d8565b60008054819060ff16610b1a576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610b2c6116fe565b90508015610b5757610b4a816010811115610b4357fe5b601e6120dd565b925060009150610b679050565b610b613385612143565b92509250505b6000805460ff191660011790559092909150565b81610b8557610d77565b606081516005016040519080825280601f01601f191660200182016040528015610bb6576020820181803883390190505b50905060005b8251811015610c0757828181518110610bd157fe5b602001015160f81c60f81b828281518110610be857fe5b60200101906001600160f81b031916908160001a905350600101610bbc565b8151600160fd1b90839083908110610c1b57fe5b60200101906001600160f81b031916908160001a905350602860f81b828260010181518110610c4657fe5b60200101906001600160f81b031916908160001a905350600a840460300160f81b828260020181518110610c7657fe5b60200101906001600160f81b031916908160001a905350600a840660300160f81b828260030181518110610ca657fe5b60200101906001600160f81b031916908160001a905350602960f81b828260040181518110610cd157fe5b60200101906001600160f81b031916908160001a905350818415610d735760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b83811015610d38578181015183820152602001610d20565b50505050905090810190601f168015610d655780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5050505b5050565b60018054604080516020600284861615610100026000190190941693909304601f81018490048402820184019092528181529291830182828015610e005780601f10610dd557610100808354040283529160200191610e00565b820191906000526020600020905b815481529060010190602001808311610de357829003601f168201915b505050505081565b336000818152600f602090815260408083206001600160a01b03871680855290835281842086905581518681529151939493909284927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925929081900390910190a360019150505b92915050565b6000610e8034610ad3565b509050610eb0816040518060400160405280600b81526020016a1b5a5b9d0819985a5b195960aa1b815250610b7b565b50565b60085481565b6000805460ff16610efe576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155610f106116fe565b14610f5b576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b610f64826114ba565b90505b6000805460ff19166001179055919050565b600d5481565b6000806000610f8c6125a3565b90925090506000826003811115610f9f57fe5b14610fdb5760405162461bcd60e51b8152600401808060200182810382526035815260200180614c996035913960400191505060405180910390fd5b9150505b90565b6000805460ff16611027576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff1916815561103d33868686612652565b1490506000805460ff191660011790559392505050565b6004546001600160a01b031681565b60035460ff1681565b6000611076614987565b6040518060200160405280611089611cb6565b90526001600160a01b0384166000908152600e60205260408120549192509081906110b5908490612962565b909250905060008260038111156110c857fe5b1461111a576040805162461bcd60e51b815260206004820152601f60248201527f62616c616e636520636f756c64206e6f742062652063616c63756c6174656400604482015290519081900360640190fd5b925050505b919050565b600061112e6129b5565b905090565b60035460009061010090046001600160a01b03163314611160576111596001603f6120dd565b905061111f565b60055460408051623f1ee960e11b815290516001600160a01b0392831692851691627e3dd2916004808301926020929190829003018186803b1580156111a557600080fd5b505afa1580156111b9573d6000803e3d6000fd5b505050506040513d60208110156111cf57600080fd5b5051611222576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600580546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517f7ac369dbd14fa5ea3f473ed67cc9d598964a77501540ba6751eb0b3decf5870d9281900390910190a160005b9392505050565b600b5481565b6000611299346129e1565b509050610eb081604051806040016040528060128152602001711c995c185e509bdc9c9bddc819985a5b195960721b815250610b7b565b6005546001600160a01b031681565b6000805460ff16611324576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556113366116fe565b9050801561135c5761135481601081111561134d57fe5b60306120dd565b915050610f67565b61136583612a63565b9150506000805460ff19166001179055919050565b60095481565b6001600160a01b03166000908152600e602052604090205490565b6000805460ff166113e0576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556113f26116fe565b1461143d576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b50600b546000805460ff1916600117905590565b6000610e6f82612b96565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f81018490048402820184019092528181529291830182828015610e005780601f10610dd557610100808354040283529160200191610e00565b60008060006114c884612c17565b909250905060008260038111156114db57fe5b146112815760405162461bcd60e51b8152600401808060200182810382526037815260200180614ba46037913960400191505060405180910390fd5b60035461010090046001600160a01b031633146115655760405162461bcd60e51b8152600401808060200182810382526024815260200180614ae06024913960400191505060405180910390fd5b6009541580156115755750600a54155b6115b05760405162461bcd60e51b8152600401808060200182810382526023815260200180614b046023913960400191505060405180910390fd5b6007849055836115f15760405162461bcd60e51b8152600401808060200182810382526030815260200180614b276030913960400191505060405180910390fd5b60006115fc87611133565b90508015611651576040805162461bcd60e51b815260206004820152601a60248201527f73657474696e6720636f6d7074726f6c6c6572206661696c6564000000000000604482015290519081900360640190fd5b611659612ccb565b600955670de0b6b3a7640000600a5561167186612ccf565b905080156116b05760405162461bcd60e51b8152600401808060200182810382526022815260200180614b576022913960400191505060405180910390fd5b83516116c390600190602087019061499a565b5082516116d790600290602086019061499a565b50506003805460ff90921660ff199283161790556000805490911660011790555050505050565b600080611709612ccb565b6009549091508082141561172257600092505050610fdf565b600061172c6129b5565b600b54600c54600a54600654604080516315f2405360e01b815260048101879052602481018690526044810185905290519596509394929391926000926001600160a01b03909216916315f24053916064808301926020929190829003018186803b15801561179a57600080fd5b505afa1580156117ae573d6000803e3d6000fd5b505050506040513d60208110156117c457600080fd5b5051905065048c27395000811115611823576040805162461bcd60e51b815260206004820152601c60248201527f626f72726f772072617465206973206162737572646c79206869676800000000604482015290519081900360640190fd5b6000806118308989612e44565b9092509050600082600381111561184357fe5b14611895576040805162461bcd60e51b815260206004820152601f60248201527f636f756c64206e6f742063616c63756c61746520626c6f636b2064656c746100604482015290519081900360640190fd5b61189d614987565b6000806000806118bb60405180602001604052808a81525087612e67565b909750945060008760038111156118ce57fe5b14611900576118eb600960068960038111156118e657fe5b612ecf565b9e505050505050505050505050505050610fdf565b61190a858c612962565b9097509350600087600381111561191d57fe5b14611935576118eb600960018960038111156118e657fe5b61193f848c612f35565b9097509250600087600381111561195257fe5b1461196a576118eb600960048960038111156118e657fe5b6119856040518060200160405280600854815250858c612f5b565b9097509150600087600381111561199857fe5b146119b0576118eb600960058960038111156118e657fe5b6119bb858a8b612f5b565b909750905060008760038111156119ce57fe5b146119e6576118eb600960038960038111156118e657fe5b60098e9055600a819055600b839055600c829055604080518d8152602081018690528082018390526060810185905290517f4dec04e750ca11537cabcd8a9eab06494de08da3735bc8871cd41250e190bc049181900360800190a160009e50505050505050505050505050505090565b6000805460ff16611a9b576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611ab133338686612652565b1490506000805460ff1916600117905592915050565b600a5481565b6000611ada833484612fb7565b509050611b1581604051806040016040528060168152602001751b1a5c5d5a59185d19509bdc9c9bddc819985a5b195960521b815250610b7b565b505050565b6006546000906001600160a01b031663b8168816611b366129b5565b600b54600c546008546040518563ffffffff1660e01b81526004018085815260200184815260200183815260200182815260200194505050505060206040518083038186803b158015611b8857600080fd5b505afa158015611b9c573d6000803e3d6000fd5b505050506040513d6020811015611bb257600080fd5b5051905090565b6000805460ff16611bfe576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19169055611c14338585856130e9565b90506000805460ff191660011790559392505050565b60035460009061010090046001600160a01b03163314611c5057611159600160456120dd565b600480546001600160a01b038481166001600160a01b0319831681179093556040805191909216808252602082019390935281517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a9929181900390910190a16000611281565b6000805460ff16611cfb576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155611d0d6116fe565b14611d58576040805162461bcd60e51b81526020600482015260166024820152751858d8dc9d59481a5b9d195c995cdd0819985a5b195960521b604482015290519081900360640190fd5b611d60610f7f565b90506000805460ff1916600117905590565b6001600160a01b0381166000908152600e6020526040812054819081908190818080611d9d89612c17565b935090506000816003811115611daf57fe5b14611dcd5760095b975060009650869550859450611e009350505050565b611dd56125a3565b925090506000816003811115611de757fe5b14611df3576009611db7565b5060009650919450925090505b9193509193565b6000610e6f8261334f565b6000610e6f826133ce565b6001600160a01b039182166000908152600f6020908152604080832093909416825291909152205490565b6000611e548234613448565b509050610d77816040518060400160405280601881526020017f7265706179426f72726f77426568616c66206661696c65640000000000000000815250610b7b565b6004546000906001600160a01b031633141580611eb1575033155b15611ec957611ec2600160006120dd565b9050610fdf565b60038054600480546001600160a01b03818116610100818102610100600160a81b0319871617968790556001600160a01b031990931690935560408051948390048216808652929095041660208401528351909391927ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc92908290030190a1600454604080516001600160a01b038085168252909216602083015280517fca4f2f25d0898edd99413412fb94012f9e54ec8142f9b093e7720646a95b16a99281900390910190a160009250505090565b600080611fa46116fe565b90508015611fca57611fc2816010811115611fbb57fe5b60406120dd565b91505061111f565b61128183612ccf565b6006546001600160a01b031681565b60035461010090046001600160a01b031681565b6006546000906001600160a01b03166315f240536120126129b5565b600b54600c546040518463ffffffff1660e01b815260040180848152602001838152602001828152602001935050505060206040518083038186803b158015611b8857600080fd5b6000805460ff1661209f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556120b16116fe565b905080156120cf576113548160108111156120c857fe5b60466120dd565b611365836134f3565b600181565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa083601081111561210c57fe5b83605081111561211857fe5b604080519283526020830191909152600082820152519081900360600190a182601081111561128157fe5b60055460408051634ef4c3e160e01b81523060048201526001600160a01b03858116602483015260448201859052915160009384938493911691634ef4c3e19160648082019260209290919082900301818787803b1580156121a457600080fd5b505af11580156121b8573d6000803e3d6000fd5b505050506040513d60208110156121ce57600080fd5b5051905080156121f2576121e56003601f83612ecf565b92506000915061259c9050565b6121fa612ccb565b6009541461220e576121e5600a60226120dd565b612216614a18565b61221e6125a3565b604083018190526020830182600381111561223557fe5b600381111561224057fe5b905250600090508160200151600381111561225757fe5b146122815761227360096021836020015160038111156118e657fe5b93506000925061259c915050565b61228b868661359b565b60c08201819052604080516020810182529083015181526122ac9190613637565b60608301819052602083018260038111156122c357fe5b60038111156122ce57fe5b90525060009050816020015160038111156122e557fe5b14612337576040805162461bcd60e51b815260206004820181905260248201527f4d494e545f45584348414e47455f43414c43554c4154494f4e5f4641494c4544604482015290519081900360640190fd5b612347600d548260600151612f35565b608083018190526020830182600381111561235e57fe5b600381111561236957fe5b905250600090508160200151600381111561238057fe5b146123bc5760405162461bcd60e51b8152600401808060200182810382526028815260200180614cce6028913960400191505060405180910390fd5b6001600160a01b0386166000908152600e602052604090205460608201516123e49190612f35565b60a08301819052602083018260038111156123fb57fe5b600381111561240657fe5b905250600090508160200151600381111561241d57fe5b146124595760405162461bcd60e51b815260040180806020018281038252602b815260200180614b79602b913960400191505060405180910390fd5b6080810151600d5560a08101516001600160a01b0387166000818152600e60209081526040918290209390935560c084015160608086015183519485529484019190915282820193909352517f4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f929181900390910190a1606081015160408051918252516001600160a01b038816913091600080516020614c158339815191529181900360200190a360055460c08201516060830151604080516341c728b960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916341c728b991608480830192600092919082900301818387803b15801561256f57600080fd5b505af1158015612583573d6000803e3d6000fd5b5060009250612590915050565b8160c001519350935050505b9250929050565b600d546000908190806125be5750506007546000915061264e565b60006125c86129b5565b905060006125d4614987565b60006125e584600b54600c5461364e565b9350905060008160038111156125f757fe5b1461260c5795506000945061264e9350505050565b612616838661368c565b92509050600081600381111561262857fe5b1461263d5795506000945061264e9350505050565b505160009550935061264e92505050565b9091565b600554604080516317b9b84b60e31b81523060048201526001600160a01b03868116602483015285811660448301526064820185905291516000938493169163bdcdc25891608480830192602092919082900301818787803b1580156126b757600080fd5b505af11580156126cb573d6000803e3d6000fd5b505050506040513d60208110156126e157600080fd5b505190508015612700576126f86003604a83612ecf565b91505061295a565b836001600160a01b0316856001600160a01b03161415612726576126f86002604b6120dd565b60006001600160a01b038781169087161415612745575060001961276d565b506001600160a01b038086166000908152600f60209081526040808320938a16835292905220545b60008060008061277d8589612e44565b9094509250600084600381111561279057fe5b146127ae576127a16009604b6120dd565b965050505050505061295a565b6001600160a01b038a166000908152600e60205260409020546127d19089612e44565b909450915060008460038111156127e457fe5b146127f5576127a16009604c6120dd565b6001600160a01b0389166000908152600e60205260409020546128189089612f35565b9094509050600084600381111561282b57fe5b1461283c576127a16009604d6120dd565b6001600160a01b03808b166000908152600e6020526040808220859055918b168152208190556000198514612894576001600160a01b03808b166000908152600f60209081526040808320938f168352929052208390555b886001600160a01b03168a6001600160a01b0316600080516020614c158339815191528a6040518082815260200191505060405180910390a36005546040805163352b4a3f60e11b81523060048201526001600160a01b038d811660248301528c81166044830152606482018c905291519190921691636a56947e91608480830192600092919082900301818387803b15801561293057600080fd5b505af1158015612944573d6000803e3d6000fd5b5060009250612951915050565b96505050505050505b949350505050565b600080600061296f614987565b6129798686612e67565b9092509050600082600381111561298c57fe5b1461299d575091506000905061259c565b60006129a88261373c565b9350935050509250929050565b60008060006129c44734612e44565b909250905060008260038111156129d757fe5b14610fdb57600080fd5b60008054819060ff16612a28576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612a3a6116fe565b90508015612a5857610b4a816010811115612a5157fe5b60366120dd565b610b6133338661374b565b600354600090819061010090046001600160a01b03163314612a8b57611fc2600160316120dd565b612a93612ccb565b60095414612aa757611fc2600a60336120dd565b82612ab06129b5565b1015612ac257611fc2600e60326120dd565b600c54831115612ad857611fc2600260346120dd565b50600c5482810390811115612b1e5760405162461bcd60e51b8152600401808060200182810382526024815260200180614d2a6024913960400191505060405180910390fd5b600c819055600354612b3e9061010090046001600160a01b031684613b30565b600354604080516101009092046001600160a01b0316825260208201859052818101839052517f3bad0c59cf2f06e7314077049f48a93578cd16f5ef92329f1dab1420a99c177e916060908290030190a16000611281565b6000805460ff16612bdb576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff19168155612bed6116fe565b90508015612c0b57611354816010811115612c0457fe5b60276120dd565b61136533600085613b66565b6001600160a01b038116600090815260106020526040812080548291829182918291612c4e575060009450849350612cc692505050565b612c5e8160000154600a5461402d565b90945092506000846003811115612c7157fe5b14612c86575091935060009250612cc6915050565b612c9483826001015461406c565b90945091506000846003811115612ca757fe5b14612cbc575091935060009250612cc6915050565b5060009450925050505b915091565b4390565b600354600090819061010090046001600160a01b03163314612cf757611fc2600160426120dd565b612cff612ccb565b60095414612d1357611fc2600a60416120dd565b600660009054906101000a90046001600160a01b03169050826001600160a01b0316632191f92a6040518163ffffffff1660e01b815260040160206040518083038186803b158015612d6457600080fd5b505afa158015612d78573d6000803e3d6000fd5b505050506040513d6020811015612d8e57600080fd5b5051612de1576040805162461bcd60e51b815260206004820152601c60248201527f6d61726b6572206d6574686f642072657475726e65642066616c736500000000604482015290519081900360640190fd5b600680546001600160a01b0319166001600160a01b03858116918217909255604080519284168352602083019190915280517fedffc32e068c7c95dfd4bdfd5c4d939a084d6b11c4199eac8436ed234d72f9269281900390910190a16000611281565b600080838311612e5b57506000905081830361259c565b5060039050600061259c565b6000612e71614987565b600080612e8286600001518661402d565b90925090506000826003811115612e9557fe5b14612eb45750604080516020810190915260008152909250905061259c565b60408051602081019091529081526000969095509350505050565b60007f45b96fe442630264581b197e84bbada861235052c5a1aadfff9ea4e40a969aa0846010811115612efe57fe5b846050811115612f0a57fe5b604080519283526020830191909152818101859052519081900360600190a183601081111561295a57fe5b600080838301848110612f4d5760009250905061259c565b50600291506000905061259c565b6000806000612f68614987565b612f728787612e67565b90925090506000826003811115612f8557fe5b14612f965750915060009050612faf565b612fa8612fa28261373c565b86612f35565b9350935050505b935093915050565b60008054819060ff16612ffe576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556130106116fe565b9050801561303b5761302e81601081111561302757fe5b600f6120dd565b9250600091506130d29050565b836001600160a01b031663a6afed956040518163ffffffff1660e01b8152600401602060405180830381600087803b15801561307657600080fd5b505af115801561308a573d6000803e3d6000fd5b505050506040513d60208110156130a057600080fd5b5051905080156130c05761302e8160108111156130b957fe5b60106120dd565b6130cc33878787614097565b92509250505b6000805460ff191660011790559094909350915050565b6005546040805163d02f735160e01b81523060048201526001600160a01b038781166024830152868116604483015285811660648301526084820185905291516000938493169163d02f73519160a480830192602092919082900301818787803b15801561315657600080fd5b505af115801561316a573d6000803e3d6000fd5b505050506040513d602081101561318057600080fd5b505190508015613197576126f86003601b83612ecf565b846001600160a01b0316846001600160a01b031614156131bd576126f86006601c6120dd565b6001600160a01b0384166000908152600e6020526040812054819081906131e49087612e44565b909350915060008360038111156131f757fe5b1461321a5761320f6009601a8560038111156118e657fe5b94505050505061295a565b6001600160a01b0388166000908152600e602052604090205461323d9087612f35565b9093509050600083600381111561325057fe5b146132685761320f600960198560038111156118e657fe5b6001600160a01b038088166000818152600e60209081526040808320879055938c168083529184902085905583518a815293519193600080516020614c15833981519152929081900390910190a360055460408051636d35bf9160e01b81523060048201526001600160a01b038c811660248301528b811660448301528a81166064830152608482018a905291519190921691636d35bf919160a480830192600092919082900301818387803b15801561332157600080fd5b505af1158015613335573d6000803e3d6000fd5b5060009250613342915050565b9998505050505050505050565b6000805460ff16613394576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556133a66116fe565b905080156133c4576113548160108111156133bd57fe5b60086120dd565b611365338461461a565b6000805460ff16613413576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556134256116fe565b9050801561343c57611354816010811115612c0457fe5b61136533846000613b66565b60008054819060ff1661348f576040805162461bcd60e51b815260206004820152600a6024820152691c994b595b9d195c995960b21b604482015290519081900360640190fd5b6000805460ff191681556134a16116fe565b905080156134cc576134bf8160108111156134b857fe5b60356120dd565b9250600091506134dd9050565b6134d733868661374b565b92509250505b6000805460ff1916600117905590939092509050565b60035460009061010090046001600160a01b0316331461351957611159600160476120dd565b613521612ccb565b6009541461353557611159600a60486120dd565b670de0b6b3a764000082111561355157611159600260496120dd565b6008805490839055604080518281526020810185905281517faaa68312e2ea9d50e16af5068410ab56e1a1fd06037b1a35664812c30f821460929181900390910190a16000611281565b6000336001600160a01b038416146135ec576040805162461bcd60e51b815260206004820152600f60248201526e0e6cadcc8cae440dad2e6dac2e8c6d608b1b604482015290519081900360640190fd5b813414613631576040805162461bcd60e51b815260206004820152600e60248201526d0ecc2d8eaca40dad2e6dac2e8c6d60931b604482015290519081900360640190fd5b50919050565b6000806000613644614987565b6129798686614928565b60008060008061365e8787612f35565b9092509050600082600381111561367157fe5b146136825750915060009050612faf565b612fa88186612e44565b6000613696614987565b6000806136ab86670de0b6b3a764000061402d565b909250905060008260038111156136be57fe5b146136dd5750604080516020810190915260008152909250905061259c565b6000806136ea838861406c565b909250905060008260038111156136fd57fe5b1461371f5750604080516020810190915260008152909450925061259c915050565b604080516020810190915290815260009890975095505050505050565b51670de0b6b3a7640000900490565b60055460408051631200453160e11b81523060048201526001600160a01b0386811660248301528581166044830152606482018590529151600093849384939116916324008a629160848082019260209290919082900301818787803b1580156137b457600080fd5b505af11580156137c8573d6000803e3d6000fd5b505050506040513d60208110156137de57600080fd5b505190508015613802576137f56003603883612ecf565b925060009150612faf9050565b61380a612ccb565b6009541461381e576137f5600a60396120dd565b613826614a56565b6001600160a01b038616600090815260106020526040902060010154606082015261385086612c17565b608083018190526020830182600381111561386757fe5b600381111561387257fe5b905250600090508160200151600381111561388957fe5b146138b3576138a560096037836020015160038111156118e657fe5b935060009250612faf915050565b6000198514156138cc57608081015160408201526138d4565b604081018590525b6138e287826040015161359b565b60e0820181905260808201516138f791612e44565b60a083018190526020830182600381111561390e57fe5b600381111561391957fe5b905250600090508160200151600381111561393057fe5b1461396c5760405162461bcd60e51b815260040180806020018281038252603a815260200180614bdb603a913960400191505060405180910390fd5b61397c600b548260e00151612e44565b60c083018190526020830182600381111561399357fe5b600381111561399e57fe5b90525060009050816020015160038111156139b557fe5b146139f15760405162461bcd60e51b8152600401808060200182810382526031815260200180614c356031913960400191505060405180910390fd5b60a080820180516001600160a01b03808a16600081815260106020908152604091829020948555600a5460019095019490945560c0870151600b81905560e088015195518251948f16855294840192909252828101949094526060820192909252608081019190915290517f1a2a22cb034d26d1854bdc6666a5b91fe25efbbb5dcad3b0355478d6f5c362a1929181900390910190a160055460e0820151606083015160408051631ededc9160e01b81523060048201526001600160a01b038c811660248301528b8116604483015260648201949094526084810192909252519190921691631ededc919160a480830192600092919082900301818387803b158015613afc57600080fd5b505af1158015613b10573d6000803e3d6000fd5b5060009250613b1d915050565b8160e00151935093505050935093915050565b6040516001600160a01b0383169082156108fc029083906000818181858888f19350505050158015611b15573d6000803e3d6000fd5b6000821580613b73575081155b613bae5760405162461bcd60e51b8152600401808060200182810382526034815260200180614cf66034913960400191505060405180910390fd5b613bb6614a18565b613bbe6125a3565b6040830181905260208301826003811115613bd557fe5b6003811115613be057fe5b9052506000905081602001516003811115613bf757fe5b14613c1b57613c136009602b836020015160038111156118e657fe5b915050611281565b8315613c9c576060810184905260408051602081018252908201518152613c429085612962565b6080830181905260208301826003811115613c5957fe5b6003811115613c6457fe5b9052506000905081602001516003811115613c7b57fe5b14613c9757613c1360096029836020015160038111156118e657fe5b613d15565b613cb88360405180602001604052808460400151815250613637565b6060830181905260208301826003811115613ccf57fe5b6003811115613cda57fe5b9052506000905081602001516003811115613cf157fe5b14613d0d57613c136009602a836020015160038111156118e657fe5b608081018390525b60055460608201516040805163eabe7d9160e01b81523060048201526001600160a01b03898116602483015260448201939093529051600093929092169163eabe7d919160648082019260209290919082900301818787803b158015613d7a57600080fd5b505af1158015613d8e573d6000803e3d6000fd5b505050506040513d6020811015613da457600080fd5b505190508015613dc457613dbb6003602883612ecf565b92505050611281565b613dcc612ccb565b60095414613de057613dbb600a602c6120dd565b613df0600d548360600151612e44565b60a0840181905260208401826003811115613e0757fe5b6003811115613e1257fe5b9052506000905082602001516003811115613e2957fe5b14613e4557613dbb6009602e846020015160038111156118e657fe5b6001600160a01b0386166000908152600e60205260409020546060830151613e6d9190612e44565b60c0840181905260208401826003811115613e8457fe5b6003811115613e8f57fe5b9052506000905082602001516003811115613ea657fe5b14613ec257613dbb6009602d846020015160038111156118e657fe5b8160800151613ecf6129b5565b1015613ee157613dbb600e602f6120dd565b613eef868360800151613b30565b60a0820151600d5560c08201516001600160a01b0387166000818152600e6020908152604091829020939093556060850151815190815290513093600080516020614c15833981519152928290030190a36080820151606080840151604080516001600160a01b038b168152602081019490945283810191909152517fe5b754fb1abb7f01b499791d0b820ae3b6af3424ac1c59768edb53f4ec31a9299281900390910190a160055460808301516060840151604080516351dff98960e01b81523060048201526001600160a01b038b81166024830152604482019490945260648101929092525191909216916351dff98991608480830192600092919082900301818387803b15801561400257600080fd5b505af1158015614016573d6000803e3d6000fd5b5060009250614023915050565b9695505050505050565b600080836140405750600090508061259c565b8383028385828161404d57fe5b04146140615750600291506000905061259c565b60009250905061259c565b60008082614080575060019050600061259c565b600083858161408b57fe5b04915091509250929050565b60055460408051632fe3f38f60e11b81523060048201526001600160a01b0384811660248301528781166044830152868116606483015260848201869052915160009384938493911691635fc7e71e9160a48082019260209290919082900301818787803b15801561410857600080fd5b505af115801561411c573d6000803e3d6000fd5b505050506040513d602081101561413257600080fd5b505190508015614156576141496003601283612ecf565b9250600091506146119050565b61415e612ccb565b6009541461417257614149600a60166120dd565b61417a612ccb565b846001600160a01b0316636c540baf6040518163ffffffff1660e01b815260040160206040518083038186803b1580156141b357600080fd5b505afa1580156141c7573d6000803e3d6000fd5b505050506040513d60208110156141dd57600080fd5b5051146141f057614149600a60116120dd565b866001600160a01b0316866001600160a01b0316141561421657614149600660176120dd565b8461422757614149600760156120dd565b60001985141561423d57614149600760146120dd565b60008061424b89898961374b565b9092509050811561427b5761426c82601081111561426557fe5b60186120dd565b94506000935061461192505050565b6005546040805163c488847b60e01b81523060048201526001600160a01b038981166024830152604482018590528251600094859492169263c488847b926064808301939192829003018186803b1580156142d557600080fd5b505afa1580156142e9573d6000803e3d6000fd5b505050506040513d60408110156142ff57600080fd5b5080516020909101519092509050811561434a5760405162461bcd60e51b8152600401808060200182810382526033815260200180614c666033913960400191505060405180910390fd5b80886001600160a01b03166370a082318c6040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b0316815260200191505060206040518083038186803b1580156143a157600080fd5b505afa1580156143b5573d6000803e3d6000fd5b505050506040513d60208110156143cb57600080fd5b50511015614420576040805162461bcd60e51b815260206004820152601860248201527f4c49515549444154455f5345495a455f544f4f5f4d5543480000000000000000604482015290519081900360640190fd5b60006001600160a01b0389163014156144465761443f308d8d856130e9565b90506144d0565b6040805163b2a02ff160e01b81526001600160a01b038e811660048301528d81166024830152604482018590529151918b169163b2a02ff1916064808201926020929091908290030181600087803b1580156144a157600080fd5b505af11580156144b5573d6000803e3d6000fd5b505050506040513d60208110156144cb57600080fd5b505190505b801561451a576040805162461bcd60e51b81526020600482015260146024820152731d1bdad95b881cd95a5e9d5c994819985a5b195960621b604482015290519081900360640190fd5b604080516001600160a01b03808f168252808e1660208301528183018790528b1660608201526080810184905290517f298637f684da70674f26509b10f07ec2fbc77a335ab1e7d6215a4b2484d8bb529181900360a00190a1600554604080516347ef3b3b60e01b81523060048201526001600160a01b038c811660248301528f811660448301528e811660648301526084820188905260a48201869052915191909216916347ef3b3b9160c480830192600092919082900301818387803b1580156145e557600080fd5b505af11580156145f9573d6000803e3d6000fd5b5060009250614606915050565b975092955050505050505b94509492505050565b6005546040805163368f515360e21b81523060048201526001600160a01b0385811660248301526044820185905291516000938493169163da3d454c91606480830192602092919082900301818787803b15801561467757600080fd5b505af115801561468b573d6000803e3d6000fd5b505050506040513d60208110156146a157600080fd5b5051905080156146c0576146b86003600e83612ecf565b915050610e6f565b6146c8612ccb565b600954146146db576146b8600a806120dd565b826146e46129b5565b10156146f6576146b8600e60096120dd565b6146fe614a9c565b61470785612c17565b602083018190528282600381111561471b57fe5b600381111561472657fe5b905250600090508151600381111561473a57fe5b1461475f5761475660096007836000015160038111156118e657fe5b92505050610e6f565b61476d816020015185612f35565b604083018190528282600381111561478157fe5b600381111561478c57fe5b90525060009050815160038111156147a057fe5b146147bc576147566009600c836000015160038111156118e657fe5b6147c8600b5485612f35565b60608301819052828260038111156147dc57fe5b60038111156147e757fe5b90525060009050815160038111156147fb57fe5b14614817576147566009600b836000015160038111156118e657fe5b6148218585613b30565b604080820180516001600160a01b03881660008181526010602090815290859020928355600a54600190930192909255606080860151600b81905593518551928352928201899052818501929092529081019190915290517f13ed6866d4e1ee6da46f845c46d7e54120883d75c5ea9a2dacc1c4ca8984ab809181900360800190a160055460408051635c77860560e01b81523060048201526001600160a01b0388811660248301526044820188905291519190921691635c77860591606480830192600092919082900301818387803b1580156148fe57600080fd5b505af1158015614912573d6000803e3d6000fd5b506000925061491f915050565b95945050505050565b6000614932614987565b600080614947670de0b6b3a76400008761402d565b9092509050600082600381111561495a57fe5b146149795750604080516020810190915260008152909250905061259c565b6129a881866000015161368c565b6040518060200160405280600081525090565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f106149db57805160ff1916838001178555614a08565b82800160010185558215614a08579182015b82811115614a085782518255916020019190600101906149ed565b50614a14929150614ac5565b5090565b6040805160e0810190915280600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6040805161010081019091528060008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604080516080810190915280600081526020016000815260200160008152602001600081525090565b610fdf91905b80821115614a145760008155600101614acb56fe6f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e6365696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c65644d494e545f4e45575f4143434f554e545f42414c414e43455f43414c43554c4154494f4e5f4641494c4544626f72726f7742616c616e636553746f7265643a20626f72726f7742616c616e636553746f726564496e7465726e616c206661696c656452455041595f424f52524f575f4e45575f4143434f554e545f424f52524f575f42414c414e43455f43414c43554c4154494f4e5f4641494c4544ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef52455041595f424f52524f575f4e45575f544f54414c5f42414c414e43455f43414c43554c4154494f4e5f4641494c45444c49515549444154455f434f4d5054524f4c4c45525f43414c43554c4154455f414d4f554e545f5345495a455f4641494c454465786368616e67655261746553746f7265643a2065786368616e67655261746553746f726564496e7465726e616c206661696c65644d494e545f4e45575f544f54414c5f535550504c595f43414c43554c4154494f4e5f4641494c45446f6e65206f662072656465656d546f6b656e73496e206f722072656465656d416d6f756e74496e206d757374206265207a65726f72656475636520726573657276657320756e657870656374656420756e646572666c6f77a265627a7a72315820199a52f19effc9fa278ff537a78a3ad69d70c43f1d28a9b8176430271ea86a0f64736f6c634300051100326f6e6c792061646d696e206d617920696e697469616c697a6520746865206d61726b65746d61726b6574206d6179206f6e6c7920626520696e697469616c697a6564206f6e6365696e697469616c2065786368616e67652072617465206d7573742062652067726561746572207468616e207a65726f2e73657474696e6720696e7465726573742072617465206d6f64656c206661696c6564" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CEther<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CEther<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CEther<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CEther))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CEther<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CETHER_ABI.clone(), client).into()
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
                CETHER_ABI.clone(),
                CETHER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `liquidateBorrow` (0xaae40a2a) function"]
        pub fn liquidate_borrow(
            &self,
            borrower: ethers::core::types::Address,
            c_token_collateral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 228, 10, 42], (borrower, c_token_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x1249c58b) function"]
        pub fn mint(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 73, 197, 139], ())
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
        #[doc = "Calls the contract's `repayBorrow` (0x4e4d9fea) function"]
        pub fn repay_borrow(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 77, 159, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBorrowBehalf` (0xe5974619) function"]
        pub fn repay_borrow_behalf(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 151, 70, 25], borrower)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CEtherEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CEther<M> {
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
    pub enum CEtherEvents {
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
    impl ethers::contract::EthLogDecode for CEtherEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CEtherEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CEtherEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CEtherEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CEtherEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CEtherEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CEtherEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CEtherEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CEtherEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CEtherEvents::NewMarketInterestRateModelFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CEtherEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CEtherEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CEtherEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CEtherEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CEtherEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CEtherEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CEtherEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CEtherEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CEtherEvents::AccrueInterestFilter(element) => element.fmt(f),
                CEtherEvents::ApprovalFilter(element) => element.fmt(f),
                CEtherEvents::BorrowFilter(element) => element.fmt(f),
                CEtherEvents::FailureFilter(element) => element.fmt(f),
                CEtherEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CEtherEvents::MintFilter(element) => element.fmt(f),
                CEtherEvents::NewAdminFilter(element) => element.fmt(f),
                CEtherEvents::NewComptrollerFilter(element) => element.fmt(f),
                CEtherEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CEtherEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CEtherEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CEtherEvents::RedeemFilter(element) => element.fmt(f),
                CEtherEvents::RepayBorrowFilter(element) => element.fmt(f),
                CEtherEvents::ReservesAddedFilter(element) => element.fmt(f),
                CEtherEvents::ReservesReducedFilter(element) => element.fmt(f),
                CEtherEvents::TransferFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `liquidateBorrow`function with signature `liquidateBorrow(address,address)` and selector `[170, 228, 10, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "liquidateBorrow", abi = "liquidateBorrow(address,address)")]
    pub struct LiquidateBorrowCall {
        pub borrower: ethers::core::types::Address,
        pub c_token_collateral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint()` and selector `[18, 73, 197, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint()")]
    pub struct MintCall;
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
    #[doc = "Container type for all input parameters for the `repayBorrow`function with signature `repayBorrow()` and selector `[78, 77, 159, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayBorrow", abi = "repayBorrow()")]
    pub struct RepayBorrowCall;
    #[doc = "Container type for all input parameters for the `repayBorrowBehalf`function with signature `repayBorrowBehalf(address)` and selector `[229, 151, 70, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayBorrowBehalf", abi = "repayBorrowBehalf(address)")]
    pub struct RepayBorrowBehalfCall {
        pub borrower: ethers::core::types::Address,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CEtherCalls {
        AcceptAdmin(AcceptAdminCall),
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
    }
    impl ethers::core::abi::AbiDecode for CEtherCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::IsCToken(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CEtherCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CEtherCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CEtherCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CEtherCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CEtherCalls::AcceptAdmin(element) => element.encode(),
                CEtherCalls::ReduceReserves(element) => element.encode(),
                CEtherCalls::SetComptroller(element) => element.encode(),
                CEtherCalls::SetInterestRateModel(element) => element.encode(),
                CEtherCalls::SetPendingAdmin(element) => element.encode(),
                CEtherCalls::SetReserveFactor(element) => element.encode(),
                CEtherCalls::AccrualBlockNumber(element) => element.encode(),
                CEtherCalls::AccrueInterest(element) => element.encode(),
                CEtherCalls::Admin(element) => element.encode(),
                CEtherCalls::Allowance(element) => element.encode(),
                CEtherCalls::Approve(element) => element.encode(),
                CEtherCalls::BalanceOf(element) => element.encode(),
                CEtherCalls::BalanceOfUnderlying(element) => element.encode(),
                CEtherCalls::Borrow(element) => element.encode(),
                CEtherCalls::BorrowBalanceCurrent(element) => element.encode(),
                CEtherCalls::BorrowBalanceStored(element) => element.encode(),
                CEtherCalls::BorrowIndex(element) => element.encode(),
                CEtherCalls::BorrowRatePerBlock(element) => element.encode(),
                CEtherCalls::Comptroller(element) => element.encode(),
                CEtherCalls::Decimals(element) => element.encode(),
                CEtherCalls::ExchangeRateCurrent(element) => element.encode(),
                CEtherCalls::ExchangeRateStored(element) => element.encode(),
                CEtherCalls::GetAccountSnapshot(element) => element.encode(),
                CEtherCalls::GetCash(element) => element.encode(),
                CEtherCalls::Initialize(element) => element.encode(),
                CEtherCalls::InterestRateModel(element) => element.encode(),
                CEtherCalls::IsCToken(element) => element.encode(),
                CEtherCalls::LiquidateBorrow(element) => element.encode(),
                CEtherCalls::Mint(element) => element.encode(),
                CEtherCalls::Name(element) => element.encode(),
                CEtherCalls::PendingAdmin(element) => element.encode(),
                CEtherCalls::Redeem(element) => element.encode(),
                CEtherCalls::RedeemUnderlying(element) => element.encode(),
                CEtherCalls::RepayBorrow(element) => element.encode(),
                CEtherCalls::RepayBorrowBehalf(element) => element.encode(),
                CEtherCalls::ReserveFactorMantissa(element) => element.encode(),
                CEtherCalls::Seize(element) => element.encode(),
                CEtherCalls::SupplyRatePerBlock(element) => element.encode(),
                CEtherCalls::Symbol(element) => element.encode(),
                CEtherCalls::TotalBorrows(element) => element.encode(),
                CEtherCalls::TotalBorrowsCurrent(element) => element.encode(),
                CEtherCalls::TotalReserves(element) => element.encode(),
                CEtherCalls::TotalSupply(element) => element.encode(),
                CEtherCalls::Transfer(element) => element.encode(),
                CEtherCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CEtherCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CEtherCalls::AcceptAdmin(element) => element.fmt(f),
                CEtherCalls::ReduceReserves(element) => element.fmt(f),
                CEtherCalls::SetComptroller(element) => element.fmt(f),
                CEtherCalls::SetInterestRateModel(element) => element.fmt(f),
                CEtherCalls::SetPendingAdmin(element) => element.fmt(f),
                CEtherCalls::SetReserveFactor(element) => element.fmt(f),
                CEtherCalls::AccrualBlockNumber(element) => element.fmt(f),
                CEtherCalls::AccrueInterest(element) => element.fmt(f),
                CEtherCalls::Admin(element) => element.fmt(f),
                CEtherCalls::Allowance(element) => element.fmt(f),
                CEtherCalls::Approve(element) => element.fmt(f),
                CEtherCalls::BalanceOf(element) => element.fmt(f),
                CEtherCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CEtherCalls::Borrow(element) => element.fmt(f),
                CEtherCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CEtherCalls::BorrowBalanceStored(element) => element.fmt(f),
                CEtherCalls::BorrowIndex(element) => element.fmt(f),
                CEtherCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CEtherCalls::Comptroller(element) => element.fmt(f),
                CEtherCalls::Decimals(element) => element.fmt(f),
                CEtherCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CEtherCalls::ExchangeRateStored(element) => element.fmt(f),
                CEtherCalls::GetAccountSnapshot(element) => element.fmt(f),
                CEtherCalls::GetCash(element) => element.fmt(f),
                CEtherCalls::Initialize(element) => element.fmt(f),
                CEtherCalls::InterestRateModel(element) => element.fmt(f),
                CEtherCalls::IsCToken(element) => element.fmt(f),
                CEtherCalls::LiquidateBorrow(element) => element.fmt(f),
                CEtherCalls::Mint(element) => element.fmt(f),
                CEtherCalls::Name(element) => element.fmt(f),
                CEtherCalls::PendingAdmin(element) => element.fmt(f),
                CEtherCalls::Redeem(element) => element.fmt(f),
                CEtherCalls::RedeemUnderlying(element) => element.fmt(f),
                CEtherCalls::RepayBorrow(element) => element.fmt(f),
                CEtherCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CEtherCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CEtherCalls::Seize(element) => element.fmt(f),
                CEtherCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CEtherCalls::Symbol(element) => element.fmt(f),
                CEtherCalls::TotalBorrows(element) => element.fmt(f),
                CEtherCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CEtherCalls::TotalReserves(element) => element.fmt(f),
                CEtherCalls::TotalSupply(element) => element.fmt(f),
                CEtherCalls::Transfer(element) => element.fmt(f),
                CEtherCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CEtherCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CEtherCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CEtherCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CEtherCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CEtherCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CEtherCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CEtherCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CEtherCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CEtherCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CEtherCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CEtherCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CEtherCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CEtherCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CEtherCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CEtherCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CEtherCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CEtherCalls {
        fn from(var: AdminCall) -> Self {
            CEtherCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CEtherCalls {
        fn from(var: AllowanceCall) -> Self {
            CEtherCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CEtherCalls {
        fn from(var: ApproveCall) -> Self {
            CEtherCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CEtherCalls {
        fn from(var: BalanceOfCall) -> Self {
            CEtherCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CEtherCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CEtherCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CEtherCalls {
        fn from(var: BorrowCall) -> Self {
            CEtherCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CEtherCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CEtherCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CEtherCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CEtherCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CEtherCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CEtherCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CEtherCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CEtherCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CEtherCalls {
        fn from(var: ComptrollerCall) -> Self {
            CEtherCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CEtherCalls {
        fn from(var: DecimalsCall) -> Self {
            CEtherCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CEtherCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CEtherCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CEtherCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CEtherCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CEtherCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CEtherCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CEtherCalls {
        fn from(var: GetCashCall) -> Self {
            CEtherCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CEtherCalls {
        fn from(var: InitializeCall) -> Self {
            CEtherCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CEtherCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CEtherCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CEtherCalls {
        fn from(var: IsCTokenCall) -> Self {
            CEtherCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CEtherCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CEtherCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CEtherCalls {
        fn from(var: MintCall) -> Self {
            CEtherCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CEtherCalls {
        fn from(var: NameCall) -> Self {
            CEtherCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CEtherCalls {
        fn from(var: PendingAdminCall) -> Self {
            CEtherCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CEtherCalls {
        fn from(var: RedeemCall) -> Self {
            CEtherCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CEtherCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CEtherCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CEtherCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CEtherCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CEtherCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CEtherCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CEtherCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CEtherCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CEtherCalls {
        fn from(var: SeizeCall) -> Self {
            CEtherCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CEtherCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CEtherCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CEtherCalls {
        fn from(var: SymbolCall) -> Self {
            CEtherCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CEtherCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CEtherCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CEtherCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CEtherCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CEtherCalls {
        fn from(var: TotalReservesCall) -> Self {
            CEtherCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CEtherCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CEtherCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CEtherCalls {
        fn from(var: TransferCall) -> Self {
            CEtherCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CEtherCalls {
        fn from(var: TransferFromCall) -> Self {
            CEtherCalls::TransferFrom(var)
        }
    }
}
