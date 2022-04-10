pub use revertingerc20_mod::*;
#[allow(clippy::too_many_arguments)]
mod revertingerc20_mod {
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
    #[doc = "RevertingERC20 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static REVERTINGERC20_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static REVERTINGERC20_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506101ad806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806340c10f191461004657806370a082311461005b578063a9059cbb1461008e575b600080fd5b610059610054366004610127565b6100b1565b005b61007b610069366004610105565b60006020819052908152604090205481565b6040519081526020015b60405180910390f35b6100a161009c366004610127565b6100e2565b6040519015158152602001610085565b6001600160a01b038216600090815260208190526040812080548392906100d9908490610151565b90915550505050565b6000806000fd5b80356001600160a01b038116811461010057600080fd5b919050565b60006020828403121561011757600080fd5b610120826100e9565b9392505050565b6000806040838503121561013a57600080fd5b610143836100e9565b946020939093013593505050565b6000821982111561017257634e487b7160e01b600052601160045260246000fd5b50019056fea264697066735822122083ba103ef137e0c6baf7972b53dc4ee19a965169016a528c7d0b53dd2cd4a10e64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct RevertingERC20<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for RevertingERC20<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for RevertingERC20<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(RevertingERC20))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> RevertingERC20<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), REVERTINGERC20_ABI.clone(), client)
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
                REVERTINGERC20_ABI.clone(),
                REVERTINGERC20_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for RevertingERC20<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum RevertingERC20Calls {
        BalanceOf(BalanceOfCall),
        Mint(MintCall),
        Transfer(TransferCall),
    }
    impl ethers::core::abi::AbiDecode for RevertingERC20Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RevertingERC20Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(RevertingERC20Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(RevertingERC20Calls::Transfer(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for RevertingERC20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                RevertingERC20Calls::BalanceOf(element) => element.encode(),
                RevertingERC20Calls::Mint(element) => element.encode(),
                RevertingERC20Calls::Transfer(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for RevertingERC20Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                RevertingERC20Calls::BalanceOf(element) => element.fmt(f),
                RevertingERC20Calls::Mint(element) => element.fmt(f),
                RevertingERC20Calls::Transfer(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for RevertingERC20Calls {
        fn from(var: BalanceOfCall) -> Self {
            RevertingERC20Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<MintCall> for RevertingERC20Calls {
        fn from(var: MintCall) -> Self {
            RevertingERC20Calls::Mint(var)
        }
    }
    impl ::std::convert::From<TransferCall> for RevertingERC20Calls {
        fn from(var: TransferCall) -> Self {
            RevertingERC20Calls::Transfer(var)
        }
    }
}
