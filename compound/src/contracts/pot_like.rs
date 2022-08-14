pub use pot_like::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod pot_like {
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
    #[doc = "PotLike was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static POTLIKE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chi\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drip\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dsr\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"join\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pie\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rho\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct PotLike<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PotLike<M> {
        fn clone(&self) -> Self {
            PotLike(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PotLike<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PotLike<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PotLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PotLike<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), POTLIKE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `chi` (0xc92aecc4) function"]
        pub fn chi(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([201, 42, 236, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drip` (0x9f678cca) function"]
        pub fn drip(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 103, 140, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dsr` (0x487bf082) function"]
        pub fn dsr(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([72, 123, 240, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exit` (0x7f8661a1) function"]
        pub fn exit(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 134, 97, 161], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `join` (0x049878f3) function"]
        pub fn join(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 152, 120, 243], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pie` (0x0bebac86) function"]
        pub fn pie(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 235, 172, 134], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rho` (0x20aba08b) function"]
        pub fn rho(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([32, 171, 160, 139], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PotLike<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `chi` function with signature `chi()` and selector `[201, 42, 236, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "chi", abi = "chi()")]
    pub struct ChiCall;
    #[doc = "Container type for all input parameters for the `drip` function with signature `drip()` and selector `[159, 103, 140, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "drip", abi = "drip()")]
    pub struct DripCall;
    #[doc = "Container type for all input parameters for the `dsr` function with signature `dsr()` and selector `[72, 123, 240, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dsr", abi = "dsr()")]
    pub struct DsrCall;
    #[doc = "Container type for all input parameters for the `exit` function with signature `exit(uint256)` and selector `[127, 134, 97, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exit", abi = "exit(uint256)")]
    pub struct ExitCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `join` function with signature `join(uint256)` and selector `[4, 152, 120, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "join", abi = "join(uint256)")]
    pub struct JoinCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `pie` function with signature `pie(address)` and selector `[11, 235, 172, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pie", abi = "pie(address)")]
    pub struct PieCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `rho` function with signature `rho()` and selector `[32, 171, 160, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rho", abi = "rho()")]
    pub struct RhoCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PotLikeCalls {
        Chi(ChiCall),
        Drip(DripCall),
        Dsr(DsrCall),
        Exit(ExitCall),
        Join(JoinCall),
        Pie(PieCall),
        Rho(RhoCall),
    }
    impl ethers::core::abi::AbiDecode for PotLikeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <ChiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Chi(decoded));
            }
            if let Ok(decoded) = <DripCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Drip(decoded));
            }
            if let Ok(decoded) = <DsrCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Dsr(decoded));
            }
            if let Ok(decoded) = <ExitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Exit(decoded));
            }
            if let Ok(decoded) = <JoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Join(decoded));
            }
            if let Ok(decoded) = <PieCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Pie(decoded));
            }
            if let Ok(decoded) = <RhoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PotLikeCalls::Rho(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PotLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PotLikeCalls::Chi(element) => element.encode(),
                PotLikeCalls::Drip(element) => element.encode(),
                PotLikeCalls::Dsr(element) => element.encode(),
                PotLikeCalls::Exit(element) => element.encode(),
                PotLikeCalls::Join(element) => element.encode(),
                PotLikeCalls::Pie(element) => element.encode(),
                PotLikeCalls::Rho(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PotLikeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PotLikeCalls::Chi(element) => element.fmt(f),
                PotLikeCalls::Drip(element) => element.fmt(f),
                PotLikeCalls::Dsr(element) => element.fmt(f),
                PotLikeCalls::Exit(element) => element.fmt(f),
                PotLikeCalls::Join(element) => element.fmt(f),
                PotLikeCalls::Pie(element) => element.fmt(f),
                PotLikeCalls::Rho(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ChiCall> for PotLikeCalls {
        fn from(var: ChiCall) -> Self {
            PotLikeCalls::Chi(var)
        }
    }
    impl ::std::convert::From<DripCall> for PotLikeCalls {
        fn from(var: DripCall) -> Self {
            PotLikeCalls::Drip(var)
        }
    }
    impl ::std::convert::From<DsrCall> for PotLikeCalls {
        fn from(var: DsrCall) -> Self {
            PotLikeCalls::Dsr(var)
        }
    }
    impl ::std::convert::From<ExitCall> for PotLikeCalls {
        fn from(var: ExitCall) -> Self {
            PotLikeCalls::Exit(var)
        }
    }
    impl ::std::convert::From<JoinCall> for PotLikeCalls {
        fn from(var: JoinCall) -> Self {
            PotLikeCalls::Join(var)
        }
    }
    impl ::std::convert::From<PieCall> for PotLikeCalls {
        fn from(var: PieCall) -> Self {
            PotLikeCalls::Pie(var)
        }
    }
    impl ::std::convert::From<RhoCall> for PotLikeCalls {
        fn from(var: RhoCall) -> Self {
            PotLikeCalls::Rho(var)
        }
    }
    #[doc = "Container type for all return fields from the `chi` function with signature `chi()` and selector `[201, 42, 236, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ChiReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `drip` function with signature `drip()` and selector `[159, 103, 140, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DripReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dsr` function with signature `dsr()` and selector `[72, 123, 240, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DsrReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `pie` function with signature `pie(address)` and selector `[11, 235, 172, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PieReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `rho` function with signature `rho()` and selector `[32, 171, 160, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RhoReturn(pub ethers::core::types::U256);
}
