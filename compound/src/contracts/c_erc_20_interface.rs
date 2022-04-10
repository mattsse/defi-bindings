pub use cerc20interface_mod::*;
#[allow(clippy::too_many_arguments)]
mod cerc20interface_mod {
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
    #[doc = "CErc20Interface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CERC20INTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"addAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"_addReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract CTokenInterface\",\"name\":\"cTokenCollateral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"mintAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemTokens\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemUnderlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayBorrowBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CERC20INTERFACE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct CErc20Interface<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CErc20Interface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CErc20Interface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CErc20Interface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CErc20Interface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CERC20INTERFACE_ABI.clone(), client)
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
                CERC20INTERFACE_ABI.clone(),
                CERC20INTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        #[doc = "Calls the contract's `borrow` (0xc5ebeaec) function"]
        pub fn borrow(
            &self,
            borrow_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 235, 234, 236], borrow_amount)
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
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CErc20Interface<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    pub enum CErc20InterfaceCalls {
        AddReserves(AddReservesCall),
        Borrow(BorrowCall),
        LiquidateBorrow(LiquidateBorrowCall),
        Mint(MintCall),
        Redeem(RedeemCall),
        RedeemUnderlying(RedeemUnderlyingCall),
        RepayBorrow(RepayBorrowCall),
        RepayBorrowBehalf(RepayBorrowBehalfCall),
        Underlying(UnderlyingCall),
    }
    impl ethers::core::abi::AbiDecode for CErc20InterfaceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::AddReserves(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <LiquidateBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::LiquidateBorrow(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CErc20InterfaceCalls::Mint(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RedeemUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::RedeemUnderlying(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::RepayBorrow(decoded));
            }
            if let Ok(decoded) =
                <RepayBorrowBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::RepayBorrowBehalf(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CErc20InterfaceCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CErc20InterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CErc20InterfaceCalls::AddReserves(element) => element.encode(),
                CErc20InterfaceCalls::Borrow(element) => element.encode(),
                CErc20InterfaceCalls::LiquidateBorrow(element) => element.encode(),
                CErc20InterfaceCalls::Mint(element) => element.encode(),
                CErc20InterfaceCalls::Redeem(element) => element.encode(),
                CErc20InterfaceCalls::RedeemUnderlying(element) => element.encode(),
                CErc20InterfaceCalls::RepayBorrow(element) => element.encode(),
                CErc20InterfaceCalls::RepayBorrowBehalf(element) => element.encode(),
                CErc20InterfaceCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CErc20InterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CErc20InterfaceCalls::AddReserves(element) => element.fmt(f),
                CErc20InterfaceCalls::Borrow(element) => element.fmt(f),
                CErc20InterfaceCalls::LiquidateBorrow(element) => element.fmt(f),
                CErc20InterfaceCalls::Mint(element) => element.fmt(f),
                CErc20InterfaceCalls::Redeem(element) => element.fmt(f),
                CErc20InterfaceCalls::RedeemUnderlying(element) => element.fmt(f),
                CErc20InterfaceCalls::RepayBorrow(element) => element.fmt(f),
                CErc20InterfaceCalls::RepayBorrowBehalf(element) => element.fmt(f),
                CErc20InterfaceCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddReservesCall> for CErc20InterfaceCalls {
        fn from(var: AddReservesCall) -> Self {
            CErc20InterfaceCalls::AddReserves(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for CErc20InterfaceCalls {
        fn from(var: BorrowCall) -> Self {
            CErc20InterfaceCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<LiquidateBorrowCall> for CErc20InterfaceCalls {
        fn from(var: LiquidateBorrowCall) -> Self {
            CErc20InterfaceCalls::LiquidateBorrow(var)
        }
    }
    impl ::std::convert::From<MintCall> for CErc20InterfaceCalls {
        fn from(var: MintCall) -> Self {
            CErc20InterfaceCalls::Mint(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CErc20InterfaceCalls {
        fn from(var: RedeemCall) -> Self {
            CErc20InterfaceCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<RedeemUnderlyingCall> for CErc20InterfaceCalls {
        fn from(var: RedeemUnderlyingCall) -> Self {
            CErc20InterfaceCalls::RedeemUnderlying(var)
        }
    }
    impl ::std::convert::From<RepayBorrowCall> for CErc20InterfaceCalls {
        fn from(var: RepayBorrowCall) -> Self {
            CErc20InterfaceCalls::RepayBorrow(var)
        }
    }
    impl ::std::convert::From<RepayBorrowBehalfCall> for CErc20InterfaceCalls {
        fn from(var: RepayBorrowBehalfCall) -> Self {
            CErc20InterfaceCalls::RepayBorrowBehalf(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for CErc20InterfaceCalls {
        fn from(var: UnderlyingCall) -> Self {
            CErc20InterfaceCalls::Underlying(var)
        }
    }
}
