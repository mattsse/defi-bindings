pub use ctokenstorage_mod::*;
#[allow(clippy::too_many_arguments)]
mod ctokenstorage_mod {
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
    #[doc = "CTokenStorage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CTOKENSTORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accrualBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptroller\",\"outputs\":[{\"internalType\":\"contract ComptrollerInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"interestRateModel\",\"outputs\":[{\"internalType\":\"contract InterestRateModel\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrows\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalReserves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CTOKENSTORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061037d806100206000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80635fe3b5671161008c57806395d89b411161006657806395d89b41146101d5578063aa5af0fd146101dd578063f3fdb15a146101e5578063f851a440146101ed576100cf565b80635fe3b567146101bd5780636c540baf146101c55780638f840ddd146101cd576100cf565b806306fdde03146100d4578063173b99041461015157806318160ddd1461016b5780632678224714610173578063313ce5671461019757806347bd3718146101b5575b600080fd5b6100dc6101f5565b6040805160208082528351818301528351919283929083019185019080838360005b838110156101165781810151838201526020016100fe565b50505050905090810190601f1680156101435780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b610159610282565b60408051918252519081900360200190f35b610159610288565b61017b61028e565b604080516001600160a01b039092168252519081900360200190f35b61019f61029d565b6040805160ff9092168252519081900360200190f35b6101596102a6565b61017b6102ac565b6101596102bb565b6101596102c1565b6100dc6102c7565b61015961031f565b61017b610325565b61017b610334565b60018054604080516020600284861615610100026000190190941693909304601f8101849004840282018401909252818152929183018282801561027a5780601f1061024f5761010080835404028352916020019161027a565b820191906000526020600020905b81548152906001019060200180831161025d57829003601f168201915b505050505081565b60085481565b600d5481565b6004546001600160a01b031681565b60035460ff1681565b600b5481565b6005546001600160a01b031681565b60095481565b600c5481565b6002805460408051602060018416156101000260001901909316849004601f8101849004840282018401909252818152929183018282801561027a5780601f1061024f5761010080835404028352916020019161027a565b600a5481565b6006546001600160a01b031681565b60035461010090046001600160a01b03168156fea265627a7a7231582076680555f1c211e166e527613f79c28e675633e1e7e1c250848c60d80252fd2064736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CTokenStorage<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CTokenStorage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CTokenStorage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CTokenStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CTokenStorage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CTOKENSTORAGE_ABI.clone(), client)
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
                CTOKENSTORAGE_ABI.clone(),
                CTOKENSTORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `accrualBlockNumber` (0x6c540baf) function"]
        pub fn accrual_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 84, 11, 175], ())
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
        #[doc = "Calls the contract's `borrowIndex` (0xaa5af0fd) function"]
        pub fn borrow_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 90, 240, 253], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `comptroller` (0x5fe3b567) function"]
        pub fn comptroller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([95, 227, 181, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `interestRateModel` (0xf3fdb15a) function"]
        pub fn interest_rate_model(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([243, 253, 177, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `reserveFactorMantissa` (0x173b9904) function"]
        pub fn reserve_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 59, 153, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrows` (0x47bd3718) function"]
        pub fn total_borrows(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([71, 189, 55, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalReserves` (0x8f840ddd) function"]
        pub fn total_reserves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([143, 132, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CTokenStorage<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `accrualBlockNumber`function with signature `accrualBlockNumber()` and selector `[108, 84, 11, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accrualBlockNumber", abi = "accrualBlockNumber()")]
    pub struct AccrualBlockNumberCall;
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
    #[doc = "Container type for all input parameters for the `borrowIndex`function with signature `borrowIndex()` and selector `[170, 90, 240, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowIndex", abi = "borrowIndex()")]
    pub struct BorrowIndexCall;
    #[doc = "Container type for all input parameters for the `comptroller`function with signature `comptroller()` and selector `[95, 227, 181, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "comptroller", abi = "comptroller()")]
    pub struct ComptrollerCall;
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `interestRateModel`function with signature `interestRateModel()` and selector `[243, 253, 177, 90]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "interestRateModel", abi = "interestRateModel()")]
    pub struct InterestRateModelCall;
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    #[doc = "Container type for all input parameters for the `reserveFactorMantissa`function with signature `reserveFactorMantissa()` and selector `[23, 59, 153, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "reserveFactorMantissa", abi = "reserveFactorMantissa()")]
    pub struct ReserveFactorMantissaCall;
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalBorrows`function with signature `totalBorrows()` and selector `[71, 189, 55, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalBorrows", abi = "totalBorrows()")]
    pub struct TotalBorrowsCall;
    #[doc = "Container type for all input parameters for the `totalReserves`function with signature `totalReserves()` and selector `[143, 132, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalReserves", abi = "totalReserves()")]
    pub struct TotalReservesCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CTokenStorageCalls {
        AccrualBlockNumber(AccrualBlockNumberCall),
        Admin(AdminCall),
        BorrowIndex(BorrowIndexCall),
        Comptroller(ComptrollerCall),
        Decimals(DecimalsCall),
        InterestRateModel(InterestRateModelCall),
        Name(NameCall),
        PendingAdmin(PendingAdminCall),
        ReserveFactorMantissa(ReserveFactorMantissaCall),
        Symbol(SymbolCall),
        TotalBorrows(TotalBorrowsCall),
        TotalReserves(TotalReservesCall),
        TotalSupply(TotalSupplyCall),
    }
    impl ethers::core::abi::AbiDecode for CTokenStorageCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccrualBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::AccrualBlockNumber(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BorrowIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::BorrowIndex(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::Comptroller(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <InterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::InterestRateModel(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CTokenStorageCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <ReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::ReserveFactorMantissa(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::TotalBorrows(decoded));
            }
            if let Ok(decoded) =
                <TotalReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::TotalReserves(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CTokenStorageCalls::TotalSupply(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CTokenStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CTokenStorageCalls::AccrualBlockNumber(element) => element.encode(),
                CTokenStorageCalls::Admin(element) => element.encode(),
                CTokenStorageCalls::BorrowIndex(element) => element.encode(),
                CTokenStorageCalls::Comptroller(element) => element.encode(),
                CTokenStorageCalls::Decimals(element) => element.encode(),
                CTokenStorageCalls::InterestRateModel(element) => element.encode(),
                CTokenStorageCalls::Name(element) => element.encode(),
                CTokenStorageCalls::PendingAdmin(element) => element.encode(),
                CTokenStorageCalls::ReserveFactorMantissa(element) => element.encode(),
                CTokenStorageCalls::Symbol(element) => element.encode(),
                CTokenStorageCalls::TotalBorrows(element) => element.encode(),
                CTokenStorageCalls::TotalReserves(element) => element.encode(),
                CTokenStorageCalls::TotalSupply(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CTokenStorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CTokenStorageCalls::AccrualBlockNumber(element) => element.fmt(f),
                CTokenStorageCalls::Admin(element) => element.fmt(f),
                CTokenStorageCalls::BorrowIndex(element) => element.fmt(f),
                CTokenStorageCalls::Comptroller(element) => element.fmt(f),
                CTokenStorageCalls::Decimals(element) => element.fmt(f),
                CTokenStorageCalls::InterestRateModel(element) => element.fmt(f),
                CTokenStorageCalls::Name(element) => element.fmt(f),
                CTokenStorageCalls::PendingAdmin(element) => element.fmt(f),
                CTokenStorageCalls::ReserveFactorMantissa(element) => element.fmt(f),
                CTokenStorageCalls::Symbol(element) => element.fmt(f),
                CTokenStorageCalls::TotalBorrows(element) => element.fmt(f),
                CTokenStorageCalls::TotalReserves(element) => element.fmt(f),
                CTokenStorageCalls::TotalSupply(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccrualBlockNumberCall> for CTokenStorageCalls {
        fn from(var: AccrualBlockNumberCall) -> Self {
            CTokenStorageCalls::AccrualBlockNumber(var)
        }
    }
    impl ::std::convert::From<AdminCall> for CTokenStorageCalls {
        fn from(var: AdminCall) -> Self {
            CTokenStorageCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BorrowIndexCall> for CTokenStorageCalls {
        fn from(var: BorrowIndexCall) -> Self {
            CTokenStorageCalls::BorrowIndex(var)
        }
    }
    impl ::std::convert::From<ComptrollerCall> for CTokenStorageCalls {
        fn from(var: ComptrollerCall) -> Self {
            CTokenStorageCalls::Comptroller(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CTokenStorageCalls {
        fn from(var: DecimalsCall) -> Self {
            CTokenStorageCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<InterestRateModelCall> for CTokenStorageCalls {
        fn from(var: InterestRateModelCall) -> Self {
            CTokenStorageCalls::InterestRateModel(var)
        }
    }
    impl ::std::convert::From<NameCall> for CTokenStorageCalls {
        fn from(var: NameCall) -> Self {
            CTokenStorageCalls::Name(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for CTokenStorageCalls {
        fn from(var: PendingAdminCall) -> Self {
            CTokenStorageCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<ReserveFactorMantissaCall> for CTokenStorageCalls {
        fn from(var: ReserveFactorMantissaCall) -> Self {
            CTokenStorageCalls::ReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CTokenStorageCalls {
        fn from(var: SymbolCall) -> Self {
            CTokenStorageCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalBorrowsCall> for CTokenStorageCalls {
        fn from(var: TotalBorrowsCall) -> Self {
            CTokenStorageCalls::TotalBorrows(var)
        }
    }
    impl ::std::convert::From<TotalReservesCall> for CTokenStorageCalls {
        fn from(var: TotalReservesCall) -> Self {
            CTokenStorageCalls::TotalReserves(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CTokenStorageCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CTokenStorageCalls::TotalSupply(var)
        }
    }
}
