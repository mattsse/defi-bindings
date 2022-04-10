pub use reserveconfiguration_mod::*;
#[allow(clippy::too_many_arguments)]
mod reserveconfiguration_mod {
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
    #[doc = "ReserveConfiguration was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static RESERVECONFIGURATION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEBT_CEILING_DECIMALS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_RESERVES_COUNT\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static RESERVECONFIGURATION_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60ab610038600b82828239805160001a607314602b57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe7300000000000000000000000000000000000000003014608060405260043610603d5760003560e01c8063280d5de914604257806331b561ba14605c575b600080fd5b6049600281565b6040519081526020015b60405180910390f35b6063608081565b60405161ffff9091168152602001605356fea26469706673582212202369be01fb82b2c8ecf0d05ea1d2bc7cc67c6bff147a60cf2c8109f08660591864736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ReserveConfiguration<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ReserveConfiguration<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ReserveConfiguration<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ReserveConfiguration))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ReserveConfiguration<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                RESERVECONFIGURATION_ABI.clone(),
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
                RESERVECONFIGURATION_ABI.clone(),
                RESERVECONFIGURATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DEBT_CEILING_DECIMALS` (0x280d5de9) function"]
        pub fn debt_ceiling_decimals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([40, 13, 93, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_RESERVES_COUNT` (0x31b561ba) function"]
        pub fn max_reserves_count(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([49, 181, 97, 186], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ReserveConfiguration<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `DEBT_CEILING_DECIMALS`function with signature `DEBT_CEILING_DECIMALS()` and selector `[40, 13, 93, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEBT_CEILING_DECIMALS", abi = "DEBT_CEILING_DECIMALS()")]
    pub struct DebtCeilingDecimalsCall;
    #[doc = "Container type for all input parameters for the `MAX_RESERVES_COUNT`function with signature `MAX_RESERVES_COUNT()` and selector `[49, 181, 97, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_RESERVES_COUNT", abi = "MAX_RESERVES_COUNT()")]
    pub struct MaxReservesCountCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ReserveConfigurationCalls {
        DebtCeilingDecimals(DebtCeilingDecimalsCall),
        MaxReservesCount(MaxReservesCountCall),
    }
    impl ethers::core::abi::AbiDecode for ReserveConfigurationCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DebtCeilingDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReserveConfigurationCalls::DebtCeilingDecimals(decoded));
            }
            if let Ok(decoded) =
                <MaxReservesCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReserveConfigurationCalls::MaxReservesCount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ReserveConfigurationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ReserveConfigurationCalls::DebtCeilingDecimals(element) => element.encode(),
                ReserveConfigurationCalls::MaxReservesCount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ReserveConfigurationCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ReserveConfigurationCalls::DebtCeilingDecimals(element) => element.fmt(f),
                ReserveConfigurationCalls::MaxReservesCount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DebtCeilingDecimalsCall> for ReserveConfigurationCalls {
        fn from(var: DebtCeilingDecimalsCall) -> Self {
            ReserveConfigurationCalls::DebtCeilingDecimals(var)
        }
    }
    impl ::std::convert::From<MaxReservesCountCall> for ReserveConfigurationCalls {
        fn from(var: MaxReservesCountCall) -> Self {
            ReserveConfigurationCalls::MaxReservesCount(var)
        }
    }
}
