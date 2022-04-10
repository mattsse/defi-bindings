pub use ctoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod ctoken_mod {
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
    #[doc = "CToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cashPrior\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestAccumulated\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AccrueInterest\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"error\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"info\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"detail\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Failure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"oldComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewComptroller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"oldInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewMarketInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewReserveFactor\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"accountBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"totalBorrows\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RepayBorrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"benefactor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalReserves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservesReduced\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_acceptAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reduceAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_reduceReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"newComptroller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setComptroller\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setInterestRateModel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setPendingAdmin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newReserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_setReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accrueInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"balanceOfUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowBalanceCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowBalanceStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchangeRateCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"exchangeRateStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"comptroller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract InterestRateModel\",\"name\":\"interestRateModel_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"initialExchangeRateMantissa_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seize\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supplyRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"totalBorrowsCurrent\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct CToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CTOKEN_ABI.clone(), client).into()
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
                CTOKEN_ABI.clone(),
                CTOKEN_BYTECODE.clone().into(),
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CToken<M> {
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
    pub enum CTokenEvents {
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
    impl ethers::contract::EthLogDecode for CTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(CTokenEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(CTokenEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FailureFilter::decode_log(log) {
                return Ok(CTokenEvents::FailureFilter(decoded));
            }
            if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
                return Ok(CTokenEvents::LiquidateBorrowFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(CTokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(CTokenEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewComptrollerFilter::decode_log(log) {
                return Ok(CTokenEvents::NewComptrollerFilter(decoded));
            }
            if let Ok(decoded) = NewMarketInterestRateModelFilter::decode_log(log) {
                return Ok(CTokenEvents::NewMarketInterestRateModelFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(CTokenEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = NewReserveFactorFilter::decode_log(log) {
                return Ok(CTokenEvents::NewReserveFactorFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(CTokenEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RepayBorrowFilter::decode_log(log) {
                return Ok(CTokenEvents::RepayBorrowFilter(decoded));
            }
            if let Ok(decoded) = ReservesAddedFilter::decode_log(log) {
                return Ok(CTokenEvents::ReservesAddedFilter(decoded));
            }
            if let Ok(decoded) = ReservesReducedFilter::decode_log(log) {
                return Ok(CTokenEvents::ReservesReducedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CTokenEvents::AccrueInterestFilter(element) => element.fmt(f),
                CTokenEvents::ApprovalFilter(element) => element.fmt(f),
                CTokenEvents::BorrowFilter(element) => element.fmt(f),
                CTokenEvents::FailureFilter(element) => element.fmt(f),
                CTokenEvents::LiquidateBorrowFilter(element) => element.fmt(f),
                CTokenEvents::MintFilter(element) => element.fmt(f),
                CTokenEvents::NewAdminFilter(element) => element.fmt(f),
                CTokenEvents::NewComptrollerFilter(element) => element.fmt(f),
                CTokenEvents::NewMarketInterestRateModelFilter(element) => element.fmt(f),
                CTokenEvents::NewPendingAdminFilter(element) => element.fmt(f),
                CTokenEvents::NewReserveFactorFilter(element) => element.fmt(f),
                CTokenEvents::RedeemFilter(element) => element.fmt(f),
                CTokenEvents::RepayBorrowFilter(element) => element.fmt(f),
                CTokenEvents::ReservesAddedFilter(element) => element.fmt(f),
                CTokenEvents::ReservesReducedFilter(element) => element.fmt(f),
                CTokenEvents::TransferFilter(element) => element.fmt(f),
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
    pub enum CTokenCalls {
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
        Name(NameCall),
        PendingAdmin(PendingAdminCall),
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
    impl ethers::core::abi::AbiDecode for CTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <ReduceReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::ReduceReserves(decoded));
            }
            if let Ok(decoded) =
                <SetComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::SetComptroller(decoded));
            }
            if let Ok(decoded) =
                <SetInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::SetInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::SetPendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BalanceOfUnderlying(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BorrowBalanceCurrent(decoded));
            }
            if let Ok(decoded) =
                <BorrowBalanceStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BorrowBalanceStored(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <BorrowRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::BorrowRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::ExchangeRateCurrent(decoded));
            }
            if let Ok(decoded) =
                <ExchangeRateStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::ExchangeRateStored(decoded));
            }
            if let Ok(decoded) =
                <GetAccountSnapshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::GetAccountSnapshot(decoded));
            }
            if let Ok(decoded) =
                <GetCashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::GetCash(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <IsCTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::IsCToken(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CTokenCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SeizeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Seize(decoded));
            }
            if let Ok(decoded) =
                <SupplyRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::SupplyRatePerBlock(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCurrentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::TotalBorrowsCurrent(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CTokenCalls::AcceptAdmin(element) => element.encode(),
                CTokenCalls::ReduceReserves(element) => element.encode(),
                CTokenCalls::SetComptroller(element) => element.encode(),
                CTokenCalls::SetInterestRateModel(element) => element.encode(),
                CTokenCalls::SetPendingAdmin(element) => element.encode(),
                CTokenCalls::SetReserveFactor(element) => element.encode(),
                CTokenCalls::AccrualBlockNumber(element) => element.encode(),
                CTokenCalls::AccrueInterest(element) => element.encode(),
                CTokenCalls::Admin(element) => element.encode(),
                CTokenCalls::Allowance(element) => element.encode(),
                CTokenCalls::Approve(element) => element.encode(),
                CTokenCalls::BalanceOf(element) => element.encode(),
                CTokenCalls::BalanceOfUnderlying(element) => element.encode(),
                CTokenCalls::BorrowBalanceCurrent(element) => element.encode(),
                CTokenCalls::BorrowBalanceStored(element) => element.encode(),
                CTokenCalls::BorrowIndex(element) => element.encode(),
                CTokenCalls::BorrowRatePerBlock(element) => element.encode(),
                CTokenCalls::Comptroller(element) => element.encode(),
                CTokenCalls::Decimals(element) => element.encode(),
                CTokenCalls::ExchangeRateCurrent(element) => element.encode(),
                CTokenCalls::ExchangeRateStored(element) => element.encode(),
                CTokenCalls::GetAccountSnapshot(element) => element.encode(),
                CTokenCalls::GetCash(element) => element.encode(),
                CTokenCalls::Initialize(element) => element.encode(),
                CTokenCalls::InterestRateModel(element) => element.encode(),
                CTokenCalls::IsCToken(element) => element.encode(),
                CTokenCalls::Name(element) => element.encode(),
                CTokenCalls::PendingAdmin(element) => element.encode(),
                CTokenCalls::ReserveFactorMantissa(element) => element.encode(),
                CTokenCalls::Seize(element) => element.encode(),
                CTokenCalls::SupplyRatePerBlock(element) => element.encode(),
                CTokenCalls::Symbol(element) => element.encode(),
                CTokenCalls::TotalBorrows(element) => element.encode(),
                CTokenCalls::TotalBorrowsCurrent(element) => element.encode(),
                CTokenCalls::TotalReserves(element) => element.encode(),
                CTokenCalls::TotalSupply(element) => element.encode(),
                CTokenCalls::Transfer(element) => element.encode(),
                CTokenCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CTokenCalls::AcceptAdmin(element) => element.fmt(f),
                CTokenCalls::ReduceReserves(element) => element.fmt(f),
                CTokenCalls::SetComptroller(element) => element.fmt(f),
                CTokenCalls::SetInterestRateModel(element) => element.fmt(f),
                CTokenCalls::SetPendingAdmin(element) => element.fmt(f),
                CTokenCalls::SetReserveFactor(element) => element.fmt(f),
                CTokenCalls::AccrualBlockNumber(element) => element.fmt(f),
                CTokenCalls::AccrueInterest(element) => element.fmt(f),
                CTokenCalls::Admin(element) => element.fmt(f),
                CTokenCalls::Allowance(element) => element.fmt(f),
                CTokenCalls::Approve(element) => element.fmt(f),
                CTokenCalls::BalanceOf(element) => element.fmt(f),
                CTokenCalls::BalanceOfUnderlying(element) => element.fmt(f),
                CTokenCalls::BorrowBalanceCurrent(element) => element.fmt(f),
                CTokenCalls::BorrowBalanceStored(element) => element.fmt(f),
                CTokenCalls::BorrowIndex(element) => element.fmt(f),
                CTokenCalls::BorrowRatePerBlock(element) => element.fmt(f),
                CTokenCalls::Comptroller(element) => element.fmt(f),
                CTokenCalls::Decimals(element) => element.fmt(f),
                CTokenCalls::ExchangeRateCurrent(element) => element.fmt(f),
                CTokenCalls::ExchangeRateStored(element) => element.fmt(f),
                CTokenCalls::GetAccountSnapshot(element) => element.fmt(f),
                CTokenCalls::GetCash(element) => element.fmt(f),
                CTokenCalls::Initialize(element) => element.fmt(f),
                CTokenCalls::InterestRateModel(element) => element.fmt(f),
                CTokenCalls::IsCToken(element) => element.fmt(f),
                CTokenCalls::Name(element) => element.fmt(f),
                CTokenCalls::PendingAdmin(element) => element.fmt(f),
                CTokenCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CTokenCalls::Seize(element) => element.fmt(f),
                CTokenCalls::SupplyRatePerBlock(element) => element.fmt(f),
                CTokenCalls::Symbol(element) => element.fmt(f),
                CTokenCalls::TotalBorrows(element) => element.fmt(f),
                CTokenCalls::TotalBorrowsCurrent(element) => element.fmt(f),
                CTokenCalls::TotalReserves(element) => element.fmt(f),
                CTokenCalls::TotalSupply(element) => element.fmt(f),
                CTokenCalls::Transfer(element) => element.fmt(f),
                CTokenCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for CTokenCalls {
        fn from(var: AcceptAdminCall) -> Self {
            CTokenCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<ReduceReservesCall> for CTokenCalls {
        fn from(var: ReduceReservesCall) -> Self {
            CTokenCalls::ReduceReserves(var)
        }
    }
    impl ::std::convert::From<SetComptrollerCall> for CTokenCalls {
        fn from(var: SetComptrollerCall) -> Self {
            CTokenCalls::SetComptroller(var)
        }
    }
    impl ::std::convert::From<SetInterestRateModelCall> for CTokenCalls {
        fn from(var: SetInterestRateModelCall) -> Self {
            CTokenCalls::SetInterestRateModel(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for CTokenCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            CTokenCalls::SetPendingAdmin(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for CTokenCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            CTokenCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CTokenCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CTokenCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for CTokenCalls {
        fn from(var: AccrueInterestCall) -> Self {
            CTokenCalls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CTokenCalls {
        fn from(var: AdminCall) -> Self {
            CTokenCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            CTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CTokenCalls {
        fn from(var: ApproveCall) -> Self {
            CTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            CTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalanceOfUnderlyingCall> for CTokenCalls {
        fn from(var: BalanceOfUnderlyingCall) -> Self {
            CTokenCalls::BalanceOfUnderlying(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceCurrentCall> for CTokenCalls {
        fn from(var: BorrowBalanceCurrentCall) -> Self {
            CTokenCalls::BorrowBalanceCurrent(var)
        }
    }
    impl ::std::convert::From<BorrowBalanceStoredCall> for CTokenCalls {
        fn from(var: BorrowBalanceStoredCall) -> Self {
            CTokenCalls::BorrowBalanceStored(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CTokenCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CTokenCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<BorrowRatePerBlockCall> for CTokenCalls {
        fn from(var: BorrowRatePerBlockCall) -> Self {
            CTokenCalls::BorrowRatePerBlock(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CTokenCalls {
        fn from(var: ComptrollerCall) -> Self {
            CTokenCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            CTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<ExchangeRateCurrentCall> for CTokenCalls {
        fn from(var: ExchangeRateCurrentCall) -> Self {
            CTokenCalls::ExchangeRateCurrent(var)
        }
    }
    impl ::std::convert::From<ExchangeRateStoredCall> for CTokenCalls {
        fn from(var: ExchangeRateStoredCall) -> Self {
            CTokenCalls::ExchangeRateStored(var)
        }
    }
    impl ::std::convert::From<GetAccountSnapshotCall> for CTokenCalls {
        fn from(var: GetAccountSnapshotCall) -> Self {
            CTokenCalls::GetAccountSnapshot(var)
        }
    }
    impl ::std::convert::From<GetCashCall> for CTokenCalls {
        fn from(var: GetCashCall) -> Self {
            CTokenCalls::GetCash(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CTokenCalls {
        fn from(var: InitializeCall) -> Self {
            CTokenCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CTokenCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CTokenCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<IsCTokenCall> for CTokenCalls {
        fn from(var: IsCTokenCall) -> Self {
            CTokenCalls::IsCToken(var)
        }
    }
    impl ::std::convert::From<NameCall> for CTokenCalls {
        fn from(var: NameCall) -> Self {
            CTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CTokenCalls {
        fn from(var: PendingAdminCall) -> Self {
            CTokenCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CTokenCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CTokenCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SeizeCall> for CTokenCalls {
        fn from(var: SeizeCall) -> Self {
            CTokenCalls::Seize(var)
        }
    }
    impl ::std::convert::From<SupplyRatePerBlockCall> for CTokenCalls {
        fn from(var: SupplyRatePerBlockCall) -> Self {
            CTokenCalls::SupplyRatePerBlock(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CTokenCalls {
        fn from(var: SymbolCall) -> Self {
            CTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CTokenCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CTokenCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCurrentCall> for CTokenCalls {
        fn from(var: TotalBorrowsCurrentCall) -> Self {
            CTokenCalls::TotalBorrowsCurrent(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CTokenCalls {
        fn from(var: TotalReservesCall) -> Self {
            CTokenCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CTokenCalls {
        fn from(var: TransferCall) -> Self {
            CTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            CTokenCalls::TransferFrom(var)
        }
    }
}
