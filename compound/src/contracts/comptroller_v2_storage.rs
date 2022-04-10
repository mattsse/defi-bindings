pub use comptrollerv2storage_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerv2storage_mod {
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
    #[doc = "ComptrollerV2Storage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERV2STORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isListed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauseGuardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"seizeGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transferGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERV2STORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506103e0806100206000396000f3fe608060405234801561001057600080fd5b506004361061010b5760003560e01c80638e8f294b116100a2578063dce1544911610071578063dce154491461022e578063dcfbc0c71461025a578063e6653f3d14610262578063e87554461461026a578063f851a440146102725761010b565b80638e8f294b146101ce57806394b2294b14610216578063ac0b0bb71461021e578063bb82aa5e146102265761010b565b80636d154ea5116100de5780636d154ea514610172578063731f0c2b146101985780637dc0d1d0146101be57806387f76303146101c65761010b565b806324a3d6221461011057806326782247146101345780633c94786f1461013c5780634ada90af14610158575b600080fd5b61011861027a565b604080516001600160a01b039092168252519081900360200190f35b610118610289565b610144610298565b604080519115158252519081900360200190f35b6101606102a8565b60408051918252519081900360200190f35b6101446004803603602081101561018857600080fd5b50356001600160a01b03166102ae565b610144600480360360208110156101ae57600080fd5b50356001600160a01b03166102c3565b6101186102d8565b6101446102e7565b6101f4600480360360208110156101e457600080fd5b50356001600160a01b03166102f7565b6040805193151584526020840192909252151582820152519081900360600190f35b61016061031d565b610144610323565b610118610333565b6101186004803603604081101561024457600080fd5b506001600160a01b038135169060200135610342565b610118610377565b610144610386565b610160610396565b61011861039c565b600a546001600160a01b031681565b6001546001600160a01b031681565b600a54600160a01b900460ff1681565b60065481565b600c6020526000908152604090205460ff1681565b600b6020526000908152604090205460ff1681565b6004546001600160a01b031681565b600a54600160b01b900460ff1681565b60096020526000908152604090208054600182015460039092015460ff91821692911683565b60075481565b600a54600160b81b900460ff1681565b6002546001600160a01b031681565b6008602052816000526040600020818154811061035b57fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b600a54600160a81b900460ff1681565b60055481565b6000546001600160a01b03168156fea265627a7a72315820927a5f1616ec3cfa5ea26aa5fe33234f107322babd336941bce2a9cdd71eeb9964736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ComptrollerV2Storage<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerV2Storage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerV2Storage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerV2Storage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerV2Storage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                COMPTROLLERV2STORAGE_ABI.clone(),
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
                COMPTROLLERV2STORAGE_ABI.clone(),
                COMPTROLLERV2STORAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_borrowGuardianPaused` (0xe6653f3d) function"]
        pub fn _borrow_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 101, 63, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_mintGuardianPaused` (0x3c94786f) function"]
        pub fn _mint_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([60, 148, 120, 111], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `borrowGuardianPaused` (0x6d154ea5) function"]
        pub fn borrow_guardian_paused(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 21, 78, 165], p0)
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
        #[doc = "Calls the contract's `markets` (0x8e8f294b) function"]
        pub fn markets(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash([142, 143, 41, 75], p0)
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
        #[doc = "Calls the contract's `mintGuardianPaused` (0x731f0c2b) function"]
        pub fn mint_guardian_paused(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 31, 12, 43], p0)
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
        #[doc = "Calls the contract's `pauseGuardian` (0x24a3d622) function"]
        pub fn pause_guardian(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([36, 163, 214, 34], ())
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
        #[doc = "Calls the contract's `seizeGuardianPaused` (0xac0b0bb7) function"]
        pub fn seize_guardian_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 11, 11, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferGuardianPaused` (0x87f76303) function"]
        pub fn transfer_guardian_paused(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 247, 99, 3], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ComptrollerV2Storage<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `_borrowGuardianPaused`function with signature `_borrowGuardianPaused()` and selector `[230, 101, 63, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_borrowGuardianPaused", abi = "_borrowGuardianPaused()")]
    pub struct _BorrowGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `_mintGuardianPaused`function with signature `_mintGuardianPaused()` and selector `[60, 148, 120, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_mintGuardianPaused", abi = "_mintGuardianPaused()")]
    pub struct _MintGuardianPausedCall;
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
    #[doc = "Container type for all input parameters for the `borrowGuardianPaused`function with signature `borrowGuardianPaused(address)` and selector `[109, 21, 78, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowGuardianPaused", abi = "borrowGuardianPaused(address)")]
    pub struct BorrowGuardianPausedCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `markets`function with signature `markets(address)` and selector `[142, 143, 41, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "markets", abi = "markets(address)")]
    pub struct MarketsCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `mintGuardianPaused`function with signature `mintGuardianPaused(address)` and selector `[115, 31, 12, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintGuardianPaused", abi = "mintGuardianPaused(address)")]
    pub struct MintGuardianPausedCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `pauseGuardian`function with signature `pauseGuardian()` and selector `[36, 163, 214, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pauseGuardian", abi = "pauseGuardian()")]
    pub struct PauseGuardianCall;
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
    #[doc = "Container type for all input parameters for the `seizeGuardianPaused`function with signature `seizeGuardianPaused()` and selector `[172, 11, 11, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "seizeGuardianPaused", abi = "seizeGuardianPaused()")]
    pub struct SeizeGuardianPausedCall;
    #[doc = "Container type for all input parameters for the `transferGuardianPaused`function with signature `transferGuardianPaused()` and selector `[135, 247, 99, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferGuardianPaused", abi = "transferGuardianPaused()")]
    pub struct TransferGuardianPausedCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ComptrollerV2StorageCalls {
        _BorrowGuardianPaused(_BorrowGuardianPausedCall),
        _MintGuardianPaused(_MintGuardianPausedCall),
        AccountAssets(AccountAssetsCall),
        Admin(AdminCall),
        BorrowGuardianPaused(BorrowGuardianPausedCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        ComptrollerImplementation(ComptrollerImplementationCall),
        LiquidationIncentiveMantissa(LiquidationIncentiveMantissaCall),
        Markets(MarketsCall),
        MaxAssets(MaxAssetsCall),
        MintGuardianPaused(MintGuardianPausedCall),
        Oracle(OracleCall),
        PauseGuardian(PauseGuardianCall),
        PendingAdmin(PendingAdminCall),
        PendingComptrollerImplementation(PendingComptrollerImplementationCall),
        SeizeGuardianPaused(SeizeGuardianPausedCall),
        TransferGuardianPaused(TransferGuardianPausedCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerV2StorageCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <_BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <_MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV2StorageCalls::ComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV2StorageCalls::LiquidationIncentiveMantissa(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::Markets(decoded));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::MaxAssets(decoded));
            }
            if let Ok(decoded) =
                <MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::MintGuardianPaused(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::PauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV2StorageCalls::PendingComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SeizeGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <TransferGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV2StorageCalls::TransferGuardianPaused(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerV2StorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerV2StorageCalls::_BorrowGuardianPaused(element) => element.encode(),
                ComptrollerV2StorageCalls::_MintGuardianPaused(element) => element.encode(),
                ComptrollerV2StorageCalls::AccountAssets(element) => element.encode(),
                ComptrollerV2StorageCalls::Admin(element) => element.encode(),
                ComptrollerV2StorageCalls::BorrowGuardianPaused(element) => element.encode(),
                ComptrollerV2StorageCalls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerV2StorageCalls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerV2StorageCalls::LiquidationIncentiveMantissa(element) => {
                    element.encode()
                }
                ComptrollerV2StorageCalls::Markets(element) => element.encode(),
                ComptrollerV2StorageCalls::MaxAssets(element) => element.encode(),
                ComptrollerV2StorageCalls::MintGuardianPaused(element) => element.encode(),
                ComptrollerV2StorageCalls::Oracle(element) => element.encode(),
                ComptrollerV2StorageCalls::PauseGuardian(element) => element.encode(),
                ComptrollerV2StorageCalls::PendingAdmin(element) => element.encode(),
                ComptrollerV2StorageCalls::PendingComptrollerImplementation(element) => {
                    element.encode()
                }
                ComptrollerV2StorageCalls::SeizeGuardianPaused(element) => element.encode(),
                ComptrollerV2StorageCalls::TransferGuardianPaused(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerV2StorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerV2StorageCalls::_BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerV2StorageCalls::_MintGuardianPaused(element) => element.fmt(f),
                ComptrollerV2StorageCalls::AccountAssets(element) => element.fmt(f),
                ComptrollerV2StorageCalls::Admin(element) => element.fmt(f),
                ComptrollerV2StorageCalls::BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerV2StorageCalls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerV2StorageCalls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerV2StorageCalls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerV2StorageCalls::Markets(element) => element.fmt(f),
                ComptrollerV2StorageCalls::MaxAssets(element) => element.fmt(f),
                ComptrollerV2StorageCalls::MintGuardianPaused(element) => element.fmt(f),
                ComptrollerV2StorageCalls::Oracle(element) => element.fmt(f),
                ComptrollerV2StorageCalls::PauseGuardian(element) => element.fmt(f),
                ComptrollerV2StorageCalls::PendingAdmin(element) => element.fmt(f),
                ComptrollerV2StorageCalls::PendingComptrollerImplementation(element) => {
                    element.fmt(f)
                }
                ComptrollerV2StorageCalls::SeizeGuardianPaused(element) => element.fmt(f),
                ComptrollerV2StorageCalls::TransferGuardianPaused(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<_BorrowGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: _BorrowGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::_BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<_MintGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: _MintGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::_MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerV2StorageCalls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerV2StorageCalls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerV2StorageCalls {
        fn from(var: AdminCall) -> Self {
            ComptrollerV2StorageCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BorrowGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: BorrowGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerV2StorageCalls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerV2StorageCalls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerV2StorageCalls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerV2StorageCalls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerV2StorageCalls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerV2StorageCalls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerV2StorageCalls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerV2StorageCalls::Markets(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerV2StorageCalls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerV2StorageCalls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<MintGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: MintGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerV2StorageCalls {
        fn from(var: OracleCall) -> Self {
            ComptrollerV2StorageCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PauseGuardianCall> for ComptrollerV2StorageCalls {
        fn from(var: PauseGuardianCall) -> Self {
            ComptrollerV2StorageCalls::PauseGuardian(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerV2StorageCalls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerV2StorageCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerV2StorageCalls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerV2StorageCalls::PendingComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<SeizeGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: SeizeGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::SeizeGuardianPaused(var)
        }
    }
    impl ::std::convert::From<TransferGuardianPausedCall> for ComptrollerV2StorageCalls {
        fn from(var: TransferGuardianPausedCall) -> Self {
            ComptrollerV2StorageCalls::TransferGuardianPaused(var)
        }
    }
}
