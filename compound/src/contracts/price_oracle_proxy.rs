pub use price_oracle_proxy::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod price_oracle_proxy {
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
    #[doc = "PriceOracleProxy was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEORACLEPROXY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"guardian_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"v1PriceOracle_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cEthAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cUsdcAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cSaiAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cDaiAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"cUsdtAddress_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cDaiAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cEthAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cSaiAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cUsdcAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cUsdtAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"daiOracleKey\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUnderlyingPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"guardian\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPriceOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"saiPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSaiPrice\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"usdcOracleKey\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"v1PriceOracle\",\"outputs\":[{\"internalType\":\"contract V1PriceOracleInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEORACLEPROXY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50604051610739380380610739833981810160405260e081101561003357600080fd5b508051602082015160408301516060840151608085015160a086015160c090960151600180546001600160a01b039788166001600160a01b03199182161790915560008054968816968216969096179095556002805494871694861694909417909355600380549286169285169290921790915560058054918516918416919091179055600680549484169483169490941790935560048054929093169116179055610655806100e46000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c8063a86b19441161008c578063f9c99e9c11610066578063f9c99e9c1461015e578063fc57d4df1461017d578063fe10c98d146101a3578063ff11439b146101ab576100cf565b8063a86b194414610134578063e5ee0f6e1461014e578063f2c65bf914610156576100cf565b80631a3cb2e8146100d457806321b49128146100f85780632ed58e15146101005780633e76f25514610108578063452a93201461011057806366331bba14610118575b600080fd5b6100dc6101b3565b604080516001600160a01b039092168252519081900360200190f35b6100dc6101c2565b6100dc6101d1565b6100dc6101e0565b6100dc6101e5565b6101206101e0565b604080519115158252519081900360200190f35b61013c6101f4565b60408051918252519081900360200190f35b6100dc6101fa565b6100dc6101ff565b61017b6004803603602081101561017457600080fd5b503561020e565b005b61013c6004803603602081101561019357600080fd5b50356001600160a01b031661030d565b6100dc6105df565b6100dc6105ee565b6004546001600160a01b031681565b6005546001600160a01b031681565b6002546001600160a01b031681565b600181565b6001546001600160a01b031681565b60075481565b600281565b6006546001600160a01b031681565b6001546001600160a01b031633146102575760405162461bcd60e51b81526004018080602001828103825260238152602001806105fe6023913960400191505060405180910390fd5b600754156102ac576040805162461bcd60e51b815260206004820152601e60248201527f534149207072696365206d6179206f6e6c7920626520736574206f6e63650000604482015290519081900360640190fd5b67016345785d8a00008110610308576040805162461bcd60e51b815260206004820152601b60248201527f534149207072696365206d757374206265203c20302e31204554480000000000604482015290519081900360640190fd5b600755565b60025460009082906001600160a01b038083169116141561033957670de0b6b3a76400009150506105da565b6003546001600160a01b038281169116148061036257506004546001600160a01b038281169116145b156103e857600054604080516317a6948f60e21b81526001600482015290516001600160a01b0390921691635e9a523c91602480820192602092909190829003018186803b1580156103b357600080fd5b505afa1580156103c7573d6000803e3d6000fd5b505050506040513d60208110156103dd57600080fd5b505191506105da9050565b6006546001600160a01b038281169116141561044a57600054604080516317a6948f60e21b81526002600482015290516001600160a01b0390921691635e9a523c91602480820192602092909190829003018186803b1580156103b357600080fd5b6005546001600160a01b03828116911614156104f3576000600754116104e757600054604080516317a6948f60e21b81526002600482015290516001600160a01b0390921691635e9a523c91602480820192602092909190829003018186803b1580156104b657600080fd5b505afa1580156104ca573d6000803e3d6000fd5b505050506040513d60208110156104e057600080fd5b50516104eb565b6007545b9150506105da565b6000816001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b15801561052e57600080fd5b505afa158015610542573d6000803e3d6000fd5b505050506040513d602081101561055857600080fd5b5051600054604080516317a6948f60e21b81526001600160a01b0380851660048301529151939450911691635e9a523c91602480820192602092909190829003018186803b1580156105a957600080fd5b505afa1580156105bd573d6000803e3d6000fd5b505050506040513d60208110156105d357600080fd5b5051925050505b919050565b6000546001600160a01b031681565b6003546001600160a01b03168156fe6f6e6c7920677561726469616e206d6179207365742074686520534149207072696365a265627a7a723158200f0830c5406cd4525239936a174eb71b2b2081b4044f3a93daff4991df4b104664736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    pub struct PriceOracleProxy<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PriceOracleProxy<M> {
        fn clone(&self) -> Self {
            PriceOracleProxy(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PriceOracleProxy<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceOracleProxy<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceOracleProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PriceOracleProxy<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEORACLEPROXY_ABI.clone(), client)
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
                PRICEORACLEPROXY_ABI.clone(),
                PRICEORACLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `cDaiAddress` (0xf2c65bf9) function"]
        pub fn c_dai_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([242, 198, 91, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cEthAddress` (0x2ed58e15) function"]
        pub fn c_eth_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([46, 213, 142, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cSaiAddress` (0x21b49128) function"]
        pub fn c_sai_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 180, 145, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cUsdcAddress` (0xff11439b) function"]
        pub fn c_usdc_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([255, 17, 67, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cUsdtAddress` (0x1a3cb2e8) function"]
        pub fn c_usdt_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([26, 60, 178, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `daiOracleKey` (0xe5ee0f6e) function"]
        pub fn dai_oracle_key(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([229, 238, 15, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUnderlyingPrice` (0xfc57d4df) function"]
        pub fn get_underlying_price(
            &self,
            c_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 87, 212, 223], c_token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `guardian` (0x452a9320) function"]
        pub fn guardian(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([69, 42, 147, 32], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPriceOracle` (0x66331bba) function"]
        pub fn is_price_oracle(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 51, 27, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `saiPrice` (0xa86b1944) function"]
        pub fn sai_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 107, 25, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSaiPrice` (0xf9c99e9c) function"]
        pub fn set_sai_price(
            &self,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 201, 158, 156], price)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `usdcOracleKey` (0x3e76f255) function"]
        pub fn usdc_oracle_key(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([62, 118, 242, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `v1PriceOracle` (0xfe10c98d) function"]
        pub fn v_1_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([254, 16, 201, 141], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PriceOracleProxy<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `cDaiAddress` function with signature `cDaiAddress()` and selector `[242, 198, 91, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cDaiAddress", abi = "cDaiAddress()")]
    pub struct CdaiAddressCall;
    #[doc = "Container type for all input parameters for the `cEthAddress` function with signature `cEthAddress()` and selector `[46, 213, 142, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cEthAddress", abi = "cEthAddress()")]
    pub struct CethAddressCall;
    #[doc = "Container type for all input parameters for the `cSaiAddress` function with signature `cSaiAddress()` and selector `[33, 180, 145, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cSaiAddress", abi = "cSaiAddress()")]
    pub struct CsaiAddressCall;
    #[doc = "Container type for all input parameters for the `cUsdcAddress` function with signature `cUsdcAddress()` and selector `[255, 17, 67, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cUsdcAddress", abi = "cUsdcAddress()")]
    pub struct CusdcAddressCall;
    #[doc = "Container type for all input parameters for the `cUsdtAddress` function with signature `cUsdtAddress()` and selector `[26, 60, 178, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cUsdtAddress", abi = "cUsdtAddress()")]
    pub struct CusdtAddressCall;
    #[doc = "Container type for all input parameters for the `daiOracleKey` function with signature `daiOracleKey()` and selector `[229, 238, 15, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "daiOracleKey", abi = "daiOracleKey()")]
    pub struct DaiOracleKeyCall;
    #[doc = "Container type for all input parameters for the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `[252, 87, 212, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUnderlyingPrice", abi = "getUnderlyingPrice(address)")]
    pub struct GetUnderlyingPriceCall {
        pub c_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `guardian` function with signature `guardian()` and selector `[69, 42, 147, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "guardian", abi = "guardian()")]
    pub struct GuardianCall;
    #[doc = "Container type for all input parameters for the `isPriceOracle` function with signature `isPriceOracle()` and selector `[102, 51, 27, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPriceOracle", abi = "isPriceOracle()")]
    pub struct IsPriceOracleCall;
    #[doc = "Container type for all input parameters for the `saiPrice` function with signature `saiPrice()` and selector `[168, 107, 25, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "saiPrice", abi = "saiPrice()")]
    pub struct SaiPriceCall;
    #[doc = "Container type for all input parameters for the `setSaiPrice` function with signature `setSaiPrice(uint256)` and selector `[249, 201, 158, 156]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSaiPrice", abi = "setSaiPrice(uint256)")]
    pub struct SetSaiPriceCall {
        pub price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `usdcOracleKey` function with signature `usdcOracleKey()` and selector `[62, 118, 242, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "usdcOracleKey", abi = "usdcOracleKey()")]
    pub struct UsdcOracleKeyCall;
    #[doc = "Container type for all input parameters for the `v1PriceOracle` function with signature `v1PriceOracle()` and selector `[254, 16, 201, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "v1PriceOracle", abi = "v1PriceOracle()")]
    pub struct V1PriceOracleCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleProxyCalls {
        CdaiAddress(CdaiAddressCall),
        CethAddress(CethAddressCall),
        CsaiAddress(CsaiAddressCall),
        CusdcAddress(CusdcAddressCall),
        CusdtAddress(CusdtAddressCall),
        DaiOracleKey(DaiOracleKeyCall),
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        Guardian(GuardianCall),
        IsPriceOracle(IsPriceOracleCall),
        SaiPrice(SaiPriceCall),
        SetSaiPrice(SetSaiPriceCall),
        UsdcOracleKey(UsdcOracleKeyCall),
        V1PriceOracle(V1PriceOracleCall),
    }
    impl ethers::core::abi::AbiDecode for PriceOracleProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CdaiAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::CdaiAddress(decoded));
            }
            if let Ok(decoded) =
                <CethAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::CethAddress(decoded));
            }
            if let Ok(decoded) =
                <CsaiAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::CsaiAddress(decoded));
            }
            if let Ok(decoded) =
                <CusdcAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::CusdcAddress(decoded));
            }
            if let Ok(decoded) =
                <CusdtAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::CusdtAddress(decoded));
            }
            if let Ok(decoded) =
                <DaiOracleKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::DaiOracleKey(decoded));
            }
            if let Ok(decoded) =
                <GetUnderlyingPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) =
                <GuardianCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::Guardian(decoded));
            }
            if let Ok(decoded) =
                <IsPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::IsPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SaiPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::SaiPrice(decoded));
            }
            if let Ok(decoded) =
                <SetSaiPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::SetSaiPrice(decoded));
            }
            if let Ok(decoded) =
                <UsdcOracleKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::UsdcOracleKey(decoded));
            }
            if let Ok(decoded) =
                <V1PriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleProxyCalls::V1PriceOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceOracleProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceOracleProxyCalls::CdaiAddress(element) => element.encode(),
                PriceOracleProxyCalls::CethAddress(element) => element.encode(),
                PriceOracleProxyCalls::CsaiAddress(element) => element.encode(),
                PriceOracleProxyCalls::CusdcAddress(element) => element.encode(),
                PriceOracleProxyCalls::CusdtAddress(element) => element.encode(),
                PriceOracleProxyCalls::DaiOracleKey(element) => element.encode(),
                PriceOracleProxyCalls::GetUnderlyingPrice(element) => element.encode(),
                PriceOracleProxyCalls::Guardian(element) => element.encode(),
                PriceOracleProxyCalls::IsPriceOracle(element) => element.encode(),
                PriceOracleProxyCalls::SaiPrice(element) => element.encode(),
                PriceOracleProxyCalls::SetSaiPrice(element) => element.encode(),
                PriceOracleProxyCalls::UsdcOracleKey(element) => element.encode(),
                PriceOracleProxyCalls::V1PriceOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceOracleProxyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleProxyCalls::CdaiAddress(element) => element.fmt(f),
                PriceOracleProxyCalls::CethAddress(element) => element.fmt(f),
                PriceOracleProxyCalls::CsaiAddress(element) => element.fmt(f),
                PriceOracleProxyCalls::CusdcAddress(element) => element.fmt(f),
                PriceOracleProxyCalls::CusdtAddress(element) => element.fmt(f),
                PriceOracleProxyCalls::DaiOracleKey(element) => element.fmt(f),
                PriceOracleProxyCalls::GetUnderlyingPrice(element) => element.fmt(f),
                PriceOracleProxyCalls::Guardian(element) => element.fmt(f),
                PriceOracleProxyCalls::IsPriceOracle(element) => element.fmt(f),
                PriceOracleProxyCalls::SaiPrice(element) => element.fmt(f),
                PriceOracleProxyCalls::SetSaiPrice(element) => element.fmt(f),
                PriceOracleProxyCalls::UsdcOracleKey(element) => element.fmt(f),
                PriceOracleProxyCalls::V1PriceOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CdaiAddressCall> for PriceOracleProxyCalls {
        fn from(var: CdaiAddressCall) -> Self {
            PriceOracleProxyCalls::CdaiAddress(var)
        }
    }
    impl ::std::convert::From<CethAddressCall> for PriceOracleProxyCalls {
        fn from(var: CethAddressCall) -> Self {
            PriceOracleProxyCalls::CethAddress(var)
        }
    }
    impl ::std::convert::From<CsaiAddressCall> for PriceOracleProxyCalls {
        fn from(var: CsaiAddressCall) -> Self {
            PriceOracleProxyCalls::CsaiAddress(var)
        }
    }
    impl ::std::convert::From<CusdcAddressCall> for PriceOracleProxyCalls {
        fn from(var: CusdcAddressCall) -> Self {
            PriceOracleProxyCalls::CusdcAddress(var)
        }
    }
    impl ::std::convert::From<CusdtAddressCall> for PriceOracleProxyCalls {
        fn from(var: CusdtAddressCall) -> Self {
            PriceOracleProxyCalls::CusdtAddress(var)
        }
    }
    impl ::std::convert::From<DaiOracleKeyCall> for PriceOracleProxyCalls {
        fn from(var: DaiOracleKeyCall) -> Self {
            PriceOracleProxyCalls::DaiOracleKey(var)
        }
    }
    impl ::std::convert::From<GetUnderlyingPriceCall> for PriceOracleProxyCalls {
        fn from(var: GetUnderlyingPriceCall) -> Self {
            PriceOracleProxyCalls::GetUnderlyingPrice(var)
        }
    }
    impl ::std::convert::From<GuardianCall> for PriceOracleProxyCalls {
        fn from(var: GuardianCall) -> Self {
            PriceOracleProxyCalls::Guardian(var)
        }
    }
    impl ::std::convert::From<IsPriceOracleCall> for PriceOracleProxyCalls {
        fn from(var: IsPriceOracleCall) -> Self {
            PriceOracleProxyCalls::IsPriceOracle(var)
        }
    }
    impl ::std::convert::From<SaiPriceCall> for PriceOracleProxyCalls {
        fn from(var: SaiPriceCall) -> Self {
            PriceOracleProxyCalls::SaiPrice(var)
        }
    }
    impl ::std::convert::From<SetSaiPriceCall> for PriceOracleProxyCalls {
        fn from(var: SetSaiPriceCall) -> Self {
            PriceOracleProxyCalls::SetSaiPrice(var)
        }
    }
    impl ::std::convert::From<UsdcOracleKeyCall> for PriceOracleProxyCalls {
        fn from(var: UsdcOracleKeyCall) -> Self {
            PriceOracleProxyCalls::UsdcOracleKey(var)
        }
    }
    impl ::std::convert::From<V1PriceOracleCall> for PriceOracleProxyCalls {
        fn from(var: V1PriceOracleCall) -> Self {
            PriceOracleProxyCalls::V1PriceOracle(var)
        }
    }
    #[doc = "Container type for all return fields from the `cDaiAddress` function with signature `cDaiAddress()` and selector `[242, 198, 91, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CdaiAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cEthAddress` function with signature `cEthAddress()` and selector `[46, 213, 142, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CethAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cSaiAddress` function with signature `cSaiAddress()` and selector `[33, 180, 145, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CsaiAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cUsdcAddress` function with signature `cUsdcAddress()` and selector `[255, 17, 67, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CusdcAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cUsdtAddress` function with signature `cUsdtAddress()` and selector `[26, 60, 178, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CusdtAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `daiOracleKey` function with signature `daiOracleKey()` and selector `[229, 238, 15, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DaiOracleKeyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getUnderlyingPrice` function with signature `getUnderlyingPrice(address)` and selector `[252, 87, 212, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUnderlyingPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `guardian` function with signature `guardian()` and selector `[69, 42, 147, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GuardianReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isPriceOracle` function with signature `isPriceOracle()` and selector `[102, 51, 27, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsPriceOracleReturn(pub bool);
    #[doc = "Container type for all return fields from the `saiPrice` function with signature `saiPrice()` and selector `[168, 107, 25, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SaiPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `usdcOracleKey` function with signature `usdcOracleKey()` and selector `[62, 118, 242, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UsdcOracleKeyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `v1PriceOracle` function with signature `v1PriceOracle()` and selector `[254, 16, 201, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct V1PriceOracleReturn(pub ethers::core::types::Address);
}
