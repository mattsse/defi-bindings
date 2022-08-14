pub use flash_loan_simple_receiver_base::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod flash_loan_simple_receiver_base {
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
    #[doc = "FlashLoanSimpleReceiverBase was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FLASHLOANSIMPLERECEIVERBASE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initiator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct FlashLoanSimpleReceiverBase<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FlashLoanSimpleReceiverBase<M> {
        fn clone(&self) -> Self {
            FlashLoanSimpleReceiverBase(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FlashLoanSimpleReceiverBase<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FlashLoanSimpleReceiverBase<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FlashLoanSimpleReceiverBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FlashLoanSimpleReceiverBase<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                FLASHLOANSIMPLERECEIVERBASE_ABI.clone(),
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
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOperation` (0x1b11d0ff) function"]
        pub fn execute_operation(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            premium: ethers::core::types::U256,
            initiator: ethers::core::types::Address,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [27, 17, 208, 255],
                    (asset, amount, premium, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for FlashLoanSimpleReceiverBase<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[doc = "Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
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
    #[doc = "Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `[27, 17, 208, 255]`"]
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
        name = "executeOperation",
        abi = "executeOperation(address,uint256,uint256,address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
        pub initiator: ethers::core::types::Address,
        pub params: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FlashLoanSimpleReceiverBaseCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
    }
    impl ethers::core::abi::AbiDecode for FlashLoanSimpleReceiverBaseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashLoanSimpleReceiverBaseCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FlashLoanSimpleReceiverBaseCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashLoanSimpleReceiverBaseCalls::ExecuteOperation(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FlashLoanSimpleReceiverBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FlashLoanSimpleReceiverBaseCalls::AddressesProvider(element) => element.encode(),
                FlashLoanSimpleReceiverBaseCalls::Pool(element) => element.encode(),
                FlashLoanSimpleReceiverBaseCalls::ExecuteOperation(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FlashLoanSimpleReceiverBaseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FlashLoanSimpleReceiverBaseCalls::AddressesProvider(element) => element.fmt(f),
                FlashLoanSimpleReceiverBaseCalls::Pool(element) => element.fmt(f),
                FlashLoanSimpleReceiverBaseCalls::ExecuteOperation(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for FlashLoanSimpleReceiverBaseCalls {
        fn from(var: AddressesProviderCall) -> Self {
            FlashLoanSimpleReceiverBaseCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<PoolCall> for FlashLoanSimpleReceiverBaseCalls {
        fn from(var: PoolCall) -> Self {
            FlashLoanSimpleReceiverBaseCalls::Pool(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for FlashLoanSimpleReceiverBaseCalls {
        fn from(var: ExecuteOperationCall) -> Self {
            FlashLoanSimpleReceiverBaseCalls::ExecuteOperation(var)
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
    #[doc = "Container type for all return fields from the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `[27, 17, 208, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExecuteOperationReturn(pub bool);
}
