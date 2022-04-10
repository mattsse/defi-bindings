pub use ipriceoraclegetter_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipriceoraclegetter_mod {
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
    #[doc = "IPriceOracleGetter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPRICEORACLEGETTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY_UNIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IPRICEORACLEGETTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IPriceOracleGetter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IPriceOracleGetter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPriceOracleGetter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPriceOracleGetter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IPriceOracleGetter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPRICEORACLEGETTER_ABI.clone(), client)
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
                IPRICEORACLEGETTER_ABI.clone(),
                IPRICEORACLEGETTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPriceOracleGetter<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPriceOracleGetterCalls {
        BaseCurrency(BaseCurrencyCall),
        BaseCurrencyUnit(BaseCurrencyUnitCall),
        GetAssetPrice(GetAssetPriceCall),
    }
    impl ethers::core::abi::AbiDecode for IPriceOracleGetterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BaseCurrencyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleGetterCalls::BaseCurrency(decoded));
            }
            if let Ok(decoded) =
                <BaseCurrencyUnitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleGetterCalls::BaseCurrencyUnit(decoded));
            }
            if let Ok(decoded) =
                <GetAssetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleGetterCalls::GetAssetPrice(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPriceOracleGetterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPriceOracleGetterCalls::BaseCurrency(element) => element.encode(),
                IPriceOracleGetterCalls::BaseCurrencyUnit(element) => element.encode(),
                IPriceOracleGetterCalls::GetAssetPrice(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPriceOracleGetterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPriceOracleGetterCalls::BaseCurrency(element) => element.fmt(f),
                IPriceOracleGetterCalls::BaseCurrencyUnit(element) => element.fmt(f),
                IPriceOracleGetterCalls::GetAssetPrice(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BaseCurrencyCall> for IPriceOracleGetterCalls {
        fn from(var: BaseCurrencyCall) -> Self {
            IPriceOracleGetterCalls::BaseCurrency(var)
        }
    }
    impl ::std::convert::From<BaseCurrencyUnitCall> for IPriceOracleGetterCalls {
        fn from(var: BaseCurrencyUnitCall) -> Self {
            IPriceOracleGetterCalls::BaseCurrencyUnit(var)
        }
    }
    impl ::std::convert::From<GetAssetPriceCall> for IPriceOracleGetterCalls {
        fn from(var: GetAssetPriceCall) -> Self {
            IPriceOracleGetterCalls::GetAssetPrice(var)
        }
    }
}
