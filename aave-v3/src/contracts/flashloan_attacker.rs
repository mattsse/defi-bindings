pub use flashloan_attacker::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod flashloan_attacker {
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
    #[doc = "FlashloanAttacker was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FLASHLOANATTACKER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"supplyAsset\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static FLASHLOANATTACKER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c060405234801561001057600080fd5b50604051610a65380380610a6583398101604081905261002f91610166565b80806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663026b1d5f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ac9190610166565b6001600160a01b031660a0816001600160a01b03168152505050806001600160a01b031663026b1d5f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610104573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101289190610166565b600180546001600160a01b0319166001600160a01b03929092169190911790555061018a565b6001600160a01b038116811461016357600080fd5b50565b60006020828403121561017857600080fd5b81516101838161014e565b9392505050565b60805160a0516108b16101b46000396000818160d201526102ef01526000605601526108b16000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630542975c146100515780631416d762146100955780631b11d0ff146100aa5780637535d246146100cd575b600080fd5b6100787f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100a86100a3366004610512565b6100f4565b005b6100bd6100b83660046105af565b610253565b604051901515815260200161008c565b6100787f000000000000000000000000000000000000000000000000000000000000000081565b60405163140e25ad60e31b81526004810182905282906001600160a01b0382169063a0712d68906024016020604051808303816000875af115801561013d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610161919061067d565b5060015460405163095ea7b360e01b81526001600160a01b03918216600482015260001960248201529082169063095ea7b3906044016020604051808303816000875af11580156101b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101da919061067d565b5060015460405163617ba03760e01b81526001600160a01b03858116600483015260248201859052306044830152600060648301529091169063617ba03790608401600060405180830381600087803b15801561023657600080fd5b505af115801561024a573d6000803e3d6000fd5b50505050505050565b60008581610261878761037a565b905061026c88610390565b60405163140e25ad60e31b8152600481018790526001600160a01b0383169063a0712d68906024016020604051808303816000875af11580156102b3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102d7919061067d565b5060405163095ea7b360e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811660048301526024820183905289169063095ea7b3906044016020604051808303816000875af1158015610347573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061036b919061067d565b50600198975050505050505050565b8082018281101561038a57600080fd5b92915050565b6001546040516335ea6a7560e01b81526001600160a01b03838116600483015260009216906335ea6a75906024016101e060405180830381865afa1580156103dc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610400919061073f565b6101008101516040516370a0823160e01b81526001600160a01b03918216600482015291925083916000918316906370a0823190602401602060405180830381865afa158015610454573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104789190610862565b60015460405163a415bcad60e01b81526001600160a01b03878116600483015260248201849052600260448301526000606483015230608483015292935091169063a415bcad9060a401600060405180830381600087803b1580156104dc57600080fd5b505af11580156104f0573d6000803e3d6000fd5b5050505050505050565b6001600160a01b038116811461050f57600080fd5b50565b6000806040838503121561052557600080fd5b8235610530816104fa565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b6040516101e0810167ffffffffffffffff811182821017156105785761057861053e565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156105a7576105a761053e565b604052919050565b600080600080600060a086880312156105c757600080fd5b85356105d2816104fa565b945060208681013594506040870135935060608701356105f1816104fa565b9250608087013567ffffffffffffffff8082111561060e57600080fd5b818901915089601f83011261062257600080fd5b8135818111156106345761063461053e565b610646601f8201601f1916850161057e565b91508082528a8482850101111561065c57600080fd5b80848401858401376000848284010152508093505050509295509295909350565b60006020828403121561068f57600080fd5b8151801515811461069f57600080fd5b9392505050565b6000602082840312156106b857600080fd5b6040516020810181811067ffffffffffffffff821117156106db576106db61053e565b6040529151825250919050565b80516fffffffffffffffffffffffffffffffff8116811461070857600080fd5b919050565b805164ffffffffff8116811461070857600080fd5b805161ffff8116811461070857600080fd5b8051610708816104fa565b60006101e0828403121561075257600080fd5b61075a610554565b61076484846106a6565b8152610772602084016106e8565b6020820152610783604084016106e8565b6040820152610794606084016106e8565b60608201526107a5608084016106e8565b60808201526107b660a084016106e8565b60a08201526107c760c0840161070d565b60c08201526107d860e08401610722565b60e08201526101006107eb818501610734565b908201526101206107fd848201610734565b9082015261014061080f848201610734565b90820152610160610821848201610734565b908201526101806108338482016106e8565b908201526101a06108458482016106e8565b908201526101c06108578482016106e8565b908201529392505050565b60006020828403121561087457600080fd5b505191905056fea26469706673582212200d02afd8cc2f11be9eab6430989c60bf69ec31114831cf253402be650c73d3ef64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct FlashloanAttacker<M>(ethers::contract::Contract<M>);
    impl<M> Clone for FlashloanAttacker<M> {
        fn clone(&self) -> Self {
            FlashloanAttacker(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FlashloanAttacker<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FlashloanAttacker<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FlashloanAttacker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FlashloanAttacker<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), FLASHLOANATTACKER_ABI.clone(), client)
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
                FLASHLOANATTACKER_ABI.clone(),
                FLASHLOANATTACKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOperation` (0x1b11d0ff) function"]
        pub fn execute_operation(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            premium: ethers::core::types::U256,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 17, 208, 255], (asset, amount, premium, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supplyAsset` (0x1416d762) function"]
        pub fn supply_asset(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 22, 215, 98], (asset, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for FlashloanAttacker<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    #[doc = "Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    #[doc = "Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `[27, 17, 208, 255]`"]
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
        name = "executeOperation",
        abi = "executeOperation(address,uint256,uint256,address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
        pub p3: ethers::core::types::Address,
        pub p4: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `supplyAsset` function with signature `supplyAsset(address,uint256)` and selector `[20, 22, 215, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supplyAsset", abi = "supplyAsset(address,uint256)")]
    pub struct SupplyAssetCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FlashloanAttackerCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        SupplyAsset(SupplyAssetCall),
    }
    impl ethers::core::abi::AbiDecode for FlashloanAttackerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashloanAttackerCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FlashloanAttackerCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashloanAttackerCalls::ExecuteOperation(decoded));
            }
            if let Ok(decoded) =
                <SupplyAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashloanAttackerCalls::SupplyAsset(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FlashloanAttackerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FlashloanAttackerCalls::AddressesProvider(element) => element.encode(),
                FlashloanAttackerCalls::Pool(element) => element.encode(),
                FlashloanAttackerCalls::ExecuteOperation(element) => element.encode(),
                FlashloanAttackerCalls::SupplyAsset(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FlashloanAttackerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FlashloanAttackerCalls::AddressesProvider(element) => element.fmt(f),
                FlashloanAttackerCalls::Pool(element) => element.fmt(f),
                FlashloanAttackerCalls::ExecuteOperation(element) => element.fmt(f),
                FlashloanAttackerCalls::SupplyAsset(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for FlashloanAttackerCalls {
        fn from(var: AddressesProviderCall) -> Self {
            FlashloanAttackerCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<PoolCall> for FlashloanAttackerCalls {
        fn from(var: PoolCall) -> Self {
            FlashloanAttackerCalls::Pool(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for FlashloanAttackerCalls {
        fn from(var: ExecuteOperationCall) -> Self {
            FlashloanAttackerCalls::ExecuteOperation(var)
        }
    }
    impl ::std::convert::From<SupplyAssetCall> for FlashloanAttackerCalls {
        fn from(var: SupplyAssetCall) -> Self {
            FlashloanAttackerCalls::SupplyAsset(var)
        }
    }
    #[doc = "Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressesProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `[27, 17, 208, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExecuteOperationReturn(pub bool);
}
