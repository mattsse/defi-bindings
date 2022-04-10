pub use mapleglobalsmock_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleglobalsmock_mod {
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
    #[doc = "MapleGlobalsMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLEGLOBALSMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"governor_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"mapleTreasury_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"investorFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"governor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"investorFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mapleTreasury\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolPaused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"governor_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGovernor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"investorFee_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInvestorFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleTreasury_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMapleTreasury\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused_\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setProtocolPaused\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTreasuryFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasuryFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLEGLOBALSMOCK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060405161035a38038061035a83398101604081905261002f91610089565b600080546001600160a01b039586166001600160a01b03199182161790915560018054949095169316929092179092556002919091556003556100cc565b80516001600160a01b038116811461008457600080fd5b919050565b6000806000806080858703121561009f57600080fd5b6100a88561006d565b93506100b66020860161006d565b6040860151606090960151949790965092505050565b61027f806100db6000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c806377e741c71161006657806377e741c7146101535780638275d47114610166578063a5a2760514610192578063c42cf535146101a5578063cc32d176146101d557600080fd5b80630c340a24146100a357806316a12d7a146100d3578063425fad58146100ea5780635e0454671461010e5780637303de2514610123575b600080fd5b6000546100b6906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100dc60025481565b6040519081526020016100ca565b6001546100fe90600160a01b900460ff1681565b60405190151581526020016100ca565b61012161011c366004610230565b600255565b005b6101216101313660046101de565b600180546001600160a01b0319166001600160a01b0392909216919091179055565b610121610161366004610230565b600355565b61012161017436600461020e565b60018054911515600160a01b0260ff60a01b19909216919091179055565b6001546100b6906001600160a01b031681565b6101216101b33660046101de565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6100dc60035481565b6000602082840312156101f057600080fd5b81356001600160a01b038116811461020757600080fd5b9392505050565b60006020828403121561022057600080fd5b8135801515811461020757600080fd5b60006020828403121561024257600080fd5b503591905056fea2646970667358221220960fa598f91991aa59fd5d826ee225ed174faa4e40506b35f958bb6a3edf990264736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MapleGlobalsMock<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleGlobalsMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleGlobalsMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleGlobalsMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleGlobalsMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAPLEGLOBALSMOCK_ABI.clone(), client)
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
                MAPLEGLOBALSMOCK_ABI.clone(),
                MAPLEGLOBALSMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `governor` (0x0c340a24) function"]
        pub fn governor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([12, 52, 10, 36], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `investorFee` (0x16a12d7a) function"]
        pub fn investor_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([22, 161, 45, 122], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleTreasury` (0xa5a27605) function"]
        pub fn maple_treasury(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([165, 162, 118, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolPaused` (0x425fad58) function"]
        pub fn protocol_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 95, 173, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGovernor` (0xc42cf535) function"]
        pub fn set_governor(
            &self,
            governor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 44, 245, 53], governor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setInvestorFee` (0x5e045467) function"]
        pub fn set_investor_fee(
            &self,
            investor_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 4, 84, 103], investor_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMapleTreasury` (0x7303de25) function"]
        pub fn set_maple_treasury(
            &self,
            maple_treasury: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 3, 222, 37], maple_treasury)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setProtocolPaused` (0x8275d471) function"]
        pub fn set_protocol_paused(
            &self,
            paused: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 117, 212, 113], paused)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTreasuryFee` (0x77e741c7) function"]
        pub fn set_treasury_fee(
            &self,
            treasury_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 231, 65, 199], treasury_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `treasuryFee` (0xcc32d176) function"]
        pub fn treasury_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([204, 50, 209, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MapleGlobalsMock<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `governor`function with signature `governor()` and selector `[12, 52, 10, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    #[doc = "Container type for all input parameters for the `investorFee`function with signature `investorFee()` and selector `[22, 161, 45, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "investorFee", abi = "investorFee()")]
    pub struct InvestorFeeCall;
    #[doc = "Container type for all input parameters for the `mapleTreasury`function with signature `mapleTreasury()` and selector `[165, 162, 118, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mapleTreasury", abi = "mapleTreasury()")]
    pub struct MapleTreasuryCall;
    #[doc = "Container type for all input parameters for the `protocolPaused`function with signature `protocolPaused()` and selector `[66, 95, 173, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "protocolPaused", abi = "protocolPaused()")]
    pub struct ProtocolPausedCall;
    #[doc = "Container type for all input parameters for the `setGovernor`function with signature `setGovernor(address)` and selector `[196, 44, 245, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGovernor", abi = "setGovernor(address)")]
    pub struct SetGovernorCall {
        pub governor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setInvestorFee`function with signature `setInvestorFee(uint256)` and selector `[94, 4, 84, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setInvestorFee", abi = "setInvestorFee(uint256)")]
    pub struct SetInvestorFeeCall {
        pub investor_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setMapleTreasury`function with signature `setMapleTreasury(address)` and selector `[115, 3, 222, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMapleTreasury", abi = "setMapleTreasury(address)")]
    pub struct SetMapleTreasuryCall {
        pub maple_treasury: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setProtocolPaused`function with signature `setProtocolPaused(bool)` and selector `[130, 117, 212, 113]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setProtocolPaused", abi = "setProtocolPaused(bool)")]
    pub struct SetProtocolPausedCall {
        pub paused: bool,
    }
    #[doc = "Container type for all input parameters for the `setTreasuryFee`function with signature `setTreasuryFee(uint256)` and selector `[119, 231, 65, 199]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTreasuryFee", abi = "setTreasuryFee(uint256)")]
    pub struct SetTreasuryFeeCall {
        pub treasury_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `treasuryFee`function with signature `treasuryFee()` and selector `[204, 50, 209, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "treasuryFee", abi = "treasuryFee()")]
    pub struct TreasuryFeeCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleGlobalsMockCalls {
        Governor(GovernorCall),
        InvestorFee(InvestorFeeCall),
        MapleTreasury(MapleTreasuryCall),
        ProtocolPaused(ProtocolPausedCall),
        SetGovernor(SetGovernorCall),
        SetInvestorFee(SetInvestorFeeCall),
        SetMapleTreasury(SetMapleTreasuryCall),
        SetProtocolPaused(SetProtocolPausedCall),
        SetTreasuryFee(SetTreasuryFeeCall),
        TreasuryFee(TreasuryFeeCall),
    }
    impl ethers::core::abi::AbiDecode for MapleGlobalsMockCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GovernorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::Governor(decoded));
            }
            if let Ok(decoded) =
                <InvestorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::InvestorFee(decoded));
            }
            if let Ok(decoded) =
                <MapleTreasuryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::MapleTreasury(decoded));
            }
            if let Ok(decoded) =
                <ProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::ProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <SetGovernorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::SetGovernor(decoded));
            }
            if let Ok(decoded) =
                <SetInvestorFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::SetInvestorFee(decoded));
            }
            if let Ok(decoded) =
                <SetMapleTreasuryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::SetMapleTreasury(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::SetProtocolPaused(decoded));
            }
            if let Ok(decoded) =
                <SetTreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::SetTreasuryFee(decoded));
            }
            if let Ok(decoded) =
                <TreasuryFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleGlobalsMockCalls::TreasuryFee(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleGlobalsMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleGlobalsMockCalls::Governor(element) => element.encode(),
                MapleGlobalsMockCalls::InvestorFee(element) => element.encode(),
                MapleGlobalsMockCalls::MapleTreasury(element) => element.encode(),
                MapleGlobalsMockCalls::ProtocolPaused(element) => element.encode(),
                MapleGlobalsMockCalls::SetGovernor(element) => element.encode(),
                MapleGlobalsMockCalls::SetInvestorFee(element) => element.encode(),
                MapleGlobalsMockCalls::SetMapleTreasury(element) => element.encode(),
                MapleGlobalsMockCalls::SetProtocolPaused(element) => element.encode(),
                MapleGlobalsMockCalls::SetTreasuryFee(element) => element.encode(),
                MapleGlobalsMockCalls::TreasuryFee(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleGlobalsMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleGlobalsMockCalls::Governor(element) => element.fmt(f),
                MapleGlobalsMockCalls::InvestorFee(element) => element.fmt(f),
                MapleGlobalsMockCalls::MapleTreasury(element) => element.fmt(f),
                MapleGlobalsMockCalls::ProtocolPaused(element) => element.fmt(f),
                MapleGlobalsMockCalls::SetGovernor(element) => element.fmt(f),
                MapleGlobalsMockCalls::SetInvestorFee(element) => element.fmt(f),
                MapleGlobalsMockCalls::SetMapleTreasury(element) => element.fmt(f),
                MapleGlobalsMockCalls::SetProtocolPaused(element) => element.fmt(f),
                MapleGlobalsMockCalls::SetTreasuryFee(element) => element.fmt(f),
                MapleGlobalsMockCalls::TreasuryFee(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GovernorCall> for MapleGlobalsMockCalls {
        fn from(var: GovernorCall) -> Self {
            MapleGlobalsMockCalls::Governor(var)
        }
    }
    impl ::std::convert::From<InvestorFeeCall> for MapleGlobalsMockCalls {
        fn from(var: InvestorFeeCall) -> Self {
            MapleGlobalsMockCalls::InvestorFee(var)
        }
    }
    impl ::std::convert::From<MapleTreasuryCall> for MapleGlobalsMockCalls {
        fn from(var: MapleTreasuryCall) -> Self {
            MapleGlobalsMockCalls::MapleTreasury(var)
        }
    }
    impl ::std::convert::From<ProtocolPausedCall> for MapleGlobalsMockCalls {
        fn from(var: ProtocolPausedCall) -> Self {
            MapleGlobalsMockCalls::ProtocolPaused(var)
        }
    }
    impl ::std::convert::From<SetGovernorCall> for MapleGlobalsMockCalls {
        fn from(var: SetGovernorCall) -> Self {
            MapleGlobalsMockCalls::SetGovernor(var)
        }
    }
    impl ::std::convert::From<SetInvestorFeeCall> for MapleGlobalsMockCalls {
        fn from(var: SetInvestorFeeCall) -> Self {
            MapleGlobalsMockCalls::SetInvestorFee(var)
        }
    }
    impl ::std::convert::From<SetMapleTreasuryCall> for MapleGlobalsMockCalls {
        fn from(var: SetMapleTreasuryCall) -> Self {
            MapleGlobalsMockCalls::SetMapleTreasury(var)
        }
    }
    impl ::std::convert::From<SetProtocolPausedCall> for MapleGlobalsMockCalls {
        fn from(var: SetProtocolPausedCall) -> Self {
            MapleGlobalsMockCalls::SetProtocolPaused(var)
        }
    }
    impl ::std::convert::From<SetTreasuryFeeCall> for MapleGlobalsMockCalls {
        fn from(var: SetTreasuryFeeCall) -> Self {
            MapleGlobalsMockCalls::SetTreasuryFee(var)
        }
    }
    impl ::std::convert::From<TreasuryFeeCall> for MapleGlobalsMockCalls {
        fn from(var: TreasuryFeeCall) -> Self {
            MapleGlobalsMockCalls::TreasuryFee(var)
        }
    }
}
