pub use jump_rate_model::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod jump_rate_model {
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
    #[doc = "JumpRateModel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static JUMPRATEMODEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"baseRatePerYear\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"multiplierPerYear\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"jumpMultiplierPerYear\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"kink_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"baseRatePerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"multiplierPerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"jumpMultiplierPerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"kink\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewInterestParams\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blocksPerYear\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInterestRateModel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"jumpMultiplierPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"kink\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"multiplierPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"utilizationRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static JUMPRATEMODEL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516107cb3803806107cb8339818101604052608081101561003357600080fd5b5080516020808301516040840151606090940151929390929091610065908590622014809061039e6100fd821b17901c565b60015561008083622014806100fd602090811b61039e17901c565b60005561009b82622014806100fd602090811b61039e17901c565b60028190556003829055600154600054604080519283526020830191909152818101929092526060810183905290517f6960ab234c7ef4b0c9197100f5393cfcde7c453ac910a27bd2000aa1dd4c068d9181900360800190a1505050506101ee565b600061014583836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f00000000000081525061014c60201b60201c565b9392505050565b600081836101d85760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561019d578181015183820152602001610185565b50505050905090810190601f1680156101ca5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5060008385816101e457fe5b0495945050505050565b6105ce806101fd6000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063a385fb9611610066578063a385fb9614610120578063b816881614610128578063b9f9850a14610157578063f14039de1461015f578063fd2da3391461016757610093565b806315f24053146100985780632191f92a146100d35780636e71e2d8146100ef5780638726bb8914610118575b600080fd5b6100c1600480360360608110156100ae57600080fd5b508035906020810135906040013561016f565b60408051918252519081900360200190f35b6100db610247565b604080519115158252519081900360200190f35b6100c16004803603606081101561010557600080fd5b508035906020810135906040013561024c565b6100c161029e565b6100c16102a4565b6100c16004803603608081101561013e57600080fd5b50803590602081013590604081013590606001356102ab565b6100c161032a565b6100c1610330565b6100c1610336565b60008061017d85858561024c565b905060035481116101cf576101c76001546101bb670de0b6b3a76400006101af6000548661033c90919063ffffffff16565b9063ffffffff61039e16565b9063ffffffff6103e016565b915050610240565b60006101fa6001546101bb670de0b6b3a76400006101af60005460035461033c90919063ffffffff16565b905060006102136003548461043a90919063ffffffff16565b905061023a826101bb670de0b6b3a76400006101af6002548661033c90919063ffffffff16565b93505050505b9392505050565b600181565b60008261025b57506000610240565b61029661027e83610272878763ffffffff6103e016565b9063ffffffff61043a16565b6101af85670de0b6b3a764000063ffffffff61033c16565b949350505050565b60005481565b6220148081565b6000806102c6670de0b6b3a76400008463ffffffff61043a16565b905060006102d587878761016f565b905060006102f5670de0b6b3a76400006101af848663ffffffff61033c16565b905061031e670de0b6b3a76400006101af836103128c8c8c61024c565b9063ffffffff61033c16565b98975050505050505050565b60025481565b60015481565b60035481565b60008261034b57506000610398565b8282028284828161035857fe5b04146103955760405162461bcd60e51b81526004018080602001828103825260218152602001806105796021913960400191505060405180910390fd5b90505b92915050565b600061039583836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f00000000000081525061047c565b600082820183811015610395576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b600061039583836040518060400160405280601f81526020017f536166654d6174683a207375627472616374696f6e20756e646572666c6f770081525061051e565b600081836105085760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b838110156104cd5781810151838201526020016104b5565b50505050905090810190601f1680156104fa5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b50600083858161051457fe5b0495945050505050565b600081848411156105705760405162461bcd60e51b81526020600482018181528351602484015283519092839260449091019190850190808383600083156104cd5781810151838201526020016104b5565b50505090039056fe536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77a265627a7a72315820790e86309733fdd68458e32d400a2ad598e57e23f707ea4a2d537fa5262ac42064736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    pub struct JumpRateModel<M>(ethers::contract::Contract<M>);
    impl<M> Clone for JumpRateModel<M> {
        fn clone(&self) -> Self {
            JumpRateModel(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for JumpRateModel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for JumpRateModel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(JumpRateModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> JumpRateModel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), JUMPRATEMODEL_ABI.clone(), client)
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
                JUMPRATEMODEL_ABI.clone(),
                JUMPRATEMODEL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `baseRatePerBlock` (0xf14039de) function"]
        pub fn base_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 64, 57, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blocksPerYear` (0xa385fb96) function"]
        pub fn blocks_per_year(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([163, 133, 251, 150], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBorrowRate` (0x15f24053) function"]
        pub fn get_borrow_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 242, 64, 83], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyRate` (0xb8168816) function"]
        pub fn get_supply_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
            reserve_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [184, 22, 136, 22],
                    (cash, borrows, reserves, reserve_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInterestRateModel` (0x2191f92a) function"]
        pub fn is_interest_rate_model(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([33, 145, 249, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `jumpMultiplierPerBlock` (0xb9f9850a) function"]
        pub fn jump_multiplier_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([185, 249, 133, 10], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kink` (0xfd2da339) function"]
        pub fn kink(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([253, 45, 163, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multiplierPerBlock` (0x8726bb89) function"]
        pub fn multiplier_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 38, 187, 137], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `utilizationRate` (0x6e71e2d8) function"]
        pub fn utilization_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 113, 226, 216], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewInterestParams` event"]
        pub fn new_interest_params_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewInterestParamsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewInterestParamsFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for JumpRateModel<M> {
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
        name = "NewInterestParams",
        abi = "NewInterestParams(uint256,uint256,uint256,uint256)"
    )]
    pub struct NewInterestParamsFilter {
        pub base_rate_per_block: ethers::core::types::U256,
        pub multiplier_per_block: ethers::core::types::U256,
        pub jump_multiplier_per_block: ethers::core::types::U256,
        pub kink: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `baseRatePerBlock` function with signature `baseRatePerBlock()` and selector `[241, 64, 57, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "baseRatePerBlock", abi = "baseRatePerBlock()")]
    pub struct BaseRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `blocksPerYear` function with signature `blocksPerYear()` and selector `[163, 133, 251, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "blocksPerYear", abi = "blocksPerYear()")]
    pub struct BlocksPerYearCall;
    #[doc = "Container type for all input parameters for the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowRate", abi = "getBorrowRate(uint256,uint256,uint256)")]
    pub struct GetBorrowRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
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
        name = "getSupplyRate",
        abi = "getSupplyRate(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetSupplyRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
        pub reserve_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isInterestRateModel", abi = "isInterestRateModel()")]
    pub struct IsInterestRateModelCall;
    #[doc = "Container type for all input parameters for the `jumpMultiplierPerBlock` function with signature `jumpMultiplierPerBlock()` and selector `[185, 249, 133, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "jumpMultiplierPerBlock", abi = "jumpMultiplierPerBlock()")]
    pub struct JumpMultiplierPerBlockCall;
    #[doc = "Container type for all input parameters for the `kink` function with signature `kink()` and selector `[253, 45, 163, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "kink", abi = "kink()")]
    pub struct KinkCall;
    #[doc = "Container type for all input parameters for the `multiplierPerBlock` function with signature `multiplierPerBlock()` and selector `[135, 38, 187, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "multiplierPerBlock", abi = "multiplierPerBlock()")]
    pub struct MultiplierPerBlockCall;
    #[doc = "Container type for all input parameters for the `utilizationRate` function with signature `utilizationRate(uint256,uint256,uint256)` and selector `[110, 113, 226, 216]`"]
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
        name = "utilizationRate",
        abi = "utilizationRate(uint256,uint256,uint256)"
    )]
    pub struct UtilizationRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum JumpRateModelCalls {
        BaseRatePerBlock(BaseRatePerBlockCall),
        BlocksPerYear(BlocksPerYearCall),
        GetBorrowRate(GetBorrowRateCall),
        GetSupplyRate(GetSupplyRateCall),
        IsInterestRateModel(IsInterestRateModelCall),
        JumpMultiplierPerBlock(JumpMultiplierPerBlockCall),
        Kink(KinkCall),
        MultiplierPerBlock(MultiplierPerBlockCall),
        UtilizationRate(UtilizationRateCall),
    }
    impl ethers::core::abi::AbiDecode for JumpRateModelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BaseRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::BaseRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <BlocksPerYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::BlocksPerYear(decoded));
            }
            if let Ok(decoded) =
                <GetBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::GetBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::GetSupplyRate(decoded));
            }
            if let Ok(decoded) =
                <IsInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::IsInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <JumpMultiplierPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::JumpMultiplierPerBlock(decoded));
            }
            if let Ok(decoded) = <KinkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(JumpRateModelCalls::Kink(decoded));
            }
            if let Ok(decoded) =
                <MultiplierPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::MultiplierPerBlock(decoded));
            }
            if let Ok(decoded) =
                <UtilizationRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(JumpRateModelCalls::UtilizationRate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for JumpRateModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                JumpRateModelCalls::BaseRatePerBlock(element) => element.encode(),
                JumpRateModelCalls::BlocksPerYear(element) => element.encode(),
                JumpRateModelCalls::GetBorrowRate(element) => element.encode(),
                JumpRateModelCalls::GetSupplyRate(element) => element.encode(),
                JumpRateModelCalls::IsInterestRateModel(element) => element.encode(),
                JumpRateModelCalls::JumpMultiplierPerBlock(element) => element.encode(),
                JumpRateModelCalls::Kink(element) => element.encode(),
                JumpRateModelCalls::MultiplierPerBlock(element) => element.encode(),
                JumpRateModelCalls::UtilizationRate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for JumpRateModelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                JumpRateModelCalls::BaseRatePerBlock(element) => element.fmt(f),
                JumpRateModelCalls::BlocksPerYear(element) => element.fmt(f),
                JumpRateModelCalls::GetBorrowRate(element) => element.fmt(f),
                JumpRateModelCalls::GetSupplyRate(element) => element.fmt(f),
                JumpRateModelCalls::IsInterestRateModel(element) => element.fmt(f),
                JumpRateModelCalls::JumpMultiplierPerBlock(element) => element.fmt(f),
                JumpRateModelCalls::Kink(element) => element.fmt(f),
                JumpRateModelCalls::MultiplierPerBlock(element) => element.fmt(f),
                JumpRateModelCalls::UtilizationRate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BaseRatePerBlockCall> for JumpRateModelCalls {
        fn from(var: BaseRatePerBlockCall) -> Self {
            JumpRateModelCalls::BaseRatePerBlock(var)
        }
    }
    impl ::std::convert::From<BlocksPerYearCall> for JumpRateModelCalls {
        fn from(var: BlocksPerYearCall) -> Self {
            JumpRateModelCalls::BlocksPerYear(var)
        }
    }
    impl ::std::convert::From<GetBorrowRateCall> for JumpRateModelCalls {
        fn from(var: GetBorrowRateCall) -> Self {
            JumpRateModelCalls::GetBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyRateCall> for JumpRateModelCalls {
        fn from(var: GetSupplyRateCall) -> Self {
            JumpRateModelCalls::GetSupplyRate(var)
        }
    }
    impl ::std::convert::From<IsInterestRateModelCall> for JumpRateModelCalls {
        fn from(var: IsInterestRateModelCall) -> Self {
            JumpRateModelCalls::IsInterestRateModel(var)
        }
    }
    impl ::std::convert::From<JumpMultiplierPerBlockCall> for JumpRateModelCalls {
        fn from(var: JumpMultiplierPerBlockCall) -> Self {
            JumpRateModelCalls::JumpMultiplierPerBlock(var)
        }
    }
    impl ::std::convert::From<KinkCall> for JumpRateModelCalls {
        fn from(var: KinkCall) -> Self {
            JumpRateModelCalls::Kink(var)
        }
    }
    impl ::std::convert::From<MultiplierPerBlockCall> for JumpRateModelCalls {
        fn from(var: MultiplierPerBlockCall) -> Self {
            JumpRateModelCalls::MultiplierPerBlock(var)
        }
    }
    impl ::std::convert::From<UtilizationRateCall> for JumpRateModelCalls {
        fn from(var: UtilizationRateCall) -> Self {
            JumpRateModelCalls::UtilizationRate(var)
        }
    }
    #[doc = "Container type for all return fields from the `baseRatePerBlock` function with signature `baseRatePerBlock()` and selector `[241, 64, 57, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BaseRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `blocksPerYear` function with signature `blocksPerYear()` and selector `[163, 133, 251, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BlocksPerYearReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBorrowRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsInterestRateModelReturn(pub bool);
    #[doc = "Container type for all return fields from the `jumpMultiplierPerBlock` function with signature `jumpMultiplierPerBlock()` and selector `[185, 249, 133, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct JumpMultiplierPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `kink` function with signature `kink()` and selector `[253, 45, 163, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct KinkReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `multiplierPerBlock` function with signature `multiplierPerBlock()` and selector `[135, 38, 187, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MultiplierPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `utilizationRate` function with signature `utilizationRate(uint256,uint256,uint256)` and selector `[110, 113, 226, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UtilizationRateReturn(pub ethers::core::types::U256);
}
