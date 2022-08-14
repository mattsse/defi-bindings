pub use l2_encoder::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod l2_encoder {
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
    #[doc = "L2Encoder was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static L2ENCODER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeBorrowParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"debtAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"debtToCover\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"receiveAToken\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeLiquidationCall\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeRebalanceStableBorrowRate\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeRepayParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeRepayWithATokensParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"permitV\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitS\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeRepayWithPermitParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"useAsCollateral\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeSetUserUseReserveAsCollateral\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeSupplyParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"permitV\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitS\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeSupplyWithPermitParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeSwapBorrowRateMode\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeWithdrawParams\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static L2ENCODER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405234801561001057600080fd5b5060405161111138038061111183398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b6080516110406100d16000396000818161014b0152818161022b015281816102fd015281816103a001528181610453015281816105250152818161060d015281816106a70152818161079f01528181610881015261094001526110406000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c806388d518521161007157806388d51852146101855780638da7fb18146101ad5780639d2ffc1b146101c0578063b76398e4146101d3578063fc0eed85146101e6578063fed63a93146101f457600080fd5b80631a64acf2146100b95780631a8f6dee146100df5780631fd34797146100f25780635cc7bc1014610105578063671a7fae146101185780637535d24614610146575b600080fd5b6100cc6100c7366004610b81565b610207565b6040519081526020015b60405180910390f35b6100cc6100ed366004610bcb565b6102d9565b6100cc610100366004610c04565b61037c565b6100cc610113366004610c04565b61042f565b61012b610126366004610c46565b6104fd565b604080519384526020840192909252908201526060016100d6565b61016d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100d6565b610198610193366004610cc4565b6105e7565b604080519283526020830191909152016100d6565b6100cc6101bb366004610d28565b610766565b6100cc6101ce366004610d28565b61077b565b6100cc6101e1366004610d5d565b61085d565b6100cc6100ed366004610d9f565b61012b610202366004610dd4565b610918565b6040516335ea6a7560e01b81526001600160a01b03858116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa158015610273573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102979190610ee7565b60e081015190915060006102aa87610a21565b905060006102b787610a93565b60109290921b60909290921b60989690961b9590950101019695505050505050565b6040516335ea6a7560e01b81526001600160a01b03838116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa158015610345573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103699190610ee7565b60e00151601084901b0191505092915050565b6040516335ea6a7560e01b81526001600160a01b03838116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa1580156103e8573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061040c9190610ee7565b60e0810151909150600061041f85610a93565b60101b9190910195945050505050565b6040516335ea6a7560e01b81526001600160a01b03838116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa15801561049b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104bf9190610ee7565b60e0810151909150600060001985146104e0576104db85610a21565b61041f565b5071ffffffffffffffffffffffffffffffff000001949350505050565b6040516335ea6a7560e01b81526001600160a01b0388811660048301526000918291829182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa15801561056d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105919190610ee7565b60e081015190915060006105a48c610a21565b905060006105b18b610af4565b905060008a60c01b8260a01b018d60901b018360101b0184019050808a8a97509750975050505050509750975097945050505050565b6040516335ea6a7560e01b81526001600160a01b038681166004830152600091829182917f0000000000000000000000000000000000000000000000000000000000000000909116906335ea6a75906024016101e060405180830381865afa158015610657573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061067b9190610ee7565b60e08101516040516335ea6a7560e01b81526001600160a01b038a8116600483015292935090916000917f0000000000000000000000000000000000000000000000000000000000000000909116906335ea6a75906024016101e060405180830381865afa1580156106f1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107159190610ee7565b60e0810151909150600060001989146107365761073189610a21565b61073f565b6001600160801b035b60109290921b9390930160208a901b019550608087901b0193505050509550959350505050565b600061077384848461077b565b949350505050565b6040516335ea6a7560e01b81526001600160a01b03848116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa1580156107e7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061080b9190610ee7565b60e08101519091506000600019861461082c5761082786610a21565b610835565b6001600160801b035b9050600061084286610a93565b60901b60109290921b91909101919091019695505050505050565b6040516335ea6a7560e01b81526001600160a01b03848116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa1580156108c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108ed9190610ee7565b60e0810151909150600061090086610a21565b60101b609086901b0191909101925050509392505050565b6040516335ea6a7560e01b81526001600160a01b0388811660048301526000918291829182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a75906024016101e060405180830381865afa158015610988573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109ac9190610ee7565b60e081015190915060006000198c146109cd576109c88c610a21565b6109d6565b6001600160801b035b905060006109e38c610a93565b905060006109f08c610af4565b60b89b909b1b60989b909b1b9a909a0160909190911b0160109190911b01019b959a50939850939650505050505050565b60006001600160801b03821115610a8f5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b60648201526084015b60405180910390fd5b5090565b600060ff821115610a8f5760405162461bcd60e51b815260206004820152602560248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2038604482015264206269747360d81b6064820152608401610a86565b600063ffffffff821115610a8f5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610a86565b6001600160a01b0381168114610b6e57600080fd5b50565b61ffff81168114610b6e57600080fd5b60008060008060808587031215610b9757600080fd5b8435610ba281610b59565b935060208501359250604085013591506060850135610bc081610b71565b939692955090935050565b60008060408385031215610bde57600080fd5b8235610be981610b59565b91506020830135610bf981610b59565b809150509250929050565b60008060408385031215610c1757600080fd5b8235610c2281610b59565b946020939093013593505050565b803560ff81168114610c4157600080fd5b919050565b600080600080600080600060e0888a031215610c6157600080fd5b8735610c6c81610b59565b9650602088013595506040880135610c8381610b71565b945060608801359350610c9860808901610c30565b925060a0880135915060c0880135905092959891949750929550565b80358015158114610c4157600080fd5b600080600080600060a08688031215610cdc57600080fd5b8535610ce781610b59565b94506020860135610cf781610b59565b93506040860135610d0781610b59565b925060608601359150610d1c60808701610cb4565b90509295509295909350565b600080600060608486031215610d3d57600080fd5b8335610d4881610b59565b95602085013595506040909401359392505050565b600080600060608486031215610d7257600080fd5b8335610d7d81610b59565b9250602084013591506040840135610d9481610b71565b809150509250925092565b60008060408385031215610db257600080fd5b8235610dbd81610b59565b9150610dcb60208401610cb4565b90509250929050565b600080600080600080600060e0888a031215610def57600080fd5b8735610dfa81610b59565b9650602088013595506040880135945060608801359350610c9860808901610c30565b6040516101e0810167ffffffffffffffff81118282101715610e4f57634e487b7160e01b600052604160045260246000fd5b60405290565b600060208284031215610e6757600080fd5b6040516020810181811067ffffffffffffffff82111715610e9857634e487b7160e01b600052604160045260246000fd5b6040529151825250919050565b80516001600160801b0381168114610c4157600080fd5b805164ffffffffff81168114610c4157600080fd5b8051610c4181610b71565b8051610c4181610b59565b60006101e08284031215610efa57600080fd5b610f02610e1d565b610f0c8484610e55565b8152610f1a60208401610ea5565b6020820152610f2b60408401610ea5565b6040820152610f3c60608401610ea5565b6060820152610f4d60808401610ea5565b6080820152610f5e60a08401610ea5565b60a0820152610f6f60c08401610ebc565b60c0820152610f8060e08401610ed1565b60e0820152610100610f93818501610edc565b90820152610120610fa5848201610edc565b90820152610140610fb7848201610edc565b90820152610160610fc9848201610edc565b90820152610180610fdb848201610ea5565b908201526101a0610fed848201610ea5565b908201526101c0610fff848201610ea5565b90820152939250505056fea2646970667358221220c8e438d0783de925732ad433f32ed5990917ec3116c1297d21b3f096282b61de64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct L2Encoder<M>(ethers::contract::Contract<M>);
    impl<M> Clone for L2Encoder<M> {
        fn clone(&self) -> Self {
            L2Encoder(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for L2Encoder<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for L2Encoder<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(L2Encoder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> L2Encoder<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), L2ENCODER_ABI.clone(), client).into()
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
                L2ENCODER_ABI.clone(),
                L2ENCODER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeBorrowParams` (0x1a64acf2) function"]
        pub fn encode_borrow_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [26, 100, 172, 242],
                    (asset, amount, interest_rate_mode, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeLiquidationCall` (0x88d51852) function"]
        pub fn encode_liquidation_call(
            &self,
            collateral_asset: ethers::core::types::Address,
            debt_asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
            debt_to_cover: ethers::core::types::U256,
            receive_a_token: bool,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash(
                    [136, 213, 24, 82],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        receive_a_token,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeRebalanceStableBorrowRate` (0x1a8f6dee) function"]
        pub fn encode_rebalance_stable_borrow_rate(
            &self,
            asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([26, 143, 109, 238], (asset, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeRepayParams` (0x9d2ffc1b) function"]
        pub fn encode_repay_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([157, 47, 252, 27], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeRepayWithATokensParams` (0x8da7fb18) function"]
        pub fn encode_repay_with_a_tokens_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([141, 167, 251, 24], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeRepayWithPermitParams` (0xfed63a93) function"]
        pub fn encode_repay_with_permit_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32], [u8; 32])> {
            self.0
                .method_hash(
                    [254, 214, 58, 147],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeSetUserUseReserveAsCollateral` (0xfc0eed85) function"]
        pub fn encode_set_user_use_reserve_as_collateral(
            &self,
            asset: ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([252, 14, 237, 133], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeSupplyParams` (0xb76398e4) function"]
        pub fn encode_supply_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([183, 99, 152, 228], (asset, amount, referral_code))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeSupplyWithPermitParams` (0x671a7fae) function"]
        pub fn encode_supply_with_permit_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            referral_code: u16,
            deadline: ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32], [u8; 32])> {
            self.0
                .method_hash(
                    [103, 26, 127, 174],
                    (
                        asset,
                        amount,
                        referral_code,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeSwapBorrowRateMode` (0x1fd34797) function"]
        pub fn encode_swap_borrow_rate_mode(
            &self,
            asset: ethers::core::types::Address,
            interest_rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([31, 211, 71, 151], (asset, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeWithdrawParams` (0x5cc7bc10) function"]
        pub fn encode_withdraw_params(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 199, 188, 16], (asset, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for L2Encoder<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    #[doc = "Container type for all input parameters for the `encodeBorrowParams` function with signature `encodeBorrowParams(address,uint256,uint256,uint16)` and selector `[26, 100, 172, 242]`"]
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
        name = "encodeBorrowParams",
        abi = "encodeBorrowParams(address,uint256,uint256,uint16)"
    )]
    pub struct EncodeBorrowParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `encodeLiquidationCall` function with signature `encodeLiquidationCall(address,address,address,uint256,bool)` and selector `[136, 213, 24, 82]`"]
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
        name = "encodeLiquidationCall",
        abi = "encodeLiquidationCall(address,address,address,uint256,bool)"
    )]
    pub struct EncodeLiquidationCallCall {
        pub collateral_asset: ethers::core::types::Address,
        pub debt_asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        pub debt_to_cover: ethers::core::types::U256,
        pub receive_a_token: bool,
    }
    #[doc = "Container type for all input parameters for the `encodeRebalanceStableBorrowRate` function with signature `encodeRebalanceStableBorrowRate(address,address)` and selector `[26, 143, 109, 238]`"]
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
        name = "encodeRebalanceStableBorrowRate",
        abi = "encodeRebalanceStableBorrowRate(address,address)"
    )]
    pub struct EncodeRebalanceStableBorrowRateCall {
        pub asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `encodeRepayParams` function with signature `encodeRepayParams(address,uint256,uint256)` and selector `[157, 47, 252, 27]`"]
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
        name = "encodeRepayParams",
        abi = "encodeRepayParams(address,uint256,uint256)"
    )]
    pub struct EncodeRepayParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `encodeRepayWithATokensParams` function with signature `encodeRepayWithATokensParams(address,uint256,uint256)` and selector `[141, 167, 251, 24]`"]
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
        name = "encodeRepayWithATokensParams",
        abi = "encodeRepayWithATokensParams(address,uint256,uint256)"
    )]
    pub struct EncodeRepayWithATokensParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `encodeRepayWithPermitParams` function with signature `encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[254, 214, 58, 147]`"]
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
        name = "encodeRepayWithPermitParams",
        abi = "encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct EncodeRepayWithPermitParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `encodeSetUserUseReserveAsCollateral` function with signature `encodeSetUserUseReserveAsCollateral(address,bool)` and selector `[252, 14, 237, 133]`"]
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
        name = "encodeSetUserUseReserveAsCollateral",
        abi = "encodeSetUserUseReserveAsCollateral(address,bool)"
    )]
    pub struct EncodeSetUserUseReserveAsCollateralCall {
        pub asset: ethers::core::types::Address,
        pub use_as_collateral: bool,
    }
    #[doc = "Container type for all input parameters for the `encodeSupplyParams` function with signature `encodeSupplyParams(address,uint256,uint16)` and selector `[183, 99, 152, 228]`"]
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
        name = "encodeSupplyParams",
        abi = "encodeSupplyParams(address,uint256,uint16)"
    )]
    pub struct EncodeSupplyParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `encodeSupplyWithPermitParams` function with signature `encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)` and selector `[103, 26, 127, 174]`"]
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
        name = "encodeSupplyWithPermitParams",
        abi = "encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct EncodeSupplyWithPermitParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub referral_code: u16,
        pub deadline: ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `encodeSwapBorrowRateMode` function with signature `encodeSwapBorrowRateMode(address,uint256)` and selector `[31, 211, 71, 151]`"]
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
        name = "encodeSwapBorrowRateMode",
        abi = "encodeSwapBorrowRateMode(address,uint256)"
    )]
    pub struct EncodeSwapBorrowRateModeCall {
        pub asset: ethers::core::types::Address,
        pub interest_rate_mode: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `encodeWithdrawParams` function with signature `encodeWithdrawParams(address,uint256)` and selector `[92, 199, 188, 16]`"]
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
        name = "encodeWithdrawParams",
        abi = "encodeWithdrawParams(address,uint256)"
    )]
    pub struct EncodeWithdrawParamsCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum L2EncoderCalls {
        Pool(PoolCall),
        EncodeBorrowParams(EncodeBorrowParamsCall),
        EncodeLiquidationCall(EncodeLiquidationCallCall),
        EncodeRebalanceStableBorrowRate(EncodeRebalanceStableBorrowRateCall),
        EncodeRepayParams(EncodeRepayParamsCall),
        EncodeRepayWithATokensParams(EncodeRepayWithATokensParamsCall),
        EncodeRepayWithPermitParams(EncodeRepayWithPermitParamsCall),
        EncodeSetUserUseReserveAsCollateral(EncodeSetUserUseReserveAsCollateralCall),
        EncodeSupplyParams(EncodeSupplyParamsCall),
        EncodeSupplyWithPermitParams(EncodeSupplyWithPermitParamsCall),
        EncodeSwapBorrowRateMode(EncodeSwapBorrowRateModeCall),
        EncodeWithdrawParams(EncodeWithdrawParamsCall),
    }
    impl ethers::core::abi::AbiDecode for L2EncoderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(L2EncoderCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <EncodeBorrowParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2EncoderCalls::EncodeBorrowParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeLiquidationCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2EncoderCalls::EncodeLiquidationCall(decoded));
            }
            if let Ok(decoded) =
                <EncodeRebalanceStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeRebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <EncodeRepayParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2EncoderCalls::EncodeRepayParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeRepayWithATokensParamsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeRepayWithATokensParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeRepayWithPermitParamsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeRepayWithPermitParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeSetUserUseReserveAsCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeSetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded) =
                <EncodeSupplyParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2EncoderCalls::EncodeSupplyParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeSupplyWithPermitParamsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeSupplyWithPermitParams(decoded));
            }
            if let Ok(decoded) =
                <EncodeSwapBorrowRateModeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(L2EncoderCalls::EncodeSwapBorrowRateMode(decoded));
            }
            if let Ok(decoded) =
                <EncodeWithdrawParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2EncoderCalls::EncodeWithdrawParams(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for L2EncoderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                L2EncoderCalls::Pool(element) => element.encode(),
                L2EncoderCalls::EncodeBorrowParams(element) => element.encode(),
                L2EncoderCalls::EncodeLiquidationCall(element) => element.encode(),
                L2EncoderCalls::EncodeRebalanceStableBorrowRate(element) => element.encode(),
                L2EncoderCalls::EncodeRepayParams(element) => element.encode(),
                L2EncoderCalls::EncodeRepayWithATokensParams(element) => element.encode(),
                L2EncoderCalls::EncodeRepayWithPermitParams(element) => element.encode(),
                L2EncoderCalls::EncodeSetUserUseReserveAsCollateral(element) => element.encode(),
                L2EncoderCalls::EncodeSupplyParams(element) => element.encode(),
                L2EncoderCalls::EncodeSupplyWithPermitParams(element) => element.encode(),
                L2EncoderCalls::EncodeSwapBorrowRateMode(element) => element.encode(),
                L2EncoderCalls::EncodeWithdrawParams(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for L2EncoderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                L2EncoderCalls::Pool(element) => element.fmt(f),
                L2EncoderCalls::EncodeBorrowParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeLiquidationCall(element) => element.fmt(f),
                L2EncoderCalls::EncodeRebalanceStableBorrowRate(element) => element.fmt(f),
                L2EncoderCalls::EncodeRepayParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeRepayWithATokensParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeRepayWithPermitParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeSetUserUseReserveAsCollateral(element) => element.fmt(f),
                L2EncoderCalls::EncodeSupplyParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeSupplyWithPermitParams(element) => element.fmt(f),
                L2EncoderCalls::EncodeSwapBorrowRateMode(element) => element.fmt(f),
                L2EncoderCalls::EncodeWithdrawParams(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PoolCall> for L2EncoderCalls {
        fn from(var: PoolCall) -> Self {
            L2EncoderCalls::Pool(var)
        }
    }
    impl ::std::convert::From<EncodeBorrowParamsCall> for L2EncoderCalls {
        fn from(var: EncodeBorrowParamsCall) -> Self {
            L2EncoderCalls::EncodeBorrowParams(var)
        }
    }
    impl ::std::convert::From<EncodeLiquidationCallCall> for L2EncoderCalls {
        fn from(var: EncodeLiquidationCallCall) -> Self {
            L2EncoderCalls::EncodeLiquidationCall(var)
        }
    }
    impl ::std::convert::From<EncodeRebalanceStableBorrowRateCall> for L2EncoderCalls {
        fn from(var: EncodeRebalanceStableBorrowRateCall) -> Self {
            L2EncoderCalls::EncodeRebalanceStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<EncodeRepayParamsCall> for L2EncoderCalls {
        fn from(var: EncodeRepayParamsCall) -> Self {
            L2EncoderCalls::EncodeRepayParams(var)
        }
    }
    impl ::std::convert::From<EncodeRepayWithATokensParamsCall> for L2EncoderCalls {
        fn from(var: EncodeRepayWithATokensParamsCall) -> Self {
            L2EncoderCalls::EncodeRepayWithATokensParams(var)
        }
    }
    impl ::std::convert::From<EncodeRepayWithPermitParamsCall> for L2EncoderCalls {
        fn from(var: EncodeRepayWithPermitParamsCall) -> Self {
            L2EncoderCalls::EncodeRepayWithPermitParams(var)
        }
    }
    impl ::std::convert::From<EncodeSetUserUseReserveAsCollateralCall> for L2EncoderCalls {
        fn from(var: EncodeSetUserUseReserveAsCollateralCall) -> Self {
            L2EncoderCalls::EncodeSetUserUseReserveAsCollateral(var)
        }
    }
    impl ::std::convert::From<EncodeSupplyParamsCall> for L2EncoderCalls {
        fn from(var: EncodeSupplyParamsCall) -> Self {
            L2EncoderCalls::EncodeSupplyParams(var)
        }
    }
    impl ::std::convert::From<EncodeSupplyWithPermitParamsCall> for L2EncoderCalls {
        fn from(var: EncodeSupplyWithPermitParamsCall) -> Self {
            L2EncoderCalls::EncodeSupplyWithPermitParams(var)
        }
    }
    impl ::std::convert::From<EncodeSwapBorrowRateModeCall> for L2EncoderCalls {
        fn from(var: EncodeSwapBorrowRateModeCall) -> Self {
            L2EncoderCalls::EncodeSwapBorrowRateMode(var)
        }
    }
    impl ::std::convert::From<EncodeWithdrawParamsCall> for L2EncoderCalls {
        fn from(var: EncodeWithdrawParamsCall) -> Self {
            L2EncoderCalls::EncodeWithdrawParams(var)
        }
    }
    #[doc = "Container type for all return fields from the `POOL` function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `encodeBorrowParams` function with signature `encodeBorrowParams(address,uint256,uint256,uint16)` and selector `[26, 100, 172, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeBorrowParamsReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeLiquidationCall` function with signature `encodeLiquidationCall(address,address,address,uint256,bool)` and selector `[136, 213, 24, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeLiquidationCallReturn(pub [u8; 32], pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeRebalanceStableBorrowRate` function with signature `encodeRebalanceStableBorrowRate(address,address)` and selector `[26, 143, 109, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeRebalanceStableBorrowRateReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeRepayParams` function with signature `encodeRepayParams(address,uint256,uint256)` and selector `[157, 47, 252, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeRepayParamsReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeRepayWithATokensParams` function with signature `encodeRepayWithATokensParams(address,uint256,uint256)` and selector `[141, 167, 251, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeRepayWithATokensParamsReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeRepayWithPermitParams` function with signature `encodeRepayWithPermitParams(address,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `[254, 214, 58, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeRepayWithPermitParamsReturn(pub [u8; 32], pub [u8; 32], pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeSetUserUseReserveAsCollateral` function with signature `encodeSetUserUseReserveAsCollateral(address,bool)` and selector `[252, 14, 237, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeSetUserUseReserveAsCollateralReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeSupplyParams` function with signature `encodeSupplyParams(address,uint256,uint16)` and selector `[183, 99, 152, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeSupplyParamsReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeSupplyWithPermitParams` function with signature `encodeSupplyWithPermitParams(address,uint256,uint16,uint256,uint8,bytes32,bytes32)` and selector `[103, 26, 127, 174]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeSupplyWithPermitParamsReturn(pub [u8; 32], pub [u8; 32], pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeSwapBorrowRateMode` function with signature `encodeSwapBorrowRateMode(address,uint256)` and selector `[31, 211, 71, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeSwapBorrowRateModeReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeWithdrawParams` function with signature `encodeWithdrawParams(address,uint256)` and selector `[92, 199, 188, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EncodeWithdrawParamsReturn(pub [u8; 32]);
}
