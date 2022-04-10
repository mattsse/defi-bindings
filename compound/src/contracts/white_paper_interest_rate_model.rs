pub use whitepaperinterestratemodel_mod::*;
#[allow(clippy::too_many_arguments)]
mod whitepaperinterestratemodel_mod {
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
    #[doc = "WhitePaperInterestRateModel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WHITEPAPERINTERESTRATEMODEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"baseRatePerYear\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"multiplierPerYear\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"baseRatePerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"multiplierPerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewInterestParams\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blocksPerYear\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInterestRateModel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"multiplierPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"utilizationRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static WHITEPAPERINTERESTRATEMODEL_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b506040516106da3803806106da8339818101604052604081101561003357600080fd5b508051602091820151909161005690839062201480906102ee6100bc821b17901c565b60015561007181622014806100bc602090811b6102ee17901c565b600081905560015460408051918252602082019290925281517ff35fa19c15e9ba782633a5df62a98b20217151addc68e3ff2cd623a48d37ec27929181900390910190a150506101ad565b600061010483836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f00000000000081525061010b60201b60201c565b9392505050565b600081836101975760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561015c578181015183820152602001610144565b50505050905090810190601f1680156101895780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5060008385816101a357fe5b0495945050505050565b61051e806101bc6000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c80638726bb891161005b5780638726bb8914610102578063a385fb961461010a578063b816881614610112578063f14039de146101415761007d565b806315f24053146100825780632191f92a146100bd5780636e71e2d8146100d9575b600080fd5b6100ab6004803603606081101561009857600080fd5b5080359060208101359060400135610149565b60408051918252519081900360200190f35b6100c56101a3565b604080519115158252519081900360200190f35b6100ab600480360360608110156100ef57600080fd5b50803590602081013590604001356101a8565b6100ab6101fa565b6100ab610200565b6100ab6004803603608081101561012857600080fd5b5080359060208101359060408101359060600135610207565b6100ab610286565b6000806101578585856101a8565b905061019860015461018c670de0b6b3a76400006101806000548661028c90919063ffffffff16565b9063ffffffff6102ee16565b9063ffffffff61033016565b9150505b9392505050565b600181565b6000826101b75750600061019c565b6101f26101da836101ce878763ffffffff61033016565b9063ffffffff61038a16565b61018085670de0b6b3a764000063ffffffff61028c16565b949350505050565b60005481565b6220148081565b600080610222670de0b6b3a76400008463ffffffff61038a16565b90506000610231878787610149565b90506000610251670de0b6b3a7640000610180848663ffffffff61028c16565b905061027a670de0b6b3a76400006101808361026e8c8c8c6101a8565b9063ffffffff61028c16565b98975050505050505050565b60015481565b60008261029b575060006102e8565b828202828482816102a857fe5b04146102e55760405162461bcd60e51b81526004018080602001828103825260218152602001806104c96021913960400191505060405180910390fd5b90505b92915050565b60006102e583836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f0000000000008152506103cc565b6000828201838110156102e5576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b60006102e583836040518060400160405280601f81526020017f536166654d6174683a207375627472616374696f6e20756e646572666c6f770081525061046e565b600081836104585760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561041d578181015183820152602001610405565b50505050905090810190601f16801561044a5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b50600083858161046457fe5b0495945050505050565b600081848411156104c05760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561041d578181015183820152602001610405565b50505090039056fe536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77a265627a7a72315820c2f7375614941a3b54c44dbd3328414dc073ae2c42974d7320cbf2bb0919485d64736f6c63430005110032" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct WhitePaperInterestRateModel<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for WhitePaperInterestRateModel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for WhitePaperInterestRateModel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(WhitePaperInterestRateModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> WhitePaperInterestRateModel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                WHITEPAPERINTERESTRATEMODEL_ABI.clone(),
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
                WHITEPAPERINTERESTRATEMODEL_ABI.clone(),
                WHITEPAPERINTERESTRATEMODEL_BYTECODE.clone().into(),
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for WhitePaperInterestRateModel<M>
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
    #[ethevent(name = "NewInterestParams", abi = "NewInterestParams(uint256,uint256)")]
    pub struct NewInterestParamsFilter {
        pub base_rate_per_block: ethers::core::types::U256,
        pub multiplier_per_block: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `baseRatePerBlock`function with signature `baseRatePerBlock()` and selector `[241, 64, 57, 222]`"]
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
    #[doc = "Container type for all input parameters for the `blocksPerYear`function with signature `blocksPerYear()` and selector `[163, 133, 251, 150]`"]
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
    #[doc = "Container type for all input parameters for the `getBorrowRate`function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
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
    #[doc = "Container type for all input parameters for the `getSupplyRate`function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
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
    #[doc = "Container type for all input parameters for the `isInterestRateModel`function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
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
    #[doc = "Container type for all input parameters for the `multiplierPerBlock`function with signature `multiplierPerBlock()` and selector `[135, 38, 187, 137]`"]
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
    #[doc = "Container type for all input parameters for the `utilizationRate`function with signature `utilizationRate(uint256,uint256,uint256)` and selector `[110, 113, 226, 216]`"]
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
    pub enum WhitePaperInterestRateModelCalls {
        BaseRatePerBlock(BaseRatePerBlockCall),
        BlocksPerYear(BlocksPerYearCall),
        GetBorrowRate(GetBorrowRateCall),
        GetSupplyRate(GetSupplyRateCall),
        IsInterestRateModel(IsInterestRateModelCall),
        MultiplierPerBlock(MultiplierPerBlockCall),
        UtilizationRate(UtilizationRateCall),
    }
    impl ethers::core::abi::AbiDecode for WhitePaperInterestRateModelCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BaseRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::BaseRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <BlocksPerYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::BlocksPerYear(decoded));
            }
            if let Ok(decoded) =
                <GetBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::GetBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::GetSupplyRate(decoded));
            }
            if let Ok(decoded) =
                <IsInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::IsInterestRateModel(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MultiplierPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::MultiplierPerBlock(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UtilizationRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WhitePaperInterestRateModelCalls::UtilizationRate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for WhitePaperInterestRateModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                WhitePaperInterestRateModelCalls::BaseRatePerBlock(element) => element.encode(),
                WhitePaperInterestRateModelCalls::BlocksPerYear(element) => element.encode(),
                WhitePaperInterestRateModelCalls::GetBorrowRate(element) => element.encode(),
                WhitePaperInterestRateModelCalls::GetSupplyRate(element) => element.encode(),
                WhitePaperInterestRateModelCalls::IsInterestRateModel(element) => element.encode(),
                WhitePaperInterestRateModelCalls::MultiplierPerBlock(element) => element.encode(),
                WhitePaperInterestRateModelCalls::UtilizationRate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for WhitePaperInterestRateModelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WhitePaperInterestRateModelCalls::BaseRatePerBlock(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::BlocksPerYear(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::GetBorrowRate(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::GetSupplyRate(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::IsInterestRateModel(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::MultiplierPerBlock(element) => element.fmt(f),
                WhitePaperInterestRateModelCalls::UtilizationRate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BaseRatePerBlockCall> for WhitePaperInterestRateModelCalls {
        fn from(var: BaseRatePerBlockCall) -> Self {
            WhitePaperInterestRateModelCalls::BaseRatePerBlock(var)
        }
    }
    impl ::std::convert::From<BlocksPerYearCall> for WhitePaperInterestRateModelCalls {
        fn from(var: BlocksPerYearCall) -> Self {
            WhitePaperInterestRateModelCalls::BlocksPerYear(var)
        }
    }
    impl ::std::convert::From<GetBorrowRateCall> for WhitePaperInterestRateModelCalls {
        fn from(var: GetBorrowRateCall) -> Self {
            WhitePaperInterestRateModelCalls::GetBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyRateCall> for WhitePaperInterestRateModelCalls {
        fn from(var: GetSupplyRateCall) -> Self {
            WhitePaperInterestRateModelCalls::GetSupplyRate(var)
        }
    }
    impl ::std::convert::From<IsInterestRateModelCall> for WhitePaperInterestRateModelCalls {
        fn from(var: IsInterestRateModelCall) -> Self {
            WhitePaperInterestRateModelCalls::IsInterestRateModel(var)
        }
    }
    impl ::std::convert::From<MultiplierPerBlockCall> for WhitePaperInterestRateModelCalls {
        fn from(var: MultiplierPerBlockCall) -> Self {
            WhitePaperInterestRateModelCalls::MultiplierPerBlock(var)
        }
    }
    impl ::std::convert::From<UtilizationRateCall> for WhitePaperInterestRateModelCalls {
        fn from(var: UtilizationRateCall) -> Self {
            WhitePaperInterestRateModelCalls::UtilizationRate(var)
        }
    }
}
