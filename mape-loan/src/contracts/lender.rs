pub use lender_mod::*;
#[allow(clippy::too_many_arguments)]
mod lender_mod {
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
    #[doc = "Lender was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LENDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_acceptLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_acceptNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_claimFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_repossess\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralRepossessed_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundsRepossessed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_setPendingLender\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_acceptLender\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_acceptNewTerms\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_claimFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_closeLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_fundLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_makePayment\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_postCollateral\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_repossess\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_returnFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_setPendingLender\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_skim\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LENDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611455806100206000396000f3fe608060405234801561001057600080fd5b50600436106101a95760003560e01c80637327de97116100f9578063afb7a0f611610097578063c2b63f4f11610071578063c2b63f4f146103bd578063d75edca4146103d0578063ec16a8d8146103e3578063fc179d67146103f657600080fd5b8063afb7a0f614610384578063b6a0f33114610397578063c1a3c9d7146103aa57600080fd5b80638b196cd5116100d35780638b196cd51461033857806398e5c2341461034b578063a18814da1461035e578063ac8a05011461037157600080fd5b80637327de97146102ff5780637c2e27d4146103125780637fe255811461032557600080fd5b80633d73ea8f1161016657806351cf23b11161014057806351cf23b1146102b3578063574c7844146102c6578063687468d1146102d95780636c202fcb146102ec57600080fd5b80633d73ea8f14610265578063429bf6c8146102785780635100d1211461028b57600080fd5b8063035e30b3146101ae57806305eee8e7146101d65780631515a116146101eb5780631649e2ef146101fe578063188b80b8146102315780632893a76e14610244575b600080fd5b6101c16101bc3660046110aa565b610409565b60405190151581526020015b60405180910390f35b6101e96101e4366004611027565b6104c2565b005b6101e96101f9366004610f4b565b610554565b61021161020c3660046111cc565b6105aa565b6040805194855260208501939093529183015260608201526080016101cd565b6101e961023f366004611155565b610640565b6102576102523660046111cc565b6106ae565b6040519081526020016101cd565b6102576102733660046111cc565b610733565b6102116102863660046111cc565b610763565b61029e610299366004610f66565b610797565b604080519283526020830191909152016101cd565b6101e96102c13660046110aa565b610822565b6101e96102d43660046110e6565b6108aa565b6101c16102e73660046111cc565b610915565b6101c16102fa366004611155565b6109c6565b6101c161030d366004610f99565b610a81565b6101c16103203660046111cc565b610ab4565b6101c16103333660046110aa565b610adb565b6102576103463660046110aa565b610b69565b6101c1610359366004610f66565b610bf0565b6101c161036c3660046111f6565b610c1b565b6101e961037f366004610f66565b610c4d565b610257610392366004610f99565b610cac565b6101c16103a53660046111cc565b610ce6565b6101c16103b8366004610f66565b610d0d565b6101e96103cb3660046111f6565b610d38565b6101c16103de3660046111cc565b610d9f565b6101c16103f1366004610f4b565b610dc6565b6101c1610404366004610fdc565b610e53565b6040516001600160a01b038381166024830152604482018390526000919085169063e920b1e160e01b906064015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516104759190611378565b6000604051808303816000865af19150503d80600081146104b2576040519150601f19603f3d011682016040523d82523d6000602084013e6104b7565b606091505b509095945050505050565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b15801561053257600080fd5b505af1158015610546573d6000803e3d6000fd5b505050505050505050505050565b806001600160a01b0316630fe3d9b76040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561058f57600080fd5b505af11580156105a3573d6000803e3d6000fd5b5050505050565b600080600080856001600160a01b031663d05951a0866040518263ffffffff1660e01b81526004016105de91815260200190565b608060405180830381600087803b1580156105f857600080fd5b505af115801561060c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106309190611288565b9299919850965090945092505050565b6040516328565a7760e11b81526001600160a01b038716906350acb4ee9061067490889088908890889088906004016113e5565b600060405180830381600087803b15801561068e57600080fd5b505af11580156106a2573d6000803e3d6000fd5b50505050505050505050565b6040516322baaeeb60e11b8152600481018290526000906001600160a01b038416906345755dd6906024015b602060405180830381600087803b1580156106f457600080fd5b505af1158015610708573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061072c919061124b565b9392505050565b6040516350f2012f60e01b8152600481018290526000906001600160a01b038416906350f2012f906024016106da565b600080600080856001600160a01b0316635114cb52866040518263ffffffff1660e01b81526004016105de91815260200190565b6040516347350e9f60e01b81526001600160a01b03828116600483015260009182918516906347350e9f906024016040805180830381600087803b1580156107de57600080fd5b505af11580156107f2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108169190611264565b915091505b9250929050565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b15801561086c57600080fd5b505af1158015610880573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108a49190611229565b50505050565b604051632b2d48ad60e21b81526001600160a01b0386169063acb522b4906108dc9087908790879087906004016113b3565b600060405180830381600087803b1580156108f657600080fd5b505af115801561090a573d6000803e3d6000fd5b505050505050505050565b6000826001600160a01b03166350f2012f60e01b8360405160240161093c91815260200190565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161097a9190611378565b6000604051808303816000865af19150503d80600081146109b7576040519150601f19603f3d011682016040523d82523d6000602084013e6109bc565b606091505b5090949350505050565b6000866001600160a01b03166350acb4ee60e01b87878787876040516024016109f39594939291906113e5565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610a319190611378565b6000604051808303816000865af19150503d8060008114610a6e576040519150601f19603f3d011682016040523d82523d6000602084013e610a73565b606091505b509098975050505050505050565b6040516001600160a01b03838116602483015282811660448301526000919085169063712b772f60e01b90606401610437565b6000826001600160a01b03166345755dd660e01b8360405160240161093c91815260200190565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b158015610b2957600080fd5b505af1158015610b3d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b619190611229565b949350505050565b60405163e920b1e160e01b81526001600160a01b038381166004830152602482018390526000919085169063e920b1e1906044015b602060405180830381600087803b158015610bb857600080fd5b505af1158015610bcc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b61919061124b565b6040516001600160a01b038281166024830152600091908416906347350e9f60e01b9060440161093c565b604051602481018390526001600160a01b0382811660448301526000919085169063390d685560e01b90606401610437565b60405163267f4ac360e01b81526001600160a01b03828116600483015283169063267f4ac390602401600060405180830381600087803b158015610c9057600080fd5b505af1158015610ca4573d6000803e3d6000fd5b505050505050565b60405163712b772f60e01b81526001600160a01b03838116600483015282811660248301526000919085169063712b772f90604401610b9e565b6000826001600160a01b031663d05951a060e01b8360405160240161093c91815260200190565b6040516001600160a01b0382811660248301526000919084169063267f4ac360e01b9060440161093c565b60405163390d685560e01b8152600481018390526001600160a01b03828116602483015284169063390d685590604401600060405180830381600087803b158015610d8257600080fd5b505af1158015610d96573d6000803e3d6000fd5b50505050505050565b6000826001600160a01b0316635114cb5260e01b8360405160240161093c91815260200190565b60408051600481526024810182526020810180516001600160e01b0316630fe3d9b760e01b17905290516000916001600160a01b03841691610e089190611378565b6000604051808303816000865af19150503d8060008114610e45576040519150601f19603f3d011682016040523d82523d6000602084013e610e4a565b606091505b50909392505050565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b158015610ea957600080fd5b505af1158015610ebd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ee19190611229565b95945050505050565b80356001600160a01b0381168114610f0157600080fd5b919050565b60008083601f840112610f1857600080fd5b50813567ffffffffffffffff811115610f3057600080fd5b6020830191508360208260051b850101111561081b57600080fd5b600060208284031215610f5d57600080fd5b61072c82610eea565b60008060408385031215610f7957600080fd5b610f8283610eea565b9150610f9060208401610eea565b90509250929050565b600080600060608486031215610fae57600080fd5b610fb784610eea565b9250610fc560208501610eea565b9150610fd360408501610eea565b90509250925092565b60008060008060808587031215610ff257600080fd5b610ffb85610eea565b935061100960208601610eea565b925061101760408601610eea565b9396929550929360600135925050565b600080600080600080600080610100898b03121561104457600080fd5b61104d89610eea565b975061105b60208a01610eea565b965061106960408a01610eea565b9550606089013594506080890135935060a089013560ff8116811461108d57600080fd5b979a969950949793969295929450505060c08201359160e0013590565b6000806000606084860312156110bf57600080fd5b6110c884610eea565b92506110d660208501610eea565b9150604084013590509250925092565b6000806000806000608086880312156110fe57600080fd5b61110786610eea565b945061111560208701610eea565b935060408601359250606086013567ffffffffffffffff81111561113857600080fd5b61114488828901610f06565b969995985093965092949392505050565b60008060008060008060a0878903121561116e57600080fd5b61117787610eea565b955061118560208801610eea565b945060408701359350606087013567ffffffffffffffff8111156111a857600080fd5b6111b489828a01610f06565b979a9699509497949695608090950135949350505050565b600080604083850312156111df57600080fd5b6111e883610eea565b946020939093013593505050565b60008060006060848603121561120b57600080fd5b61121484610eea565b925060208401359150610fd360408501610eea565b60006020828403121561123b57600080fd5b8151801515811461072c57600080fd5b60006020828403121561125d57600080fd5b5051919050565b6000806040838503121561127757600080fd5b505080516020909101519092909150565b6000806000806080858703121561129e57600080fd5b505082516020840151604085015160609095015191969095509092509050565b81835260006020808501808196508560051b810191508460005b878110156113425782840389528135601e198836030181126112f957600080fd5b8701803567ffffffffffffffff81111561131257600080fd5b80360389131561132157600080fd5b61132e868289850161134f565b9a87019a95505050908401906001016112d8565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b81811015611399576020818601810151858301520161137f565b818111156113a8576000828501525b509190910192915050565b60018060a01b03851681528360208201526060604082015260006113db6060830184866112be565b9695505050505050565b60018060a01b038616815284602082015260806040820152600061140d6080830185876112be565b9050826060830152969550505050505056fea2646970667358221220fa6d5b1c4a2936abbd12db04d7894e362c7388bde4221f17d53c2702d16a0b4e64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Lender<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Lender<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Lender<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Lender))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Lender<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LENDER_ABI.clone(), client).into()
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
                LENDER_ABI.clone(),
                LENDER_BYTECODE.clone().into(),
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Lender<M> {
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
    pub enum LenderCalls {
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
    impl ethers::core::abi::AbiDecode for LenderCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Erc20ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::Erc20Approve(decoded));
            }
            if let Ok(decoded) =
                <Erc20PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::Erc20Permit(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::Erc20Transfer(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::Erc20TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <LoanAcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanAcceptLender(decoded));
            }
            if let Ok(decoded) =
                <LoanAcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanAcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <LoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <LoanRejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanRejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanRepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanRepossess(decoded));
            }
            if let Ok(decoded) =
                <LoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanSetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanSetPendingLender(decoded));
            }
            if let Ok(decoded) =
                <LoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::LoanSkim(decoded));
            }
            if let Ok(decoded) =
                <TryLoanAcceptLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanAcceptLender(decoded));
            }
            if let Ok(decoded) =
                <TryLoanAcceptNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanAcceptNewTerms(decoded));
            }
            if let Ok(decoded) =
                <TryLoanClaimFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanClaimFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <TryLoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <TryLoanRepossessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanRepossess(decoded));
            }
            if let Ok(decoded) =
                <TryLoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSetPendingLenderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanSetPendingLender(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LenderCalls::TryLoanSkim(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LenderCalls::Erc20Approve(element) => element.encode(),
                LenderCalls::Erc20Permit(element) => element.encode(),
                LenderCalls::Erc20Transfer(element) => element.encode(),
                LenderCalls::Erc20TransferFrom(element) => element.encode(),
                LenderCalls::LoanAcceptLender(element) => element.encode(),
                LenderCalls::LoanAcceptNewTerms(element) => element.encode(),
                LenderCalls::LoanClaimFunds(element) => element.encode(),
                LenderCalls::LoanCloseLoan(element) => element.encode(),
                LenderCalls::LoanFundLoan(element) => element.encode(),
                LenderCalls::LoanMakePayment(element) => element.encode(),
                LenderCalls::LoanPostCollateral(element) => element.encode(),
                LenderCalls::LoanRejectNewTerms(element) => element.encode(),
                LenderCalls::LoanRepossess(element) => element.encode(),
                LenderCalls::LoanReturnFunds(element) => element.encode(),
                LenderCalls::LoanSetPendingLender(element) => element.encode(),
                LenderCalls::LoanSkim(element) => element.encode(),
                LenderCalls::TryLoanAcceptLender(element) => element.encode(),
                LenderCalls::TryLoanAcceptNewTerms(element) => element.encode(),
                LenderCalls::TryLoanClaimFunds(element) => element.encode(),
                LenderCalls::TryLoanCloseLoan(element) => element.encode(),
                LenderCalls::TryLoanFundLoan(element) => element.encode(),
                LenderCalls::TryLoanMakePayment(element) => element.encode(),
                LenderCalls::TryLoanPostCollateral(element) => element.encode(),
                LenderCalls::TryLoanRepossess(element) => element.encode(),
                LenderCalls::TryLoanReturnFunds(element) => element.encode(),
                LenderCalls::TryLoanSetPendingLender(element) => element.encode(),
                LenderCalls::TryLoanSkim(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LenderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LenderCalls::Erc20Approve(element) => element.fmt(f),
                LenderCalls::Erc20Permit(element) => element.fmt(f),
                LenderCalls::Erc20Transfer(element) => element.fmt(f),
                LenderCalls::Erc20TransferFrom(element) => element.fmt(f),
                LenderCalls::LoanAcceptLender(element) => element.fmt(f),
                LenderCalls::LoanAcceptNewTerms(element) => element.fmt(f),
                LenderCalls::LoanClaimFunds(element) => element.fmt(f),
                LenderCalls::LoanCloseLoan(element) => element.fmt(f),
                LenderCalls::LoanFundLoan(element) => element.fmt(f),
                LenderCalls::LoanMakePayment(element) => element.fmt(f),
                LenderCalls::LoanPostCollateral(element) => element.fmt(f),
                LenderCalls::LoanRejectNewTerms(element) => element.fmt(f),
                LenderCalls::LoanRepossess(element) => element.fmt(f),
                LenderCalls::LoanReturnFunds(element) => element.fmt(f),
                LenderCalls::LoanSetPendingLender(element) => element.fmt(f),
                LenderCalls::LoanSkim(element) => element.fmt(f),
                LenderCalls::TryLoanAcceptLender(element) => element.fmt(f),
                LenderCalls::TryLoanAcceptNewTerms(element) => element.fmt(f),
                LenderCalls::TryLoanClaimFunds(element) => element.fmt(f),
                LenderCalls::TryLoanCloseLoan(element) => element.fmt(f),
                LenderCalls::TryLoanFundLoan(element) => element.fmt(f),
                LenderCalls::TryLoanMakePayment(element) => element.fmt(f),
                LenderCalls::TryLoanPostCollateral(element) => element.fmt(f),
                LenderCalls::TryLoanRepossess(element) => element.fmt(f),
                LenderCalls::TryLoanReturnFunds(element) => element.fmt(f),
                LenderCalls::TryLoanSetPendingLender(element) => element.fmt(f),
                LenderCalls::TryLoanSkim(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Erc20ApproveCall> for LenderCalls {
        fn from(var: Erc20ApproveCall) -> Self {
            LenderCalls::Erc20Approve(var)
        }
    }
    impl ::std::convert::From<Erc20PermitCall> for LenderCalls {
        fn from(var: Erc20PermitCall) -> Self {
            LenderCalls::Erc20Permit(var)
        }
    }
    impl ::std::convert::From<Erc20TransferCall> for LenderCalls {
        fn from(var: Erc20TransferCall) -> Self {
            LenderCalls::Erc20Transfer(var)
        }
    }
    impl ::std::convert::From<Erc20TransferFromCall> for LenderCalls {
        fn from(var: Erc20TransferFromCall) -> Self {
            LenderCalls::Erc20TransferFrom(var)
        }
    }
    impl ::std::convert::From<LoanAcceptLenderCall> for LenderCalls {
        fn from(var: LoanAcceptLenderCall) -> Self {
            LenderCalls::LoanAcceptLender(var)
        }
    }
    impl ::std::convert::From<LoanAcceptNewTermsCall> for LenderCalls {
        fn from(var: LoanAcceptNewTermsCall) -> Self {
            LenderCalls::LoanAcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanClaimFundsCall> for LenderCalls {
        fn from(var: LoanClaimFundsCall) -> Self {
            LenderCalls::LoanClaimFunds(var)
        }
    }
    impl ::std::convert::From<LoanCloseLoanCall> for LenderCalls {
        fn from(var: LoanCloseLoanCall) -> Self {
            LenderCalls::LoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<LoanFundLoanCall> for LenderCalls {
        fn from(var: LoanFundLoanCall) -> Self {
            LenderCalls::LoanFundLoan(var)
        }
    }
    impl ::std::convert::From<LoanMakePaymentCall> for LenderCalls {
        fn from(var: LoanMakePaymentCall) -> Self {
            LenderCalls::LoanMakePayment(var)
        }
    }
    impl ::std::convert::From<LoanPostCollateralCall> for LenderCalls {
        fn from(var: LoanPostCollateralCall) -> Self {
            LenderCalls::LoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<LoanRejectNewTermsCall> for LenderCalls {
        fn from(var: LoanRejectNewTermsCall) -> Self {
            LenderCalls::LoanRejectNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanRepossessCall> for LenderCalls {
        fn from(var: LoanRepossessCall) -> Self {
            LenderCalls::LoanRepossess(var)
        }
    }
    impl ::std::convert::From<LoanReturnFundsCall> for LenderCalls {
        fn from(var: LoanReturnFundsCall) -> Self {
            LenderCalls::LoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<LoanSetPendingLenderCall> for LenderCalls {
        fn from(var: LoanSetPendingLenderCall) -> Self {
            LenderCalls::LoanSetPendingLender(var)
        }
    }
    impl ::std::convert::From<LoanSkimCall> for LenderCalls {
        fn from(var: LoanSkimCall) -> Self {
            LenderCalls::LoanSkim(var)
        }
    }
    impl ::std::convert::From<TryLoanAcceptLenderCall> for LenderCalls {
        fn from(var: TryLoanAcceptLenderCall) -> Self {
            LenderCalls::TryLoanAcceptLender(var)
        }
    }
    impl ::std::convert::From<TryLoanAcceptNewTermsCall> for LenderCalls {
        fn from(var: TryLoanAcceptNewTermsCall) -> Self {
            LenderCalls::TryLoanAcceptNewTerms(var)
        }
    }
    impl ::std::convert::From<TryLoanClaimFundsCall> for LenderCalls {
        fn from(var: TryLoanClaimFundsCall) -> Self {
            LenderCalls::TryLoanClaimFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanCloseLoanCall> for LenderCalls {
        fn from(var: TryLoanCloseLoanCall) -> Self {
            LenderCalls::TryLoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanFundLoanCall> for LenderCalls {
        fn from(var: TryLoanFundLoanCall) -> Self {
            LenderCalls::TryLoanFundLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanMakePaymentCall> for LenderCalls {
        fn from(var: TryLoanMakePaymentCall) -> Self {
            LenderCalls::TryLoanMakePayment(var)
        }
    }
    impl ::std::convert::From<TryLoanPostCollateralCall> for LenderCalls {
        fn from(var: TryLoanPostCollateralCall) -> Self {
            LenderCalls::TryLoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<TryLoanRepossessCall> for LenderCalls {
        fn from(var: TryLoanRepossessCall) -> Self {
            LenderCalls::TryLoanRepossess(var)
        }
    }
    impl ::std::convert::From<TryLoanReturnFundsCall> for LenderCalls {
        fn from(var: TryLoanReturnFundsCall) -> Self {
            LenderCalls::TryLoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanSetPendingLenderCall> for LenderCalls {
        fn from(var: TryLoanSetPendingLenderCall) -> Self {
            LenderCalls::TryLoanSetPendingLender(var)
        }
    }
    impl ::std::convert::From<TryLoanSkimCall> for LenderCalls {
        fn from(var: TryLoanSkimCall) -> Self {
            LenderCalls::TryLoanSkim(var)
        }
    }
}
