pub use reservoir::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod reservoir {
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
    #[doc = "Reservoir was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static RESERVOIR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"dripRate_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract EIP20Interface\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"target_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drip\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dripRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dripStart\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dripped\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"target\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"contract EIP20Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static RESERVOIR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060405161052c38038061052c8339818101604052606081101561003357600080fd5b5080516020820151604090920151436000908155600192909255600280546001600160a01b039485166001600160a01b031991821617909155600380549490921693169290921790915560045561049d8061008f6000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c806388a91a8a1461006757806395f632b3146100815780639f678cca14610089578063d326159214610091578063d4b8399214610099578063fc0c546a146100bd575b600080fd5b61006f6100c5565b60408051918252519081900360200190f35b61006f6100cb565b61006f6100d1565b61006f6102c9565b6100a16102cf565b604080516001600160a01b039092168252519081900360200190f35b6100a16102de565b60005481565b60045481565b600254604080516370a0823160e01b815230600482015290516000926001600160a01b031691839183916370a08231916024808301926020929190829003018186803b15801561012057600080fd5b505afa158015610134573d6000803e3d6000fd5b505050506040513d602081101561014a57600080fd5b50516001546000805460045460035460408051808201909152601281527164726970546f74616c206f766572666c6f7760701b60208201529596509394919390926001600160a01b03909116914391906101a9908790878503906102ed565b905060006101e382866040518060400160405280601381526020017264656c74614472697020756e646572666c6f7760681b8152506103a0565b905060006101f189836103fa565b9050600061022487836040518060400160405280600c81526020016b1d185d5d1bdb1bd9da58d85b60a21b815250610413565b9050806004819055508a6001600160a01b031663a9059cbb87846040518363ffffffff1660e01b815260040180836001600160a01b03166001600160a01b0316815260200182815260200192505050602060405180830381600087803b15801561028d57600080fd5b505af11580156102a1573d6000803e3d6000fd5b505050506040513d60208110156102b757600080fd5b50919b50505050505050505050505090565b60015481565b6003546001600160a01b031681565b6002546001600160a01b031681565b6000836102fc57506000610399565b8383028385828161030957fe5b041483906103955760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561035a578181015183820152602001610342565b50505050905090810190601f1680156103875780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5090505b9392505050565b600081848411156103f25760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561035a578181015183820152602001610342565b505050900390565b600081831161040a57508161040d565b50805b92915050565b600083830182858210156103955760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561035a57818101518382015260200161034256fea265627a7a723158200dd88f6dcfee04dea3636dfcff8802f97734601fbfedb7f56bef5bed81a64b1b64736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    pub struct Reservoir<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Reservoir<M> {
        fn clone(&self) -> Self {
            Reservoir(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Reservoir<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Reservoir<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Reservoir))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Reservoir<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), RESERVOIR_ABI.clone(), client).into()
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                RESERVOIR_ABI.clone(),
                RESERVOIR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `drip` (0x9f678cca) function"]
        pub fn drip(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 103, 140, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dripRate` (0xd3261592) function"]
        pub fn drip_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([211, 38, 21, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dripStart` (0x88a91a8a) function"]
        pub fn drip_start(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([136, 169, 26, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dripped` (0x95f632b3) function"]
        pub fn dripped(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 246, 50, 179], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `target` (0xd4b83992) function"]
        pub fn target(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([212, 184, 57, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Reservoir<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `drip` function with signature `drip()` and selector `[159, 103, 140, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "drip", abi = "drip()")]
    pub struct DripCall;
    #[doc = "Container type for all input parameters for the `dripRate` function with signature `dripRate()` and selector `[211, 38, 21, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dripRate", abi = "dripRate()")]
    pub struct DripRateCall;
    #[doc = "Container type for all input parameters for the `dripStart` function with signature `dripStart()` and selector `[136, 169, 26, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dripStart", abi = "dripStart()")]
    pub struct DripStartCall;
    #[doc = "Container type for all input parameters for the `dripped` function with signature `dripped()` and selector `[149, 246, 50, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dripped", abi = "dripped()")]
    pub struct DrippedCall;
    #[doc = "Container type for all input parameters for the `target` function with signature `target()` and selector `[212, 184, 57, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "target", abi = "target()")]
    pub struct TargetCall;
    #[doc = "Container type for all input parameters for the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ReservoirCalls {
        Drip(DripCall),
        DripRate(DripRateCall),
        DripStart(DripStartCall),
        Dripped(DrippedCall),
        Target(TargetCall),
        Token(TokenCall),
    }
    impl ethers::core::abi::AbiDecode for ReservoirCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <DripCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ReservoirCalls::Drip(decoded));
            }
            if let Ok(decoded) =
                <DripRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservoirCalls::DripRate(decoded));
            }
            if let Ok(decoded) =
                <DripStartCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservoirCalls::DripStart(decoded));
            }
            if let Ok(decoded) =
                <DrippedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservoirCalls::Dripped(decoded));
            }
            if let Ok(decoded) = <TargetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservoirCalls::Target(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservoirCalls::Token(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ReservoirCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ReservoirCalls::Drip(element) => element.encode(),
                ReservoirCalls::DripRate(element) => element.encode(),
                ReservoirCalls::DripStart(element) => element.encode(),
                ReservoirCalls::Dripped(element) => element.encode(),
                ReservoirCalls::Target(element) => element.encode(),
                ReservoirCalls::Token(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ReservoirCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ReservoirCalls::Drip(element) => element.fmt(f),
                ReservoirCalls::DripRate(element) => element.fmt(f),
                ReservoirCalls::DripStart(element) => element.fmt(f),
                ReservoirCalls::Dripped(element) => element.fmt(f),
                ReservoirCalls::Target(element) => element.fmt(f),
                ReservoirCalls::Token(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DripCall> for ReservoirCalls {
        fn from(var: DripCall) -> Self {
            ReservoirCalls::Drip(var)
        }
    }
    impl ::std::convert::From<DripRateCall> for ReservoirCalls {
        fn from(var: DripRateCall) -> Self {
            ReservoirCalls::DripRate(var)
        }
    }
    impl ::std::convert::From<DripStartCall> for ReservoirCalls {
        fn from(var: DripStartCall) -> Self {
            ReservoirCalls::DripStart(var)
        }
    }
    impl ::std::convert::From<DrippedCall> for ReservoirCalls {
        fn from(var: DrippedCall) -> Self {
            ReservoirCalls::Dripped(var)
        }
    }
    impl ::std::convert::From<TargetCall> for ReservoirCalls {
        fn from(var: TargetCall) -> Self {
            ReservoirCalls::Target(var)
        }
    }
    impl ::std::convert::From<TokenCall> for ReservoirCalls {
        fn from(var: TokenCall) -> Self {
            ReservoirCalls::Token(var)
        }
    }
    #[doc = "Container type for all return fields from the `drip` function with signature `drip()` and selector `[159, 103, 140, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DripReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dripRate` function with signature `dripRate()` and selector `[211, 38, 21, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DripRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dripStart` function with signature `dripStart()` and selector `[136, 169, 26, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DripStartReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dripped` function with signature `dripped()` and selector `[149, 246, 50, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DrippedReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `target` function with signature `target()` and selector `[212, 184, 57, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TargetReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenReturn(pub ethers::core::types::Address);
}
