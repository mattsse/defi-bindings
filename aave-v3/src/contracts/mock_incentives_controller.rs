pub use mockincentivescontroller_mod::*;
#[allow(clippy::too_many_arguments)]
mod mockincentivescontroller_mod {
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
    #[doc = "MockIncentivesController was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKINCENTIVESCONTROLLER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"claimer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ClaimerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsAccrued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"claimer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"DISTRIBUTION_END\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"PRECISION\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"REWARD_TOKEN\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"assets\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"claimRewardsOnBehalf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureAssets\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getAssetData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getClaimer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRewardsBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getUserAssetData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getUserUnclaimedRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"handleAction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setClaimer\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKINCENTIVESCONTROLLER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610519806100206000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c806379f171b21161008c57806399248ea71161006657806399248ea714610203578063aaf5eb681461020a578063f11b818814610219578063f5cf673b1461025657600080fd5b806379f171b2146101d15780638b599f26146101e5578063919cd40f146101fc57600080fd5b806331873e2e116100c857806331873e2e146101615780633373ee4c146101765780636d34b96e1461018c57806374d945ec146101a557600080fd5b80631652e7b7146100ef578063198fa81e146101275780633111e7b314610149575b600080fd5b6101076100fd366004610284565b5060009081908190565b604080519384526020840192909252908201526060015b60405180910390f35b61013b610135366004610284565b50600090565b60405190815260200161011e565b61013b6101573660046102f2565b6000949350505050565b61017461016f36600461034f565b505050565b005b61013b610184366004610382565b600092915050565b61013b61019a3660046103b5565b600095945050505050565b6101b96101b3366004610284565b50600190565b6040516001600160a01b03909116815260200161011e565b6101746101df366004610423565b50505050565b61013b6101f336600461048f565b60009392505050565b600061013b565b60006101b9565b6040516000815260200161011e565b6102276100fd366004610284565b604080516fffffffffffffffffffffffffffffffff94851681529390921660208401529082015260600161011e565b610174610264366004610382565b5050565b80356001600160a01b038116811461027f57600080fd5b919050565b60006020828403121561029657600080fd5b61029f82610268565b9392505050565b60008083601f8401126102b857600080fd5b50813567ffffffffffffffff8111156102d057600080fd5b6020830191508360208260051b85010111156102eb57600080fd5b9250929050565b6000806000806060858703121561030857600080fd5b843567ffffffffffffffff81111561031f57600080fd5b61032b878288016102a6565b9095509350506020850135915061034460408601610268565b905092959194509250565b60008060006060848603121561036457600080fd5b61036d84610268565b95602085013595506040909401359392505050565b6000806040838503121561039557600080fd5b61039e83610268565b91506103ac60208401610268565b90509250929050565b6000806000806000608086880312156103cd57600080fd5b853567ffffffffffffffff8111156103e457600080fd5b6103f0888289016102a6565b9096509450506020860135925061040960408701610268565b915061041760608701610268565b90509295509295909350565b6000806000806040858703121561043957600080fd5b843567ffffffffffffffff8082111561045157600080fd5b61045d888389016102a6565b9096509450602087013591508082111561047657600080fd5b50610483878288016102a6565b95989497509550505050565b6000806000604084860312156104a457600080fd5b833567ffffffffffffffff8111156104bb57600080fd5b6104c7868287016102a6565b90945092506104da905060208501610268565b9050925092509256fea2646970667358221220ba9a7fbdfbe89ef26cab26425d798d50a1b13feb2ce5836568377e0b76e12aee64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct MockIncentivesController<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MockIncentivesController<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockIncentivesController<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockIncentivesController))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MockIncentivesController<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKINCENTIVESCONTROLLER_ABI.clone(),
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
                MOCKINCENTIVESCONTROLLER_ABI.clone(),
                MOCKINCENTIVESCONTROLLER_BYTECODE.clone().into(),
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
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (u128, u128, ethers::core::types::U256)>
        {
            self.0
                .method_hash([241, 27, 129, 136], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewards` (0x3111e7b3) function"]
        pub fn claim_rewards(
            &self,
            p0: ::std::vec::Vec<ethers::core::types::Address>,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([49, 17, 231, 179], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewardsOnBehalf` (0x6d34b96e) function"]
        pub fn claim_rewards_on_behalf(
            &self,
            p0: ::std::vec::Vec<ethers::core::types::Address>,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::Address,
            p3: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([109, 52, 185, 110], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureAssets` (0x79f171b2) function"]
        pub fn configure_assets(
            &self,
            p0: ::std::vec::Vec<ethers::core::types::Address>,
            p1: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 241, 113, 178], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAssetData` (0x1652e7b7) function"]
        pub fn get_asset_data(
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
                .method_hash([22, 82, 231, 183], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClaimer` (0x74d945ec) function"]
        pub fn get_claimer(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([116, 217, 69, 236], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewardsBalance` (0x8b599f26) function"]
        pub fn get_rewards_balance(
            &self,
            p0: ::std::vec::Vec<ethers::core::types::Address>,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 89, 159, 38], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserAssetData` (0x3373ee4c) function"]
        pub fn get_user_asset_data(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([51, 115, 238, 76], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserUnclaimedRewards` (0x198fa81e) function"]
        pub fn get_user_unclaimed_rewards(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([25, 143, 168, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handleAction` (0x31873e2e) function"]
        pub fn handle_action(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 135, 62, 46], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setClaimer` (0xf5cf673b) function"]
        pub fn set_claimer(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 207, 103, 59], (p0, p1))
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
        ) -> ethers::contract::builders::Event<M, MockIncentivesControllerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockIncentivesController<M>
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
        abi = "RewardsClaimed(address,address,uint256)"
    )]
    pub struct RewardsClaimedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockIncentivesControllerEvents {
        ClaimerSetFilter(ClaimerSetFilter),
        RewardsAccruedFilter(RewardsAccruedFilter),
        RewardsClaimedFilter(RewardsClaimedFilter),
    }
    impl ethers::contract::EthLogDecode for MockIncentivesControllerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ClaimerSetFilter::decode_log(log) {
                return Ok(MockIncentivesControllerEvents::ClaimerSetFilter(decoded));
            }
            if let Ok(decoded) = RewardsAccruedFilter::decode_log(log) {
                return Ok(MockIncentivesControllerEvents::RewardsAccruedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardsClaimedFilter::decode_log(log) {
                return Ok(MockIncentivesControllerEvents::RewardsClaimedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardsClaimedFilter::decode_log(log) {
                return Ok(MockIncentivesControllerEvents::RewardsClaimedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockIncentivesControllerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockIncentivesControllerEvents::ClaimerSetFilter(element) => element.fmt(f),
                MockIncentivesControllerEvents::RewardsAccruedFilter(element) => element.fmt(f),
                MockIncentivesControllerEvents::RewardsClaimedFilter(element) => element.fmt(f),
                MockIncentivesControllerEvents::RewardsClaimedFilter(element) => element.fmt(f),
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
    pub struct AssetsCall(pub ethers::core::types::Address);
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
    pub struct ClaimRewardsCall(
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
    );
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
    pub struct ClaimRewardsOnBehalfCall(
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
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
    pub struct ConfigureAssetsCall(
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ::std::vec::Vec<ethers::core::types::U256>,
    );
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
    pub struct GetAssetDataCall(pub ethers::core::types::Address);
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
    pub struct GetClaimerCall(pub ethers::core::types::Address);
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
    pub struct GetRewardsBalanceCall(
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
    );
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
    pub struct GetUserAssetDataCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
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
    pub struct GetUserUnclaimedRewardsCall(pub ethers::core::types::Address);
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
    pub struct HandleActionCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
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
    pub struct SetClaimerCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockIncentivesControllerCalls {
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
    impl ethers::core::abi::AbiDecode for MockIncentivesControllerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DistributionEndCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::DistributionEnd(decoded));
            }
            if let Ok(decoded) =
                <PrecisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::Precision(decoded));
            }
            if let Ok(decoded) =
                <RewardTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::RewardToken(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::Assets(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsOnBehalfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::ClaimRewardsOnBehalf(decoded));
            }
            if let Ok(decoded) =
                <ConfigureAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::ConfigureAssets(decoded));
            }
            if let Ok(decoded) =
                <GetAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::GetAssetData(decoded));
            }
            if let Ok(decoded) =
                <GetClaimerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::GetClaimer(decoded));
            }
            if let Ok(decoded) =
                <GetRewardsBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::GetRewardsBalance(decoded));
            }
            if let Ok(decoded) =
                <GetUserAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::GetUserAssetData(decoded));
            }
            if let Ok(decoded) =
                <GetUserUnclaimedRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::GetUserUnclaimedRewards(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HandleActionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::HandleAction(decoded));
            }
            if let Ok(decoded) =
                <SetClaimerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockIncentivesControllerCalls::SetClaimer(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockIncentivesControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockIncentivesControllerCalls::DistributionEnd(element) => element.encode(),
                MockIncentivesControllerCalls::Precision(element) => element.encode(),
                MockIncentivesControllerCalls::RewardToken(element) => element.encode(),
                MockIncentivesControllerCalls::Assets(element) => element.encode(),
                MockIncentivesControllerCalls::ClaimRewards(element) => element.encode(),
                MockIncentivesControllerCalls::ClaimRewardsOnBehalf(element) => element.encode(),
                MockIncentivesControllerCalls::ConfigureAssets(element) => element.encode(),
                MockIncentivesControllerCalls::GetAssetData(element) => element.encode(),
                MockIncentivesControllerCalls::GetClaimer(element) => element.encode(),
                MockIncentivesControllerCalls::GetRewardsBalance(element) => element.encode(),
                MockIncentivesControllerCalls::GetUserAssetData(element) => element.encode(),
                MockIncentivesControllerCalls::GetUserUnclaimedRewards(element) => element.encode(),
                MockIncentivesControllerCalls::HandleAction(element) => element.encode(),
                MockIncentivesControllerCalls::SetClaimer(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockIncentivesControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockIncentivesControllerCalls::DistributionEnd(element) => element.fmt(f),
                MockIncentivesControllerCalls::Precision(element) => element.fmt(f),
                MockIncentivesControllerCalls::RewardToken(element) => element.fmt(f),
                MockIncentivesControllerCalls::Assets(element) => element.fmt(f),
                MockIncentivesControllerCalls::ClaimRewards(element) => element.fmt(f),
                MockIncentivesControllerCalls::ClaimRewardsOnBehalf(element) => element.fmt(f),
                MockIncentivesControllerCalls::ConfigureAssets(element) => element.fmt(f),
                MockIncentivesControllerCalls::GetAssetData(element) => element.fmt(f),
                MockIncentivesControllerCalls::GetClaimer(element) => element.fmt(f),
                MockIncentivesControllerCalls::GetRewardsBalance(element) => element.fmt(f),
                MockIncentivesControllerCalls::GetUserAssetData(element) => element.fmt(f),
                MockIncentivesControllerCalls::GetUserUnclaimedRewards(element) => element.fmt(f),
                MockIncentivesControllerCalls::HandleAction(element) => element.fmt(f),
                MockIncentivesControllerCalls::SetClaimer(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DistributionEndCall> for MockIncentivesControllerCalls {
        fn from(var: DistributionEndCall) -> Self {
            MockIncentivesControllerCalls::DistributionEnd(var)
        }
    }
    impl ::std::convert::From<PrecisionCall> for MockIncentivesControllerCalls {
        fn from(var: PrecisionCall) -> Self {
            MockIncentivesControllerCalls::Precision(var)
        }
    }
    impl ::std::convert::From<RewardTokenCall> for MockIncentivesControllerCalls {
        fn from(var: RewardTokenCall) -> Self {
            MockIncentivesControllerCalls::RewardToken(var)
        }
    }
    impl ::std::convert::From<AssetsCall> for MockIncentivesControllerCalls {
        fn from(var: AssetsCall) -> Self {
            MockIncentivesControllerCalls::Assets(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for MockIncentivesControllerCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            MockIncentivesControllerCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsOnBehalfCall> for MockIncentivesControllerCalls {
        fn from(var: ClaimRewardsOnBehalfCall) -> Self {
            MockIncentivesControllerCalls::ClaimRewardsOnBehalf(var)
        }
    }
    impl ::std::convert::From<ConfigureAssetsCall> for MockIncentivesControllerCalls {
        fn from(var: ConfigureAssetsCall) -> Self {
            MockIncentivesControllerCalls::ConfigureAssets(var)
        }
    }
    impl ::std::convert::From<GetAssetDataCall> for MockIncentivesControllerCalls {
        fn from(var: GetAssetDataCall) -> Self {
            MockIncentivesControllerCalls::GetAssetData(var)
        }
    }
    impl ::std::convert::From<GetClaimerCall> for MockIncentivesControllerCalls {
        fn from(var: GetClaimerCall) -> Self {
            MockIncentivesControllerCalls::GetClaimer(var)
        }
    }
    impl ::std::convert::From<GetRewardsBalanceCall> for MockIncentivesControllerCalls {
        fn from(var: GetRewardsBalanceCall) -> Self {
            MockIncentivesControllerCalls::GetRewardsBalance(var)
        }
    }
    impl ::std::convert::From<GetUserAssetDataCall> for MockIncentivesControllerCalls {
        fn from(var: GetUserAssetDataCall) -> Self {
            MockIncentivesControllerCalls::GetUserAssetData(var)
        }
    }
    impl ::std::convert::From<GetUserUnclaimedRewardsCall> for MockIncentivesControllerCalls {
        fn from(var: GetUserUnclaimedRewardsCall) -> Self {
            MockIncentivesControllerCalls::GetUserUnclaimedRewards(var)
        }
    }
    impl ::std::convert::From<HandleActionCall> for MockIncentivesControllerCalls {
        fn from(var: HandleActionCall) -> Self {
            MockIncentivesControllerCalls::HandleAction(var)
        }
    }
    impl ::std::convert::From<SetClaimerCall> for MockIncentivesControllerCalls {
        fn from(var: SetClaimerCall) -> Self {
            MockIncentivesControllerCalls::SetClaimer(var)
        }
    }
}
