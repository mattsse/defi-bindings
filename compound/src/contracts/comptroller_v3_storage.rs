pub use comptrollerv3storage_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerv3storage_mod {
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
    #[doc = "ComptrollerV3Storage was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERV3STORAGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accountAssets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allMarkets\",\"outputs\":[{\"internalType\":\"contract CToken\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"closeFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compAccrued\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compBorrowerIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSpeeds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplierIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compSupplyState\",\"outputs\":[{\"internalType\":\"uint224\",\"name\":\"index\",\"type\":\"uint224\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"block\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"comptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationIncentiveMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isListed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralFactorMantissa\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isComped\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mintGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauseGuardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingComptrollerImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"seizeGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transferGuardianPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERV3STORAGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610668806100206000396000f3fe608060405234801561001057600080fd5b50600436106101735760003560e01c80638e8f294b116100de578063ca0af04311610097578063dcfbc0c711610071578063dcfbc0c714610403578063e6653f3d1461040b578063e875544614610413578063f851a4401461041b57610173565b8063ca0af04314610383578063cc7ebdc4146103b1578063dce15449146103d757610173565b80638e8f294b146102ed57806394b2294b14610335578063aa9007541461033d578063ac0b0bb714610345578063b21be7fd1461034d578063bb82aa5e1461037b57610173565b80636b79c38d116101305780636b79c38d1461021d5780636d154ea51461026b578063731f0c2b146102915780637dc0d1d0146102b757806387f76303146102bf5780638c57804e146102c757610173565b80631d7b33d71461017857806324a3d622146101b057806326782247146101d45780633c94786f146101dc5780634ada90af146101f857806352d84d1e14610200575b600080fd5b61019e6004803603602081101561018e57600080fd5b50356001600160a01b0316610423565b60408051918252519081900360200190f35b6101b8610435565b604080516001600160a01b039092168252519081900360200190f35b6101b8610444565b6101e4610453565b604080519115158252519081900360200190f35b61019e610463565b6101b86004803603602081101561021657600080fd5b5035610469565b6102436004803603602081101561023357600080fd5b50356001600160a01b0316610490565b604080516001600160e01b03909316835263ffffffff90911660208301528051918290030190f35b6101e46004803603602081101561028157600080fd5b50356001600160a01b03166104ba565b6101e4600480360360208110156102a757600080fd5b50356001600160a01b03166104cf565b6101b86104e4565b6101e46104f3565b610243600480360360208110156102dd57600080fd5b50356001600160a01b0316610503565b6103136004803603602081101561030357600080fd5b50356001600160a01b031661052d565b6040805193151584526020840192909252151582820152519081900360600190f35b61019e610553565b61019e610559565b6101e461055f565b61019e6004803603604081101561036357600080fd5b506001600160a01b038135811691602001351661056f565b6101b861058c565b61019e6004803603604081101561039957600080fd5b506001600160a01b038135811691602001351661059b565b61019e600480360360208110156103c757600080fd5b50356001600160a01b03166105b8565b6101b8600480360360408110156103ed57600080fd5b506001600160a01b0381351690602001356105ca565b6101b86105ff565b6101e461060e565b61019e61061e565b6101b8610624565b600f6020526000908152604090205481565b600a546001600160a01b031681565b6001546001600160a01b031681565b600a54600160a01b900460ff1681565b60065481565b600d818154811061047657fe5b6000918252602090912001546001600160a01b0316905081565b6010602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b600c6020526000908152604090205460ff1681565b600b6020526000908152604090205460ff1681565b6004546001600160a01b031681565b600a54600160b01b900460ff1681565b6011602052600090815260409020546001600160e01b03811690600160e01b900463ffffffff1682565b60096020526000908152604090208054600182015460039092015460ff91821692911683565b60075481565b600e5481565b600a54600160b81b900460ff1681565b601260209081526000928352604080842090915290825290205481565b6002546001600160a01b031681565b601360209081526000928352604080842090915290825290205481565b60146020526000908152604090205481565b600860205281600052604060002081815481106105e357fe5b6000918252602090912001546001600160a01b03169150829050565b6003546001600160a01b031681565b600a54600160a81b900460ff1681565b60055481565b6000546001600160a01b03168156fea265627a7a72315820fe26432306b7089a9138e67e2ed82071bfb8b99ce84239d054807eb5a532e28364736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ComptrollerV3Storage<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerV3Storage<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerV3Storage<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerV3Storage))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerV3Storage<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                COMPTROLLERV3STORAGE_ABI.clone(),
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
                COMPTROLLERV3STORAGE_ABI.clone(),
                COMPTROLLERV3STORAGE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `allMarkets` (0x52d84d1e) function"]
        pub fn all_markets(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([82, 216, 77, 30], p0)
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
        #[doc = "Calls the contract's `compAccrued` (0xcc7ebdc4) function"]
        pub fn comp_accrued(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([204, 126, 189, 196], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compBorrowState` (0x8c57804e) function"]
        pub fn comp_borrow_state(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([140, 87, 128, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compBorrowerIndex` (0xca0af043) function"]
        pub fn comp_borrower_index(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([202, 10, 240, 67], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compRate` (0xaa900754) function"]
        pub fn comp_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 144, 7, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSpeeds` (0x1d7b33d7) function"]
        pub fn comp_speeds(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 123, 51, 215], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSupplierIndex` (0xb21be7fd) function"]
        pub fn comp_supplier_index(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([178, 27, 231, 253], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `compSupplyState` (0x6b79c38d) function"]
        pub fn comp_supply_state(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u32)> {
            self.0
                .method_hash([107, 121, 195, 141], p0)
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
        for ComptrollerV3Storage<M>
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
    #[doc = "Container type for all input parameters for the `allMarkets`function with signature `allMarkets(uint256)` and selector `[82, 216, 77, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allMarkets", abi = "allMarkets(uint256)")]
    pub struct AllMarketsCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `compAccrued`function with signature `compAccrued(address)` and selector `[204, 126, 189, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compAccrued", abi = "compAccrued(address)")]
    pub struct CompAccruedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compBorrowState`function with signature `compBorrowState(address)` and selector `[140, 87, 128, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compBorrowState", abi = "compBorrowState(address)")]
    pub struct CompBorrowStateCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compBorrowerIndex`function with signature `compBorrowerIndex(address,address)` and selector `[202, 10, 240, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compBorrowerIndex", abi = "compBorrowerIndex(address,address)")]
    pub struct CompBorrowerIndexCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `compRate`function with signature `compRate()` and selector `[170, 144, 7, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compRate", abi = "compRate()")]
    pub struct CompRateCall;
    #[doc = "Container type for all input parameters for the `compSpeeds`function with signature `compSpeeds(address)` and selector `[29, 123, 51, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSpeeds", abi = "compSpeeds(address)")]
    pub struct CompSpeedsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `compSupplierIndex`function with signature `compSupplierIndex(address,address)` and selector `[178, 27, 231, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSupplierIndex", abi = "compSupplierIndex(address,address)")]
    pub struct CompSupplierIndexCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `compSupplyState`function with signature `compSupplyState(address)` and selector `[107, 121, 195, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "compSupplyState", abi = "compSupplyState(address)")]
    pub struct CompSupplyStateCall(pub ethers::core::types::Address);
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
    pub enum ComptrollerV3StorageCalls {
        _BorrowGuardianPaused(_BorrowGuardianPausedCall),
        _MintGuardianPaused(_MintGuardianPausedCall),
        AccountAssets(AccountAssetsCall),
        Admin(AdminCall),
        AllMarkets(AllMarketsCall),
        BorrowGuardianPaused(BorrowGuardianPausedCall),
        CloseFactorMantissa(CloseFactorMantissaCall),
        CompAccrued(CompAccruedCall),
        CompBorrowState(CompBorrowStateCall),
        CompBorrowerIndex(CompBorrowerIndexCall),
        CompRate(CompRateCall),
        CompSpeeds(CompSpeedsCall),
        CompSupplierIndex(CompSupplierIndexCall),
        CompSupplyState(CompSupplyStateCall),
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
    impl ethers::core::abi::AbiDecode for ComptrollerV3StorageCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <_BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::_BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <_MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::_MintGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <AccountAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::AccountAssets(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AllMarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::AllMarkets(decoded));
            }
            if let Ok(decoded) =
                <BorrowGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::BorrowGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <CloseFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CloseFactorMantissa(decoded));
            }
            if let Ok(decoded) =
                <CompAccruedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompAccrued(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompBorrowState(decoded));
            }
            if let Ok(decoded) =
                <CompBorrowerIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompBorrowerIndex(decoded));
            }
            if let Ok(decoded) =
                <CompRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompRate(decoded));
            }
            if let Ok(decoded) =
                <CompSpeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompSpeeds(decoded));
            }
            if let Ok(decoded) =
                <CompSupplierIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompSupplierIndex(decoded));
            }
            if let Ok(decoded) =
                <CompSupplyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::CompSupplyState(decoded));
            }
            if let Ok(decoded) =
                <ComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV3StorageCalls::ComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LiquidationIncentiveMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV3StorageCalls::LiquidationIncentiveMantissa(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::Markets(decoded));
            }
            if let Ok(decoded) =
                <MaxAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::MaxAssets(decoded));
            }
            if let Ok(decoded) =
                <MintGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::MintGuardianPaused(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::Oracle(decoded));
            }
            if let Ok(decoded) =
                <PauseGuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::PauseGuardian(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <PendingComptrollerImplementationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ComptrollerV3StorageCalls::PendingComptrollerImplementation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SeizeGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::SeizeGuardianPaused(decoded));
            }
            if let Ok(decoded) =
                <TransferGuardianPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerV3StorageCalls::TransferGuardianPaused(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerV3StorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerV3StorageCalls::_BorrowGuardianPaused(element) => element.encode(),
                ComptrollerV3StorageCalls::_MintGuardianPaused(element) => element.encode(),
                ComptrollerV3StorageCalls::AccountAssets(element) => element.encode(),
                ComptrollerV3StorageCalls::Admin(element) => element.encode(),
                ComptrollerV3StorageCalls::AllMarkets(element) => element.encode(),
                ComptrollerV3StorageCalls::BorrowGuardianPaused(element) => element.encode(),
                ComptrollerV3StorageCalls::CloseFactorMantissa(element) => element.encode(),
                ComptrollerV3StorageCalls::CompAccrued(element) => element.encode(),
                ComptrollerV3StorageCalls::CompBorrowState(element) => element.encode(),
                ComptrollerV3StorageCalls::CompBorrowerIndex(element) => element.encode(),
                ComptrollerV3StorageCalls::CompRate(element) => element.encode(),
                ComptrollerV3StorageCalls::CompSpeeds(element) => element.encode(),
                ComptrollerV3StorageCalls::CompSupplierIndex(element) => element.encode(),
                ComptrollerV3StorageCalls::CompSupplyState(element) => element.encode(),
                ComptrollerV3StorageCalls::ComptrollerImplementation(element) => element.encode(),
                ComptrollerV3StorageCalls::LiquidationIncentiveMantissa(element) => {
                    element.encode()
                }
                ComptrollerV3StorageCalls::Markets(element) => element.encode(),
                ComptrollerV3StorageCalls::MaxAssets(element) => element.encode(),
                ComptrollerV3StorageCalls::MintGuardianPaused(element) => element.encode(),
                ComptrollerV3StorageCalls::Oracle(element) => element.encode(),
                ComptrollerV3StorageCalls::PauseGuardian(element) => element.encode(),
                ComptrollerV3StorageCalls::PendingAdmin(element) => element.encode(),
                ComptrollerV3StorageCalls::PendingComptrollerImplementation(element) => {
                    element.encode()
                }
                ComptrollerV3StorageCalls::SeizeGuardianPaused(element) => element.encode(),
                ComptrollerV3StorageCalls::TransferGuardianPaused(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerV3StorageCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerV3StorageCalls::_BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerV3StorageCalls::_MintGuardianPaused(element) => element.fmt(f),
                ComptrollerV3StorageCalls::AccountAssets(element) => element.fmt(f),
                ComptrollerV3StorageCalls::Admin(element) => element.fmt(f),
                ComptrollerV3StorageCalls::AllMarkets(element) => element.fmt(f),
                ComptrollerV3StorageCalls::BorrowGuardianPaused(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CloseFactorMantissa(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompAccrued(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompBorrowState(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompBorrowerIndex(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompRate(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompSpeeds(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompSupplierIndex(element) => element.fmt(f),
                ComptrollerV3StorageCalls::CompSupplyState(element) => element.fmt(f),
                ComptrollerV3StorageCalls::ComptrollerImplementation(element) => element.fmt(f),
                ComptrollerV3StorageCalls::LiquidationIncentiveMantissa(element) => element.fmt(f),
                ComptrollerV3StorageCalls::Markets(element) => element.fmt(f),
                ComptrollerV3StorageCalls::MaxAssets(element) => element.fmt(f),
                ComptrollerV3StorageCalls::MintGuardianPaused(element) => element.fmt(f),
                ComptrollerV3StorageCalls::Oracle(element) => element.fmt(f),
                ComptrollerV3StorageCalls::PauseGuardian(element) => element.fmt(f),
                ComptrollerV3StorageCalls::PendingAdmin(element) => element.fmt(f),
                ComptrollerV3StorageCalls::PendingComptrollerImplementation(element) => {
                    element.fmt(f)
                }
                ComptrollerV3StorageCalls::SeizeGuardianPaused(element) => element.fmt(f),
                ComptrollerV3StorageCalls::TransferGuardianPaused(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<_BorrowGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: _BorrowGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::_BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<_MintGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: _MintGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::_MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<AccountAssetsCall> for ComptrollerV3StorageCalls {
        fn from(var: AccountAssetsCall) -> Self {
            ComptrollerV3StorageCalls::AccountAssets(var)
        }
    }
    impl ::std::convert::From<AdminCall> for ComptrollerV3StorageCalls {
        fn from(var: AdminCall) -> Self {
            ComptrollerV3StorageCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AllMarketsCall> for ComptrollerV3StorageCalls {
        fn from(var: AllMarketsCall) -> Self {
            ComptrollerV3StorageCalls::AllMarkets(var)
        }
    }
    impl ::std::convert::From<BorrowGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: BorrowGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::BorrowGuardianPaused(var)
        }
    }
    impl ::std::convert::From<CloseFactorMantissaCall> for ComptrollerV3StorageCalls {
        fn from(var: CloseFactorMantissaCall) -> Self {
            ComptrollerV3StorageCalls::CloseFactorMantissa(var)
        }
    }
    impl ::std::convert::From<CompAccruedCall> for ComptrollerV3StorageCalls {
        fn from(var: CompAccruedCall) -> Self {
            ComptrollerV3StorageCalls::CompAccrued(var)
        }
    }
    impl ::std::convert::From<CompBorrowStateCall> for ComptrollerV3StorageCalls {
        fn from(var: CompBorrowStateCall) -> Self {
            ComptrollerV3StorageCalls::CompBorrowState(var)
        }
    }
    impl ::std::convert::From<CompBorrowerIndexCall> for ComptrollerV3StorageCalls {
        fn from(var: CompBorrowerIndexCall) -> Self {
            ComptrollerV3StorageCalls::CompBorrowerIndex(var)
        }
    }
    impl ::std::convert::From<CompRateCall> for ComptrollerV3StorageCalls {
        fn from(var: CompRateCall) -> Self {
            ComptrollerV3StorageCalls::CompRate(var)
        }
    }
    impl ::std::convert::From<CompSpeedsCall> for ComptrollerV3StorageCalls {
        fn from(var: CompSpeedsCall) -> Self {
            ComptrollerV3StorageCalls::CompSpeeds(var)
        }
    }
    impl ::std::convert::From<CompSupplierIndexCall> for ComptrollerV3StorageCalls {
        fn from(var: CompSupplierIndexCall) -> Self {
            ComptrollerV3StorageCalls::CompSupplierIndex(var)
        }
    }
    impl ::std::convert::From<CompSupplyStateCall> for ComptrollerV3StorageCalls {
        fn from(var: CompSupplyStateCall) -> Self {
            ComptrollerV3StorageCalls::CompSupplyState(var)
        }
    }
    impl ::std::convert::From<ComptrollerImplementationCall> for ComptrollerV3StorageCalls {
        fn from(var: ComptrollerImplementationCall) -> Self {
            ComptrollerV3StorageCalls::ComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<LiquidationIncentiveMantissaCall> for ComptrollerV3StorageCalls {
        fn from(var: LiquidationIncentiveMantissaCall) -> Self {
            ComptrollerV3StorageCalls::LiquidationIncentiveMantissa(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerV3StorageCalls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerV3StorageCalls::Markets(var)
        }
    }
    impl ::std::convert::From<MaxAssetsCall> for ComptrollerV3StorageCalls {
        fn from(var: MaxAssetsCall) -> Self {
            ComptrollerV3StorageCalls::MaxAssets(var)
        }
    }
    impl ::std::convert::From<MintGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: MintGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::MintGuardianPaused(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerV3StorageCalls {
        fn from(var: OracleCall) -> Self {
            ComptrollerV3StorageCalls::Oracle(var)
        }
    }
    impl ::std::convert::From<PauseGuardianCall> for ComptrollerV3StorageCalls {
        fn from(var: PauseGuardianCall) -> Self {
            ComptrollerV3StorageCalls::PauseGuardian(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for ComptrollerV3StorageCalls {
        fn from(var: PendingAdminCall) -> Self {
            ComptrollerV3StorageCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<PendingComptrollerImplementationCall> for ComptrollerV3StorageCalls {
        fn from(var: PendingComptrollerImplementationCall) -> Self {
            ComptrollerV3StorageCalls::PendingComptrollerImplementation(var)
        }
    }
    impl ::std::convert::From<SeizeGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: SeizeGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::SeizeGuardianPaused(var)
        }
    }
    impl ::std::convert::From<TransferGuardianPausedCall> for ComptrollerV3StorageCalls {
        fn from(var: TransferGuardianPausedCall) -> Self {
            ComptrollerV3StorageCalls::TransferGuardianPaused(var)
        }
    }
}
