pub use iaaveincentivescontroller_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaaveincentivescontroller_mod {
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
    #[doc = "IAaveIncentivesController was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAAVEINCENTIVESCONTROLLER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"claimer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ClaimerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsAccrued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"claimer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DISTRIBUTION_END\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PRECISION\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REWARD_TOKEN\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewardsOnBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"emissionsPerSecond\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureAssets\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getClaimer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRewardsBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserAssetData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserUnclaimedRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"userBalance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalSupply\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"handleAction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"claimer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setClaimer\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IAAVEINCENTIVESCONTROLLER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAaveIncentivesController<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAaveIncentivesController<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAaveIncentivesController<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAaveIncentivesController))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAaveIncentivesController<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IAAVEINCENTIVESCONTROLLER_ABI.clone(),
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
                IAAVEINCENTIVESCONTROLLER_ABI.clone(),
                IAAVEINCENTIVESCONTROLLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DISTRIBUTION_END` (0x919cd40f) function"]
        pub fn distribution_end(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 156, 212, 15], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRECISION` (0xaaf5eb68) function"]
        pub fn precision(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([170, 245, 235, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `REWARD_TOKEN` (0x99248ea7) function"]
        pub fn reward_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([153, 36, 142, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assets` (0xf11b8188) function"]
        pub fn assets(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128, ethers::core::types::U256)>
        {
            self.0
                .method_hash([241, 27, 129, 136], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewards` (0x3111e7b3) function"]
        pub fn claim_rewards(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([49, 17, 231, 179], (assets, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewardsOnBehalf` (0x6d34b96e) function"]
        pub fn claim_rewards_on_behalf(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amount: ethers::core::types::U256,
            user: ethers::core::types::Address,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([109, 52, 185, 110], (assets, amount, user, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureAssets` (0x79f171b2) function"]
        pub fn configure_assets(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            emissions_per_second: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 241, 113, 178], (assets, emissions_per_second))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetData` (0x1652e7b7) function"]
        pub fn get_asset_data(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([22, 82, 231, 183], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClaimer` (0x74d945ec) function"]
        pub fn get_claimer(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([116, 217, 69, 236], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewardsBalance` (0x8b599f26) function"]
        pub fn get_rewards_balance(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 89, 159, 38], (assets, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserAssetData` (0x3373ee4c) function"]
        pub fn get_user_asset_data(
            &self,
            user: ethers::core::types::Address,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([51, 115, 238, 76], (user, asset))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserUnclaimedRewards` (0x198fa81e) function"]
        pub fn get_user_unclaimed_rewards(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([25, 143, 168, 30], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handleAction` (0x31873e2e) function"]
        pub fn handle_action(
            &self,
            asset: ethers::core::types::Address,
            user_balance: ethers::core::types::U256,
            total_supply: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 135, 62, 46], (asset, user_balance, total_supply))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setClaimer` (0xf5cf673b) function"]
        pub fn set_claimer(
            &self,
            user: ethers::core::types::Address,
            claimer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 207, 103, 59], (user, claimer))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ClaimerSet` event"]
        pub fn claimer_set_filter(&self) -> ethers::contract::builders::Event<M, ClaimerSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RewardsAccrued` event"]
        pub fn rewards_accrued_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RewardsAccruedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RewardsClaimed` event"]
        pub fn rewards_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RewardsClaimedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, IAaveIncentivesControllerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAaveIncentivesController<M>
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
    #[ethevent(name = "ClaimerSet", abi = "ClaimerSet(address,address)")]
    pub struct ClaimerSetFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub claimer: ethers::core::types::Address,
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
    #[ethevent(name = "RewardsAccrued", abi = "RewardsAccrued(address,uint256)")]
    pub struct RewardsAccruedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        name = "RewardsClaimed",
        abi = "RewardsClaimed(address,address,address,uint256)"
    )]
    pub struct RewardsClaimedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub claimer: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveIncentivesControllerEvents {
        ClaimerSetFilter(ClaimerSetFilter),
        RewardsAccruedFilter(RewardsAccruedFilter),
        RewardsClaimedFilter(RewardsClaimedFilter),
    }
    impl ethers::contract::EthLogDecode for IAaveIncentivesControllerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ClaimerSetFilter::decode_log(log) {
                return Ok(IAaveIncentivesControllerEvents::ClaimerSetFilter(decoded));
            }
            if let Ok(decoded) = RewardsAccruedFilter::decode_log(log) {
                return Ok(IAaveIncentivesControllerEvents::RewardsAccruedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardsClaimedFilter::decode_log(log) {
                return Ok(IAaveIncentivesControllerEvents::RewardsClaimedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardsClaimedFilter::decode_log(log) {
                return Ok(IAaveIncentivesControllerEvents::RewardsClaimedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAaveIncentivesControllerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveIncentivesControllerEvents::ClaimerSetFilter(element) => element.fmt(f),
                IAaveIncentivesControllerEvents::RewardsAccruedFilter(element) => element.fmt(f),
                IAaveIncentivesControllerEvents::RewardsClaimedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DISTRIBUTION_END`function with signature `DISTRIBUTION_END()` and selector `[145, 156, 212, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DISTRIBUTION_END", abi = "DISTRIBUTION_END()")]
    pub struct DistributionEndCall;
    #[doc = "Container type for all input parameters for the `PRECISION`function with signature `PRECISION()` and selector `[170, 245, 235, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PRECISION", abi = "PRECISION()")]
    pub struct PrecisionCall;
    #[doc = "Container type for all input parameters for the `REWARD_TOKEN`function with signature `REWARD_TOKEN()` and selector `[153, 36, 142, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "REWARD_TOKEN", abi = "REWARD_TOKEN()")]
    pub struct RewardTokenCall;
    #[doc = "Container type for all input parameters for the `assets`function with signature `assets(address)` and selector `[241, 27, 129, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assets", abi = "assets(address)")]
    pub struct AssetsCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address[],uint256,address)` and selector `[49, 17, 231, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(address[],uint256,address)")]
    pub struct ClaimRewardsCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimRewardsOnBehalf`function with signature `claimRewardsOnBehalf(address[],uint256,address,address)` and selector `[109, 52, 185, 110]`"]
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
        name = "claimRewardsOnBehalf",
        abi = "claimRewardsOnBehalf(address[],uint256,address,address)"
    )]
    pub struct ClaimRewardsOnBehalfCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `configureAssets`function with signature `configureAssets(address[],uint256[])` and selector `[121, 241, 113, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "configureAssets", abi = "configureAssets(address[],uint256[])")]
    pub struct ConfigureAssetsCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub emissions_per_second: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `getAssetData`function with signature `getAssetData(address)` and selector `[22, 82, 231, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAssetData", abi = "getAssetData(address)")]
    pub struct GetAssetDataCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getClaimer`function with signature `getClaimer(address)` and selector `[116, 217, 69, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getClaimer", abi = "getClaimer(address)")]
    pub struct GetClaimerCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getRewardsBalance`function with signature `getRewardsBalance(address[],address)` and selector `[139, 89, 159, 38]`"]
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
        name = "getRewardsBalance",
        abi = "getRewardsBalance(address[],address)"
    )]
    pub struct GetRewardsBalanceCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserAssetData`function with signature `getUserAssetData(address,address)` and selector `[51, 115, 238, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserAssetData", abi = "getUserAssetData(address,address)")]
    pub struct GetUserAssetDataCall {
        pub user: ethers::core::types::Address,
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserUnclaimedRewards`function with signature `getUserUnclaimedRewards(address)` and selector `[25, 143, 168, 30]`"]
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
        name = "getUserUnclaimedRewards",
        abi = "getUserUnclaimedRewards(address)"
    )]
    pub struct GetUserUnclaimedRewardsCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `handleAction`function with signature `handleAction(address,uint256,uint256)` and selector `[49, 135, 62, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "handleAction", abi = "handleAction(address,uint256,uint256)")]
    pub struct HandleActionCall {
        pub asset: ethers::core::types::Address,
        pub user_balance: ethers::core::types::U256,
        pub total_supply: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setClaimer`function with signature `setClaimer(address,address)` and selector `[245, 207, 103, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setClaimer", abi = "setClaimer(address,address)")]
    pub struct SetClaimerCall {
        pub user: ethers::core::types::Address,
        pub claimer: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAaveIncentivesControllerCalls {
        DistributionEnd(DistributionEndCall),
        Precision(PrecisionCall),
        RewardToken(RewardTokenCall),
        Assets(AssetsCall),
        ClaimRewards(ClaimRewardsCall),
        ClaimRewardsOnBehalf(ClaimRewardsOnBehalfCall),
        ConfigureAssets(ConfigureAssetsCall),
        GetAssetData(GetAssetDataCall),
        GetClaimer(GetClaimerCall),
        GetRewardsBalance(GetRewardsBalanceCall),
        GetUserAssetData(GetUserAssetDataCall),
        GetUserUnclaimedRewards(GetUserUnclaimedRewardsCall),
        HandleAction(HandleActionCall),
        SetClaimer(SetClaimerCall),
    }
    impl ethers::core::abi::AbiDecode for IAaveIncentivesControllerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DistributionEndCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::DistributionEnd(decoded));
            }
            if let Ok(decoded) =
                <PrecisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::Precision(decoded));
            }
            if let Ok(decoded) =
                <RewardTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::RewardToken(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsOnBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::ClaimRewardsOnBehalf(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ConfigureAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::ConfigureAssets(decoded));
            }
            if let Ok(decoded) =
                <GetAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::GetAssetData(decoded));
            }
            if let Ok(decoded) =
                <GetClaimerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::GetClaimer(decoded));
            }
            if let Ok(decoded) =
                <GetRewardsBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::GetRewardsBalance(decoded));
            }
            if let Ok(decoded) =
                <GetUserAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::GetUserAssetData(decoded));
            }
            if let Ok(decoded) =
                <GetUserUnclaimedRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::GetUserUnclaimedRewards(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HandleActionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::HandleAction(decoded));
            }
            if let Ok(decoded) =
                <SetClaimerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAaveIncentivesControllerCalls::SetClaimer(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAaveIncentivesControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAaveIncentivesControllerCalls::DistributionEnd(element) => element.encode(),
                IAaveIncentivesControllerCalls::Precision(element) => element.encode(),
                IAaveIncentivesControllerCalls::RewardToken(element) => element.encode(),
                IAaveIncentivesControllerCalls::Assets(element) => element.encode(),
                IAaveIncentivesControllerCalls::ClaimRewards(element) => element.encode(),
                IAaveIncentivesControllerCalls::ClaimRewardsOnBehalf(element) => element.encode(),
                IAaveIncentivesControllerCalls::ConfigureAssets(element) => element.encode(),
                IAaveIncentivesControllerCalls::GetAssetData(element) => element.encode(),
                IAaveIncentivesControllerCalls::GetClaimer(element) => element.encode(),
                IAaveIncentivesControllerCalls::GetRewardsBalance(element) => element.encode(),
                IAaveIncentivesControllerCalls::GetUserAssetData(element) => element.encode(),
                IAaveIncentivesControllerCalls::GetUserUnclaimedRewards(element) => {
                    element.encode()
                }
                IAaveIncentivesControllerCalls::HandleAction(element) => element.encode(),
                IAaveIncentivesControllerCalls::SetClaimer(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAaveIncentivesControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAaveIncentivesControllerCalls::DistributionEnd(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::Precision(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::RewardToken(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::Assets(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::ClaimRewards(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::ClaimRewardsOnBehalf(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::ConfigureAssets(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::GetAssetData(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::GetClaimer(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::GetRewardsBalance(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::GetUserAssetData(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::GetUserUnclaimedRewards(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::HandleAction(element) => element.fmt(f),
                IAaveIncentivesControllerCalls::SetClaimer(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DistributionEndCall> for IAaveIncentivesControllerCalls {
        fn from(var: DistributionEndCall) -> Self {
            IAaveIncentivesControllerCalls::DistributionEnd(var)
        }
    }
    impl ::std::convert::From<PrecisionCall> for IAaveIncentivesControllerCalls {
        fn from(var: PrecisionCall) -> Self {
            IAaveIncentivesControllerCalls::Precision(var)
        }
    }
    impl ::std::convert::From<RewardTokenCall> for IAaveIncentivesControllerCalls {
        fn from(var: RewardTokenCall) -> Self {
            IAaveIncentivesControllerCalls::RewardToken(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for IAaveIncentivesControllerCalls {
        fn from(var: AssetsCall) -> Self {
            IAaveIncentivesControllerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for IAaveIncentivesControllerCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            IAaveIncentivesControllerCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsOnBehalfCall> for IAaveIncentivesControllerCalls {
        fn from(var: ClaimRewardsOnBehalfCall) -> Self {
            IAaveIncentivesControllerCalls::ClaimRewardsOnBehalf(var)
        }
    }
    impl ::std::convert::From<ConfigureAssetsCall> for IAaveIncentivesControllerCalls {
        fn from(var: ConfigureAssetsCall) -> Self {
            IAaveIncentivesControllerCalls::ConfigureAssets(var)
        }
    }
    impl ::std::convert::From<GetAssetDataCall> for IAaveIncentivesControllerCalls {
        fn from(var: GetAssetDataCall) -> Self {
            IAaveIncentivesControllerCalls::GetAssetData(var)
        }
    }
    impl ::std::convert::From<GetClaimerCall> for IAaveIncentivesControllerCalls {
        fn from(var: GetClaimerCall) -> Self {
            IAaveIncentivesControllerCalls::GetClaimer(var)
        }
    }
    impl ::std::convert::From<GetRewardsBalanceCall> for IAaveIncentivesControllerCalls {
        fn from(var: GetRewardsBalanceCall) -> Self {
            IAaveIncentivesControllerCalls::GetRewardsBalance(var)
        }
    }
    impl ::std::convert::From<GetUserAssetDataCall> for IAaveIncentivesControllerCalls {
        fn from(var: GetUserAssetDataCall) -> Self {
            IAaveIncentivesControllerCalls::GetUserAssetData(var)
        }
    }
    impl ::std::convert::From<GetUserUnclaimedRewardsCall> for IAaveIncentivesControllerCalls {
        fn from(var: GetUserUnclaimedRewardsCall) -> Self {
            IAaveIncentivesControllerCalls::GetUserUnclaimedRewards(var)
        }
    }
    impl ::std::convert::From<HandleActionCall> for IAaveIncentivesControllerCalls {
        fn from(var: HandleActionCall) -> Self {
            IAaveIncentivesControllerCalls::HandleAction(var)
        }
    }
    impl ::std::convert::From<SetClaimerCall> for IAaveIncentivesControllerCalls {
        fn from(var: SetClaimerCall) -> Self {
            IAaveIncentivesControllerCalls::SetClaimer(var)
        }
    }
}
