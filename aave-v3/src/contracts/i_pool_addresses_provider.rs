pub use i_pool_addresses_provider::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_pool_addresses_provider {
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
    #[doc = "IPoolAddressesProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLADDRESSESPROVIDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ACLAdminUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ACLManagerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldImplementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newImplementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSetAsProxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"oldMarketId\",\"type\":\"string\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"newMarketId\",\"type\":\"string\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MarketIdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolConfiguratorUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolDataProviderUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleSentinelUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ProxyCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACLAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACLManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMarketId\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolConfigurator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolDataProvider\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracleSentinel\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAclAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setACLAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAclManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setACLManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newImplementationAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAddressAsProxy\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newMarketId\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMarketId\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPoolConfiguratorImpl\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolConfiguratorImpl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newDataProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolDataProvider\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPoolImpl\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolImpl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracleSentinel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceOracleSentinel\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IPoolAddressesProvider<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPoolAddressesProvider<M> {
        fn clone(&self) -> Self {
            IPoolAddressesProvider(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPoolAddressesProvider<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPoolAddressesProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPoolAddressesProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPoolAddressesProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IPOOLADDRESSESPROVIDER_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `getACLAdmin` (0x0e67178c) function"]
        pub fn get_acl_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([14, 103, 23, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getACLManager` (0x707cd716) function"]
        pub fn get_acl_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([112, 124, 215, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddress` (0x21f8a721) function"]
        pub fn get_address(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 248, 167, 33], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMarketId` (0x568ef470) function"]
        pub fn get_market_id(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([86, 142, 244, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPool` (0x026b1d5f) function"]
        pub fn get_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([2, 107, 29, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolConfigurator` (0x631adfca) function"]
        pub fn get_pool_configurator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 26, 223, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolDataProvider` (0xe860accb) function"]
        pub fn get_pool_data_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 96, 172, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracle` (0xfca513a8) function"]
        pub fn get_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracleSentinel` (0x5eb88d3d) function"]
        pub fn get_price_oracle_sentinel(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 184, 141, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACLAdmin` (0x76d84ffc) function"]
        pub fn set_acl_admin(
            &self,
            new_acl_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 216, 79, 252], new_acl_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACLManager` (0xed301ca9) function"]
        pub fn set_acl_manager(
            &self,
            new_acl_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 48, 28, 169], new_acl_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddress` (0xca446dd9) function"]
        pub fn set_address(
            &self,
            id: [u8; 32],
            new_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 68, 109, 217], (id, new_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddressAsProxy` (0x5dcc528c) function"]
        pub fn set_address_as_proxy(
            &self,
            id: [u8; 32],
            new_implementation_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 204, 82, 140], (id, new_implementation_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMarketId` (0xf67b1847) function"]
        pub fn set_market_id(
            &self,
            new_market_id: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 123, 24, 71], new_market_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolConfiguratorImpl` (0xe4ca28b7) function"]
        pub fn set_pool_configurator_impl(
            &self,
            new_pool_configurator_impl: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 202, 40, 183], new_pool_configurator_impl)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolDataProvider` (0xe44e9ed1) function"]
        pub fn set_pool_data_provider(
            &self,
            new_data_provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 78, 158, 209], new_data_provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolImpl` (0xa1564406) function"]
        pub fn set_pool_impl(
            &self,
            new_pool_impl: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 86, 68, 6], new_pool_impl)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracle` (0x530e784f) function"]
        pub fn set_price_oracle(
            &self,
            new_price_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], new_price_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracleSentinel` (0x74944cec) function"]
        pub fn set_price_oracle_sentinel(
            &self,
            new_price_oracle_sentinel: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 148, 76, 236], new_price_oracle_sentinel)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ACLAdminUpdated` event"]
        pub fn acl_admin_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AcladminUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ACLManagerUpdated` event"]
        pub fn acl_manager_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AclmanagerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers::contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressSetAsProxy` event"]
        pub fn address_set_as_proxy_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressSetAsProxyFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketIdSet` event"]
        pub fn market_id_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketIdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolConfiguratorUpdated` event"]
        pub fn pool_configurator_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolConfiguratorUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolDataProviderUpdated` event"]
        pub fn pool_data_provider_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolDataProviderUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolUpdated` event"]
        pub fn pool_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleSentinelUpdated` event"]
        pub fn price_oracle_sentinel_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceOracleSentinelUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleUpdated` event"]
        pub fn price_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProxyCreated` event"]
        pub fn proxy_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProxyCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPoolAddressesProviderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPoolAddressesProvider<M>
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
    #[ethevent(name = "ACLAdminUpdated", abi = "ACLAdminUpdated(address,address)")]
    pub struct AcladminUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "ACLManagerUpdated", abi = "ACLManagerUpdated(address,address)")]
    pub struct AclmanagerUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "AddressSetAsProxy",
        abi = "AddressSetAsProxy(bytes32,address,address,address)"
    )]
    pub struct AddressSetAsProxyFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ethers::core::types::Address,
        pub old_implementation_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_implementation_address: ethers::core::types::Address,
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
    #[ethevent(name = "MarketIdSet", abi = "MarketIdSet(string,string)")]
    pub struct MarketIdSetFilter {
        #[ethevent(indexed)]
        pub old_market_id: ethers::core::types::H256,
        #[ethevent(indexed)]
        pub new_market_id: ethers::core::types::H256,
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
        name = "PoolConfiguratorUpdated",
        abi = "PoolConfiguratorUpdated(address,address)"
    )]
    pub struct PoolConfiguratorUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PoolDataProviderUpdated",
        abi = "PoolDataProviderUpdated(address,address)"
    )]
    pub struct PoolDataProviderUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "PoolUpdated", abi = "PoolUpdated(address,address)")]
    pub struct PoolUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PriceOracleSentinelUpdated",
        abi = "PriceOracleSentinelUpdated(address,address)"
    )]
    pub struct PriceOracleSentinelUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PriceOracleUpdated",
        abi = "PriceOracleUpdated(address,address)"
    )]
    pub struct PriceOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "ProxyCreated", abi = "ProxyCreated(bytes32,address,address)")]
    pub struct ProxyCreatedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolAddressesProviderEvents {
        AcladminUpdatedFilter(AcladminUpdatedFilter),
        AclmanagerUpdatedFilter(AclmanagerUpdatedFilter),
        AddressSetFilter(AddressSetFilter),
        AddressSetAsProxyFilter(AddressSetAsProxyFilter),
        MarketIdSetFilter(MarketIdSetFilter),
        PoolConfiguratorUpdatedFilter(PoolConfiguratorUpdatedFilter),
        PoolDataProviderUpdatedFilter(PoolDataProviderUpdatedFilter),
        PoolUpdatedFilter(PoolUpdatedFilter),
        PriceOracleSentinelUpdatedFilter(PriceOracleSentinelUpdatedFilter),
        PriceOracleUpdatedFilter(PriceOracleUpdatedFilter),
        ProxyCreatedFilter(ProxyCreatedFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolAddressesProviderEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AcladminUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AcladminUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AclmanagerUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AclmanagerUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = AddressSetAsProxyFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AddressSetAsProxyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MarketIdSetFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::MarketIdSetFilter(decoded));
            }
            if let Ok(decoded) = PoolConfiguratorUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PoolDataProviderUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PoolUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleSentinelUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PriceOracleUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ProxyCreatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::ProxyCreatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolAddressesProviderEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolAddressesProviderEvents::AcladminUpdatedFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::AclmanagerUpdatedFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::AddressSetFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::AddressSetAsProxyFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::MarketIdSetFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderEvents::PoolUpdatedFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderEvents::PriceOracleUpdatedFilter(element) => element.fmt(f),
                IPoolAddressesProviderEvents::ProxyCreatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getACLAdmin` function with signature `getACLAdmin()` and selector `[14, 103, 23, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACLAdmin", abi = "getACLAdmin()")]
    pub struct GetACLAdminCall;
    #[doc = "Container type for all input parameters for the `getACLManager` function with signature `getACLManager()` and selector `[112, 124, 215, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACLManager", abi = "getACLManager()")]
    pub struct GetACLManagerCall;
    #[doc = "Container type for all input parameters for the `getAddress` function with signature `getAddress(bytes32)` and selector `[33, 248, 167, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAddress", abi = "getAddress(bytes32)")]
    pub struct GetAddressCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getMarketId` function with signature `getMarketId()` and selector `[86, 142, 244, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMarketId", abi = "getMarketId()")]
    pub struct GetMarketIdCall;
    #[doc = "Container type for all input parameters for the `getPool` function with signature `getPool()` and selector `[2, 107, 29, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPool", abi = "getPool()")]
    pub struct GetPoolCall;
    #[doc = "Container type for all input parameters for the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `[99, 26, 223, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolConfigurator", abi = "getPoolConfigurator()")]
    pub struct GetPoolConfiguratorCall;
    #[doc = "Container type for all input parameters for the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `[232, 96, 172, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolDataProvider", abi = "getPoolDataProvider()")]
    pub struct GetPoolDataProviderCall;
    #[doc = "Container type for all input parameters for the `getPriceOracle` function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracle", abi = "getPriceOracle()")]
    pub struct GetPriceOracleCall;
    #[doc = "Container type for all input parameters for the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `[94, 184, 141, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracleSentinel", abi = "getPriceOracleSentinel()")]
    pub struct GetPriceOracleSentinelCall;
    #[doc = "Container type for all input parameters for the `setACLAdmin` function with signature `setACLAdmin(address)` and selector `[118, 216, 79, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setACLAdmin", abi = "setACLAdmin(address)")]
    pub struct SetACLAdminCall {
        pub new_acl_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setACLManager` function with signature `setACLManager(address)` and selector `[237, 48, 28, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setACLManager", abi = "setACLManager(address)")]
    pub struct SetACLManagerCall {
        pub new_acl_manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAddress` function with signature `setAddress(bytes32,address)` and selector `[202, 68, 109, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAddress", abi = "setAddress(bytes32,address)")]
    pub struct SetAddressCall {
        pub id: [u8; 32],
        pub new_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAddressAsProxy` function with signature `setAddressAsProxy(bytes32,address)` and selector `[93, 204, 82, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAddressAsProxy", abi = "setAddressAsProxy(bytes32,address)")]
    pub struct SetAddressAsProxyCall {
        pub id: [u8; 32],
        pub new_implementation_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMarketId` function with signature `setMarketId(string)` and selector `[246, 123, 24, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMarketId", abi = "setMarketId(string)")]
    pub struct SetMarketIdCall {
        pub new_market_id: String,
    }
    #[doc = "Container type for all input parameters for the `setPoolConfiguratorImpl` function with signature `setPoolConfiguratorImpl(address)` and selector `[228, 202, 40, 183]`"]
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
        name = "setPoolConfiguratorImpl",
        abi = "setPoolConfiguratorImpl(address)"
    )]
    pub struct SetPoolConfiguratorImplCall {
        pub new_pool_configurator_impl: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPoolDataProvider` function with signature `setPoolDataProvider(address)` and selector `[228, 78, 158, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolDataProvider", abi = "setPoolDataProvider(address)")]
    pub struct SetPoolDataProviderCall {
        pub new_data_provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPoolImpl` function with signature `setPoolImpl(address)` and selector `[161, 86, 68, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolImpl", abi = "setPoolImpl(address)")]
    pub struct SetPoolImplCall {
        pub new_pool_impl: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceOracle` function with signature `setPriceOracle(address)` and selector `[83, 14, 120, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPriceOracle", abi = "setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub new_price_oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceOracleSentinel` function with signature `setPriceOracleSentinel(address)` and selector `[116, 148, 76, 236]`"]
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
        name = "setPriceOracleSentinel",
        abi = "setPriceOracleSentinel(address)"
    )]
    pub struct SetPriceOracleSentinelCall {
        pub new_price_oracle_sentinel: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolAddressesProviderCalls {
        GetACLAdmin(GetACLAdminCall),
        GetACLManager(GetACLManagerCall),
        GetAddress(GetAddressCall),
        GetMarketId(GetMarketIdCall),
        GetPool(GetPoolCall),
        GetPoolConfigurator(GetPoolConfiguratorCall),
        GetPoolDataProvider(GetPoolDataProviderCall),
        GetPriceOracle(GetPriceOracleCall),
        GetPriceOracleSentinel(GetPriceOracleSentinelCall),
        SetACLAdmin(SetACLAdminCall),
        SetACLManager(SetACLManagerCall),
        SetAddress(SetAddressCall),
        SetAddressAsProxy(SetAddressAsProxyCall),
        SetMarketId(SetMarketIdCall),
        SetPoolConfiguratorImpl(SetPoolConfiguratorImplCall),
        SetPoolDataProvider(SetPoolDataProviderCall),
        SetPoolImpl(SetPoolImplCall),
        SetPriceOracle(SetPriceOracleCall),
        SetPriceOracleSentinel(SetPriceOracleSentinelCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolAddressesProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetACLAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetACLAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetACLManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetACLManager(decoded));
            }
            if let Ok(decoded) =
                <GetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetAddress(decoded));
            }
            if let Ok(decoded) =
                <GetMarketIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetMarketId(decoded));
            }
            if let Ok(decoded) =
                <GetPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetPool(decoded));
            }
            if let Ok(decoded) =
                <GetPoolConfiguratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetPoolConfigurator(decoded));
            }
            if let Ok(decoded) =
                <GetPoolDataProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetPoolDataProvider(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleSentinelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::GetPriceOracleSentinel(decoded));
            }
            if let Ok(decoded) =
                <SetACLAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetACLAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetACLManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetACLManager(decoded));
            }
            if let Ok(decoded) =
                <SetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetAddress(decoded));
            }
            if let Ok(decoded) =
                <SetAddressAsProxyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetAddressAsProxy(decoded));
            }
            if let Ok(decoded) =
                <SetMarketIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetMarketId(decoded));
            }
            if let Ok(decoded) =
                <SetPoolConfiguratorImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetPoolConfiguratorImpl(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetPoolDataProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetPoolDataProvider(decoded));
            }
            if let Ok(decoded) =
                <SetPoolImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetPoolImpl(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleSentinelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolAddressesProviderCalls::SetPriceOracleSentinel(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolAddressesProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolAddressesProviderCalls::GetACLAdmin(element) => element.encode(),
                IPoolAddressesProviderCalls::GetACLManager(element) => element.encode(),
                IPoolAddressesProviderCalls::GetAddress(element) => element.encode(),
                IPoolAddressesProviderCalls::GetMarketId(element) => element.encode(),
                IPoolAddressesProviderCalls::GetPool(element) => element.encode(),
                IPoolAddressesProviderCalls::GetPoolConfigurator(element) => element.encode(),
                IPoolAddressesProviderCalls::GetPoolDataProvider(element) => element.encode(),
                IPoolAddressesProviderCalls::GetPriceOracle(element) => element.encode(),
                IPoolAddressesProviderCalls::GetPriceOracleSentinel(element) => element.encode(),
                IPoolAddressesProviderCalls::SetACLAdmin(element) => element.encode(),
                IPoolAddressesProviderCalls::SetACLManager(element) => element.encode(),
                IPoolAddressesProviderCalls::SetAddress(element) => element.encode(),
                IPoolAddressesProviderCalls::SetAddressAsProxy(element) => element.encode(),
                IPoolAddressesProviderCalls::SetMarketId(element) => element.encode(),
                IPoolAddressesProviderCalls::SetPoolConfiguratorImpl(element) => element.encode(),
                IPoolAddressesProviderCalls::SetPoolDataProvider(element) => element.encode(),
                IPoolAddressesProviderCalls::SetPoolImpl(element) => element.encode(),
                IPoolAddressesProviderCalls::SetPriceOracle(element) => element.encode(),
                IPoolAddressesProviderCalls::SetPriceOracleSentinel(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolAddressesProviderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolAddressesProviderCalls::GetACLAdmin(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetACLManager(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetAddress(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetMarketId(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetPool(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetPoolConfigurator(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetPoolDataProvider(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetPriceOracle(element) => element.fmt(f),
                IPoolAddressesProviderCalls::GetPriceOracleSentinel(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetACLAdmin(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetACLManager(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetAddress(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetAddressAsProxy(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetMarketId(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetPoolConfiguratorImpl(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetPoolDataProvider(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetPoolImpl(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetPriceOracle(element) => element.fmt(f),
                IPoolAddressesProviderCalls::SetPriceOracleSentinel(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetACLAdminCall> for IPoolAddressesProviderCalls {
        fn from(var: GetACLAdminCall) -> Self {
            IPoolAddressesProviderCalls::GetACLAdmin(var)
        }
    }
    impl ::std::convert::From<GetACLManagerCall> for IPoolAddressesProviderCalls {
        fn from(var: GetACLManagerCall) -> Self {
            IPoolAddressesProviderCalls::GetACLManager(var)
        }
    }
    impl ::std::convert::From<GetAddressCall> for IPoolAddressesProviderCalls {
        fn from(var: GetAddressCall) -> Self {
            IPoolAddressesProviderCalls::GetAddress(var)
        }
    }
    impl ::std::convert::From<GetMarketIdCall> for IPoolAddressesProviderCalls {
        fn from(var: GetMarketIdCall) -> Self {
            IPoolAddressesProviderCalls::GetMarketId(var)
        }
    }
    impl ::std::convert::From<GetPoolCall> for IPoolAddressesProviderCalls {
        fn from(var: GetPoolCall) -> Self {
            IPoolAddressesProviderCalls::GetPool(var)
        }
    }
    impl ::std::convert::From<GetPoolConfiguratorCall> for IPoolAddressesProviderCalls {
        fn from(var: GetPoolConfiguratorCall) -> Self {
            IPoolAddressesProviderCalls::GetPoolConfigurator(var)
        }
    }
    impl ::std::convert::From<GetPoolDataProviderCall> for IPoolAddressesProviderCalls {
        fn from(var: GetPoolDataProviderCall) -> Self {
            IPoolAddressesProviderCalls::GetPoolDataProvider(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleCall> for IPoolAddressesProviderCalls {
        fn from(var: GetPriceOracleCall) -> Self {
            IPoolAddressesProviderCalls::GetPriceOracle(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleSentinelCall> for IPoolAddressesProviderCalls {
        fn from(var: GetPriceOracleSentinelCall) -> Self {
            IPoolAddressesProviderCalls::GetPriceOracleSentinel(var)
        }
    }
    impl ::std::convert::From<SetACLAdminCall> for IPoolAddressesProviderCalls {
        fn from(var: SetACLAdminCall) -> Self {
            IPoolAddressesProviderCalls::SetACLAdmin(var)
        }
    }
    impl ::std::convert::From<SetACLManagerCall> for IPoolAddressesProviderCalls {
        fn from(var: SetACLManagerCall) -> Self {
            IPoolAddressesProviderCalls::SetACLManager(var)
        }
    }
    impl ::std::convert::From<SetAddressCall> for IPoolAddressesProviderCalls {
        fn from(var: SetAddressCall) -> Self {
            IPoolAddressesProviderCalls::SetAddress(var)
        }
    }
    impl ::std::convert::From<SetAddressAsProxyCall> for IPoolAddressesProviderCalls {
        fn from(var: SetAddressAsProxyCall) -> Self {
            IPoolAddressesProviderCalls::SetAddressAsProxy(var)
        }
    }
    impl ::std::convert::From<SetMarketIdCall> for IPoolAddressesProviderCalls {
        fn from(var: SetMarketIdCall) -> Self {
            IPoolAddressesProviderCalls::SetMarketId(var)
        }
    }
    impl ::std::convert::From<SetPoolConfiguratorImplCall> for IPoolAddressesProviderCalls {
        fn from(var: SetPoolConfiguratorImplCall) -> Self {
            IPoolAddressesProviderCalls::SetPoolConfiguratorImpl(var)
        }
    }
    impl ::std::convert::From<SetPoolDataProviderCall> for IPoolAddressesProviderCalls {
        fn from(var: SetPoolDataProviderCall) -> Self {
            IPoolAddressesProviderCalls::SetPoolDataProvider(var)
        }
    }
    impl ::std::convert::From<SetPoolImplCall> for IPoolAddressesProviderCalls {
        fn from(var: SetPoolImplCall) -> Self {
            IPoolAddressesProviderCalls::SetPoolImpl(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for IPoolAddressesProviderCalls {
        fn from(var: SetPriceOracleCall) -> Self {
            IPoolAddressesProviderCalls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleSentinelCall> for IPoolAddressesProviderCalls {
        fn from(var: SetPriceOracleSentinelCall) -> Self {
            IPoolAddressesProviderCalls::SetPriceOracleSentinel(var)
        }
    }
    #[doc = "Container type for all return fields from the `getACLAdmin` function with signature `getACLAdmin()` and selector `[14, 103, 23, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetACLAdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getACLManager` function with signature `getACLManager()` and selector `[112, 124, 215, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetACLManagerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAddress` function with signature `getAddress(bytes32)` and selector `[33, 248, 167, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMarketId` function with signature `getMarketId()` and selector `[86, 142, 244, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMarketIdReturn(pub String);
    #[doc = "Container type for all return fields from the `getPool` function with signature `getPool()` and selector `[2, 107, 29, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `[99, 26, 223, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolConfiguratorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `[232, 96, 172, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolDataProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPriceOracle` function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriceOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `[94, 184, 141, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriceOracleSentinelReturn(pub ethers::core::types::Address);
}
