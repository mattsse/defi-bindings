pub use iaaveoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaaveoracle_mod {
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
    #[doc = "IAaveOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAAVEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AssetSourceUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"baseCurrency\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"baseCurrencyUnit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BaseCurrencySet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fallbackOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FallbackOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY_UNIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsPrices\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFallbackOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSourceOfAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"sources\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssetSources\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fallbackOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFallbackOracle\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IAAVEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAaveOracle<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAaveOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAaveOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAaveOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAaveOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IAAVEORACLE_ABI.clone(), client).into()
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
                IAAVEORACLE_ABI.clone(),
                IAAVEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BASE_CURRENCY` (0xe19f4700) function"]
        pub fn base_currency(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([225, 159, 71, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BASE_CURRENCY_UNIT` (0x8c89b64f) function"]
        pub fn base_currency_unit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([140, 137, 182, 79], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetPrice` (0xb3596f07) function"]
        pub fn get_asset_price(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 89, 111, 7], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsPrices` (0x9d23d9f2) function"]
        pub fn get_assets_prices(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([157, 35, 217, 242], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFallbackOracle` (0x6210308c) function"]
        pub fn get_fallback_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([98, 16, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSourceOfAsset` (0x92bf2be0) function"]
        pub fn get_source_of_asset(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([146, 191, 43, 224], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAssetSources` (0xabfd5310) function"]
        pub fn set_asset_sources(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            sources: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 253, 83, 16], (assets, sources))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFallbackOracle` (0x170aee73) function"]
        pub fn set_fallback_oracle(
            &self,
            fallback_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 10, 238, 115], fallback_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetSourceUpdated` event"]
        pub fn asset_source_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AssetSourceUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BaseCurrencySet` event"]
        pub fn base_currency_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BaseCurrencySetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FallbackOracleUpdated` event"]
        pub fn fallback_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FallbackOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAaveOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAaveOracle<M> {
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
        name = "AssetSourceUpdated",
        abi = "AssetSourceUpdated(address,address)"
    )]
    pub struct AssetSourceUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub source: ethers::core::types::Address,
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
    #[ethevent(name = "BaseCurrencySet", abi = "BaseCurrencySet(address,uint256)")]
    pub struct BaseCurrencySetFilter {
        #[ethevent(indexed)]
        pub base_currency: ethers::core::types::Address,
        pub base_currency_unit: ethers::core::types::U256,
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
    #[ethevent(name = "FallbackOracleUpdated", abi = "FallbackOracleUpdated(address)")]
    pub struct FallbackOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub fallback_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveOracleEvents {
        AssetSourceUpdatedFilter(AssetSourceUpdatedFilter),
        BaseCurrencySetFilter(BaseCurrencySetFilter),
        FallbackOracleUpdatedFilter(FallbackOracleUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for IAaveOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetSourceUpdatedFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::AssetSourceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BaseCurrencySetFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::BaseCurrencySetFilter(decoded));
            }
            if let Ok(decoded) = FallbackOracleUpdatedFilter::decode_log(log) {
                return Ok(IAaveOracleEvents::FallbackOracleUpdatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAaveOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveOracleEvents::AssetSourceUpdatedFilter(element) => element.fmt(f),
                IAaveOracleEvents::BaseCurrencySetFilter(element) => element.fmt(f),
                IAaveOracleEvents::FallbackOracleUpdatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER`function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    #[doc = "Container type for all input parameters for the `BASE_CURRENCY`function with signature `BASE_CURRENCY()` and selector `[225, 159, 71, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BASE_CURRENCY", abi = "BASE_CURRENCY()")]
    pub struct BaseCurrencyCall;
    #[doc = "Container type for all input parameters for the `BASE_CURRENCY_UNIT`function with signature `BASE_CURRENCY_UNIT()` and selector `[140, 137, 182, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BASE_CURRENCY_UNIT", abi = "BASE_CURRENCY_UNIT()")]
    pub struct BaseCurrencyUnitCall;
    #[doc = "Container type for all input parameters for the `getAssetPrice`function with signature `getAssetPrice(address)` and selector `[179, 89, 111, 7]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetPrice", abi = "getAssetPrice(address)")]
    pub struct GetAssetPriceCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAssetsPrices`function with signature `getAssetsPrices(address[])` and selector `[157, 35, 217, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsPrices", abi = "getAssetsPrices(address[])")]
    pub struct GetAssetsPricesCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getFallbackOracle`function with signature `getFallbackOracle()` and selector `[98, 16, 48, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getFallbackOracle", abi = "getFallbackOracle()")]
    pub struct GetFallbackOracleCall;
    #[doc = "Container type for all input parameters for the `getSourceOfAsset`function with signature `getSourceOfAsset(address)` and selector `[146, 191, 43, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSourceOfAsset", abi = "getSourceOfAsset(address)")]
    pub struct GetSourceOfAssetCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAssetSources`function with signature `setAssetSources(address[],address[])` and selector `[171, 253, 83, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAssetSources", abi = "setAssetSources(address[],address[])")]
    pub struct SetAssetSourcesCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub sources: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setFallbackOracle`function with signature `setFallbackOracle(address)` and selector `[23, 10, 238, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFallbackOracle", abi = "setFallbackOracle(address)")]
    pub struct SetFallbackOracleCall {
        pub fallback_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveOracleCalls {
        AddressesProvider(AddressesProviderCall),
        BaseCurrency(BaseCurrencyCall),
        BaseCurrencyUnit(BaseCurrencyUnitCall),
        GetAssetPrice(GetAssetPriceCall),
        GetAssetsPrices(GetAssetsPricesCall),
        GetFallbackOracle(GetFallbackOracleCall),
        GetSourceOfAsset(GetSourceOfAssetCall),
        SetAssetSources(SetAssetSourcesCall),
        SetFallbackOracle(SetFallbackOracleCall),
    }
    impl ethers::core::abi::AbiDecode for IAaveOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <BaseCurrencyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::BaseCurrency(decoded));
            }
            if let Ok(decoded) =
                <BaseCurrencyUnitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::BaseCurrencyUnit(decoded));
            }
            if let Ok(decoded) =
                <GetAssetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::GetAssetPrice(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsPricesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::GetAssetsPrices(decoded));
            }
            if let Ok(decoded) =
                <GetFallbackOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::GetFallbackOracle(decoded));
            }
            if let Ok(decoded) =
                <GetSourceOfAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::GetSourceOfAsset(decoded));
            }
            if let Ok(decoded) =
                <SetAssetSourcesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::SetAssetSources(decoded));
            }
            if let Ok(decoded) =
                <SetFallbackOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveOracleCalls::SetFallbackOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveOracleCalls::AddressesProvider(element) => element.encode(),
                IAaveOracleCalls::BaseCurrency(element) => element.encode(),
                IAaveOracleCalls::BaseCurrencyUnit(element) => element.encode(),
                IAaveOracleCalls::GetAssetPrice(element) => element.encode(),
                IAaveOracleCalls::GetAssetsPrices(element) => element.encode(),
                IAaveOracleCalls::GetFallbackOracle(element) => element.encode(),
                IAaveOracleCalls::GetSourceOfAsset(element) => element.encode(),
                IAaveOracleCalls::SetAssetSources(element) => element.encode(),
                IAaveOracleCalls::SetFallbackOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveOracleCalls::AddressesProvider(element) => element.fmt(f),
                IAaveOracleCalls::BaseCurrency(element) => element.fmt(f),
                IAaveOracleCalls::BaseCurrencyUnit(element) => element.fmt(f),
                IAaveOracleCalls::GetAssetPrice(element) => element.fmt(f),
                IAaveOracleCalls::GetAssetsPrices(element) => element.fmt(f),
                IAaveOracleCalls::GetFallbackOracle(element) => element.fmt(f),
                IAaveOracleCalls::GetSourceOfAsset(element) => element.fmt(f),
                IAaveOracleCalls::SetAssetSources(element) => element.fmt(f),
                IAaveOracleCalls::SetFallbackOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for IAaveOracleCalls {
        fn from(var: AddressesProviderCall) -> Self {
            IAaveOracleCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<BaseCurrencyCall> for IAaveOracleCalls {
        fn from(var: BaseCurrencyCall) -> Self {
            IAaveOracleCalls::BaseCurrency(var)
        }
    }
    impl ::std::convert::From<BaseCurrencyUnitCall> for IAaveOracleCalls {
        fn from(var: BaseCurrencyUnitCall) -> Self {
            IAaveOracleCalls::BaseCurrencyUnit(var)
        }
    }
    impl ::std::convert::From<GetAssetPriceCall> for IAaveOracleCalls {
        fn from(var: GetAssetPriceCall) -> Self {
            IAaveOracleCalls::GetAssetPrice(var)
        }
    }
    impl ::std::convert::From<GetAssetsPricesCall> for IAaveOracleCalls {
        fn from(var: GetAssetsPricesCall) -> Self {
            IAaveOracleCalls::GetAssetsPrices(var)
        }
    }
    impl ::std::convert::From<GetFallbackOracleCall> for IAaveOracleCalls {
        fn from(var: GetFallbackOracleCall) -> Self {
            IAaveOracleCalls::GetFallbackOracle(var)
        }
    }
    impl ::std::convert::From<GetSourceOfAssetCall> for IAaveOracleCalls {
        fn from(var: GetSourceOfAssetCall) -> Self {
            IAaveOracleCalls::GetSourceOfAsset(var)
        }
    }
    impl ::std::convert::From<SetAssetSourcesCall> for IAaveOracleCalls {
        fn from(var: SetAssetSourcesCall) -> Self {
            IAaveOracleCalls::SetAssetSources(var)
        }
    }
    impl ::std::convert::From<SetFallbackOracleCall> for IAaveOracleCalls {
        fn from(var: SetFallbackOracleCall) -> Self {
            IAaveOracleCalls::SetFallbackOracle(var)
        }
    }
}
