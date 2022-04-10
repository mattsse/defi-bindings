pub use unitrolleradminstorage_mod::*;
#[allow(clippy::too_many_arguments)]
mod unitrolleradminstorage_mod {
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
    #[doc = "UnitrollerAdminStorage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNITROLLERADMINSTORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNITROLLERADMINSTORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060f08061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060465760003560e01c80632678224714604b578063bb82aa5e14606d578063dcfbc0c7146073578063f851a440146079575b600080fd5b6051607f565b604080516001600160a01b039092168252519081900360200190f35b6051608e565b6051609d565b605160ac565b6001546001600160a01b031681565b6002546001600160a01b031681565b6003546001600160a01b031681565b6000546001600160a01b03168156fea265627a7a7231582036eebadc5d3f166dfe7975d2a515d4761e95aa0dc6365c1976dc695da1373cf264736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct UnitrollerAdminStorage<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UnitrollerAdminStorage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UnitrollerAdminStorage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UnitrollerAdminStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UnitrollerAdminStorage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                UNITROLLERADMINSTORAGE_ABI.clone(),
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
                UNITROLLERADMINSTORAGE_ABI.clone(),
                UNITROLLERADMINSTORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `comptrollerImplementation` (0xbb82aa5e) function"]
        pub fn comptroller_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([187, 130, 170, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingAdmin` (0x26782247) function"]
        pub fn pending_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingComptrollerImplementation` (0xdcfbc0c7) function"]
        pub fn pending_comptroller_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 251, 192, 199], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for UnitrollerAdminStorage<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `comptrollerImplementation`function with signature `comptrollerImplementation()` and selector `[187, 130, 170, 94]`"]
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
        name = "comptrollerImplementation",
        abi = "comptrollerImplementation()"
    )]
    pub struct ComptrollerImplementationCall;
    #[doc = "Container type for all input parameters for the `pendingAdmin`function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    #[doc = "Container type for all input parameters for the `pendingComptrollerImplementation`function with signature `pendingComptrollerImplementation()` and selector `[220, 251, 192, 199]`"]
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
        name = "pendingComptrollerImplementation",
        abi = "pendingComptrollerImplementation()"
    )]
    pub struct PendingComptrollerImplementationCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UnitrollerAdminStorageCalls {
        Admin(AdminCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
    }
    impl ethers::core::abi::AbiDecode for UnitrollerAdminStorageCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnitrollerAdminStorageCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnitrollerAdminStorageCalls::ComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnitrollerAdminStorageCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnitrollerAdminStorageCalls::PendingComptrollerImplementation(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UnitrollerAdminStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UnitrollerAdminStorageCalls::Admin(element) => element.encode(),
                UnitrollerAdminStorageCalls::ComptrollerImplementation(element) => element.encode(),
                UnitrollerAdminStorageCalls::PendingAdmin(element) => element.encode(),
                UnitrollerAdminStorageCalls::PendingComptrollerImplementation(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for UnitrollerAdminStorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UnitrollerAdminStorageCalls::Admin(element) => element.fmt(f),
                UnitrollerAdminStorageCalls::ComptrollerImplementation(element) => element.fmt(f),
                UnitrollerAdminStorageCalls::PendingAdmin(element) => element.fmt(f),
                UnitrollerAdminStorageCalls::PendingComptrollerImplementation(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AdminCall> for UnitrollerAdminStorageCalls {
        fn from(var: AdminCall) -> Self {
            UnitrollerAdminStorageCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for UnitrollerAdminStorageCalls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            UnitrollerAdminStorageCalls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for UnitrollerAdminStorageCalls {
        fn from(var: PendingAdminCall) -> Self {
            UnitrollerAdminStorageCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for UnitrollerAdminStorageCalls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            UnitrollerAdminStorageCalls::PendingComptrollerImplementation(var)
        }
    }
}
