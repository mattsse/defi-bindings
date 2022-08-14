pub use mock_flash_loan_receiver::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_flash_loan_receiver {
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
    #[doc = "MockFlashLoanReceiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKFLASHLOANRECEIVER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_assets\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"_amounts\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"_premiums\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutedWithFail\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_assets\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"_amounts\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"_premiums\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutedWithSuccess\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"premiums\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountToApprove\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToApprove\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAmountToApprove\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"fail\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFailExecutionTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"flag\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSimulateEOA\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"simulateEOA\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKFLASHLOANRECEIVER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c060405234801561001057600080fd5b50604051610a57380380610a5783398101604081905261002f916100d8565b80806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663026b1d5f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ac91906100d8565b6001600160a01b031660a052506100fc9050565b6001600160a01b03811681146100d557600080fd5b50565b6000602082840312156100ea57600080fd5b81516100f5816100c0565b9392505050565b60805160a05161093061012760003960008181610121015261040f01526000609201526109306000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80637535d2461161005b5780637535d2461461011c578063920f5c8414610143578063bf443f8514610156578063e9a6a25b1461016957600080fd5b80630542975c1461008d578063388f70f1146100d15780634444f331146100f45780635e76bba31461010b575b600080fd5b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100f26100df3660046104fc565b6000805460ff1916911515919091179055565b005b60025460ff165b60405190151581526020016100c8565b6001546040519081526020016100c8565b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6100fb610151366004610682565b61018a565b6100f261016436600461079c565b600155565b6100f26101773660046104fc565b6002805460ff1916911515919091179055565b6000805460ff16156101de577f9972b212e52913783072b960dd41527ae8b6e609d017b64039758dda0ce412788686866040516101c9939291906107f0565b60405180910390a15060025460ff16156104e2565b60005b86518110156104a25760008782815181106101fe576101fe610865565b6020026020010151905087828151811061021a5761021a610865565b60209081029190910101516040516370a0823160e01b81523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa15801561026a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061028e919061087b565b8783815181106102a0576102a0610865565b602002602001015111156102fa5760405162461bcd60e51b815260206004820181905260248201527f496e76616c69642062616c616e636520666f722074686520636f6e7472616374604482015260640160405180910390fd5b60006001546000141561034a5786838151811061031957610319610865565b602002602001015188848151811061033357610333610865565b602002602001015161034591906108aa565b61034e565b6001545b9050816001600160a01b031663a0712d6888858151811061037157610371610865565b60200260200101516040518263ffffffff1660e01b815260040161039791815260200190565b6020604051808303816000875af11580156103b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103da91906108c2565b508883815181106103ed576103ed610865565b602090810291909101015160405163095ea7b360e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081166004830152602482018490529091169063095ea7b3906044016020604051808303816000875af1158015610468573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061048c91906108c2565b505050808061049a906108df565b9150506101e1565b507fbd6b6bfac59612765a81cc4fdee74ab4859671fa14a562056f9eea438735a78a8686866040516104d6939291906107f0565b60405180910390a15060015b95945050505050565b80151581146104f957600080fd5b50565b60006020828403121561050e57600080fd5b8135610519816104eb565b9392505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561055f5761055f610520565b604052919050565b600067ffffffffffffffff82111561058157610581610520565b5060051b60200190565b80356001600160a01b03811681146105a257600080fd5b919050565b600082601f8301126105b857600080fd5b813560206105cd6105c883610567565b610536565b82815260059290921b840181019181810190868411156105ec57600080fd5b8286015b8481101561060757803583529183019183016105f0565b509695505050505050565b600082601f83011261062357600080fd5b813567ffffffffffffffff81111561063d5761063d610520565b610650601f8201601f1916602001610536565b81815284602083860101111561066557600080fd5b816020850160208301376000918101602001919091529392505050565b600080600080600060a0868803121561069a57600080fd5b853567ffffffffffffffff808211156106b257600080fd5b818801915088601f8301126106c657600080fd5b813560206106d66105c883610567565b82815260059290921b8401810191818101908c8411156106f557600080fd5b948201945b8386101561071a5761070b8661058b565b825294820194908201906106fa565b9950508901359250508082111561073057600080fd5b61073c89838a016105a7565b9550604088013591508082111561075257600080fd5b61075e89838a016105a7565b945061076c6060890161058b565b9350608088013591508082111561078257600080fd5b5061078f88828901610612565b9150509295509295909350565b6000602082840312156107ae57600080fd5b5035919050565b600081518084526020808501945080840160005b838110156107e5578151875295820195908201906001016107c9565b509495945050505050565b606080825284519082018190526000906020906080840190828801845b828110156108325781516001600160a01b03168452928401929084019060010161080d565b5050508381038285015261084681876107b5565b915050828103604084015261085b81856107b5565b9695505050505050565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561088d57600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b600082198211156108bd576108bd610894565b500190565b6000602082840312156108d457600080fd5b8151610519816104eb565b60006000198214156108f3576108f3610894565b506001019056fea26469706673582212206d9e9aacf566dbccea99948b3d9eb8d68b9d8bb597e2a658bc827e5f3591379364736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockFlashLoanReceiver<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockFlashLoanReceiver<M> {
        fn clone(&self) -> Self {
            MockFlashLoanReceiver(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockFlashLoanReceiver<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockFlashLoanReceiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockFlashLoanReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockFlashLoanReceiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKFLASHLOANRECEIVER_ABI.clone(),
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MOCKFLASHLOANRECEIVER_ABI.clone(),
                MOCKFLASHLOANRECEIVER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOperation` (0x920f5c84) function"]
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            premiums: ::std::vec::Vec<ethers::core::types::U256>,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 15, 92, 132], (assets, amounts, premiums, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountToApprove` (0x5e76bba3) function"]
        pub fn get_amount_to_approve(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 118, 187, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAmountToApprove` (0xbf443f85) function"]
        pub fn set_amount_to_approve(
            &self,
            amount_to_approve: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 68, 63, 133], amount_to_approve)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFailExecutionTransfer` (0x388f70f1) function"]
        pub fn set_fail_execution_transfer(
            &self,
            fail: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 143, 112, 241], fail)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSimulateEOA` (0xe9a6a25b) function"]
        pub fn set_simulate_eoa(
            &self,
            flag: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 166, 162, 91], flag)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `simulateEOA` (0x4444f331) function"]
        pub fn simulate_eoa(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 68, 243, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ExecutedWithFail` event"]
        pub fn executed_with_fail_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutedWithFailFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutedWithSuccess` event"]
        pub fn executed_with_success_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutedWithSuccessFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MockFlashLoanReceiverEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockFlashLoanReceiver<M>
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
        name = "ExecutedWithFail",
        abi = "ExecutedWithFail(address[],uint256[],uint256[])"
    )]
    pub struct ExecutedWithFailFilter {
        pub assets: Vec<ethers::core::types::Address>,
        pub amounts: Vec<ethers::core::types::U256>,
        pub premiums: Vec<ethers::core::types::U256>,
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
        name = "ExecutedWithSuccess",
        abi = "ExecutedWithSuccess(address[],uint256[],uint256[])"
    )]
    pub struct ExecutedWithSuccessFilter {
        pub assets: Vec<ethers::core::types::Address>,
        pub amounts: Vec<ethers::core::types::U256>,
        pub premiums: Vec<ethers::core::types::U256>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockFlashLoanReceiverEvents {
        ExecutedWithFailFilter(ExecutedWithFailFilter),
        ExecutedWithSuccessFilter(ExecutedWithSuccessFilter),
    }
    impl ethers::contract::EthLogDecode for MockFlashLoanReceiverEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ExecutedWithFailFilter::decode_log(log) {
                return Ok(MockFlashLoanReceiverEvents::ExecutedWithFailFilter(decoded));
            }
            if let Ok(decoded) = ExecutedWithSuccessFilter::decode_log(log) {
                return Ok(MockFlashLoanReceiverEvents::ExecutedWithSuccessFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockFlashLoanReceiverEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFlashLoanReceiverEvents::ExecutedWithFailFilter(element) => element.fmt(f),
                MockFlashLoanReceiverEvents::ExecutedWithSuccessFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
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
    #[doc = "Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    #[doc = "Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `[146, 15, 92, 132]`"]
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
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<ethers::core::types::U256>,
        pub p3: ethers::core::types::Address,
        pub p4: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getAmountToApprove` function with signature `getAmountToApprove()` and selector `[94, 118, 187, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountToApprove", abi = "getAmountToApprove()")]
    pub struct GetAmountToApproveCall;
    #[doc = "Container type for all input parameters for the `setAmountToApprove` function with signature `setAmountToApprove(uint256)` and selector `[191, 68, 63, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAmountToApprove", abi = "setAmountToApprove(uint256)")]
    pub struct SetAmountToApproveCall {
        pub amount_to_approve: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFailExecutionTransfer` function with signature `setFailExecutionTransfer(bool)` and selector `[56, 143, 112, 241]`"]
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
        name = "setFailExecutionTransfer",
        abi = "setFailExecutionTransfer(bool)"
    )]
    pub struct SetFailExecutionTransferCall {
        pub fail: bool,
    }
    #[doc = "Container type for all input parameters for the `setSimulateEOA` function with signature `setSimulateEOA(bool)` and selector `[233, 166, 162, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSimulateEOA", abi = "setSimulateEOA(bool)")]
    pub struct SetSimulateEOACall {
        pub flag: bool,
    }
    #[doc = "Container type for all input parameters for the `simulateEOA` function with signature `simulateEOA()` and selector `[68, 68, 243, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "simulateEOA", abi = "simulateEOA()")]
    pub struct SimulateEOACall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockFlashLoanReceiverCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        GetAmountToApprove(GetAmountToApproveCall),
        SetAmountToApprove(SetAmountToApproveCall),
        SetFailExecutionTransfer(SetFailExecutionTransferCall),
        SetSimulateEOA(SetSimulateEOACall),
        SimulateEOA(SimulateEOACall),
    }
    impl ethers::core::abi::AbiDecode for MockFlashLoanReceiverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockFlashLoanReceiverCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::ExecuteOperation(decoded));
            }
            if let Ok(decoded) =
                <GetAmountToApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::GetAmountToApprove(decoded));
            }
            if let Ok(decoded) =
                <SetAmountToApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::SetAmountToApprove(decoded));
            }
            if let Ok(decoded) =
                <SetFailExecutionTransferCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockFlashLoanReceiverCalls::SetFailExecutionTransfer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetSimulateEOACall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::SetSimulateEOA(decoded));
            }
            if let Ok(decoded) =
                <SimulateEOACall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanReceiverCalls::SimulateEOA(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockFlashLoanReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockFlashLoanReceiverCalls::AddressesProvider(element) => element.encode(),
                MockFlashLoanReceiverCalls::Pool(element) => element.encode(),
                MockFlashLoanReceiverCalls::ExecuteOperation(element) => element.encode(),
                MockFlashLoanReceiverCalls::GetAmountToApprove(element) => element.encode(),
                MockFlashLoanReceiverCalls::SetAmountToApprove(element) => element.encode(),
                MockFlashLoanReceiverCalls::SetFailExecutionTransfer(element) => element.encode(),
                MockFlashLoanReceiverCalls::SetSimulateEOA(element) => element.encode(),
                MockFlashLoanReceiverCalls::SimulateEOA(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockFlashLoanReceiverCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFlashLoanReceiverCalls::AddressesProvider(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::Pool(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::ExecuteOperation(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::GetAmountToApprove(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::SetAmountToApprove(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::SetFailExecutionTransfer(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::SetSimulateEOA(element) => element.fmt(f),
                MockFlashLoanReceiverCalls::SimulateEOA(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for MockFlashLoanReceiverCalls {
        fn from(var: AddressesProviderCall) -> Self {
            MockFlashLoanReceiverCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<PoolCall> for MockFlashLoanReceiverCalls {
        fn from(var: PoolCall) -> Self {
            MockFlashLoanReceiverCalls::Pool(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for MockFlashLoanReceiverCalls {
        fn from(var: ExecuteOperationCall) -> Self {
            MockFlashLoanReceiverCalls::ExecuteOperation(var)
        }
    }
    impl ::std::convert::From<GetAmountToApproveCall> for MockFlashLoanReceiverCalls {
        fn from(var: GetAmountToApproveCall) -> Self {
            MockFlashLoanReceiverCalls::GetAmountToApprove(var)
        }
    }
    impl ::std::convert::From<SetAmountToApproveCall> for MockFlashLoanReceiverCalls {
        fn from(var: SetAmountToApproveCall) -> Self {
            MockFlashLoanReceiverCalls::SetAmountToApprove(var)
        }
    }
    impl ::std::convert::From<SetFailExecutionTransferCall> for MockFlashLoanReceiverCalls {
        fn from(var: SetFailExecutionTransferCall) -> Self {
            MockFlashLoanReceiverCalls::SetFailExecutionTransfer(var)
        }
    }
    impl ::std::convert::From<SetSimulateEOACall> for MockFlashLoanReceiverCalls {
        fn from(var: SetSimulateEOACall) -> Self {
            MockFlashLoanReceiverCalls::SetSimulateEOA(var)
        }
    }
    impl ::std::convert::From<SimulateEOACall> for MockFlashLoanReceiverCalls {
        fn from(var: SimulateEOACall) -> Self {
            MockFlashLoanReceiverCalls::SimulateEOA(var)
        }
    }
    #[doc = "Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressesProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `[146, 15, 92, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExecuteOperationReturn(pub bool);
    #[doc = "Container type for all return fields from the `getAmountToApprove` function with signature `getAmountToApprove()` and selector `[94, 118, 187, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAmountToApproveReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `simulateEOA` function with signature `simulateEOA()` and selector `[68, 68, 243, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SimulateEOAReturn(pub bool);
}
