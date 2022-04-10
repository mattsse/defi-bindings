pub use mapleloanfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleloanfactory_mod {
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
    #[doc = "MapleLoanFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLELOANFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DefaultVersionSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ImplementationRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"initializationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"migrationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MapleGlobalsSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathEnabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createInstance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"defaultImplementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInstanceAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instanceAddress_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementationOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isLoan\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mapleGlobals\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"migratorForPath\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDefaultVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGlobals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"upgradeEnabledForPath\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeInstance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"versionOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLELOANFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162001ccb38038062001ccb83398101604081905262000034916200013e565b8060006001600160a01b031681600360006101000a8154816001600160a01b0302191690836001600160a01b0316021790556001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b1580156200009f57600080fd5b505afa158015620000b4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000da91906200013e565b6001600160a01b03161415620001365760405162461bcd60e51b815260206004820152601560248201527f4d50463a433a494e56414c49445f474c4f42414c530000000000000000000000604482015260640160405180910390fd5b505062000170565b6000602082840312156200015157600080fd5b81516001600160a01b03811681146200016957600080fd5b9392505050565b611b4b80620001806000396000f3fe60806040523480156200001157600080fd5b5060043610620001155760003560e01c80638636f07e11620000a3578063b4e6747f116200006e578063b4e6747f14620002a0578063cc2e0a2614620002b7578063d867e0de14620002ce578063fe69f70814620002ff57600080fd5b80638636f07e1462000225578063881829121462000251578063b28317bf1462000268578063b39c4593146200027f57600080fd5b80632819cbc211620000e45780632819cbc214620001ac5780633a60339a14620001e3578063517b657f14620001f757806385b8a52f146200020e57600080fd5b80630db3ff45146200011a5780630e6e4b25146200015957806312700bae14620001895780631798d48214620001a2575b600080fd5b620001466200012b366004620015f3565b6001600160a01b031660009081526001602052604090205490565b6040519081526020015b60405180910390f35b620001706200016a36600462001633565b62000316565b6040516001600160a01b03909116815260200162000150565b620001a06200019a3660046200169d565b62000359565b005b6200014660045481565b620001d2620001bd366004620015f3565b60066020526000908152604090205460ff1681565b604051901515815260200162000150565b60035462000170906001600160a01b031681565b620001706200020836600462001633565b62000562565b620001706200021f36600462001734565b620005a9565b620001706200023636600462001683565b6000908152602081905260409020546001600160a01b031690565b620001a06200026236600462001734565b620005d3565b620001a06200027936600462001757565b6200077f565b6004546000908152602081905260409020546001600160a01b031662000170565b620001a0620002b136600462001683565b62000939565b620001a0620002c8366004620015f3565b62000a9b565b620001d2620002df36600462001734565b600560209081526000928352604080842090915290825290205460ff1681565b620001a062000310366004620016e4565b62000c74565b60006200034f848484604051602001620003339392919062001788565b6040516020818303038152906040528051906020012062000e37565b90505b9392505050565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b158015620003a857600080fd5b505afa158015620003bd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003e3919062001613565b6001600160a01b0316336001600160a01b0316146200041f5760405162461bcd60e51b815260040162000416906200185e565b60405180910390fd5b82620004675760405162461bcd60e51b815260206004820152601660248201527526a8231d29249d24a72b20a624a22fab22a929a4a7a760511b604482015260640162000416565b806001600160a01b0316826001600160a01b0316847fe69962526b7f07862bf85663f861564361295f9601236fbbe056591eb1b63f3b60405160405180910390a4620004b4838362000f12565b620005025760405162461bcd60e51b815260206004820152601e60248201527f4d50463a52493a4641494c5f464f525f494d504c454d454e544154494f4e0000604482015260640162000416565b6200050f83848362000fbe565b6200055d5760405162461bcd60e51b815260206004820152601860248201527f4d50463a52493a4641494c5f464f525f4d49475241544f520000000000000000604482015260640162000416565b505050565b60006001600660006200057787878762001048565b6001600160a01b03811682526020820192909252604001600020805460ff191692151592909217909155949350505050565b60008281526002602090815260408083208484529091529020546001600160a01b03165b92915050565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b1580156200062257600080fd5b505afa15801562000637573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200065d919062001613565b6001600160a01b0316336001600160a01b031614620006905760405162461bcd60e51b815260040162000416906200185e565b80821415620006e25760405162461bcd60e51b815260206004820152601f60248201527f4d50463a4455503a4f56455257524954494e475f494e495449414c495a455200604482015260640162000416565b620006f08282600062000fbe565b6200072f5760405162461bcd60e51b815260206004820152600e60248201526d1354118e9115540e91905253115160921b604482015260640162000416565b604051819083907fa46f1addc2236b7d93ed2a8a507f1c47cc92656d2b6f82bf0876da9e964b9e5e90600090a360009182526005602090815260408084209284529190529020805460ff19169055565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b158015620007ce57600080fd5b505afa158015620007e3573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000809919062001613565b6001600160a01b0316336001600160a01b0316146200083c5760405162461bcd60e51b815260040162000416906200185e565b818314156200088e5760405162461bcd60e51b815260206004820152601f60248201527f4d50463a4555503a4f56455257524954494e475f494e495449414c495a455200604482015260640162000416565b6200089b83838362000fbe565b620008da5760405162461bcd60e51b815260206004820152600e60248201526d1354118e9155540e91905253115160921b604482015260640162000416565b806001600160a01b031682847f549a41e54b51dcc3f29b51bb02a8adcc4ec5ae26604608e41bde60311dcef10660405160405180910390a45060009182526005602090815260408084209284529190529020805460ff19166001179055565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b1580156200098857600080fd5b505afa1580156200099d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620009c3919062001613565b6001600160a01b0316336001600160a01b031614620009f65760405162461bcd60e51b815260040162000416906200185e565b80158062000a1a57506000818152602081905260409020546001600160a01b031615155b62000a685760405162461bcd60e51b815260206004820152601760248201527f4d50463a5344563a494e56414c49445f56455253494f4e000000000000000000604482015260640162000416565b600481905560405181907fddb2013cf7f102d15447c4c1e94cf56823455f02eb244d0c3b2ef6516338934690600090a250565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b15801562000aea57600080fd5b505afa15801562000aff573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000b25919062001613565b6001600160a01b0316336001600160a01b03161462000b585760405162461bcd60e51b815260040162000416906200185e565b60006001600160a01b0316816001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b15801562000b9d57600080fd5b505afa15801562000bb2573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000bd8919062001613565b6001600160a01b0316141562000c2a5760405162461bcd60e51b81526020600482015260166024820152754d50463a53473a494e56414c49445f474c4f42414c5360501b604482015260640162000416565b600380546001600160a01b0319166001600160a01b0383169081179091556040517f9074839b84a74138be159cb7813a72c2a44c35fe8c53da66a16da385d348c5f190600090a250565b600060016000336001600160a01b0316635c60da1b6040518163ffffffff1660e01b815260040160206040518083038186803b15801562000cb457600080fd5b505afa15801562000cc9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000cef919062001613565b6001600160a01b03168152602080820192909252604090810160009081205480825260058452828220888352909352205490915060ff1662000d695760405162461bcd60e51b81526020600482015260126024820152711354118e95524e9393d517d0531313d5d15160721b604482015260640162000416565b8381336001600160a01b03167ffbb4f36b70dba8ecedc8b38361f44f1b0c61e04ec4e0ccf620649dc558573f5f868660405162000da89291906200182f565b60405180910390a462000df3338585858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506200114b92505050565b62000e315760405162461bcd60e51b815260206004820152600d60248201526c1354118e95524e919052531151609a1b604482015260640162000416565b50505050565b600060ff60f81b30836040518060200162000e529062001599565b601f1982820381018352601f90910116604081815230602083015260009082015260600160408051601f198184030181529082905262000e969291602001620017b8565b6040516020818303038152906040528051906020012060405160200162000ef494939291906001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60408051601f19818403018152919052805160209091012092915050565b600082158062000f3857506000838152602081905260409020546001600160a01b031615155b8062000f5b57506001600160a01b03821660009081526001602052604090205415155b8062000f6f57506001600160a01b0382163b155b1562000f7e57506000620005cd565b5060008281526020818152604080832080546001600160a01b0319166001600160a01b039590951694851790559282526001908190529190209190915590565b600083158062000fcc575082155b1562000fdb5750600062000352565b6001600160a01b0382161580159062000ffc57506001600160a01b0382163b155b156200100b5750600062000352565b506000928352600260209081526040808520938552929052912080546001600160a01b0319166001600160a01b0392909216919091179055600190565b600080620010b885858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250506040516200109c92508991508890889060200162001788565b6040516020818303038152906040528051906020012062001363565b9250905080620010fb5760405162461bcd60e51b815260206004820152600d60248201526c1354118e90d24e919052531151609a1b604482015260640162000416565b816001600160a01b03166004547f8919387fe006fdc29a3dfcc183071d7974bc03747fedb0019630f1e13f85cc6487876040516200113b9291906200182f565b60405180910390a3509392505050565b60006001600160a01b0384163b620011665750600062000352565b6000838152602081905260409020546001600160a01b0316806200118f57600091505062000352565b60006200119c8662001401565b909350905082620011b35760009250505062000352565b604080516001600160a01b0384811660248084019190915283518084039091018152604490920183526020820180516001600160e01b0316636bc26a1360e11b1790529151918816916200120891906200179a565b6000604051808303816000865af19150503d806000811462001247576040519150601f19603f3d011682016040523d82523d6000602084013e6200124c565b606091505b50508093505082620012645760009250505062000352565b6001600160a01b038082166000908152600160209081526040808320548352600282528083208984529091529020541680620012aa576000855114935050505062000352565b866001600160a01b031663c3fbb6fd60e01b8287604051602401620012d1929190620017eb565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516200131191906200179a565b6000604051808303816000865af19150503d806000811462001350576040519150601f19603f3d011682016040523d82523d6000602084013e62001355565b606091505b509098975050505050505050565b60008082306000604051620013789062001599565b6001600160a01b039283168152911660208201526040018190604051809103906000f5905080158015620013b0573d6000803e3d6000fd5b5090506000620013c08262001401565b6001600160a01b03811660009081526001602052604090205490925090508015801590620013f65750620013f6838288620014b1565b935050509250929050565b60408051600481526024810182526020810180516001600160e01b0316635c60da1b60e01b179052905160009182916060916001600160a01b038616916200144a91906200179a565b600060405180830381855afa9150503d806000811462001487576040519150601f19603f3d011682016040523d82523d6000602084013e6200148c565b606091505b5080519194509150620014a9908201602090810190830162001613565b915050915091565b60008281526002602090815260408083209091528120546001600160a01b031680620014e257505080511562000352565b846001600160a01b031663c3fbb6fd60e01b828560405160240162001509929190620017eb565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b03199094169390931790925290516200154991906200179a565b6000604051808303816000865af19150503d806000811462001588576040519150601f19603f3d011682016040523d82523d6000602084013e6200158d565b606091505b50909695505050505050565b61024580620018d183390190565b60008083601f840112620015ba57600080fd5b50813567ffffffffffffffff811115620015d357600080fd5b602083019150836020828501011115620015ec57600080fd5b9250929050565b6000602082840312156200160657600080fd5b81356200035281620018b7565b6000602082840312156200162657600080fd5b81516200035281620018b7565b6000806000604084860312156200164957600080fd5b833567ffffffffffffffff8111156200166157600080fd5b6200166f86828701620015a7565b909790965060209590950135949350505050565b6000602082840312156200169657600080fd5b5035919050565b600080600060608486031215620016b357600080fd5b833592506020840135620016c781620018b7565b91506040840135620016d981620018b7565b809150509250925092565b600080600060408486031215620016fa57600080fd5b83359250602084013567ffffffffffffffff8111156200171957600080fd5b6200172786828701620015a7565b9497909650939450505050565b600080604083850312156200174857600080fd5b50508035926020909101359150565b6000806000606084860312156200176d57600080fd5b83359250602084013591506040840135620016d981620018b7565b82848237909101908152602001919050565b60008251620017ae81846020870162001888565b9190910192915050565b60008351620017cc81846020880162001888565b835190830190620017e281836020880162001888565b01949350505050565b60018060a01b038316815260406020820152600082518060408401526200181a81606085016020870162001888565b601f01601f1916919091016060019392505050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b60208082526010908201526f26a8231d2727aa2fa3a7ab22a92727a960811b604082015260600190565b60005b83811015620018a55781810151838201526020016200188b565b8381111562000e315750506000910152565b6001600160a01b0381168114620018cd57600080fd5b5056fe608060405234801561001057600080fd5b5060405161024538038061024583398101604081905261002f91610169565b6001600160a01b0382167f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15560006001600160a01b0382161561007257816100e3565b826001600160a01b031663b39c45936040518163ffffffff1660e01b815260040160206040518083038186803b1580156100ab57600080fd5b505afa1580156100bf573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e39190610147565b90506001600160a01b0381166100f857600080fd5b6001600160a01b03167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc555061019c9050565b80516001600160a01b038116811461014257600080fd5b919050565b60006020828403121561015957600080fd5b6101628261012b565b9392505050565b6000806040838503121561017c57600080fd5b6101858361012b565b91506101936020840161012b565b90509250929050565b609b806101aa6000396000f3fe60806040526000602d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b90506001600160a01b0381163b604257600080fd5b3660008037600080366000845af43d6000803e8080156060573d6000f35b3d6000fdfea264697066735822122013a25b98319c27a40db55f3631c97987f28084748f8435e37093bc0c0c85c4f764736f6c63430008070033a2646970667358221220898162477168b04ff32b360f0394c201ce3fff44a6e6598573b73d75e85b327364736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MapleLoanFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleLoanFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleLoanFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleLoanFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleLoanFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAPLELOANFACTORY_ABI.clone(), client)
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
                MAPLELOANFACTORY_ABI.clone(),
                MAPLELOANFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `createInstance` (0x517b657f) function"]
        pub fn create_instance(
            &self,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([81, 123, 101, 127], (arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultImplementation` (0xb39c4593) function"]
        pub fn default_implementation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([179, 156, 69, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultVersion` (0x1798d482) function"]
        pub fn default_version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 152, 212, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableUpgradePath` (0x88182912) function"]
        pub fn disable_upgrade_path(
            &self,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 24, 41, 18], (from_version, to_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableUpgradePath` (0xb28317bf) function"]
        pub fn enable_upgrade_path(
            &self,
            from_version: ethers::core::types::U256,
            to_version: ethers::core::types::U256,
            migrator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 131, 23, 191], (from_version, to_version, migrator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInstanceAddress` (0x0e6e4b25) function"]
        pub fn get_instance_address(
            &self,
            arguments: ethers::core::types::Bytes,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([14, 110, 75, 37], (arguments, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `implementationOf` (0x8636f07e) function"]
        pub fn implementation_of(
            &self,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([134, 54, 240, 126], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLoan` (0x2819cbc2) function"]
        pub fn is_loan(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([40, 25, 203, 194], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mapleGlobals` (0x3a60339a) function"]
        pub fn maple_globals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 96, 51, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migratorForPath` (0x85b8a52f) function"]
        pub fn migrator_for_path(
            &self,
            old_version: ethers::core::types::U256,
            new_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([133, 184, 165, 47], (old_version, new_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerImplementation` (0x12700bae) function"]
        pub fn register_implementation(
            &self,
            version: ethers::core::types::U256,
            implementation_address: ethers::core::types::Address,
            initializer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 112, 11, 174],
                    (version, implementation_address, initializer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDefaultVersion` (0xb4e6747f) function"]
        pub fn set_default_version(
            &self,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 230, 116, 127], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGlobals` (0xcc2e0a26) function"]
        pub fn set_globals(
            &self,
            maple_globals: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 46, 10, 38], maple_globals)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeEnabledForPath` (0xd867e0de) function"]
        pub fn upgrade_enabled_for_path(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([216, 103, 224, 222], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeInstance` (0xfe69f708) function"]
        pub fn upgrade_instance(
            &self,
            to_version: ethers::core::types::U256,
            arguments: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 105, 247, 8], (to_version, arguments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `versionOf` (0x0db3ff45) function"]
        pub fn version_of(
            &self,
            implementation: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([13, 179, 255, 69], implementation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DefaultVersionSet` event"]
        pub fn default_version_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DefaultVersionSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ImplementationRegistered` event"]
        pub fn implementation_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ImplementationRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InstanceDeployed` event"]
        pub fn instance_deployed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InstanceDeployedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InstanceUpgraded` event"]
        pub fn instance_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InstanceUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MapleGlobalsSet` event"]
        pub fn maple_globals_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MapleGlobalsSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpgradePathDisabled` event"]
        pub fn upgrade_path_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpgradePathDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpgradePathEnabled` event"]
        pub fn upgrade_path_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpgradePathEnabledFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MapleLoanFactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MapleLoanFactory<M> {
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
    #[ethevent(name = "DefaultVersionSet", abi = "DefaultVersionSet(uint256)")]
    pub struct DefaultVersionSetFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
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
        name = "ImplementationRegistered",
        abi = "ImplementationRegistered(uint256,address,address)"
    )]
    pub struct ImplementationRegisteredFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub implementation_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub initializer: ethers::core::types::Address,
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
        name = "InstanceDeployed",
        abi = "InstanceDeployed(uint256,address,bytes)"
    )]
    pub struct InstanceDeployedFilter {
        #[ethevent(indexed)]
        pub version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub instance: ethers::core::types::Address,
        pub initialization_arguments: ethers::core::types::Bytes,
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
        name = "InstanceUpgraded",
        abi = "InstanceUpgraded(address,uint256,uint256,bytes)"
    )]
    pub struct InstanceUpgradedFilter {
        #[ethevent(indexed)]
        pub instance: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
        pub migration_arguments: ethers::core::types::Bytes,
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
    #[ethevent(name = "MapleGlobalsSet", abi = "MapleGlobalsSet(address)")]
    pub struct MapleGlobalsSetFilter {
        #[ethevent(indexed)]
        pub maple_globals: ethers::core::types::Address,
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
        name = "UpgradePathDisabled",
        abi = "UpgradePathDisabled(uint256,uint256)"
    )]
    pub struct UpgradePathDisabledFilter {
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
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
        name = "UpgradePathEnabled",
        abi = "UpgradePathEnabled(uint256,uint256,address)"
    )]
    pub struct UpgradePathEnabledFilter {
        #[ethevent(indexed)]
        pub from_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_version: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub migrator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleLoanFactoryEvents {
        DefaultVersionSetFilter(DefaultVersionSetFilter),
        ImplementationRegisteredFilter(ImplementationRegisteredFilter),
        InstanceDeployedFilter(InstanceDeployedFilter),
        InstanceUpgradedFilter(InstanceUpgradedFilter),
        MapleGlobalsSetFilter(MapleGlobalsSetFilter),
        UpgradePathDisabledFilter(UpgradePathDisabledFilter),
        UpgradePathEnabledFilter(UpgradePathEnabledFilter),
    }
    impl ethers::contract::EthLogDecode for MapleLoanFactoryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DefaultVersionSetFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::DefaultVersionSetFilter(decoded));
            }
            if let Ok(decoded) = ImplementationRegisteredFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::ImplementationRegisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InstanceDeployedFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::InstanceDeployedFilter(decoded));
            }
            if let Ok(decoded) = InstanceUpgradedFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::InstanceUpgradedFilter(decoded));
            }
            if let Ok(decoded) = MapleGlobalsSetFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::MapleGlobalsSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathDisabledFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::UpgradePathDisabledFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathEnabledFilter::decode_log(log) {
                return Ok(MapleLoanFactoryEvents::UpgradePathEnabledFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MapleLoanFactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanFactoryEvents::DefaultVersionSetFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::ImplementationRegisteredFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::InstanceDeployedFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::InstanceUpgradedFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::MapleGlobalsSetFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::UpgradePathDisabledFilter(element) => element.fmt(f),
                MapleLoanFactoryEvents::UpgradePathEnabledFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `createInstance`function with signature `createInstance(bytes,bytes32)` and selector `[81, 123, 101, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createInstance", abi = "createInstance(bytes,bytes32)")]
    pub struct CreateInstanceCall {
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `defaultImplementation`function with signature `defaultImplementation()` and selector `[179, 156, 69, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "defaultImplementation", abi = "defaultImplementation()")]
    pub struct DefaultImplementationCall;
    #[doc = "Container type for all input parameters for the `defaultVersion`function with signature `defaultVersion()` and selector `[23, 152, 212, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "defaultVersion", abi = "defaultVersion()")]
    pub struct DefaultVersionCall;
    #[doc = "Container type for all input parameters for the `disableUpgradePath`function with signature `disableUpgradePath(uint256,uint256)` and selector `[136, 24, 41, 18]`"]
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
        name = "disableUpgradePath",
        abi = "disableUpgradePath(uint256,uint256)"
    )]
    pub struct DisableUpgradePathCall {
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `enableUpgradePath`function with signature `enableUpgradePath(uint256,uint256,address)` and selector `[178, 131, 23, 191]`"]
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
        name = "enableUpgradePath",
        abi = "enableUpgradePath(uint256,uint256,address)"
    )]
    pub struct EnableUpgradePathCall {
        pub from_version: ethers::core::types::U256,
        pub to_version: ethers::core::types::U256,
        pub migrator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getInstanceAddress`function with signature `getInstanceAddress(bytes,bytes32)` and selector `[14, 110, 75, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getInstanceAddress", abi = "getInstanceAddress(bytes,bytes32)")]
    pub struct GetInstanceAddressCall {
        pub arguments: ethers::core::types::Bytes,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `implementationOf`function with signature `implementationOf(uint256)` and selector `[134, 54, 240, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "implementationOf", abi = "implementationOf(uint256)")]
    pub struct ImplementationOfCall {
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isLoan`function with signature `isLoan(address)` and selector `[40, 25, 203, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isLoan", abi = "isLoan(address)")]
    pub struct IsLoanCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `mapleGlobals`function with signature `mapleGlobals()` and selector `[58, 96, 51, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mapleGlobals", abi = "mapleGlobals()")]
    pub struct MapleGlobalsCall;
    #[doc = "Container type for all input parameters for the `migratorForPath`function with signature `migratorForPath(uint256,uint256)` and selector `[133, 184, 165, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "migratorForPath", abi = "migratorForPath(uint256,uint256)")]
    pub struct MigratorForPathCall {
        pub old_version: ethers::core::types::U256,
        pub new_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `registerImplementation`function with signature `registerImplementation(uint256,address,address)` and selector `[18, 112, 11, 174]`"]
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
        name = "registerImplementation",
        abi = "registerImplementation(uint256,address,address)"
    )]
    pub struct RegisterImplementationCall {
        pub version: ethers::core::types::U256,
        pub implementation_address: ethers::core::types::Address,
        pub initializer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDefaultVersion`function with signature `setDefaultVersion(uint256)` and selector `[180, 230, 116, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDefaultVersion", abi = "setDefaultVersion(uint256)")]
    pub struct SetDefaultVersionCall {
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setGlobals`function with signature `setGlobals(address)` and selector `[204, 46, 10, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGlobals", abi = "setGlobals(address)")]
    pub struct SetGlobalsCall {
        pub maple_globals: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `upgradeEnabledForPath`function with signature `upgradeEnabledForPath(uint256,uint256)` and selector `[216, 103, 224, 222]`"]
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
        name = "upgradeEnabledForPath",
        abi = "upgradeEnabledForPath(uint256,uint256)"
    )]
    pub struct UpgradeEnabledForPathCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `upgradeInstance`function with signature `upgradeInstance(uint256,bytes)` and selector `[254, 105, 247, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upgradeInstance", abi = "upgradeInstance(uint256,bytes)")]
    pub struct UpgradeInstanceCall {
        pub to_version: ethers::core::types::U256,
        pub arguments: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `versionOf`function with signature `versionOf(address)` and selector `[13, 179, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "versionOf", abi = "versionOf(address)")]
    pub struct VersionOfCall {
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MapleLoanFactoryCalls {
        CreateInstance(CreateInstanceCall),
        DefaultImplementation(DefaultImplementationCall),
        DefaultVersion(DefaultVersionCall),
        DisableUpgradePath(DisableUpgradePathCall),
        EnableUpgradePath(EnableUpgradePathCall),
        GetInstanceAddress(GetInstanceAddressCall),
        ImplementationOf(ImplementationOfCall),
        IsLoan(IsLoanCall),
        MapleGlobals(MapleGlobalsCall),
        MigratorForPath(MigratorForPathCall),
        RegisterImplementation(RegisterImplementationCall),
        SetDefaultVersion(SetDefaultVersionCall),
        SetGlobals(SetGlobalsCall),
        UpgradeEnabledForPath(UpgradeEnabledForPathCall),
        UpgradeInstance(UpgradeInstanceCall),
        VersionOf(VersionOfCall),
    }
    impl ethers::core::abi::AbiDecode for MapleLoanFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::CreateInstance(decoded));
            }
            if let Ok(decoded) =
                <DefaultImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::DefaultImplementation(decoded));
            }
            if let Ok(decoded) =
                <DefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::DefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <DisableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::DisableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <EnableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::EnableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <GetInstanceAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::GetInstanceAddress(decoded));
            }
            if let Ok(decoded) =
                <ImplementationOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::ImplementationOf(decoded));
            }
            if let Ok(decoded) = <IsLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::IsLoan(decoded));
            }
            if let Ok(decoded) =
                <MapleGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::MapleGlobals(decoded));
            }
            if let Ok(decoded) =
                <MigratorForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::MigratorForPath(decoded));
            }
            if let Ok(decoded) =
                <RegisterImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::RegisterImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::SetDefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::SetGlobals(decoded));
            }
            if let Ok(decoded) =
                <UpgradeEnabledForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::UpgradeEnabledForPath(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::UpgradeInstance(decoded));
            }
            if let Ok(decoded) =
                <VersionOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleLoanFactoryCalls::VersionOf(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleLoanFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleLoanFactoryCalls::CreateInstance(element) => element.encode(),
                MapleLoanFactoryCalls::DefaultImplementation(element) => element.encode(),
                MapleLoanFactoryCalls::DefaultVersion(element) => element.encode(),
                MapleLoanFactoryCalls::DisableUpgradePath(element) => element.encode(),
                MapleLoanFactoryCalls::EnableUpgradePath(element) => element.encode(),
                MapleLoanFactoryCalls::GetInstanceAddress(element) => element.encode(),
                MapleLoanFactoryCalls::ImplementationOf(element) => element.encode(),
                MapleLoanFactoryCalls::IsLoan(element) => element.encode(),
                MapleLoanFactoryCalls::MapleGlobals(element) => element.encode(),
                MapleLoanFactoryCalls::MigratorForPath(element) => element.encode(),
                MapleLoanFactoryCalls::RegisterImplementation(element) => element.encode(),
                MapleLoanFactoryCalls::SetDefaultVersion(element) => element.encode(),
                MapleLoanFactoryCalls::SetGlobals(element) => element.encode(),
                MapleLoanFactoryCalls::UpgradeEnabledForPath(element) => element.encode(),
                MapleLoanFactoryCalls::UpgradeInstance(element) => element.encode(),
                MapleLoanFactoryCalls::VersionOf(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleLoanFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleLoanFactoryCalls::CreateInstance(element) => element.fmt(f),
                MapleLoanFactoryCalls::DefaultImplementation(element) => element.fmt(f),
                MapleLoanFactoryCalls::DefaultVersion(element) => element.fmt(f),
                MapleLoanFactoryCalls::DisableUpgradePath(element) => element.fmt(f),
                MapleLoanFactoryCalls::EnableUpgradePath(element) => element.fmt(f),
                MapleLoanFactoryCalls::GetInstanceAddress(element) => element.fmt(f),
                MapleLoanFactoryCalls::ImplementationOf(element) => element.fmt(f),
                MapleLoanFactoryCalls::IsLoan(element) => element.fmt(f),
                MapleLoanFactoryCalls::MapleGlobals(element) => element.fmt(f),
                MapleLoanFactoryCalls::MigratorForPath(element) => element.fmt(f),
                MapleLoanFactoryCalls::RegisterImplementation(element) => element.fmt(f),
                MapleLoanFactoryCalls::SetDefaultVersion(element) => element.fmt(f),
                MapleLoanFactoryCalls::SetGlobals(element) => element.fmt(f),
                MapleLoanFactoryCalls::UpgradeEnabledForPath(element) => element.fmt(f),
                MapleLoanFactoryCalls::UpgradeInstance(element) => element.fmt(f),
                MapleLoanFactoryCalls::VersionOf(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateInstanceCall> for MapleLoanFactoryCalls {
        fn from(var: CreateInstanceCall) -> Self {
            MapleLoanFactoryCalls::CreateInstance(var)
        }
    }
    impl ::std::convert::From<DefaultImplementationCall> for MapleLoanFactoryCalls {
        fn from(var: DefaultImplementationCall) -> Self {
            MapleLoanFactoryCalls::DefaultImplementation(var)
        }
    }
    impl ::std::convert::From<DefaultVersionCall> for MapleLoanFactoryCalls {
        fn from(var: DefaultVersionCall) -> Self {
            MapleLoanFactoryCalls::DefaultVersion(var)
        }
    }
    impl ::std::convert::From<DisableUpgradePathCall> for MapleLoanFactoryCalls {
        fn from(var: DisableUpgradePathCall) -> Self {
            MapleLoanFactoryCalls::DisableUpgradePath(var)
        }
    }
    impl ::std::convert::From<EnableUpgradePathCall> for MapleLoanFactoryCalls {
        fn from(var: EnableUpgradePathCall) -> Self {
            MapleLoanFactoryCalls::EnableUpgradePath(var)
        }
    }
    impl ::std::convert::From<GetInstanceAddressCall> for MapleLoanFactoryCalls {
        fn from(var: GetInstanceAddressCall) -> Self {
            MapleLoanFactoryCalls::GetInstanceAddress(var)
        }
    }
    impl ::std::convert::From<ImplementationOfCall> for MapleLoanFactoryCalls {
        fn from(var: ImplementationOfCall) -> Self {
            MapleLoanFactoryCalls::ImplementationOf(var)
        }
    }
    impl ::std::convert::From<IsLoanCall> for MapleLoanFactoryCalls {
        fn from(var: IsLoanCall) -> Self {
            MapleLoanFactoryCalls::IsLoan(var)
        }
    }
    impl ::std::convert::From<MapleGlobalsCall> for MapleLoanFactoryCalls {
        fn from(var: MapleGlobalsCall) -> Self {
            MapleLoanFactoryCalls::MapleGlobals(var)
        }
    }
    impl ::std::convert::From<MigratorForPathCall> for MapleLoanFactoryCalls {
        fn from(var: MigratorForPathCall) -> Self {
            MapleLoanFactoryCalls::MigratorForPath(var)
        }
    }
    impl ::std::convert::From<RegisterImplementationCall> for MapleLoanFactoryCalls {
        fn from(var: RegisterImplementationCall) -> Self {
            MapleLoanFactoryCalls::RegisterImplementation(var)
        }
    }
    impl ::std::convert::From<SetDefaultVersionCall> for MapleLoanFactoryCalls {
        fn from(var: SetDefaultVersionCall) -> Self {
            MapleLoanFactoryCalls::SetDefaultVersion(var)
        }
    }
    impl ::std::convert::From<SetGlobalsCall> for MapleLoanFactoryCalls {
        fn from(var: SetGlobalsCall) -> Self {
            MapleLoanFactoryCalls::SetGlobals(var)
        }
    }
    impl ::std::convert::From<UpgradeEnabledForPathCall> for MapleLoanFactoryCalls {
        fn from(var: UpgradeEnabledForPathCall) -> Self {
            MapleLoanFactoryCalls::UpgradeEnabledForPath(var)
        }
    }
    impl ::std::convert::From<UpgradeInstanceCall> for MapleLoanFactoryCalls {
        fn from(var: UpgradeInstanceCall) -> Self {
            MapleLoanFactoryCalls::UpgradeInstance(var)
        }
    }
    impl ::std::convert::From<VersionOfCall> for MapleLoanFactoryCalls {
        fn from(var: VersionOfCall) -> Self {
            MapleLoanFactoryCalls::VersionOf(var)
        }
    }
}
