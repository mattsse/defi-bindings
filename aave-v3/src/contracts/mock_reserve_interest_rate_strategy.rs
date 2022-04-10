pub use mockreserveinterestratestrategy_mod::*;
#[allow(clippy::too_many_arguments)]
mod mockreserveinterestratestrategy_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "MockReserveInterestRateStrategy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKRESERVEINTERESTRATESTRATEGY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"optimalUsageRatio\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseVariableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableRateSlope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableRateSlope2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableRateSlope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableRateSlope2\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"OPTIMAL_USAGE_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct DataTypes.CalculateInterestRatesParams\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"unbacked\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityAdded\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityTaken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalStableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalVariableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"averageStableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aToken\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateInterestRates\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidityRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableBorrowRate\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBaseVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateSlope1\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateSlope2\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVariableRateSlope1\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVariableRateSlope2\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidityRate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidityRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"stableBorrowRate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStableBorrowRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"variableBorrowRate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVariableBorrowRate\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKRESERVEINTERESTRATESTRATEGY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x61016060405234801561001157600080fd5b5060405161056938038061056983398101604081905261003091610061565b6080959095526001600160a01b0390951660a05260c09290925260e0526101005261012091909152610140526100ca565b600080600080600080600060e0888a03121561007c57600080fd5b87516001600160a01b038116811461009357600080fd5b602089015160408a015160608b015160808c015160a08d015160c0909d0151949e939d50919b909a50909850965090945092505050565b60805160a05160c05160e051610100516101205161014051610432610137600039600061012f0152600061021e015260008181610244015261026c01526000818160ff015261029001526000818161015501526102b10152600060be0152600061019301526104326000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c806380031e371161007157806380031e37146101b5578063a5898709146101bd578063aa16fe34146101f6578063cecced5114610209578063d5cd73911461021c578063f42024091461024257600080fd5b80630542975c146100b95780630b3429a2146100fd57806314e32da41461012d57806334762ca5146101535780633a244adf1461017957806354c365c61461018e575b600080fd5b6100e07f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b7f00000000000000000000000000000000000000000000000000000000000000005b6040519081526020016100f4565b7f000000000000000000000000000000000000000000000000000000000000000061011f565b7f000000000000000000000000000000000000000000000000000000000000000061011f565b61018c6101873660046102e4565b600055565b005b61011f7f000000000000000000000000000000000000000000000000000000000000000081565b61011f610268565b6101db6101cb366004610351565b6000546001546002549193909250565b604080519384526020840192909252908201526060016100f4565b61018c6102043660046102e4565b600255565b61018c6102173660046102e4565b600155565b7f000000000000000000000000000000000000000000000000000000000000000061011f565b7f000000000000000000000000000000000000000000000000000000000000000061011f565b60007f00000000000000000000000000000000000000000000000000000000000000006102d57f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006103d6565b6102df91906103d6565b905090565b6000602082840312156102f657600080fd5b5035919050565b604051610120810167ffffffffffffffff8111828210171561032f57634e487b7160e01b600052604160045260246000fd5b60405290565b80356001600160a01b038116811461034c57600080fd5b919050565b6000610120828403121561036457600080fd5b61036c6102fd565b823581526020830135602082015260408301356040820152606083013560608201526080830135608082015260a083013560a082015260c083013560c08201526103b860e08401610335565b60e08201526101006103cb818501610335565b908201529392505050565b600082198211156103f757634e487b7160e01b600052601160045260246000fd5b50019056fea26469706673582212200c951f4bb9fa2afd6986df4ffae3dbac91f1e2673b39be6445183e588560ea3f64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct MockReserveInterestRateStrategy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MockReserveInterestRateStrategy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockReserveInterestRateStrategy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockReserveInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MockReserveInterestRateStrategy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKRESERVEINTERESTRATESTRATEGY_ABI.clone(),
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
                MOCKRESERVEINTERESTRATESTRATEGY_ABI.clone(),
                MOCKRESERVEINTERESTRATESTRATEGY_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `OPTIMAL_USAGE_RATIO` (0x54c365c6) function"]
        pub fn optimal_usage_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 195, 101, 198], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateInterestRates` (0xa5898709) function"]
        pub fn calculate_interest_rates(
            &self,
            p0: CalculateInterestRatesParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (p0,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBaseVariableBorrowRate` (0x34762ca5) function"]
        pub fn get_base_variable_borrow_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 118, 44, 165], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMaxVariableBorrowRate` (0x80031e37) function"]
        pub fn get_max_variable_borrow_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([128, 3, 30, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStableRateSlope1` (0xd5cd7391) function"]
        pub fn get_stable_rate_slope_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([213, 205, 115, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStableRateSlope2` (0x14e32da4) function"]
        pub fn get_stable_rate_slope_2(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([20, 227, 45, 164], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVariableRateSlope1` (0x0b3429a2) function"]
        pub fn get_variable_rate_slope_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 52, 41, 162], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getVariableRateSlope2` (0xf4202409) function"]
        pub fn get_variable_rate_slope_2(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 32, 36, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidityRate` (0x3a244adf) function"]
        pub fn set_liquidity_rate(
            &self,
            liquidity_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 36, 74, 223], liquidity_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStableBorrowRate` (0xcecced51) function"]
        pub fn set_stable_borrow_rate(
            &self,
            stable_borrow_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 204, 237, 81], stable_borrow_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVariableBorrowRate` (0xaa16fe34) function"]
        pub fn set_variable_borrow_rate(
            &self,
            variable_borrow_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 22, 254, 52], variable_borrow_rate)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockReserveInterestRateStrategy<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER`function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
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
    #[doc = "Container type for all input parameters for the `OPTIMAL_USAGE_RATIO`function with signature `OPTIMAL_USAGE_RATIO()` and selector `[84, 195, 101, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "OPTIMAL_USAGE_RATIO", abi = "OPTIMAL_USAGE_RATIO()")]
    pub struct OptimalUsageRatioCall;
    #[doc = "Container type for all input parameters for the `calculateInterestRates`function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `[165, 137, 135, 9]`"]
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
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall(pub CalculateInterestRatesParams);
    #[doc = "Container type for all input parameters for the `getBaseVariableBorrowRate`function with signature `getBaseVariableBorrowRate()` and selector `[52, 118, 44, 165]`"]
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
        name = "getBaseVariableBorrowRate",
        abi = "getBaseVariableBorrowRate()"
    )]
    pub struct GetBaseVariableBorrowRateCall;
    #[doc = "Container type for all input parameters for the `getMaxVariableBorrowRate`function with signature `getMaxVariableBorrowRate()` and selector `[128, 3, 30, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMaxVariableBorrowRate", abi = "getMaxVariableBorrowRate()")]
    pub struct GetMaxVariableBorrowRateCall;
    #[doc = "Container type for all input parameters for the `getStableRateSlope1`function with signature `getStableRateSlope1()` and selector `[213, 205, 115, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getStableRateSlope1", abi = "getStableRateSlope1()")]
    pub struct GetStableRateSlope1Call;
    #[doc = "Container type for all input parameters for the `getStableRateSlope2`function with signature `getStableRateSlope2()` and selector `[20, 227, 45, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getStableRateSlope2", abi = "getStableRateSlope2()")]
    pub struct GetStableRateSlope2Call;
    #[doc = "Container type for all input parameters for the `getVariableRateSlope1`function with signature `getVariableRateSlope1()` and selector `[11, 52, 41, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getVariableRateSlope1", abi = "getVariableRateSlope1()")]
    pub struct GetVariableRateSlope1Call;
    #[doc = "Container type for all input parameters for the `getVariableRateSlope2`function with signature `getVariableRateSlope2()` and selector `[244, 32, 36, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getVariableRateSlope2", abi = "getVariableRateSlope2()")]
    pub struct GetVariableRateSlope2Call;
    #[doc = "Container type for all input parameters for the `setLiquidityRate`function with signature `setLiquidityRate(uint256)` and selector `[58, 36, 74, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLiquidityRate", abi = "setLiquidityRate(uint256)")]
    pub struct SetLiquidityRateCall {
        pub liquidity_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStableBorrowRate`function with signature `setStableBorrowRate(uint256)` and selector `[206, 204, 237, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setStableBorrowRate", abi = "setStableBorrowRate(uint256)")]
    pub struct SetStableBorrowRateCall {
        pub stable_borrow_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setVariableBorrowRate`function with signature `setVariableBorrowRate(uint256)` and selector `[170, 22, 254, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setVariableBorrowRate", abi = "setVariableBorrowRate(uint256)")]
    pub struct SetVariableBorrowRateCall {
        pub variable_borrow_rate: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockReserveInterestRateStrategyCalls {
        AddressesProvider(AddressesProviderCall),
        OptimalUsageRatio(OptimalUsageRatioCall),
        CalculateInterestRates(CalculateInterestRatesCall),
        GetBaseVariableBorrowRate(GetBaseVariableBorrowRateCall),
        GetMaxVariableBorrowRate(GetMaxVariableBorrowRateCall),
        GetStableRateSlope1(GetStableRateSlope1Call),
        GetStableRateSlope2(GetStableRateSlope2Call),
        GetVariableRateSlope1(GetVariableRateSlope1Call),
        GetVariableRateSlope2(GetVariableRateSlope2Call),
        SetLiquidityRate(SetLiquidityRateCall),
        SetStableBorrowRate(SetStableBorrowRateCall),
        SetVariableBorrowRate(SetVariableBorrowRateCall),
    }
    impl ethers::core::abi::AbiDecode for MockReserveInterestRateStrategyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::AddressesProvider(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OptimalUsageRatioCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::OptimalUsageRatio(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CalculateInterestRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::CalculateInterestRates(decoded));
            }
            if let Ok(decoded) =
                <GetBaseVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    MockReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(decoded),
                );
            }
            if let Ok(decoded) =
                <GetMaxVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetStableRateSlope1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::GetStableRateSlope1(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetStableRateSlope2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::GetStableRateSlope2(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetVariableRateSlope1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::GetVariableRateSlope1(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetVariableRateSlope2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::GetVariableRateSlope2(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetLiquidityRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::SetLiquidityRate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::SetStableBorrowRate(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveInterestRateStrategyCalls::SetVariableBorrowRate(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockReserveInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockReserveInterestRateStrategyCalls::AddressesProvider(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::OptimalUsageRatio(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetStableRateSlope1(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetStableRateSlope2(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetVariableRateSlope1(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::GetVariableRateSlope2(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::SetLiquidityRate(element) => element.encode(),
                MockReserveInterestRateStrategyCalls::SetStableBorrowRate(element) => {
                    element.encode()
                }
                MockReserveInterestRateStrategyCalls::SetVariableBorrowRate(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for MockReserveInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockReserveInterestRateStrategyCalls::AddressesProvider(element) => element.fmt(f),
                MockReserveInterestRateStrategyCalls::OptimalUsageRatio(element) => element.fmt(f),
                MockReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetStableRateSlope1(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetStableRateSlope2(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetVariableRateSlope1(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::GetVariableRateSlope2(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::SetLiquidityRate(element) => element.fmt(f),
                MockReserveInterestRateStrategyCalls::SetStableBorrowRate(element) => {
                    element.fmt(f)
                }
                MockReserveInterestRateStrategyCalls::SetVariableBorrowRate(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: AddressesProviderCall) -> Self {
            MockReserveInterestRateStrategyCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<OptimalUsageRatioCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: OptimalUsageRatioCall) -> Self {
            MockReserveInterestRateStrategyCalls::OptimalUsageRatio(var)
        }
    }
    impl ::std::convert::From<CalculateInterestRatesCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: CalculateInterestRatesCall) -> Self {
            MockReserveInterestRateStrategyCalls::CalculateInterestRates(var)
        }
    }
    impl ::std::convert::From<GetBaseVariableBorrowRateCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetBaseVariableBorrowRateCall) -> Self {
            MockReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetMaxVariableBorrowRateCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetMaxVariableBorrowRateCall) -> Self {
            MockReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetStableRateSlope1Call> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetStableRateSlope1Call) -> Self {
            MockReserveInterestRateStrategyCalls::GetStableRateSlope1(var)
        }
    }
    impl ::std::convert::From<GetStableRateSlope2Call> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetStableRateSlope2Call) -> Self {
            MockReserveInterestRateStrategyCalls::GetStableRateSlope2(var)
        }
    }
    impl ::std::convert::From<GetVariableRateSlope1Call> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetVariableRateSlope1Call) -> Self {
            MockReserveInterestRateStrategyCalls::GetVariableRateSlope1(var)
        }
    }
    impl ::std::convert::From<GetVariableRateSlope2Call> for MockReserveInterestRateStrategyCalls {
        fn from(var: GetVariableRateSlope2Call) -> Self {
            MockReserveInterestRateStrategyCalls::GetVariableRateSlope2(var)
        }
    }
    impl ::std::convert::From<SetLiquidityRateCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: SetLiquidityRateCall) -> Self {
            MockReserveInterestRateStrategyCalls::SetLiquidityRate(var)
        }
    }
    impl ::std::convert::From<SetStableBorrowRateCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: SetStableBorrowRateCall) -> Self {
            MockReserveInterestRateStrategyCalls::SetStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<SetVariableBorrowRateCall> for MockReserveInterestRateStrategyCalls {
        fn from(var: SetVariableBorrowRateCall) -> Self {
            MockReserveInterestRateStrategyCalls::SetVariableBorrowRate(var)
        }
    }
}
