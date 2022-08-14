pub use mock_reserve_configuration::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_reserve_configuration {
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
    #[doc = "MockReserveConfiguration was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKRESERVECONFIGURATION_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"configuration\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowCap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowingEnabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCaps\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDecimals\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEModeCategory\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFlags\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFrozen\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLiquidationBonus\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLiquidationProtocolFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLiquidationThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLtv\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getParams\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStableRateBorrowingEnabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyCap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUnbackedMintCap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowingEnabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDecimals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"categoryId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEModeCategory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"frozen\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFrozen\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"bonus\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidationBonus\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidationProtocolFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidationProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"threshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidationThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"ltv\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLtv\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reserveFactor\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveFactor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStableRateBorrowingEnabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"supplyCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSupplyCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"unbackedMintCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUnbackedMintCap\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKRESERVECONFIGURATION_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610c02806100206000396000f3fe608060405234801561001057600080fd5b50600436106101da5760003560e01c80638145bd2e11610104578063b6a3f59a116100a2578063ead8aa0211610071578063ead8aa02146104cc578063f0141d84146104ed578063f1514a1a1461050a578063fa573d071461051d57600080fd5b8063b6a3f59a14610469578063c37bdcec1461047c578063d0b0c8161461049a578063e08a28a3146104ad57600080fd5b80639d706d31116100de5780639d706d31146103e9578063a37e52e314610422578063a620063514610435578063aede7b761461044857600080fd5b80638145bd2e146103a85780638c8885c8146103c357806392dfb2fb146103d657600080fd5b80635f558e531161017c57806371cb13321161014b57806371cb1332146103385780637495b3531461034b57806379750bc4146103765780637e932d321461039557600080fd5b80635f558e53146102c55780636c70bee9146102e15780636cc7149d146102eb578063717186d11461032557600080fd5b8063356f235c116101b8578063356f235c146102375780634ae9b8bc1461025457806359aa9e72146102725780635e615a6b1461029057600080fd5b80631c446983146101df57806320361814146101f457806328842d4f14610224575b600080fd5b6101f26101ed366004610b35565b610530565b005b60408051602081019091526000549081905260741c640fffffffff165b6040519081526020015b60405180910390f35b6101f2610232366004610b35565b610551565b60408051602081019091526000549081905260a81c60ff16610211565b60408051602081019091526000549081905260101c61ffff16610211565b60408051602080820190925260005490819052901c61ffff16610211565b61029861056b565b604080519687526020870195909552938501929092526060840152608083015260a082015260c00161021b565b6040805160208101825260005490819052901c61ffff16610211565b6000546102119081565b6102f36105d4565b60408051951515865293151560208601529115159284019290925290151560608301521515608082015260a00161021b565b6101f2610333366004610b35565b610643565b6101f2610346366004610b4e565b61065d565b604080516020810190915260005490819052600160391b1615155b604051901515815260200161021b565b6040805160208101909152600054908190526001603a1b161515610366565b6101f26103a3366004610b4e565b610677565b60408051602081019091526000549081905261ffff16610211565b6101f26103d1366004610b35565b610691565b6101f26103e4366004610b35565b6106ab565b604080516020810190915260005490819052640fffffffff605082901c81169160741c166040805192835260208301919091520161021b565b6101f2610430366004610b35565b6106c5565b6101f2610443366004610b35565b6106df565b60408051602081019091526000549081905260501c640fffffffff16610211565b6101f2610477366004610b35565b6106f9565b60408051602081019091526000549081905260981c61ffff16610211565b6101f26104a8366004610b35565b610713565b6040805160208101909152600054908190526001603b1b161515610366565b60408051602081019091526000549081905260b01c640fffffffff16610211565b60408051602081019091526000549081905260301c60ff16610211565b6101f2610518366004610b4e565b61072d565b6101f261052b366004610b35565b610747565b6040805160208101909152600054815261054a8183610761565b5160005550565b6040805160208101909152600054815261054a81836107c1565b6000806000806000806105c160006040518060200160405290816000820154815250505161ffff80821692601083901c821692602081901c831692603082901c60ff90811693604084901c9092169260a81c1690565b949b939a50919850965094509092509050565b60008060008060006106326000604051806020016040529081600082015481525050516701000000000000008116151591600160391b82161515916001603a1b81161515916001603b1b821615159167100000000000000016151590565b945094509450945094509091929394565b6040805160208101909152600054815261054a8183610814565b6040805160208101909152600054815261054a818361086c565b6040805160208101909152600054815261054a818361089a565b6040805160208101909152600054815261054a81836108c8565b6040805160208101909152600054815261054a818361091b565b6040805160208101909152600054815261054a8183610973565b6040805160208101909152600054815261054a81836109bc565b6040805160208101909152600054815261054a8183610a0e565b6040805160208101909152600054815261054a8183610a66565b6040805160208101909152600054815261054a8183610ab7565b6040805160208101909152600054815261054a8183610ae5565b604080518082019091526002815261363760f01b602082015261ffff8211156107a65760405162461bcd60e51b815260040161079d9190610b77565b60405180910390fd5b50815169ffff0000000000000000191660409190911b179052565b604080518082019091526002815261363560f01b602082015261ffff8211156107fd5760405162461bcd60e51b815260040161079d9190610b77565b50815165ffff00000000191660209190911b179052565b60408051808201909152600281526106c760f31b6020820152640fffffffff8211156108535760405162461bcd60e51b815260040161079d9190610b77565b508151640fffffffff60501b191660509190911b179052565b603b8161087a57600061087d565b60015b8351670800000000000000191660ff9190911690911b1790915250565b6039816108a85760006108ab565b60015b8351670200000000000000191660ff9190911690911b1790915250565b6040805180820190915260028152611b1b60f11b602082015260ff8211156109035760405162461bcd60e51b815260040161079d9190610b77565b50815166ff000000000000191660309190911b179052565b6040805180820190915260028152611b9960f11b6020820152640fffffffff82111561095a5760405162461bcd60e51b815260040161079d9190610b77565b508151640fffffffff60b01b191660b09190911b179052565b604080518082019091526002815261363360f01b602082015261ffff8211156109af5760405162461bcd60e51b815260040161079d9190610b77565b50815161ffff1916179052565b604080518082019091526002815261037360f41b602082015261ffff8211156109f85760405162461bcd60e51b815260040161079d9190610b77565b50815161ffff60981b191660989190911b179052565b604080518082019091526002815261363960f01b6020820152640fffffffff821115610a4d5760405162461bcd60e51b815260040161079d9190610b77565b508151640fffffffff60741b191660749190911b179052565b6040805180820190915260028152610d8d60f21b602082015261ffff821115610aa25760405162461bcd60e51b815260040161079d9190610b77565b50815163ffff0000191660109190911b179052565b603a81610ac5576000610ac8565b60015b8351670400000000000000191660ff9190911690911b1790915250565b604080518082019091526002815261373160f01b602082015260ff821115610b205760405162461bcd60e51b815260040161079d9190610b77565b50815160ff60a81b191660a89190911b179052565b600060208284031215610b4757600080fd5b5035919050565b600060208284031215610b6057600080fd5b81358015158114610b7057600080fd5b9392505050565b600060208083528351808285015260005b81811015610ba457858101830151858201604001528201610b88565b81811115610bb6576000604083870101525b50601f01601f191692909201604001939250505056fea2646970667358221220ace953e8eddeed2ce21129d5769ad88e0070bf9d5c8c7471fe9c9eb69b81791a64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    pub struct MockReserveConfiguration<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockReserveConfiguration<M> {
        fn clone(&self) -> Self {
            MockReserveConfiguration(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockReserveConfiguration<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockReserveConfiguration<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockReserveConfiguration))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockReserveConfiguration<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                MOCKRESERVECONFIGURATION_ABI.clone(),
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MOCKRESERVECONFIGURATION_ABI.clone(),
                MOCKRESERVECONFIGURATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `configuration` (0x6c70bee9) function"]
        pub fn configuration(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 112, 190, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBorrowCap` (0xaede7b76) function"]
        pub fn get_borrow_cap(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([174, 222, 123, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBorrowingEnabled` (0x79750bc4) function"]
        pub fn get_borrowing_enabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([121, 117, 11, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCaps` (0x9d706d31) function"]
        pub fn get_caps(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 112, 109, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDecimals` (0xf0141d84) function"]
        pub fn get_decimals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([240, 20, 29, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEModeCategory` (0x356f235c) function"]
        pub fn get_e_mode_category(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([53, 111, 35, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFlags` (0x6cc7149d) function"]
        pub fn get_flags(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, (bool, bool, bool, bool, bool)> {
            self.0
                .method_hash([108, 199, 20, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFrozen` (0x7495b353) function"]
        pub fn get_frozen(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([116, 149, 179, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidationBonus` (0x59aa9e72) function"]
        pub fn get_liquidation_bonus(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([89, 170, 158, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidationProtocolFee` (0xc37bdcec) function"]
        pub fn get_liquidation_protocol_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([195, 123, 220, 236], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLiquidationThreshold` (0x4ae9b8bc) function"]
        pub fn get_liquidation_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([74, 233, 184, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLtv` (0x8145bd2e) function"]
        pub fn get_ltv(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([129, 69, 189, 46], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getParams` (0x5e615a6b) function"]
        pub fn get_params(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([94, 97, 90, 107], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveFactor` (0x5f558e53) function"]
        pub fn get_reserve_factor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 85, 142, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStableRateBorrowingEnabled` (0xe08a28a3) function"]
        pub fn get_stable_rate_borrowing_enabled(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 138, 40, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyCap` (0x20361814) function"]
        pub fn get_supply_cap(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([32, 54, 24, 20], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUnbackedMintCap` (0xead8aa02) function"]
        pub fn get_unbacked_mint_cap(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([234, 216, 170, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowCap` (0x717186d1) function"]
        pub fn set_borrow_cap(
            &self,
            borrow_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 113, 134, 209], borrow_cap)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowingEnabled` (0xf1514a1a) function"]
        pub fn set_borrowing_enabled(
            &self,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 81, 74, 26], enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDecimals` (0x8c8885c8) function"]
        pub fn set_decimals(
            &self,
            decimals: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 136, 133, 200], decimals)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEModeCategory` (0xfa573d07) function"]
        pub fn set_e_mode_category(
            &self,
            category_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 87, 61, 7], category_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFrozen` (0x7e932d32) function"]
        pub fn set_frozen(&self, frozen: bool) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 147, 45, 50], frozen)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationBonus` (0x28842d4f) function"]
        pub fn set_liquidation_bonus(
            &self,
            bonus: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 132, 45, 79], bonus)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationProtocolFee` (0xa6200635) function"]
        pub fn set_liquidation_protocol_fee(
            &self,
            liquidation_protocol_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 32, 6, 53], liquidation_protocol_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationThreshold` (0xd0b0c816) function"]
        pub fn set_liquidation_threshold(
            &self,
            threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 176, 200, 22], threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLtv` (0xa37e52e3) function"]
        pub fn set_ltv(
            &self,
            ltv: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 126, 82, 227], ltv)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveFactor` (0x1c446983) function"]
        pub fn set_reserve_factor(
            &self,
            reserve_factor: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 68, 105, 131], reserve_factor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStableRateBorrowingEnabled` (0x71cb1332) function"]
        pub fn set_stable_rate_borrowing_enabled(
            &self,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 203, 19, 50], enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSupplyCap` (0xb6a3f59a) function"]
        pub fn set_supply_cap(
            &self,
            supply_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 163, 245, 154], supply_cap)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUnbackedMintCap` (0x92dfb2fb) function"]
        pub fn set_unbacked_mint_cap(
            &self,
            unbacked_mint_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 223, 178, 251], unbacked_mint_cap)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockReserveConfiguration<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `configuration` function with signature `configuration()` and selector `[108, 112, 190, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "configuration", abi = "configuration()")]
    pub struct ConfigurationCall;
    #[doc = "Container type for all input parameters for the `getBorrowCap` function with signature `getBorrowCap()` and selector `[174, 222, 123, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowCap", abi = "getBorrowCap()")]
    pub struct GetBorrowCapCall;
    #[doc = "Container type for all input parameters for the `getBorrowingEnabled` function with signature `getBorrowingEnabled()` and selector `[121, 117, 11, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowingEnabled", abi = "getBorrowingEnabled()")]
    pub struct GetBorrowingEnabledCall;
    #[doc = "Container type for all input parameters for the `getCaps` function with signature `getCaps()` and selector `[157, 112, 109, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCaps", abi = "getCaps()")]
    pub struct GetCapsCall;
    #[doc = "Container type for all input parameters for the `getDecimals` function with signature `getDecimals()` and selector `[240, 20, 29, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDecimals", abi = "getDecimals()")]
    pub struct GetDecimalsCall;
    #[doc = "Container type for all input parameters for the `getEModeCategory` function with signature `getEModeCategory()` and selector `[53, 111, 35, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEModeCategory", abi = "getEModeCategory()")]
    pub struct GetEModeCategoryCall;
    #[doc = "Container type for all input parameters for the `getFlags` function with signature `getFlags()` and selector `[108, 199, 20, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getFlags", abi = "getFlags()")]
    pub struct GetFlagsCall;
    #[doc = "Container type for all input parameters for the `getFrozen` function with signature `getFrozen()` and selector `[116, 149, 179, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getFrozen", abi = "getFrozen()")]
    pub struct GetFrozenCall;
    #[doc = "Container type for all input parameters for the `getLiquidationBonus` function with signature `getLiquidationBonus()` and selector `[89, 170, 158, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLiquidationBonus", abi = "getLiquidationBonus()")]
    pub struct GetLiquidationBonusCall;
    #[doc = "Container type for all input parameters for the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee()` and selector `[195, 123, 220, 236]`"]
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
        name = "getLiquidationProtocolFee",
        abi = "getLiquidationProtocolFee()"
    )]
    pub struct GetLiquidationProtocolFeeCall;
    #[doc = "Container type for all input parameters for the `getLiquidationThreshold` function with signature `getLiquidationThreshold()` and selector `[74, 233, 184, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLiquidationThreshold", abi = "getLiquidationThreshold()")]
    pub struct GetLiquidationThresholdCall;
    #[doc = "Container type for all input parameters for the `getLtv` function with signature `getLtv()` and selector `[129, 69, 189, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLtv", abi = "getLtv()")]
    pub struct GetLtvCall;
    #[doc = "Container type for all input parameters for the `getParams` function with signature `getParams()` and selector `[94, 97, 90, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getParams", abi = "getParams()")]
    pub struct GetParamsCall;
    #[doc = "Container type for all input parameters for the `getReserveFactor` function with signature `getReserveFactor()` and selector `[95, 85, 142, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReserveFactor", abi = "getReserveFactor()")]
    pub struct GetReserveFactorCall;
    #[doc = "Container type for all input parameters for the `getStableRateBorrowingEnabled` function with signature `getStableRateBorrowingEnabled()` and selector `[224, 138, 40, 163]`"]
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
        name = "getStableRateBorrowingEnabled",
        abi = "getStableRateBorrowingEnabled()"
    )]
    pub struct GetStableRateBorrowingEnabledCall;
    #[doc = "Container type for all input parameters for the `getSupplyCap` function with signature `getSupplyCap()` and selector `[32, 54, 24, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSupplyCap", abi = "getSupplyCap()")]
    pub struct GetSupplyCapCall;
    #[doc = "Container type for all input parameters for the `getUnbackedMintCap` function with signature `getUnbackedMintCap()` and selector `[234, 216, 170, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUnbackedMintCap", abi = "getUnbackedMintCap()")]
    pub struct GetUnbackedMintCapCall;
    #[doc = "Container type for all input parameters for the `setBorrowCap` function with signature `setBorrowCap(uint256)` and selector `[113, 113, 134, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBorrowCap", abi = "setBorrowCap(uint256)")]
    pub struct SetBorrowCapCall {
        pub borrow_cap: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setBorrowingEnabled` function with signature `setBorrowingEnabled(bool)` and selector `[241, 81, 74, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBorrowingEnabled", abi = "setBorrowingEnabled(bool)")]
    pub struct SetBorrowingEnabledCall {
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setDecimals` function with signature `setDecimals(uint256)` and selector `[140, 136, 133, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDecimals", abi = "setDecimals(uint256)")]
    pub struct SetDecimalsCall {
        pub decimals: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setEModeCategory` function with signature `setEModeCategory(uint256)` and selector `[250, 87, 61, 7]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setEModeCategory", abi = "setEModeCategory(uint256)")]
    pub struct SetEModeCategoryCall {
        pub category_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFrozen` function with signature `setFrozen(bool)` and selector `[126, 147, 45, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setFrozen", abi = "setFrozen(bool)")]
    pub struct SetFrozenCall {
        pub frozen: bool,
    }
    #[doc = "Container type for all input parameters for the `setLiquidationBonus` function with signature `setLiquidationBonus(uint256)` and selector `[40, 132, 45, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLiquidationBonus", abi = "setLiquidationBonus(uint256)")]
    pub struct SetLiquidationBonusCall {
        pub bonus: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLiquidationProtocolFee` function with signature `setLiquidationProtocolFee(uint256)` and selector `[166, 32, 6, 53]`"]
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
        name = "setLiquidationProtocolFee",
        abi = "setLiquidationProtocolFee(uint256)"
    )]
    pub struct SetLiquidationProtocolFeeCall {
        pub liquidation_protocol_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLiquidationThreshold` function with signature `setLiquidationThreshold(uint256)` and selector `[208, 176, 200, 22]`"]
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
        name = "setLiquidationThreshold",
        abi = "setLiquidationThreshold(uint256)"
    )]
    pub struct SetLiquidationThresholdCall {
        pub threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setLtv` function with signature `setLtv(uint256)` and selector `[163, 126, 82, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLtv", abi = "setLtv(uint256)")]
    pub struct SetLtvCall {
        pub ltv: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReserveFactor` function with signature `setReserveFactor(uint256)` and selector `[28, 68, 105, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setReserveFactor", abi = "setReserveFactor(uint256)")]
    pub struct SetReserveFactorCall {
        pub reserve_factor: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStableRateBorrowingEnabled` function with signature `setStableRateBorrowingEnabled(bool)` and selector `[113, 203, 19, 50]`"]
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
        name = "setStableRateBorrowingEnabled",
        abi = "setStableRateBorrowingEnabled(bool)"
    )]
    pub struct SetStableRateBorrowingEnabledCall {
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setSupplyCap` function with signature `setSupplyCap(uint256)` and selector `[182, 163, 245, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSupplyCap", abi = "setSupplyCap(uint256)")]
    pub struct SetSupplyCapCall {
        pub supply_cap: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setUnbackedMintCap` function with signature `setUnbackedMintCap(uint256)` and selector `[146, 223, 178, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUnbackedMintCap", abi = "setUnbackedMintCap(uint256)")]
    pub struct SetUnbackedMintCapCall {
        pub unbacked_mint_cap: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockReserveConfigurationCalls {
        Configuration(ConfigurationCall),
        GetBorrowCap(GetBorrowCapCall),
        GetBorrowingEnabled(GetBorrowingEnabledCall),
        GetCaps(GetCapsCall),
        GetDecimals(GetDecimalsCall),
        GetEModeCategory(GetEModeCategoryCall),
        GetFlags(GetFlagsCall),
        GetFrozen(GetFrozenCall),
        GetLiquidationBonus(GetLiquidationBonusCall),
        GetLiquidationProtocolFee(GetLiquidationProtocolFeeCall),
        GetLiquidationThreshold(GetLiquidationThresholdCall),
        GetLtv(GetLtvCall),
        GetParams(GetParamsCall),
        GetReserveFactor(GetReserveFactorCall),
        GetStableRateBorrowingEnabled(GetStableRateBorrowingEnabledCall),
        GetSupplyCap(GetSupplyCapCall),
        GetUnbackedMintCap(GetUnbackedMintCapCall),
        SetBorrowCap(SetBorrowCapCall),
        SetBorrowingEnabled(SetBorrowingEnabledCall),
        SetDecimals(SetDecimalsCall),
        SetEModeCategory(SetEModeCategoryCall),
        SetFrozen(SetFrozenCall),
        SetLiquidationBonus(SetLiquidationBonusCall),
        SetLiquidationProtocolFee(SetLiquidationProtocolFeeCall),
        SetLiquidationThreshold(SetLiquidationThresholdCall),
        SetLtv(SetLtvCall),
        SetReserveFactor(SetReserveFactorCall),
        SetStableRateBorrowingEnabled(SetStableRateBorrowingEnabledCall),
        SetSupplyCap(SetSupplyCapCall),
        SetUnbackedMintCap(SetUnbackedMintCapCall),
    }
    impl ethers::core::abi::AbiDecode for MockReserveConfigurationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::Configuration(decoded));
            }
            if let Ok(decoded) =
                <GetBorrowCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetBorrowCap(decoded));
            }
            if let Ok(decoded) =
                <GetBorrowingEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetBorrowingEnabled(decoded));
            }
            if let Ok(decoded) =
                <GetCapsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetCaps(decoded));
            }
            if let Ok(decoded) =
                <GetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetDecimals(decoded));
            }
            if let Ok(decoded) =
                <GetEModeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetEModeCategory(decoded));
            }
            if let Ok(decoded) =
                <GetFlagsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetFlags(decoded));
            }
            if let Ok(decoded) =
                <GetFrozenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetFrozen(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationBonusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetLiquidationBonus(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockReserveConfigurationCalls::GetLiquidationProtocolFee(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetLiquidationThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetLiquidationThreshold(
                    decoded,
                ));
            }
            if let Ok(decoded) = <GetLtvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetLtv(decoded));
            }
            if let Ok(decoded) =
                <GetParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetParams(decoded));
            }
            if let Ok(decoded) =
                <GetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <GetStableRateBorrowingEnabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockReserveConfigurationCalls::GetStableRateBorrowingEnabled(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <GetUnbackedMintCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::GetUnbackedMintCap(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetBorrowCap(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowingEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetBorrowingEnabled(decoded));
            }
            if let Ok(decoded) =
                <SetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetDecimals(decoded));
            }
            if let Ok(decoded) =
                <SetEModeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetEModeCategory(decoded));
            }
            if let Ok(decoded) =
                <SetFrozenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetFrozen(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationBonusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetLiquidationBonus(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockReserveConfigurationCalls::SetLiquidationProtocolFee(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetLiquidationThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetLiquidationThreshold(
                    decoded,
                ));
            }
            if let Ok(decoded) = <SetLtvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetLtv(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <SetStableRateBorrowingEnabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockReserveConfigurationCalls::SetStableRateBorrowingEnabled(decoded));
            }
            if let Ok(decoded) =
                <SetSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <SetUnbackedMintCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockReserveConfigurationCalls::SetUnbackedMintCap(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockReserveConfigurationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockReserveConfigurationCalls::Configuration(element) => element.encode(),
                MockReserveConfigurationCalls::GetBorrowCap(element) => element.encode(),
                MockReserveConfigurationCalls::GetBorrowingEnabled(element) => element.encode(),
                MockReserveConfigurationCalls::GetCaps(element) => element.encode(),
                MockReserveConfigurationCalls::GetDecimals(element) => element.encode(),
                MockReserveConfigurationCalls::GetEModeCategory(element) => element.encode(),
                MockReserveConfigurationCalls::GetFlags(element) => element.encode(),
                MockReserveConfigurationCalls::GetFrozen(element) => element.encode(),
                MockReserveConfigurationCalls::GetLiquidationBonus(element) => element.encode(),
                MockReserveConfigurationCalls::GetLiquidationProtocolFee(element) => {
                    element.encode()
                }
                MockReserveConfigurationCalls::GetLiquidationThreshold(element) => element.encode(),
                MockReserveConfigurationCalls::GetLtv(element) => element.encode(),
                MockReserveConfigurationCalls::GetParams(element) => element.encode(),
                MockReserveConfigurationCalls::GetReserveFactor(element) => element.encode(),
                MockReserveConfigurationCalls::GetStableRateBorrowingEnabled(element) => {
                    element.encode()
                }
                MockReserveConfigurationCalls::GetSupplyCap(element) => element.encode(),
                MockReserveConfigurationCalls::GetUnbackedMintCap(element) => element.encode(),
                MockReserveConfigurationCalls::SetBorrowCap(element) => element.encode(),
                MockReserveConfigurationCalls::SetBorrowingEnabled(element) => element.encode(),
                MockReserveConfigurationCalls::SetDecimals(element) => element.encode(),
                MockReserveConfigurationCalls::SetEModeCategory(element) => element.encode(),
                MockReserveConfigurationCalls::SetFrozen(element) => element.encode(),
                MockReserveConfigurationCalls::SetLiquidationBonus(element) => element.encode(),
                MockReserveConfigurationCalls::SetLiquidationProtocolFee(element) => {
                    element.encode()
                }
                MockReserveConfigurationCalls::SetLiquidationThreshold(element) => element.encode(),
                MockReserveConfigurationCalls::SetLtv(element) => element.encode(),
                MockReserveConfigurationCalls::SetReserveFactor(element) => element.encode(),
                MockReserveConfigurationCalls::SetStableRateBorrowingEnabled(element) => {
                    element.encode()
                }
                MockReserveConfigurationCalls::SetSupplyCap(element) => element.encode(),
                MockReserveConfigurationCalls::SetUnbackedMintCap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockReserveConfigurationCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockReserveConfigurationCalls::Configuration(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetBorrowCap(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetBorrowingEnabled(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetCaps(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetDecimals(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetEModeCategory(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetFlags(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetFrozen(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetLiquidationBonus(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetLiquidationProtocolFee(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetLiquidationThreshold(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetLtv(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetParams(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetReserveFactor(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetStableRateBorrowingEnabled(element) => {
                    element.fmt(f)
                }
                MockReserveConfigurationCalls::GetSupplyCap(element) => element.fmt(f),
                MockReserveConfigurationCalls::GetUnbackedMintCap(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetBorrowCap(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetBorrowingEnabled(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetDecimals(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetEModeCategory(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetFrozen(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetLiquidationBonus(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetLiquidationProtocolFee(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetLiquidationThreshold(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetLtv(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetReserveFactor(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetStableRateBorrowingEnabled(element) => {
                    element.fmt(f)
                }
                MockReserveConfigurationCalls::SetSupplyCap(element) => element.fmt(f),
                MockReserveConfigurationCalls::SetUnbackedMintCap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ConfigurationCall> for MockReserveConfigurationCalls {
        fn from(var: ConfigurationCall) -> Self {
            MockReserveConfigurationCalls::Configuration(var)
        }
    }
    impl ::std::convert::From<GetBorrowCapCall> for MockReserveConfigurationCalls {
        fn from(var: GetBorrowCapCall) -> Self {
            MockReserveConfigurationCalls::GetBorrowCap(var)
        }
    }
    impl ::std::convert::From<GetBorrowingEnabledCall> for MockReserveConfigurationCalls {
        fn from(var: GetBorrowingEnabledCall) -> Self {
            MockReserveConfigurationCalls::GetBorrowingEnabled(var)
        }
    }
    impl ::std::convert::From<GetCapsCall> for MockReserveConfigurationCalls {
        fn from(var: GetCapsCall) -> Self {
            MockReserveConfigurationCalls::GetCaps(var)
        }
    }
    impl ::std::convert::From<GetDecimalsCall> for MockReserveConfigurationCalls {
        fn from(var: GetDecimalsCall) -> Self {
            MockReserveConfigurationCalls::GetDecimals(var)
        }
    }
    impl ::std::convert::From<GetEModeCategoryCall> for MockReserveConfigurationCalls {
        fn from(var: GetEModeCategoryCall) -> Self {
            MockReserveConfigurationCalls::GetEModeCategory(var)
        }
    }
    impl ::std::convert::From<GetFlagsCall> for MockReserveConfigurationCalls {
        fn from(var: GetFlagsCall) -> Self {
            MockReserveConfigurationCalls::GetFlags(var)
        }
    }
    impl ::std::convert::From<GetFrozenCall> for MockReserveConfigurationCalls {
        fn from(var: GetFrozenCall) -> Self {
            MockReserveConfigurationCalls::GetFrozen(var)
        }
    }
    impl ::std::convert::From<GetLiquidationBonusCall> for MockReserveConfigurationCalls {
        fn from(var: GetLiquidationBonusCall) -> Self {
            MockReserveConfigurationCalls::GetLiquidationBonus(var)
        }
    }
    impl ::std::convert::From<GetLiquidationProtocolFeeCall> for MockReserveConfigurationCalls {
        fn from(var: GetLiquidationProtocolFeeCall) -> Self {
            MockReserveConfigurationCalls::GetLiquidationProtocolFee(var)
        }
    }
    impl ::std::convert::From<GetLiquidationThresholdCall> for MockReserveConfigurationCalls {
        fn from(var: GetLiquidationThresholdCall) -> Self {
            MockReserveConfigurationCalls::GetLiquidationThreshold(var)
        }
    }
    impl ::std::convert::From<GetLtvCall> for MockReserveConfigurationCalls {
        fn from(var: GetLtvCall) -> Self {
            MockReserveConfigurationCalls::GetLtv(var)
        }
    }
    impl ::std::convert::From<GetParamsCall> for MockReserveConfigurationCalls {
        fn from(var: GetParamsCall) -> Self {
            MockReserveConfigurationCalls::GetParams(var)
        }
    }
    impl ::std::convert::From<GetReserveFactorCall> for MockReserveConfigurationCalls {
        fn from(var: GetReserveFactorCall) -> Self {
            MockReserveConfigurationCalls::GetReserveFactor(var)
        }
    }
    impl ::std::convert::From<GetStableRateBorrowingEnabledCall> for MockReserveConfigurationCalls {
        fn from(var: GetStableRateBorrowingEnabledCall) -> Self {
            MockReserveConfigurationCalls::GetStableRateBorrowingEnabled(var)
        }
    }
    impl ::std::convert::From<GetSupplyCapCall> for MockReserveConfigurationCalls {
        fn from(var: GetSupplyCapCall) -> Self {
            MockReserveConfigurationCalls::GetSupplyCap(var)
        }
    }
    impl ::std::convert::From<GetUnbackedMintCapCall> for MockReserveConfigurationCalls {
        fn from(var: GetUnbackedMintCapCall) -> Self {
            MockReserveConfigurationCalls::GetUnbackedMintCap(var)
        }
    }
    impl ::std::convert::From<SetBorrowCapCall> for MockReserveConfigurationCalls {
        fn from(var: SetBorrowCapCall) -> Self {
            MockReserveConfigurationCalls::SetBorrowCap(var)
        }
    }
    impl ::std::convert::From<SetBorrowingEnabledCall> for MockReserveConfigurationCalls {
        fn from(var: SetBorrowingEnabledCall) -> Self {
            MockReserveConfigurationCalls::SetBorrowingEnabled(var)
        }
    }
    impl ::std::convert::From<SetDecimalsCall> for MockReserveConfigurationCalls {
        fn from(var: SetDecimalsCall) -> Self {
            MockReserveConfigurationCalls::SetDecimals(var)
        }
    }
    impl ::std::convert::From<SetEModeCategoryCall> for MockReserveConfigurationCalls {
        fn from(var: SetEModeCategoryCall) -> Self {
            MockReserveConfigurationCalls::SetEModeCategory(var)
        }
    }
    impl ::std::convert::From<SetFrozenCall> for MockReserveConfigurationCalls {
        fn from(var: SetFrozenCall) -> Self {
            MockReserveConfigurationCalls::SetFrozen(var)
        }
    }
    impl ::std::convert::From<SetLiquidationBonusCall> for MockReserveConfigurationCalls {
        fn from(var: SetLiquidationBonusCall) -> Self {
            MockReserveConfigurationCalls::SetLiquidationBonus(var)
        }
    }
    impl ::std::convert::From<SetLiquidationProtocolFeeCall> for MockReserveConfigurationCalls {
        fn from(var: SetLiquidationProtocolFeeCall) -> Self {
            MockReserveConfigurationCalls::SetLiquidationProtocolFee(var)
        }
    }
    impl ::std::convert::From<SetLiquidationThresholdCall> for MockReserveConfigurationCalls {
        fn from(var: SetLiquidationThresholdCall) -> Self {
            MockReserveConfigurationCalls::SetLiquidationThreshold(var)
        }
    }
    impl ::std::convert::From<SetLtvCall> for MockReserveConfigurationCalls {
        fn from(var: SetLtvCall) -> Self {
            MockReserveConfigurationCalls::SetLtv(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for MockReserveConfigurationCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            MockReserveConfigurationCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<SetStableRateBorrowingEnabledCall> for MockReserveConfigurationCalls {
        fn from(var: SetStableRateBorrowingEnabledCall) -> Self {
            MockReserveConfigurationCalls::SetStableRateBorrowingEnabled(var)
        }
    }
    impl ::std::convert::From<SetSupplyCapCall> for MockReserveConfigurationCalls {
        fn from(var: SetSupplyCapCall) -> Self {
            MockReserveConfigurationCalls::SetSupplyCap(var)
        }
    }
    impl ::std::convert::From<SetUnbackedMintCapCall> for MockReserveConfigurationCalls {
        fn from(var: SetUnbackedMintCapCall) -> Self {
            MockReserveConfigurationCalls::SetUnbackedMintCap(var)
        }
    }
    #[doc = "Container type for all return fields from the `configuration` function with signature `configuration()` and selector `[108, 112, 190, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConfigurationReturn {
        pub data: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getBorrowCap` function with signature `getBorrowCap()` and selector `[174, 222, 123, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBorrowCapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getBorrowingEnabled` function with signature `getBorrowingEnabled()` and selector `[121, 117, 11, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBorrowingEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `getCaps` function with signature `getCaps()` and selector `[157, 112, 109, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCapsReturn(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getDecimals` function with signature `getDecimals()` and selector `[240, 20, 29, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetDecimalsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getEModeCategory` function with signature `getEModeCategory()` and selector `[53, 111, 35, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetEModeCategoryReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFlags` function with signature `getFlags()` and selector `[108, 199, 20, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetFlagsReturn(pub bool, pub bool, pub bool, pub bool, pub bool);
    #[doc = "Container type for all return fields from the `getFrozen` function with signature `getFrozen()` and selector `[116, 149, 179, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetFrozenReturn(pub bool);
    #[doc = "Container type for all return fields from the `getLiquidationBonus` function with signature `getLiquidationBonus()` and selector `[89, 170, 158, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetLiquidationBonusReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee()` and selector `[195, 123, 220, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetLiquidationProtocolFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getLiquidationThreshold` function with signature `getLiquidationThreshold()` and selector `[74, 233, 184, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetLiquidationThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getLtv` function with signature `getLtv()` and selector `[129, 69, 189, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetLtvReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getParams` function with signature `getParams()` and selector `[94, 97, 90, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetParamsReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getReserveFactor` function with signature `getReserveFactor()` and selector `[95, 85, 142, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveFactorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getStableRateBorrowingEnabled` function with signature `getStableRateBorrowingEnabled()` and selector `[224, 138, 40, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetStableRateBorrowingEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `getSupplyCap` function with signature `getSupplyCap()` and selector `[32, 54, 24, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyCapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getUnbackedMintCap` function with signature `getUnbackedMintCap()` and selector `[234, 216, 170, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUnbackedMintCapReturn(pub ethers::core::types::U256);
}
