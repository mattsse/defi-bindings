pub use iownable_mod::*;
#[allow(clippy::too_many_arguments)]
mod iownable_mod {
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
    #[doc = "IOwnable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IOWNABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferPending\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IOWNABLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IOwnable<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IOwnable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IOwnable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOwnable))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IOwnable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IOWNABLE_ABI.clone(), client).into()
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
                IOWNABLE_ABI.clone(),
                IOWNABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `acceptOwnership` (0x79ba5097) function"]
        pub fn accept_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingOwner` (0xe30c3978) function"]
        pub fn pending_owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipAccepted` event"]
        pub fn ownership_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferPending` event"]
        pub fn ownership_transfer_pending_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferPendingFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IOwnableEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IOwnable<M> {
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
    #[ethevent(name = "OwnershipAccepted", abi = "OwnershipAccepted(address)")]
    pub struct OwnershipAcceptedFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
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
        name = "OwnershipTransferPending",
        abi = "OwnershipTransferPending(address)"
    )]
    pub struct OwnershipTransferPendingFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOwnableEvents {
        OwnershipAcceptedFilter(OwnershipAcceptedFilter),
        OwnershipTransferPendingFilter(OwnershipTransferPendingFilter),
    }
    impl ethers::contract::EthLogDecode for IOwnableEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OwnershipAcceptedFilter::decode_log(log) {
                return Ok(IOwnableEvents::OwnershipAcceptedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferPendingFilter::decode_log(log) {
                return Ok(IOwnableEvents::OwnershipTransferPendingFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IOwnableEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOwnableEvents::OwnershipAcceptedFilter(element) => element.fmt(f),
                IOwnableEvents::OwnershipTransferPendingFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptOwnership`function with signature `acceptOwnership()` and selector `[121, 186, 80, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pendingOwner`function with signature `pendingOwner()` and selector `[227, 12, 57, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IOwnableCalls {
        AcceptOwnership(AcceptOwnershipCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for IOwnableCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOwnableCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOwnableCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOwnableCalls::PendingOwner(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOwnableCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IOwnableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOwnableCalls::AcceptOwnership(element) => element.encode(),
                IOwnableCalls::Owner(element) => element.encode(),
                IOwnableCalls::PendingOwner(element) => element.encode(),
                IOwnableCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOwnableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOwnableCalls::AcceptOwnership(element) => element.fmt(f),
                IOwnableCalls::Owner(element) => element.fmt(f),
                IOwnableCalls::PendingOwner(element) => element.fmt(f),
                IOwnableCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for IOwnableCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            IOwnableCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IOwnableCalls {
        fn from(var: OwnerCall) -> Self {
            IOwnableCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingOwnerCall> for IOwnableCalls {
        fn from(var: PendingOwnerCall) -> Self {
            IOwnableCalls::PendingOwner(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for IOwnableCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            IOwnableCalls::TransferOwnership(var)
        }
    }
}
