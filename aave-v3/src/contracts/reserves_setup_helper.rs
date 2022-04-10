pub use reservessetuphelper_mod::*;
#[allow(clippy::too_many_arguments)]
mod reservessetuphelper_mod {
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
    #[doc = "ReservesSetupHelper was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static RESERVESSETUPHELPER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract PoolConfigurator\",\"name\":\"configurator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct ReservesSetupHelper.ConfigureReserveInput[]\",\"name\":\"inputParams\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseLTV\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationBonus\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowCap\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"supplyCap\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"stableBorrowingEnabled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"borrowingEnabled\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureReserves\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static RESERVESSETUPHELPER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50600080546001600160a01b031916339081178255604051909182917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908290a350610899806100616000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063715018a614610051578063778128311461005b5780638da5cb5b1461006e578063f2fde38b1461008d575b600080fd5b6100596100a0565b005b610059610069366004610720565b61011d565b600054604080516001600160a01b039092168252519081900360200190f35b61005961009b3660046107a9565b61061e565b6000546001600160a01b031633146100d35760405162461bcd60e51b81526004016100ca906107cd565b60405180910390fd5b600080546040516001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908390a3600080546001600160a01b0319169055565b6000546001600160a01b031633146101475760405162461bcd60e51b81526004016100ca906107cd565b60005b8181101561061857836001600160a01b0316637c4e560b84848481811061017357610173610802565b61018a9260206101209092020190810191506107a9565b85858581811061019c5761019c610802565b90506101200201602001358686868181106101b9576101b9610802565b90506101200201604001358787878181106101d6576101d6610802565b6040516001600160e01b031960e089901b1681526001600160a01b039096166004870152602486019490945250604484019190915260606101209092020101356064820152608401600060405180830381600087803b15801561023857600080fd5b505af115801561024c573d6000803e3d6000fd5b5050505082828281811061026257610262610802565b9050610120020161010001602081019061027c9190610818565b1561049557836001600160a01b031663682cf2648484848181106102a2576102a2610802565b6102b99260206101209092020190810191506107a9565b6040516001600160e01b031960e084901b1681526001600160a01b03909116600482015260016024820152604401600060405180830381600087803b15801561030157600080fd5b505af1158015610315573d6000803e3d6000fd5b50505050836001600160a01b031663d14a098384848481811061033a5761033a610802565b6103519260206101209092020190810191506107a9565b85858581811061036357610363610802565b9050610120020160a001356040518363ffffffff1660e01b815260040161039f9291906001600160a01b03929092168252602082015260400190565b600060405180830381600087803b1580156103b957600080fd5b505af11580156103cd573d6000803e3d6000fd5b50505050836001600160a01b0316638a751a608484848181106103f2576103f2610802565b6104099260206101209092020190810191506107a9565b85858581811061041b5761041b610802565b9050610120020160e00160208101906104349190610818565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015215156024820152604401600060405180830381600087803b15801561047c57600080fd5b505af1158015610490573d6000803e3d6000fd5b505050505b836001600160a01b031663571f03e58484848181106104b6576104b6610802565b6104cd9260206101209092020190810191506107a9565b8585858181106104df576104df610802565b9050610120020160c001356040518363ffffffff1660e01b815260040161051b9291906001600160a01b03929092168252602082015260400190565b600060405180830381600087803b15801561053557600080fd5b505af1158015610549573d6000803e3d6000fd5b50505050836001600160a01b0316634b4e675384848481811061056e5761056e610802565b6105859260206101209092020190810191506107a9565b85858581811061059757610597610802565b90506101200201608001356040518363ffffffff1660e01b81526004016105d39291906001600160a01b03929092168252602082015260400190565b600060405180830381600087803b1580156105ed57600080fd5b505af1158015610601573d6000803e3d6000fd5b5050505080806106109061083a565b91505061014a565b50505050565b6000546001600160a01b031633146106485760405162461bcd60e51b81526004016100ca906107cd565b6001600160a01b0381166106ad5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016100ca565b600080546040516001600160a01b03808516939216917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e091a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038116811461071d57600080fd5b50565b60008060006040848603121561073557600080fd5b833561074081610708565b9250602084013567ffffffffffffffff8082111561075d57600080fd5b818601915086601f83011261077157600080fd5b81358181111561078057600080fd5b8760206101208302850101111561079657600080fd5b6020830194508093505050509250925092565b6000602082840312156107bb57600080fd5b81356107c681610708565b9392505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561082a57600080fd5b813580151581146107c657600080fd5b600060001982141561085c57634e487b7160e01b600052601160045260246000fd5b506001019056fea264697066735822122001e4e09253dffca9e7ac718220ee68bad97842b2132c0e60b5b12e42be1370bd64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ReservesSetupHelper<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ReservesSetupHelper<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ReservesSetupHelper<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ReservesSetupHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ReservesSetupHelper<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), RESERVESSETUPHELPER_ABI.clone(), client)
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
                RESERVESSETUPHELPER_ABI.clone(),
                RESERVESSETUPHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `configureReserves` (0x77812831) function"]
        pub fn configure_reserves(
            &self,
            configurator: ethers::core::types::Address,
            input_params: ::std::vec::Vec<ConfigureReserveInput>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 129, 40, 49], (configurator, input_params))
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ReservesSetupHelper<M>
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `configureReserves`function with signature `configureReserves(address,(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool)[])` and selector `[119, 129, 40, 49]`"]
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
        name = "configureReserves",
        abi = "configureReserves(address,(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool)[])"
    )]
    pub struct ConfigureReservesCall {
        pub configurator: ethers::core::types::Address,
        pub input_params: ::std::vec::Vec<ConfigureReserveInput>,
    }
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
    pub enum ReservesSetupHelperCalls {
        ConfigureReserves(ConfigureReservesCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ReservesSetupHelperCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ConfigureReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservesSetupHelperCalls::ConfigureReserves(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservesSetupHelperCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservesSetupHelperCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ReservesSetupHelperCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ReservesSetupHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ReservesSetupHelperCalls::ConfigureReserves(element) => element.encode(),
                ReservesSetupHelperCalls::Owner(element) => element.encode(),
                ReservesSetupHelperCalls::RenounceOwnership(element) => element.encode(),
                ReservesSetupHelperCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ReservesSetupHelperCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ReservesSetupHelperCalls::ConfigureReserves(element) => element.fmt(f),
                ReservesSetupHelperCalls::Owner(element) => element.fmt(f),
                ReservesSetupHelperCalls::RenounceOwnership(element) => element.fmt(f),
                ReservesSetupHelperCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ConfigureReservesCall> for ReservesSetupHelperCalls {
        fn from(var: ConfigureReservesCall) -> Self {
            ReservesSetupHelperCalls::ConfigureReserves(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ReservesSetupHelperCalls {
        fn from(var: OwnerCall) -> Self {
            ReservesSetupHelperCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ReservesSetupHelperCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ReservesSetupHelperCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ReservesSetupHelperCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ReservesSetupHelperCalls::TransferOwnership(var)
        }
    }
    #[doc = "`ConfigureReserveInput(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConfigureReserveInput {
        pub asset: ethers::core::types::Address,
        pub base_ltv: ethers::core::types::U256,
        pub liquidation_threshold: ethers::core::types::U256,
        pub liquidation_bonus: ethers::core::types::U256,
        pub reserve_factor: ethers::core::types::U256,
        pub borrow_cap: ethers::core::types::U256,
        pub supply_cap: ethers::core::types::U256,
        pub stable_borrowing_enabled: bool,
        pub borrowing_enabled: bool,
    }
}
