pub use loanuser_mod::*;
#[allow(clippy::too_many_arguments)]
mod loanuser_mod {
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
    #[doc = "LoanUser was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LOANUSER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_closeLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_fundLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_makePayment\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_postCollateral\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_returnFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_skim\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LOANUSER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610d85806100206000396000f3fe608060405234801561001057600080fd5b506004361061010b5760003560e01c8063687468d1116100a25780638b196cd5116100715780638b196cd514610239578063afb7a0f61461024c578063b6a0f3311461025f578063d75edca414610272578063fc179d671461028557600080fd5b8063687468d1146101ed5780637327de97146102005780637c2e27d4146102135780637fe255811461022657600080fd5b80633d73ea8f116100de5780633d73ea8f146101a1578063429bf6c8146101b457806351cf23b1146101c7578063574c7844146101da57600080fd5b8063035e30b31461011057806305eee8e7146101385780631649e2ef1461014d5780632893a76e14610180575b600080fd5b61012361011e366004610ac1565b610298565b60405190151581526020015b60405180910390f35b61014b610146366004610a3e565b610351565b005b61016061015b366004610b9b565b6103e3565b60408051948552602085019390935291830152606082015260800161012f565b61019361018e366004610b9b565b610479565b60405190815260200161012f565b6101936101af366004610b9b565b6104fe565b6101606101c2366004610b9b565b61052e565b61014b6101d5366004610ac1565b610562565b61014b6101e8366004610afd565b6105ea565b6101236101fb366004610b9b565b610655565b61012361020e3660046109b0565b610706565b610123610221366004610b9b565b610739565b610123610234366004610ac1565b610760565b610193610247366004610ac1565b6107ee565b61019361025a3660046109b0565b610875565b61012361026d366004610b9b565b6108af565b610123610280366004610b9b565b6108d6565b6101236102933660046109f3565b6108fd565b6040516001600160a01b038381166024830152604482018390526000919085169063e920b1e160e01b906064015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516103049190610c5f565b6000604051808303816000865af19150503d8060008114610341576040519150601f19603f3d011682016040523d82523d6000602084013e610346565b606091505b509095945050505050565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b1580156103c157600080fd5b505af11580156103d5573d6000803e3d6000fd5b505050505050505050505050565b600080600080856001600160a01b031663d05951a0866040518263ffffffff1660e01b815260040161041791815260200190565b608060405180830381600087803b15801561043157600080fd5b505af1158015610445573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104699190610c00565b9299919850965090945092505050565b6040516322baaeeb60e11b8152600481018290526000906001600160a01b038416906345755dd6906024015b602060405180830381600087803b1580156104bf57600080fd5b505af11580156104d3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104f79190610be7565b9392505050565b6040516350f2012f60e01b8152600481018290526000906001600160a01b038416906350f2012f906024016104a5565b600080600080856001600160a01b0316635114cb52866040518263ffffffff1660e01b815260040161041791815260200190565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b1580156105ac57600080fd5b505af11580156105c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e49190610bc5565b50505050565b604051632b2d48ad60e21b81526001600160a01b0386169063acb522b49061061c908790879087908790600401610c9a565b600060405180830381600087803b15801561063657600080fd5b505af115801561064a573d6000803e3d6000fd5b505050505050505050565b6000826001600160a01b03166350f2012f60e01b8360405160240161067c91815260200190565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516106ba9190610c5f565b6000604051808303816000865af19150503d80600081146106f7576040519150601f19603f3d011682016040523d82523d6000602084013e6106fc565b606091505b5090949350505050565b6040516001600160a01b03838116602483015282811660448301526000919085169063712b772f60e01b906064016102c6565b6000826001600160a01b03166345755dd660e01b8360405160240161067c91815260200190565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b1580156107ae57600080fd5b505af11580156107c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107e69190610bc5565b949350505050565b60405163e920b1e160e01b81526001600160a01b038381166004830152602482018390526000919085169063e920b1e1906044015b602060405180830381600087803b15801561083d57600080fd5b505af1158015610851573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107e69190610be7565b60405163712b772f60e01b81526001600160a01b03838116600483015282811660248301526000919085169063712b772f90604401610823565b6000826001600160a01b031663d05951a060e01b8360405160240161067c91815260200190565b6000826001600160a01b0316635114cb5260e01b8360405160240161067c91815260200190565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b15801561095357600080fd5b505af1158015610967573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098b9190610bc5565b95945050505050565b80356001600160a01b03811681146109ab57600080fd5b919050565b6000806000606084860312156109c557600080fd5b6109ce84610994565b92506109dc60208501610994565b91506109ea60408501610994565b90509250925092565b60008060008060808587031215610a0957600080fd5b610a1285610994565b9350610a2060208601610994565b9250610a2e60408601610994565b9396929550929360600135925050565b600080600080600080600080610100898b031215610a5b57600080fd5b610a6489610994565b9750610a7260208a01610994565b9650610a8060408a01610994565b9550606089013594506080890135935060a089013560ff81168114610aa457600080fd5b979a969950949793969295929450505060c08201359160e0013590565b600080600060608486031215610ad657600080fd5b610adf84610994565b9250610aed60208501610994565b9150604084013590509250925092565b600080600080600060808688031215610b1557600080fd5b610b1e86610994565b9450610b2c60208701610994565b935060408601359250606086013567ffffffffffffffff80821115610b5057600080fd5b818801915088601f830112610b6457600080fd5b813581811115610b7357600080fd5b8960208260051b8501011115610b8857600080fd5b9699959850939650602001949392505050565b60008060408385031215610bae57600080fd5b610bb783610994565b946020939093013593505050565b600060208284031215610bd757600080fd5b815180151581146104f757600080fd5b600060208284031215610bf957600080fd5b5051919050565b60008060008060808587031215610c1657600080fd5b505082516020840151604085015160609095015191969095509092509050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b81811015610c805760208186018101518583015201610c66565b81811115610c8f576000828501525b509190910192915050565b6001600160a01b0385168152602080820185905260606040830181905282018390526000906080600585901b840181019190840186845b87811015610d4057868503607f190183528135368a9003601e19018112610cf757600080fd5b8901803567ffffffffffffffff811115610d1057600080fd5b8036038b1315610d1f57600080fd5b610d2c8782888501610c36565b965050509183019190830190600101610cd1565b5092999850505050505050505056fea26469706673582212208699aa5bf5f70f2bebf686f81c6a92f0953b5b1c5a666dd89d43342941eb679e64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct LoanUser<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for LoanUser<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LoanUser<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LoanUser))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> LoanUser<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LOANUSER_ABI.clone(), client).into()
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
                LOANUSER_ABI.clone(),
                LOANUSER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `erc20_approve` (0x51cf23b1) function"]
        pub fn erc_20_approve(
            &self,
            token: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 207, 35, 177], (token, spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_permit` (0x05eee8e7) function"]
        pub fn erc_20_permit(
            &self,
            token: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [5, 238, 232, 231],
                    (token, owner, spender, amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_transfer` (0x7fe25581) function"]
        pub fn erc_20_transfer(
            &self,
            token: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 226, 85, 129], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erc20_transferFrom` (0xfc179d67) function"]
        pub fn erc_20_transfer_from(
            &self,
            token: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 23, 157, 103], (token, owner, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_closeLoan` (0x1649e2ef) function"]
        pub fn loan_close_loan(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([22, 73, 226, 239], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_fundLoan` (0x8b196cd5) function"]
        pub fn loan_fund_loan(
            &self,
            loan: ethers::core::types::Address,
            lender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 25, 108, 213], (loan, lender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_makePayment` (0x429bf6c8) function"]
        pub fn loan_make_payment(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([66, 155, 246, 200], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_postCollateral` (0x3d73ea8f) function"]
        pub fn loan_post_collateral(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([61, 115, 234, 143], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_rejectNewTerms` (0x574c7844) function"]
        pub fn loan_reject_new_terms(
            &self,
            loan: ethers::core::types::Address,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 76, 120, 68], (loan, refinancer, deadline, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_returnFunds` (0x2893a76e) function"]
        pub fn loan_return_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([40, 147, 167, 110], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_skim` (0xafb7a0f6) function"]
        pub fn loan_skim(
            &self,
            loan: ethers::core::types::Address,
            token: ethers::core::types::Address,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([175, 183, 160, 246], (loan, token, destination))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_closeLoan` (0xb6a0f331) function"]
        pub fn try_loan_close_loan(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([182, 160, 243, 49], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_fundLoan` (0x035e30b3) function"]
        pub fn try_loan_fund_loan(
            &self,
            loan: ethers::core::types::Address,
            lender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([3, 94, 48, 179], (loan, lender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_makePayment` (0xd75edca4) function"]
        pub fn try_loan_make_payment(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([215, 94, 220, 164], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_postCollateral` (0x687468d1) function"]
        pub fn try_loan_post_collateral(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([104, 116, 104, 209], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_returnFunds` (0x7c2e27d4) function"]
        pub fn try_loan_return_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([124, 46, 39, 212], (loan, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_skim` (0x7327de97) function"]
        pub fn try_loan_skim(
            &self,
            loan: ethers::core::types::Address,
            token: ethers::core::types::Address,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 39, 222, 151], (loan, token, destination))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LoanUser<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `erc20_approve`function with signature `erc20_approve(address,address,uint256)` and selector `[81, 207, 35, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "erc20_approve", abi = "erc20_approve(address,address,uint256)")]
    pub struct Erc20ApproveCall {
        pub token: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `erc20_permit`function with signature `erc20_permit(address,address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[5, 238, 232, 231]`"]
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
        name = "erc20_permit",
        abi = "erc20_permit(address,address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct Erc20PermitCall {
        pub token: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `erc20_transfer`function with signature `erc20_transfer(address,address,uint256)` and selector `[127, 226, 85, 129]`"]
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
        name = "erc20_transfer",
        abi = "erc20_transfer(address,address,uint256)"
    )]
    pub struct Erc20TransferCall {
        pub token: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `erc20_transferFrom`function with signature `erc20_transferFrom(address,address,address,uint256)` and selector `[252, 23, 157, 103]`"]
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
        name = "erc20_transferFrom",
        abi = "erc20_transferFrom(address,address,address,uint256)"
    )]
    pub struct Erc20TransferFromCall {
        pub token: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_closeLoan`function with signature `loan_closeLoan(address,uint256)` and selector `[22, 73, 226, 239]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_closeLoan", abi = "loan_closeLoan(address,uint256)")]
    pub struct LoanCloseLoanCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_fundLoan`function with signature `loan_fundLoan(address,address,uint256)` and selector `[139, 25, 108, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_fundLoan", abi = "loan_fundLoan(address,address,uint256)")]
    pub struct LoanFundLoanCall {
        pub loan: ethers::core::types::Address,
        pub lender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_makePayment`function with signature `loan_makePayment(address,uint256)` and selector `[66, 155, 246, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_makePayment", abi = "loan_makePayment(address,uint256)")]
    pub struct LoanMakePaymentCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_postCollateral`function with signature `loan_postCollateral(address,uint256)` and selector `[61, 115, 234, 143]`"]
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
        name = "loan_postCollateral",
        abi = "loan_postCollateral(address,uint256)"
    )]
    pub struct LoanPostCollateralCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_rejectNewTerms`function with signature `loan_rejectNewTerms(address,address,uint256,bytes[])` and selector `[87, 76, 120, 68]`"]
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
        name = "loan_rejectNewTerms",
        abi = "loan_rejectNewTerms(address,address,uint256,bytes[])"
    )]
    pub struct LoanRejectNewTermsCall {
        pub loan: ethers::core::types::Address,
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `loan_returnFunds`function with signature `loan_returnFunds(address,uint256)` and selector `[40, 147, 167, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_returnFunds", abi = "loan_returnFunds(address,uint256)")]
    pub struct LoanReturnFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_skim`function with signature `loan_skim(address,address,address)` and selector `[175, 183, 160, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_skim", abi = "loan_skim(address,address,address)")]
    pub struct LoanSkimCall {
        pub loan: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_loan_closeLoan`function with signature `try_loan_closeLoan(address,uint256)` and selector `[182, 160, 243, 49]`"]
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
        name = "try_loan_closeLoan",
        abi = "try_loan_closeLoan(address,uint256)"
    )]
    pub struct TryLoanCloseLoanCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_fundLoan`function with signature `try_loan_fundLoan(address,address,uint256)` and selector `[3, 94, 48, 179]`"]
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
        name = "try_loan_fundLoan",
        abi = "try_loan_fundLoan(address,address,uint256)"
    )]
    pub struct TryLoanFundLoanCall {
        pub loan: ethers::core::types::Address,
        pub lender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_makePayment`function with signature `try_loan_makePayment(address,uint256)` and selector `[215, 94, 220, 164]`"]
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
        name = "try_loan_makePayment",
        abi = "try_loan_makePayment(address,uint256)"
    )]
    pub struct TryLoanMakePaymentCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_postCollateral`function with signature `try_loan_postCollateral(address,uint256)` and selector `[104, 116, 104, 209]`"]
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
        name = "try_loan_postCollateral",
        abi = "try_loan_postCollateral(address,uint256)"
    )]
    pub struct TryLoanPostCollateralCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_returnFunds`function with signature `try_loan_returnFunds(address,uint256)` and selector `[124, 46, 39, 212]`"]
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
        name = "try_loan_returnFunds",
        abi = "try_loan_returnFunds(address,uint256)"
    )]
    pub struct TryLoanReturnFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_skim`function with signature `try_loan_skim(address,address,address)` and selector `[115, 39, 222, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "try_loan_skim", abi = "try_loan_skim(address,address,address)")]
    pub struct TryLoanSkimCall {
        pub loan: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LoanUserCalls {
        Erc20Approve(Erc20ApproveCall),
        Erc20Permit(Erc20PermitCall),
        Erc20Transfer(Erc20TransferCall),
        Erc20TransferFrom(Erc20TransferFromCall),
        LoanCloseLoan(LoanCloseLoanCall),
        LoanFundLoan(LoanFundLoanCall),
        LoanMakePayment(LoanMakePaymentCall),
        LoanPostCollateral(LoanPostCollateralCall),
        LoanRejectNewTerms(LoanRejectNewTermsCall),
        LoanReturnFunds(LoanReturnFundsCall),
        LoanSkim(LoanSkimCall),
        TryLoanCloseLoan(TryLoanCloseLoanCall),
        TryLoanFundLoan(TryLoanFundLoanCall),
        TryLoanMakePayment(TryLoanMakePaymentCall),
        TryLoanPostCollateral(TryLoanPostCollateralCall),
        TryLoanReturnFunds(TryLoanReturnFundsCall),
        TryLoanSkim(TryLoanSkimCall),
    }
    impl ethers::core::abi::AbiDecode for LoanUserCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Erc20ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::Erc20Approve(decoded));
            }
            if let Ok(decoded) =
                <Erc20PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::Erc20Permit(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::Erc20Transfer(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::Erc20TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <LoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <LoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <LoanRejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanRejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::LoanSkim(decoded));
            }
            if let Ok(decoded) =
                <TryLoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <TryLoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <TryLoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LoanUserCalls::TryLoanSkim(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LoanUserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LoanUserCalls::Erc20Approve(element) => element.encode(),
                LoanUserCalls::Erc20Permit(element) => element.encode(),
                LoanUserCalls::Erc20Transfer(element) => element.encode(),
                LoanUserCalls::Erc20TransferFrom(element) => element.encode(),
                LoanUserCalls::LoanCloseLoan(element) => element.encode(),
                LoanUserCalls::LoanFundLoan(element) => element.encode(),
                LoanUserCalls::LoanMakePayment(element) => element.encode(),
                LoanUserCalls::LoanPostCollateral(element) => element.encode(),
                LoanUserCalls::LoanRejectNewTerms(element) => element.encode(),
                LoanUserCalls::LoanReturnFunds(element) => element.encode(),
                LoanUserCalls::LoanSkim(element) => element.encode(),
                LoanUserCalls::TryLoanCloseLoan(element) => element.encode(),
                LoanUserCalls::TryLoanFundLoan(element) => element.encode(),
                LoanUserCalls::TryLoanMakePayment(element) => element.encode(),
                LoanUserCalls::TryLoanPostCollateral(element) => element.encode(),
                LoanUserCalls::TryLoanReturnFunds(element) => element.encode(),
                LoanUserCalls::TryLoanSkim(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LoanUserCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LoanUserCalls::Erc20Approve(element) => element.fmt(f),
                LoanUserCalls::Erc20Permit(element) => element.fmt(f),
                LoanUserCalls::Erc20Transfer(element) => element.fmt(f),
                LoanUserCalls::Erc20TransferFrom(element) => element.fmt(f),
                LoanUserCalls::LoanCloseLoan(element) => element.fmt(f),
                LoanUserCalls::LoanFundLoan(element) => element.fmt(f),
                LoanUserCalls::LoanMakePayment(element) => element.fmt(f),
                LoanUserCalls::LoanPostCollateral(element) => element.fmt(f),
                LoanUserCalls::LoanRejectNewTerms(element) => element.fmt(f),
                LoanUserCalls::LoanReturnFunds(element) => element.fmt(f),
                LoanUserCalls::LoanSkim(element) => element.fmt(f),
                LoanUserCalls::TryLoanCloseLoan(element) => element.fmt(f),
                LoanUserCalls::TryLoanFundLoan(element) => element.fmt(f),
                LoanUserCalls::TryLoanMakePayment(element) => element.fmt(f),
                LoanUserCalls::TryLoanPostCollateral(element) => element.fmt(f),
                LoanUserCalls::TryLoanReturnFunds(element) => element.fmt(f),
                LoanUserCalls::TryLoanSkim(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Erc20ApproveCall> for LoanUserCalls {
        fn from(var: Erc20ApproveCall) -> Self {
            LoanUserCalls::Erc20Approve(var)
        }
    }
    impl ::std::convert::From<Erc20PermitCall> for LoanUserCalls {
        fn from(var: Erc20PermitCall) -> Self {
            LoanUserCalls::Erc20Permit(var)
        }
    }
    impl ::std::convert::From<Erc20TransferCall> for LoanUserCalls {
        fn from(var: Erc20TransferCall) -> Self {
            LoanUserCalls::Erc20Transfer(var)
        }
    }
    impl ::std::convert::From<Erc20TransferFromCall> for LoanUserCalls {
        fn from(var: Erc20TransferFromCall) -> Self {
            LoanUserCalls::Erc20TransferFrom(var)
        }
    }
    impl ::std::convert::From<LoanCloseLoanCall> for LoanUserCalls {
        fn from(var: LoanCloseLoanCall) -> Self {
            LoanUserCalls::LoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<LoanFundLoanCall> for LoanUserCalls {
        fn from(var: LoanFundLoanCall) -> Self {
            LoanUserCalls::LoanFundLoan(var)
        }
    }
    impl ::std::convert::From<LoanMakePaymentCall> for LoanUserCalls {
        fn from(var: LoanMakePaymentCall) -> Self {
            LoanUserCalls::LoanMakePayment(var)
        }
    }
    impl ::std::convert::From<LoanPostCollateralCall> for LoanUserCalls {
        fn from(var: LoanPostCollateralCall) -> Self {
            LoanUserCalls::LoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<LoanRejectNewTermsCall> for LoanUserCalls {
        fn from(var: LoanRejectNewTermsCall) -> Self {
            LoanUserCalls::LoanRejectNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanReturnFundsCall> for LoanUserCalls {
        fn from(var: LoanReturnFundsCall) -> Self {
            LoanUserCalls::LoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<LoanSkimCall> for LoanUserCalls {
        fn from(var: LoanSkimCall) -> Self {
            LoanUserCalls::LoanSkim(var)
        }
    }
    impl ::std::convert::From<TryLoanCloseLoanCall> for LoanUserCalls {
        fn from(var: TryLoanCloseLoanCall) -> Self {
            LoanUserCalls::TryLoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanFundLoanCall> for LoanUserCalls {
        fn from(var: TryLoanFundLoanCall) -> Self {
            LoanUserCalls::TryLoanFundLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanMakePaymentCall> for LoanUserCalls {
        fn from(var: TryLoanMakePaymentCall) -> Self {
            LoanUserCalls::TryLoanMakePayment(var)
        }
    }
    impl ::std::convert::From<TryLoanPostCollateralCall> for LoanUserCalls {
        fn from(var: TryLoanPostCollateralCall) -> Self {
            LoanUserCalls::TryLoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<TryLoanReturnFundsCall> for LoanUserCalls {
        fn from(var: TryLoanReturnFundsCall) -> Self {
            LoanUserCalls::TryLoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanSkimCall> for LoanUserCalls {
        fn from(var: TryLoanSkimCall) -> Self {
            LoanUserCalls::TryLoanSkim(var)
        }
    }
}
