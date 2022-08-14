pub use aggregator_interface::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod aggregator_interface {
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
    #[doc = "AggregatorInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AGGREGATORINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AnswerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewRound\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct AggregatorInterface<M>(ethers::contract::Contract<M>);
    impl<M> Clone for AggregatorInterface<M> {
        fn clone(&self) -> Self {
            AggregatorInterface(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for AggregatorInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AggregatorInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AggregatorInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> AggregatorInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), AGGREGATORINTERFACE_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getAnswer` (0xb5ab58dc) function"]
        pub fn get_answer(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([181, 171, 88, 220], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimestamp` (0xb633620c) function"]
        pub fn get_timestamp(
            &self,
            round_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestAnswer` (0x50d25bcd) function"]
        pub fn latest_answer(&self) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRound` (0x668a0f02) function"]
        pub fn latest_round(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestTimestamp` (0x8205bf6a) function"]
        pub fn latest_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AnswerUpdated` event"]
        pub fn answer_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AnswerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewRound` event"]
        pub fn new_round_filter(&self) -> ethers::contract::builders::Event<M, NewRoundFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AggregatorInterfaceEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for AggregatorInterface<M>
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: I256,
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        pub updated_at: ethers::core::types::U256,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ethers::core::types::Address,
        pub started_at: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AggregatorInterfaceEvents {
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        NewRoundFilter(NewRoundFilter),
    }
    impl ethers::contract::EthLogDecode for AggregatorInterfaceEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(AggregatorInterfaceEvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(AggregatorInterfaceEvents::NewRoundFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AggregatorInterfaceEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AggregatorInterfaceEvents::AnswerUpdatedFilter(element) => element.fmt(f),
                AggregatorInterfaceEvents::NewRoundFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getAnswer` function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall {
        pub round_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `latestAnswer` function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    #[doc = "Container type for all input parameters for the `latestRound` function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    #[doc = "Container type for all input parameters for the `latestTimestamp` function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AggregatorInterfaceCalls {
        GetAnswer(GetAnswerCall),
        GetTimestamp(GetTimestampCall),
        LatestAnswer(LatestAnswerCall),
        LatestRound(LatestRoundCall),
        LatestTimestamp(LatestTimestampCall),
    }
    impl ethers::core::abi::AbiDecode for AggregatorInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorInterfaceCalls::GetAnswer(decoded));
            }
            if let Ok(decoded) =
                <GetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorInterfaceCalls::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestAnswerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorInterfaceCalls::LatestAnswer(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorInterfaceCalls::LatestRound(decoded));
            }
            if let Ok(decoded) =
                <LatestTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorInterfaceCalls::LatestTimestamp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AggregatorInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AggregatorInterfaceCalls::GetAnswer(element) => element.encode(),
                AggregatorInterfaceCalls::GetTimestamp(element) => element.encode(),
                AggregatorInterfaceCalls::LatestAnswer(element) => element.encode(),
                AggregatorInterfaceCalls::LatestRound(element) => element.encode(),
                AggregatorInterfaceCalls::LatestTimestamp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AggregatorInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AggregatorInterfaceCalls::GetAnswer(element) => element.fmt(f),
                AggregatorInterfaceCalls::GetTimestamp(element) => element.fmt(f),
                AggregatorInterfaceCalls::LatestAnswer(element) => element.fmt(f),
                AggregatorInterfaceCalls::LatestRound(element) => element.fmt(f),
                AggregatorInterfaceCalls::LatestTimestamp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAnswerCall> for AggregatorInterfaceCalls {
        fn from(var: GetAnswerCall) -> Self {
            AggregatorInterfaceCalls::GetAnswer(var)
        }
    }
    impl ::std::convert::From<GetTimestampCall> for AggregatorInterfaceCalls {
        fn from(var: GetTimestampCall) -> Self {
            AggregatorInterfaceCalls::GetTimestamp(var)
        }
    }
    impl ::std::convert::From<LatestAnswerCall> for AggregatorInterfaceCalls {
        fn from(var: LatestAnswerCall) -> Self {
            AggregatorInterfaceCalls::LatestAnswer(var)
        }
    }
    impl ::std::convert::From<LatestRoundCall> for AggregatorInterfaceCalls {
        fn from(var: LatestRoundCall) -> Self {
            AggregatorInterfaceCalls::LatestRound(var)
        }
    }
    impl ::std::convert::From<LatestTimestampCall> for AggregatorInterfaceCalls {
        fn from(var: LatestTimestampCall) -> Self {
            AggregatorInterfaceCalls::LatestTimestamp(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAnswer` function with signature `getAnswer(uint256)` and selector `[181, 171, 88, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAnswerReturn(pub I256);
    #[doc = "Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `[182, 51, 98, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTimestampReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestAnswer` function with signature `latestAnswer()` and selector `[80, 210, 91, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestAnswerReturn(pub I256);
    #[doc = "Container type for all return fields from the `latestRound` function with signature `latestRound()` and selector `[102, 138, 15, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestRoundReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `latestTimestamp` function with signature `latestTimestamp()` and selector `[130, 5, 191, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LatestTimestampReturn(pub ethers::core::types::U256);
}
