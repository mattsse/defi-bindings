pub use timelockinterface_mod::*;
#[allow(clippy::too_many_arguments)]
mod timelockinterface_mod {
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
    #[doc = "TimelockInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TIMELOCKINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"GRACE_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTransaction\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queueTransaction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queuedTransactions\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TIMELOCKINTERFACE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct TimelockInterface<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for TimelockInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for TimelockInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TimelockInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> TimelockInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TIMELOCKINTERFACE_ABI.clone(), client)
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
                TIMELOCKINTERFACE_ABI.clone(),
                TIMELOCKINTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `GRACE_PERIOD` (0xc1a287e2) function"]
        pub fn grace_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 162, 135, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `acceptAdmin` (0x0e18b681) function"]
        pub fn accept_admin(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 24, 182, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelTransaction` (0x591fcdfe) function"]
        pub fn cancel_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 31, 205, 254], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delay` (0x6a42b8f8) function"]
        pub fn delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 66, 184, 248], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeTransaction` (0x0825f38f) function"]
        pub fn execute_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([8, 37, 243, 143], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queueTransaction` (0x3a66f901) function"]
        pub fn queue_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([58, 102, 249, 1], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queuedTransactions` (0xf2b06537) function"]
        pub fn queued_transactions(
            &self,
            hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([242, 176, 101, 55], hash)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for TimelockInterface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `GRACE_PERIOD`function with signature `GRACE_PERIOD()` and selector `[193, 162, 135, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "GRACE_PERIOD", abi = "GRACE_PERIOD()")]
    pub struct GracePeriodCall;
    #[doc = "Container type for all input parameters for the `acceptAdmin`function with signature `acceptAdmin()` and selector `[14, 24, 182, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptAdmin", abi = "acceptAdmin()")]
    pub struct AcceptAdminCall;
    #[doc = "Container type for all input parameters for the `cancelTransaction`function with signature `cancelTransaction(address,uint256,string,bytes,uint256)` and selector `[89, 31, 205, 254]`"]
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
        name = "cancelTransaction",
        abi = "cancelTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct CancelTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `delay`function with signature `delay()` and selector `[106, 66, 184, 248]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delay", abi = "delay()")]
    pub struct DelayCall;
    #[doc = "Container type for all input parameters for the `executeTransaction`function with signature `executeTransaction(address,uint256,string,bytes,uint256)` and selector `[8, 37, 243, 143]`"]
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
        name = "executeTransaction",
        abi = "executeTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct ExecuteTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `queueTransaction`function with signature `queueTransaction(address,uint256,string,bytes,uint256)` and selector `[58, 102, 249, 1]`"]
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
        name = "queueTransaction",
        abi = "queueTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct QueueTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `queuedTransactions`function with signature `queuedTransactions(bytes32)` and selector `[242, 176, 101, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "queuedTransactions", abi = "queuedTransactions(bytes32)")]
    pub struct QueuedTransactionsCall {
        pub hash: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TimelockInterfaceCalls {
        GracePeriod(GracePeriodCall),
        AcceptAdmin(AcceptAdminCall),
        CancelTransaction(CancelTransactionCall),
        Delay(DelayCall),
        ExecuteTransaction(ExecuteTransactionCall),
        QueueTransaction(QueueTransactionCall),
        QueuedTransactions(QueuedTransactionsCall),
    }
    impl ethers::core::abi::AbiDecode for TimelockInterfaceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) =
                <CancelTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::CancelTransaction(decoded));
            }
            if let Ok(decoded) = <DelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::Delay(decoded));
            }
            if let Ok(decoded) =
                <ExecuteTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::ExecuteTransaction(decoded));
            }
            if let Ok(decoded) =
                <QueueTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::QueueTransaction(decoded));
            }
            if let Ok(decoded) =
                <QueuedTransactionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockInterfaceCalls::QueuedTransactions(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TimelockInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TimelockInterfaceCalls::GracePeriod(element) => element.encode(),
                TimelockInterfaceCalls::AcceptAdmin(element) => element.encode(),
                TimelockInterfaceCalls::CancelTransaction(element) => element.encode(),
                TimelockInterfaceCalls::Delay(element) => element.encode(),
                TimelockInterfaceCalls::ExecuteTransaction(element) => element.encode(),
                TimelockInterfaceCalls::QueueTransaction(element) => element.encode(),
                TimelockInterfaceCalls::QueuedTransactions(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TimelockInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TimelockInterfaceCalls::GracePeriod(element) => element.fmt(f),
                TimelockInterfaceCalls::AcceptAdmin(element) => element.fmt(f),
                TimelockInterfaceCalls::CancelTransaction(element) => element.fmt(f),
                TimelockInterfaceCalls::Delay(element) => element.fmt(f),
                TimelockInterfaceCalls::ExecuteTransaction(element) => element.fmt(f),
                TimelockInterfaceCalls::QueueTransaction(element) => element.fmt(f),
                TimelockInterfaceCalls::QueuedTransactions(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GracePeriodCall> for TimelockInterfaceCalls {
        fn from(var: GracePeriodCall) -> Self {
            TimelockInterfaceCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for TimelockInterfaceCalls {
        fn from(var: AcceptAdminCall) -> Self {
            TimelockInterfaceCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<CancelTransactionCall> for TimelockInterfaceCalls {
        fn from(var: CancelTransactionCall) -> Self {
            TimelockInterfaceCalls::CancelTransaction(var)
        }
    }
    impl ::std::convert::From<DelayCall> for TimelockInterfaceCalls {
        fn from(var: DelayCall) -> Self {
            TimelockInterfaceCalls::Delay(var)
        }
    }
    impl ::std::convert::From<ExecuteTransactionCall> for TimelockInterfaceCalls {
        fn from(var: ExecuteTransactionCall) -> Self {
            TimelockInterfaceCalls::ExecuteTransaction(var)
        }
    }
    impl ::std::convert::From<QueueTransactionCall> for TimelockInterfaceCalls {
        fn from(var: QueueTransactionCall) -> Self {
            TimelockInterfaceCalls::QueueTransaction(var)
        }
    }
    impl ::std::convert::From<QueuedTransactionsCall> for TimelockInterfaceCalls {
        fn from(var: QueuedTransactionsCall) -> Self {
            TimelockInterfaceCalls::QueuedTransactions(var)
        }
    }
}
