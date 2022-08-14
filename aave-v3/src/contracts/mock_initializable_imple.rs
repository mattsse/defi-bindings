pub use mock_initializable_imple::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_initializable_imple {
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
    #[doc = "MockInitializableImple was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKINITIALIZABLEIMPLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REVISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"txt\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"vals\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setValue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setValueViaProxy\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"text\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"value\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"values\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKINITIALIZABLEIMPLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805534801561001457600080fd5b506105ca806100246000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c80635dd216101161005b5780635dd21610146100b75780635e383d21146100cc578063d31f8b6b146100df578063dde43cba146100f257600080fd5b80631f1bd692146100825780633fa4f245146100a057806355241077146100b7575b600080fd5b61008a6100fa565b6040516100979190610364565b60405180910390f35b6100a960345481565b604051908152602001610097565b6100ca6100c53660046103b9565b603455565b005b6100a96100da3660046103b9565b610188565b6100ca6100ed366004610499565b6101a9565b6100a9600181565b6035805461010790610559565b80601f016020809104026020016040519081016040528092919081815260200182805461013390610559565b80156101805780601f1061015557610100808354040283529160200191610180565b820191906000526020600020905b81548152906001019060200180831161016357829003601f168201915b505050505081565b6036818154811061019857600080fd5b600091825260209091200154905081565b6001805460ff16806101ba5750303b155b806101c6575060005481115b61022d5760405162461bcd60e51b815260206004820152602e60248201527f436f6e747261637420696e7374616e63652068617320616c726561647920626560448201526d195b881a5b9a5d1a585b1a5e995960921b606482015260840160405180910390fd5b60015460ff1615801561024c576001805460ff19168117905560008290555b60348590558351610264906035906020870190610291565b508251610278906036906020860190610315565b50801561028a576001805460ff191690555b5050505050565b82805461029d90610559565b90600052602060002090601f0160209004810192826102bf5760008555610305565b82601f106102d857805160ff1916838001178555610305565b82800160010185558215610305579182015b828111156103055782518255916020019190600101906102ea565b5061031192915061034f565b5090565b82805482825590600052602060002090810192821561030557916020028201828111156103055782518255916020019190600101906102ea565b5b808211156103115760008155600101610350565b600060208083528351808285015260005b8181101561039157858101830151858201604001528201610375565b818111156103a3576000604083870101525b50601f01601f1916929092016040019392505050565b6000602082840312156103cb57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610411576104116103d2565b604052919050565b600082601f83011261042a57600080fd5b8135602067ffffffffffffffff821115610446576104466103d2565b8160051b6104558282016103e8565b928352848101820192828101908785111561046f57600080fd5b83870192505b8483101561048e57823582529183019190830190610475565b979650505050505050565b6000806000606084860312156104ae57600080fd5b8335925060208085013567ffffffffffffffff808211156104ce57600080fd5b818701915087601f8301126104e257600080fd5b8135818111156104f4576104f46103d2565b610506601f8201601f191685016103e8565b818152898583860101111561051a57600080fd5b81858501868301376000918101909401529193506040860135918083111561054157600080fd5b505061054f86828701610419565b9150509250925092565b600181811c9082168061056d57607f821691505b6020821081141561058e57634e487b7160e01b600052602260045260246000fd5b5091905056fea26469706673582212203a2fe3a54b0e3ce5be30ca5e18cad38c2713138c8d57899bfa465ffbcd7fcaa464736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockInitializableImple<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockInitializableImple<M> {
        fn clone(&self) -> Self {
            MockInitializableImple(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockInitializableImple<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockInitializableImple<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockInitializableImple))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockInitializableImple<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKINITIALIZABLEIMPLE_ABI.clone(),
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MOCKINITIALIZABLEIMPLE_ABI.clone(),
                MOCKINITIALIZABLEIMPLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `REVISION` (0xdde43cba) function"]
        pub fn revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 228, 60, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xd31f8b6b) function"]
        pub fn initialize(
            &self,
            val: ethers::core::types::U256,
            txt: String,
            vals: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 31, 139, 107], (val, txt, vals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setValue` (0x55241077) function"]
        pub fn set_value(
            &self,
            new_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 36, 16, 119], new_value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setValueViaProxy` (0x5dd21610) function"]
        pub fn set_value_via_proxy(
            &self,
            new_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 210, 22, 16], new_value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `text` (0x1f1bd692) function"]
        pub fn text(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([31, 27, 214, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `value` (0x3fa4f245) function"]
        pub fn value(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([63, 164, 242, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `values` (0x5e383d21) function"]
        pub fn values(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 56, 61, 33], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockInitializableImple<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `REVISION` function with signature `REVISION()` and selector `[221, 228, 60, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "REVISION", abi = "REVISION()")]
    pub struct RevisionCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256,string,uint256[])` and selector `[211, 31, 139, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,string,uint256[])")]
    pub struct InitializeCall {
        pub val: ethers::core::types::U256,
        pub txt: String,
        pub vals: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `setValue` function with signature `setValue(uint256)` and selector `[85, 36, 16, 119]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setValue", abi = "setValue(uint256)")]
    pub struct SetValueCall {
        pub new_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setValueViaProxy` function with signature `setValueViaProxy(uint256)` and selector `[93, 210, 22, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setValueViaProxy", abi = "setValueViaProxy(uint256)")]
    pub struct SetValueViaProxyCall {
        pub new_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `text` function with signature `text()` and selector `[31, 27, 214, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "text", abi = "text()")]
    pub struct TextCall;
    #[doc = "Container type for all input parameters for the `value` function with signature `value()` and selector `[63, 164, 242, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "value", abi = "value()")]
    pub struct ValueCall;
    #[doc = "Container type for all input parameters for the `values` function with signature `values(uint256)` and selector `[94, 56, 61, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "values", abi = "values(uint256)")]
    pub struct ValuesCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockInitializableImpleCalls {
        Revision(RevisionCall),
        Initialize(InitializeCall),
        SetValue(SetValueCall),
        SetValueViaProxy(SetValueViaProxyCall),
        Text(TextCall),
        Value(ValueCall),
        Values(ValuesCall),
    }
    impl ethers::core::abi::AbiDecode for MockInitializableImpleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <RevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::Revision(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SetValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::SetValue(decoded));
            }
            if let Ok(decoded) =
                <SetValueViaProxyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::SetValueViaProxy(decoded));
            }
            if let Ok(decoded) = <TextCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockInitializableImpleCalls::Text(decoded));
            }
            if let Ok(decoded) = <ValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::Value(decoded));
            }
            if let Ok(decoded) = <ValuesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockInitializableImpleCalls::Values(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockInitializableImpleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockInitializableImpleCalls::Revision(element) => element.encode(),
                MockInitializableImpleCalls::Initialize(element) => element.encode(),
                MockInitializableImpleCalls::SetValue(element) => element.encode(),
                MockInitializableImpleCalls::SetValueViaProxy(element) => element.encode(),
                MockInitializableImpleCalls::Text(element) => element.encode(),
                MockInitializableImpleCalls::Value(element) => element.encode(),
                MockInitializableImpleCalls::Values(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockInitializableImpleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockInitializableImpleCalls::Revision(element) => element.fmt(f),
                MockInitializableImpleCalls::Initialize(element) => element.fmt(f),
                MockInitializableImpleCalls::SetValue(element) => element.fmt(f),
                MockInitializableImpleCalls::SetValueViaProxy(element) => element.fmt(f),
                MockInitializableImpleCalls::Text(element) => element.fmt(f),
                MockInitializableImpleCalls::Value(element) => element.fmt(f),
                MockInitializableImpleCalls::Values(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<RevisionCall> for MockInitializableImpleCalls {
        fn from(var: RevisionCall) -> Self {
            MockInitializableImpleCalls::Revision(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockInitializableImpleCalls {
        fn from(var: InitializeCall) -> Self {
            MockInitializableImpleCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<SetValueCall> for MockInitializableImpleCalls {
        fn from(var: SetValueCall) -> Self {
            MockInitializableImpleCalls::SetValue(var)
        }
    }
    impl ::std::convert::From<SetValueViaProxyCall> for MockInitializableImpleCalls {
        fn from(var: SetValueViaProxyCall) -> Self {
            MockInitializableImpleCalls::SetValueViaProxy(var)
        }
    }
    impl ::std::convert::From<TextCall> for MockInitializableImpleCalls {
        fn from(var: TextCall) -> Self {
            MockInitializableImpleCalls::Text(var)
        }
    }
    impl ::std::convert::From<ValueCall> for MockInitializableImpleCalls {
        fn from(var: ValueCall) -> Self {
            MockInitializableImpleCalls::Value(var)
        }
    }
    impl ::std::convert::From<ValuesCall> for MockInitializableImpleCalls {
        fn from(var: ValuesCall) -> Self {
            MockInitializableImpleCalls::Values(var)
        }
    }
    #[doc = "Container type for all return fields from the `REVISION` function with signature `REVISION()` and selector `[221, 228, 60, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RevisionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `text` function with signature `text()` and selector `[31, 27, 214, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TextReturn(pub String);
    #[doc = "Container type for all return fields from the `value` function with signature `value()` and selector `[63, 164, 242, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ValueReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `values` function with signature `values(uint256)` and selector `[94, 56, 61, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ValuesReturn(pub ethers::core::types::U256);
}
