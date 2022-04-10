pub use erc20user_mod::*;
#[allow(clippy::too_many_arguments)]
mod erc20user_mod {
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
    #[doc = "ERC20User was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERC20USER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC20USER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610477806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806305eee8e71461005157806351cf23b1146100665780637fe2558114610079578063fc179d67146100a0575b600080fd5b61006461005f366004610359565b6100b3565b005b6100646100743660046103dc565b610145565b61008c6100873660046103dc565b6101cd565b604051901515815260200160405180910390f35b61008c6100ae36600461030e565b61025b565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b15801561012357600080fd5b505af1158015610137573d6000803e3d6000fd5b505050505050505050505050565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b15801561018f57600080fd5b505af11580156101a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101c79190610418565b50505050565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b15801561021b57600080fd5b505af115801561022f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102539190610418565b949350505050565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b1580156102b157600080fd5b505af11580156102c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102e99190610418565b95945050505050565b80356001600160a01b038116811461030957600080fd5b919050565b6000806000806080858703121561032457600080fd5b61032d856102f2565b935061033b602086016102f2565b9250610349604086016102f2565b9396929550929360600135925050565b600080600080600080600080610100898b03121561037657600080fd5b61037f896102f2565b975061038d60208a016102f2565b965061039b60408a016102f2565b9550606089013594506080890135935060a089013560ff811681146103bf57600080fd5b979a969950949793969295929450505060c08201359160e0013590565b6000806000606084860312156103f157600080fd5b6103fa846102f2565b9250610408602085016102f2565b9150604084013590509250925092565b60006020828403121561042a57600080fd5b8151801515811461043a57600080fd5b939250505056fea2646970667358221220a5c412b2f35a19a42e744c99b1827100df63715d4215954150ab68d27e36c79964736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ERC20User<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ERC20User<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ERC20User<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC20User))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ERC20User<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC20USER_ABI.clone(), client).into()
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
                ERC20USER_ABI.clone(),
                ERC20USER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `erc20_approve` (0x51cf23b1) function"]
        pub fn erc_20_approve(
            &self,
            token: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 207, 35, 177], (token, spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_permit` (0x05eee8e7) function"]
        pub fn erc_20_permit(
            &self,
            token: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [5, 238, 232, 231],
                    (token, owner, spender, amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_transfer` (0x7fe25581) function"]
        pub fn erc_20_transfer(
            &self,
            token: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 226, 85, 129], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_transferFrom` (0xfc179d67) function"]
        pub fn erc_20_transfer_from(
            &self,
            token: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 23, 157, 103], (token, owner, recipient, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC20User<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `erc20_approve`function with signature `erc20_approve(address,address,uint256)` and selector `[81, 207, 35, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "erc20_approve", abi = "erc20_approve(address,address,uint256)")]
    pub struct Erc20ApproveCall {
        pub token: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `erc20_permit`function with signature `erc20_permit(address,address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[5, 238, 232, 231]`"]
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
        name = "erc20_permit",
        abi = "erc20_permit(address,address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct Erc20PermitCall {
        pub token: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `erc20_transfer`function with signature `erc20_transfer(address,address,uint256)` and selector `[127, 226, 85, 129]`"]
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
        name = "erc20_transfer",
        abi = "erc20_transfer(address,address,uint256)"
    )]
    pub struct Erc20TransferCall {
        pub token: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `erc20_transferFrom`function with signature `erc20_transferFrom(address,address,address,uint256)` and selector `[252, 23, 157, 103]`"]
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
        name = "erc20_transferFrom",
        abi = "erc20_transferFrom(address,address,address,uint256)"
    )]
    pub struct Erc20TransferFromCall {
        pub token: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC20UserCalls {
        Erc20Approve(Erc20ApproveCall),
        Erc20Permit(Erc20PermitCall),
        Erc20Transfer(Erc20TransferCall),
        Erc20TransferFrom(Erc20TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for ERC20UserCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Erc20ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20UserCalls::Erc20Approve(decoded));
            }
            if let Ok(decoded) =
                <Erc20PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20UserCalls::Erc20Permit(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20UserCalls::Erc20Transfer(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC20UserCalls::Erc20TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC20UserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC20UserCalls::Erc20Approve(element) => element.encode(),
                ERC20UserCalls::Erc20Permit(element) => element.encode(),
                ERC20UserCalls::Erc20Transfer(element) => element.encode(),
                ERC20UserCalls::Erc20TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC20UserCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC20UserCalls::Erc20Approve(element) => element.fmt(f),
                ERC20UserCalls::Erc20Permit(element) => element.fmt(f),
                ERC20UserCalls::Erc20Transfer(element) => element.fmt(f),
                ERC20UserCalls::Erc20TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Erc20ApproveCall> for ERC20UserCalls {
        fn from(var: Erc20ApproveCall) -> Self {
            ERC20UserCalls::Erc20Approve(var)
        }
    }
    impl ::std::convert::From<Erc20PermitCall> for ERC20UserCalls {
        fn from(var: Erc20PermitCall) -> Self {
            ERC20UserCalls::Erc20Permit(var)
        }
    }
    impl ::std::convert::From<Erc20TransferCall> for ERC20UserCalls {
        fn from(var: Erc20TransferCall) -> Self {
            ERC20UserCalls::Erc20Transfer(var)
        }
    }
    impl ::std::convert::From<Erc20TransferFromCall> for ERC20UserCalls {
        fn from(var: Erc20TransferFromCall) -> Self {
            ERC20UserCalls::Erc20TransferFrom(var)
        }
    }
}
