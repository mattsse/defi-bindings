pub use i_pool_addresses_provider_registry::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_pool_addresses_provider_registry {
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
    #[doc = "IPoolAddressesProviderRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLADDRESSESPROVIDERREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressesProviderRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressesProviderUnregistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProviderAddressById\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProviderIdByAddress\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProvidersList\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerAddressesProvider\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unregisterAddressesProvider\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IPoolAddressesProviderRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPoolAddressesProviderRegistry<M> {
        fn clone(&self) -> Self {
            IPoolAddressesProviderRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPoolAddressesProviderRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPoolAddressesProviderRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPoolAddressesProviderRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPoolAddressesProviderRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IPOOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `getAddressesProviderAddressById` (0x57dc0566) function"]
        pub fn get_addresses_provider_address_by_id(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([87, 220, 5, 102], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProviderIdByAddress` (0xd0267be7) function"]
        pub fn get_addresses_provider_id_by_address(
            &self,
            addresses_provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([208, 38, 123, 231], addresses_provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProvidersList` (0x365ccbbf) function"]
        pub fn get_addresses_providers_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([54, 92, 203, 191], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAddressesProvider` (0xd258191e) function"]
        pub fn register_addresses_provider(
            &self,
            provider: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 88, 25, 30], (provider, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unregisterAddressesProvider` (0x0de26707) function"]
        pub fn unregister_addresses_provider(
            &self,
            provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 226, 103, 7], provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddressesProviderRegistered` event"]
        pub fn addresses_provider_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressesProviderRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressesProviderUnregistered` event"]
        pub fn addresses_provider_unregistered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressesProviderUnregisteredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, IPoolAddressesProviderRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPoolAddressesProviderRegistry<M>
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
    #[ethevent(
        name = "AddressesProviderRegistered",
        abi = "AddressesProviderRegistered(address,uint256)"
    )]
    pub struct AddressesProviderRegisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
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
        name = "AddressesProviderUnregistered",
        abi = "AddressesProviderUnregistered(address,uint256)"
    )]
    pub struct AddressesProviderUnregisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolAddressesProviderRegistryEvents {
        AddressesProviderRegisteredFilter(AddressesProviderRegisteredFilter),
        AddressesProviderUnregisteredFilter(AddressesProviderUnregisteredFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolAddressesProviderRegistryEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressesProviderRegisteredFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AddressesProviderUnregisteredFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolAddressesProviderRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(
                    element,
                ) => element.fmt(f),
                IPoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                    element,
                ) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getAddressesProviderAddressById` function with signature `getAddressesProviderAddressById(uint256)` and selector `[87, 220, 5, 102]`"]
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
        name = "getAddressesProviderAddressById",
        abi = "getAddressesProviderAddressById(uint256)"
    )]
    pub struct GetAddressesProviderAddressByIdCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAddressesProviderIdByAddress` function with signature `getAddressesProviderIdByAddress(address)` and selector `[208, 38, 123, 231]`"]
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
        name = "getAddressesProviderIdByAddress",
        abi = "getAddressesProviderIdByAddress(address)"
    )]
    pub struct GetAddressesProviderIdByAddressCall {
        pub addresses_provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAddressesProvidersList` function with signature `getAddressesProvidersList()` and selector `[54, 92, 203, 191]`"]
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
        name = "getAddressesProvidersList",
        abi = "getAddressesProvidersList()"
    )]
    pub struct GetAddressesProvidersListCall;
    #[doc = "Container type for all input parameters for the `registerAddressesProvider` function with signature `registerAddressesProvider(address,uint256)` and selector `[210, 88, 25, 30]`"]
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
        name = "registerAddressesProvider",
        abi = "registerAddressesProvider(address,uint256)"
    )]
    pub struct RegisterAddressesProviderCall {
        pub provider: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unregisterAddressesProvider` function with signature `unregisterAddressesProvider(address)` and selector `[13, 226, 103, 7]`"]
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
        name = "unregisterAddressesProvider",
        abi = "unregisterAddressesProvider(address)"
    )]
    pub struct UnregisterAddressesProviderCall {
        pub provider: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolAddressesProviderRegistryCalls {
        GetAddressesProviderAddressById(GetAddressesProviderAddressByIdCall),
        GetAddressesProviderIdByAddress(GetAddressesProviderIdByAddressCall),
        GetAddressesProvidersList(GetAddressesProvidersListCall),
        RegisterAddressesProvider(RegisterAddressesProviderCall),
        UnregisterAddressesProvider(UnregisterAddressesProviderCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolAddressesProviderRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAddressesProviderAddressByIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(decoded),
                );
            }
            if let Ok(decoded) =
                <GetAddressesProviderIdByAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(decoded),
                );
            }
            if let Ok(decoded) =
                <GetAddressesProvidersListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolAddressesProviderRegistryCalls::GetAddressesProvidersList(decoded));
            }
            if let Ok(decoded) =
                <RegisterAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolAddressesProviderRegistryCalls::RegisterAddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <UnregisterAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    IPoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolAddressesProviderRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(element) => {
                    element.encode()
                }
                IPoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(element) => {
                    element.encode()
                }
                IPoolAddressesProviderRegistryCalls::GetAddressesProvidersList(element) => {
                    element.encode()
                }
                IPoolAddressesProviderRegistryCalls::RegisterAddressesProvider(element) => {
                    element.encode()
                }
                IPoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for IPoolAddressesProviderRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderRegistryCalls::GetAddressesProvidersList(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderRegistryCalls::RegisterAddressesProvider(element) => {
                    element.fmt(f)
                }
                IPoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<GetAddressesProviderAddressByIdCall>
        for IPoolAddressesProviderRegistryCalls
    {
        fn from(var: GetAddressesProviderAddressByIdCall) -> Self {
            IPoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(var)
        }
    }
    impl ::std::convert::From<GetAddressesProviderIdByAddressCall>
        for IPoolAddressesProviderRegistryCalls
    {
        fn from(var: GetAddressesProviderIdByAddressCall) -> Self {
            IPoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(var)
        }
    }
    impl ::std::convert::From<GetAddressesProvidersListCall> for IPoolAddressesProviderRegistryCalls {
        fn from(var: GetAddressesProvidersListCall) -> Self {
            IPoolAddressesProviderRegistryCalls::GetAddressesProvidersList(var)
        }
    }
    impl ::std::convert::From<RegisterAddressesProviderCall> for IPoolAddressesProviderRegistryCalls {
        fn from(var: RegisterAddressesProviderCall) -> Self {
            IPoolAddressesProviderRegistryCalls::RegisterAddressesProvider(var)
        }
    }
    impl ::std::convert::From<UnregisterAddressesProviderCall> for IPoolAddressesProviderRegistryCalls {
        fn from(var: UnregisterAddressesProviderCall) -> Self {
            IPoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAddressesProviderAddressById` function with signature `getAddressesProviderAddressById(uint256)` and selector `[87, 220, 5, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressesProviderAddressByIdReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAddressesProviderIdByAddress` function with signature `getAddressesProviderIdByAddress(address)` and selector `[208, 38, 123, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressesProviderIdByAddressReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getAddressesProvidersList` function with signature `getAddressesProvidersList()` and selector `[54, 92, 203, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressesProvidersListReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
}
