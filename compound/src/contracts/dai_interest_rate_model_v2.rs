pub use dai_interest_rate_model_v2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod dai_interest_rate_model_v2 {
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
    #[doc = "DAIInterestRateModelV2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DAIINTERESTRATEMODELV2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"jumpMultiplierPerYear\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"kink_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"pot_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"jug_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"baseRatePerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"multiplierPerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"jumpMultiplierPerBlock\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"kink\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewInterestParams\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assumedOneMinusReserveFactorMantissa\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseRatePerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blocksPerYear\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dsrPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gapPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveFactorMantissa\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInterestRateModel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"jumpMultiplierPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"kink\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"multiplierPerBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"poke\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"cash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrows\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserves\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"utilizationRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DAIINTERESTRATEMODELV2_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b50604051620010d1380380620010d1833981810160405260808110156200003757600080fd5b5080516020808301516040840151606090940151929390929091600090819086908690620000759084906220148090620006e86200015c821b17901c565b6001556200009383622014806200015c602090811b620006e817901c565b600055620000b182622014806200015c602090811b620006e817901c565b60028190556003829055600154600054604080519283526020830191909152818101929092526060810183905290517f6960ab234c7ef4b0c9197100f5393cfcde7c453ac910a27bd2000aa1dd4c068d9181900360800190a15050600480546001600160a01b038087166001600160a01b0319928316179092556005805492861692909116919091179055506200015290506001600160e01b03620001af16565b5050505062000721565b6000620001a683836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f0000000000008152506200046360201b60201c565b90505b92915050565b60055460408051636cb1c69b60e11b8152644554482d4160d81b600482015281516000936001600160a01b03169263d9638d369260248082019391829003018186803b158015620001ff57600080fd5b505afa15801562000214573d6000803e3d6000fd5b505050506040513d60408110156200022b57600080fd5b505160055460408051635001f3b560e01b815290519293506000926200032a92600f9262000302926b033b2e3c9fd0803ce8000000926200031692670de0b6b3a76400009286928692620002ee926001600160a01b0390921691635001f3b591600480820192602092909190829003018186803b158015620002ac57600080fd5b505afa158015620002c1573d6000803e3d6000fd5b505050506040513d6020811015620002d857600080fd5b50518b906200050a602090811b6200072a17901c565b6200056560201b620007841790919060201c565b620005af60201b620006861790919060201c565b6200015c60201b620006e81790919060201c565b905062000359670d2f13f7789f000062000316670de0b6b3a7640000620003026001600160e01b036200060d16565b6001819055811115620003cc57620003c360035462000316670de0b6b3a7640000620003026220148066470de4df820000816200039257fe5b04620003af600154886200056560201b620007841790919060201c565b6200050a60201b6200072a1790919060201c565b6000556200040d565b6200040960035462000316670de0b6b3a76400006220148066470de4df82000081620003f457fe5b04620005af60201b620006861790919060201c565b6000555b600154600054600254600354604080519485526020850193909352838301919091526060830152517f6960ab234c7ef4b0c9197100f5393cfcde7c453ac910a27bd2000aa1dd4c068d9181900360800190a15050565b60008183620004f35760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b83811015620004b75781810151838201526020016200049d565b50505050905090810190601f168015620004e55780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5060008385816200050057fe5b0495945050505050565b600082820183811015620001a6576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b6000620001a683836040518060400160405280601f81526020017f536166654d6174683a207375627472616374696f6e20756e646572666c6f7700815250620006c460201b60201c565b600082620005c057506000620001a9565b82820282848281620005ce57fe5b0414620001a65760405162461bcd60e51b8152600401808060200182810382526021815260200180620010b06021913960400191505060405180910390fd5b6000620006bf600f62000302633b9aca00620003166b033b2e3c9fd0803ce8000000600460009054906101000a90046001600160a01b03166001600160a01b031663487bf0826040518163ffffffff1660e01b815260040160206040518083038186803b1580156200067e57600080fd5b505afa15801562000693573d6000803e3d6000fd5b505050506040513d6020811015620006aa57600080fd5b50519062000565602090811b6200078417901c565b905090565b60008184841115620007195760405162461bcd60e51b8152602060048201818152835160248401528351909283926044909101919085019080838360008315620004b75781810151838201526020016200049d565b505050900390565b61097f80620007316000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c806396456c5c1161008c578063b9f9850a11610066578063b9f9850a146101ad578063f14039de146101b5578063f52d21f3146101bd578063fd2da339146101c5576100cf565b806396456c5c1461016e578063a385fb9614610176578063b81688161461017e576100cf565b806315f24053146100d4578063181783581461010f5780632191f92a146101195780636dac7cd5146101355780636e71e2d81461013d5780638726bb8914610166575b600080fd5b6100fd600480360360608110156100ea57600080fd5b50803590602081013590604001356101cd565b60408051918252519081900360200190f35b6101176102a5565b005b6101216104eb565b604080519115158252519081900360200190f35b6100fd6104f0565b6100fd6004803603606081101561015357600080fd5b50803590602081013590604001356104fc565b6100fd610542565b6100fd610548565b6100fd6105f2565b6100fd6004803603608081101561019457600080fd5b50803590602081013590604081013590606001356105f9565b6100fd61066b565b6100fd610671565b6100fd610677565b6100fd610680565b6000806101db8585856104fc565b9050600354811161022d57610225600154610219670de0b6b3a764000061020d6000548661068690919063ffffffff16565b9063ffffffff6106e816565b9063ffffffff61072a16565b91505061029e565b6000610258600154610219670de0b6b3a764000061020d60005460035461068690919063ffffffff16565b905060006102716003548461078490919063ffffffff16565b905061029882610219670de0b6b3a764000061020d6002548661068690919063ffffffff16565b93505050505b9392505050565b60055460408051636cb1c69b60e11b8152644554482d4160d81b600482015281516000936001600160a01b03169263d9638d369260248082019391829003018186803b1580156102f457600080fd5b505afa158015610308573d6000803e3d6000fd5b505050506040513d604081101561031e57600080fd5b505160055460408051635001f3b560e01b815290519293506000926103eb92600f926103df926b033b2e3c9fd0803ce80000009261020d92670de0b6b3a764000092869286926103d3926001600160a01b0390921691635001f3b591600480820192602092909190829003018186803b15801561039a57600080fd5b505afa1580156103ae573d6000803e3d6000fd5b505050506040513d60208110156103c457600080fd5b50518b9063ffffffff61072a16565b9063ffffffff61078416565b9063ffffffff61068616565b905061040d670d2f13f7789f000061020d670de0b6b3a76400006103df610548565b600181905581111561045f5761045760035461020d670de0b6b3a76400006103df6220148066470de4df8200008161044157fe5b046102196001548861078490919063ffffffff16565b600055610495565b61049160035461020d670de0b6b3a76400006220148066470de4df8200008161048457fe5b049063ffffffff61068616565b6000555b600154600054600254600354604080519485526020850193909352838301919091526060830152517f6960ab234c7ef4b0c9197100f5393cfcde7c453ac910a27bd2000aa1dd4c068d9181900360800190a15050565b600181565b670d2f13f7789f000081565b60008261050b5750600061029e565b61053a610522836103d3878763ffffffff61072a16565b61020d85670de0b6b3a764000063ffffffff61068616565b949350505050565b60005481565b60006105ed600f6103df633b9aca0061020d6b033b2e3c9fd0803ce8000000600460009054906101000a90046001600160a01b03166001600160a01b031663487bf0826040518163ffffffff1660e01b815260040160206040518083038186803b1580156105b557600080fd5b505afa1580156105c9573d6000803e3d6000fd5b505050506040513d60208110156105df57600080fd5b50519063ffffffff61078416565b905090565b6220148081565b600080610608868686866107c6565b90506000610620856103d3898963ffffffff61072a16565b90508061062f5750905061053a565b600061064d8261020d610640610548565b8b9063ffffffff61068616565b905061065f818463ffffffff61072a16565b98975050505050505050565b60025481565b60015481565b64023703e87b81565b60035481565b600082610695575060006106e2565b828202828482816106a257fe5b04146106df5760405162461bcd60e51b815260040180806020018281038252602181526020018061092a6021913960400191505060405180910390fd5b90505b92915050565b60006106df83836040518060400160405280601a81526020017f536166654d6174683a206469766973696f6e206279207a65726f00000000000081525061082d565b6000828201838110156106df576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b60006106df83836040518060400160405280601f81526020017f536166654d6174683a207375627472616374696f6e20756e646572666c6f77008152506108cf565b6000806107e1670de0b6b3a76400008463ffffffff61078416565b905060006107f08787876101cd565b90506000610810670de0b6b3a764000061020d848663ffffffff61068616565b905061065f670de0b6b3a764000061020d836103df8c8c8c6104fc565b600081836108b95760405162461bcd60e51b81526004018080602001828103825283818151815260200191508051906020019080838360005b8381101561087e578181015183820152602001610866565b50505050905090810190601f1680156108ab5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b5060008385816108c557fe5b0495945050505050565b600081848411156109215760405162461bcd60e51b815260206004820181815283516024840152835190928392604490910191908501908083836000831561087e578181015183820152602001610866565b50505090039056fe536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77a265627a7a7231582088ad76653e6d80c5196e79505a44e2320185bcedad929e3df673777b1b8e848d64736f6c63430005110032536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77" . parse () . expect ("invalid bytecode")
        });
    pub struct DAIInterestRateModelV2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DAIInterestRateModelV2<M> {
        fn clone(&self) -> Self {
            DAIInterestRateModelV2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DAIInterestRateModelV2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DAIInterestRateModelV2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DAIInterestRateModelV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DAIInterestRateModelV2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                DAIINTERESTRATEMODELV2_ABI.clone(),
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
                DAIINTERESTRATEMODELV2_ABI.clone(),
                DAIINTERESTRATEMODELV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `assumedOneMinusReserveFactorMantissa` (0x6dac7cd5) function"]
        pub fn assumed_one_minus_reserve_factor_mantissa(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([109, 172, 124, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `baseRatePerBlock` (0xf14039de) function"]
        pub fn base_rate_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 64, 57, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blocksPerYear` (0xa385fb96) function"]
        pub fn blocks_per_year(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([163, 133, 251, 150], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dsrPerBlock` (0x96456c5c) function"]
        pub fn dsr_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([150, 69, 108, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gapPerBlock` (0xf52d21f3) function"]
        pub fn gap_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 45, 33, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBorrowRate` (0x15f24053) function"]
        pub fn get_borrow_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([21, 242, 64, 83], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyRate` (0xb8168816) function"]
        pub fn get_supply_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
            reserve_factor_mantissa: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [184, 22, 136, 22],
                    (cash, borrows, reserves, reserve_factor_mantissa),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInterestRateModel` (0x2191f92a) function"]
        pub fn is_interest_rate_model(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([33, 145, 249, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `jumpMultiplierPerBlock` (0xb9f9850a) function"]
        pub fn jump_multiplier_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([185, 249, 133, 10], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kink` (0xfd2da339) function"]
        pub fn kink(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([253, 45, 163, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multiplierPerBlock` (0x8726bb89) function"]
        pub fn multiplier_per_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 38, 187, 137], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poke` (0x18178358) function"]
        pub fn poke(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 23, 131, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `utilizationRate` (0x6e71e2d8) function"]
        pub fn utilization_rate(
            &self,
            cash: ethers::core::types::U256,
            borrows: ethers::core::types::U256,
            reserves: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 113, 226, 216], (cash, borrows, reserves))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewInterestParams` event"]
        pub fn new_interest_params_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewInterestParamsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewInterestParamsFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for DAIInterestRateModelV2<M>
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
        name = "NewInterestParams",
        abi = "NewInterestParams(uint256,uint256,uint256,uint256)"
    )]
    pub struct NewInterestParamsFilter {
        pub base_rate_per_block: ethers::core::types::U256,
        pub multiplier_per_block: ethers::core::types::U256,
        pub jump_multiplier_per_block: ethers::core::types::U256,
        pub kink: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `assumedOneMinusReserveFactorMantissa` function with signature `assumedOneMinusReserveFactorMantissa()` and selector `[109, 172, 124, 213]`"]
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
        name = "assumedOneMinusReserveFactorMantissa",
        abi = "assumedOneMinusReserveFactorMantissa()"
    )]
    pub struct AssumedOneMinusReserveFactorMantissaCall;
    #[doc = "Container type for all input parameters for the `baseRatePerBlock` function with signature `baseRatePerBlock()` and selector `[241, 64, 57, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "baseRatePerBlock", abi = "baseRatePerBlock()")]
    pub struct BaseRatePerBlockCall;
    #[doc = "Container type for all input parameters for the `blocksPerYear` function with signature `blocksPerYear()` and selector `[163, 133, 251, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "blocksPerYear", abi = "blocksPerYear()")]
    pub struct BlocksPerYearCall;
    #[doc = "Container type for all input parameters for the `dsrPerBlock` function with signature `dsrPerBlock()` and selector `[150, 69, 108, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dsrPerBlock", abi = "dsrPerBlock()")]
    pub struct DsrPerBlockCall;
    #[doc = "Container type for all input parameters for the `gapPerBlock` function with signature `gapPerBlock()` and selector `[245, 45, 33, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gapPerBlock", abi = "gapPerBlock()")]
    pub struct GapPerBlockCall;
    #[doc = "Container type for all input parameters for the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBorrowRate", abi = "getBorrowRate(uint256,uint256,uint256)")]
    pub struct GetBorrowRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
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
        name = "getSupplyRate",
        abi = "getSupplyRate(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetSupplyRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
        pub reserve_factor_mantissa: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isInterestRateModel", abi = "isInterestRateModel()")]
    pub struct IsInterestRateModelCall;
    #[doc = "Container type for all input parameters for the `jumpMultiplierPerBlock` function with signature `jumpMultiplierPerBlock()` and selector `[185, 249, 133, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "jumpMultiplierPerBlock", abi = "jumpMultiplierPerBlock()")]
    pub struct JumpMultiplierPerBlockCall;
    #[doc = "Container type for all input parameters for the `kink` function with signature `kink()` and selector `[253, 45, 163, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "kink", abi = "kink()")]
    pub struct KinkCall;
    #[doc = "Container type for all input parameters for the `multiplierPerBlock` function with signature `multiplierPerBlock()` and selector `[135, 38, 187, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "multiplierPerBlock", abi = "multiplierPerBlock()")]
    pub struct MultiplierPerBlockCall;
    #[doc = "Container type for all input parameters for the `poke` function with signature `poke()` and selector `[24, 23, 131, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poke", abi = "poke()")]
    pub struct PokeCall;
    #[doc = "Container type for all input parameters for the `utilizationRate` function with signature `utilizationRate(uint256,uint256,uint256)` and selector `[110, 113, 226, 216]`"]
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
        name = "utilizationRate",
        abi = "utilizationRate(uint256,uint256,uint256)"
    )]
    pub struct UtilizationRateCall {
        pub cash: ethers::core::types::U256,
        pub borrows: ethers::core::types::U256,
        pub reserves: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DAIInterestRateModelV2Calls {
        AssumedOneMinusReserveFactorMantissa(AssumedOneMinusReserveFactorMantissaCall),
        BaseRatePerBlock(BaseRatePerBlockCall),
        BlocksPerYear(BlocksPerYearCall),
        DsrPerBlock(DsrPerBlockCall),
        GapPerBlock(GapPerBlockCall),
        GetBorrowRate(GetBorrowRateCall),
        GetSupplyRate(GetSupplyRateCall),
        IsInterestRateModel(IsInterestRateModelCall),
        JumpMultiplierPerBlock(JumpMultiplierPerBlockCall),
        Kink(KinkCall),
        MultiplierPerBlock(MultiplierPerBlockCall),
        Poke(PokeCall),
        UtilizationRate(UtilizationRateCall),
    }
    impl ethers::core::abi::AbiDecode for DAIInterestRateModelV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssumedOneMinusReserveFactorMantissaCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    DAIInterestRateModelV2Calls::AssumedOneMinusReserveFactorMantissa(decoded),
                );
            }
            if let Ok(decoded) =
                <BaseRatePerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::BaseRatePerBlock(decoded));
            }
            if let Ok(decoded) =
                <BlocksPerYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::BlocksPerYear(decoded));
            }
            if let Ok(decoded) =
                <DsrPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::DsrPerBlock(decoded));
            }
            if let Ok(decoded) =
                <GapPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::GapPerBlock(decoded));
            }
            if let Ok(decoded) =
                <GetBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::GetBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::GetSupplyRate(decoded));
            }
            if let Ok(decoded) =
                <IsInterestRateModelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::IsInterestRateModel(decoded));
            }
            if let Ok(decoded) =
                <JumpMultiplierPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::JumpMultiplierPerBlock(decoded));
            }
            if let Ok(decoded) = <KinkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DAIInterestRateModelV2Calls::Kink(decoded));
            }
            if let Ok(decoded) =
                <MultiplierPerBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::MultiplierPerBlock(decoded));
            }
            if let Ok(decoded) = <PokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DAIInterestRateModelV2Calls::Poke(decoded));
            }
            if let Ok(decoded) =
                <UtilizationRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DAIInterestRateModelV2Calls::UtilizationRate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DAIInterestRateModelV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DAIInterestRateModelV2Calls::AssumedOneMinusReserveFactorMantissa(element) => {
                    element.encode()
                }
                DAIInterestRateModelV2Calls::BaseRatePerBlock(element) => element.encode(),
                DAIInterestRateModelV2Calls::BlocksPerYear(element) => element.encode(),
                DAIInterestRateModelV2Calls::DsrPerBlock(element) => element.encode(),
                DAIInterestRateModelV2Calls::GapPerBlock(element) => element.encode(),
                DAIInterestRateModelV2Calls::GetBorrowRate(element) => element.encode(),
                DAIInterestRateModelV2Calls::GetSupplyRate(element) => element.encode(),
                DAIInterestRateModelV2Calls::IsInterestRateModel(element) => element.encode(),
                DAIInterestRateModelV2Calls::JumpMultiplierPerBlock(element) => element.encode(),
                DAIInterestRateModelV2Calls::Kink(element) => element.encode(),
                DAIInterestRateModelV2Calls::MultiplierPerBlock(element) => element.encode(),
                DAIInterestRateModelV2Calls::Poke(element) => element.encode(),
                DAIInterestRateModelV2Calls::UtilizationRate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DAIInterestRateModelV2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DAIInterestRateModelV2Calls::AssumedOneMinusReserveFactorMantissa(element) => {
                    element.fmt(f)
                }
                DAIInterestRateModelV2Calls::BaseRatePerBlock(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::BlocksPerYear(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::DsrPerBlock(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::GapPerBlock(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::GetBorrowRate(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::GetSupplyRate(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::IsInterestRateModel(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::JumpMultiplierPerBlock(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::Kink(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::MultiplierPerBlock(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::Poke(element) => element.fmt(f),
                DAIInterestRateModelV2Calls::UtilizationRate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssumedOneMinusReserveFactorMantissaCall>
        for DAIInterestRateModelV2Calls
    {
        fn from(var: AssumedOneMinusReserveFactorMantissaCall) -> Self {
            DAIInterestRateModelV2Calls::AssumedOneMinusReserveFactorMantissa(var)
        }
    }
    impl ::std::convert::From<BaseRatePerBlockCall> for DAIInterestRateModelV2Calls {
        fn from(var: BaseRatePerBlockCall) -> Self {
            DAIInterestRateModelV2Calls::BaseRatePerBlock(var)
        }
    }
    impl ::std::convert::From<BlocksPerYearCall> for DAIInterestRateModelV2Calls {
        fn from(var: BlocksPerYearCall) -> Self {
            DAIInterestRateModelV2Calls::BlocksPerYear(var)
        }
    }
    impl ::std::convert::From<DsrPerBlockCall> for DAIInterestRateModelV2Calls {
        fn from(var: DsrPerBlockCall) -> Self {
            DAIInterestRateModelV2Calls::DsrPerBlock(var)
        }
    }
    impl ::std::convert::From<GapPerBlockCall> for DAIInterestRateModelV2Calls {
        fn from(var: GapPerBlockCall) -> Self {
            DAIInterestRateModelV2Calls::GapPerBlock(var)
        }
    }
    impl ::std::convert::From<GetBorrowRateCall> for DAIInterestRateModelV2Calls {
        fn from(var: GetBorrowRateCall) -> Self {
            DAIInterestRateModelV2Calls::GetBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetSupplyRateCall> for DAIInterestRateModelV2Calls {
        fn from(var: GetSupplyRateCall) -> Self {
            DAIInterestRateModelV2Calls::GetSupplyRate(var)
        }
    }
    impl ::std::convert::From<IsInterestRateModelCall> for DAIInterestRateModelV2Calls {
        fn from(var: IsInterestRateModelCall) -> Self {
            DAIInterestRateModelV2Calls::IsInterestRateModel(var)
        }
    }
    impl ::std::convert::From<JumpMultiplierPerBlockCall> for DAIInterestRateModelV2Calls {
        fn from(var: JumpMultiplierPerBlockCall) -> Self {
            DAIInterestRateModelV2Calls::JumpMultiplierPerBlock(var)
        }
    }
    impl ::std::convert::From<KinkCall> for DAIInterestRateModelV2Calls {
        fn from(var: KinkCall) -> Self {
            DAIInterestRateModelV2Calls::Kink(var)
        }
    }
    impl ::std::convert::From<MultiplierPerBlockCall> for DAIInterestRateModelV2Calls {
        fn from(var: MultiplierPerBlockCall) -> Self {
            DAIInterestRateModelV2Calls::MultiplierPerBlock(var)
        }
    }
    impl ::std::convert::From<PokeCall> for DAIInterestRateModelV2Calls {
        fn from(var: PokeCall) -> Self {
            DAIInterestRateModelV2Calls::Poke(var)
        }
    }
    impl ::std::convert::From<UtilizationRateCall> for DAIInterestRateModelV2Calls {
        fn from(var: UtilizationRateCall) -> Self {
            DAIInterestRateModelV2Calls::UtilizationRate(var)
        }
    }
    #[doc = "Container type for all return fields from the `assumedOneMinusReserveFactorMantissa` function with signature `assumedOneMinusReserveFactorMantissa()` and selector `[109, 172, 124, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssumedOneMinusReserveFactorMantissaReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `baseRatePerBlock` function with signature `baseRatePerBlock()` and selector `[241, 64, 57, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BaseRatePerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `blocksPerYear` function with signature `blocksPerYear()` and selector `[163, 133, 251, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BlocksPerYearReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `dsrPerBlock` function with signature `dsrPerBlock()` and selector `[150, 69, 108, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DsrPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `gapPerBlock` function with signature `gapPerBlock()` and selector `[245, 45, 33, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GapPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getBorrowRate` function with signature `getBorrowRate(uint256,uint256,uint256)` and selector `[21, 242, 64, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetBorrowRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSupplyRate` function with signature `getSupplyRate(uint256,uint256,uint256,uint256)` and selector `[184, 22, 136, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isInterestRateModel` function with signature `isInterestRateModel()` and selector `[33, 145, 249, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsInterestRateModelReturn(pub bool);
    #[doc = "Container type for all return fields from the `jumpMultiplierPerBlock` function with signature `jumpMultiplierPerBlock()` and selector `[185, 249, 133, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct JumpMultiplierPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `kink` function with signature `kink()` and selector `[253, 45, 163, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct KinkReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `multiplierPerBlock` function with signature `multiplierPerBlock()` and selector `[135, 38, 187, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MultiplierPerBlockReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `utilizationRate` function with signature `utilizationRate(uint256,uint256,uint256)` and selector `[110, 113, 226, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UtilizationRateReturn(pub ethers::core::types::U256);
}
