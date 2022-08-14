pub use i_stable_debt_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_stable_debt_token {
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
    #[doc = "IStableDebtToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISTABLEDEBTTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"currentBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"avgStableRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalSupply\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"currentBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"avgStableRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalSupply\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_ASSET_ADDRESS\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAverageStableRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalSupplyAndAvgRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalSupplyLastUpdated\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserLastUpdated\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserStableRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IStableDebtToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IStableDebtToken<M> {
        fn clone(&self) -> Self {
            IStableDebtToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IStableDebtToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IStableDebtToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IStableDebtToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IStableDebtToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISTABLEDEBTTOKEN_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `UNDERLYING_ASSET_ADDRESS` (0xb16a19de) function"]
        pub fn underlying_asset_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([177, 106, 25, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            from: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 194, 159, 172], (from, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAverageStableRate` (0x90f6fcf2) function"]
        pub fn get_average_stable_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([144, 246, 252, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyData` (0x79774338) function"]
        pub fn get_supply_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u64,
            ),
        > {
            self.0
                .method_hash([121, 119, 67, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalSupplyAndAvgRate` (0xf731e9be) function"]
        pub fn get_total_supply_and_avg_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([247, 49, 233, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalSupplyLastUpdated` (0xe7484890) function"]
        pub fn get_total_supply_last_updated(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 72, 72, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserLastUpdated` (0x79ce6b8c) function"]
        pub fn get_user_last_updated(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([121, 206, 107, 140], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserStableRate` (0xe78c9b3b) function"]
        pub fn get_user_stable_rate(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([231, 140, 155, 59], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc222ec8a) function"]
        pub fn initialize(
            &self,
            pool: ethers::core::types::Address,
            underlying_asset: ethers::core::types::Address,
            incentives_controller: ethers::core::types::Address,
            debt_token_decimals: u8,
            debt_token_name: String,
            debt_token_symbol: String,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 34, 236, 138],
                    (
                        pool,
                        underlying_asset,
                        incentives_controller,
                        debt_token_decimals,
                        debt_token_name,
                        debt_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xb3f1c93d) function"]
        pub fn mint(
            &self,
            user: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (bool, ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([179, 241, 201, 61], (user, on_behalf_of, amount, rate))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `principalBalanceOf` (0xc634dfaa) function"]
        pub fn principal_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 52, 223, 170], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IStableDebtTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IStableDebtToken<M> {
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
        name = "Burn",
        abi = "Burn(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub current_balance: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub avg_stable_rate: ethers::core::types::U256,
        pub new_total_supply: ethers::core::types::U256,
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
        name = "Initialized",
        abi = "Initialized(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: String,
        pub debt_token_symbol: String,
        pub params: ethers::core::types::Bytes,
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
        name = "Mint",
        abi = "Mint(address,address,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub current_balance: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub new_rate: ethers::core::types::U256,
        pub avg_stable_rate: ethers::core::types::U256,
        pub new_total_supply: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IStableDebtTokenEvents {
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
    }
    impl ethers::contract::EthLogDecode for IStableDebtTokenEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IStableDebtTokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IStableDebtTokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IStableDebtTokenEvents::MintFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IStableDebtTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IStableDebtTokenEvents::BurnFilter(element) => element.fmt(f),
                IStableDebtTokenEvents::InitializedFilter(element) => element.fmt(f),
                IStableDebtTokenEvents::MintFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "UNDERLYING_ASSET_ADDRESS", abi = "UNDERLYING_ASSET_ADDRESS()")]
    pub struct UnderlyingAssetAddressCall;
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `[144, 246, 252, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAverageStableRate", abi = "getAverageStableRate()")]
    pub struct GetAverageStableRateCall;
    #[doc = "Container type for all input parameters for the `getSupplyData` function with signature `getSupplyData()` and selector `[121, 119, 67, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSupplyData", abi = "getSupplyData()")]
    pub struct GetSupplyDataCall;
    #[doc = "Container type for all input parameters for the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `[247, 49, 233, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTotalSupplyAndAvgRate", abi = "getTotalSupplyAndAvgRate()")]
    pub struct GetTotalSupplyAndAvgRateCall;
    #[doc = "Container type for all input parameters for the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `[231, 72, 72, 144]`"]
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
        name = "getTotalSupplyLastUpdated",
        abi = "getTotalSupplyLastUpdated()"
    )]
    pub struct GetTotalSupplyLastUpdatedCall;
    #[doc = "Container type for all input parameters for the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `[121, 206, 107, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserLastUpdated", abi = "getUserLastUpdated(address)")]
    pub struct GetUserLastUpdatedCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `[231, 140, 155, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserStableRate", abi = "getUserStableRate(address)")]
    pub struct GetUserStableRateCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint8,string,string,bytes)` and selector `[194, 34, 236, 138]`"]
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
        abi = "initialize(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub pool: ethers::core::types::Address,
        pub underlying_asset: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: String,
        pub debt_token_symbol: String,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,address,uint256,uint256)")]
    pub struct MintCall {
        pub user: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `[198, 52, 223, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "principalBalanceOf", abi = "principalBalanceOf(address)")]
    pub struct PrincipalBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IStableDebtTokenCalls {
        UnderlyingAssetAddress(UnderlyingAssetAddressCall),
        Burn(BurnCall),
        GetAverageStableRate(GetAverageStableRateCall),
        GetSupplyData(GetSupplyDataCall),
        GetTotalSupplyAndAvgRate(GetTotalSupplyAndAvgRateCall),
        GetTotalSupplyLastUpdated(GetTotalSupplyLastUpdatedCall),
        GetUserLastUpdated(GetUserLastUpdatedCall),
        GetUserStableRate(GetUserStableRateCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        PrincipalBalanceOf(PrincipalBalanceOfCall),
    }
    impl ethers::core::abi::AbiDecode for IStableDebtTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <UnderlyingAssetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::UnderlyingAssetAddress(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IStableDebtTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <GetAverageStableRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::GetAverageStableRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::GetSupplyData(decoded));
            }
            if let Ok(decoded) =
                <GetTotalSupplyAndAvgRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IStableDebtTokenCalls::GetTotalSupplyAndAvgRate(decoded));
            }
            if let Ok(decoded) =
                <GetTotalSupplyLastUpdatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IStableDebtTokenCalls::GetTotalSupplyLastUpdated(decoded));
            }
            if let Ok(decoded) =
                <GetUserLastUpdatedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::GetUserLastUpdated(decoded));
            }
            if let Ok(decoded) =
                <GetUserStableRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::GetUserStableRate(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IStableDebtTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <PrincipalBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStableDebtTokenCalls::PrincipalBalanceOf(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IStableDebtTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IStableDebtTokenCalls::UnderlyingAssetAddress(element) => element.encode(),
                IStableDebtTokenCalls::Burn(element) => element.encode(),
                IStableDebtTokenCalls::GetAverageStableRate(element) => element.encode(),
                IStableDebtTokenCalls::GetSupplyData(element) => element.encode(),
                IStableDebtTokenCalls::GetTotalSupplyAndAvgRate(element) => element.encode(),
                IStableDebtTokenCalls::GetTotalSupplyLastUpdated(element) => element.encode(),
                IStableDebtTokenCalls::GetUserLastUpdated(element) => element.encode(),
                IStableDebtTokenCalls::GetUserStableRate(element) => element.encode(),
                IStableDebtTokenCalls::Initialize(element) => element.encode(),
                IStableDebtTokenCalls::Mint(element) => element.encode(),
                IStableDebtTokenCalls::PrincipalBalanceOf(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IStableDebtTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IStableDebtTokenCalls::UnderlyingAssetAddress(element) => element.fmt(f),
                IStableDebtTokenCalls::Burn(element) => element.fmt(f),
                IStableDebtTokenCalls::GetAverageStableRate(element) => element.fmt(f),
                IStableDebtTokenCalls::GetSupplyData(element) => element.fmt(f),
                IStableDebtTokenCalls::GetTotalSupplyAndAvgRate(element) => element.fmt(f),
                IStableDebtTokenCalls::GetTotalSupplyLastUpdated(element) => element.fmt(f),
                IStableDebtTokenCalls::GetUserLastUpdated(element) => element.fmt(f),
                IStableDebtTokenCalls::GetUserStableRate(element) => element.fmt(f),
                IStableDebtTokenCalls::Initialize(element) => element.fmt(f),
                IStableDebtTokenCalls::Mint(element) => element.fmt(f),
                IStableDebtTokenCalls::PrincipalBalanceOf(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<UnderlyingAssetAddressCall> for IStableDebtTokenCalls {
        fn from(var: UnderlyingAssetAddressCall) -> Self {
            IStableDebtTokenCalls::UnderlyingAssetAddress(var)
        }
    }
    impl ::std::convert::From<BurnCall> for IStableDebtTokenCalls {
        fn from(var: BurnCall) -> Self {
            IStableDebtTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<GetAverageStableRateCall> for IStableDebtTokenCalls {
        fn from(var: GetAverageStableRateCall) -> Self {
            IStableDebtTokenCalls::GetAverageStableRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyDataCall> for IStableDebtTokenCalls {
        fn from(var: GetSupplyDataCall) -> Self {
            IStableDebtTokenCalls::GetSupplyData(var)
        }
    }
    impl ::std::convert::From<GetTotalSupplyAndAvgRateCall> for IStableDebtTokenCalls {
        fn from(var: GetTotalSupplyAndAvgRateCall) -> Self {
            IStableDebtTokenCalls::GetTotalSupplyAndAvgRate(var)
        }
    }
    impl ::std::convert::From<GetTotalSupplyLastUpdatedCall> for IStableDebtTokenCalls {
        fn from(var: GetTotalSupplyLastUpdatedCall) -> Self {
            IStableDebtTokenCalls::GetTotalSupplyLastUpdated(var)
        }
    }
    impl ::std::convert::From<GetUserLastUpdatedCall> for IStableDebtTokenCalls {
        fn from(var: GetUserLastUpdatedCall) -> Self {
            IStableDebtTokenCalls::GetUserLastUpdated(var)
        }
    }
    impl ::std::convert::From<GetUserStableRateCall> for IStableDebtTokenCalls {
        fn from(var: GetUserStableRateCall) -> Self {
            IStableDebtTokenCalls::GetUserStableRate(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IStableDebtTokenCalls {
        fn from(var: InitializeCall) -> Self {
            IStableDebtTokenCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MintCall> for IStableDebtTokenCalls {
        fn from(var: MintCall) -> Self {
            IStableDebtTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<PrincipalBalanceOfCall> for IStableDebtTokenCalls {
        fn from(var: PrincipalBalanceOfCall) -> Self {
            IStableDebtTokenCalls::PrincipalBalanceOf(var)
        }
    }
    #[doc = "Container type for all return fields from the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UnderlyingAssetAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BurnReturn(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `[144, 246, 252, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAverageStableRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSupplyData` function with signature `getSupplyData()` and selector `[121, 119, 67, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyDataReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub u64,
    );
    #[doc = "Container type for all return fields from the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `[247, 49, 233, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTotalSupplyAndAvgRateReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `[231, 72, 72, 144]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTotalSupplyLastUpdatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `[121, 206, 107, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserLastUpdatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `[231, 140, 155, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserStableRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintReturn(
        pub bool,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `[198, 52, 223, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PrincipalBalanceOfReturn(pub ethers::core::types::U256);
}
