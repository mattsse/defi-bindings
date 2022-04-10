pub use priceoraclesentinel_mod::*;
#[allow(clippy::too_many_arguments)]
mod priceoraclesentinel_mod {
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
    #[doc = "PriceOracleSentinel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEORACLESENTINEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ISequencerOracle\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gracePeriod\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newGracePeriod\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GracePeriodUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newSequencerOracle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SequencerOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGracePeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSequencerOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBorrowAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isLiquidationAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newGracePeriod\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGracePeriod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newSequencerOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencerOracle\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEORACLESENTINEL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405234801561001057600080fd5b506040516107c03803806107c083398101604081905261002f91610076565b6001600160a01b03928316608052600080546001600160a01b03191692909316919091179091556001556100b9565b6001600160a01b038116811461007357600080fd5b50565b60008060006060848603121561008b57600080fd5b83516100968161005e565b60208501519093506100a78161005e565b80925050604084015190509250925092565b6080516106df6100e16000396000818160870152818161013b01526102c201526106df6000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c80637a5d20ea1161005b5780637a5d20ea146100d7578063dbd18388146100ef578063f0aef31c14610100578063f2f659601461011557600080fd5b80630542975c1461008257806312168dc2146100c657806349aa2e81146100d7575b600080fd5b6100a97f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6000546001600160a01b03166100a9565b6100df610128565b60405190151581526020016100bd565b6001546040519081526020016100bd565b61011361010e366004610544565b610137565b005b610113610123366004610568565b6102be565b6000610132610489565b905090565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663707cd7166040518163ffffffff1660e01b8152600401602060405180830381865afa158015610197573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101bb9190610581565b604051637be53ca160e01b81523360048201529091506001600160a01b03821690637be53ca190602401602060405180830381865afa158015610202573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610226919061059e565b604051806040016040528060018152602001603160f81b815250906102675760405162461bcd60e51b815260040161025e91906105c0565b60405180910390fd5b50600080546001600160a01b0319166001600160a01b0384169081179091556040519081527f95cbf1d8f44ec81ff345ed9cf2fe53b6a6473e072bf046ee412f198c54dba449906020015b60405180910390a15050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663707cd7166040518163ffffffff1660e01b8152600401602060405180830381865afa15801561031e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103429190610581565b60405163674b5e4d60e01b81523360048201529091506001600160a01b0382169063674b5e4d90602401602060405180830381865afa158015610389573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103ad919061059e565b8061041b5750604051637be53ca160e01b81523360048201526001600160a01b03821690637be53ca190602401602060405180830381865afa1580156103f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061041b919061059e565b604051806040016040528060018152602001600d60fa1b815250906104535760405162461bcd60e51b815260040161025e91906105c0565b5060018290556040518281527f33d1191f5a3abfe19d468d51bb5ece97489f1277a912a5b5c65992fc279ad3d4906020016102b2565b60008060008060009054906101000a90046001600160a01b03166001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa1580156104e0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105049190610634565b5093505092505081600014801561052557506001546105238242610684565b115b9250505090565b6001600160a01b038116811461054157600080fd5b50565b60006020828403121561055657600080fd5b81356105618161052c565b9392505050565b60006020828403121561057a57600080fd5b5035919050565b60006020828403121561059357600080fd5b81516105618161052c565b6000602082840312156105b057600080fd5b8151801515811461056157600080fd5b600060208083528351808285015260005b818110156105ed578581018301518582016040015282016105d1565b818111156105ff576000604083870101525b50601f01601f1916929092016040019392505050565b805169ffffffffffffffffffff8116811461062f57600080fd5b919050565b600080600080600060a0868803121561064c57600080fd5b61065586610615565b945060208601519350604086015192506060860151915061067860808701610615565b90509295509295909350565b6000828210156106a457634e487b7160e01b600052601160045260246000fd5b50039056fea264697066735822122073225dbfa2436627153c5a61c32ebb4795803da2b75dc06ef5a8bd7cde0025d764736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct PriceOracleSentinel<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PriceOracleSentinel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceOracleSentinel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceOracleSentinel))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PriceOracleSentinel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEORACLESENTINEL_ABI.clone(), client)
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
                PRICEORACLESENTINEL_ABI.clone(),
                PRICEORACLESENTINEL_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `getGracePeriod` (0xdbd18388) function"]
        pub fn get_grace_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 209, 131, 136], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSequencerOracle` (0x12168dc2) function"]
        pub fn get_sequencer_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([18, 22, 141, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isBorrowAllowed` (0x49aa2e81) function"]
        pub fn is_borrow_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 170, 46, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLiquidationAllowed` (0x7a5d20ea) function"]
        pub fn is_liquidation_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 93, 32, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGracePeriod` (0xf2f65960) function"]
        pub fn set_grace_period(
            &self,
            new_grace_period: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 246, 89, 96], new_grace_period)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSequencerOracle` (0xf0aef31c) function"]
        pub fn set_sequencer_oracle(
            &self,
            new_sequencer_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 174, 243, 28], new_sequencer_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GracePeriodUpdated` event"]
        pub fn grace_period_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GracePeriodUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequencerOracleUpdated` event"]
        pub fn sequencer_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequencerOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PriceOracleSentinelEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PriceOracleSentinel<M>
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
    #[ethevent(name = "GracePeriodUpdated", abi = "GracePeriodUpdated(uint256)")]
    pub struct GracePeriodUpdatedFilter {
        pub new_grace_period: ethers::core::types::U256,
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
        name = "SequencerOracleUpdated",
        abi = "SequencerOracleUpdated(address)"
    )]
    pub struct SequencerOracleUpdatedFilter {
        pub new_sequencer_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleSentinelEvents {
        GracePeriodUpdatedFilter(GracePeriodUpdatedFilter),
        SequencerOracleUpdatedFilter(SequencerOracleUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for PriceOracleSentinelEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = GracePeriodUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleSentinelEvents::GracePeriodUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SequencerOracleUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleSentinelEvents::SequencerOracleUpdatedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PriceOracleSentinelEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleSentinelEvents::GracePeriodUpdatedFilter(element) => element.fmt(f),
                PriceOracleSentinelEvents::SequencerOracleUpdatedFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `getGracePeriod`function with signature `getGracePeriod()` and selector `[219, 209, 131, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getGracePeriod", abi = "getGracePeriod()")]
    pub struct GetGracePeriodCall;
    #[doc = "Container type for all input parameters for the `getSequencerOracle`function with signature `getSequencerOracle()` and selector `[18, 22, 141, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSequencerOracle", abi = "getSequencerOracle()")]
    pub struct GetSequencerOracleCall;
    #[doc = "Container type for all input parameters for the `isBorrowAllowed`function with signature `isBorrowAllowed()` and selector `[73, 170, 46, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isBorrowAllowed", abi = "isBorrowAllowed()")]
    pub struct IsBorrowAllowedCall;
    #[doc = "Container type for all input parameters for the `isLiquidationAllowed`function with signature `isLiquidationAllowed()` and selector `[122, 93, 32, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isLiquidationAllowed", abi = "isLiquidationAllowed()")]
    pub struct IsLiquidationAllowedCall;
    #[doc = "Container type for all input parameters for the `setGracePeriod`function with signature `setGracePeriod(uint256)` and selector `[242, 246, 89, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGracePeriod", abi = "setGracePeriod(uint256)")]
    pub struct SetGracePeriodCall {
        pub new_grace_period: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSequencerOracle`function with signature `setSequencerOracle(address)` and selector `[240, 174, 243, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSequencerOracle", abi = "setSequencerOracle(address)")]
    pub struct SetSequencerOracleCall {
        pub new_sequencer_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleSentinelCalls {
        AddressesProvider(AddressesProviderCall),
        GetGracePeriod(GetGracePeriodCall),
        GetSequencerOracle(GetSequencerOracleCall),
        IsBorrowAllowed(IsBorrowAllowedCall),
        IsLiquidationAllowed(IsLiquidationAllowedCall),
        SetGracePeriod(SetGracePeriodCall),
        SetSequencerOracle(SetSequencerOracleCall),
    }
    impl ethers::core::abi::AbiDecode for PriceOracleSentinelCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <GetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::GetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <GetSequencerOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::GetSequencerOracle(decoded));
            }
            if let Ok(decoded) =
                <IsBorrowAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::IsBorrowAllowed(decoded));
            }
            if let Ok(decoded) =
                <IsLiquidationAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::IsLiquidationAllowed(decoded));
            }
            if let Ok(decoded) =
                <SetGracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::SetGracePeriod(decoded));
            }
            if let Ok(decoded) =
                <SetSequencerOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleSentinelCalls::SetSequencerOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceOracleSentinelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceOracleSentinelCalls::AddressesProvider(element) => element.encode(),
                PriceOracleSentinelCalls::GetGracePeriod(element) => element.encode(),
                PriceOracleSentinelCalls::GetSequencerOracle(element) => element.encode(),
                PriceOracleSentinelCalls::IsBorrowAllowed(element) => element.encode(),
                PriceOracleSentinelCalls::IsLiquidationAllowed(element) => element.encode(),
                PriceOracleSentinelCalls::SetGracePeriod(element) => element.encode(),
                PriceOracleSentinelCalls::SetSequencerOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceOracleSentinelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleSentinelCalls::AddressesProvider(element) => element.fmt(f),
                PriceOracleSentinelCalls::GetGracePeriod(element) => element.fmt(f),
                PriceOracleSentinelCalls::GetSequencerOracle(element) => element.fmt(f),
                PriceOracleSentinelCalls::IsBorrowAllowed(element) => element.fmt(f),
                PriceOracleSentinelCalls::IsLiquidationAllowed(element) => element.fmt(f),
                PriceOracleSentinelCalls::SetGracePeriod(element) => element.fmt(f),
                PriceOracleSentinelCalls::SetSequencerOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for PriceOracleSentinelCalls {
        fn from(var: AddressesProviderCall) -> Self {
            PriceOracleSentinelCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<GetGracePeriodCall> for PriceOracleSentinelCalls {
        fn from(var: GetGracePeriodCall) -> Self {
            PriceOracleSentinelCalls::GetGracePeriod(var)
        }
    }
    impl ::std::convert::From<GetSequencerOracleCall> for PriceOracleSentinelCalls {
        fn from(var: GetSequencerOracleCall) -> Self {
            PriceOracleSentinelCalls::GetSequencerOracle(var)
        }
    }
    impl ::std::convert::From<IsBorrowAllowedCall> for PriceOracleSentinelCalls {
        fn from(var: IsBorrowAllowedCall) -> Self {
            PriceOracleSentinelCalls::IsBorrowAllowed(var)
        }
    }
    impl ::std::convert::From<IsLiquidationAllowedCall> for PriceOracleSentinelCalls {
        fn from(var: IsLiquidationAllowedCall) -> Self {
            PriceOracleSentinelCalls::IsLiquidationAllowed(var)
        }
    }
    impl ::std::convert::From<SetGracePeriodCall> for PriceOracleSentinelCalls {
        fn from(var: SetGracePeriodCall) -> Self {
            PriceOracleSentinelCalls::SetGracePeriod(var)
        }
    }
    impl ::std::convert::From<SetSequencerOracleCall> for PriceOracleSentinelCalls {
        fn from(var: SetSequencerOracleCall) -> Self {
            PriceOracleSentinelCalls::SetSequencerOracle(var)
        }
    }
}
