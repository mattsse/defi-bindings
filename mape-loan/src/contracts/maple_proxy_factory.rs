pub use mapleproxyfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod mapleproxyfactory_mod {
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
    #[doc = "MapleProxyFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MAPLEPROXYFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DefaultVersionSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ImplementationRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"initializationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"migrationArguments_\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InstanceUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MapleGlobalsSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UpgradePathEnabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createInstance\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instance_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultImplementation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"defaultImplementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableUpgradePath\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInstanceAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"instanceAddress_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementationOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mapleGlobals\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newVersion_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"migratorForPath\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"migrator_\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initializer_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerImplementation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDefaultVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"mapleGlobals_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGlobals\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"upgradeEnabledForPath\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"toVersion_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arguments_\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeInstance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"implementation_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"versionOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"version_\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MAPLEPROXYFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162001c3e38038062001c3e83398101604081905262000034916200012a565b600380546001600160a01b0319166001600160a01b0383169081179091556040805163030d028960e21b8152905160009291630c340a24916004808301926020929190829003018186803b1580156200008c57600080fd5b505afa158015620000a1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000c791906200012a565b6001600160a01b03161415620001235760405162461bcd60e51b815260206004820152601560248201527f4d50463a433a494e56414c49445f474c4f42414c530000000000000000000000604482015260640160405180910390fd5b506200015c565b6000602082840312156200013d57600080fd5b81516001600160a01b03811681146200015557600080fd5b9392505050565b611ad2806200016c6000396000f3fe60806040523480156200001157600080fd5b5060043610620001095760003560e01c80638636f07e11620000a3578063b4e6747f116200006e578063b4e6747f146200025d578063cc2e0a261462000274578063d867e0de146200028b578063fe69f70814620002cd57600080fd5b80638636f07e14620001e257806388182912146200020e578063b28317bf1462000225578063b39c4593146200023c57600080fd5b80631798d48211620000e45780631798d48214620001965780633a60339a14620001a0578063517b657f14620001b457806385b8a52f14620001cb57600080fd5b80630db3ff45146200010e5780630e6e4b25146200014d57806312700bae146200017d575b600080fd5b6200013a6200011f3660046200157a565b6001600160a01b031660009081526001602052604090205490565b6040519081526020015b60405180910390f35b620001646200015e366004620015ba565b620002e4565b6040516001600160a01b03909116815260200162000144565b620001946200018e36600462001624565b62000327565b005b6200013a60045481565b60035462000164906001600160a01b031681565b62000164620001c5366004620015ba565b62000530565b62000164620001dc366004620016bb565b62000633565b62000164620001f33660046200160a565b6000908152602081905260409020546001600160a01b031690565b620001946200021f366004620016bb565b6200065d565b6200019462000236366004620016de565b62000809565b6004546000908152602081905260409020546001600160a01b031662000164565b620001946200026e3660046200160a565b620009c3565b62000194620002853660046200157a565b62000b25565b620002bc6200029c366004620016bb565b600560209081526000928352604080842090915290825290205460ff1681565b604051901515815260200162000144565b62000194620002de3660046200166b565b62000cfe565b60006200031d84848460405160200162000301939291906200170f565b6040516020818303038152906040528051906020012062000ec1565b90505b9392505050565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b1580156200037657600080fd5b505afa1580156200038b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003b191906200159a565b6001600160a01b0316336001600160a01b031614620003ed5760405162461bcd60e51b8152600401620003e490620017e5565b60405180910390fd5b82620004355760405162461bcd60e51b815260206004820152601660248201527526a8231d29249d24a72b20a624a22fab22a929a4a7a760511b6044820152606401620003e4565b806001600160a01b0316826001600160a01b0316847fe69962526b7f07862bf85663f861564361295f9601236fbbe056591eb1b63f3b60405160405180910390a462000482838362000f9c565b620004d05760405162461bcd60e51b815260206004820152601e60248201527f4d50463a52493a4641494c5f464f525f494d504c454d454e544154494f4e00006044820152606401620003e4565b620004dd83848362001048565b6200052b5760405162461bcd60e51b815260206004820152601860248201527f4d50463a52493a4641494c5f464f525f4d49475241544f5200000000000000006044820152606401620003e4565b505050565b600080620005a085858080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050604051620005849250899150889088906020016200170f565b60405160208183030381529060405280519060200120620010d2565b9250905080620005e35760405162461bcd60e51b815260206004820152600d60248201526c1354118e90d24e919052531151609a1b6044820152606401620003e4565b816001600160a01b03166004547f8919387fe006fdc29a3dfcc183071d7974bc03747fedb0019630f1e13f85cc64878760405162000623929190620017b6565b60405180910390a3509392505050565b60008281526002602090815260408083208484529091529020546001600160a01b03165b92915050565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b158015620006ac57600080fd5b505afa158015620006c1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620006e791906200159a565b6001600160a01b0316336001600160a01b0316146200071a5760405162461bcd60e51b8152600401620003e490620017e5565b808214156200076c5760405162461bcd60e51b815260206004820152601f60248201527f4d50463a4455503a4f56455257524954494e475f494e495449414c495a4552006044820152606401620003e4565b6200077a8282600062001048565b620007b95760405162461bcd60e51b815260206004820152600e60248201526d1354118e9115540e91905253115160921b6044820152606401620003e4565b604051819083907fa46f1addc2236b7d93ed2a8a507f1c47cc92656d2b6f82bf0876da9e964b9e5e90600090a360009182526005602090815260408084209284529190529020805460ff19169055565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b1580156200085857600080fd5b505afa1580156200086d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200089391906200159a565b6001600160a01b0316336001600160a01b031614620008c65760405162461bcd60e51b8152600401620003e490620017e5565b81831415620009185760405162461bcd60e51b815260206004820152601f60248201527f4d50463a4555503a4f56455257524954494e475f494e495449414c495a4552006044820152606401620003e4565b6200092583838362001048565b620009645760405162461bcd60e51b815260206004820152600e60248201526d1354118e9155540e91905253115160921b6044820152606401620003e4565b806001600160a01b031682847f549a41e54b51dcc3f29b51bb02a8adcc4ec5ae26604608e41bde60311dcef10660405160405180910390a45060009182526005602090815260408084209284529190529020805460ff19166001179055565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b15801562000a1257600080fd5b505afa15801562000a27573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000a4d91906200159a565b6001600160a01b0316336001600160a01b03161462000a805760405162461bcd60e51b8152600401620003e490620017e5565b80158062000aa457506000818152602081905260409020546001600160a01b031615155b62000af25760405162461bcd60e51b815260206004820152601760248201527f4d50463a5344563a494e56414c49445f56455253494f4e0000000000000000006044820152606401620003e4565b600481905560405181907fddb2013cf7f102d15447c4c1e94cf56823455f02eb244d0c3b2ef6516338934690600090a250565b600360009054906101000a90046001600160a01b03166001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b15801562000b7457600080fd5b505afa15801562000b89573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000baf91906200159a565b6001600160a01b0316336001600160a01b03161462000be25760405162461bcd60e51b8152600401620003e490620017e5565b60006001600160a01b0316816001600160a01b0316630c340a246040518163ffffffff1660e01b815260040160206040518083038186803b15801562000c2757600080fd5b505afa15801562000c3c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000c6291906200159a565b6001600160a01b0316141562000cb45760405162461bcd60e51b81526020600482015260166024820152754d50463a53473a494e56414c49445f474c4f42414c5360501b6044820152606401620003e4565b600380546001600160a01b0319166001600160a01b0383169081179091556040517f9074839b84a74138be159cb7813a72c2a44c35fe8c53da66a16da385d348c5f190600090a250565b600060016000336001600160a01b0316635c60da1b6040518163ffffffff1660e01b815260040160206040518083038186803b15801562000d3e57600080fd5b505afa15801562000d53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000d7991906200159a565b6001600160a01b03168152602080820192909252604090810160009081205480825260058452828220888352909352205490915060ff1662000df35760405162461bcd60e51b81526020600482015260126024820152711354118e95524e9393d517d0531313d5d15160721b6044820152606401620003e4565b8381336001600160a01b03167ffbb4f36b70dba8ecedc8b38361f44f1b0c61e04ec4e0ccf620649dc558573f5f868660405162000e32929190620017b6565b60405180910390a462000e7d338585858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506200117092505050565b62000ebb5760405162461bcd60e51b815260206004820152600d60248201526c1354118e95524e919052531151609a1b6044820152606401620003e4565b50505050565b600060ff60f81b30836040518060200162000edc9062001520565b601f1982820381018352601f90910116604081815230602083015260009082015260600160408051601f198184030181529082905262000f2092916020016200173f565b6040516020818303038152906040528051906020012060405160200162000f7e94939291906001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60408051601f19818403018152919052805160209091012092915050565b600082158062000fc257506000838152602081905260409020546001600160a01b031615155b8062000fe557506001600160a01b03821660009081526001602052604090205415155b8062000ff957506001600160a01b0382163b155b15620010085750600062000657565b5060008281526020818152604080832080546001600160a01b0319166001600160a01b039590951694851790559282526001908190529190209190915590565b600083158062001056575082155b15620010655750600062000320565b6001600160a01b038216158015906200108657506001600160a01b0382163b155b15620010955750600062000320565b506000928352600260209081526040808520938552929052912080546001600160a01b0319166001600160a01b0392909216919091179055600190565b60008082306000604051620010e79062001520565b6001600160a01b039283168152911660208201526040018190604051809103906000f59050801580156200111f573d6000803e3d6000fd5b50905060006200112f8262001388565b6001600160a01b038116600090815260016020526040902054909250905080158015906200116557506200116583828862001438565b935050509250929050565b60006001600160a01b0384163b6200118b5750600062000320565b6000838152602081905260409020546001600160a01b031680620011b457600091505062000320565b6000620011c18662001388565b909350905082620011d85760009250505062000320565b604080516001600160a01b0384811660248084019190915283518084039091018152604490920183526020820180516001600160e01b0316636bc26a1360e11b1790529151918816916200122d919062001721565b6000604051808303816000865af19150503d80600081146200126c576040519150601f19603f3d011682016040523d82523d6000602084013e62001271565b606091505b50508093505082620012895760009250505062000320565b6001600160a01b038082166000908152600160209081526040808320548352600282528083208984529091529020541680620012cf576000855114935050505062000320565b866001600160a01b031663c3fbb6fd60e01b8287604051602401620012f692919062001772565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905162001336919062001721565b6000604051808303816000865af19150503d806000811462001375576040519150601f19603f3d011682016040523d82523d6000602084013e6200137a565b606091505b509098975050505050505050565b60408051600481526024810182526020810180516001600160e01b0316635c60da1b60e01b179052905160009182916060916001600160a01b03861691620013d1919062001721565b600060405180830381855afa9150503d80600081146200140e576040519150601f19603f3d011682016040523d82523d6000602084013e62001413565b606091505b50805191945091506200143090820160209081019083016200159a565b915050915091565b60008281526002602090815260408083209091528120546001600160a01b0316806200146957505080511562000320565b846001600160a01b031663c3fbb6fd60e01b82856040516024016200149092919062001772565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b0319909416939093179092529051620014d0919062001721565b6000604051808303816000865af19150503d80600081146200150f576040519150601f19603f3d011682016040523d82523d6000602084013e62001514565b606091505b50909695505050505050565b610245806200185883390190565b60008083601f8401126200154157600080fd5b50813567ffffffffffffffff8111156200155a57600080fd5b6020830191508360208285010111156200157357600080fd5b9250929050565b6000602082840312156200158d57600080fd5b813562000320816200183e565b600060208284031215620015ad57600080fd5b815162000320816200183e565b600080600060408486031215620015d057600080fd5b833567ffffffffffffffff811115620015e857600080fd5b620015f6868287016200152e565b909790965060209590950135949350505050565b6000602082840312156200161d57600080fd5b5035919050565b6000806000606084860312156200163a57600080fd5b8335925060208401356200164e816200183e565b9150604084013562001660816200183e565b809150509250925092565b6000806000604084860312156200168157600080fd5b83359250602084013567ffffffffffffffff811115620016a057600080fd5b620016ae868287016200152e565b9497909650939450505050565b60008060408385031215620016cf57600080fd5b50508035926020909101359150565b600080600060608486031215620016f457600080fd5b8335925060208401359150604084013562001660816200183e565b82848237909101908152602001919050565b60008251620017358184602087016200180f565b9190910192915050565b60008351620017538184602088016200180f565b835190830190620017698183602088016200180f565b01949350505050565b60018060a01b03831681526040602082015260008251806040840152620017a18160608501602087016200180f565b601f01601f1916919091016060019392505050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b60208082526010908201526f26a8231d2727aa2fa3a7ab22a92727a960811b604082015260600190565b60005b838110156200182c57818101518382015260200162001812565b8381111562000ebb5750506000910152565b6001600160a01b03811681146200185457600080fd5b5056fe608060405234801561001057600080fd5b5060405161024538038061024583398101604081905261002f91610169565b6001600160a01b0382167f7a45a402e4cb6e08ebc196f20f66d5d30e67285a2a8aa80503fa409e727a4af15560006001600160a01b0382161561007257816100e3565b826001600160a01b031663b39c45936040518163ffffffff1660e01b815260040160206040518083038186803b1580156100ab57600080fd5b505afa1580156100bf573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e39190610147565b90506001600160a01b0381166100f857600080fd5b6001600160a01b03167f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc555061019c9050565b80516001600160a01b038116811461014257600080fd5b919050565b60006020828403121561015957600080fd5b6101628261012b565b9392505050565b6000806040838503121561017c57600080fd5b6101858361012b565b91506101936020840161012b565b90509250929050565b609b806101aa6000396000f3fe60806040526000602d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5490565b90506001600160a01b0381163b604257600080fd5b3660008037600080366000845af43d6000803e8080156060573d6000f35b3d6000fdfea264697066735822122013a25b98319c27a40db55f3631c97987f28084748f8435e37093bc0c0c85c4f764736f6c63430008070033a2646970667358221220594b405d1869d4bc935afe53508eaf9b06b54be9a973d1565847cbdc243b572664736f6c63430008070033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct MapleProxyFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MapleProxyFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MapleProxyFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MapleProxyFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MapleProxyFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MAPLEPROXYFACTORY_ABI.clone(), client)
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
                MAPLEPROXYFACTORY_ABI.clone(),
                MAPLEPROXYFACTORY_BYTECODE.clone().into(),
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, MapleProxyFactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MapleProxyFactory<M>
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
    pub enum MapleProxyFactoryEvents {
        DefaultVersionSetFilter(DefaultVersionSetFilter),
        ImplementationRegisteredFilter(ImplementationRegisteredFilter),
        InstanceDeployedFilter(InstanceDeployedFilter),
        InstanceUpgradedFilter(InstanceUpgradedFilter),
        MapleGlobalsSetFilter(MapleGlobalsSetFilter),
        UpgradePathDisabledFilter(UpgradePathDisabledFilter),
        UpgradePathEnabledFilter(UpgradePathEnabledFilter),
    }
    impl ethers::contract::EthLogDecode for MapleProxyFactoryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DefaultVersionSetFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::DefaultVersionSetFilter(decoded));
            }
            if let Ok(decoded) = ImplementationRegisteredFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::ImplementationRegisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InstanceDeployedFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::InstanceDeployedFilter(decoded));
            }
            if let Ok(decoded) = InstanceUpgradedFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::InstanceUpgradedFilter(decoded));
            }
            if let Ok(decoded) = MapleGlobalsSetFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::MapleGlobalsSetFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathDisabledFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::UpgradePathDisabledFilter(decoded));
            }
            if let Ok(decoded) = UpgradePathEnabledFilter::decode_log(log) {
                return Ok(MapleProxyFactoryEvents::UpgradePathEnabledFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MapleProxyFactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleProxyFactoryEvents::DefaultVersionSetFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::ImplementationRegisteredFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::InstanceDeployedFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::InstanceUpgradedFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::MapleGlobalsSetFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::UpgradePathDisabledFilter(element) => element.fmt(f),
                MapleProxyFactoryEvents::UpgradePathEnabledFilter(element) => element.fmt(f),
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
    pub enum MapleProxyFactoryCalls {
        CreateInstance(CreateInstanceCall),
        DefaultImplementation(DefaultImplementationCall),
        DefaultVersion(DefaultVersionCall),
        DisableUpgradePath(DisableUpgradePathCall),
        EnableUpgradePath(EnableUpgradePathCall),
        GetInstanceAddress(GetInstanceAddressCall),
        ImplementationOf(ImplementationOfCall),
        MapleGlobals(MapleGlobalsCall),
        MigratorForPath(MigratorForPathCall),
        RegisterImplementation(RegisterImplementationCall),
        SetDefaultVersion(SetDefaultVersionCall),
        SetGlobals(SetGlobalsCall),
        UpgradeEnabledForPath(UpgradeEnabledForPathCall),
        UpgradeInstance(UpgradeInstanceCall),
        VersionOf(VersionOfCall),
    }
    impl ethers::core::abi::AbiDecode for MapleProxyFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::CreateInstance(decoded));
            }
            if let Ok(decoded) =
                <DefaultImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::DefaultImplementation(decoded));
            }
            if let Ok(decoded) =
                <DefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::DefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <DisableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::DisableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <EnableUpgradePathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::EnableUpgradePath(decoded));
            }
            if let Ok(decoded) =
                <GetInstanceAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::GetInstanceAddress(decoded));
            }
            if let Ok(decoded) =
                <ImplementationOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::ImplementationOf(decoded));
            }
            if let Ok(decoded) =
                <MapleGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::MapleGlobals(decoded));
            }
            if let Ok(decoded) =
                <MigratorForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::MigratorForPath(decoded));
            }
            if let Ok(decoded) =
                <RegisterImplementationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::RegisterImplementation(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::SetDefaultVersion(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::SetGlobals(decoded));
            }
            if let Ok(decoded) =
                <UpgradeEnabledForPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::UpgradeEnabledForPath(decoded));
            }
            if let Ok(decoded) =
                <UpgradeInstanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::UpgradeInstance(decoded));
            }
            if let Ok(decoded) =
                <VersionOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MapleProxyFactoryCalls::VersionOf(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MapleProxyFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MapleProxyFactoryCalls::CreateInstance(element) => element.encode(),
                MapleProxyFactoryCalls::DefaultImplementation(element) => element.encode(),
                MapleProxyFactoryCalls::DefaultVersion(element) => element.encode(),
                MapleProxyFactoryCalls::DisableUpgradePath(element) => element.encode(),
                MapleProxyFactoryCalls::EnableUpgradePath(element) => element.encode(),
                MapleProxyFactoryCalls::GetInstanceAddress(element) => element.encode(),
                MapleProxyFactoryCalls::ImplementationOf(element) => element.encode(),
                MapleProxyFactoryCalls::MapleGlobals(element) => element.encode(),
                MapleProxyFactoryCalls::MigratorForPath(element) => element.encode(),
                MapleProxyFactoryCalls::RegisterImplementation(element) => element.encode(),
                MapleProxyFactoryCalls::SetDefaultVersion(element) => element.encode(),
                MapleProxyFactoryCalls::SetGlobals(element) => element.encode(),
                MapleProxyFactoryCalls::UpgradeEnabledForPath(element) => element.encode(),
                MapleProxyFactoryCalls::UpgradeInstance(element) => element.encode(),
                MapleProxyFactoryCalls::VersionOf(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MapleProxyFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MapleProxyFactoryCalls::CreateInstance(element) => element.fmt(f),
                MapleProxyFactoryCalls::DefaultImplementation(element) => element.fmt(f),
                MapleProxyFactoryCalls::DefaultVersion(element) => element.fmt(f),
                MapleProxyFactoryCalls::DisableUpgradePath(element) => element.fmt(f),
                MapleProxyFactoryCalls::EnableUpgradePath(element) => element.fmt(f),
                MapleProxyFactoryCalls::GetInstanceAddress(element) => element.fmt(f),
                MapleProxyFactoryCalls::ImplementationOf(element) => element.fmt(f),
                MapleProxyFactoryCalls::MapleGlobals(element) => element.fmt(f),
                MapleProxyFactoryCalls::MigratorForPath(element) => element.fmt(f),
                MapleProxyFactoryCalls::RegisterImplementation(element) => element.fmt(f),
                MapleProxyFactoryCalls::SetDefaultVersion(element) => element.fmt(f),
                MapleProxyFactoryCalls::SetGlobals(element) => element.fmt(f),
                MapleProxyFactoryCalls::UpgradeEnabledForPath(element) => element.fmt(f),
                MapleProxyFactoryCalls::UpgradeInstance(element) => element.fmt(f),
                MapleProxyFactoryCalls::VersionOf(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateInstanceCall> for MapleProxyFactoryCalls {
        fn from(var: CreateInstanceCall) -> Self {
            MapleProxyFactoryCalls::CreateInstance(var)
        }
    }
    impl ::std::convert::From<DefaultImplementationCall> for MapleProxyFactoryCalls {
        fn from(var: DefaultImplementationCall) -> Self {
            MapleProxyFactoryCalls::DefaultImplementation(var)
        }
    }
    impl ::std::convert::From<DefaultVersionCall> for MapleProxyFactoryCalls {
        fn from(var: DefaultVersionCall) -> Self {
            MapleProxyFactoryCalls::DefaultVersion(var)
        }
    }
    impl ::std::convert::From<DisableUpgradePathCall> for MapleProxyFactoryCalls {
        fn from(var: DisableUpgradePathCall) -> Self {
            MapleProxyFactoryCalls::DisableUpgradePath(var)
        }
    }
    impl ::std::convert::From<EnableUpgradePathCall> for MapleProxyFactoryCalls {
        fn from(var: EnableUpgradePathCall) -> Self {
            MapleProxyFactoryCalls::EnableUpgradePath(var)
        }
    }
    impl ::std::convert::From<GetInstanceAddressCall> for MapleProxyFactoryCalls {
        fn from(var: GetInstanceAddressCall) -> Self {
            MapleProxyFactoryCalls::GetInstanceAddress(var)
        }
    }
    impl ::std::convert::From<ImplementationOfCall> for MapleProxyFactoryCalls {
        fn from(var: ImplementationOfCall) -> Self {
            MapleProxyFactoryCalls::ImplementationOf(var)
        }
    }
    impl ::std::convert::From<MapleGlobalsCall> for MapleProxyFactoryCalls {
        fn from(var: MapleGlobalsCall) -> Self {
            MapleProxyFactoryCalls::MapleGlobals(var)
        }
    }
    impl ::std::convert::From<MigratorForPathCall> for MapleProxyFactoryCalls {
        fn from(var: MigratorForPathCall) -> Self {
            MapleProxyFactoryCalls::MigratorForPath(var)
        }
    }
    impl ::std::convert::From<RegisterImplementationCall> for MapleProxyFactoryCalls {
        fn from(var: RegisterImplementationCall) -> Self {
            MapleProxyFactoryCalls::RegisterImplementation(var)
        }
    }
    impl ::std::convert::From<SetDefaultVersionCall> for MapleProxyFactoryCalls {
        fn from(var: SetDefaultVersionCall) -> Self {
            MapleProxyFactoryCalls::SetDefaultVersion(var)
        }
    }
    impl ::std::convert::From<SetGlobalsCall> for MapleProxyFactoryCalls {
        fn from(var: SetGlobalsCall) -> Self {
            MapleProxyFactoryCalls::SetGlobals(var)
        }
    }
    impl ::std::convert::From<UpgradeEnabledForPathCall> for MapleProxyFactoryCalls {
        fn from(var: UpgradeEnabledForPathCall) -> Self {
            MapleProxyFactoryCalls::UpgradeEnabledForPath(var)
        }
    }
    impl ::std::convert::From<UpgradeInstanceCall> for MapleProxyFactoryCalls {
        fn from(var: UpgradeInstanceCall) -> Self {
            MapleProxyFactoryCalls::UpgradeInstance(var)
        }
    }
    impl ::std::convert::From<VersionOfCall> for MapleProxyFactoryCalls {
        fn from(var: VersionOfCall) -> Self {
            MapleProxyFactoryCalls::VersionOf(var)
        }
    }
}
