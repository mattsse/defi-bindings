pub use aaveoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod aaveoracle_mod {
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
    #[doc = "AaveOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"sources\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"fallbackOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"baseCurrency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseCurrencyUnit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"source\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AssetSourceUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"baseCurrency\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"baseCurrencyUnit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BaseCurrencySet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fallbackOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FallbackOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BASE_CURRENCY_UNIT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetsPrices\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFallbackOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSourceOfAsset\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"sources\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssetSources\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fallbackOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFallbackOracle\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static AAVEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e06040523480156200001157600080fd5b5060405162000fb538038062000fb583398101604081905262000034916200034e565b6001600160a01b0386166080526200004c83620000ab565b620000588585620000f5565b6001600160a01b03821660a081905260c08290526040518281527fe27c4c1372396a3d15a9922f74f9dfc7c72b1ad6d63868470787249c356454c19060200160405180910390a25050505050506200049a565b600180546001600160a01b0319166001600160a01b0383169081179091556040517fce7a780d33665b1ea097af5f155e3821b809ecbaa839d3b33aa83ba28168cefb90600090a250565b8051825114604051806040016040528060028152602001611b9b60f11b815250906200013f5760405162461bcd60e51b815260040162000136919062000402565b60405180910390fd5b5060005b82518110156200025b578181815181106200016257620001626200045a565b60200260200101516000808584815181106200018257620001826200045a565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a8154816001600160a01b0302191690836001600160a01b03160217905550818181518110620001e357620001e36200045a565b60200260200101516001600160a01b03168382815181106200020957620002096200045a565b60200260200101516001600160a01b03167f22c5b7b2d8561d39f7f210b6b326a1aa69f15311163082308ac4877db6339dc160405160405180910390a380620002528162000470565b91505062000143565b505050565b6001600160a01b03811681146200027657600080fd5b50565b634e487b7160e01b600052604160045260246000fd5b80516200029c8162000260565b919050565b600082601f830112620002b357600080fd5b815160206001600160401b0380831115620002d257620002d262000279565b8260051b604051601f19603f83011681018181108482111715620002fa57620002fa62000279565b6040529384528581018301938381019250878511156200031957600080fd5b83870191505b84821015620003435762000333826200028f565b835291830191908301906200031f565b979650505050505050565b60008060008060008060c087890312156200036857600080fd5b8651620003758162000260565b60208801519096506001600160401b03808211156200039357600080fd5b620003a18a838b01620002a1565b96506040890151915080821115620003b857600080fd5b50620003c789828a01620002a1565b9450506060870151620003da8162000260565b6080880151909350620003ed8162000260565b8092505060a087015190509295509295509295565b600060208083528351808285015260005b81811015620004315785810183015185820160400152820162000413565b8181111562000444576000604083870101525b50601f01601f1916929092016040019392505050565b634e487b7160e01b600052603260045260246000fd5b60006000198214156200049357634e487b7160e01b600052601160045260246000fd5b5060010190565b60805160a05160c051610ad7620004de6000396000818161010701526103610152600081816101ae0152610336015260008181609d01526104f90152610ad76000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c806392bf2be01161006657806392bf2be0146101375780639d23d9f214610163578063abfd531014610183578063b3596f0714610196578063e19f4700146101a957600080fd5b80630542975c14610098578063170aee73146100dc5780636210308c146100f15780638c89b64f14610102575b600080fd5b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ef6100ea366004610844565b6101d0565b005b6001546001600160a01b03166100bf565b6101297f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020016100d3565b6100bf610145366004610844565b6001600160a01b039081166000908152602081905260409020541690565b6101766101713660046108ad565b6101e4565b6040516100d391906108ef565b6100ef610191366004610933565b610299565b6101296101a4366004610844565b610314565b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b6101d86104f5565b6101e181610696565b50565b606060008267ffffffffffffffff8111156102015761020161099f565b60405190808252806020026020018201604052801561022a578160200160208202803683370190505b50905060005b838110156102915761026285858381811061024d5761024d6109b5565b90506020020160208101906101a49190610844565b828281518110610274576102746109b5565b602090810291909101015280610289816109cb565b915050610230565b509392505050565b6102a16104f5565b61030e848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250506040805160208088028281018201909352878252909350879250869182918501908490808284376000920191909152506106e092505050565b50505050565b6001600160a01b038082166000818152602081905260408120549092908116917f0000000000000000000000000000000000000000000000000000000000000000909116141561038657507f000000000000000000000000000000000000000000000000000000000000000092915050565b6001600160a01b0381166104095760015460405163b3596f0760e01b81526001600160a01b0385811660048301529091169063b3596f0790602401602060405180830381865afa1580156103de573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061040291906109f4565b9392505050565b6000816001600160a01b03166350d25bcd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610449573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061046d91906109f4565b9050600081131561047f579392505050565b60015460405163b3596f0760e01b81526001600160a01b0386811660048301529091169063b3596f0790602401602060405180830381865afa1580156104c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104ed91906109f4565b949350505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663707cd7166040518163ffffffff1660e01b8152600401602060405180830381865afa158015610555573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105799190610a0d565b604051629f719760e51b81523360048201529091506001600160a01b038216906313ee32e090602401602060405180830381865afa1580156105bf573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e39190610a2a565b806106515750604051637be53ca160e01b81523360048201526001600160a01b03821690637be53ca190602401602060405180830381865afa15801561062d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106519190610a2a565b604051806040016040528060018152602001603560f81b815250906106925760405162461bcd60e51b81526004016106899190610a4c565b60405180910390fd5b5050565b600180546001600160a01b0319166001600160a01b0383169081179091556040517fce7a780d33665b1ea097af5f155e3821b809ecbaa839d3b33aa83ba28168cefb90600090a250565b8051825114604051806040016040528060028152602001611b9b60f11b8152509061071e5760405162461bcd60e51b81526004016106899190610a4c565b5060005b825181101561082a5781818151811061073d5761073d6109b5565b602002602001015160008085848151811061075a5761075a6109b5565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a8154816001600160a01b0302191690836001600160a01b031602179055508181815181106107b8576107b86109b5565b60200260200101516001600160a01b03168382815181106107db576107db6109b5565b60200260200101516001600160a01b03167f22c5b7b2d8561d39f7f210b6b326a1aa69f15311163082308ac4877db6339dc160405160405180910390a380610822816109cb565b915050610722565b505050565b6001600160a01b03811681146101e157600080fd5b60006020828403121561085657600080fd5b81356104028161082f565b60008083601f84011261087357600080fd5b50813567ffffffffffffffff81111561088b57600080fd5b6020830191508360208260051b85010111156108a657600080fd5b9250929050565b600080602083850312156108c057600080fd5b823567ffffffffffffffff8111156108d757600080fd5b6108e385828601610861565b90969095509350505050565b6020808252825182820181905260009190848201906040850190845b818110156109275783518352928401929184019160010161090b565b50909695505050505050565b6000806000806040858703121561094957600080fd5b843567ffffffffffffffff8082111561096157600080fd5b61096d88838901610861565b9096509450602087013591508082111561098657600080fd5b5061099387828801610861565b95989497509550505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60006000198214156109ed57634e487b7160e01b600052601160045260246000fd5b5060010190565b600060208284031215610a0657600080fd5b5051919050565b600060208284031215610a1f57600080fd5b81516104028161082f565b600060208284031215610a3c57600080fd5b8151801515811461040257600080fd5b600060208083528351808285015260005b81811015610a7957858101830151858201604001528201610a5d565b81811115610a8b576000604083870101525b50601f01601f191692909201604001939250505056fea264697066735822122085db3eba2ac28f37f5d5afcfa9927b15c799a70b2464c329c9966fd9ddee1a2564736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct AaveOracle<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AaveOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AaveOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), AAVEORACLE_ABI.clone(), client).into()
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
                AAVEORACLE_ABI.clone(),
                AAVEORACLE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `BASE_CURRENCY` (0xe19f4700) function"]
        pub fn base_currency(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([225, 159, 71, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BASE_CURRENCY_UNIT` (0x8c89b64f) function"]
        pub fn base_currency_unit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([140, 137, 182, 79], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetPrice` (0xb3596f07) function"]
        pub fn get_asset_price(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 89, 111, 7], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetsPrices` (0x9d23d9f2) function"]
        pub fn get_assets_prices(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([157, 35, 217, 242], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFallbackOracle` (0x6210308c) function"]
        pub fn get_fallback_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([98, 16, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSourceOfAsset` (0x92bf2be0) function"]
        pub fn get_source_of_asset(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([146, 191, 43, 224], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAssetSources` (0xabfd5310) function"]
        pub fn set_asset_sources(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            sources: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 253, 83, 16], (assets, sources))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFallbackOracle` (0x170aee73) function"]
        pub fn set_fallback_oracle(
            &self,
            fallback_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 10, 238, 115], fallback_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AssetSourceUpdated` event"]
        pub fn asset_source_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AssetSourceUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BaseCurrencySet` event"]
        pub fn base_currency_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BaseCurrencySetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FallbackOracleUpdated` event"]
        pub fn fallback_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FallbackOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AaveOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AaveOracle<M> {
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
        name = "AssetSourceUpdated",
        abi = "AssetSourceUpdated(address,address)"
    )]
    pub struct AssetSourceUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub source: ethers::core::types::Address,
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
    #[ethevent(name = "BaseCurrencySet", abi = "BaseCurrencySet(address,uint256)")]
    pub struct BaseCurrencySetFilter {
        #[ethevent(indexed)]
        pub base_currency: ethers::core::types::Address,
        pub base_currency_unit: ethers::core::types::U256,
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
    #[ethevent(name = "FallbackOracleUpdated", abi = "FallbackOracleUpdated(address)")]
    pub struct FallbackOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub fallback_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveOracleEvents {
        AssetSourceUpdatedFilter(AssetSourceUpdatedFilter),
        BaseCurrencySetFilter(BaseCurrencySetFilter),
        FallbackOracleUpdatedFilter(FallbackOracleUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for AaveOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AssetSourceUpdatedFilter::decode_log(log) {
                return Ok(AaveOracleEvents::AssetSourceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BaseCurrencySetFilter::decode_log(log) {
                return Ok(AaveOracleEvents::BaseCurrencySetFilter(decoded));
            }
            if let Ok(decoded) = FallbackOracleUpdatedFilter::decode_log(log) {
                return Ok(AaveOracleEvents::FallbackOracleUpdatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AaveOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveOracleEvents::AssetSourceUpdatedFilter(element) => element.fmt(f),
                AaveOracleEvents::BaseCurrencySetFilter(element) => element.fmt(f),
                AaveOracleEvents::FallbackOracleUpdatedFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `BASE_CURRENCY`function with signature `BASE_CURRENCY()` and selector `[225, 159, 71, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BASE_CURRENCY", abi = "BASE_CURRENCY()")]
    pub struct BaseCurrencyCall;
    #[doc = "Container type for all input parameters for the `BASE_CURRENCY_UNIT`function with signature `BASE_CURRENCY_UNIT()` and selector `[140, 137, 182, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BASE_CURRENCY_UNIT", abi = "BASE_CURRENCY_UNIT()")]
    pub struct BaseCurrencyUnitCall;
    #[doc = "Container type for all input parameters for the `getAssetPrice`function with signature `getAssetPrice(address)` and selector `[179, 89, 111, 7]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetPrice", abi = "getAssetPrice(address)")]
    pub struct GetAssetPriceCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getAssetsPrices`function with signature `getAssetsPrices(address[])` and selector `[157, 35, 217, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetsPrices", abi = "getAssetsPrices(address[])")]
    pub struct GetAssetsPricesCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getFallbackOracle`function with signature `getFallbackOracle()` and selector `[98, 16, 48, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getFallbackOracle", abi = "getFallbackOracle()")]
    pub struct GetFallbackOracleCall;
    #[doc = "Container type for all input parameters for the `getSourceOfAsset`function with signature `getSourceOfAsset(address)` and selector `[146, 191, 43, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSourceOfAsset", abi = "getSourceOfAsset(address)")]
    pub struct GetSourceOfAssetCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAssetSources`function with signature `setAssetSources(address[],address[])` and selector `[171, 253, 83, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAssetSources", abi = "setAssetSources(address[],address[])")]
    pub struct SetAssetSourcesCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub sources: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setFallbackOracle`function with signature `setFallbackOracle(address)` and selector `[23, 10, 238, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFallbackOracle", abi = "setFallbackOracle(address)")]
    pub struct SetFallbackOracleCall {
        pub fallback_oracle: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveOracleCalls {
        AddressesProvider(AddressesProviderCall),
        BaseCurrency(BaseCurrencyCall),
        BaseCurrencyUnit(BaseCurrencyUnitCall),
        GetAssetPrice(GetAssetPriceCall),
        GetAssetsPrices(GetAssetsPricesCall),
        GetFallbackOracle(GetFallbackOracleCall),
        GetSourceOfAsset(GetSourceOfAssetCall),
        SetAssetSources(SetAssetSourcesCall),
        SetFallbackOracle(SetFallbackOracleCall),
    }
    impl ethers::core::abi::AbiDecode for AaveOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <BaseCurrencyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::BaseCurrency(decoded));
            }
            if let Ok(decoded) =
                <BaseCurrencyUnitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::BaseCurrencyUnit(decoded));
            }
            if let Ok(decoded) =
                <GetAssetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::GetAssetPrice(decoded));
            }
            if let Ok(decoded) =
                <GetAssetsPricesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::GetAssetsPrices(decoded));
            }
            if let Ok(decoded) =
                <GetFallbackOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::GetFallbackOracle(decoded));
            }
            if let Ok(decoded) =
                <GetSourceOfAssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::GetSourceOfAsset(decoded));
            }
            if let Ok(decoded) =
                <SetAssetSourcesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::SetAssetSources(decoded));
            }
            if let Ok(decoded) =
                <SetFallbackOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveOracleCalls::SetFallbackOracle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveOracleCalls::AddressesProvider(element) => element.encode(),
                AaveOracleCalls::BaseCurrency(element) => element.encode(),
                AaveOracleCalls::BaseCurrencyUnit(element) => element.encode(),
                AaveOracleCalls::GetAssetPrice(element) => element.encode(),
                AaveOracleCalls::GetAssetsPrices(element) => element.encode(),
                AaveOracleCalls::GetFallbackOracle(element) => element.encode(),
                AaveOracleCalls::GetSourceOfAsset(element) => element.encode(),
                AaveOracleCalls::SetAssetSources(element) => element.encode(),
                AaveOracleCalls::SetFallbackOracle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveOracleCalls::AddressesProvider(element) => element.fmt(f),
                AaveOracleCalls::BaseCurrency(element) => element.fmt(f),
                AaveOracleCalls::BaseCurrencyUnit(element) => element.fmt(f),
                AaveOracleCalls::GetAssetPrice(element) => element.fmt(f),
                AaveOracleCalls::GetAssetsPrices(element) => element.fmt(f),
                AaveOracleCalls::GetFallbackOracle(element) => element.fmt(f),
                AaveOracleCalls::GetSourceOfAsset(element) => element.fmt(f),
                AaveOracleCalls::SetAssetSources(element) => element.fmt(f),
                AaveOracleCalls::SetFallbackOracle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for AaveOracleCalls {
        fn from(var: AddressesProviderCall) -> Self {
            AaveOracleCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<BaseCurrencyCall> for AaveOracleCalls {
        fn from(var: BaseCurrencyCall) -> Self {
            AaveOracleCalls::BaseCurrency(var)
        }
    }
    impl ::std::convert::From<BaseCurrencyUnitCall> for AaveOracleCalls {
        fn from(var: BaseCurrencyUnitCall) -> Self {
            AaveOracleCalls::BaseCurrencyUnit(var)
        }
    }
    impl ::std::convert::From<GetAssetPriceCall> for AaveOracleCalls {
        fn from(var: GetAssetPriceCall) -> Self {
            AaveOracleCalls::GetAssetPrice(var)
        }
    }
    impl ::std::convert::From<GetAssetsPricesCall> for AaveOracleCalls {
        fn from(var: GetAssetsPricesCall) -> Self {
            AaveOracleCalls::GetAssetsPrices(var)
        }
    }
    impl ::std::convert::From<GetFallbackOracleCall> for AaveOracleCalls {
        fn from(var: GetFallbackOracleCall) -> Self {
            AaveOracleCalls::GetFallbackOracle(var)
        }
    }
    impl ::std::convert::From<GetSourceOfAssetCall> for AaveOracleCalls {
        fn from(var: GetSourceOfAssetCall) -> Self {
            AaveOracleCalls::GetSourceOfAsset(var)
        }
    }
    impl ::std::convert::From<SetAssetSourcesCall> for AaveOracleCalls {
        fn from(var: SetAssetSourcesCall) -> Self {
            AaveOracleCalls::SetAssetSources(var)
        }
    }
    impl ::std::convert::From<SetFallbackOracleCall> for AaveOracleCalls {
        fn from(var: SetFallbackOracleCall) -> Self {
            AaveOracleCalls::SetFallbackOracle(var)
        }
    }
}
