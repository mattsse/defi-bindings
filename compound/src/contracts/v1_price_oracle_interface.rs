pub use v1_price_oracle_interface::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod v1_price_oracle_interface {
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
    #[doc = "V1PriceOracleInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static V1PRICEORACLEINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetPrices\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct V1PriceOracleInterface<M>(ethers::contract::Contract<M>);
    impl<M> Clone for V1PriceOracleInterface<M> {
        fn clone(&self) -> Self {
            V1PriceOracleInterface(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for V1PriceOracleInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for V1PriceOracleInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(V1PriceOracleInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> V1PriceOracleInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                V1PRICEORACLEINTERFACE_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `assetPrices` (0x5e9a523c) function"]
        pub fn asset_prices(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 154, 82, 60], asset)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for V1PriceOracleInterface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `assetPrices` function with signature `assetPrices(address)` and selector `[94, 154, 82, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetPrices", abi = "assetPrices(address)")]
    pub struct AssetPricesCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `assetPrices` function with signature `assetPrices(address)` and selector `[94, 154, 82, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetPricesReturn(pub ethers::core::types::U256);
}
