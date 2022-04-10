pub use mockflashloansimplereceiver_mod::*;
#[allow(clippy::too_many_arguments)]
mod mockflashloansimplereceiver_mod {
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
    #[doc = "MockFlashLoanSimpleReceiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKFLASHLOANSIMPLERECEIVER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutedWithFail\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutedWithSuccess\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountToApprove\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToApprove\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAmountToApprove\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"fail\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFailExecutionTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"flag\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSimulateEOA\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"simulateEOA\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKFLASHLOANSIMPLERECEIVER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60c060405234801561001057600080fd5b5060405161073a38038061073a83398101604081905261002f916100d8565b80806001600160a01b03166080816001600160a01b031681525050806001600160a01b031663026b1d5f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610088573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ac91906100d8565b6001600160a01b031660a052506100fc9050565b6001600160a01b03811681146100d557600080fd5b50565b6000602082840312156100ea57600080fd5b81516100f5816100c0565b9392505050565b60805160a05161061361012760003960008181610138015261035401526000609201526106136000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80635e76bba31161005b5780635e76bba3146101225780637535d24614610133578063bf443f851461015a578063e9a6a25b1461016d57600080fd5b80630542975c1461008d5780631b11d0ff146100d1578063388f70f1146100f45780634444f33114610117575b600080fd5b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100e46100df366004610473565b61018e565b60405190151581526020016100c8565b61011561010236600461056a565b6000805460ff1916911515919091179055565b005b60025460ff166100e4565b6001546040519081526020016100c8565b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b61011561016836600461058e565b600155565b61011561017b36600461056a565b6002805460ff1916911515919091179055565b6000805460ff16156101f157604080516001600160a01b0388168152602081018790529081018590527f816f6a6bc084e1996be1a831afa1af30763d0501b6b43b9e1922a11f347366d79060600160405180910390a15060025460ff1615610422565b6040516370a0823160e01b815230600482015286906001600160a01b038216906370a0823190602401602060405180830381865afa158015610237573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061025b91906105a7565b8611156102ae5760405162461bcd60e51b815260206004820181905260248201527f496e76616c69642062616c616e636520666f722074686520636f6e7472616374604482015260640160405180910390fd5b6000600154600014156102ca576102c5878761042b565b6102ce565b6001545b60405163140e25ad60e31b8152600481018890529091506001600160a01b0383169063a0712d68906024016020604051808303816000875af1158015610318573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033c91906105c0565b5060405163095ea7b360e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811660048301526024820183905289169063095ea7b3906044016020604051808303816000875af11580156103ac573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103d091906105c0565b50604080516001600160a01b038a168152602081018990529081018790527f7d94e9d0c906b8d7b2b52a581b9e9ba728aa6f8cd8532bd87243d193f47401be9060600160405180910390a16001925050505b95945050505050565b8082018281101561043b57600080fd5b92915050565b80356001600160a01b038116811461045857600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b600080600080600060a0868803121561048b57600080fd5b61049486610441565b945060208601359350604086013592506104b060608701610441565b9150608086013567ffffffffffffffff808211156104cd57600080fd5b818801915088601f8301126104e157600080fd5b8135818111156104f3576104f361045d565b604051601f8201601f19908116603f0116810190838211818310171561051b5761051b61045d565b816040528281528b602084870101111561053457600080fd5b8260208601602083013760006020848301015280955050505050509295509295909350565b801515811461056757600080fd5b50565b60006020828403121561057c57600080fd5b813561058781610559565b9392505050565b6000602082840312156105a057600080fd5b5035919050565b6000602082840312156105b957600080fd5b5051919050565b6000602082840312156105d257600080fd5b81516105878161055956fea26469706673582212205aca82f4640e610993905cdfee459201a55643293040d170ad8252571b614a1b64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct MockFlashLoanSimpleReceiver<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MockFlashLoanSimpleReceiver<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockFlashLoanSimpleReceiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockFlashLoanSimpleReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MockFlashLoanSimpleReceiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKFLASHLOANSIMPLERECEIVER_ABI.clone(),
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
                MOCKFLASHLOANSIMPLERECEIVER_ABI.clone(),
                MOCKFLASHLOANSIMPLERECEIVER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `executeOperation` (0x1b11d0ff) function"]
        pub fn execute_operation(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            premium: ethers::core::types::U256,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 17, 208, 255], (asset, amount, premium, p3, p4))
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
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, MockFlashLoanSimpleReceiverEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockFlashLoanSimpleReceiver<M>
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
        abi = "ExecutedWithFail(address,uint256,uint256)"
    )]
    pub struct ExecutedWithFailFilter {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
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
        abi = "ExecutedWithSuccess(address,uint256,uint256)"
    )]
    pub struct ExecutedWithSuccessFilter {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockFlashLoanSimpleReceiverEvents {
        ExecutedWithFailFilter(ExecutedWithFailFilter),
        ExecutedWithSuccessFilter(ExecutedWithSuccessFilter),
    }
    impl ethers::contract::EthLogDecode for MockFlashLoanSimpleReceiverEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ExecutedWithFailFilter::decode_log(log) {
                return Ok(MockFlashLoanSimpleReceiverEvents::ExecutedWithFailFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ExecutedWithSuccessFilter::decode_log(log) {
                return Ok(MockFlashLoanSimpleReceiverEvents::ExecutedWithSuccessFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockFlashLoanSimpleReceiverEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFlashLoanSimpleReceiverEvents::ExecutedWithFailFilter(element) => {
                    element.fmt(f)
                }
                MockFlashLoanSimpleReceiverEvents::ExecutedWithSuccessFilter(element) => {
                    element.fmt(f)
                }
            }
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
    #[doc = "Container type for all input parameters for the `POOL`function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
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
    #[doc = "Container type for all input parameters for the `executeOperation`function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `[27, 17, 208, 255]`"]
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
        abi = "executeOperation(address,uint256,uint256,address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
        pub p3: ethers::core::types::Address,
        pub p4: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getAmountToApprove`function with signature `getAmountToApprove()` and selector `[94, 118, 187, 163]`"]
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
    #[doc = "Container type for all input parameters for the `setAmountToApprove`function with signature `setAmountToApprove(uint256)` and selector `[191, 68, 63, 133]`"]
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
    #[doc = "Container type for all input parameters for the `setFailExecutionTransfer`function with signature `setFailExecutionTransfer(bool)` and selector `[56, 143, 112, 241]`"]
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
    #[doc = "Container type for all input parameters for the `setSimulateEOA`function with signature `setSimulateEOA(bool)` and selector `[233, 166, 162, 91]`"]
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
    #[doc = "Container type for all input parameters for the `simulateEOA`function with signature `simulateEOA()` and selector `[68, 68, 243, 49]`"]
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
    pub enum MockFlashLoanSimpleReceiverCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        GetAmountToApprove(GetAmountToApproveCall),
        SetAmountToApprove(SetAmountToApproveCall),
        SetFailExecutionTransfer(SetFailExecutionTransferCall),
        SetSimulateEOA(SetSimulateEOACall),
        SimulateEOA(SimulateEOACall),
    }
    impl ethers::core::abi::AbiDecode for MockFlashLoanSimpleReceiverCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockFlashLoanSimpleReceiverCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::ExecuteOperation(decoded));
            }
            if let Ok(decoded) =
                <GetAmountToApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::GetAmountToApprove(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetAmountToApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::SetAmountToApprove(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetFailExecutionTransferCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::SetFailExecutionTransfer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetSimulateEOACall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::SetSimulateEOA(decoded));
            }
            if let Ok(decoded) =
                <SimulateEOACall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockFlashLoanSimpleReceiverCalls::SimulateEOA(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockFlashLoanSimpleReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockFlashLoanSimpleReceiverCalls::AddressesProvider(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::Pool(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::ExecuteOperation(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::GetAmountToApprove(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::SetAmountToApprove(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::SetFailExecutionTransfer(element) => {
                    element.encode()
                }
                MockFlashLoanSimpleReceiverCalls::SetSimulateEOA(element) => element.encode(),
                MockFlashLoanSimpleReceiverCalls::SimulateEOA(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockFlashLoanSimpleReceiverCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockFlashLoanSimpleReceiverCalls::AddressesProvider(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::Pool(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::ExecuteOperation(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::GetAmountToApprove(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::SetAmountToApprove(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::SetFailExecutionTransfer(element) => {
                    element.fmt(f)
                }
                MockFlashLoanSimpleReceiverCalls::SetSimulateEOA(element) => element.fmt(f),
                MockFlashLoanSimpleReceiverCalls::SimulateEOA(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: AddressesProviderCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<PoolCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: PoolCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::Pool(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: ExecuteOperationCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::ExecuteOperation(var)
        }
    }
    impl ::std::convert::From<GetAmountToApproveCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: GetAmountToApproveCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::GetAmountToApprove(var)
        }
    }
    impl ::std::convert::From<SetAmountToApproveCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: SetAmountToApproveCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::SetAmountToApprove(var)
        }
    }
    impl ::std::convert::From<SetFailExecutionTransferCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: SetFailExecutionTransferCall) -> Self {
            MockFlashLoanSimpleReceiverCalls::SetFailExecutionTransfer(var)
        }
    }
    impl ::std::convert::From<SetSimulateEOACall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: SetSimulateEOACall) -> Self {
            MockFlashLoanSimpleReceiverCalls::SetSimulateEOA(var)
        }
    }
    impl ::std::convert::From<SimulateEOACall> for MockFlashLoanSimpleReceiverCalls {
        fn from(var: SimulateEOACall) -> Self {
            MockFlashLoanSimpleReceiverCalls::SimulateEOA(var)
        }
    }
}
