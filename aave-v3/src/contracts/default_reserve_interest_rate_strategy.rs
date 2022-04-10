pub use defaultreserveinterestratestrategy_mod::*;
#[allow(clippy::too_many_arguments)]
mod defaultreserveinterestratestrategy_mod {
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
    #[doc = "DefaultReserveInterestRateStrategy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DEFAULTRESERVEINTERESTRATESTRATEGY_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"optimalUsageRatio\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseVariableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableRateSlope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"variableRateSlope2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableRateSlope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableRateSlope2\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseStableRateOffset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"stableRateExcessOffset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"optimalStableToTotalDebtRatio\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXCESS_USAGE_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"OPTIMAL_USAGE_RATIO\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct DataTypes.CalculateInterestRatesParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"unbacked\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityAdded\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidityTaken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalStableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalVariableDebt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"averageStableBorrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aToken\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateInterestRates\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBaseStableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBaseVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMaxVariableBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateExcessOffset\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateSlope1\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateSlope2\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVariableRateSlope1\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVariableRateSlope2\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
    });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEFAULTRESERVEINTERESTRATESTRATEGY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x61020060405234801561001157600080fd5b50604051610e68380380610e6883398101604081905261003091610144565b886b033b2e3c9fd0803ce8000000101560405180604001604052806002815260200161383360f01b815250906100825760405162461bcd60e51b815260040161007991906101cf565b60405180910390fd5b50806b033b2e3c9fd0803ce80000001015604051806040016040528060028152602001610e0d60f21b815250906100cc5760405162461bcd60e51b815260040161007991906101cf565b5060808990526100e8896b033b2e3c9fd0803ce8000000610224565b60c05260a0819052610106816b033b2e3c9fd0803ce8000000610224565b60e052506001600160a01b0390981661010052610120959095526101409390935261016091909152610180526101a0526101c052506101e052610249565b6000806000806000806000806000806101408b8d03121561016457600080fd5b8a516001600160a01b038116811461017b57600080fd5b809a505060208b0151985060408b0151975060608b0151965060808b0151955060a08b0151945060c08b0151935060e08b015192506101008b015191506101208b015190509295989b9194979a5092959850565b600060208083528351808285015260005b818110156101fc578581018301518582016040015282016101e0565b8181111561020e576000604083870101525b50601f01601f1916929092016040019392505050565b60008282101561024457634e487b7160e01b600052601160045260246000fd5b500390565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516101e051610b03610365600039600081816102640152610813015260006108b601526000818161016501526105de01526000818161028a0152818161060901526106de0152600081816102b0015281816102ff015261064601526000818161013501528181610323015281816106710152818161075001526108d701526000818161018b0152818161034401526103eb0152600060f40152600081816102d901526107bd01526000818161023801526105820152600081816101db0152818161078c01526107de0152600081816101b401528181610551015281816105a3015281816106b5015261072a0152610b036000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c8063a58987091161008c578063bc62690811610066578063bc62690814610262578063d5cd739114610288578063f4202409146102ae578063fe5fd698146102d457600080fd5b8063a589870914610205578063a9c622f814610233578063acd786861461025a57600080fd5b806334762ca5116100c857806334762ca51461018957806354c365c6146101af5780636fb92589146101d657806380031e37146101fd57600080fd5b80630542975c146100ef5780630b3429a21461013357806314e32da414610163575b600080fd5b6101167f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b7f00000000000000000000000000000000000000000000000000000000000000005b60405190815260200161012a565b7f0000000000000000000000000000000000000000000000000000000000000000610155565b7f0000000000000000000000000000000000000000000000000000000000000000610155565b6101557f000000000000000000000000000000000000000000000000000000000000000081565b6101557f000000000000000000000000000000000000000000000000000000000000000081565b6101556102fb565b610218610213366004610a26565b610377565b6040805193845260208401929092529082015260600161012a565b6101557f000000000000000000000000000000000000000000000000000000000000000081565b6101556108af565b7f0000000000000000000000000000000000000000000000000000000000000000610155565b7f0000000000000000000000000000000000000000000000000000000000000000610155565b7f0000000000000000000000000000000000000000000000000000000000000000610155565b6101557f000000000000000000000000000000000000000000000000000000000000000081565b60007f00000000000000000000000000000000000000000000000000000000000000006103687f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000610a55565b6103729190610a55565b905090565b60008060006103cb6040518061012001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081525090565b6103dd60808601356060870135610a55565b6020820152600060808201527f000000000000000000000000000000000000000000000000000000000000000060408201526104176108af565b606082015260208101511561054f57602081015161043a906060870135906108fb565b8160e0018181525050846040013585602001358660e00160208101906104609190610a6d565b6001600160a01b03166370a082316104806101208a016101008b01610a6d565b6040516001600160e01b031960e084901b1681526001600160a01b039091166004820152602401602060405180830381865afa1580156104c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104e89190610a9d565b6104f29190610a55565b6104fc9190610ab6565b808252602082015161050d91610a55565b61010082018190526020820151610523916108fb565b60a08201526101008101516105499061053e90873590610a55565b6020830151906108fb565b60c08201525b7f00000000000000000000000000000000000000000000000000000000000000008160a0015111156106b05760006105d77f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008460a001516105d19190610ab6565b906108fb565b90506106037f00000000000000000000000000000000000000000000000000000000000000008261093a565b61062d907f0000000000000000000000000000000000000000000000000000000000000000610a55565b8260600181815161063e9190610a55565b90525061066b7f00000000000000000000000000000000000000000000000000000000000000008261093a565b610695907f0000000000000000000000000000000000000000000000000000000000000000610a55565b826040018181516106a69190610a55565b90525061078a9050565b61070b7f00000000000000000000000000000000000000000000000000000000000000006105d18360a001517f000000000000000000000000000000000000000000000000000000000000000061093a90919063ffffffff16565b8160600181815161071c9190610a55565b90525060a0810151610775907f0000000000000000000000000000000000000000000000000000000000000000906105d1907f00000000000000000000000000000000000000000000000000000000000000009061093a565b816040018181516107869190610a55565b9052505b7f00000000000000000000000000000000000000000000000000000000000000008160e00151111561084e57600061080c7f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000008460e001516105d19190610ab6565b90506108387f00000000000000000000000000000000000000000000000000000000000000008261093a565b826060018181516108499190610a55565b905250505b61089161086160c0870135612710610ab6565b61088b8360c0015161088589606001358a6080013587604001518c60a0013561097e565b9061093a565b906109e5565b60808201819052606082015160409092015190969195509350915050565b60006103727f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000610a55565b600081156b033b2e3c9fd0803ce80000006002840419048411171561091f57600080fd5b506b033b2e3c9fd0803ce80000009190910260028204010490565b600081156b019d971e4fe8401e74000000198390048411151761095c57600080fd5b506b033b2e3c9fd0803ce800000091026b019d971e4fe8401e74000000010490565b60008061098b8587610a55565b90508061099c5760009150506109dd565b60006109ab8561088588610a0b565b905060006109bc856108858a610a0b565b905060006109d66109cc85610a0b565b6105d18486610a55565b9450505050505b949350505050565b6000811561138819839004841115176109fd57600080fd5b506127109102611388010490565b633b9aca008181029081048214610a2157600080fd5b919050565b60006101208284031215610a3957600080fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115610a6857610a68610a3f565b500190565b600060208284031215610a7f57600080fd5b81356001600160a01b0381168114610a9657600080fd5b9392505050565b600060208284031215610aaf57600080fd5b5051919050565b600082821015610ac857610ac8610a3f565b50039056fea2646970667358221220a74a27b9d8b6a6d7054b532399c673256b44bf75cf4374e4a9e0f715f4e3f91d64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct DefaultReserveInterestRateStrategy<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for DefaultReserveInterestRateStrategy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DefaultReserveInterestRateStrategy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DefaultReserveInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> DefaultReserveInterestRateStrategy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                DEFAULTRESERVEINTERESTRATESTRATEGY_ABI.clone(),
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
                DEFAULTRESERVEINTERESTRATESTRATEGY_ABI.clone(),
                DEFAULTRESERVEINTERESTRATESTRATEGY_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` (0xfe5fd698) function"]
        pub fn max_excess_stable_to_total_debt_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([254, 95, 214, 152], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_EXCESS_USAGE_RATIO` (0xa9c622f8) function"]
        pub fn max_excess_usage_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([169, 198, 34, 248], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` (0x6fb92589) function"]
        pub fn optimal_stable_to_total_debt_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([111, 185, 37, 137], ())
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
            params: CalculateInterestRatesParams,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBaseStableBorrowRate` (0xacd78686) function"]
        pub fn get_base_stable_borrow_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([172, 215, 134, 134], ())
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
        #[doc = "Calls the contract's `getStableRateExcessOffset` (0xbc626908) function"]
        pub fn get_stable_rate_excess_offset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 98, 105, 8], ())
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for DefaultReserveInterestRateStrategy<M>
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
    #[doc = "Container type for all input parameters for the `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO`function with signature `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `[254, 95, 214, 152]`"]
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
        name = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct MaxExcessStableToTotalDebtRatioCall;
    #[doc = "Container type for all input parameters for the `MAX_EXCESS_USAGE_RATIO`function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `[169, 198, 34, 248]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_EXCESS_USAGE_RATIO", abi = "MAX_EXCESS_USAGE_RATIO()")]
    pub struct MaxExcessUsageRatioCall;
    #[doc = "Container type for all input parameters for the `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO`function with signature `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `[111, 185, 37, 137]`"]
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
        name = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct OptimalStableToTotalDebtRatioCall;
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
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    #[doc = "Container type for all input parameters for the `getBaseStableBorrowRate`function with signature `getBaseStableBorrowRate()` and selector `[172, 215, 134, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBaseStableBorrowRate", abi = "getBaseStableBorrowRate()")]
    pub struct GetBaseStableBorrowRateCall;
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
    #[doc = "Container type for all input parameters for the `getStableRateExcessOffset`function with signature `getStableRateExcessOffset()` and selector `[188, 98, 105, 8]`"]
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
        name = "getStableRateExcessOffset",
        abi = "getStableRateExcessOffset()"
    )]
    pub struct GetStableRateExcessOffsetCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DefaultReserveInterestRateStrategyCalls {
        AddressesProvider(AddressesProviderCall),
        MaxExcessStableToTotalDebtRatio(MaxExcessStableToTotalDebtRatioCall),
        MaxExcessUsageRatio(MaxExcessUsageRatioCall),
        OptimalStableToTotalDebtRatio(OptimalStableToTotalDebtRatioCall),
        OptimalUsageRatio(OptimalUsageRatioCall),
        CalculateInterestRates(CalculateInterestRatesCall),
        GetBaseStableBorrowRate(GetBaseStableBorrowRateCall),
        GetBaseVariableBorrowRate(GetBaseVariableBorrowRateCall),
        GetMaxVariableBorrowRate(GetMaxVariableBorrowRateCall),
        GetStableRateExcessOffset(GetStableRateExcessOffsetCall),
        GetStableRateSlope1(GetStableRateSlope1Call),
        GetStableRateSlope2(GetStableRateSlope2Call),
        GetVariableRateSlope1(GetVariableRateSlope1Call),
        GetVariableRateSlope2(GetVariableRateSlope2Call),
    }
    impl ethers::core::abi::AbiDecode for DefaultReserveInterestRateStrategyCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::AddressesProvider(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MaxExcessStableToTotalDebtRatioCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::MaxExcessStableToTotalDebtRatio(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <MaxExcessUsageRatioCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::MaxExcessUsageRatio(decoded));
            }
            if let Ok(decoded) =
                <OptimalStableToTotalDebtRatioCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::OptimalStableToTotalDebtRatio(decoded),
                );
            }
            if let Ok(decoded) =
                <OptimalUsageRatioCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::OptimalUsageRatio(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CalculateInterestRatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::CalculateInterestRates(decoded),
                );
            }
            if let Ok(decoded) =
                <GetBaseStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::GetBaseStableBorrowRate(decoded),
                );
            }
            if let Ok(decoded) =
                <GetBaseVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(decoded),
                );
            }
            if let Ok(decoded) =
                <GetMaxVariableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(decoded),
                );
            }
            if let Ok(decoded) =
                <GetStableRateExcessOffsetCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DefaultReserveInterestRateStrategyCalls::GetStableRateExcessOffset(decoded),
                );
            }
            if let Ok(decoded) =
                <GetStableRateSlope1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::GetStableRateSlope1(decoded));
            }
            if let Ok(decoded) =
                <GetStableRateSlope2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::GetStableRateSlope2(decoded));
            }
            if let Ok(decoded) =
                <GetVariableRateSlope1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope1(decoded));
            }
            if let Ok(decoded) =
                <GetVariableRateSlope2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope2(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DefaultReserveInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DefaultReserveInterestRateStrategyCalls::AddressesProvider(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::MaxExcessStableToTotalDebtRatio(
                    element,
                ) => element.encode(),
                DefaultReserveInterestRateStrategyCalls::MaxExcessUsageRatio(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::OptimalStableToTotalDebtRatio(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::OptimalUsageRatio(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetBaseStableBorrowRate(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateExcessOffset(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateSlope1(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateSlope2(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope1(element) => {
                    element.encode()
                }
                DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope2(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for DefaultReserveInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DefaultReserveInterestRateStrategyCalls::AddressesProvider(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::MaxExcessStableToTotalDebtRatio(
                    element,
                ) => element.fmt(f),
                DefaultReserveInterestRateStrategyCalls::MaxExcessUsageRatio(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::OptimalStableToTotalDebtRatio(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::OptimalUsageRatio(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::CalculateInterestRates(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetBaseStableBorrowRate(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateExcessOffset(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateSlope1(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetStableRateSlope2(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope1(element) => {
                    element.fmt(f)
                }
                DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope2(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: AddressesProviderCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<MaxExcessStableToTotalDebtRatioCall>
        for DefaultReserveInterestRateStrategyCalls
    {
        fn from(var: MaxExcessStableToTotalDebtRatioCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::MaxExcessStableToTotalDebtRatio(var)
        }
    }
    impl ::std::convert::From<MaxExcessUsageRatioCall> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: MaxExcessUsageRatioCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::MaxExcessUsageRatio(var)
        }
    }
    impl ::std::convert::From<OptimalStableToTotalDebtRatioCall>
        for DefaultReserveInterestRateStrategyCalls
    {
        fn from(var: OptimalStableToTotalDebtRatioCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::OptimalStableToTotalDebtRatio(var)
        }
    }
    impl ::std::convert::From<OptimalUsageRatioCall> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: OptimalUsageRatioCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::OptimalUsageRatio(var)
        }
    }
    impl ::std::convert::From<CalculateInterestRatesCall> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: CalculateInterestRatesCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::CalculateInterestRates(var)
        }
    }
    impl ::std::convert::From<GetBaseStableBorrowRateCall> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: GetBaseStableBorrowRateCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetBaseStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetBaseVariableBorrowRateCall>
        for DefaultReserveInterestRateStrategyCalls
    {
        fn from(var: GetBaseVariableBorrowRateCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetBaseVariableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetMaxVariableBorrowRateCall>
        for DefaultReserveInterestRateStrategyCalls
    {
        fn from(var: GetMaxVariableBorrowRateCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetMaxVariableBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetStableRateExcessOffsetCall>
        for DefaultReserveInterestRateStrategyCalls
    {
        fn from(var: GetStableRateExcessOffsetCall) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetStableRateExcessOffset(var)
        }
    }
    impl ::std::convert::From<GetStableRateSlope1Call> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: GetStableRateSlope1Call) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetStableRateSlope1(var)
        }
    }
    impl ::std::convert::From<GetStableRateSlope2Call> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: GetStableRateSlope2Call) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetStableRateSlope2(var)
        }
    }
    impl ::std::convert::From<GetVariableRateSlope1Call> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: GetVariableRateSlope1Call) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope1(var)
        }
    }
    impl ::std::convert::From<GetVariableRateSlope2Call> for DefaultReserveInterestRateStrategyCalls {
        fn from(var: GetVariableRateSlope2Call) -> Self {
            DefaultReserveInterestRateStrategyCalls::GetVariableRateSlope2(var)
        }
    }
}
