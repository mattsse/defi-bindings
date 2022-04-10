pub use lendermock_mod::*;
#[allow(clippy::too_many_arguments)]
mod lendermock_mod {
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
    #[doc = "LenderMock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LENDERMOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_acceptNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_claimFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolDelegate\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"poolDelegate_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolDelegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_acceptLender\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_acceptNewTerms\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_claimFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_closeLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_fundLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_makePayment\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_postCollateral\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_repossess\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_returnFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_setPendingLender\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_skim\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LENDERMOCK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052600080546001600160a01b031916600817905534801561002357600080fd5b506114d6806100336000396000f3fe608060405234801561001057600080fd5b50600436106101cf5760003560e01c80636fc7d91111610104578063ac8a0501116100a2578063c2b63f4f11610071578063c2b63f4f1461043e578063d75edca414610451578063ec16a8d814610464578063fc179d671461047757600080fd5b8063ac8a0501146103f2578063afb7a0f614610405578063b6a0f33114610418578063c1a3c9d71461042b57600080fd5b80637fe25581116100de5780637fe25581146103a65780638b196cd5146103b957806398e5c234146103cc578063a18814da146103df57600080fd5b80636fc7d911146103505780637327de97146103805780637c2e27d41461039357600080fd5b80634046af2b1161017157806351cf23b11161014b57806351cf23b114610304578063574c784414610317578063687468d11461032a5780636c202fcb1461033d57600080fd5b80634046af2b1461029e578063429bf6c8146102c95780635100d121146102dc57600080fd5b80631649e2ef116101ad5780631649e2ef14610224578063188b80b8146102575780632893a76e1461026a5780633d73ea8f1461028b57600080fd5b8063035e30b3146101d457806305eee8e7146101fc5780631515a11614610211575b600080fd5b6101e76101e236600461112b565b61048a565b60405190151581526020015b60405180910390f35b61020f61020a3660046110a8565b610543565b005b61020f61021f366004610fcc565b6105d5565b61023761023236600461124d565b61062b565b6040805194855260208501939093529183015260608201526080016101f3565b61020f6102653660046111d6565b6106c1565b61027d61027836600461124d565b61072f565b6040519081526020016101f3565b61027d61029936600461124d565b6107b4565b6000546102b1906001600160a01b031681565b6040516001600160a01b0390911681526020016101f3565b6102376102d736600461124d565b6107e4565b6102ef6102ea366004610fe7565b610818565b604080519283526020830191909152016101f3565b61020f61031236600461112b565b6108a3565b61020f610325366004611167565b61092b565b6101e761033836600461124d565b610996565b6101e761034b3660046111d6565b610a47565b61020f61035e366004610fcc565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6101e761038e36600461101a565b610b02565b6101e76103a136600461124d565b610b35565b6101e76103b436600461112b565b610b5c565b61027d6103c736600461112b565b610bea565b6101e76103da366004610fe7565b610c71565b6101e76103ed366004611277565b610c9c565b61020f610400366004610fe7565b610cce565b61027d61041336600461101a565b610d2d565b6101e761042636600461124d565b610d67565b6101e7610439366004610fe7565b610d8e565b61020f61044c366004611277565b610db9565b6101e761045f36600461124d565b610e20565b6101e7610472366004610fcc565b610e47565b6101e761048536600461105d565b610ed4565b6040516001600160a01b038381166024830152604482018390526000919085169063e920b1e160e01b906064015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516104f691906113f9565b6000604051808303816000865af19150503d8060008114610533576040519150601f19603f3d011682016040523d82523d6000602084013e610538565b606091505b509095945050505050565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b1580156105b357600080fd5b505af11580156105c7573d6000803e3d6000fd5b505050505050505050505050565b806001600160a01b0316630fe3d9b76040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561061057600080fd5b505af1158015610624573d6000803e3d6000fd5b5050505050565b600080600080856001600160a01b031663d05951a0866040518263ffffffff1660e01b815260040161065f91815260200190565b608060405180830381600087803b15801561067957600080fd5b505af115801561068d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106b19190611309565b9299919850965090945092505050565b6040516328565a7760e11b81526001600160a01b038716906350acb4ee906106f59088908890889088908890600401611466565b600060405180830381600087803b15801561070f57600080fd5b505af1158015610723573d6000803e3d6000fd5b50505050505050505050565b6040516322baaeeb60e11b8152600481018290526000906001600160a01b038416906345755dd6906024015b602060405180830381600087803b15801561077557600080fd5b505af1158015610789573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107ad91906112cc565b9392505050565b6040516350f2012f60e01b8152600481018290526000906001600160a01b038416906350f2012f9060240161075b565b600080600080856001600160a01b0316635114cb52866040518263ffffffff1660e01b815260040161065f91815260200190565b6040516347350e9f60e01b81526001600160a01b03828116600483015260009182918516906347350e9f906024016040805180830381600087803b15801561085f57600080fd5b505af1158015610873573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089791906112e5565b915091505b9250929050565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b1580156108ed57600080fd5b505af1158015610901573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092591906112aa565b50505050565b604051632b2d48ad60e21b81526001600160a01b0386169063acb522b49061095d908790879087908790600401611434565b600060405180830381600087803b15801561097757600080fd5b505af115801561098b573d6000803e3d6000fd5b505050505050505050565b6000826001600160a01b03166350f2012f60e01b836040516024016109bd91815260200190565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516109fb91906113f9565b6000604051808303816000865af19150503d8060008114610a38576040519150601f19603f3d011682016040523d82523d6000602084013e610a3d565b606091505b5090949350505050565b6000866001600160a01b03166350acb4ee60e01b8787878787604051602401610a74959493929190611466565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610ab291906113f9565b6000604051808303816000865af19150503d8060008114610aef576040519150601f19603f3d011682016040523d82523d6000602084013e610af4565b606091505b509098975050505050505050565b6040516001600160a01b03838116602483015282811660448301526000919085169063712b772f60e01b906064016104b8565b6000826001600160a01b03166345755dd660e01b836040516024016109bd91815260200190565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b158015610baa57600080fd5b505af1158015610bbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be291906112aa565b949350505050565b60405163e920b1e160e01b81526001600160a01b038381166004830152602482018390526000919085169063e920b1e1906044015b602060405180830381600087803b158015610c3957600080fd5b505af1158015610c4d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be291906112cc565b6040516001600160a01b038281166024830152600091908416906347350e9f60e01b906044016109bd565b604051602481018390526001600160a01b0382811660448301526000919085169063390d685560e01b906064016104b8565b60405163267f4ac360e01b81526001600160a01b03828116600483015283169063267f4ac390602401600060405180830381600087803b158015610d1157600080fd5b505af1158015610d25573d6000803e3d6000fd5b505050505050565b60405163712b772f60e01b81526001600160a01b03838116600483015282811660248301526000919085169063712b772f90604401610c1f565b6000826001600160a01b031663d05951a060e01b836040516024016109bd91815260200190565b6040516001600160a01b0382811660248301526000919084169063267f4ac360e01b906044016109bd565b60405163390d685560e01b8152600481018390526001600160a01b03828116602483015284169063390d685590604401600060405180830381600087803b158015610e0357600080fd5b505af1158015610e17573d6000803e3d6000fd5b50505050505050565b6000826001600160a01b0316635114cb5260e01b836040516024016109bd91815260200190565b60408051600481526024810182526020810180516001600160e01b0316630fe3d9b760e01b17905290516000916001600160a01b03841691610e8991906113f9565b6000604051808303816000865af19150503d8060008114610ec6576040519150601f19603f3d011682016040523d82523d6000602084013e610ecb565b606091505b50909392505050565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b158015610f2a57600080fd5b505af1158015610f3e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f6291906112aa565b95945050505050565b80356001600160a01b0381168114610f8257600080fd5b919050565b60008083601f840112610f9957600080fd5b50813567ffffffffffffffff811115610fb157600080fd5b6020830191508360208260051b850101111561089c57600080fd5b600060208284031215610fde57600080fd5b6107ad82610f6b565b60008060408385031215610ffa57600080fd5b61100383610f6b565b915061101160208401610f6b565b90509250929050565b60008060006060848603121561102f57600080fd5b61103884610f6b565b925061104660208501610f6b565b915061105460408501610f6b565b90509250925092565b6000806000806080858703121561107357600080fd5b61107c85610f6b565b935061108a60208601610f6b565b925061109860408601610f6b565b9396929550929360600135925050565b600080600080600080600080610100898b0312156110c557600080fd5b6110ce89610f6b565b97506110dc60208a01610f6b565b96506110ea60408a01610f6b565b9550606089013594506080890135935060a089013560ff8116811461110e57600080fd5b979a969950949793969295929450505060c08201359160e0013590565b60008060006060848603121561114057600080fd5b61114984610f6b565b925061115760208501610f6b565b9150604084013590509250925092565b60008060008060006080868803121561117f57600080fd5b61118886610f6b565b945061119660208701610f6b565b935060408601359250606086013567ffffffffffffffff8111156111b957600080fd5b6111c588828901610f87565b969995985093965092949392505050565b60008060008060008060a087890312156111ef57600080fd5b6111f887610f6b565b955061120660208801610f6b565b945060408701359350606087013567ffffffffffffffff81111561122957600080fd5b61123589828a01610f87565b979a9699509497949695608090950135949350505050565b6000806040838503121561126057600080fd5b61126983610f6b565b946020939093013593505050565b60008060006060848603121561128c57600080fd5b61129584610f6b565b92506020840135915061105460408501610f6b565b6000602082840312156112bc57600080fd5b815180151581146107ad57600080fd5b6000602082840312156112de57600080fd5b5051919050565b600080604083850312156112f857600080fd5b505080516020909101519092909150565b6000806000806080858703121561131f57600080fd5b505082516020840151604085015160609095015191969095509092509050565b81835260006020808501808196508560051b810191508460005b878110156113c35782840389528135601e1988360301811261137a57600080fd5b8701803567ffffffffffffffff81111561139357600080fd5b8036038913156113a257600080fd5b6113af86828985016113d0565b9a87019a9550505090840190600101611359565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b8181101561141a5760208186018101518583015201611400565b81811115611429576000828501525b509190910192915050565b60018060a01b038516815283602082015260606040820152600061145c60608301848661133f565b9695505050505050565b60018060a01b038616815284602082015260806040820152600061148e60808301858761133f565b9050826060830152969550505050505056fea2646970667358221220301d0fee16738fb00c4d4c34da46b6b890b0014431a7574e9b2ea66fcb50d08564736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct LenderMock<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for LenderMock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LenderMock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LenderMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> LenderMock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LENDERMOCK_ABI.clone(), client).into()
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
                LENDERMOCK_ABI.clone(),
                LENDERMOCK_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `loan_acceptLender` (0x1515a116) function"]
        pub fn loan_accept_lender(
            &self,
            loan: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 21, 161, 22], loan)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_acceptNewTerms` (0x188b80b8) function"]
        pub fn loan_accept_new_terms(
            &self,
            loan: ethers::core::types::Address,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [24, 139, 128, 184],
                    (loan, refinancer, deadline, calls, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loan_claimFunds` (0xc2b63f4f) function"]
        pub fn loan_claim_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 182, 63, 79], (loan, amount, destination))
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
        #[doc = "Calls the contract's `loan_repossess` (0x5100d121) function"]
        pub fn loan_repossess(
            &self,
            loan: ethers::core::types::Address,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([81, 0, 209, 33], (loan, destination))
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
        #[doc = "Calls the contract's `loan_setPendingLender` (0xac8a0501) function"]
        pub fn loan_set_pending_lender(
            &self,
            loan: ethers::core::types::Address,
            lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 138, 5, 1], (loan, lender))
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
        #[doc = "Calls the contract's `poolDelegate` (0x4046af2b) function"]
        pub fn pool_delegate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([64, 70, 175, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolDelegate` (0x6fc7d911) function"]
        pub fn set_pool_delegate(
            &self,
            pool_delegate: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 199, 217, 17], pool_delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_acceptLender` (0xec16a8d8) function"]
        pub fn try_loan_accept_lender(
            &self,
            loan: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([236, 22, 168, 216], loan)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_acceptNewTerms` (0x6c202fcb) function"]
        pub fn try_loan_accept_new_terms(
            &self,
            loan: ethers::core::types::Address,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [108, 32, 47, 203],
                    (loan, refinancer, deadline, calls, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_claimFunds` (0xa18814da) function"]
        pub fn try_loan_claim_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([161, 136, 20, 218], (loan, amount, destination))
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
        #[doc = "Calls the contract's `try_loan_repossess` (0x98e5c234) function"]
        pub fn try_loan_repossess(
            &self,
            loan: ethers::core::types::Address,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([152, 229, 194, 52], (loan, destination))
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
        #[doc = "Calls the contract's `try_loan_setPendingLender` (0xc1a3c9d7) function"]
        pub fn try_loan_set_pending_lender(
            &self,
            loan: ethers::core::types::Address,
            lender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([193, 163, 201, 215], (loan, lender))
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LenderMock<M> {
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
    #[doc = "Container type for all input parameters for the `loan_acceptLender`function with signature `loan_acceptLender(address)` and selector `[21, 21, 161, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_acceptLender", abi = "loan_acceptLender(address)")]
    pub struct LoanAcceptLenderCall {
        pub loan: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `loan_acceptNewTerms`function with signature `loan_acceptNewTerms(address,address,uint256,bytes[],uint256)` and selector `[24, 139, 128, 184]`"]
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
        name = "loan_acceptNewTerms",
        abi = "loan_acceptNewTerms(address,address,uint256,bytes[],uint256)"
    )]
    pub struct LoanAcceptNewTermsCall {
        pub loan: ethers::core::types::Address,
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `loan_claimFunds`function with signature `loan_claimFunds(address,uint256,address)` and selector `[194, 182, 63, 79]`"]
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
        name = "loan_claimFunds",
        abi = "loan_claimFunds(address,uint256,address)"
    )]
    pub struct LoanClaimFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `loan_repossess`function with signature `loan_repossess(address,address)` and selector `[81, 0, 209, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_repossess", abi = "loan_repossess(address,address)")]
    pub struct LoanRepossessCall {
        pub loan: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `loan_setPendingLender`function with signature `loan_setPendingLender(address,address)` and selector `[172, 138, 5, 1]`"]
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
        name = "loan_setPendingLender",
        abi = "loan_setPendingLender(address,address)"
    )]
    pub struct LoanSetPendingLenderCall {
        pub loan: ethers::core::types::Address,
        pub lender: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `poolDelegate`function with signature `poolDelegate()` and selector `[64, 70, 175, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolDelegate", abi = "poolDelegate()")]
    pub struct PoolDelegateCall;
    #[doc = "Container type for all input parameters for the `setPoolDelegate`function with signature `setPoolDelegate(address)` and selector `[111, 199, 217, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolDelegate", abi = "setPoolDelegate(address)")]
    pub struct SetPoolDelegateCall {
        pub pool_delegate: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_loan_acceptLender`function with signature `try_loan_acceptLender(address)` and selector `[236, 22, 168, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "try_loan_acceptLender", abi = "try_loan_acceptLender(address)")]
    pub struct TryLoanAcceptLenderCall {
        pub loan: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `try_loan_acceptNewTerms`function with signature `try_loan_acceptNewTerms(address,address,uint256,bytes[],uint256)` and selector `[108, 32, 47, 203]`"]
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
        name = "try_loan_acceptNewTerms",
        abi = "try_loan_acceptNewTerms(address,address,uint256,bytes[],uint256)"
    )]
    pub struct TryLoanAcceptNewTermsCall {
        pub loan: ethers::core::types::Address,
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `try_loan_claimFunds`function with signature `try_loan_claimFunds(address,uint256,address)` and selector `[161, 136, 20, 218]`"]
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
        name = "try_loan_claimFunds",
        abi = "try_loan_claimFunds(address,uint256,address)"
    )]
    pub struct TryLoanClaimFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `try_loan_repossess`function with signature `try_loan_repossess(address,address)` and selector `[152, 229, 194, 52]`"]
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
        name = "try_loan_repossess",
        abi = "try_loan_repossess(address,address)"
    )]
    pub struct TryLoanRepossessCall {
        pub loan: ethers::core::types::Address,
        pub destination: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `try_loan_setPendingLender`function with signature `try_loan_setPendingLender(address,address)` and selector `[193, 163, 201, 215]`"]
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
        name = "try_loan_setPendingLender",
        abi = "try_loan_setPendingLender(address,address)"
    )]
    pub struct TryLoanSetPendingLenderCall {
        pub loan: ethers::core::types::Address,
        pub lender: ethers::core::types::Address,
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
    pub enum LenderMockCalls {
        Erc20Approve(Erc20ApproveCall),
        Erc20Permit(Erc20PermitCall),
        Erc20Transfer(Erc20TransferCall),
        Erc20TransferFrom(Erc20TransferFromCall),
        LoanAcceptLender(LoanAcceptLenderCall),
        LoanAcceptNewTerms(LoanAcceptNewTermsCall),
        LoanClaimFunds(LoanClaimFundsCall),
        LoanCloseLoan(LoanCloseLoanCall),
        LoanFundLoan(LoanFundLoanCall),
        LoanMakePayment(LoanMakePaymentCall),
        LoanPostCollateral(LoanPostCollateralCall),
        LoanRejectNewTerms(LoanRejectNewTermsCall),
        LoanRepossess(LoanRepossessCall),
        LoanReturnFunds(LoanReturnFundsCall),
        LoanSetPendingLender(LoanSetPendingLenderCall),
        LoanSkim(LoanSkimCall),
        PoolDelegate(PoolDelegateCall),
        SetPoolDelegate(SetPoolDelegateCall),
        TryLoanAcceptLender(TryLoanAcceptLenderCall),
        TryLoanAcceptNewTerms(TryLoanAcceptNewTermsCall),
        TryLoanClaimFunds(TryLoanClaimFundsCall),
        TryLoanCloseLoan(TryLoanCloseLoanCall),
        TryLoanFundLoan(TryLoanFundLoanCall),
        TryLoanMakePayment(TryLoanMakePaymentCall),
        TryLoanPostCollateral(TryLoanPostCollateralCall),
        TryLoanRepossess(TryLoanRepossessCall),
        TryLoanReturnFunds(TryLoanReturnFundsCall),
        TryLoanSetPendingLender(TryLoanSetPendingLenderCall),
        TryLoanSkim(TryLoanSkimCall),
    }
    impl ethers::core::abi::AbiDecode for LenderMockCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Erc20ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::Erc20Approve(decoded));
            }
            if let Ok(decoded) =
                <Erc20PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::Erc20Permit(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::Erc20Transfer(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::Erc20TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <LoanAcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanAcceptLender(decoded));
            }
            if let Ok(decoded) =
                <LoanAcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanAcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <LoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <LoanRejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanRejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanRepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanRepossess(decoded));
            }
            if let Ok(decoded) =
                <LoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanSetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanSetPendingLender(decoded));
            }
            if let Ok(decoded) =
                <LoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::LoanSkim(decoded));
            }
            if let Ok(decoded) =
                <PoolDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::PoolDelegate(decoded));
            }
            if let Ok(decoded) =
                <SetPoolDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::SetPoolDelegate(decoded));
            }
            if let Ok(decoded) =
                <TryLoanAcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanAcceptLender(decoded));
            }
            if let Ok(decoded) =
                <TryLoanAcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanAcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <TryLoanClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <TryLoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <TryLoanRepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanRepossess(decoded));
            }
            if let Ok(decoded) =
                <TryLoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanSetPendingLender(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderMockCalls::TryLoanSkim(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LenderMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LenderMockCalls::Erc20Approve(element) => element.encode(),
                LenderMockCalls::Erc20Permit(element) => element.encode(),
                LenderMockCalls::Erc20Transfer(element) => element.encode(),
                LenderMockCalls::Erc20TransferFrom(element) => element.encode(),
                LenderMockCalls::LoanAcceptLender(element) => element.encode(),
                LenderMockCalls::LoanAcceptNewTerms(element) => element.encode(),
                LenderMockCalls::LoanClaimFunds(element) => element.encode(),
                LenderMockCalls::LoanCloseLoan(element) => element.encode(),
                LenderMockCalls::LoanFundLoan(element) => element.encode(),
                LenderMockCalls::LoanMakePayment(element) => element.encode(),
                LenderMockCalls::LoanPostCollateral(element) => element.encode(),
                LenderMockCalls::LoanRejectNewTerms(element) => element.encode(),
                LenderMockCalls::LoanRepossess(element) => element.encode(),
                LenderMockCalls::LoanReturnFunds(element) => element.encode(),
                LenderMockCalls::LoanSetPendingLender(element) => element.encode(),
                LenderMockCalls::LoanSkim(element) => element.encode(),
                LenderMockCalls::PoolDelegate(element) => element.encode(),
                LenderMockCalls::SetPoolDelegate(element) => element.encode(),
                LenderMockCalls::TryLoanAcceptLender(element) => element.encode(),
                LenderMockCalls::TryLoanAcceptNewTerms(element) => element.encode(),
                LenderMockCalls::TryLoanClaimFunds(element) => element.encode(),
                LenderMockCalls::TryLoanCloseLoan(element) => element.encode(),
                LenderMockCalls::TryLoanFundLoan(element) => element.encode(),
                LenderMockCalls::TryLoanMakePayment(element) => element.encode(),
                LenderMockCalls::TryLoanPostCollateral(element) => element.encode(),
                LenderMockCalls::TryLoanRepossess(element) => element.encode(),
                LenderMockCalls::TryLoanReturnFunds(element) => element.encode(),
                LenderMockCalls::TryLoanSetPendingLender(element) => element.encode(),
                LenderMockCalls::TryLoanSkim(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LenderMockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LenderMockCalls::Erc20Approve(element) => element.fmt(f),
                LenderMockCalls::Erc20Permit(element) => element.fmt(f),
                LenderMockCalls::Erc20Transfer(element) => element.fmt(f),
                LenderMockCalls::Erc20TransferFrom(element) => element.fmt(f),
                LenderMockCalls::LoanAcceptLender(element) => element.fmt(f),
                LenderMockCalls::LoanAcceptNewTerms(element) => element.fmt(f),
                LenderMockCalls::LoanClaimFunds(element) => element.fmt(f),
                LenderMockCalls::LoanCloseLoan(element) => element.fmt(f),
                LenderMockCalls::LoanFundLoan(element) => element.fmt(f),
                LenderMockCalls::LoanMakePayment(element) => element.fmt(f),
                LenderMockCalls::LoanPostCollateral(element) => element.fmt(f),
                LenderMockCalls::LoanRejectNewTerms(element) => element.fmt(f),
                LenderMockCalls::LoanRepossess(element) => element.fmt(f),
                LenderMockCalls::LoanReturnFunds(element) => element.fmt(f),
                LenderMockCalls::LoanSetPendingLender(element) => element.fmt(f),
                LenderMockCalls::LoanSkim(element) => element.fmt(f),
                LenderMockCalls::PoolDelegate(element) => element.fmt(f),
                LenderMockCalls::SetPoolDelegate(element) => element.fmt(f),
                LenderMockCalls::TryLoanAcceptLender(element) => element.fmt(f),
                LenderMockCalls::TryLoanAcceptNewTerms(element) => element.fmt(f),
                LenderMockCalls::TryLoanClaimFunds(element) => element.fmt(f),
                LenderMockCalls::TryLoanCloseLoan(element) => element.fmt(f),
                LenderMockCalls::TryLoanFundLoan(element) => element.fmt(f),
                LenderMockCalls::TryLoanMakePayment(element) => element.fmt(f),
                LenderMockCalls::TryLoanPostCollateral(element) => element.fmt(f),
                LenderMockCalls::TryLoanRepossess(element) => element.fmt(f),
                LenderMockCalls::TryLoanReturnFunds(element) => element.fmt(f),
                LenderMockCalls::TryLoanSetPendingLender(element) => element.fmt(f),
                LenderMockCalls::TryLoanSkim(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Erc20ApproveCall> for LenderMockCalls {
        fn from(var: Erc20ApproveCall) -> Self {
            LenderMockCalls::Erc20Approve(var)
        }
    }
    impl ::std::convert::From<Erc20PermitCall> for LenderMockCalls {
        fn from(var: Erc20PermitCall) -> Self {
            LenderMockCalls::Erc20Permit(var)
        }
    }
    impl ::std::convert::From<Erc20TransferCall> for LenderMockCalls {
        fn from(var: Erc20TransferCall) -> Self {
            LenderMockCalls::Erc20Transfer(var)
        }
    }
    impl ::std::convert::From<Erc20TransferFromCall> for LenderMockCalls {
        fn from(var: Erc20TransferFromCall) -> Self {
            LenderMockCalls::Erc20TransferFrom(var)
        }
    }
    impl ::std::convert::From<LoanAcceptLenderCall> for LenderMockCalls {
        fn from(var: LoanAcceptLenderCall) -> Self {
            LenderMockCalls::LoanAcceptLender(var)
        }
    }
    impl ::std::convert::From<LoanAcceptNewTermsCall> for LenderMockCalls {
        fn from(var: LoanAcceptNewTermsCall) -> Self {
            LenderMockCalls::LoanAcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanClaimFundsCall> for LenderMockCalls {
        fn from(var: LoanClaimFundsCall) -> Self {
            LenderMockCalls::LoanClaimFunds(var)
        }
    }
    impl ::std::convert::From<LoanCloseLoanCall> for LenderMockCalls {
        fn from(var: LoanCloseLoanCall) -> Self {
            LenderMockCalls::LoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<LoanFundLoanCall> for LenderMockCalls {
        fn from(var: LoanFundLoanCall) -> Self {
            LenderMockCalls::LoanFundLoan(var)
        }
    }
    impl ::std::convert::From<LoanMakePaymentCall> for LenderMockCalls {
        fn from(var: LoanMakePaymentCall) -> Self {
            LenderMockCalls::LoanMakePayment(var)
        }
    }
    impl ::std::convert::From<LoanPostCollateralCall> for LenderMockCalls {
        fn from(var: LoanPostCollateralCall) -> Self {
            LenderMockCalls::LoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<LoanRejectNewTermsCall> for LenderMockCalls {
        fn from(var: LoanRejectNewTermsCall) -> Self {
            LenderMockCalls::LoanRejectNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanRepossessCall> for LenderMockCalls {
        fn from(var: LoanRepossessCall) -> Self {
            LenderMockCalls::LoanRepossess(var)
        }
    }
    impl ::std::convert::From<LoanReturnFundsCall> for LenderMockCalls {
        fn from(var: LoanReturnFundsCall) -> Self {
            LenderMockCalls::LoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<LoanSetPendingLenderCall> for LenderMockCalls {
        fn from(var: LoanSetPendingLenderCall) -> Self {
            LenderMockCalls::LoanSetPendingLender(var)
        }
    }
    impl ::std::convert::From<LoanSkimCall> for LenderMockCalls {
        fn from(var: LoanSkimCall) -> Self {
            LenderMockCalls::LoanSkim(var)
        }
    }
    impl ::std::convert::From<PoolDelegateCall> for LenderMockCalls {
        fn from(var: PoolDelegateCall) -> Self {
            LenderMockCalls::PoolDelegate(var)
        }
    }
    impl ::std::convert::From<SetPoolDelegateCall> for LenderMockCalls {
        fn from(var: SetPoolDelegateCall) -> Self {
            LenderMockCalls::SetPoolDelegate(var)
        }
    }
    impl ::std::convert::From<TryLoanAcceptLenderCall> for LenderMockCalls {
        fn from(var: TryLoanAcceptLenderCall) -> Self {
            LenderMockCalls::TryLoanAcceptLender(var)
        }
    }
    impl ::std::convert::From<TryLoanAcceptNewTermsCall> for LenderMockCalls {
        fn from(var: TryLoanAcceptNewTermsCall) -> Self {
            LenderMockCalls::TryLoanAcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<TryLoanClaimFundsCall> for LenderMockCalls {
        fn from(var: TryLoanClaimFundsCall) -> Self {
            LenderMockCalls::TryLoanClaimFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanCloseLoanCall> for LenderMockCalls {
        fn from(var: TryLoanCloseLoanCall) -> Self {
            LenderMockCalls::TryLoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanFundLoanCall> for LenderMockCalls {
        fn from(var: TryLoanFundLoanCall) -> Self {
            LenderMockCalls::TryLoanFundLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanMakePaymentCall> for LenderMockCalls {
        fn from(var: TryLoanMakePaymentCall) -> Self {
            LenderMockCalls::TryLoanMakePayment(var)
        }
    }
    impl ::std::convert::From<TryLoanPostCollateralCall> for LenderMockCalls {
        fn from(var: TryLoanPostCollateralCall) -> Self {
            LenderMockCalls::TryLoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<TryLoanRepossessCall> for LenderMockCalls {
        fn from(var: TryLoanRepossessCall) -> Self {
            LenderMockCalls::TryLoanRepossess(var)
        }
    }
    impl ::std::convert::From<TryLoanReturnFundsCall> for LenderMockCalls {
        fn from(var: TryLoanReturnFundsCall) -> Self {
            LenderMockCalls::TryLoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanSetPendingLenderCall> for LenderMockCalls {
        fn from(var: TryLoanSetPendingLenderCall) -> Self {
            LenderMockCalls::TryLoanSetPendingLender(var)
        }
    }
    impl ::std::convert::From<TryLoanSkimCall> for LenderMockCalls {
        fn from(var: TryLoanSkimCall) -> Self {
            LenderMockCalls::TryLoanSkim(var)
        }
    }
}
