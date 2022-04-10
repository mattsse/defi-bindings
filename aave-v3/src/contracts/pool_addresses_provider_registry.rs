pub use pooladdressesproviderregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod pooladdressesproviderregistry_mod {
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
    #[doc = "PoolAddressesProviderRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static POOLADDRESSESPROVIDERREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressesProviderRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressesProviderUnregistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProviderAddressById\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressesProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProviderIdByAddress\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddressesProvidersList\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerAddressesProvider\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unregisterAddressesProvider\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static POOLADDRESSESPROVIDERREGISTRY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50604051610a99380380610a9983398101604081905261002f9161017a565b600080546001600160a01b03191633908117825560405190918291600080516020610a79833981519152908290a3506100678161006d565b506101aa565b6000546001600160a01b031633146100cc5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064015b60405180910390fd5b6001600160a01b0381166101315760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016100c3565b600080546040516001600160a01b0380851693921691600080516020610a7983398151915291a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b60006020828403121561018c57600080fd5b81516001600160a01b03811681146101a357600080fd5b9392505050565b6108c0806101b96000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80638da5cb5b1161005b5780638da5cb5b14610109578063d0267be71461011a578063d258191e14610151578063f2fde38b1461016457600080fd5b80630de267071461008d578063365ccbbf146100a257806357dc0566146100c0578063715018a614610101575b600080fd5b6100a061009b3660046106fd565b610177565b005b6100aa610279565b6040516100b7919061071f565b60405180910390f35b6100e96100ce36600461076c565b6000908152600260205260409020546001600160a01b031690565b6040516001600160a01b0390911681526020016100b7565b6100a06102db565b6000546001600160a01b03166100e9565b6101436101283660046106fd565b6001600160a01b031660009081526001602052604090205490565b6040519081526020016100b7565b6100a061015f366004610785565b61034f565b6100a06101723660046106fd565b610506565b6000546001600160a01b031633146101aa5760405162461bcd60e51b81526004016101a1906107af565b60405180910390fd5b6001600160a01b038116600090815260016020818152604092839020548351808501909452918352603760f81b908301526101f85760405162461bcd60e51b81526004016101a191906107e4565b506001600160a01b038116600081815260016020818152604080842080548086526002845291852080546001600160a01b0319169055948452919052915561023f826105f0565b60405181906001600160a01b038416907f254723080701bde71d562cad0e967cef23d86bb27ee842c190a2596820f3b24190600090a35050565b606060038054806020026020016040519081016040528092919081815260200182805480156102d157602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116102b3575b5050505050905090565b6000546001600160a01b031633146103055760405162461bcd60e51b81526004016101a1906107af565b600080546040516001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908390a3600080546001600160a01b0319169055565b6000546001600160a01b031633146103795760405162461bcd60e51b81526004016101a1906107af565b6040805180820190915260018152600760fb1b6020820152816103af5760405162461bcd60e51b81526004016101a191906107e4565b5060008181526002602090815260409182902054825180840190935260018352600760fb1b918301919091526001600160a01b0316156104025760405162461bcd60e51b81526004016101a191906107e4565b506001600160a01b03821660009081526001602090815260409182902054825180840190935260028352611c1b60f11b91830191909152156104575760405162461bcd60e51b81526004016101a191906107e4565b506001600160a01b03821660008181526001602081815260408084208690558584526002825280842080546001600160a01b0319908116871790915560038054878752600490945282862084905593830184559284527fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b90910180549092168417909155518392917fc2e7cc813550ef0e7126cc0571281850ce5df2e9c400acf3589c38e4627f85f191a35050565b6000546001600160a01b031633146105305760405162461bcd60e51b81526004016101a1906107af565b6001600160a01b0381166105955760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016101a1565b600080546040516001600160a01b03808516939216917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e091a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038116600090815260046020526040812080549082905560035490919061062090600190610839565b9050808210156106a95760006003828154811061063f5761063f61085e565b600091825260209091200154600380546001600160a01b03909216925082918590811061066e5761066e61085e565b600091825260208083209190910180546001600160a01b0319166001600160a01b039485161790559290911681526004909152604090208290555b60038054806106ba576106ba610874565b600082815260209020810160001990810180546001600160a01b0319169055019055505050565b80356001600160a01b03811681146106f857600080fd5b919050565b60006020828403121561070f57600080fd5b610718826106e1565b9392505050565b6020808252825182820181905260009190848201906040850190845b818110156107605783516001600160a01b03168352928401929184019160010161073b565b50909695505050505050565b60006020828403121561077e57600080fd5b5035919050565b6000806040838503121561079857600080fd5b6107a1836106e1565b946020939093013593505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b600060208083528351808285015260005b81811015610811578581018301518582016040015282016107f5565b81811115610823576000604083870101525b50601f01601f1916929092016040019392505050565b60008282101561085957634e487b7160e01b600052601160045260246000fd5b500390565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052603160045260246000fdfea26469706673582212201e973cbb66afc97a685954aa7da59a1b500a8894296fcc4fe7c72cc445eb386c64736f6c634300080a00338be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct PoolAddressesProviderRegistry<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PoolAddressesProviderRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PoolAddressesProviderRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PoolAddressesProviderRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PoolAddressesProviderRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
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
                POOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
                POOLADDRESSESPROVIDERREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getAddressesProviderAddressById` (0x57dc0566) function"]
        pub fn get_addresses_provider_address_by_id(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([87, 220, 5, 102], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProviderIdByAddress` (0xd0267be7) function"]
        pub fn get_addresses_provider_id_by_address(
            &self,
            addresses_provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([208, 38, 123, 231], addresses_provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProvidersList` (0x365ccbbf) function"]
        pub fn get_addresses_providers_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([54, 92, 203, 191], ())
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
        #[doc = "Calls the contract's `registerAddressesProvider` (0xd258191e) function"]
        pub fn register_addresses_provider(
            &self,
            provider: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 88, 25, 30], (provider, id))
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
        #[doc = "Calls the contract's `unregisterAddressesProvider` (0x0de26707) function"]
        pub fn unregister_addresses_provider(
            &self,
            provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 226, 103, 7], provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddressesProviderRegistered` event"]
        pub fn addresses_provider_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressesProviderRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressesProviderUnregistered` event"]
        pub fn addresses_provider_unregistered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressesProviderUnregisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolAddressesProviderRegistryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PoolAddressesProviderRegistry<M>
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
        name = "AddressesProviderRegistered",
        abi = "AddressesProviderRegistered(address,uint256)"
    )]
    pub struct AddressesProviderRegisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
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
        name = "AddressesProviderUnregistered",
        abi = "AddressesProviderUnregistered(address,uint256)"
    )]
    pub struct AddressesProviderUnregisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolAddressesProviderRegistryEvents {
        AddressesProviderRegisteredFilter(AddressesProviderRegisteredFilter),
        AddressesProviderUnregisteredFilter(AddressesProviderUnregisteredFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for PoolAddressesProviderRegistryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressesProviderRegisteredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = AddressesProviderUnregisteredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::OwnershipTransferredFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PoolAddressesProviderRegistryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                    element,
                ) => element.fmt(f),
                PoolAddressesProviderRegistryEvents::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getAddressesProviderAddressById`function with signature `getAddressesProviderAddressById(uint256)` and selector `[87, 220, 5, 102]`"]
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
        name = "getAddressesProviderAddressById",
        abi = "getAddressesProviderAddressById(uint256)"
    )]
    pub struct GetAddressesProviderAddressByIdCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAddressesProviderIdByAddress`function with signature `getAddressesProviderIdByAddress(address)` and selector `[208, 38, 123, 231]`"]
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
        name = "getAddressesProviderIdByAddress",
        abi = "getAddressesProviderIdByAddress(address)"
    )]
    pub struct GetAddressesProviderIdByAddressCall {
        pub addresses_provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAddressesProvidersList`function with signature `getAddressesProvidersList()` and selector `[54, 92, 203, 191]`"]
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
        name = "getAddressesProvidersList",
        abi = "getAddressesProvidersList()"
    )]
    pub struct GetAddressesProvidersListCall;
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
    #[doc = "Container type for all input parameters for the `registerAddressesProvider`function with signature `registerAddressesProvider(address,uint256)` and selector `[210, 88, 25, 30]`"]
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
        name = "registerAddressesProvider",
        abi = "registerAddressesProvider(address,uint256)"
    )]
    pub struct RegisterAddressesProviderCall {
        pub provider: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `unregisterAddressesProvider`function with signature `unregisterAddressesProvider(address)` and selector `[13, 226, 103, 7]`"]
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
        name = "unregisterAddressesProvider",
        abi = "unregisterAddressesProvider(address)"
    )]
    pub struct UnregisterAddressesProviderCall {
        pub provider: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolAddressesProviderRegistryCalls {
        GetAddressesProviderAddressById(GetAddressesProviderAddressByIdCall),
        GetAddressesProviderIdByAddress(GetAddressesProviderIdByAddressCall),
        GetAddressesProvidersList(GetAddressesProvidersListCall),
        Owner(OwnerCall),
        RegisterAddressesProvider(RegisterAddressesProviderCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UnregisterAddressesProvider(UnregisterAddressesProviderCall),
    }
    impl ethers::core::abi::AbiDecode for PoolAddressesProviderRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAddressesProviderAddressByIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    PoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(decoded),
                );
            }
            if let Ok(decoded) =
                <GetAddressesProviderIdByAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    PoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(decoded),
                );
            }
            if let Ok(decoded) =
                <GetAddressesProvidersListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolAddressesProviderRegistryCalls::GetAddressesProvidersList(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderRegistryCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RegisterAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolAddressesProviderRegistryCalls::RegisterAddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderRegistryCalls::RenounceOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderRegistryCalls::TransferOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnregisterAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    PoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PoolAddressesProviderRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(element) => {
                    element.encode()
                }
                PoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(element) => {
                    element.encode()
                }
                PoolAddressesProviderRegistryCalls::GetAddressesProvidersList(element) => {
                    element.encode()
                }
                PoolAddressesProviderRegistryCalls::Owner(element) => element.encode(),
                PoolAddressesProviderRegistryCalls::RegisterAddressesProvider(element) => {
                    element.encode()
                }
                PoolAddressesProviderRegistryCalls::RenounceOwnership(element) => element.encode(),
                PoolAddressesProviderRegistryCalls::TransferOwnership(element) => element.encode(),
                PoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for PoolAddressesProviderRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderRegistryCalls::GetAddressesProvidersList(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderRegistryCalls::Owner(element) => element.fmt(f),
                PoolAddressesProviderRegistryCalls::RegisterAddressesProvider(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderRegistryCalls::RenounceOwnership(element) => element.fmt(f),
                PoolAddressesProviderRegistryCalls::TransferOwnership(element) => element.fmt(f),
                PoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<GetAddressesProviderAddressByIdCall>
        for PoolAddressesProviderRegistryCalls
    {
        fn from(var: GetAddressesProviderAddressByIdCall) -> Self {
            PoolAddressesProviderRegistryCalls::GetAddressesProviderAddressById(var)
        }
    }
    impl ::std::convert::From<GetAddressesProviderIdByAddressCall>
        for PoolAddressesProviderRegistryCalls
    {
        fn from(var: GetAddressesProviderIdByAddressCall) -> Self {
            PoolAddressesProviderRegistryCalls::GetAddressesProviderIdByAddress(var)
        }
    }
    impl ::std::convert::From<GetAddressesProvidersListCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: GetAddressesProvidersListCall) -> Self {
            PoolAddressesProviderRegistryCalls::GetAddressesProvidersList(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: OwnerCall) -> Self {
            PoolAddressesProviderRegistryCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RegisterAddressesProviderCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: RegisterAddressesProviderCall) -> Self {
            PoolAddressesProviderRegistryCalls::RegisterAddressesProvider(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PoolAddressesProviderRegistryCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PoolAddressesProviderRegistryCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnregisterAddressesProviderCall> for PoolAddressesProviderRegistryCalls {
        fn from(var: UnregisterAddressesProviderCall) -> Self {
            PoolAddressesProviderRegistryCalls::UnregisterAddressesProvider(var)
        }
    }
}
