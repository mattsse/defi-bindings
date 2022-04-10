pub use borrower_mod::*;
#[allow(clippy::too_many_arguments)]
mod borrower_mod {
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
    #[doc = "Borrower was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BORROWER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"erc20_transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_acceptBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_closeLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_drawdownFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_fundLoan\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsLent_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_makePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"principal_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interest_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delegateFee_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"treasuryFee_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_postCollateral\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralPosted_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_proposeNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_rejectNewTerms\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_removeCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_returnFunds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_setPendingBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_skim\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fundsReturned_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"loan_upgrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxied_upgrade\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mapleProxyFactory_createInstance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_acceptBorrower\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_closeLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_drawdownFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"lender_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_fundLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_makePayment\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_postCollateral\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refinancer_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calls_\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_proposeNewTerms\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_removeCollateral\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_returnFunds\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_setPendingBorrower\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"destination_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_skim\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"loan_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_loan_upgrade\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxied_upgrade\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"factory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"try_mapleProxyFactory_createInstance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"ok_\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BORROWER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506116bd806100206000396000f3fe608060405234801561001057600080fd5b50600436106101fb5760003560e01c80637327de971161011a578063b6a0f331116100ad578063d75edca41161007c578063d75edca41461045e578063e5743b4f1461031b578063eb54553414610471578063f29fa84114610484578063fc179d671461049757600080fd5b8063b6a0f33114610412578063bb6830a614610425578063c6340ccf14610438578063ca6380691461044b57600080fd5b806399c96d4c116100e957806399c96d4c146103d9578063a67beab3146103ec578063afb7a0f6146103ff578063b501482f146103ec57600080fd5b80637327de971461038d5780637c2e27d4146103a05780637fe25581146103b35780638b196cd5146103c657600080fd5b8063429bf6c811610192578063574c784411610161578063574c78441461034157806366c1434514610354578063687468d1146103675780636d03fbc31461037a57600080fd5b8063429bf6c8146102dd57806348b4e6bc146102f05780634ad11ba71461031b57806351cf23b11461032e57600080fd5b80632893a76e116101ce5780632893a76e14610283578063329ff38f146102a457806332e7772e146102b75780633d73ea8f146102ca57600080fd5b8063020578a214610200578063035e30b31461022857806305eee8e71461023b5780631649e2ef14610250575b600080fd5b61021361020e366004611414565b6104aa565b60405190151581526020015b60405180910390f35b6102136102363660046112a9565b610563565b61024e610249366004611220565b610595565b005b61026361025e3660046113e8565b610627565b60408051948552602085019390935291830152606082015260800161021f565b6102966102913660046113e8565b6106bd565b60405190815260200161021f565b6102136102b2366004611414565b610742565b61024e6102c5366004611414565b610774565b6102966102d83660046113e8565b6107db565b6102636102eb3660046113e8565b61080b565b6103036102fe36600461138c565b61083f565b6040516001600160a01b03909116815260200161021f565b61021361032936600461144b565b6108cd565b61024e61033c3660046112a9565b610982565b61024e61034f3660046112ea565b610a0a565b61021361036236600461114b565b610a75565b6102136103753660046113e8565b610b26565b61024e6103883660046112ea565b610b4d565b61021361039b366004611184565b610b7f565b6102136103ae3660046113e8565b610bb2565b6102136103c13660046112a9565b610bd9565b6102966103d43660046112a9565b610c67565b6102136103e736600461138c565b610cee565b61024e6103fa36600461144b565b610d17565b61029661040d366004611184565b610d7f565b6102136104203660046113e8565b610db9565b610213610433366004611111565b610de0565b610296610446366004611414565b610e6d565b6102136104593660046112ea565b610ea6565b61021361046c3660046113e8565b610f5e565b61024e61047f366004611111565b610f85565b61024e61049236600461114b565b610fdb565b6102136104a53660046111cf565b61103a565b604051602481018390526001600160a01b03828116604483015260009190851690636a0eee4b60e11b906064015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516105169190611541565b6000604051808303816000865af19150503d8060008114610553576040519150601f19603f3d011682016040523d82523d6000602084013e610558565b606091505b509095945050505050565b6040516001600160a01b038381166024830152604482018390526000919085169063e920b1e160e01b906064016104d8565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b15801561060557600080fd5b505af1158015610619573d6000803e3d6000fd5b505050505050505050505050565b600080600080856001600160a01b031663d05951a0866040518263ffffffff1660e01b815260040161065b91815260200190565b608060405180830381600087803b15801561067557600080fd5b505af1158015610689573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ad91906114e2565b9299919850965090945092505050565b6040516322baaeeb60e11b8152600481018290526000906001600160a01b038416906345755dd6906024015b602060405180830381600087803b15801561070357600080fd5b505af1158015610717573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061073b91906114c9565b9392505050565b604051602481018390526001600160a01b03828116604483015260009190851690633330112160e21b906064016104d8565b604051636a0eee4b60e11b8152600481018390526001600160a01b03828116602483015284169063d41ddc9690604401600060405180830381600087803b1580156107be57600080fd5b505af11580156107d2573d6000803e3d6000fd5b50505050505050565b6040516350f2012f60e01b8152600481018290526000906001600160a01b038416906350f2012f906024016106e9565b600080600080856001600160a01b0316635114cb52866040518263ffffffff1660e01b815260040161065b91815260200190565b60405163517b657f60e01b81526000906001600160a01b0386169063517b657f9061087290879087908790600401611631565b602060405180830381600087803b15801561088c57600080fd5b505af11580156108a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108c4919061112e565b95945050505050565b6000846001600160a01b0316633b99bcee60e01b8585856040516024016108f693929190611655565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516109349190611541565b6000604051808303816000865af19150503d8060008114610971576040519150601f19603f3d011682016040523d82523d6000602084013e610976565b606091505b50909695505050505050565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b1580156109cc57600080fd5b505af11580156109e0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a0491906114a7565b50505050565b604051632b2d48ad60e21b81526001600160a01b0386169063acb522b490610a3c90879087908790879060040161157c565b600060405180830381600087803b158015610a5657600080fd5b505af1158015610a6a573d6000803e3d6000fd5b505050505050505050565b6040516001600160a01b03828116602483015260009190841690631f3f19ab60e01b906044015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610ada9190611541565b6000604051808303816000865af19150503d8060008114610b17576040519150601f19603f3d011682016040523d82523d6000602084013e610b1c565b606091505b5090949350505050565b6000826001600160a01b03166350f2012f60e01b83604051602401610a9c91815260200190565b6040516387accaf160e01b81526001600160a01b038616906387accaf190610a3c90879087908790879060040161157c565b6040516001600160a01b03838116602483015282811660448301526000919085169063712b772f60e01b906064016104d8565b6000826001600160a01b03166345755dd660e01b83604051602401610a9c91815260200190565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b158015610c2757600080fd5b505af1158015610c3b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c5f91906114a7565b949350505050565b60405163e920b1e160e01b81526001600160a01b038381166004830152602482018390526000919085169063e920b1e1906044015b602060405180830381600087803b158015610cb657600080fd5b505af1158015610cca573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c5f91906114c9565b6000846001600160a01b031663517b657f60e01b8585856040516024016108f693929190611631565b604051631dccde7760e11b81526001600160a01b03851690633b99bcee90610d4790869086908690600401611655565b600060405180830381600087803b158015610d6157600080fd5b505af1158015610d75573d6000803e3d6000fd5b5050505050505050565b60405163712b772f60e01b81526001600160a01b03838116600483015282811660248301526000919085169063712b772f90604401610c9c565b6000826001600160a01b031663d05951a060e01b83604051602401610a9c91815260200190565b60408051600481526024810182526020810180516001600160e01b03166301daa38f60e01b17905290516000916001600160a01b03841691610e229190611541565b6000604051808303816000865af19150503d8060008114610e5f576040519150601f19603f3d011682016040523d82523d6000602084013e610e64565b606091505b50909392505050565b604051633330112160e21b8152600481018390526001600160a01b0382811660248301526000919085169063ccc0448490604401610c9c565b6000856001600160a01b03166387accaf160e01b86868686604051602401610ed1949392919061157c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610f0f9190611541565b6000604051808303816000865af19150503d8060008114610f4c576040519150601f19603f3d011682016040523d82523d6000602084013e610f51565b606091505b5090979650505050505050565b6000826001600160a01b0316635114cb5260e01b83604051602401610a9c91815260200190565b806001600160a01b03166301daa38f6040518163ffffffff1660e01b8152600401600060405180830381600087803b158015610fc057600080fd5b505af1158015610fd4573d6000803e3d6000fd5b5050505050565b604051631f3f19ab60e01b81526001600160a01b038281166004830152831690631f3f19ab90602401600060405180830381600087803b15801561101e57600080fd5b505af1158015611032573d6000803e3d6000fd5b505050505050565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b15801561109057600080fd5b505af11580156110a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108c491906114a7565b60008083601f8401126110da57600080fd5b50813567ffffffffffffffff8111156110f257600080fd5b60208301915083602082850101111561110a57600080fd5b9250929050565b60006020828403121561112357600080fd5b813561073b8161166f565b60006020828403121561114057600080fd5b815161073b8161166f565b6000806040838503121561115e57600080fd5b82356111698161166f565b915060208301356111798161166f565b809150509250929050565b60008060006060848603121561119957600080fd5b83356111a48161166f565b925060208401356111b48161166f565b915060408401356111c48161166f565b809150509250925092565b600080600080608085870312156111e557600080fd5b84356111f08161166f565b935060208501356112008161166f565b925060408501356112108161166f565b9396929550929360600135925050565b600080600080600080600080610100898b03121561123d57600080fd5b88356112488161166f565b975060208901356112588161166f565b965060408901356112688161166f565b9550606089013594506080890135935060a089013560ff8116811461128c57600080fd5b979a969950949793969295929450505060c08201359160e0013590565b6000806000606084860312156112be57600080fd5b83356112c98161166f565b925060208401356112d98161166f565b929592945050506040919091013590565b60008060008060006080868803121561130257600080fd5b853561130d8161166f565b9450602086013561131d8161166f565b935060408601359250606086013567ffffffffffffffff8082111561134157600080fd5b818801915088601f83011261135557600080fd5b81358181111561136457600080fd5b8960208260051b850101111561137957600080fd5b9699959850939650602001949392505050565b600080600080606085870312156113a257600080fd5b84356113ad8161166f565b9350602085013567ffffffffffffffff8111156113c957600080fd5b6113d5878288016110c8565b9598909750949560400135949350505050565b600080604083850312156113fb57600080fd5b82356114068161166f565b946020939093013593505050565b60008060006060848603121561142957600080fd5b83356114348161166f565b92506020840135915060408401356111c48161166f565b6000806000806060858703121561146157600080fd5b843561146c8161166f565b935060208501359250604085013567ffffffffffffffff81111561148f57600080fd5b61149b878288016110c8565b95989497509550505050565b6000602082840312156114b957600080fd5b8151801515811461073b57600080fd5b6000602082840312156114db57600080fd5b5051919050565b600080600080608085870312156114f857600080fd5b505082516020840151604085015160609095015191969095509092509050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b818110156115625760208186018101518583015201611548565b81811115611571576000828501525b509190910192915050565b6001600160a01b0385168152602080820185905260606040830181905282018390526000906080600585901b840181019190840186845b8781101561162257868503607f190183528135368a9003601e190181126115d957600080fd5b8901803567ffffffffffffffff8111156115f257600080fd5b8036038b131561160157600080fd5b61160e8782888501611518565b9650505091830191908301906001016115b3565b50929998505050505050505050565b604081526000611645604083018587611518565b9050826020830152949350505050565b8381526040602082015260006108c4604083018486611518565b6001600160a01b038116811461168457600080fd5b5056fea2646970667358221220843571d3c054f015a97b5a8e4d8dad67c3467fb0a495d42f17e785930c8c105f64736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Borrower<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Borrower<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Borrower<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Borrower))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Borrower<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BORROWER_ABI.clone(), client).into()
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
                BORROWER_ABI.clone(),
                BORROWER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `loan_acceptBorrower` (0xeb545534) function"]
        pub fn loan_accept_borrower(
            &self,
            loan: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 84, 85, 52], loan)
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
        #[doc = "Calls the contract's `loan_drawdownFunds` (0xc6340ccf) function"]
        pub fn loan_drawdown_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 52, 12, 207], (loan, amount, destination))
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
        #[doc = "Calls the contract's `loan_proposeNewTerms` (0x6d03fbc3) function"]
        pub fn loan_propose_new_terms(
            &self,
            loan: ethers::core::types::Address,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 3, 251, 195], (loan, refinancer, deadline, calls))
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
        #[doc = "Calls the contract's `loan_removeCollateral` (0x32e7772e) function"]
        pub fn loan_remove_collateral(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 231, 119, 46], (loan, amount, destination))
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
        #[doc = "Calls the contract's `loan_setPendingBorrower` (0xf29fa841) function"]
        pub fn loan_set_pending_borrower(
            &self,
            loan: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 159, 168, 65], (loan, borrower))
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
        #[doc = "Calls the contract's `loan_upgrade` (0xa67beab3) function"]
        pub fn loan_upgrade(
            &self,
            loan: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 123, 234, 179], (loan, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxied_upgrade` (0xb501482f) function"]
        pub fn maple_proxied_upgrade(
            &self,
            instance: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 1, 72, 47], (instance, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleProxyFactory_createInstance` (0x48b4e6bc) function"]
        pub fn maple_proxy_factory_create_instance(
            &self,
            factory: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([72, 180, 230, 188], (factory, arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_acceptBorrower` (0xbb6830a6) function"]
        pub fn try_loan_accept_borrower(
            &self,
            loan: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 104, 48, 166], loan)
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
        #[doc = "Calls the contract's `try_loan_drawdownFunds` (0x329ff38f) function"]
        pub fn try_loan_drawdown_funds(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([50, 159, 243, 143], (loan, amount, destination))
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
        #[doc = "Calls the contract's `try_loan_proposeNewTerms` (0xca638069) function"]
        pub fn try_loan_propose_new_terms(
            &self,
            loan: ethers::core::types::Address,
            refinancer: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            calls: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 99, 128, 105], (loan, refinancer, deadline, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_loan_removeCollateral` (0x020578a2) function"]
        pub fn try_loan_remove_collateral(
            &self,
            loan: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            destination: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([2, 5, 120, 162], (loan, amount, destination))
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
        #[doc = "Calls the contract's `try_loan_setPendingBorrower` (0x66c14345) function"]
        pub fn try_loan_set_pending_borrower(
            &self,
            loan: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 193, 67, 69], (loan, borrower))
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
        #[doc = "Calls the contract's `try_loan_upgrade` (0x4ad11ba7) function"]
        pub fn try_loan_upgrade(
            &self,
            loan: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([74, 209, 27, 167], (loan, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxied_upgrade` (0xe5743b4f) function"]
        pub fn try_maple_proxied_upgrade(
            &self,
            instance: ethers::core::types::Address,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 116, 59, 79], (instance, to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `try_mapleProxyFactory_createInstance` (0x99c96d4c) function"]
        pub fn try_maple_proxy_factory_create_instance(
            &self,
            factory: ethers::core::types::Address,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 201, 109, 76], (factory, arguments, salt))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Borrower<M> {
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
    #[doc = "Container type for all input parameters for the `loan_acceptBorrower`function with signature `loan_acceptBorrower(address)` and selector `[235, 84, 85, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_acceptBorrower", abi = "loan_acceptBorrower(address)")]
    pub struct LoanAcceptBorrowerCall {
        pub loan: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `loan_drawdownFunds`function with signature `loan_drawdownFunds(address,uint256,address)` and selector `[198, 52, 12, 207]`"]
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
        name = "loan_drawdownFunds",
        abi = "loan_drawdownFunds(address,uint256,address)"
    )]
    pub struct LoanDrawdownFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `loan_proposeNewTerms`function with signature `loan_proposeNewTerms(address,address,uint256,bytes[])` and selector `[109, 3, 251, 195]`"]
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
        name = "loan_proposeNewTerms",
        abi = "loan_proposeNewTerms(address,address,uint256,bytes[])"
    )]
    pub struct LoanProposeNewTermsCall {
        pub loan: ethers::core::types::Address,
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
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
    #[doc = "Container type for all input parameters for the `loan_removeCollateral`function with signature `loan_removeCollateral(address,uint256,address)` and selector `[50, 231, 119, 46]`"]
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
        name = "loan_removeCollateral",
        abi = "loan_removeCollateral(address,uint256,address)"
    )]
    pub struct LoanRemoveCollateralCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `loan_setPendingBorrower`function with signature `loan_setPendingBorrower(address,address)` and selector `[242, 159, 168, 65]`"]
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
        name = "loan_setPendingBorrower",
        abi = "loan_setPendingBorrower(address,address)"
    )]
    pub struct LoanSetPendingBorrowerCall {
        pub loan: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `loan_upgrade`function with signature `loan_upgrade(address,uint256,bytes)` and selector `[166, 123, 234, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loan_upgrade", abi = "loan_upgrade(address,uint256,bytes)")]
    pub struct LoanUpgradeCall {
        pub loan: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mapleProxied_upgrade`function with signature `mapleProxied_upgrade(address,uint256,bytes)` and selector `[181, 1, 72, 47]`"]
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
        name = "mapleProxied_upgrade",
        abi = "mapleProxied_upgrade(address,uint256,bytes)"
    )]
    pub struct MapleProxiedUpgradeCall {
        pub instance: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mapleProxyFactory_createInstance`function with signature `mapleProxyFactory_createInstance(address,bytes,bytes32)` and selector `[72, 180, 230, 188]`"]
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
        name = "mapleProxyFactory_createInstance",
        abi = "mapleProxyFactory_createInstance(address,bytes,bytes32)"
    )]
    pub struct MapleProxyFactoryCreateInstanceCall {
        pub factory: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `try_loan_acceptBorrower`function with signature `try_loan_acceptBorrower(address)` and selector `[187, 104, 48, 166]`"]
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
        name = "try_loan_acceptBorrower",
        abi = "try_loan_acceptBorrower(address)"
    )]
    pub struct TryLoanAcceptBorrowerCall {
        pub loan: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `try_loan_drawdownFunds`function with signature `try_loan_drawdownFunds(address,uint256,address)` and selector `[50, 159, 243, 143]`"]
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
        name = "try_loan_drawdownFunds",
        abi = "try_loan_drawdownFunds(address,uint256,address)"
    )]
    pub struct TryLoanDrawdownFundsCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub destination: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `try_loan_proposeNewTerms`function with signature `try_loan_proposeNewTerms(address,address,uint256,bytes[])` and selector `[202, 99, 128, 105]`"]
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
        name = "try_loan_proposeNewTerms",
        abi = "try_loan_proposeNewTerms(address,address,uint256,bytes[])"
    )]
    pub struct TryLoanProposeNewTermsCall {
        pub loan: ethers::core::types::Address,
        pub refinancer: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub calls: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `try_loan_removeCollateral`function with signature `try_loan_removeCollateral(address,uint256,address)` and selector `[2, 5, 120, 162]`"]
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
        name = "try_loan_removeCollateral",
        abi = "try_loan_removeCollateral(address,uint256,address)"
    )]
    pub struct TryLoanRemoveCollateralCall {
        pub loan: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `try_loan_setPendingBorrower`function with signature `try_loan_setPendingBorrower(address,address)` and selector `[102, 193, 67, 69]`"]
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
        name = "try_loan_setPendingBorrower",
        abi = "try_loan_setPendingBorrower(address,address)"
    )]
    pub struct TryLoanSetPendingBorrowerCall {
        pub loan: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `try_loan_upgrade`function with signature `try_loan_upgrade(address,uint256,bytes)` and selector `[74, 209, 27, 167]`"]
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
        name = "try_loan_upgrade",
        abi = "try_loan_upgrade(address,uint256,bytes)"
    )]
    pub struct TryLoanUpgradeCall {
        pub loan: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxied_upgrade`function with signature `try_mapleProxied_upgrade(address,uint256,bytes)` and selector `[229, 116, 59, 79]`"]
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
        name = "try_mapleProxied_upgrade",
        abi = "try_mapleProxied_upgrade(address,uint256,bytes)"
    )]
    pub struct TryMapleProxiedUpgradeCall {
        pub instance: ethers::core::types::Address,
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `try_mapleProxyFactory_createInstance`function with signature `try_mapleProxyFactory_createInstance(address,bytes,bytes32)` and selector `[153, 201, 109, 76]`"]
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
        name = "try_mapleProxyFactory_createInstance",
        abi = "try_mapleProxyFactory_createInstance(address,bytes,bytes32)"
    )]
    pub struct TryMapleProxyFactoryCreateInstanceCall {
        pub factory: ethers::core::types::Address,
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BorrowerCalls {
        Erc20Approve(Erc20ApproveCall),
        Erc20Permit(Erc20PermitCall),
        Erc20Transfer(Erc20TransferCall),
        Erc20TransferFrom(Erc20TransferFromCall),
        LoanAcceptBorrower(LoanAcceptBorrowerCall),
        LoanCloseLoan(LoanCloseLoanCall),
        LoanDrawdownFunds(LoanDrawdownFundsCall),
        LoanFundLoan(LoanFundLoanCall),
        LoanMakePayment(LoanMakePaymentCall),
        LoanPostCollateral(LoanPostCollateralCall),
        LoanProposeNewTerms(LoanProposeNewTermsCall),
        LoanRejectNewTerms(LoanRejectNewTermsCall),
        LoanRemoveCollateral(LoanRemoveCollateralCall),
        LoanReturnFunds(LoanReturnFundsCall),
        LoanSetPendingBorrower(LoanSetPendingBorrowerCall),
        LoanSkim(LoanSkimCall),
        LoanUpgrade(LoanUpgradeCall),
        MapleProxiedUpgrade(MapleProxiedUpgradeCall),
        MapleProxyFactoryCreateInstance(MapleProxyFactoryCreateInstanceCall),
        TryLoanAcceptBorrower(TryLoanAcceptBorrowerCall),
        TryLoanCloseLoan(TryLoanCloseLoanCall),
        TryLoanDrawdownFunds(TryLoanDrawdownFundsCall),
        TryLoanFundLoan(TryLoanFundLoanCall),
        TryLoanMakePayment(TryLoanMakePaymentCall),
        TryLoanPostCollateral(TryLoanPostCollateralCall),
        TryLoanProposeNewTerms(TryLoanProposeNewTermsCall),
        TryLoanRemoveCollateral(TryLoanRemoveCollateralCall),
        TryLoanReturnFunds(TryLoanReturnFundsCall),
        TryLoanSetPendingBorrower(TryLoanSetPendingBorrowerCall),
        TryLoanSkim(TryLoanSkimCall),
        TryLoanUpgrade(TryLoanUpgradeCall),
        TryMapleProxiedUpgrade(TryMapleProxiedUpgradeCall),
        TryMapleProxyFactoryCreateInstance(TryMapleProxyFactoryCreateInstanceCall),
    }
    impl ethers::core::abi::AbiDecode for BorrowerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <Erc20ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::Erc20Approve(decoded));
            }
            if let Ok(decoded) =
                <Erc20PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::Erc20Permit(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::Erc20Transfer(decoded));
            }
            if let Ok(decoded) =
                <Erc20TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::Erc20TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <LoanAcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanAcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <LoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanDrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanDrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <LoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <LoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <LoanProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanRejectNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanRejectNewTerms(decoded));
            }
            if let Ok(decoded) =
                <LoanRemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanRemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <LoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <LoanSetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanSetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <LoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanSkim(decoded));
            }
            if let Ok(decoded) =
                <LoanUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::LoanUpgrade(decoded));
            }
            if let Ok(decoded) =
                <MapleProxiedUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::MapleProxiedUpgrade(decoded));
            }
            if let Ok(decoded) =
                <MapleProxyFactoryCreateInstanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BorrowerCalls::MapleProxyFactoryCreateInstance(decoded));
            }
            if let Ok(decoded) =
                <TryLoanAcceptBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanAcceptBorrower(decoded));
            }
            if let Ok(decoded) =
                <TryLoanCloseLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanCloseLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanDrawdownFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanDrawdownFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanFundLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanFundLoan(decoded));
            }
            if let Ok(decoded) =
                <TryLoanMakePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanMakePayment(decoded));
            }
            if let Ok(decoded) =
                <TryLoanPostCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanPostCollateral(decoded));
            }
            if let Ok(decoded) =
                <TryLoanProposeNewTermsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanProposeNewTerms(decoded));
            }
            if let Ok(decoded) =
                <TryLoanRemoveCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanRemoveCollateral(decoded));
            }
            if let Ok(decoded) =
                <TryLoanReturnFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanReturnFunds(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSetPendingBorrowerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BorrowerCalls::TryLoanSetPendingBorrower(decoded));
            }
            if let Ok(decoded) =
                <TryLoanSkimCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanSkim(decoded));
            }
            if let Ok(decoded) =
                <TryLoanUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryLoanUpgrade(decoded));
            }
            if let Ok(decoded) =
                <TryMapleProxiedUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BorrowerCalls::TryMapleProxiedUpgrade(decoded));
            }
            if let Ok(decoded) =
                <TryMapleProxyFactoryCreateInstanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BorrowerCalls::TryMapleProxyFactoryCreateInstance(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BorrowerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BorrowerCalls::Erc20Approve(element) => element.encode(),
                BorrowerCalls::Erc20Permit(element) => element.encode(),
                BorrowerCalls::Erc20Transfer(element) => element.encode(),
                BorrowerCalls::Erc20TransferFrom(element) => element.encode(),
                BorrowerCalls::LoanAcceptBorrower(element) => element.encode(),
                BorrowerCalls::LoanCloseLoan(element) => element.encode(),
                BorrowerCalls::LoanDrawdownFunds(element) => element.encode(),
                BorrowerCalls::LoanFundLoan(element) => element.encode(),
                BorrowerCalls::LoanMakePayment(element) => element.encode(),
                BorrowerCalls::LoanPostCollateral(element) => element.encode(),
                BorrowerCalls::LoanProposeNewTerms(element) => element.encode(),
                BorrowerCalls::LoanRejectNewTerms(element) => element.encode(),
                BorrowerCalls::LoanRemoveCollateral(element) => element.encode(),
                BorrowerCalls::LoanReturnFunds(element) => element.encode(),
                BorrowerCalls::LoanSetPendingBorrower(element) => element.encode(),
                BorrowerCalls::LoanSkim(element) => element.encode(),
                BorrowerCalls::LoanUpgrade(element) => element.encode(),
                BorrowerCalls::MapleProxiedUpgrade(element) => element.encode(),
                BorrowerCalls::MapleProxyFactoryCreateInstance(element) => element.encode(),
                BorrowerCalls::TryLoanAcceptBorrower(element) => element.encode(),
                BorrowerCalls::TryLoanCloseLoan(element) => element.encode(),
                BorrowerCalls::TryLoanDrawdownFunds(element) => element.encode(),
                BorrowerCalls::TryLoanFundLoan(element) => element.encode(),
                BorrowerCalls::TryLoanMakePayment(element) => element.encode(),
                BorrowerCalls::TryLoanPostCollateral(element) => element.encode(),
                BorrowerCalls::TryLoanProposeNewTerms(element) => element.encode(),
                BorrowerCalls::TryLoanRemoveCollateral(element) => element.encode(),
                BorrowerCalls::TryLoanReturnFunds(element) => element.encode(),
                BorrowerCalls::TryLoanSetPendingBorrower(element) => element.encode(),
                BorrowerCalls::TryLoanSkim(element) => element.encode(),
                BorrowerCalls::TryLoanUpgrade(element) => element.encode(),
                BorrowerCalls::TryMapleProxiedUpgrade(element) => element.encode(),
                BorrowerCalls::TryMapleProxyFactoryCreateInstance(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BorrowerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BorrowerCalls::Erc20Approve(element) => element.fmt(f),
                BorrowerCalls::Erc20Permit(element) => element.fmt(f),
                BorrowerCalls::Erc20Transfer(element) => element.fmt(f),
                BorrowerCalls::Erc20TransferFrom(element) => element.fmt(f),
                BorrowerCalls::LoanAcceptBorrower(element) => element.fmt(f),
                BorrowerCalls::LoanCloseLoan(element) => element.fmt(f),
                BorrowerCalls::LoanDrawdownFunds(element) => element.fmt(f),
                BorrowerCalls::LoanFundLoan(element) => element.fmt(f),
                BorrowerCalls::LoanMakePayment(element) => element.fmt(f),
                BorrowerCalls::LoanPostCollateral(element) => element.fmt(f),
                BorrowerCalls::LoanProposeNewTerms(element) => element.fmt(f),
                BorrowerCalls::LoanRejectNewTerms(element) => element.fmt(f),
                BorrowerCalls::LoanRemoveCollateral(element) => element.fmt(f),
                BorrowerCalls::LoanReturnFunds(element) => element.fmt(f),
                BorrowerCalls::LoanSetPendingBorrower(element) => element.fmt(f),
                BorrowerCalls::LoanSkim(element) => element.fmt(f),
                BorrowerCalls::LoanUpgrade(element) => element.fmt(f),
                BorrowerCalls::MapleProxiedUpgrade(element) => element.fmt(f),
                BorrowerCalls::MapleProxyFactoryCreateInstance(element) => element.fmt(f),
                BorrowerCalls::TryLoanAcceptBorrower(element) => element.fmt(f),
                BorrowerCalls::TryLoanCloseLoan(element) => element.fmt(f),
                BorrowerCalls::TryLoanDrawdownFunds(element) => element.fmt(f),
                BorrowerCalls::TryLoanFundLoan(element) => element.fmt(f),
                BorrowerCalls::TryLoanMakePayment(element) => element.fmt(f),
                BorrowerCalls::TryLoanPostCollateral(element) => element.fmt(f),
                BorrowerCalls::TryLoanProposeNewTerms(element) => element.fmt(f),
                BorrowerCalls::TryLoanRemoveCollateral(element) => element.fmt(f),
                BorrowerCalls::TryLoanReturnFunds(element) => element.fmt(f),
                BorrowerCalls::TryLoanSetPendingBorrower(element) => element.fmt(f),
                BorrowerCalls::TryLoanSkim(element) => element.fmt(f),
                BorrowerCalls::TryLoanUpgrade(element) => element.fmt(f),
                BorrowerCalls::TryMapleProxiedUpgrade(element) => element.fmt(f),
                BorrowerCalls::TryMapleProxyFactoryCreateInstance(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Erc20ApproveCall> for BorrowerCalls {
        fn from(var: Erc20ApproveCall) -> Self {
            BorrowerCalls::Erc20Approve(var)
        }
    }
    impl ::std::convert::From<Erc20PermitCall> for BorrowerCalls {
        fn from(var: Erc20PermitCall) -> Self {
            BorrowerCalls::Erc20Permit(var)
        }
    }
    impl ::std::convert::From<Erc20TransferCall> for BorrowerCalls {
        fn from(var: Erc20TransferCall) -> Self {
            BorrowerCalls::Erc20Transfer(var)
        }
    }
    impl ::std::convert::From<Erc20TransferFromCall> for BorrowerCalls {
        fn from(var: Erc20TransferFromCall) -> Self {
            BorrowerCalls::Erc20TransferFrom(var)
        }
    }
    impl ::std::convert::From<LoanAcceptBorrowerCall> for BorrowerCalls {
        fn from(var: LoanAcceptBorrowerCall) -> Self {
            BorrowerCalls::LoanAcceptBorrower(var)
        }
    }
    impl ::std::convert::From<LoanCloseLoanCall> for BorrowerCalls {
        fn from(var: LoanCloseLoanCall) -> Self {
            BorrowerCalls::LoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<LoanDrawdownFundsCall> for BorrowerCalls {
        fn from(var: LoanDrawdownFundsCall) -> Self {
            BorrowerCalls::LoanDrawdownFunds(var)
        }
    }
    impl ::std::convert::From<LoanFundLoanCall> for BorrowerCalls {
        fn from(var: LoanFundLoanCall) -> Self {
            BorrowerCalls::LoanFundLoan(var)
        }
    }
    impl ::std::convert::From<LoanMakePaymentCall> for BorrowerCalls {
        fn from(var: LoanMakePaymentCall) -> Self {
            BorrowerCalls::LoanMakePayment(var)
        }
    }
    impl ::std::convert::From<LoanPostCollateralCall> for BorrowerCalls {
        fn from(var: LoanPostCollateralCall) -> Self {
            BorrowerCalls::LoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<LoanProposeNewTermsCall> for BorrowerCalls {
        fn from(var: LoanProposeNewTermsCall) -> Self {
            BorrowerCalls::LoanProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanRejectNewTermsCall> for BorrowerCalls {
        fn from(var: LoanRejectNewTermsCall) -> Self {
            BorrowerCalls::LoanRejectNewTerms(var)
        }
    }
    impl ::std::convert::From<LoanRemoveCollateralCall> for BorrowerCalls {
        fn from(var: LoanRemoveCollateralCall) -> Self {
            BorrowerCalls::LoanRemoveCollateral(var)
        }
    }
    impl ::std::convert::From<LoanReturnFundsCall> for BorrowerCalls {
        fn from(var: LoanReturnFundsCall) -> Self {
            BorrowerCalls::LoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<LoanSetPendingBorrowerCall> for BorrowerCalls {
        fn from(var: LoanSetPendingBorrowerCall) -> Self {
            BorrowerCalls::LoanSetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<LoanSkimCall> for BorrowerCalls {
        fn from(var: LoanSkimCall) -> Self {
            BorrowerCalls::LoanSkim(var)
        }
    }
    impl ::std::convert::From<LoanUpgradeCall> for BorrowerCalls {
        fn from(var: LoanUpgradeCall) -> Self {
            BorrowerCalls::LoanUpgrade(var)
        }
    }
    impl ::std::convert::From<MapleProxiedUpgradeCall> for BorrowerCalls {
        fn from(var: MapleProxiedUpgradeCall) -> Self {
            BorrowerCalls::MapleProxiedUpgrade(var)
        }
    }
    impl ::std::convert::From<MapleProxyFactoryCreateInstanceCall> for BorrowerCalls {
        fn from(var: MapleProxyFactoryCreateInstanceCall) -> Self {
            BorrowerCalls::MapleProxyFactoryCreateInstance(var)
        }
    }
    impl ::std::convert::From<TryLoanAcceptBorrowerCall> for BorrowerCalls {
        fn from(var: TryLoanAcceptBorrowerCall) -> Self {
            BorrowerCalls::TryLoanAcceptBorrower(var)
        }
    }
    impl ::std::convert::From<TryLoanCloseLoanCall> for BorrowerCalls {
        fn from(var: TryLoanCloseLoanCall) -> Self {
            BorrowerCalls::TryLoanCloseLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanDrawdownFundsCall> for BorrowerCalls {
        fn from(var: TryLoanDrawdownFundsCall) -> Self {
            BorrowerCalls::TryLoanDrawdownFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanFundLoanCall> for BorrowerCalls {
        fn from(var: TryLoanFundLoanCall) -> Self {
            BorrowerCalls::TryLoanFundLoan(var)
        }
    }
    impl ::std::convert::From<TryLoanMakePaymentCall> for BorrowerCalls {
        fn from(var: TryLoanMakePaymentCall) -> Self {
            BorrowerCalls::TryLoanMakePayment(var)
        }
    }
    impl ::std::convert::From<TryLoanPostCollateralCall> for BorrowerCalls {
        fn from(var: TryLoanPostCollateralCall) -> Self {
            BorrowerCalls::TryLoanPostCollateral(var)
        }
    }
    impl ::std::convert::From<TryLoanProposeNewTermsCall> for BorrowerCalls {
        fn from(var: TryLoanProposeNewTermsCall) -> Self {
            BorrowerCalls::TryLoanProposeNewTerms(var)
        }
    }
    impl ::std::convert::From<TryLoanRemoveCollateralCall> for BorrowerCalls {
        fn from(var: TryLoanRemoveCollateralCall) -> Self {
            BorrowerCalls::TryLoanRemoveCollateral(var)
        }
    }
    impl ::std::convert::From<TryLoanReturnFundsCall> for BorrowerCalls {
        fn from(var: TryLoanReturnFundsCall) -> Self {
            BorrowerCalls::TryLoanReturnFunds(var)
        }
    }
    impl ::std::convert::From<TryLoanSetPendingBorrowerCall> for BorrowerCalls {
        fn from(var: TryLoanSetPendingBorrowerCall) -> Self {
            BorrowerCalls::TryLoanSetPendingBorrower(var)
        }
    }
    impl ::std::convert::From<TryLoanSkimCall> for BorrowerCalls {
        fn from(var: TryLoanSkimCall) -> Self {
            BorrowerCalls::TryLoanSkim(var)
        }
    }
    impl ::std::convert::From<TryLoanUpgradeCall> for BorrowerCalls {
        fn from(var: TryLoanUpgradeCall) -> Self {
            BorrowerCalls::TryLoanUpgrade(var)
        }
    }
    impl ::std::convert::From<TryMapleProxiedUpgradeCall> for BorrowerCalls {
        fn from(var: TryMapleProxiedUpgradeCall) -> Self {
            BorrowerCalls::TryMapleProxiedUpgrade(var)
        }
    }
    impl ::std::convert::From<TryMapleProxyFactoryCreateInstanceCall> for BorrowerCalls {
        fn from(var: TryMapleProxyFactoryCreateInstanceCall) -> Self {
            BorrowerCalls::TryMapleProxyFactoryCreateInstance(var)
        }
    }
}
