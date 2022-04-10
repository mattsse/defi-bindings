pub use mapleloaninitializer_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleloaninitializer_mod {
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
    #[doc = "MapleLoanInitializer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLELOANINITIALIZER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowerAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralPosted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CollateralRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EstablishmentFeesSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nextPaymentDueDate_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Funded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsDrawnDown\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FundsRedirected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundsReturned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LenderAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LoanClosed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsAccepted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsProposed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"refinanceCommitment_\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewTermsRejected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"principalPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"interestPaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"delegateFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"treasuryFeePaid_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMade\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingBorrower_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingBorrowerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingLender_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PendingLenderSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Repossessed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Skimmed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"encodedArguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decodeArguments\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[2]\",\"name\":\"assets_\",\"type\":\"address[2]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"termDetails_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[3]\",\"name\":\"amounts_\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"uint256[4]\",\"name\":\"rates_\",\"type\":\"uint256[4]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"encodeArguments\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"encodedArguments_\",\"type\":\"bytes\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLELOANINITIALIZER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610a6e806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063169acd1e146100f45780631725fa1d1461011d575b6000806000806000610049600036610141565b945094509450945094506100608585858585610182565b846001600160a01b03167f51d92f21bca73d047d43c1f073eca3ab1cd5d5c7d6902a75130562b7ace294398585858560405161009f9493929190610925565b60405180910390a27ffe9a32948c4b8ec5c8a8eddeacd3f3621458e8bde95b725b625e5c8f4f2cb54d6017546018546040516100e5929190918252602082015260400190565b60405180910390a15050505050005b610107610102366004610765565b610323565b604051610114919061095c565b60405180910390f35b61013061012b3660046107d2565b610141565b6040516101149594939291906108d5565b600061014b610565565b610153610583565b61015b610583565b6101636105a1565b61016f86880188610765565b939b929a50909850965090945092505050565b60208201516101d85760405162461bcd60e51b815260206004820152601760248201527f4d4c493a493a494e56414c49445f5052494e434950414c00000000000000000060448201526064015b60405180910390fd5b6020820151604083015111156102305760405162461bcd60e51b815260206004820152601e60248201527f4d4c493a493a494e56414c49445f454e44494e475f5052494e434950414c000060448201526064016101cf565b600080546001600160a01b0319166001600160a01b0387169081179091556102935760405162461bcd60e51b815260206004820152601660248201527526a6249d249d24a72b20a624a22fa127a92927aba2a960511b60448201526064016101cf565b8351600480546001600160a01b03199081166001600160a01b039384161790915560208087015160058054909316931692909217905583516006558381015160078190556040808601516013558451600c5584830151600d81905585820151600e55845160085592840151600955830151600a556060830151600b5561031c9190600080610359565b5050505050565b6060858585858560405160200161033e9594939291906108d5565b60405160208183030381529060405290505b95945050505050565b60006103636104b7565b90508264496cebb80085836001600160a01b03166316a12d7a6040518163ffffffff1660e01b815260040160206040518083038186803b1580156103a657600080fd5b505afa1580156103ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103de9190610844565b6103e890896109eb565b6103f291906109eb565b6103fc91906109c9565b61040691906109b1565b6017819055508164496cebb80085836001600160a01b031663cc32d1766040518163ffffffff1660e01b815260040160206040518083038186803b15801561044d57600080fd5b505afa158015610461573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104859190610844565b61048f90896109eb565b61049991906109eb565b6104a391906109c9565b6104ad91906109b1565b6018555050505050565b60006104c1610536565b6001600160a01b0316633a60339a6040518163ffffffff1660e01b815260040160206040518083038186803b1580156104f957600080fd5b505afa15801561050d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105319190610741565b905090565b60006105607f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15490565b919050565b60405180604001604052806002906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b600082601f8301126105d057600080fd5b6040516040810181811067ffffffffffffffff8211171561060157634e487b7160e01b600052604160045260246000fd5b806040525080838560408601111561061857600080fd5b60005b600281101561064457813561062f81610a20565b8352602092830192919091019060010161061b565b509195945050505050565b600082601f83011261066057600080fd5b6040516060810181811067ffffffffffffffff8211171561069157634e487b7160e01b600052604160045260246000fd5b6040528083606081018610156106a657600080fd5b60005b60038110156106445781358352602092830192909101906001016106a9565b600082601f8301126106d957600080fd5b6040516080810181811067ffffffffffffffff8211171561070a57634e487b7160e01b600052604160045260246000fd5b60405280836080810186101561071f57600080fd5b60005b6004811015610644578135835260209283019290910190600101610722565b60006020828403121561075357600080fd5b815161075e81610a20565b9392505050565b60008060008060006101a0868803121561077e57600080fd5b853561078981610a20565b945061079887602088016105bf565b93506107a7876060880161064f565b92506107b68760c0880161064f565b91506107c68761012088016106c8565b90509295509295909350565b600080602083850312156107e557600080fd5b823567ffffffffffffffff808211156107fd57600080fd5b818501915085601f83011261081157600080fd5b81358181111561082057600080fd5b86602082850101111561083257600080fd5b60209290920196919550909350505050565b60006020828403121561085657600080fd5b5051919050565b8060005b60028110156108895781516001600160a01b0316845260209384019390910190600101610861565b50505050565b8060005b6003811015610889578151845260209384019390910190600101610893565b8060005b60048110156108895781518452602093840193909101906001016108b6565b6001600160a01b03861681526101a081016108f3602083018761085d565b610900606083018661088f565b61090d60c083018561088f565b61091b6101208301846108b2565b9695505050505050565b6101808101610934828761085d565b610941604083018661088f565b61094e60a083018561088f565b6103506101008301846108b2565b600060208083528351808285015260005b818110156109895785810183015185820160400152820161096d565b8181111561099b576000604083870101525b50601f01601f1916929092016040019392505050565b600082198211156109c4576109c4610a0a565b500190565b6000826109e657634e487b7160e01b600052601260045260246000fd5b500490565b6000816000190483118215151615610a0557610a05610a0a565b500290565b634e487b7160e01b600052601160045260246000fd5b6001600160a01b0381168114610a3557600080fd5b5056fea2646970667358221220072a378273a6acf5dca58c9356c9474573bc2f77a4b5962665e82eaa7bc9964e64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MapleLoanInitializer<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleLoanInitializer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleLoanInitializer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleLoanInitializer))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleLoanInitializer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MAPLELOANINITIALIZER_ABI.clone(),
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
                MAPLELOANINITIALIZER_ABI.clone(),
                MAPLELOANINITIALIZER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `decodeArguments` (0x1725fa1d) function"]
        pub fn decode_arguments(
            &self,
            encoded_arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                [ethers::core::types::Address; 2usize],
                [ethers::core::types::U256; 3usize],
                [ethers::core::types::U256; 3usize],
                [ethers::core::types::U256; 4usize],
            ),
        > {
            self.0
                .method_hash([23, 37, 250, 29], encoded_arguments)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeArguments` (0x169acd1e) function"]
        pub fn encode_arguments(
            &self,
            borrower: ethers::core::types::Address,
            assets: [ethers::core::types::Address; 2usize],
            term_details: [ethers::core::types::U256; 3usize],
            amounts: [ethers::core::types::U256; 3usize],
            rates: [ethers::core::types::U256; 4usize],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [22, 154, 205, 30],
                    (borrower, assets, term_details, amounts, rates),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BorrowerAccepted` event"]
        pub fn borrower_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowerAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralPosted` event"]
        pub fn collateral_posted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralPostedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralRemoved` event"]
        pub fn collateral_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EstablishmentFeesSet` event"]
        pub fn establishment_fees_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EstablishmentFeesSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Funded` event"]
        pub fn funded_filter(&self) -> ethers::contract::builders::Event<M, FundedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsClaimed` event"]
        pub fn funds_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsDrawnDown` event"]
        pub fn funds_drawn_down_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsDrawnDownFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsRedirected` event"]
        pub fn funds_redirected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsRedirectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundsReturned` event"]
        pub fn funds_returned_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundsReturnedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LenderAccepted` event"]
        pub fn lender_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LenderAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LoanClosed` event"]
        pub fn loan_closed_filter(&self) -> ethers::contract::builders::Event<M, LoanClosedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsAccepted` event"]
        pub fn new_terms_accepted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsAcceptedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsProposed` event"]
        pub fn new_terms_proposed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsProposedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewTermsRejected` event"]
        pub fn new_terms_rejected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewTermsRejectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentMade` event"]
        pub fn payment_made_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentMadeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PendingBorrowerSet` event"]
        pub fn pending_borrower_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PendingBorrowerSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PendingLenderSet` event"]
        pub fn pending_lender_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PendingLenderSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repossessed` event"]
        pub fn repossessed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RepossessedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Skimmed` event"]
        pub fn skimmed_filter(&self) -> ethers::contract::builders::Event<M, SkimmedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MapleLoanInitializerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MapleLoanInitializer<M>
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
    #[ethevent(name = "BorrowerAccepted", abi = "BorrowerAccepted(address)")]
    pub struct BorrowerAcceptedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
    #[ethevent(name = "CollateralPosted", abi = "CollateralPosted(uint256)")]
    pub struct CollateralPostedFilter {
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
    #[ethevent(name = "CollateralRemoved", abi = "CollateralRemoved(uint256,address)")]
    pub struct CollateralRemovedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
        name = "EstablishmentFeesSet",
        abi = "EstablishmentFeesSet(uint256,uint256)"
    )]
    pub struct EstablishmentFeesSetFilter {
        pub delegate_fee: ethers::core::types::U256,
        pub treasury_fee: ethers::core::types::U256,
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
    #[ethevent(name = "Funded", abi = "Funded(address,uint256,uint256)")]
    pub struct FundedFilter {
        #[ethevent(indexed)]
        pub lender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub next_payment_due_date: ethers::core::types::U256,
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
    #[ethevent(name = "FundsClaimed", abi = "FundsClaimed(uint256,address)")]
    pub struct FundsClaimedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsDrawnDown", abi = "FundsDrawnDown(uint256,address)")]
    pub struct FundsDrawnDownFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsRedirected", abi = "FundsRedirected(uint256,address)")]
    pub struct FundsRedirectedFilter {
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "FundsReturned", abi = "FundsReturned(uint256)")]
    pub struct FundsReturnedFilter {
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
        name = "Initialized",
        abi = "Initialized(address,address[2],uint256[3],uint256[3],uint256[4])"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        pub assets: [ethers::core::types::Address; 2],
        pub term_details: [ethers::core::types::U256; 3],
        pub amounts: [ethers::core::types::U256; 3],
        pub rates: [ethers::core::types::U256; 4],
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
    #[ethevent(name = "LenderAccepted", abi = "LenderAccepted(address)")]
    pub struct LenderAcceptedFilter {
        #[ethevent(indexed)]
        pub lender: ethers::core::types::Address,
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
        name = "LoanClosed",
        abi = "LoanClosed(uint256,uint256,uint256,uint256)"
    )]
    pub struct LoanClosedFilter {
        pub principal_paid: ethers::core::types::U256,
        pub interest_paid: ethers::core::types::U256,
        pub delegate_fee_paid: ethers::core::types::U256,
        pub treasury_fee_paid: ethers::core::types::U256,
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
        name = "NewTermsAccepted",
        abi = "NewTermsAccepted(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsAcceptedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "NewTermsProposed",
        abi = "NewTermsProposed(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsProposedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "NewTermsRejected",
        abi = "NewTermsRejected(bytes32,address,uint256,bytes[])"
    )]
    pub struct NewTermsRejectedFilter {
        pub refinance_commitment: [u8; 32],
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: Vec<ethers::core::types::Bytes>,
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
        name = "PaymentMade",
        abi = "PaymentMade(uint256,uint256,uint256,uint256)"
    )]
    pub struct PaymentMadeFilter {
        pub principal_paid: ethers::core::types::U256,
        pub interest_paid: ethers::core::types::U256,
        pub delegate_fee_paid: ethers::core::types::U256,
        pub treasury_fee_paid: ethers::core::types::U256,
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
    #[ethevent(name = "PendingBorrowerSet", abi = "PendingBorrowerSet(address)")]
    pub struct PendingBorrowerSetFilter {
        pub pending_borrower: ethers::core::types::Address,
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
    #[ethevent(name = "PendingLenderSet", abi = "PendingLenderSet(address)")]
    pub struct PendingLenderSetFilter {
        pub pending_lender: ethers::core::types::Address,
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
    #[ethevent(name = "Repossessed", abi = "Repossessed(uint256,uint256,address)")]
    pub struct RepossessedFilter {
        pub collateral_repossessed: ethers::core::types::U256,
        pub funds_repossessed: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
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
    #[ethevent(name = "Skimmed", abi = "Skimmed(address,uint256,address)")]
    pub struct SkimmedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub destination: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleLoanInitializerEvents {
        BorrowerAcceptedFilter(BorrowerAcceptedFilter),
        CollateralPostedFilter(CollateralPostedFilter),
        CollateralRemovedFilter(CollateralRemovedFilter),
        EstablishmentFeesSetFilter(EstablishmentFeesSetFilter),
        FundedFilter(FundedFilter),
        FundsClaimedFilter(FundsClaimedFilter),
        FundsDrawnDownFilter(FundsDrawnDownFilter),
        FundsRedirectedFilter(FundsRedirectedFilter),
        FundsReturnedFilter(FundsReturnedFilter),
        InitializedFilter(InitializedFilter),
        LenderAcceptedFilter(LenderAcceptedFilter),
        LoanClosedFilter(LoanClosedFilter),
        NewTermsAcceptedFilter(NewTermsAcceptedFilter),
        NewTermsProposedFilter(NewTermsProposedFilter),
        NewTermsRejectedFilter(NewTermsRejectedFilter),
        PaymentMadeFilter(PaymentMadeFilter),
        PendingBorrowerSetFilter(PendingBorrowerSetFilter),
        PendingLenderSetFilter(PendingLenderSetFilter),
        RepossessedFilter(RepossessedFilter),
        SkimmedFilter(SkimmedFilter),
    }
    impl ethers::contract::EthLogDecode for MapleLoanInitializerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowerAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::BorrowerAcceptedFilter(decoded));
            }
            if let Ok(decoded) = CollateralPostedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::CollateralPostedFilter(decoded));
            }
            if let Ok(decoded) = CollateralRemovedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::CollateralRemovedFilter(decoded));
            }
            if let Ok(decoded) = EstablishmentFeesSetFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::EstablishmentFeesSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FundedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::FundedFilter(decoded));
            }
            if let Ok(decoded) = FundsClaimedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::FundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = FundsDrawnDownFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::FundsDrawnDownFilter(decoded));
            }
            if let Ok(decoded) = FundsRedirectedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::FundsRedirectedFilter(decoded));
            }
            if let Ok(decoded) = FundsReturnedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::FundsReturnedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = LenderAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::LenderAcceptedFilter(decoded));
            }
            if let Ok(decoded) = LoanClosedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::LoanClosedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsAcceptedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::NewTermsAcceptedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsProposedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::NewTermsProposedFilter(decoded));
            }
            if let Ok(decoded) = NewTermsRejectedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::NewTermsRejectedFilter(decoded));
            }
            if let Ok(decoded) = PaymentMadeFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::PaymentMadeFilter(decoded));
            }
            if let Ok(decoded) = PendingBorrowerSetFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::PendingBorrowerSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PendingLenderSetFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::PendingLenderSetFilter(decoded));
            }
            if let Ok(decoded) = RepossessedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::RepossessedFilter(decoded));
            }
            if let Ok(decoded) = SkimmedFilter::decode_log(log) {
                return Ok(MapleLoanInitializerEvents::SkimmedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MapleLoanInitializerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanInitializerEvents::BorrowerAcceptedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::CollateralPostedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::CollateralRemovedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::EstablishmentFeesSetFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::FundedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::FundsClaimedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::FundsDrawnDownFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::FundsRedirectedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::FundsReturnedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::InitializedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::LenderAcceptedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::LoanClosedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::NewTermsAcceptedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::NewTermsProposedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::NewTermsRejectedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::PaymentMadeFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::PendingBorrowerSetFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::PendingLenderSetFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::RepossessedFilter(element) => element.fmt(f),
                MapleLoanInitializerEvents::SkimmedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `decodeArguments`function with signature `decodeArguments(bytes)` and selector `[23, 37, 250, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decodeArguments", abi = "decodeArguments(bytes)")]
    pub struct DecodeArgumentsCall {
        pub encoded_arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `encodeArguments`function with signature `encodeArguments(address,address[2],uint256[3],uint256[3],uint256[4])` and selector `[22, 154, 205, 30]`"]
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
        name = "encodeArguments",
        abi = "encodeArguments(address,address[2],uint256[3],uint256[3],uint256[4])"
    )]
    pub struct EncodeArgumentsCall {
        pub borrower: ethers::core::types::Address,
        pub assets: [ethers::core::types::Address; 2usize],
        pub term_details: [ethers::core::types::U256; 3usize],
        pub amounts: [ethers::core::types::U256; 3usize],
        pub rates: [ethers::core::types::U256; 4usize],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleLoanInitializerCalls {
        DecodeArguments(DecodeArgumentsCall),
        EncodeArguments(EncodeArgumentsCall),
    }
    impl ethers::core::abi::AbiDecode for MapleLoanInitializerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DecodeArgumentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInitializerCalls::DecodeArguments(decoded));
            }
            if let Ok(decoded) =
                <EncodeArgumentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanInitializerCalls::EncodeArguments(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleLoanInitializerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleLoanInitializerCalls::DecodeArguments(element) => element.encode(),
                MapleLoanInitializerCalls::EncodeArguments(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleLoanInitializerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanInitializerCalls::DecodeArguments(element) => element.fmt(f),
                MapleLoanInitializerCalls::EncodeArguments(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DecodeArgumentsCall> for MapleLoanInitializerCalls {
        fn from(var: DecodeArgumentsCall) -> Self {
            MapleLoanInitializerCalls::DecodeArguments(var)
        }
    }
    impl ::std::convert::From<EncodeArgumentsCall> for MapleLoanInitializerCalls {
        fn from(var: EncodeArgumentsCall) -> Self {
            MapleLoanInitializerCalls::EncodeArguments(var)
        }
    }
}
