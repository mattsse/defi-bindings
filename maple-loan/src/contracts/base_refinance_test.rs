pub use baserefinancetest_mod::*;
#[allow(clippy::too_many_arguments)]
mod baserefinancetest_mod {
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
    #[doc = "BaseRefinanceTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BASEREFINANCETEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BASEREFINANCETEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805461ff01600160b01b031916757109709ecfa91a80626ff3989d68f67f5b1dd12d000117905534801561003957600080fd5b50612457806100496000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80630a9254e414610046578063ba414fa614610050578063fa7626d414610076575b600080fd5b61004e610083565b005b60005461006290610100900460ff1681565b604051901515815260200160405180910390f35b6000546100629060ff1681565b60405161008f9061022a565b604051809103906000f0801580156100ab573d6000803e3d6000fd5b50600280546001600160a01b0319166001600160a01b03929092169190911790556040516100d890610237565b604051809103906000f0801580156100f4573d6000803e3d6000fd5b50600660006101000a8154816001600160a01b0302191690836001600160a01b0316021790555030600080600060405161012d90610244565b6001600160a01b03948516815293909216602084015260408301526060820152608001604051809103906000f08015801561016c573d6000803e3d6000fd5b50600380546001600160a01b0319166001600160a01b039290921691909117905560405161019990610251565b604051809103906000f0801580156101b5573d6000803e3d6000fd5b50600580546001600160a01b0319166001600160a01b03928316908117909155600354604051636617051360e11b8152921660048301529063cc2e0a2690602401600060405180830381600087803b15801561021057600080fd5b505af1158015610224573d6000803e3d6000fd5b50505050565b6115098061025f83390190565b6106ab8061176883390190565b61035a80611e1383390190565b6102b58061216d8339019056fe6080604052600080546001600160a01b031916600817905534801561002357600080fd5b506114d6806100336000396000f3fe608060405234801561001057600080fd5b50600436106101cf5760003560e01c80636fc7d91111610104578063ac8a0501116100a2578063c2b63f4f11610071578063c2b63f4f1461043e578063d75edca414610451578063ec16a8d814610464578063fc179d671461047757600080fd5b8063ac8a0501146103f2578063afb7a0f614610405578063b6a0f33114610418578063c1a3c9d71461042b57600080fd5b80637fe25581116100de5780637fe25581146103a65780638b196cd5146103b957806398e5c234146103cc578063a18814da146103df57600080fd5b80636fc7d911146103505780637327de97146103805780637c2e27d41461039357600080fd5b80634046af2b1161017157806351cf23b11161014b57806351cf23b114610304578063574c784414610317578063687468d11461032a5780636c202fcb1461033d57600080fd5b80634046af2b1461029e578063429bf6c8146102c95780635100d121146102dc57600080fd5b80631649e2ef116101ad5780631649e2ef14610224578063188b80b8146102575780632893a76e1461026a5780633d73ea8f1461028b57600080fd5b8063035e30b3146101d457806305eee8e7146101fc5780631515a11614610211575b600080fd5b6101e76101e236600461112b565b61048a565b60405190151581526020015b60405180910390f35b61020f61020a3660046110a8565b610543565b005b61020f61021f366004610fcc565b6105d5565b61023761023236600461124d565b61062b565b6040805194855260208501939093529183015260608201526080016101f3565b61020f6102653660046111d6565b6106c1565b61027d61027836600461124d565b61072f565b6040519081526020016101f3565b61027d61029936600461124d565b6107b4565b6000546102b1906001600160a01b031681565b6040516001600160a01b0390911681526020016101f3565b6102376102d736600461124d565b6107e4565b6102ef6102ea366004610fe7565b610818565b604080519283526020830191909152016101f3565b61020f61031236600461112b565b6108a3565b61020f610325366004611167565b61092b565b6101e761033836600461124d565b610996565b6101e761034b3660046111d6565b610a47565b61020f61035e366004610fcc565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6101e761038e36600461101a565b610b02565b6101e76103a136600461124d565b610b35565b6101e76103b436600461112b565b610b5c565b61027d6103c736600461112b565b610bea565b6101e76103da366004610fe7565b610c71565b6101e76103ed366004611277565b610c9c565b61020f610400366004610fe7565b610cce565b61027d61041336600461101a565b610d2d565b6101e761042636600461124d565b610d67565b6101e7610439366004610fe7565b610d8e565b61020f61044c366004611277565b610db9565b6101e761045f36600461124d565b610e20565b6101e7610472366004610fcc565b610e47565b6101e761048536600461105d565b610ed4565b6040516001600160a01b038381166024830152604482018390526000919085169063e920b1e160e01b906064015b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516104f691906113f9565b6000604051808303816000865af19150503d8060008114610533576040519150601f19603f3d011682016040523d82523d6000602084013e610538565b606091505b509095945050505050565b60405163d505accf60e01b81526001600160a01b0388811660048301528781166024830152604482018790526064820186905260ff8516608483015260a4820184905260c4820183905289169063d505accf9060e401600060405180830381600087803b1580156105b357600080fd5b505af11580156105c7573d6000803e3d6000fd5b505050505050505050505050565b806001600160a01b0316630fe3d9b76040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561061057600080fd5b505af1158015610624573d6000803e3d6000fd5b5050505050565b600080600080856001600160a01b031663d05951a0866040518263ffffffff1660e01b815260040161065f91815260200190565b608060405180830381600087803b15801561067957600080fd5b505af115801561068d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106b19190611309565b9299919850965090945092505050565b6040516328565a7760e11b81526001600160a01b038716906350acb4ee906106f59088908890889088908890600401611466565b600060405180830381600087803b15801561070f57600080fd5b505af1158015610723573d6000803e3d6000fd5b50505050505050505050565b6040516322baaeeb60e11b8152600481018290526000906001600160a01b038416906345755dd6906024015b602060405180830381600087803b15801561077557600080fd5b505af1158015610789573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107ad91906112cc565b9392505050565b6040516350f2012f60e01b8152600481018290526000906001600160a01b038416906350f2012f9060240161075b565b600080600080856001600160a01b0316635114cb52866040518263ffffffff1660e01b815260040161065f91815260200190565b6040516347350e9f60e01b81526001600160a01b03828116600483015260009182918516906347350e9f906024016040805180830381600087803b15801561085f57600080fd5b505af1158015610873573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089791906112e5565b915091505b9250929050565b60405163095ea7b360e01b81526001600160a01b0383811660048301526024820183905284169063095ea7b390604401602060405180830381600087803b1580156108ed57600080fd5b505af1158015610901573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092591906112aa565b50505050565b604051632b2d48ad60e21b81526001600160a01b0386169063acb522b49061095d908790879087908790600401611434565b600060405180830381600087803b15801561097757600080fd5b505af115801561098b573d6000803e3d6000fd5b505050505050505050565b6000826001600160a01b03166350f2012f60e01b836040516024016109bd91815260200190565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516109fb91906113f9565b6000604051808303816000865af19150503d8060008114610a38576040519150601f19603f3d011682016040523d82523d6000602084013e610a3d565b606091505b5090949350505050565b6000866001600160a01b03166350acb4ee60e01b8787878787604051602401610a74959493929190611466565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051610ab291906113f9565b6000604051808303816000865af19150503d8060008114610aef576040519150601f19603f3d011682016040523d82523d6000602084013e610af4565b606091505b509098975050505050505050565b6040516001600160a01b03838116602483015282811660448301526000919085169063712b772f60e01b906064016104b8565b6000826001600160a01b03166345755dd660e01b836040516024016109bd91815260200190565b60405163a9059cbb60e01b81526001600160a01b038381166004830152602482018390526000919085169063a9059cbb90604401602060405180830381600087803b158015610baa57600080fd5b505af1158015610bbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be291906112aa565b949350505050565b60405163e920b1e160e01b81526001600160a01b038381166004830152602482018390526000919085169063e920b1e1906044015b602060405180830381600087803b158015610c3957600080fd5b505af1158015610c4d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610be291906112cc565b6040516001600160a01b038281166024830152600091908416906347350e9f60e01b906044016109bd565b604051602481018390526001600160a01b0382811660448301526000919085169063390d685560e01b906064016104b8565b60405163267f4ac360e01b81526001600160a01b03828116600483015283169063267f4ac390602401600060405180830381600087803b158015610d1157600080fd5b505af1158015610d25573d6000803e3d6000fd5b505050505050565b60405163712b772f60e01b81526001600160a01b03838116600483015282811660248301526000919085169063712b772f90604401610c1f565b6000826001600160a01b031663d05951a060e01b836040516024016109bd91815260200190565b6040516001600160a01b0382811660248301526000919084169063267f4ac360e01b906044016109bd565b60405163390d685560e01b8152600481018390526001600160a01b03828116602483015284169063390d685590604401600060405180830381600087803b158015610e0357600080fd5b505af1158015610e17573d6000803e3d6000fd5b50505050505050565b6000826001600160a01b0316635114cb5260e01b836040516024016109bd91815260200190565b60408051600481526024810182526020810180516001600160e01b0316630fe3d9b760e01b17905290516000916001600160a01b03841691610e8991906113f9565b6000604051808303816000865af19150503d8060008114610ec6576040519150601f19603f3d011682016040523d82523d6000602084013e610ecb565b606091505b50909392505050565b6040516323b872dd60e01b81526001600160a01b038481166004830152838116602483015260448201839052600091908616906323b872dd90606401602060405180830381600087803b158015610f2a57600080fd5b505af1158015610f3e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f6291906112aa565b95945050505050565b80356001600160a01b0381168114610f8257600080fd5b919050565b60008083601f840112610f9957600080fd5b50813567ffffffffffffffff811115610fb157600080fd5b6020830191508360208260051b850101111561089c57600080fd5b600060208284031215610fde57600080fd5b6107ad82610f6b565b60008060408385031215610ffa57600080fd5b61100383610f6b565b915061101160208401610f6b565b90509250929050565b60008060006060848603121561102f57600080fd5b61103884610f6b565b925061104660208501610f6b565b915061105460408501610f6b565b90509250925092565b6000806000806080858703121561107357600080fd5b61107c85610f6b565b935061108a60208601610f6b565b925061109860408601610f6b565b9396929550929360600135925050565b600080600080600080600080610100898b0312156110c557600080fd5b6110ce89610f6b565b97506110dc60208a01610f6b565b96506110ea60408a01610f6b565b9550606089013594506080890135935060a089013560ff8116811461110e57600080fd5b979a969950949793969295929450505060c08201359160e0013590565b60008060006060848603121561114057600080fd5b61114984610f6b565b925061115760208501610f6b565b9150604084013590509250925092565b60008060008060006080868803121561117f57600080fd5b61118886610f6b565b945061119660208701610f6b565b935060408601359250606086013567ffffffffffffffff8111156111b957600080fd5b6111c588828901610f87565b969995985093965092949392505050565b60008060008060008060a087890312156111ef57600080fd5b6111f887610f6b565b955061120660208801610f6b565b945060408701359350606087013567ffffffffffffffff81111561122957600080fd5b61123589828a01610f87565b979a9699509497949695608090950135949350505050565b6000806040838503121561126057600080fd5b61126983610f6b565b946020939093013593505050565b60008060006060848603121561128c57600080fd5b61129584610f6b565b92506020840135915061105460408501610f6b565b6000602082840312156112bc57600080fd5b815180151581146107ad57600080fd5b6000602082840312156112de57600080fd5b5051919050565b600080604083850312156112f857600080fd5b505080516020909101519092909150565b6000806000806080858703121561131f57600080fd5b505082516020840151604085015160609095015191969095509092509050565b81835260006020808501808196508560051b810191508460005b878110156113c35782840389528135601e1988360301811261137a57600080fd5b8701803567ffffffffffffffff81111561139357600080fd5b8036038913156113a257600080fd5b6113af86828985016113d0565b9a87019a9550505090840190600101611359565b5091979650505050505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000825160005b8181101561141a5760208186018101518583015201611400565b81811115611429576000828501525b509190910192915050565b60018060a01b038516815283602082015260606040820152600061145c60608301848661133f565b9695505050505050565b60018060a01b038616815284602082015260806040820152600061148e60808301858761133f565b9050826060830152969550505050505056fea2646970667358221220301d0fee16738fb00c4d4c34da46b6b890b0014431a7574e9b2ea66fcb50d08564736f6c63430008070033608060405234801561001057600080fd5b5061068b806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c8063b163ff9711610066578063b163ff9714610104578063d157f64514610117578063e94134d91461012a578063f2f659601461013d578063fe12afe91461015057600080fd5b80634764757e146100a35780635de1858c146100b85780635f84f302146100cb5780637febd92b146100de5780639c3c2ab1146100f1575b600080fd5b6100b66100b13660046105de565b610163565b005b6100b66100c63660046105de565b6101e5565b6100b66100d93660046105de565b61021a565b6100b66100ec3660046105de565b61024f565b6100b66100ff3660046105de565b610284565b6100b66101123660046105de565b6102b9565b6100b66101253660046105de565b610397565b6100b66101383660046105de565b61040d565b6100b661014b3660046105de565b610494565b6100b661015e3660046105de565b6104c9565b806101a95760405162461bcd60e51b8152602060048201526011602482015270148e94d4148e96915493d7d05353d55395607a1b60448201526064015b60405180910390fd5b60138190556040518181527f35ea55dd4eee968798b5f61efffeb9d5f88b42c05aee6151a1bb4836c082ad21906020015b60405180910390a150565b600c8190556040518181527fd08f964ebebbb23438d7327a9e0b4d3a6977b689a76fbbc5e3ff6cf2bd57c296906020016101da565b60088190556040518181527f532f252238b3b0d2b8c8a257b087fb3fdbdc775e3e0acca8e680a2f36aafa34b906020016101da565b600a8190556040518181527f901c1ec58c5f0467430dc60e75d29fc21f09505b87ae0f9add2ca0aa75f172b5906020016101da565b600b8190556040518181527fa1367e43892cb30dbdf580f60f215aae22c2575ecd8bfef69d87c3671ad79a3b906020016101da565b60055481906102d0906001600160a01b03166104fe565b101561031e5760405162461bcd60e51b815260206004820152601860248201527f523a49503a494e53554646494349454e545f414d4f554e54000000000000000060448201526064016101a0565b80601460008282546103309190610610565b9250508190555080600d60008282546103499190610610565b9250508190555080600f60008282546103629190610610565b90915550506040518181527fc8fcde6244e516452771097a17ecb4c9e6331f498c228f1a951518aedefcb5ee906020016101da565b806103d85760405162461bcd60e51b8152602060048201526011602482015270148e94d4124e96915493d7d05353d55395607a1b60448201526064016101a0565b60078190556040518181527f262b925b6c3983fb29f10dd4493cd3accbbf1fceec18a61b59b2e663b795a37a906020016101da565b60145481111561045f5760405162461bcd60e51b815260206004820152601d60248201527f523a5345503a41424f56455f43555252454e545f5052494e434950414c00000060448201526064016101a0565b600e8190556040518181527fbef1806a01fbfd8f94363f8c74073ad697b31d9278da8c01247366e6c8aa1678906020016101da565b60068190556040518181527f376aafccbf0af4f25bc38eb52182d4604f044d0d87e4cb26e1667b50e3a1de05906020016101da565b60098190556040518181527f6c16b95dcb84ecf131270f4ff8500490839e14db1d5fb8d8a8c89da8551806f8906020016101da565b6005546000906001600160a01b0383811691161461051d57600061052d565b600f5460105461052d9190610610565b6004546001600160a01b0384811691161461054957600061054d565b6011545b6040516370a0823160e01b81523060048201526001600160a01b038516906370a082319060240160206040518083038186803b15801561058c57600080fd5b505afa1580156105a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105c491906105f7565b6105ce9190610628565b6105d89190610628565b92915050565b6000602082840312156105f057600080fd5b5035919050565b60006020828403121561060957600080fd5b5051919050565b600082198211156106235761062361063f565b500190565b60008282101561063a5761063a61063f565b500390565b634e487b7160e01b600052601160045260246000fdfea26469706673582212207ec41d97fd74277290ee65f2b3a186dff3b38b0278f194726215bf68f29423f364736f6c63430008070033608060405234801561001057600080fd5b5060405161035a38038061035a83398101604081905261002f91610089565b600080546001600160a01b039586166001600160a01b03199182161790915560018054949095169316929092179092556002919091556003556100cc565b80516001600160a01b038116811461008457600080fd5b919050565b6000806000806080858703121561009f57600080fd5b6100a88561006d565b93506100b66020860161006d565b6040860151606090960151949790965092505050565b61027f806100db6000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c806377e741c71161006657806377e741c7146101535780638275d47114610166578063a5a2760514610192578063c42cf535146101a5578063cc32d176146101d557600080fd5b80630c340a24146100a357806316a12d7a146100d3578063425fad58146100ea5780635e0454671461010e5780637303de2514610123575b600080fd5b6000546100b6906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100dc60025481565b6040519081526020016100ca565b6001546100fe90600160a01b900460ff1681565b60405190151581526020016100ca565b61012161011c366004610230565b600255565b005b6101216101313660046101de565b600180546001600160a01b0319166001600160a01b0392909216919091179055565b610121610161366004610230565b600355565b61012161017436600461020e565b60018054911515600160a01b0260ff60a01b19909216919091179055565b6001546100b6906001600160a01b031681565b6101216101b33660046101de565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b6100dc60035481565b6000602082840312156101f057600080fd5b81356001600160a01b038116811461020757600080fd5b9392505050565b60006020828403121561022057600080fd5b8135801515811461020757600080fd5b60006020828403121561024257600080fd5b503591905056fea2646970667358221220960fa598f91991aa59fd5d826ee225ed174faa4e40506b35f958bb6a3edf990264736f6c63430008070033608060405234801561001057600080fd5b50610295806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80633a60339a14610046578063cc2e0a2614610075578063fe69f708146100a7575b600080fd5b600054610059906001600160a01b031681565b6040516001600160a01b03909116815260200160405180910390f35b6100a561008336600461016c565b600080546001600160a01b0319166001600160a01b0392909216919091179055565b005b6100a56100b5366004610190565b60006100c38284018461016c565b6040516001600160a01b0382166024820152909150600090339060440160408051601f198184030181529181526020820180516001600160e01b0316636bc26a1360e11b17905251610115919061020c565b6000604051808303816000865af19150503d8060008114610152576040519150601f19603f3d011682016040523d82523d6000602084013e610157565b606091505b505090508061016557600080fd5b5050505050565b60006020828403121561017e57600080fd5b813561018981610247565b9392505050565b6000806000604084860312156101a557600080fd5b83359250602084013567ffffffffffffffff808211156101c457600080fd5b818601915086601f8301126101d857600080fd5b8135818111156101e757600080fd5b8760208285010111156101f957600080fd5b6020830194508093505050509250925092565b6000825160005b8181101561022d5760208186018101518583015201610213565b8181111561023c576000828501525b509190910192915050565b6001600160a01b038116811461025c57600080fd5b5056fea264697066735822122044c2952481de96a68fedc9e8ca53c09f645eaad1bc30867ed73a969abcfe58a864736f6c63430008070033a26469706673582212205a3f22980eb5b9c207e037e3a773fcb839044d3fbe81ba4cac5d152cbf67198164736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct BaseRefinanceTest<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BaseRefinanceTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BaseRefinanceTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BaseRefinanceTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BaseRefinanceTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BASEREFINANCETEST_ABI.clone(), client)
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
                BASEREFINANCETEST_ABI.clone(),
                BASEREFINANCETEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, BaseRefinanceTestEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for BaseRefinanceTest<M>
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BaseRefinanceTestEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for BaseRefinanceTestEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(BaseRefinanceTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for BaseRefinanceTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseRefinanceTestEvents::LogFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogAddressFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogBytesFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogBytes32Filter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogIntFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogStringFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogUintFilter(element) => element.fmt(f),
                BaseRefinanceTestEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_TEST`function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `failed`function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `setUp`function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BaseRefinanceTestCalls {
        IsTest(IsTestCall),
        Failed(FailedCall),
        SetUp(SetUpCall),
    }
    impl ethers::core::abi::AbiDecode for BaseRefinanceTestCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRefinanceTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRefinanceTestCalls::Failed(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BaseRefinanceTestCalls::SetUp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BaseRefinanceTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BaseRefinanceTestCalls::IsTest(element) => element.encode(),
                BaseRefinanceTestCalls::Failed(element) => element.encode(),
                BaseRefinanceTestCalls::SetUp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BaseRefinanceTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BaseRefinanceTestCalls::IsTest(element) => element.fmt(f),
                BaseRefinanceTestCalls::Failed(element) => element.fmt(f),
                BaseRefinanceTestCalls::SetUp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for BaseRefinanceTestCalls {
        fn from(var: IsTestCall) -> Self {
            BaseRefinanceTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<FailedCall> for BaseRefinanceTestCalls {
        fn from(var: FailedCall) -> Self {
            BaseRefinanceTestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for BaseRefinanceTestCalls {
        fn from(var: SetUpCall) -> Self {
            BaseRefinanceTestCalls::SetUp(var)
        }
    }
}
