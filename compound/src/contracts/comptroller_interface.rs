pub use comptrollerinterface_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerinterface_mod {
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
    #[doc = "ComptrollerInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"cTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterMarkets\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitMarket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isComptroller\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidateCalculateSeizeTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"minter\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"mintTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"redeemer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowerIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cTokenBorrowed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"seizeTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"seizeVerify\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAllowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"transferTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferVerify\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERINTERFACE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ComptrollerInterface<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                COMPTROLLERINTERFACE_ABI.clone(),
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
                COMPTROLLERINTERFACE_ABI.clone(),
                COMPTROLLERINTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([237, 228, 237, 208], c_token)
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
            repay_amount: ethers::core::types::U256,
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
                        repay_amount,
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
            repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [196, 136, 132, 123],
                    (c_token_borrowed, c_token_collateral, repay_amount),
                )
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
        #[doc = "Calls the contract's `mintVerify` (0x41c728b9) function"]
        pub fn mint_verify(
            &self,
            c_token: ethers::core::types::Address,
            minter: ethers::core::types::Address,
            mint_amount: ethers::core::types::U256,
            mint_tokens: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [65, 199, 40, 185],
                    (c_token, minter, mint_amount, mint_tokens),
                )
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
            repay_amount: ethers::core::types::U256,
            borrower_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 222, 220, 145],
                    (c_token, payer, borrower, repay_amount, borrower_index),
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ComptrollerInterface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
        pub c_token: ethers::core::types::Address,
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
        pub repay_amount: ethers::core::types::U256,
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
        pub repay_amount: ethers::core::types::U256,
    }
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
        pub mint_amount: ethers::core::types::U256,
        pub mint_tokens: ethers::core::types::U256,
    }
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
        pub repay_amount: ethers::core::types::U256,
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
    pub enum ComptrollerInterfaceCalls {
        BorrowAllowed(BorrowAllowedCall),
        BorrowVerify(BorrowVerifyCall),
        EnterMarkets(EnterMarketsCall),
        ExitMarket(ExitMarketCall),
        IsComptroller(IsComptrollerCall),
        LiquidateBorrowAllowed(LiquidateBorrowAllowedCall),
        LiquidateBorrowVerify(LiquidateBorrowVerifyCall),
        LiquidateCalculateSeizeTokens(LiquidateCalculateSeizeTokensCall),
        MintAllowed(MintAllowedCall),
        MintVerify(MintVerifyCall),
        RedeemAllowed(RedeemAllowedCall),
        RedeemVerify(RedeemVerifyCall),
        RepayBorrowAllowed(RepayBorrowAllowedCall),
        RepayBorrowVerify(RepayBorrowVerifyCall),
        SeizeAllowed(SeizeAllowedCall),
        SeizeVerify(SeizeVerifyCall),
        TransferAllowed(TransferAllowedCall),
        TransferVerify(TransferVerifyCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerInterfaceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::BorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <BorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::BorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <EnterMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::EnterMarkets(decoded));
            }
            if let Ok(decoded) =
                <ExitMarketCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::ExitMarket(decoded));
            }
            if let Ok(decoded) =
                <IsComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::IsComptroller(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::LiquidateBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::LiquidateBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCalculateSeizeTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerInterfaceCalls::LiquidateCalculateSeizeTokens(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MintAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::MintAllowed(decoded));
            }
            if let Ok(decoded) =
                <MintVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::MintVerify(decoded));
            }
            if let Ok(decoded) =
                <RedeemAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::RedeemAllowed(decoded));
            }
            if let Ok(decoded) =
                <RedeemVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::RedeemVerify(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::RepayBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::RepayBorrowVerify(decoded));
            }
            if let Ok(decoded) =
                <SeizeAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::SeizeAllowed(decoded));
            }
            if let Ok(decoded) =
                <SeizeVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::SeizeVerify(decoded));
            }
            if let Ok(decoded) =
                <TransferAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::TransferAllowed(decoded));
            }
            if let Ok(decoded) =
                <TransferVerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerInterfaceCalls::TransferVerify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerInterfaceCalls::BorrowAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::BorrowVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::EnterMarkets(element) => element.encode(),
                ComptrollerInterfaceCalls::ExitMarket(element) => element.encode(),
                ComptrollerInterfaceCalls::IsComptroller(element) => element.encode(),
                ComptrollerInterfaceCalls::LiquidateBorrowAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::LiquidateBorrowVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::LiquidateCalculateSeizeTokens(element) => {
                    element.encode()
                }
                ComptrollerInterfaceCalls::MintAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::MintVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::RedeemAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::RedeemVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::RepayBorrowAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::RepayBorrowVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::SeizeAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::SeizeVerify(element) => element.encode(),
                ComptrollerInterfaceCalls::TransferAllowed(element) => element.encode(),
                ComptrollerInterfaceCalls::TransferVerify(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerInterfaceCalls::BorrowAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::BorrowVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::EnterMarkets(element) => element.fmt(f),
                ComptrollerInterfaceCalls::ExitMarket(element) => element.fmt(f),
                ComptrollerInterfaceCalls::IsComptroller(element) => element.fmt(f),
                ComptrollerInterfaceCalls::LiquidateBorrowAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::LiquidateBorrowVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::LiquidateCalculateSeizeTokens(element) => element.fmt(f),
                ComptrollerInterfaceCalls::MintAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::MintVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::RedeemAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::RedeemVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::RepayBorrowAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::RepayBorrowVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::SeizeAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::SeizeVerify(element) => element.fmt(f),
                ComptrollerInterfaceCalls::TransferAllowed(element) => element.fmt(f),
                ComptrollerInterfaceCalls::TransferVerify(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BorrowAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: BorrowAllowedCall) -> Self {
            ComptrollerInterfaceCalls::BorrowAllowed(var)
        }
    }
    impl ::std::convert::From<BorrowVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: BorrowVerifyCall) -> Self {
            ComptrollerInterfaceCalls::BorrowVerify(var)
        }
    }
    impl ::std::convert::From<EnterMarketsCall> for ComptrollerInterfaceCalls {
        fn from(var: EnterMarketsCall) -> Self {
            ComptrollerInterfaceCalls::EnterMarkets(var)
        }
    }
    impl ::std::convert::From<ExitMarketCall> for ComptrollerInterfaceCalls {
        fn from(var: ExitMarketCall) -> Self {
            ComptrollerInterfaceCalls::ExitMarket(var)
        }
    }
    impl ::std::convert::From<IsComptrollerCall> for ComptrollerInterfaceCalls {
        fn from(var: IsComptrollerCall) -> Self {
            ComptrollerInterfaceCalls::IsComptroller(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: LiquidateBorrowAllowedCall) -> Self {
            ComptrollerInterfaceCalls::LiquidateBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: LiquidateBorrowVerifyCall) -> Self {
            ComptrollerInterfaceCalls::LiquidateBorrowVerify(var)
        }
    }
    impl ::std::convert::From<LiquidateCalculateSeizeTokensCall> for ComptrollerInterfaceCalls {
        fn from(var: LiquidateCalculateSeizeTokensCall) -> Self {
            ComptrollerInterfaceCalls::LiquidateCalculateSeizeTokens(var)
        }
    }
    impl ::std::convert::From<MintAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: MintAllowedCall) -> Self {
            ComptrollerInterfaceCalls::MintAllowed(var)
        }
    }
    impl ::std::convert::From<MintVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: MintVerifyCall) -> Self {
            ComptrollerInterfaceCalls::MintVerify(var)
        }
    }
    impl ::std::convert::From<RedeemAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: RedeemAllowedCall) -> Self {
            ComptrollerInterfaceCalls::RedeemAllowed(var)
        }
    }
    impl ::std::convert::From<RedeemVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: RedeemVerifyCall) -> Self {
            ComptrollerInterfaceCalls::RedeemVerify(var)
        }
    }
    impl ::std::convert::From<RepayBorrowAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: RepayBorrowAllowedCall) -> Self {
            ComptrollerInterfaceCalls::RepayBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<RepayBorrowVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: RepayBorrowVerifyCall) -> Self {
            ComptrollerInterfaceCalls::RepayBorrowVerify(var)
        }
    }
    impl ::std::convert::From<SeizeAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: SeizeAllowedCall) -> Self {
            ComptrollerInterfaceCalls::SeizeAllowed(var)
        }
    }
    impl ::std::convert::From<SeizeVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: SeizeVerifyCall) -> Self {
            ComptrollerInterfaceCalls::SeizeVerify(var)
        }
    }
    impl ::std::convert::From<TransferAllowedCall> for ComptrollerInterfaceCalls {
        fn from(var: TransferAllowedCall) -> Self {
            ComptrollerInterfaceCalls::TransferAllowed(var)
        }
    }
    impl ::std::convert::From<TransferVerifyCall> for ComptrollerInterfaceCalls {
        fn from(var: TransferVerifyCall) -> Self {
            ComptrollerInterfaceCalls::TransferVerify(var)
        }
    }
}
