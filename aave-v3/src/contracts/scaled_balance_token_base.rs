pub use scaledbalancetokenbase_mod::*;
#[allow(clippy::too_many_arguments)]
mod scaledbalancetokenbase_mod {
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
    #[doc = "ScaledBalanceTokenBase was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SCALEDBALANCETOKENBASE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getIncentivesController\",\"outputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPreviousIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getScaledUserBalanceAndSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIncentivesController\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SCALEDBALANCETOKENBASE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ScaledBalanceTokenBase<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ScaledBalanceTokenBase<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ScaledBalanceTokenBase<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ScaledBalanceTokenBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ScaledBalanceTokenBase<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                SCALEDBALANCETOKENBASE_ABI.clone(),
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
                SCALEDBALANCETOKENBASE_ABI.clone(),
                SCALEDBALANCETOKENBASE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
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
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getIncentivesController` (0x75d26413) function"]
        pub fn get_incentives_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 210, 100, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPreviousIndex` (0xe0753986) function"]
        pub fn get_previous_index(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 117, 57, 134], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getScaledUserBalanceAndSupply` (0x0afbcdc9) function"]
        pub fn get_scaled_user_balance_and_supply(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 251, 205, 201], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledBalanceOf` (0x1da24f3e) function"]
        pub fn scaled_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 162, 79, 62], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledTotalSupply` (0xb1bf962d) function"]
        pub fn scaled_total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 191, 150, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIncentivesController` (0xe655dbd8) function"]
        pub fn set_incentives_controller(
            &self,
            controller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 85, 219, 216], controller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ScaledBalanceTokenBaseEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ScaledBalanceTokenBase<M>
    {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ScaledBalanceTokenBaseEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ScaledBalanceTokenBaseEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ScaledBalanceTokenBaseEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(ScaledBalanceTokenBaseEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(ScaledBalanceTokenBaseEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ScaledBalanceTokenBaseEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ScaledBalanceTokenBaseEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ScaledBalanceTokenBaseEvents::ApprovalFilter(element) => element.fmt(f),
                ScaledBalanceTokenBaseEvents::BurnFilter(element) => element.fmt(f),
                ScaledBalanceTokenBaseEvents::MintFilter(element) => element.fmt(f),
                ScaledBalanceTokenBaseEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `POOL`function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
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
        pub account: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getIncentivesController`function with signature `getIncentivesController()` and selector `[117, 210, 100, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getIncentivesController", abi = "getIncentivesController()")]
    pub struct GetIncentivesControllerCall;
    #[doc = "Container type for all input parameters for the `getPreviousIndex`function with signature `getPreviousIndex(address)` and selector `[224, 117, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPreviousIndex", abi = "getPreviousIndex(address)")]
    pub struct GetPreviousIndexCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getScaledUserBalanceAndSupply`function with signature `getScaledUserBalanceAndSupply(address)` and selector `[10, 251, 205, 201]`"]
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
        name = "getScaledUserBalanceAndSupply",
        abi = "getScaledUserBalanceAndSupply(address)"
    )]
    pub struct GetScaledUserBalanceAndSupplyCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `scaledBalanceOf`function with signature `scaledBalanceOf(address)` and selector `[29, 162, 79, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledBalanceOf", abi = "scaledBalanceOf(address)")]
    pub struct ScaledBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `scaledTotalSupply`function with signature `scaledTotalSupply()` and selector `[177, 191, 150, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledTotalSupply", abi = "scaledTotalSupply()")]
    pub struct ScaledTotalSupplyCall;
    #[doc = "Container type for all input parameters for the `setIncentivesController`function with signature `setIncentivesController(address)` and selector `[230, 85, 219, 216]`"]
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
        name = "setIncentivesController",
        abi = "setIncentivesController(address)"
    )]
    pub struct SetIncentivesControllerCall {
        pub controller: ethers::core::types::Address,
    }
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
        pub recipient: ethers::core::types::Address,
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
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ScaledBalanceTokenBaseCalls {
        Pool(PoolCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        GetIncentivesController(GetIncentivesControllerCall),
        GetPreviousIndex(GetPreviousIndexCall),
        GetScaledUserBalanceAndSupply(GetScaledUserBalanceAndSupplyCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Name(NameCall),
        ScaledBalanceOf(ScaledBalanceOfCall),
        ScaledTotalSupply(ScaledTotalSupplyCall),
        SetIncentivesController(SetIncentivesControllerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for ScaledBalanceTokenBaseCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ScaledBalanceTokenBaseCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <GetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::GetIncentivesController(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetPreviousIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::GetPreviousIndex(decoded));
            }
            if let Ok(decoded) =
                <GetScaledUserBalanceAndSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ScaledBalanceTokenBaseCalls::GetScaledUserBalanceAndSupply(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ScaledBalanceTokenBaseCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <ScaledBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::ScaledBalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ScaledTotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::ScaledTotalSupply(decoded));
            }
            if let Ok(decoded) =
                <SetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::SetIncentivesController(
                    decoded,
                ));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ScaledBalanceTokenBaseCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ScaledBalanceTokenBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ScaledBalanceTokenBaseCalls::Pool(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Allowance(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Approve(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::BalanceOf(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Decimals(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::DecreaseAllowance(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::GetIncentivesController(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::GetPreviousIndex(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::GetScaledUserBalanceAndSupply(element) => {
                    element.encode()
                }
                ScaledBalanceTokenBaseCalls::IncreaseAllowance(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Name(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::ScaledBalanceOf(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::ScaledTotalSupply(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::SetIncentivesController(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Symbol(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::TotalSupply(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::Transfer(element) => element.encode(),
                ScaledBalanceTokenBaseCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ScaledBalanceTokenBaseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ScaledBalanceTokenBaseCalls::Pool(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Allowance(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Approve(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::BalanceOf(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Decimals(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::DecreaseAllowance(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::GetIncentivesController(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::GetPreviousIndex(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::GetScaledUserBalanceAndSupply(element) => {
                    element.fmt(f)
                }
                ScaledBalanceTokenBaseCalls::IncreaseAllowance(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Name(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::ScaledBalanceOf(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::ScaledTotalSupply(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::SetIncentivesController(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Symbol(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::TotalSupply(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::Transfer(element) => element.fmt(f),
                ScaledBalanceTokenBaseCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PoolCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: PoolCall) -> Self {
            ScaledBalanceTokenBaseCalls::Pool(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: AllowanceCall) -> Self {
            ScaledBalanceTokenBaseCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: ApproveCall) -> Self {
            ScaledBalanceTokenBaseCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: BalanceOfCall) -> Self {
            ScaledBalanceTokenBaseCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: DecimalsCall) -> Self {
            ScaledBalanceTokenBaseCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            ScaledBalanceTokenBaseCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<GetIncentivesControllerCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: GetIncentivesControllerCall) -> Self {
            ScaledBalanceTokenBaseCalls::GetIncentivesController(var)
        }
    }
    impl ::std::convert::From<GetPreviousIndexCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: GetPreviousIndexCall) -> Self {
            ScaledBalanceTokenBaseCalls::GetPreviousIndex(var)
        }
    }
    impl ::std::convert::From<GetScaledUserBalanceAndSupplyCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: GetScaledUserBalanceAndSupplyCall) -> Self {
            ScaledBalanceTokenBaseCalls::GetScaledUserBalanceAndSupply(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            ScaledBalanceTokenBaseCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<NameCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: NameCall) -> Self {
            ScaledBalanceTokenBaseCalls::Name(var)
        }
    }
    impl ::std::convert::From<ScaledBalanceOfCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: ScaledBalanceOfCall) -> Self {
            ScaledBalanceTokenBaseCalls::ScaledBalanceOf(var)
        }
    }
    impl ::std::convert::From<ScaledTotalSupplyCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: ScaledTotalSupplyCall) -> Self {
            ScaledBalanceTokenBaseCalls::ScaledTotalSupply(var)
        }
    }
    impl ::std::convert::From<SetIncentivesControllerCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: SetIncentivesControllerCall) -> Self {
            ScaledBalanceTokenBaseCalls::SetIncentivesController(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: SymbolCall) -> Self {
            ScaledBalanceTokenBaseCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ScaledBalanceTokenBaseCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: TransferCall) -> Self {
            ScaledBalanceTokenBaseCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ScaledBalanceTokenBaseCalls {
        fn from(var: TransferFromCall) -> Self {
            ScaledBalanceTokenBaseCalls::TransferFrom(var)
        }
    }
}
