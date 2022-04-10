pub use il2pool_mod::*;
#[allow(clippy::too_many_arguments)]
mod il2pool_mod {
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
    #[doc = "IL2Pool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IL2POOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args1\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"args2\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidationCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rebalanceStableBorrowRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayWithATokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUserUseReserveAsCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"supply\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"supplyWithPermit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapBorrowRateMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"args\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IL2POOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IL2Pool<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IL2Pool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IL2Pool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IL2Pool))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IL2Pool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IL2POOL_ABI.clone(), client).into()
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
                IL2POOL_ABI.clone(),
                IL2POOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `borrow` (0xd5eed868) function"]
        pub fn borrow(&self, args: [u8; 32]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 238, 216, 104], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationCall` (0xfd21ecff) function"]
        pub fn liquidation_call(
            &self,
            args_1: [u8; 32],
            args_2: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 33, 236, 255], (args_1, args_2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalanceStableBorrowRate` (0x427da177) function"]
        pub fn rebalance_stable_borrow_rate(
            &self,
            args: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 125, 161, 119], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repay` (0x563dd613) function"]
        pub fn repay(
            &self,
            args: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([86, 61, 214, 19], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayWithATokens` (0xdc7c0bff) function"]
        pub fn repay_with_a_tokens(
            &self,
            args: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([220, 124, 11, 255], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayWithPermit` (0x94b576de) function"]
        pub fn repay_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 181, 118, 222], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserUseReserveAsCollateral` (0x4d013f03) function"]
        pub fn set_user_use_reserve_as_collateral(
            &self,
            args: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 1, 63, 3], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supply` (0xf7a73840) function"]
        pub fn supply(&self, args: [u8; 32]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 167, 56, 64], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supplyWithPermit` (0x680dd47c) function"]
        pub fn supply_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 13, 212, 124], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapBorrowRateMode` (0x1fe3c6f3) function"]
        pub fn swap_borrow_rate_mode(
            &self,
            args: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 227, 198, 243], args)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x8e19899e) function"]
        pub fn withdraw(&self, args: [u8; 32]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 25, 137, 158], args)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IL2Pool<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `borrow`function with signature `borrow(bytes32)` and selector `[213, 238, 216, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrow", abi = "borrow(bytes32)")]
    pub struct BorrowCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `liquidationCall`function with signature `liquidationCall(bytes32,bytes32)` and selector `[253, 33, 236, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "liquidationCall", abi = "liquidationCall(bytes32,bytes32)")]
    pub struct LiquidationCallCall {
        pub args_1: [u8; 32],
        pub args_2: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `rebalanceStableBorrowRate`function with signature `rebalanceStableBorrowRate(bytes32)` and selector `[66, 125, 161, 119]`"]
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(bytes32)"
    )]
    pub struct RebalanceStableBorrowRateCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `repay`function with signature `repay(bytes32)` and selector `[86, 61, 214, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repay", abi = "repay(bytes32)")]
    pub struct RepayCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `repayWithATokens`function with signature `repayWithATokens(bytes32)` and selector `[220, 124, 11, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayWithATokens", abi = "repayWithATokens(bytes32)")]
    pub struct RepayWithATokensCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `repayWithPermit`function with signature `repayWithPermit(bytes32,bytes32,bytes32)` and selector `[148, 181, 118, 222]`"]
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
        name = "repayWithPermit",
        abi = "repayWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct RepayWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setUserUseReserveAsCollateral`function with signature `setUserUseReserveAsCollateral(bytes32)` and selector `[77, 1, 63, 3]`"]
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(bytes32)"
    )]
    pub struct SetUserUseReserveAsCollateralCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `supply`function with signature `supply(bytes32)` and selector `[247, 167, 56, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supply", abi = "supply(bytes32)")]
    pub struct SupplyCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `supplyWithPermit`function with signature `supplyWithPermit(bytes32,bytes32,bytes32)` and selector `[104, 13, 212, 124]`"]
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
        name = "supplyWithPermit",
        abi = "supplyWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct SupplyWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapBorrowRateMode`function with signature `swapBorrowRateMode(bytes32)` and selector `[31, 227, 198, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swapBorrowRateMode", abi = "swapBorrowRateMode(bytes32)")]
    pub struct SwapBorrowRateModeCall {
        pub args: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(bytes32)` and selector `[142, 25, 137, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(bytes32)")]
    pub struct WithdrawCall {
        pub args: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IL2PoolCalls {
        Borrow(BorrowCall),
        LiquidationCall(LiquidationCallCall),
        RebalanceStableBorrowRate(RebalanceStableBorrowRateCall),
        Repay(RepayCall),
        RepayWithATokens(RepayWithATokensCall),
        RepayWithPermit(RepayWithPermitCall),
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateralCall),
        Supply(SupplyCall),
        SupplyWithPermit(SupplyWithPermitCall),
        SwapBorrowRateMode(SwapBorrowRateModeCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IL2PoolCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <LiquidationCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::LiquidationCall(decoded));
            }
            if let Ok(decoded) =
                <RebalanceStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IL2PoolCalls::RebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded) = <RepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::Repay(decoded));
            }
            if let Ok(decoded) =
                <RepayWithATokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::RepayWithATokens(decoded));
            }
            if let Ok(decoded) =
                <RepayWithPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::RepayWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SetUserUseReserveAsCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IL2PoolCalls::SetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded) = <SupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::Supply(decoded));
            }
            if let Ok(decoded) =
                <SupplyWithPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::SupplyWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapBorrowRateModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::SwapBorrowRateMode(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IL2PoolCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IL2PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IL2PoolCalls::Borrow(element) => element.encode(),
                IL2PoolCalls::LiquidationCall(element) => element.encode(),
                IL2PoolCalls::RebalanceStableBorrowRate(element) => element.encode(),
                IL2PoolCalls::Repay(element) => element.encode(),
                IL2PoolCalls::RepayWithATokens(element) => element.encode(),
                IL2PoolCalls::RepayWithPermit(element) => element.encode(),
                IL2PoolCalls::SetUserUseReserveAsCollateral(element) => element.encode(),
                IL2PoolCalls::Supply(element) => element.encode(),
                IL2PoolCalls::SupplyWithPermit(element) => element.encode(),
                IL2PoolCalls::SwapBorrowRateMode(element) => element.encode(),
                IL2PoolCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IL2PoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IL2PoolCalls::Borrow(element) => element.fmt(f),
                IL2PoolCalls::LiquidationCall(element) => element.fmt(f),
                IL2PoolCalls::RebalanceStableBorrowRate(element) => element.fmt(f),
                IL2PoolCalls::Repay(element) => element.fmt(f),
                IL2PoolCalls::RepayWithATokens(element) => element.fmt(f),
                IL2PoolCalls::RepayWithPermit(element) => element.fmt(f),
                IL2PoolCalls::SetUserUseReserveAsCollateral(element) => element.fmt(f),
                IL2PoolCalls::Supply(element) => element.fmt(f),
                IL2PoolCalls::SupplyWithPermit(element) => element.fmt(f),
                IL2PoolCalls::SwapBorrowRateMode(element) => element.fmt(f),
                IL2PoolCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BorrowCall> for IL2PoolCalls {
        fn from(var: BorrowCall) -> Self {
            IL2PoolCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<LiquidationCallCall> for IL2PoolCalls {
        fn from(var: LiquidationCallCall) -> Self {
            IL2PoolCalls::LiquidationCall(var)
        }
    }
    impl ::std::convert::From<RebalanceStableBorrowRateCall> for IL2PoolCalls {
        fn from(var: RebalanceStableBorrowRateCall) -> Self {
            IL2PoolCalls::RebalanceStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<RepayCall> for IL2PoolCalls {
        fn from(var: RepayCall) -> Self {
            IL2PoolCalls::Repay(var)
        }
    }
    impl ::std::convert::From<RepayWithATokensCall> for IL2PoolCalls {
        fn from(var: RepayWithATokensCall) -> Self {
            IL2PoolCalls::RepayWithATokens(var)
        }
    }
    impl ::std::convert::From<RepayWithPermitCall> for IL2PoolCalls {
        fn from(var: RepayWithPermitCall) -> Self {
            IL2PoolCalls::RepayWithPermit(var)
        }
    }
    impl ::std::convert::From<SetUserUseReserveAsCollateralCall> for IL2PoolCalls {
        fn from(var: SetUserUseReserveAsCollateralCall) -> Self {
            IL2PoolCalls::SetUserUseReserveAsCollateral(var)
        }
    }
    impl ::std::convert::From<SupplyCall> for IL2PoolCalls {
        fn from(var: SupplyCall) -> Self {
            IL2PoolCalls::Supply(var)
        }
    }
    impl ::std::convert::From<SupplyWithPermitCall> for IL2PoolCalls {
        fn from(var: SupplyWithPermitCall) -> Self {
            IL2PoolCalls::SupplyWithPermit(var)
        }
    }
    impl ::std::convert::From<SwapBorrowRateModeCall> for IL2PoolCalls {
        fn from(var: SwapBorrowRateModeCall) -> Self {
            IL2PoolCalls::SwapBorrowRateMode(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IL2PoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IL2PoolCalls::Withdraw(var)
        }
    }
}
