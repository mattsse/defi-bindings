pub use simple_price_oracle::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod simple_price_oracle {
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
    #[doc = "SimplePriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIMPLEPRICEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"previousPriceMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"requestedPriceMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newPriceMantissa\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PricePosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetPrices\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUnderlyingPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPriceOracle\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDirectPrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract CToken\",\"name\":\"cToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"underlyingPriceMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUnderlyingPrice\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SIMPLEPRICEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506105ae806100206000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c806309a8acb01461005c578063127ffda01461008a5780635e9a523c146100b657806366331bba146100ee578063fc57d4df1461010a575b600080fd5b6100886004803603604081101561007257600080fd5b506001600160a01b038135169060200135610130565b005b610088600480360360408110156100a057600080fd5b506001600160a01b0381351690602001356101a8565b6100dc600480360360208110156100cc57600080fd5b50356001600160a01b031661028a565b60408051918252519081900360200190f35b6100f66102a9565b604080519115158252519081900360200190f35b6100dc6004803603602081101561012057600080fd5b50356001600160a01b03166102ae565b6001600160a01b038216600081815260208181526040918290205482519384529083015281810183905260608201839052517fdd71a1d19fcba687442a1d5c58578f1e409af71a79d10fd95a4d66efd8fa9ae79181900360800190a16001600160a01b03909116600090815260208190526040902055565b6000826001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b1580156101e357600080fd5b505afa1580156101f7573d6000803e3d6000fd5b505050506040513d602081101561020d57600080fd5b50516001600160a01b038116600081815260208181526040918290205482519384529083015281810185905260608201859052519192507fdd71a1d19fcba687442a1d5c58578f1e409af71a79d10fd95a4d66efd8fa9ae7919081900360800190a16001600160a01b031660009081526020819052604090205550565b6001600160a01b0381166000908152602081905260409020545b919050565b600181565b60006103f5826001600160a01b03166395d89b416040518163ffffffff1660e01b815260040160006040518083038186803b1580156102ec57600080fd5b505afa158015610300573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052602081101561032957600080fd5b810190808051604051939291908464010000000082111561034957600080fd5b90830190602082018581111561035e57600080fd5b825164010000000081118282018810171561037857600080fd5b82525081516020918201929091019080838360005b838110156103a557818101518382015260200161038d565b50505050905090810190601f1680156103d25780820380516001836020036101000a031916815260200191505b506040818101905260048152630c68aa8960e31b60208201529250610492915050565b156104095750670de0b6b3a76400006102a4565b600080836001600160a01b0316636f307dc36040518163ffffffff1660e01b815260040160206040518083038186803b15801561044557600080fd5b505afa158015610459573d6000803e3d6000fd5b505050506040513d602081101561046f57600080fd5b50516001600160a01b0316815260208101919091526040016000205490506102a4565b6000816040516020018082805190602001908083835b602083106104c75780518252601f1990920191602091820191016104a8565b6001836020036101000a03801982511681845116808217855250505050505090500191505060405160208183030381529060405280519060200120836040516020018082805190602001908083835b602083106105355780518252601f199092019160209182019101610516565b6001836020036101000a038019825116818451168082178552505050505050905001915050604051602081830303815290604052805190602001201490509291505056fea265627a7a723158209503bd3ae688611301296c806bfe41866fee361f0b2a644e146ac3c8678fc3e764736f6c63430005110032" . parse () . expect ("invalid bytecode")
        });
    pub struct SimplePriceOracle<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SimplePriceOracle<M> {
        fn clone(&self) -> Self {
            SimplePriceOracle(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SimplePriceOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for SimplePriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SimplePriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SimplePriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SIMPLEPRICEORACLE_ABI.clone(), client)
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
                SIMPLEPRICEORACLE_ABI.clone(),
                SIMPLEPRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `assetPrices` (0x5e9a523c) function"]
        pub fn asset_prices(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 154, 82, 60], asset)
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
        #[doc = "Calls the contract's `isPriceOracle` (0x66331bba) function"]
        pub fn is_price_oracle(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 51, 27, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDirectPrice` (0x09a8acb0) function"]
        pub fn set_direct_price(
            &self,
            asset: ethers::core::types::Address,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 168, 172, 176], (asset, price))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUnderlyingPrice` (0x127ffda0) function"]
        pub fn set_underlying_price(
            &self,
            c_token: ethers::core::types::Address,
            underlying_price_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 127, 253, 160], (c_token, underlying_price_mantissa))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PricePosted` event"]
        pub fn price_posted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PricePostedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PricePostedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for SimplePriceOracle<M>
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
        name = "PricePosted",
        abi = "PricePosted(address,uint256,uint256,uint256)"
    )]
    pub struct PricePostedFilter {
        pub asset: ethers::core::types::Address,
        pub previous_price_mantissa: ethers::core::types::U256,
        pub requested_price_mantissa: ethers::core::types::U256,
        pub new_price_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `assetPrices` function with signature `assetPrices(address)` and selector `[94, 154, 82, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assetPrices", abi = "assetPrices(address)")]
    pub struct AssetPricesCall {
        pub asset: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `setDirectPrice` function with signature `setDirectPrice(address,uint256)` and selector `[9, 168, 172, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDirectPrice", abi = "setDirectPrice(address,uint256)")]
    pub struct SetDirectPriceCall {
        pub asset: ethers::core::types::Address,
        pub price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setUnderlyingPrice` function with signature `setUnderlyingPrice(address,uint256)` and selector `[18, 127, 253, 160]`"]
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
        name = "setUnderlyingPrice",
        abi = "setUnderlyingPrice(address,uint256)"
    )]
    pub struct SetUnderlyingPriceCall {
        pub c_token: ethers::core::types::Address,
        pub underlying_price_mantissa: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SimplePriceOracleCalls {
        AssetPrices(AssetPricesCall),
        GetUnderlyingPrice(GetUnderlyingPriceCall),
        IsPriceOracle(IsPriceOracleCall),
        SetDirectPrice(SetDirectPriceCall),
        SetUnderlyingPrice(SetUnderlyingPriceCall),
    }
    impl ethers::core::abi::AbiDecode for SimplePriceOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetPricesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SimplePriceOracleCalls::AssetPrices(decoded));
            }
            if let Ok(decoded) =
                <GetUnderlyingPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SimplePriceOracleCalls::GetUnderlyingPrice(decoded));
            }
            if let Ok(decoded) =
                <IsPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SimplePriceOracleCalls::IsPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetDirectPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SimplePriceOracleCalls::SetDirectPrice(decoded));
            }
            if let Ok(decoded) =
                <SetUnderlyingPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SimplePriceOracleCalls::SetUnderlyingPrice(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SimplePriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SimplePriceOracleCalls::AssetPrices(element) => element.encode(),
                SimplePriceOracleCalls::GetUnderlyingPrice(element) => element.encode(),
                SimplePriceOracleCalls::IsPriceOracle(element) => element.encode(),
                SimplePriceOracleCalls::SetDirectPrice(element) => element.encode(),
                SimplePriceOracleCalls::SetUnderlyingPrice(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SimplePriceOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SimplePriceOracleCalls::AssetPrices(element) => element.fmt(f),
                SimplePriceOracleCalls::GetUnderlyingPrice(element) => element.fmt(f),
                SimplePriceOracleCalls::IsPriceOracle(element) => element.fmt(f),
                SimplePriceOracleCalls::SetDirectPrice(element) => element.fmt(f),
                SimplePriceOracleCalls::SetUnderlyingPrice(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetPricesCall> for SimplePriceOracleCalls {
        fn from(var: AssetPricesCall) -> Self {
            SimplePriceOracleCalls::AssetPrices(var)
        }
    }
    impl ::std::convert::From<GetUnderlyingPriceCall> for SimplePriceOracleCalls {
        fn from(var: GetUnderlyingPriceCall) -> Self {
            SimplePriceOracleCalls::GetUnderlyingPrice(var)
        }
    }
    impl ::std::convert::From<IsPriceOracleCall> for SimplePriceOracleCalls {
        fn from(var: IsPriceOracleCall) -> Self {
            SimplePriceOracleCalls::IsPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetDirectPriceCall> for SimplePriceOracleCalls {
        fn from(var: SetDirectPriceCall) -> Self {
            SimplePriceOracleCalls::SetDirectPrice(var)
        }
    }
    impl ::std::convert::From<SetUnderlyingPriceCall> for SimplePriceOracleCalls {
        fn from(var: SetUnderlyingPriceCall) -> Self {
            SimplePriceOracleCalls::SetUnderlyingPrice(var)
        }
    }
    #[doc = "Container type for all return fields from the `assetPrices` function with signature `assetPrices(address)` and selector `[94, 154, 82, 60]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetPricesReturn(pub ethers::core::types::U256);
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
}
