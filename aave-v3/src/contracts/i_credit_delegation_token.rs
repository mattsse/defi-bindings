pub use icreditdelegationtoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditdelegationtoken_mod {
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
    #[doc = "ICreditDelegationToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITDELEGATIONTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowAllowanceDelegated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveDelegation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowAllowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegationWithSig\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITDELEGATIONTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditDelegationToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditDelegationToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditDelegationToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditDelegationToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditDelegationToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ICREDITDELEGATIONTOKEN_ABI.clone(),
                client,
            )
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
                ICREDITDELEGATIONTOKEN_ABI.clone(),
                ICREDITDELEGATIONTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `approveDelegation` (0xc04a8a10) function"]
        pub fn approve_delegation(
            &self,
            delegatee: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 74, 138, 16], (delegatee, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowAllowance` (0x6bd76d24) function"]
        pub fn borrow_allowance(
            &self,
            from_user: ethers::core::types::Address,
            to_user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([107, 215, 109, 36], (from_user, to_user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegationWithSig` (0x0b52d558) function"]
        pub fn delegation_with_sig(
            &self,
            delegator: ethers::core::types::Address,
            delegatee: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [11, 82, 213, 88],
                    (delegator, delegatee, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BorrowAllowanceDelegated` event"]
        pub fn borrow_allowance_delegated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowAllowanceDelegatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowAllowanceDelegatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICreditDelegationToken<M>
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
        name = "BorrowAllowanceDelegated",
        abi = "BorrowAllowanceDelegated(address,address,address,uint256)"
    )]
    pub struct BorrowAllowanceDelegatedFilter {
        #[ethevent(indexed)]
        pub from_user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `approveDelegation`function with signature `approveDelegation(address,uint256)` and selector `[192, 74, 138, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approveDelegation", abi = "approveDelegation(address,uint256)")]
    pub struct ApproveDelegationCall {
        pub delegatee: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrowAllowance`function with signature `borrowAllowance(address,address)` and selector `[107, 215, 109, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowAllowance", abi = "borrowAllowance(address,address)")]
    pub struct BorrowAllowanceCall {
        pub from_user: ethers::core::types::Address,
        pub to_user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `delegationWithSig`function with signature `delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[11, 82, 213, 88]`"]
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
        name = "delegationWithSig",
        abi = "delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegationWithSigCall {
        pub delegator: ethers::core::types::Address,
        pub delegatee: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditDelegationTokenCalls {
        ApproveDelegation(ApproveDelegationCall),
        BorrowAllowance(BorrowAllowanceCall),
        DelegationWithSig(DelegationWithSigCall),
    }
    impl ethers::core::abi::AbiDecode for ICreditDelegationTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveDelegationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditDelegationTokenCalls::ApproveDelegation(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditDelegationTokenCalls::BorrowAllowance(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditDelegationTokenCalls::DelegationWithSig(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICreditDelegationTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICreditDelegationTokenCalls::ApproveDelegation(element) => element.encode(),
                ICreditDelegationTokenCalls::BorrowAllowance(element) => element.encode(),
                ICreditDelegationTokenCalls::DelegationWithSig(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICreditDelegationTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditDelegationTokenCalls::ApproveDelegation(element) => element.fmt(f),
                ICreditDelegationTokenCalls::BorrowAllowance(element) => element.fmt(f),
                ICreditDelegationTokenCalls::DelegationWithSig(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveDelegationCall> for ICreditDelegationTokenCalls {
        fn from(var: ApproveDelegationCall) -> Self {
            ICreditDelegationTokenCalls::ApproveDelegation(var)
        }
    }
    impl ::std::convert::From<BorrowAllowanceCall> for ICreditDelegationTokenCalls {
        fn from(var: BorrowAllowanceCall) -> Self {
            ICreditDelegationTokenCalls::BorrowAllowance(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigCall> for ICreditDelegationTokenCalls {
        fn from(var: DelegationWithSigCall) -> Self {
            ICreditDelegationTokenCalls::DelegationWithSig(var)
        }
    }
}
