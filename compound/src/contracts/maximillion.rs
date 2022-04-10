pub use maximillion_mod::*;
#[allow(clippy::too_many_arguments)]
mod maximillion_mod {
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
    #[doc = "Maximillion was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAXIMILLION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract CEther\",\"name\":\"cEther_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cEther\",\"outputs\":[{\"internalType\":\"contract CEther\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"repayBehalf\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract CEther\",\"name\":\"cEther_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"repayBehalfExplicit\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAXIMILLION_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516103363803806103368339818101604052602081101561003357600080fd5b5051600080546001600160a01b039092166001600160a01b03199092169190911790556102d1806100656000396000f3fe6080604052600436106100345760003560e01c806319b68c0014610039578063367b7f051461006a5780639f35c3d51461009a575b600080fd5b34801561004557600080fd5b5061004e6100c0565b604080516001600160a01b039092168252519081900360200190f35b6100986004803603604081101561008057600080fd5b506001600160a01b03813581169160200135166100cf565b005b610098600480360360208110156100b057600080fd5b50356001600160a01b0316610282565b6000546001600160a01b031681565b60003490506000826001600160a01b03166317bfdfbc856040518263ffffffff1660e01b815260040180826001600160a01b03166001600160a01b03168152602001915050602060405180830381600087803b15801561012e57600080fd5b505af1158015610142573d6000803e3d6000fd5b505050506040513d602081101561015857600080fd5b505190508082111561020a57826001600160a01b031663e597461982866040518363ffffffff1660e01b815260040180826001600160a01b03166001600160a01b031681526020019150506000604051808303818588803b1580156101bc57600080fd5b505af11580156101d0573d6000803e3d6000fd5b505060405133935084860380156108fc02935091506000818181858888f19350505050158015610204573d6000803e3d6000fd5b5061027c565b826001600160a01b031663e597461983866040518363ffffffff1660e01b815260040180826001600160a01b03166001600160a01b031681526020019150506000604051808303818588803b15801561026257600080fd5b505af1158015610276573d6000803e3d6000fd5b50505050505b50505050565b6000546102999082906001600160a01b03166100cf565b5056fea265627a7a72315820d1a103654733de4855aec603253a7b89e59018447417a86b97314c09c314e34764736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Maximillion<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Maximillion<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Maximillion<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Maximillion))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Maximillion<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAXIMILLION_ABI.clone(), client).into()
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
                MAXIMILLION_ABI.clone(),
                MAXIMILLION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `cEther` (0x19b68c00) function"]
        pub fn c_ether(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([25, 182, 140, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBehalf` (0x9f35c3d5) function"]
        pub fn repay_behalf(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 53, 195, 213], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayBehalfExplicit` (0x367b7f05) function"]
        pub fn repay_behalf_explicit(
            &self,
            borrower: ethers::core::types::Address,
            c_ether: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 123, 127, 5], (borrower, c_ether))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Maximillion<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `cEther`function with signature `cEther()` and selector `[25, 182, 140, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cEther", abi = "cEther()")]
    pub struct CetherCall;
    #[doc = "Container type for all input parameters for the `repayBehalf`function with signature `repayBehalf(address)` and selector `[159, 53, 195, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repayBehalf", abi = "repayBehalf(address)")]
    pub struct RepayBehalfCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repayBehalfExplicit`function with signature `repayBehalfExplicit(address,address)` and selector `[54, 123, 127, 5]`"]
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
        name = "repayBehalfExplicit",
        abi = "repayBehalfExplicit(address,address)"
    )]
    pub struct RepayBehalfExplicitCall {
        pub borrower: ethers::core::types::Address,
        pub c_ether: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MaximillionCalls {
        Cether(CetherCall),
        RepayBehalf(RepayBehalfCall),
        RepayBehalfExplicit(RepayBehalfExplicitCall),
    }
    impl ethers::core::abi::AbiDecode for MaximillionCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CetherCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaximillionCalls::Cether(decoded));
            }
            if let Ok(decoded) =
                <RepayBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaximillionCalls::RepayBehalf(decoded));
            }
            if let Ok(decoded) =
                <RepayBehalfExplicitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MaximillionCalls::RepayBehalfExplicit(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MaximillionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MaximillionCalls::Cether(element) => element.encode(),
                MaximillionCalls::RepayBehalf(element) => element.encode(),
                MaximillionCalls::RepayBehalfExplicit(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MaximillionCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MaximillionCalls::Cether(element) => element.fmt(f),
                MaximillionCalls::RepayBehalf(element) => element.fmt(f),
                MaximillionCalls::RepayBehalfExplicit(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CetherCall> for MaximillionCalls {
        fn from(var: CetherCall) -> Self {
            MaximillionCalls::Cether(var)
        }
    }
    impl ::std::convert::From<RepayBehalfCall> for MaximillionCalls {
        fn from(var: RepayBehalfCall) -> Self {
            MaximillionCalls::RepayBehalf(var)
        }
    }
    impl ::std::convert::From<RepayBehalfExplicitCall> for MaximillionCalls {
        fn from(var: RepayBehalfExplicitCall) -> Self {
            MaximillionCalls::RepayBehalfExplicit(var)
        }
    }
}
