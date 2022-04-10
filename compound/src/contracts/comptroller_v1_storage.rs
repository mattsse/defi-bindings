pub use comptrollerv1storage_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerv1storage_mod {
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
    #[doc = "ComptrollerV1Storage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERV1STORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERV1STORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506101f9806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063bb82aa5e11610066578063bb82aa5e146100e6578063dce15449146100ee578063dcfbc0c71461011a578063e875544614610122578063f851a4401461012a57610093565b806326782247146100985780634ada90af146100bc5780637dc0d1d0146100d657806394b2294b146100de575b600080fd5b6100a0610132565b604080516001600160a01b039092168252519081900360200190f35b6100c4610141565b60408051918252519081900360200190f35b6100a0610147565b6100c4610156565b6100a061015c565b6100a06004803603604081101561010457600080fd5b506001600160a01b03813516906020013561016b565b6100a06101a0565b6100c46101af565b6100a06101b5565b6001546001600160a01b031681565b60065481565b6004546001600160a01b031681565b60075481565b6002546001600160a01b031681565b6008602052816000526040600020818154811061018457fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b60055481565b6000546001600160a01b03168156fea265627a7a72315820c2627444f5e667f7a707ec0e56de51d2b941e6d2beb54e447ba8d47507fdc0b864736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ComptrollerV1Storage<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerV1Storage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerV1Storage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerV1Storage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerV1Storage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                COMPTROLLERV1STORAGE_ABI.clone(),
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
                COMPTROLLERV1STORAGE_ABI.clone(),
                COMPTROLLERV1STORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `accountAssets` (0xdce15449) function"]
        pub fn account_assets(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 225, 84, 73], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeFactorMantissa` (0xe8755446) function"]
        pub fn close_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([232, 117, 84, 70], ())
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
        #[doc = "Calls the contract's `liquidationIncentiveMantissa` (0x4ada90af) function"]
        pub fn liquidation_incentive_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([74, 218, 144, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxAssets` (0x94b2294b) function"]
        pub fn max_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 178, 41, 75], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracle` (0x7dc0d1d0) function"]
        pub fn oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([125, 192, 209, 208], ())
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
        for ComptrollerV1Storage<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `accountAssets`function with signature `accountAssets(address,uint256)` and selector `[220, 225, 84, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accountAssets", abi = "accountAssets(address,uint256)")]
    pub struct AccountAssetsCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
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
    #[doc = "Container type for all input parameters for the `closeFactorMantissa`function with signature `closeFactorMantissa()` and selector `[232, 117, 84, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "closeFactorMantissa", abi = "closeFactorMantissa()")]
    pub struct CloseFactorMantissaCall;
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
    #[doc = "Container type for all input parameters for the `liquidationIncentiveMantissa`function with signature `liquidationIncentiveMantissa()` and selector `[74, 218, 144, 175]`"]
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
        name = "liquidationIncentiveMantissa",
        abi = "liquidationIncentiveMantissa()"
    )]
    pub struct LiquidationIncentiveMantissaCall;
    #[doc = "Container type for all input parameters for the `maxAssets`function with signature `maxAssets()` and selector `[148, 178, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxAssets", abi = "maxAssets()")]
    pub struct MaxAssetsCall;
    #[doc = "Container type for all input parameters for the `oracle`function with signature `oracle()` and selector `[125, 192, 209, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
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
    pub enum ComptrollerV1StorageCalls {
        AccountAssets(AccountAssetsCall),
        Admin(AdminCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        LiquidationIncentiveMantissa(LiquidationIncentiveMantissaCall),
        MaxAssets(MaxAssetsCall),
        Oracle(OracleCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerV1StorageCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV1StorageCalls::ComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV1StorageCalls::LiquidationIncentiveMantissa(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::MaxAssets(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV1StorageCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV1StorageCalls::PendingComptrollerImplementation(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerV1StorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerV1StorageCalls::AccountAssets(element) => element.encode(),
                ComptrollerV1StorageCalls::Admin(element) => element.encode(),
                ComptrollerV1StorageCalls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerV1StorageCalls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerV1StorageCalls::LiquidationIncentiveMantissa(element) => {
                    element.encode()
                }
                ComptrollerV1StorageCalls::MaxAssets(element) => element.encode(),
                ComptrollerV1StorageCalls::Oracle(element) => element.encode(),
                ComptrollerV1StorageCalls::PendingAdmin(element) => element.encode(),
                ComptrollerV1StorageCalls::PendingComptrollerImplementation(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerV1StorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerV1StorageCalls::AccountAssets(element) => element.fmt(f),
                ComptrollerV1StorageCalls::Admin(element) => element.fmt(f),
                ComptrollerV1StorageCalls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerV1StorageCalls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerV1StorageCalls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerV1StorageCalls::MaxAssets(element) => element.fmt(f),
                ComptrollerV1StorageCalls::Oracle(element) => element.fmt(f),
                ComptrollerV1StorageCalls::PendingAdmin(element) => element.fmt(f),
                ComptrollerV1StorageCalls::PendingComptrollerImplementation(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerV1StorageCalls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerV1StorageCalls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerV1StorageCalls {
        fn from(var: AdminCall) -> Self {
            ComptrollerV1StorageCalls::Admin(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerV1StorageCalls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerV1StorageCalls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerV1StorageCalls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerV1StorageCalls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerV1StorageCalls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerV1StorageCalls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerV1StorageCalls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerV1StorageCalls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerV1StorageCalls {
        fn from(var: OracleCall) -> Self {
            ComptrollerV1StorageCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerV1StorageCalls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerV1StorageCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerV1StorageCalls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerV1StorageCalls::PendingComptrollerImplementation(var)
        }
    }
}
