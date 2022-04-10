pub use comptrollerlensinterface_mod::*;
#[allow(clippy::too_many_arguments)]
mod comptrollerlensinterface_mod {
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
    #[doc = "ComptrollerLensInterface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMPTROLLERLENSINTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimComp\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"compAccrued\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsIn\",\"outputs\":[{\"internalType\":\"contract CToken[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMPTROLLERLENSINTERFACE_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ComptrollerLensInterface<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ComptrollerLensInterface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ComptrollerLensInterface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ComptrollerLensInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ComptrollerLensInterface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                COMPTROLLERLENSINTERFACE_ABI.clone(),
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
                COMPTROLLERLENSINTERFACE_ABI.clone(),
                COMPTROLLERLENSINTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `claimComp` (0xe9af0292) function"]
        pub fn claim_comp(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 175, 2, 146], p0)
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
        #[doc = "Calls the contract's `getAccountLiquidity` (0x5ec88c79) function"]
        pub fn get_account_liquidity(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([94, 200, 140, 121], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsIn` (0xabfceffc) function"]
        pub fn get_assets_in(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([171, 252, 239, 252], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `markets` (0x8e8f294b) function"]
        pub fn markets(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256)>
        {
            self.0
                .method_hash([142, 143, 41, 75], p0)
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ComptrollerLensInterface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `claimComp`function with signature `claimComp(address)` and selector `[233, 175, 2, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimComp", abi = "claimComp(address)")]
    pub struct ClaimCompCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `getAccountLiquidity`function with signature `getAccountLiquidity(address)` and selector `[94, 200, 140, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountLiquidity", abi = "getAccountLiquidity(address)")]
    pub struct GetAccountLiquidityCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getAssetsIn`function with signature `getAssetsIn(address)` and selector `[171, 252, 239, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsIn", abi = "getAssetsIn(address)")]
    pub struct GetAssetsInCall(pub ethers::core::types::Address);
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ComptrollerLensInterfaceCalls {
        ClaimComp(ClaimCompCall),
        CompAccrued(CompAccruedCall),
        GetAccountLiquidity(GetAccountLiquidityCall),
        GetAssetsIn(GetAssetsInCall),
        Markets(MarketsCall),
        Oracle(OracleCall),
    }
    impl ethers::core::abi::AbiDecode for ComptrollerLensInterfaceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ClaimCompCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::ClaimComp(decoded));
            }
            if let Ok(decoded) =
                <CompAccruedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::CompAccrued(decoded));
            }
            if let Ok(decoded) =
                <GetAccountLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::GetAccountLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::GetAssetsIn(decoded));
            }
            if let Ok(decoded) =
                <MarketsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::Markets(decoded));
            }
            if let Ok(decoded) = <OracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ComptrollerLensInterfaceCalls::Oracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ComptrollerLensInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ComptrollerLensInterfaceCalls::ClaimComp(element) => element.encode(),
                ComptrollerLensInterfaceCalls::CompAccrued(element) => element.encode(),
                ComptrollerLensInterfaceCalls::GetAccountLiquidity(element) => element.encode(),
                ComptrollerLensInterfaceCalls::GetAssetsIn(element) => element.encode(),
                ComptrollerLensInterfaceCalls::Markets(element) => element.encode(),
                ComptrollerLensInterfaceCalls::Oracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ComptrollerLensInterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ComptrollerLensInterfaceCalls::ClaimComp(element) => element.fmt(f),
                ComptrollerLensInterfaceCalls::CompAccrued(element) => element.fmt(f),
                ComptrollerLensInterfaceCalls::GetAccountLiquidity(element) => element.fmt(f),
                ComptrollerLensInterfaceCalls::GetAssetsIn(element) => element.fmt(f),
                ComptrollerLensInterfaceCalls::Markets(element) => element.fmt(f),
                ComptrollerLensInterfaceCalls::Oracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ClaimCompCall> for ComptrollerLensInterfaceCalls {
        fn from(var: ClaimCompCall) -> Self {
            ComptrollerLensInterfaceCalls::ClaimComp(var)
        }
    }
    impl ::std::convert::From<CompAccruedCall> for ComptrollerLensInterfaceCalls {
        fn from(var: CompAccruedCall) -> Self {
            ComptrollerLensInterfaceCalls::CompAccrued(var)
        }
    }
    impl ::std::convert::From<GetAccountLiquidityCall> for ComptrollerLensInterfaceCalls {
        fn from(var: GetAccountLiquidityCall) -> Self {
            ComptrollerLensInterfaceCalls::GetAccountLiquidity(var)
        }
    }
    impl ::std::convert::From<GetAssetsInCall> for ComptrollerLensInterfaceCalls {
        fn from(var: GetAssetsInCall) -> Self {
            ComptrollerLensInterfaceCalls::GetAssetsIn(var)
        }
    }
    impl ::std::convert::From<MarketsCall> for ComptrollerLensInterfaceCalls {
        fn from(var: MarketsCall) -> Self {
            ComptrollerLensInterfaceCalls::Markets(var)
        }
    }
    impl ::std::convert::From<OracleCall> for ComptrollerLensInterfaceCalls {
        fn from(var: OracleCall) -> Self {
            ComptrollerLensInterfaceCalls::Oracle(var)
        }
    }
}
