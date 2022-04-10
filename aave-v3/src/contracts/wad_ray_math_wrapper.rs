pub use wadraymathwrapper_mod::*;
#[allow(clippy::too_many_arguments)]
mod wadraymathwrapper_mod {
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
    #[doc = "WadRayMathWrapper was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WADRAYMATHWRAPPER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"halfRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"halfWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"ray\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"rayDiv\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"rayMul\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"rayToWad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"wad\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"wadDiv\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"wadMul\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"wadToRay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static WADRAYMATHWRAPPER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061035b806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c80637df38c5b116100665780637df38c5b146101125780639c34d88014610120578063d2e3058514610133578063e304e1d314610146578063e57b6d3b1461015457600080fd5b806310de27b9146100a35780631fa89fc6146100c857806329cb5aa4146100da578063416a8b20146100ed578063761fdad6146100ff575b600080fd5b6100b66100b13660046102ea565b610167565b60405190815260200160405180910390f35b6b019d971e4fe8401e740000006100b6565b6100b66100e83660046102ea565b610178565b6b033b2e3c9fd0803ce80000006100b6565b6100b661010d366004610303565b610183565b670de0b6b3a76400006100b6565b6100b661012e366004610303565b610196565b6100b6610141366004610303565b6101a2565b6706f05b59d3b200006100b6565b6100b6610162366004610303565b6101ae565b6000610172826101ba565b92915050565b6000610172826101d5565b600061018f83836101f8565b9392505050565b600061018f8383610230565b600061018f838361026f565b600061018f83836102b3565b633b9aca0081810290810482146101d057600080fd5b919050565b633b9aca00808204908206631dcd650081106101f2576001820191505b50919050565b600081156706f05b59d3b20000198390048411151761021657600080fd5b50670de0b6b3a764000091026706f05b59d3b20000010490565b600081156b033b2e3c9fd0803ce80000006002840419048411171561025457600080fd5b506b033b2e3c9fd0803ce80000009190910260028204010490565b600081156b019d971e4fe8401e74000000198390048411151761029157600080fd5b506b033b2e3c9fd0803ce800000091026b019d971e4fe8401e74000000010490565b60008115670de0b6b3a7640000600284041904841117156102d357600080fd5b50670de0b6b3a76400009190910260028204010490565b6000602082840312156102fc57600080fd5b5035919050565b6000806040838503121561031657600080fd5b5050803592602090910135915056fea264697066735822122016b9c4b4a6291bb8c75364e369a0cd51e56eb36615d6a636de8f8e4da14378ad64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct WadRayMathWrapper<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for WadRayMathWrapper<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for WadRayMathWrapper<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(WadRayMathWrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> WadRayMathWrapper<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), WADRAYMATHWRAPPER_ABI.clone(), client)
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
                WADRAYMATHWRAPPER_ABI.clone(),
                WADRAYMATHWRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `halfRay` (0x1fa89fc6) function"]
        pub fn half_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([31, 168, 159, 198], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `halfWad` (0xe304e1d3) function"]
        pub fn half_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([227, 4, 225, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ray` (0x416a8b20) function"]
        pub fn ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 106, 139, 32], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rayDiv` (0x9c34d880) function"]
        pub fn ray_div(
            &self,
            a: ethers::core::types::U256,
            b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([156, 52, 216, 128], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rayMul` (0xd2e30585) function"]
        pub fn ray_mul(
            &self,
            a: ethers::core::types::U256,
            b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([210, 227, 5, 133], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rayToWad` (0x29cb5aa4) function"]
        pub fn ray_to_wad(
            &self,
            a: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([41, 203, 90, 164], a)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wad` (0x7df38c5b) function"]
        pub fn wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([125, 243, 140, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wadDiv` (0xe57b6d3b) function"]
        pub fn wad_div(
            &self,
            a: ethers::core::types::U256,
            b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([229, 123, 109, 59], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wadMul` (0x761fdad6) function"]
        pub fn wad_mul(
            &self,
            a: ethers::core::types::U256,
            b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([118, 31, 218, 214], (a, b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wadToRay` (0x10de27b9) function"]
        pub fn wad_to_ray(
            &self,
            a: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([16, 222, 39, 185], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for WadRayMathWrapper<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `halfRay`function with signature `halfRay()` and selector `[31, 168, 159, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "halfRay", abi = "halfRay()")]
    pub struct HalfRayCall;
    #[doc = "Container type for all input parameters for the `halfWad`function with signature `halfWad()` and selector `[227, 4, 225, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "halfWad", abi = "halfWad()")]
    pub struct HalfWadCall;
    #[doc = "Container type for all input parameters for the `ray`function with signature `ray()` and selector `[65, 106, 139, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ray", abi = "ray()")]
    pub struct RayCall;
    #[doc = "Container type for all input parameters for the `rayDiv`function with signature `rayDiv(uint256,uint256)` and selector `[156, 52, 216, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rayDiv", abi = "rayDiv(uint256,uint256)")]
    pub struct RayDivCall {
        pub a: ethers::core::types::U256,
        pub b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rayMul`function with signature `rayMul(uint256,uint256)` and selector `[210, 227, 5, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rayMul", abi = "rayMul(uint256,uint256)")]
    pub struct RayMulCall {
        pub a: ethers::core::types::U256,
        pub b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rayToWad`function with signature `rayToWad(uint256)` and selector `[41, 203, 90, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rayToWad", abi = "rayToWad(uint256)")]
    pub struct RayToWadCall {
        pub a: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wad`function with signature `wad()` and selector `[125, 243, 140, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wad", abi = "wad()")]
    pub struct WadCall;
    #[doc = "Container type for all input parameters for the `wadDiv`function with signature `wadDiv(uint256,uint256)` and selector `[229, 123, 109, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wadDiv", abi = "wadDiv(uint256,uint256)")]
    pub struct WadDivCall {
        pub a: ethers::core::types::U256,
        pub b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wadMul`function with signature `wadMul(uint256,uint256)` and selector `[118, 31, 218, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wadMul", abi = "wadMul(uint256,uint256)")]
    pub struct WadMulCall {
        pub a: ethers::core::types::U256,
        pub b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wadToRay`function with signature `wadToRay(uint256)` and selector `[16, 222, 39, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wadToRay", abi = "wadToRay(uint256)")]
    pub struct WadToRayCall {
        pub a: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum WadRayMathWrapperCalls {
        HalfRay(HalfRayCall),
        HalfWad(HalfWadCall),
        Ray(RayCall),
        RayDiv(RayDivCall),
        RayMul(RayMulCall),
        RayToWad(RayToWadCall),
        Wad(WadCall),
        WadDiv(WadDivCall),
        WadMul(WadMulCall),
        WadToRay(WadToRayCall),
    }
    impl ethers::core::abi::AbiDecode for WadRayMathWrapperCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <HalfRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::HalfRay(decoded));
            }
            if let Ok(decoded) =
                <HalfWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::HalfWad(decoded));
            }
            if let Ok(decoded) = <RayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(WadRayMathWrapperCalls::Ray(decoded));
            }
            if let Ok(decoded) = <RayDivCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::RayDiv(decoded));
            }
            if let Ok(decoded) = <RayMulCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::RayMul(decoded));
            }
            if let Ok(decoded) =
                <RayToWadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::RayToWad(decoded));
            }
            if let Ok(decoded) = <WadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(WadRayMathWrapperCalls::Wad(decoded));
            }
            if let Ok(decoded) = <WadDivCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::WadDiv(decoded));
            }
            if let Ok(decoded) = <WadMulCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::WadMul(decoded));
            }
            if let Ok(decoded) =
                <WadToRayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WadRayMathWrapperCalls::WadToRay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for WadRayMathWrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                WadRayMathWrapperCalls::HalfRay(element) => element.encode(),
                WadRayMathWrapperCalls::HalfWad(element) => element.encode(),
                WadRayMathWrapperCalls::Ray(element) => element.encode(),
                WadRayMathWrapperCalls::RayDiv(element) => element.encode(),
                WadRayMathWrapperCalls::RayMul(element) => element.encode(),
                WadRayMathWrapperCalls::RayToWad(element) => element.encode(),
                WadRayMathWrapperCalls::Wad(element) => element.encode(),
                WadRayMathWrapperCalls::WadDiv(element) => element.encode(),
                WadRayMathWrapperCalls::WadMul(element) => element.encode(),
                WadRayMathWrapperCalls::WadToRay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for WadRayMathWrapperCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WadRayMathWrapperCalls::HalfRay(element) => element.fmt(f),
                WadRayMathWrapperCalls::HalfWad(element) => element.fmt(f),
                WadRayMathWrapperCalls::Ray(element) => element.fmt(f),
                WadRayMathWrapperCalls::RayDiv(element) => element.fmt(f),
                WadRayMathWrapperCalls::RayMul(element) => element.fmt(f),
                WadRayMathWrapperCalls::RayToWad(element) => element.fmt(f),
                WadRayMathWrapperCalls::Wad(element) => element.fmt(f),
                WadRayMathWrapperCalls::WadDiv(element) => element.fmt(f),
                WadRayMathWrapperCalls::WadMul(element) => element.fmt(f),
                WadRayMathWrapperCalls::WadToRay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<HalfRayCall> for WadRayMathWrapperCalls {
        fn from(var: HalfRayCall) -> Self {
            WadRayMathWrapperCalls::HalfRay(var)
        }
    }
    impl ::std::convert::From<HalfWadCall> for WadRayMathWrapperCalls {
        fn from(var: HalfWadCall) -> Self {
            WadRayMathWrapperCalls::HalfWad(var)
        }
    }
    impl ::std::convert::From<RayCall> for WadRayMathWrapperCalls {
        fn from(var: RayCall) -> Self {
            WadRayMathWrapperCalls::Ray(var)
        }
    }
    impl ::std::convert::From<RayDivCall> for WadRayMathWrapperCalls {
        fn from(var: RayDivCall) -> Self {
            WadRayMathWrapperCalls::RayDiv(var)
        }
    }
    impl ::std::convert::From<RayMulCall> for WadRayMathWrapperCalls {
        fn from(var: RayMulCall) -> Self {
            WadRayMathWrapperCalls::RayMul(var)
        }
    }
    impl ::std::convert::From<RayToWadCall> for WadRayMathWrapperCalls {
        fn from(var: RayToWadCall) -> Self {
            WadRayMathWrapperCalls::RayToWad(var)
        }
    }
    impl ::std::convert::From<WadCall> for WadRayMathWrapperCalls {
        fn from(var: WadCall) -> Self {
            WadRayMathWrapperCalls::Wad(var)
        }
    }
    impl ::std::convert::From<WadDivCall> for WadRayMathWrapperCalls {
        fn from(var: WadDivCall) -> Self {
            WadRayMathWrapperCalls::WadDiv(var)
        }
    }
    impl ::std::convert::From<WadMulCall> for WadRayMathWrapperCalls {
        fn from(var: WadMulCall) -> Self {
            WadRayMathWrapperCalls::WadMul(var)
        }
    }
    impl ::std::convert::From<WadToRayCall> for WadRayMathWrapperCalls {
        fn from(var: WadToRayCall) -> Self {
            WadRayMathWrapperCalls::WadToRay(var)
        }
    }
}
