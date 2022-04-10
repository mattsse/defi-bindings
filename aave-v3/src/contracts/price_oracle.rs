pub use priceoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod priceoracle_mod {
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
    #[doc = "PriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AssetPriceUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EthPriceUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthUsdPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssetPrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEthUsdPrice\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061020a806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806351323f7214610051578063a0a8045e14610066578063b3596f071461007c578063b951883a146100a5575b600080fd5b61006461005f36600461016f565b6100b8565b005b6001545b60405190815260200160405180910390f35b61006a61008a366004610199565b6001600160a01b031660009081526020819052604090205490565b6100646100b33660046101bb565b610113565b6001600160a01b03821660008181526020818152604091829020849055815192835282018390524282820152517fce6e0b57367bae95ca7198e1172f653ea64a645c16ab586b4cefa9237bfc2d929181900360600190a15050565b6001819055604080518281524260208201527fb4f35977939fa8b5ffe552d517a8ff5223046b1fdd3ee0068ae38d1e2b8d0016910160405180910390a150565b80356001600160a01b038116811461016a57600080fd5b919050565b6000806040838503121561018257600080fd5b61018b83610153565b946020939093013593505050565b6000602082840312156101ab57600080fd5b6101b482610153565b9392505050565b6000602082840312156101cd57600080fd5b503591905056fea2646970667358221220bf858fab356b9fb1b31d9b31609ee75d958bbdf841deff66a05e1ba6eeee304364736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct PriceOracle<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PriceOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEORACLE_ABI.clone(), client).into()
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
                PRICEORACLE_ABI.clone(),
                PRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        #[doc = "Calls the contract's `getEthUsdPrice` (0xa0a8045e) function"]
        pub fn get_eth_usd_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([160, 168, 4, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAssetPrice` (0x51323f72) function"]
        pub fn set_asset_price(
            &self,
            asset: ethers::core::types::Address,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 50, 63, 114], (asset, price))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEthUsdPrice` (0xb951883a) function"]
        pub fn set_eth_usd_price(
            &self,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 81, 136, 58], price)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetPriceUpdated` event"]
        pub fn asset_price_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AssetPriceUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EthPriceUpdated` event"]
        pub fn eth_price_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EthPriceUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PriceOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PriceOracle<M> {
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
        name = "AssetPriceUpdated",
        abi = "AssetPriceUpdated(address,uint256,uint256)"
    )]
    pub struct AssetPriceUpdatedFilter {
        pub asset: ethers::core::types::Address,
        pub price: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
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
    #[ethevent(name = "EthPriceUpdated", abi = "EthPriceUpdated(uint256,uint256)")]
    pub struct EthPriceUpdatedFilter {
        pub price: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleEvents {
        AssetPriceUpdatedFilter(AssetPriceUpdatedFilter),
        EthPriceUpdatedFilter(EthPriceUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for PriceOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetPriceUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::AssetPriceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = EthPriceUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::EthPriceUpdatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PriceOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleEvents::AssetPriceUpdatedFilter(element) => element.fmt(f),
                PriceOracleEvents::EthPriceUpdatedFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `getEthUsdPrice`function with signature `getEthUsdPrice()` and selector `[160, 168, 4, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEthUsdPrice", abi = "getEthUsdPrice()")]
    pub struct GetEthUsdPriceCall;
    #[doc = "Container type for all input parameters for the `setAssetPrice`function with signature `setAssetPrice(address,uint256)` and selector `[81, 50, 63, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAssetPrice", abi = "setAssetPrice(address,uint256)")]
    pub struct SetAssetPriceCall {
        pub asset: ethers::core::types::Address,
        pub price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setEthUsdPrice`function with signature `setEthUsdPrice(uint256)` and selector `[185, 81, 136, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setEthUsdPrice", abi = "setEthUsdPrice(uint256)")]
    pub struct SetEthUsdPriceCall {
        pub price: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleCalls {
        GetAssetPrice(GetAssetPriceCall),
        GetEthUsdPrice(GetEthUsdPriceCall),
        SetAssetPrice(SetAssetPriceCall),
        SetEthUsdPrice(SetEthUsdPriceCall),
    }
    impl ethers::core::abi::AbiDecode for PriceOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAssetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::GetAssetPrice(decoded));
            }
            if let Ok(decoded) =
                <GetEthUsdPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::GetEthUsdPrice(decoded));
            }
            if let Ok(decoded) =
                <SetAssetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::SetAssetPrice(decoded));
            }
            if let Ok(decoded) =
                <SetEthUsdPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::SetEthUsdPrice(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceOracleCalls::GetAssetPrice(element) => element.encode(),
                PriceOracleCalls::GetEthUsdPrice(element) => element.encode(),
                PriceOracleCalls::SetAssetPrice(element) => element.encode(),
                PriceOracleCalls::SetEthUsdPrice(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleCalls::GetAssetPrice(element) => element.fmt(f),
                PriceOracleCalls::GetEthUsdPrice(element) => element.fmt(f),
                PriceOracleCalls::SetAssetPrice(element) => element.fmt(f),
                PriceOracleCalls::SetEthUsdPrice(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAssetPriceCall> for PriceOracleCalls {
        fn from(var: GetAssetPriceCall) -> Self {
            PriceOracleCalls::GetAssetPrice(var)
        }
    }
    impl ::std::convert::From<GetEthUsdPriceCall> for PriceOracleCalls {
        fn from(var: GetEthUsdPriceCall) -> Self {
            PriceOracleCalls::GetEthUsdPrice(var)
        }
    }
    impl ::std::convert::From<SetAssetPriceCall> for PriceOracleCalls {
        fn from(var: SetAssetPriceCall) -> Self {
            PriceOracleCalls::SetAssetPrice(var)
        }
    }
    impl ::std::convert::From<SetEthUsdPriceCall> for PriceOracleCalls {
        fn from(var: SetEthUsdPriceCall) -> Self {
            PriceOracleCalls::SetEthUsdPrice(var)
        }
    }
}
