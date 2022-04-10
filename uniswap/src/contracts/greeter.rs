pub use greeter_mod::*;
#[allow(clippy::too_many_arguments)]
mod greeter_mod {
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
    #[doc = "Greeter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GREETER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"gm\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_greeting\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"greet\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"greeting\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static GREETER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001a3361001f565b61006f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6106c58061007e6000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c8063715018a6146100675780638da5cb5b14610071578063c0129d4314610091578063ead710c414610099578063ef690cc0146100ac578063f2fde38b146100c1575b600080fd5b61006f6100d4565b005b6000546040516001600160a01b0390911681526020015b60405180910390f35b61006f610113565b61006f6100a736600461047d565b6101af565b6100b461025c565b604051610088919061055e565b61006f6100cf366004610591565b6102ea565b6000546001600160a01b031633146101075760405162461bcd60e51b81526004016100fe906105c1565b60405180910390fd5b610111600061037e565b565b6000546001600160a01b0316331461013d5760405162461bcd60e51b81526004016100fe906105c1565b610148600a436105f6565b60001460405180606001604052806021815260200161066f60219139906101825760405162461bcd60e51b81526004016100fe919061055e565b5060408051808201909152600280825261676d60f01b60209092019182526101ac916001916103ce565b50565b7f71b78290913af2addd8fcbe5766de306af2c8afbc466ca891e207f73638c7270816040516020016101e19190610618565b6040516020818303038152906040528051906020012014156040518060400160405280601481526020017363616e6e6f74206772656574207769746820676d60601b815250906102445760405162461bcd60e51b81526004016100fe919061055e565b5080516102589060019060208401906103ce565b5050565b6001805461026990610634565b80601f016020809104026020016040519081016040528092919081815260200182805461029590610634565b80156102e25780601f106102b7576101008083540402835291602001916102e2565b820191906000526020600020905b8154815290600101906020018083116102c557829003601f168201915b505050505081565b6000546001600160a01b031633146103145760405162461bcd60e51b81526004016100fe906105c1565b6001600160a01b0381166103795760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016100fe565b6101ac815b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b8280546103da90610634565b90600052602060002090601f0160209004810192826103fc5760008555610442565b82601f1061041557805160ff1916838001178555610442565b82800160010185558215610442579182015b82811115610442578251825591602001919060010190610427565b5061044e929150610452565b5090565b5b8082111561044e5760008155600101610453565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561048f57600080fd5b813567ffffffffffffffff808211156104a757600080fd5b818401915084601f8301126104bb57600080fd5b8135818111156104cd576104cd610467565b604051601f8201601f19908116603f011681019083821181831017156104f5576104f5610467565b8160405282815287602084870101111561050e57600080fd5b826020860160208301376000928101602001929092525095945050505050565b60005b83811015610549578181015183820152602001610531565b83811115610558576000848401525b50505050565b602081526000825180602084015261057d81604085016020870161052e565b601f01601f19169190910160400192915050565b6000602082840312156105a357600080fd5b81356001600160a01b03811681146105ba57600080fd5b9392505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b60008261061357634e487b7160e01b600052601260045260246000fd5b500690565b6000825161062a81846020870161052e565b9190910192915050565b600181811c9082168061064857607f821691505b60208210810361066857634e487b7160e01b600052602260045260246000fd5b5091905056fe696e76616c696420626c6f636b206e756d6265722c20706c656173652077616974a26469706673582212208a080f4ab1cd9aef0205233bb6e8b8c046d113364b1c43a48f5eae3ea64599da64736f6c634300080d0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Greeter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Greeter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Greeter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Greeter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Greeter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GREETER_ABI.clone(), client).into()
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
                GREETER_ABI.clone(),
                GREETER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `gm` (0xc0129d43) function"]
        pub fn gm(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 18, 157, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `greet` (0xead710c4) function"]
        pub fn greet(&self, greeting: String) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 215, 16, 196], greeting)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `greeting` (0xef690cc0) function"]
        pub fn greeting(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([239, 105, 12, 192], ())
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Greeter<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `gm`function with signature `gm()` and selector `[192, 18, 157, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gm", abi = "gm()")]
    pub struct GmCall;
    #[doc = "Container type for all input parameters for the `greet`function with signature `greet(string)` and selector `[234, 215, 16, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "greet", abi = "greet(string)")]
    pub struct GreetCall {
        pub greeting: String,
    }
    #[doc = "Container type for all input parameters for the `greeting`function with signature `greeting()` and selector `[239, 105, 12, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "greeting", abi = "greeting()")]
    pub struct GreetingCall;
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
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GreeterCalls {
        Gm(GmCall),
        Greet(GreetCall),
        Greeting(GreetingCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for GreeterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <GmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(GreeterCalls::Gm(decoded));
            }
            if let Ok(decoded) = <GreetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreeterCalls::Greet(decoded));
            }
            if let Ok(decoded) =
                <GreetingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreeterCalls::Greeting(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreeterCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreeterCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreeterCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GreeterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GreeterCalls::Gm(element) => element.encode(),
                GreeterCalls::Greet(element) => element.encode(),
                GreeterCalls::Greeting(element) => element.encode(),
                GreeterCalls::Owner(element) => element.encode(),
                GreeterCalls::RenounceOwnership(element) => element.encode(),
                GreeterCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GreeterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GreeterCalls::Gm(element) => element.fmt(f),
                GreeterCalls::Greet(element) => element.fmt(f),
                GreeterCalls::Greeting(element) => element.fmt(f),
                GreeterCalls::Owner(element) => element.fmt(f),
                GreeterCalls::RenounceOwnership(element) => element.fmt(f),
                GreeterCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GmCall> for GreeterCalls {
        fn from(var: GmCall) -> Self {
            GreeterCalls::Gm(var)
        }
    }
    impl ::std::convert::From<GreetCall> for GreeterCalls {
        fn from(var: GreetCall) -> Self {
            GreeterCalls::Greet(var)
        }
    }
    impl ::std::convert::From<GreetingCall> for GreeterCalls {
        fn from(var: GreetingCall) -> Self {
            GreeterCalls::Greeting(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for GreeterCalls {
        fn from(var: OwnerCall) -> Self {
            GreeterCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for GreeterCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            GreeterCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for GreeterCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            GreeterCalls::TransferOwnership(var)
        }
    }
}
