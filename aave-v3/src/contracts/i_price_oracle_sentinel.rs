pub use i_price_oracle_sentinel::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_price_oracle_sentinel {
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
    #[doc = "IPriceOracleSentinel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPRICEORACLESENTINEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newGracePeriod\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GracePeriodUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newSequencerOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SequencerOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSequencerOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBorrowAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isLiquidationAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newGracePeriod\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGracePeriod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newSequencerOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencerOracle\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IPriceOracleSentinel<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPriceOracleSentinel<M> {
        fn clone(&self) -> Self {
            IPriceOracleSentinel(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPriceOracleSentinel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPriceOracleSentinel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPriceOracleSentinel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPriceOracleSentinel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IPRICEORACLESENTINEL_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getGracePeriod` (0xdbd18388) function"]
        pub fn get_grace_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 209, 131, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSequencerOracle` (0x12168dc2) function"]
        pub fn get_sequencer_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([18, 22, 141, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isBorrowAllowed` (0x49aa2e81) function"]
        pub fn is_borrow_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 170, 46, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLiquidationAllowed` (0x7a5d20ea) function"]
        pub fn is_liquidation_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 93, 32, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGracePeriod` (0xf2f65960) function"]
        pub fn set_grace_period(
            &self,
            new_grace_period: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 246, 89, 96], new_grace_period)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSequencerOracle` (0xf0aef31c) function"]
        pub fn set_sequencer_oracle(
            &self,
            new_sequencer_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 174, 243, 28], new_sequencer_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GracePeriodUpdated` event"]
        pub fn grace_period_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GracePeriodUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequencerOracleUpdated` event"]
        pub fn sequencer_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequencerOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPriceOracleSentinelEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPriceOracleSentinel<M>
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
    #[ethevent(name = "GracePeriodUpdated", abi = "GracePeriodUpdated(uint256)")]
    pub struct GracePeriodUpdatedFilter {
        pub new_grace_period: ethers::core::types::U256,
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
        name = "SequencerOracleUpdated",
        abi = "SequencerOracleUpdated(address)"
    )]
    pub struct SequencerOracleUpdatedFilter {
        pub new_sequencer_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPriceOracleSentinelEvents {
        GracePeriodUpdatedFilter(GracePeriodUpdatedFilter),
        SequencerOracleUpdatedFilter(SequencerOracleUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for IPriceOracleSentinelEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = GracePeriodUpdatedFilter::decode_log(log) {
                return Ok(IPriceOracleSentinelEvents::GracePeriodUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SequencerOracleUpdatedFilter::decode_log(log) {
                return Ok(IPriceOracleSentinelEvents::SequencerOracleUpdatedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPriceOracleSentinelEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPriceOracleSentinelEvents::GracePeriodUpdatedFilter(element) => element.fmt(f),
                IPriceOracleSentinelEvents::SequencerOracleUpdatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
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
    #[doc = "Container type for all input parameters for the `getGracePeriod` function with signature `getGracePeriod()` and selector `[219, 209, 131, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getGracePeriod", abi = "getGracePeriod()")]
    pub struct GetGracePeriodCall;
    #[doc = "Container type for all input parameters for the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `[18, 22, 141, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSequencerOracle", abi = "getSequencerOracle()")]
    pub struct GetSequencerOracleCall;
    #[doc = "Container type for all input parameters for the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `[73, 170, 46, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isBorrowAllowed", abi = "isBorrowAllowed()")]
    pub struct IsBorrowAllowedCall;
    #[doc = "Container type for all input parameters for the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `[122, 93, 32, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isLiquidationAllowed", abi = "isLiquidationAllowed()")]
    pub struct IsLiquidationAllowedCall;
    #[doc = "Container type for all input parameters for the `setGracePeriod` function with signature `setGracePeriod(uint256)` and selector `[242, 246, 89, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGracePeriod", abi = "setGracePeriod(uint256)")]
    pub struct SetGracePeriodCall {
        pub new_grace_period: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSequencerOracle` function with signature `setSequencerOracle(address)` and selector `[240, 174, 243, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSequencerOracle", abi = "setSequencerOracle(address)")]
    pub struct SetSequencerOracleCall {
        pub new_sequencer_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPriceOracleSentinelCalls {
        AddressesProvider(AddressesProviderCall),
        GetGracePeriod(GetGracePeriodCall),
        GetSequencerOracle(GetSequencerOracleCall),
        IsBorrowAllowed(IsBorrowAllowedCall),
        IsLiquidationAllowed(IsLiquidationAllowedCall),
        SetGracePeriod(SetGracePeriodCall),
        SetSequencerOracle(SetSequencerOracleCall),
    }
    impl ethers::core::abi::AbiDecode for IPriceOracleSentinelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <GetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::GetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <GetSequencerOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::GetSequencerOracle(decoded));
            }
            if let Ok(decoded) =
                <IsBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::IsBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <IsLiquidationAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::IsLiquidationAllowed(decoded));
            }
            if let Ok(decoded) =
                <SetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::SetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <SetSequencerOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleSentinelCalls::SetSequencerOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPriceOracleSentinelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPriceOracleSentinelCalls::AddressesProvider(element) => element.encode(),
                IPriceOracleSentinelCalls::GetGracePeriod(element) => element.encode(),
                IPriceOracleSentinelCalls::GetSequencerOracle(element) => element.encode(),
                IPriceOracleSentinelCalls::IsBorrowAllowed(element) => element.encode(),
                IPriceOracleSentinelCalls::IsLiquidationAllowed(element) => element.encode(),
                IPriceOracleSentinelCalls::SetGracePeriod(element) => element.encode(),
                IPriceOracleSentinelCalls::SetSequencerOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPriceOracleSentinelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPriceOracleSentinelCalls::AddressesProvider(element) => element.fmt(f),
                IPriceOracleSentinelCalls::GetGracePeriod(element) => element.fmt(f),
                IPriceOracleSentinelCalls::GetSequencerOracle(element) => element.fmt(f),
                IPriceOracleSentinelCalls::IsBorrowAllowed(element) => element.fmt(f),
                IPriceOracleSentinelCalls::IsLiquidationAllowed(element) => element.fmt(f),
                IPriceOracleSentinelCalls::SetGracePeriod(element) => element.fmt(f),
                IPriceOracleSentinelCalls::SetSequencerOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for IPriceOracleSentinelCalls {
        fn from(var: AddressesProviderCall) -> Self {
            IPriceOracleSentinelCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<GetGracePeriodCall> for IPriceOracleSentinelCalls {
        fn from(var: GetGracePeriodCall) -> Self {
            IPriceOracleSentinelCalls::GetGracePeriod(var)
        }
    }
    impl ::std::convert::From<GetSequencerOracleCall> for IPriceOracleSentinelCalls {
        fn from(var: GetSequencerOracleCall) -> Self {
            IPriceOracleSentinelCalls::GetSequencerOracle(var)
        }
    }
    impl ::std::convert::From<IsBorrowAllowedCall> for IPriceOracleSentinelCalls {
        fn from(var: IsBorrowAllowedCall) -> Self {
            IPriceOracleSentinelCalls::IsBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<IsLiquidationAllowedCall> for IPriceOracleSentinelCalls {
        fn from(var: IsLiquidationAllowedCall) -> Self {
            IPriceOracleSentinelCalls::IsLiquidationAllowed(var)
        }
    }
    impl ::std::convert::From<SetGracePeriodCall> for IPriceOracleSentinelCalls {
        fn from(var: SetGracePeriodCall) -> Self {
            IPriceOracleSentinelCalls::SetGracePeriod(var)
        }
    }
    impl ::std::convert::From<SetSequencerOracleCall> for IPriceOracleSentinelCalls {
        fn from(var: SetSequencerOracleCall) -> Self {
            IPriceOracleSentinelCalls::SetSequencerOracle(var)
        }
    }
    #[doc = "Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressesProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getGracePeriod` function with signature `getGracePeriod()` and selector `[219, 209, 131, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetGracePeriodReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `[18, 22, 141, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSequencerOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `[73, 170, 46, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsBorrowAllowedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `[122, 93, 32, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsLiquidationAllowedReturn(pub bool);
}
