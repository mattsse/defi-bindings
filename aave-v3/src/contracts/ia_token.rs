pub use iatoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod iatoken_mod {
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
    #[doc = "IAToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IATOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BalanceTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"aTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"aTokenName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"aTokenSymbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_TREASURY_ADDRESS\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_ASSET_ADDRESS\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiverOfUnderlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPreviousIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getScaledUserBalanceAndSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"handleRepayment\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"aTokenDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintToTreasury\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOnLiquidation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferUnderlyingTo\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IATOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IATOKEN_ABI.clone(), client).into()
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
                IATOKEN_ABI.clone(),
                IATOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_TREASURY_ADDRESS` (0xae167335) function"]
        pub fn reserve_treasury_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([174, 22, 115, 53], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNDERLYING_ASSET_ADDRESS` (0xb16a19de) function"]
        pub fn underlying_asset_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([177, 106, 25, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0xd7020d0a) function"]
        pub fn burn(
            &self,
            from: ethers::core::types::Address,
            receiver_of_underlying: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 2, 13, 10],
                    (from, receiver_of_underlying, amount, index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPreviousIndex` (0xe0753986) function"]
        pub fn get_previous_index(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 117, 57, 134], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getScaledUserBalanceAndSupply` (0x0afbcdc9) function"]
        pub fn get_scaled_user_balance_and_supply(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 251, 205, 201], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handleRepayment` (0x88dd91a1) function"]
        pub fn handle_repayment(
            &self,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 221, 145, 161], (user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x183fb413) function"]
        pub fn initialize(
            &self,
            pool: ethers::core::types::Address,
            treasury: ethers::core::types::Address,
            underlying_asset: ethers::core::types::Address,
            incentives_controller: ethers::core::types::Address,
            a_token_decimals: u8,
            a_token_name: String,
            a_token_symbol: String,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [24, 63, 180, 19],
                    (
                        pool,
                        treasury,
                        underlying_asset,
                        incentives_controller,
                        a_token_decimals,
                        a_token_name,
                        a_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xb3f1c93d) function"]
        pub fn mint(
            &self,
            caller: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 241, 201, 61], (caller, on_behalf_of, amount, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintToTreasury` (0x7df5bd3b) function"]
        pub fn mint_to_treasury(
            &self,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 245, 189, 59], (amount, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueTokens` (0xcea9d26f) function"]
        pub fn rescue_tokens(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 169, 210, 111], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledBalanceOf` (0x1da24f3e) function"]
        pub fn scaled_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 162, 79, 62], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledTotalSupply` (0xb1bf962d) function"]
        pub fn scaled_total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 191, 150, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOnLiquidation` (0xf866c319) function"]
        pub fn transfer_on_liquidation(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 102, 195, 25], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferUnderlyingTo` (0x4efecaa5) function"]
        pub fn transfer_underlying_to(
            &self,
            user: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 254, 202, 165], (user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BalanceTransfer` event"]
        pub fn balance_transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BalanceTransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IATokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAToken<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "BalanceTransfer",
        abi = "BalanceTransfer(address,address,uint256,uint256)"
    )]
    pub struct BalanceTransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
        abi = "Initialized(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
        pub treasury: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: String,
        pub a_token_symbol: String,
        pub params: ethers::core::types::Bytes,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IATokenEvents {
        ApprovalFilter(ApprovalFilter),
        BalanceTransferFilter(BalanceTransferFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for IATokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IATokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BalanceTransferFilter::decode_log(log) {
                return Ok(IATokenEvents::BalanceTransferFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IATokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(IATokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IATokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IATokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IATokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IATokenEvents::ApprovalFilter(element) => element.fmt(f),
                IATokenEvents::BalanceTransferFilter(element) => element.fmt(f),
                IATokenEvents::BurnFilter(element) => element.fmt(f),
                IATokenEvents::InitializedFilter(element) => element.fmt(f),
                IATokenEvents::MintFilter(element) => element.fmt(f),
                IATokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `RESERVE_TREASURY_ADDRESS`function with signature `RESERVE_TREASURY_ADDRESS()` and selector `[174, 22, 115, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_TREASURY_ADDRESS", abi = "RESERVE_TREASURY_ADDRESS()")]
    pub struct ReserveTreasuryAddressCall;
    #[doc = "Container type for all input parameters for the `UNDERLYING_ASSET_ADDRESS`function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "UNDERLYING_ASSET_ADDRESS", abi = "UNDERLYING_ASSET_ADDRESS()")]
    pub struct UnderlyingAssetAddressCall;
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(address,address,uint256,uint256)` and selector `[215, 2, 13, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,address,uint256,uint256)")]
    pub struct BurnCall {
        pub from: ethers::core::types::Address,
        pub receiver_of_underlying: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPreviousIndex`function with signature `getPreviousIndex(address)` and selector `[224, 117, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPreviousIndex", abi = "getPreviousIndex(address)")]
    pub struct GetPreviousIndexCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getScaledUserBalanceAndSupply`function with signature `getScaledUserBalanceAndSupply(address)` and selector `[10, 251, 205, 201]`"]
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
        name = "getScaledUserBalanceAndSupply",
        abi = "getScaledUserBalanceAndSupply(address)"
    )]
    pub struct GetScaledUserBalanceAndSupplyCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `handleRepayment`function with signature `handleRepayment(address,uint256)` and selector `[136, 221, 145, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "handleRepayment", abi = "handleRepayment(address,uint256)")]
    pub struct HandleRepaymentCall {
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address,address,address,uint8,string,string,bytes)` and selector `[24, 63, 180, 19]`"]
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
        name = "initialize",
        abi = "initialize(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub pool: ethers::core::types::Address,
        pub treasury: ethers::core::types::Address,
        pub underlying_asset: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: String,
        pub a_token_symbol: String,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,address,uint256,uint256)")]
    pub struct MintCall {
        pub caller: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintToTreasury`function with signature `mintToTreasury(uint256,uint256)` and selector `[125, 245, 189, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintToTreasury", abi = "mintToTreasury(uint256,uint256)")]
    pub struct MintToTreasuryCall {
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `rescueTokens`function with signature `rescueTokens(address,address,uint256)` and selector `[206, 169, 210, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rescueTokens", abi = "rescueTokens(address,address,uint256)")]
    pub struct RescueTokensCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `scaledBalanceOf`function with signature `scaledBalanceOf(address)` and selector `[29, 162, 79, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledBalanceOf", abi = "scaledBalanceOf(address)")]
    pub struct ScaledBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `scaledTotalSupply`function with signature `scaledTotalSupply()` and selector `[177, 191, 150, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledTotalSupply", abi = "scaledTotalSupply()")]
    pub struct ScaledTotalSupplyCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferOnLiquidation`function with signature `transferOnLiquidation(address,address,uint256)` and selector `[248, 102, 195, 25]`"]
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
        name = "transferOnLiquidation",
        abi = "transferOnLiquidation(address,address,uint256)"
    )]
    pub struct TransferOnLiquidationCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferUnderlyingTo`function with signature `transferUnderlyingTo(address,uint256)` and selector `[78, 254, 202, 165]`"]
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
        name = "transferUnderlyingTo",
        abi = "transferUnderlyingTo(address,uint256)"
    )]
    pub struct TransferUnderlyingToCall {
        pub user: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IATokenCalls {
        DomainSeparator(DomainSeparatorCall),
        ReserveTreasuryAddress(ReserveTreasuryAddressCall),
        UnderlyingAssetAddress(UnderlyingAssetAddressCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        GetPreviousIndex(GetPreviousIndexCall),
        GetScaledUserBalanceAndSupply(GetScaledUserBalanceAndSupplyCall),
        HandleRepayment(HandleRepaymentCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        MintToTreasury(MintToTreasuryCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        RescueTokens(RescueTokensCall),
        ScaledBalanceOf(ScaledBalanceOfCall),
        ScaledTotalSupply(ScaledTotalSupplyCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOnLiquidation(TransferOnLiquidationCall),
        TransferUnderlyingTo(TransferUnderlyingToCall),
    }
    impl ethers::core::abi::AbiDecode for IATokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <ReserveTreasuryAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::ReserveTreasuryAddress(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::UnderlyingAssetAddress(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IATokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <GetPreviousIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::GetPreviousIndex(decoded));
            }
            if let Ok(decoded) =
                <GetScaledUserBalanceAndSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IATokenCalls::GetScaledUserBalanceAndSupply(decoded));
            }
            if let Ok(decoded) =
                <HandleRepaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::HandleRepayment(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IATokenCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MintToTreasuryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::MintToTreasury(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <RescueTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::RescueTokens(decoded));
            }
            if let Ok(decoded) =
                <ScaledBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::ScaledBalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ScaledTotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::ScaledTotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOnLiquidationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::TransferOnLiquidation(decoded));
            }
            if let Ok(decoded) =
                <TransferUnderlyingToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IATokenCalls::TransferUnderlyingTo(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IATokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IATokenCalls::DomainSeparator(element) => element.encode(),
                IATokenCalls::ReserveTreasuryAddress(element) => element.encode(),
                IATokenCalls::UnderlyingAssetAddress(element) => element.encode(),
                IATokenCalls::Allowance(element) => element.encode(),
                IATokenCalls::Approve(element) => element.encode(),
                IATokenCalls::BalanceOf(element) => element.encode(),
                IATokenCalls::Burn(element) => element.encode(),
                IATokenCalls::GetPreviousIndex(element) => element.encode(),
                IATokenCalls::GetScaledUserBalanceAndSupply(element) => element.encode(),
                IATokenCalls::HandleRepayment(element) => element.encode(),
                IATokenCalls::Initialize(element) => element.encode(),
                IATokenCalls::Mint(element) => element.encode(),
                IATokenCalls::MintToTreasury(element) => element.encode(),
                IATokenCalls::Nonces(element) => element.encode(),
                IATokenCalls::Permit(element) => element.encode(),
                IATokenCalls::RescueTokens(element) => element.encode(),
                IATokenCalls::ScaledBalanceOf(element) => element.encode(),
                IATokenCalls::ScaledTotalSupply(element) => element.encode(),
                IATokenCalls::TotalSupply(element) => element.encode(),
                IATokenCalls::Transfer(element) => element.encode(),
                IATokenCalls::TransferFrom(element) => element.encode(),
                IATokenCalls::TransferOnLiquidation(element) => element.encode(),
                IATokenCalls::TransferUnderlyingTo(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IATokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IATokenCalls::DomainSeparator(element) => element.fmt(f),
                IATokenCalls::ReserveTreasuryAddress(element) => element.fmt(f),
                IATokenCalls::UnderlyingAssetAddress(element) => element.fmt(f),
                IATokenCalls::Allowance(element) => element.fmt(f),
                IATokenCalls::Approve(element) => element.fmt(f),
                IATokenCalls::BalanceOf(element) => element.fmt(f),
                IATokenCalls::Burn(element) => element.fmt(f),
                IATokenCalls::GetPreviousIndex(element) => element.fmt(f),
                IATokenCalls::GetScaledUserBalanceAndSupply(element) => element.fmt(f),
                IATokenCalls::HandleRepayment(element) => element.fmt(f),
                IATokenCalls::Initialize(element) => element.fmt(f),
                IATokenCalls::Mint(element) => element.fmt(f),
                IATokenCalls::MintToTreasury(element) => element.fmt(f),
                IATokenCalls::Nonces(element) => element.fmt(f),
                IATokenCalls::Permit(element) => element.fmt(f),
                IATokenCalls::RescueTokens(element) => element.fmt(f),
                IATokenCalls::ScaledBalanceOf(element) => element.fmt(f),
                IATokenCalls::ScaledTotalSupply(element) => element.fmt(f),
                IATokenCalls::TotalSupply(element) => element.fmt(f),
                IATokenCalls::Transfer(element) => element.fmt(f),
                IATokenCalls::TransferFrom(element) => element.fmt(f),
                IATokenCalls::TransferOnLiquidation(element) => element.fmt(f),
                IATokenCalls::TransferUnderlyingTo(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for IATokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            IATokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<ReserveTreasuryAddressCall> for IATokenCalls {
        fn from(var: ReserveTreasuryAddressCall) -> Self {
            IATokenCalls::ReserveTreasuryAddress(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetAddressCall> for IATokenCalls {
        fn from(var: UnderlyingAssetAddressCall) -> Self {
            IATokenCalls::UnderlyingAssetAddress(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for IATokenCalls {
        fn from(var: AllowanceCall) -> Self {
            IATokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for IATokenCalls {
        fn from(var: ApproveCall) -> Self {
            IATokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IATokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            IATokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for IATokenCalls {
        fn from(var: BurnCall) -> Self {
            IATokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<GetPreviousIndexCall> for IATokenCalls {
        fn from(var: GetPreviousIndexCall) -> Self {
            IATokenCalls::GetPreviousIndex(var)
        }
    }
    impl ::std::convert::From<GetScaledUserBalanceAndSupplyCall> for IATokenCalls {
        fn from(var: GetScaledUserBalanceAndSupplyCall) -> Self {
            IATokenCalls::GetScaledUserBalanceAndSupply(var)
        }
    }
    impl ::std::convert::From<HandleRepaymentCall> for IATokenCalls {
        fn from(var: HandleRepaymentCall) -> Self {
            IATokenCalls::HandleRepayment(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IATokenCalls {
        fn from(var: InitializeCall) -> Self {
            IATokenCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MintCall> for IATokenCalls {
        fn from(var: MintCall) -> Self {
            IATokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<MintToTreasuryCall> for IATokenCalls {
        fn from(var: MintToTreasuryCall) -> Self {
            IATokenCalls::MintToTreasury(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for IATokenCalls {
        fn from(var: NoncesCall) -> Self {
            IATokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for IATokenCalls {
        fn from(var: PermitCall) -> Self {
            IATokenCalls::Permit(var)
        }
    }
    impl ::std::convert::From<RescueTokensCall> for IATokenCalls {
        fn from(var: RescueTokensCall) -> Self {
            IATokenCalls::RescueTokens(var)
        }
    }
    impl ::std::convert::From<ScaledBalanceOfCall> for IATokenCalls {
        fn from(var: ScaledBalanceOfCall) -> Self {
            IATokenCalls::ScaledBalanceOf(var)
        }
    }
    impl ::std::convert::From<ScaledTotalSupplyCall> for IATokenCalls {
        fn from(var: ScaledTotalSupplyCall) -> Self {
            IATokenCalls::ScaledTotalSupply(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IATokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IATokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for IATokenCalls {
        fn from(var: TransferCall) -> Self {
            IATokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for IATokenCalls {
        fn from(var: TransferFromCall) -> Self {
            IATokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOnLiquidationCall> for IATokenCalls {
        fn from(var: TransferOnLiquidationCall) -> Self {
            IATokenCalls::TransferOnLiquidation(var)
        }
    }
    impl ::std::convert::From<TransferUnderlyingToCall> for IATokenCalls {
        fn from(var: TransferUnderlyingToCall) -> Self {
            IATokenCalls::TransferUnderlyingTo(var)
        }
    }
}
