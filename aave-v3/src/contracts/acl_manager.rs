pub use acl_manager::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod acl_manager {
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
    #[doc = "ACLManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ACLMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ASSET_LISTING_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BRIDGE_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EMERGENCY_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FLASH_BORROWER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RISK_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetListingAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addEmergencyAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addFlashBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPoolAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addRiskAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAssetListingAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBridge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isFlashBorrower\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPoolAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isRiskAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeAssetListingAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeEmergencyAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeFlashBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removePoolAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRiskAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"adminRole\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoleAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ACLMANAGER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a06040523480156200001157600080fd5b50604051620010b5380380620010b58339810160408190526200003491620001e3565b806001600160a01b03166080816001600160a01b0316815250506000816001600160a01b0316630e67178c6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200008f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000b59190620001e3565b604080518082019091526002815261373560f01b60208201529091506001600160a01b038216620001045760405162461bcd60e51b8152600401620000fb91906200020a565b60405180910390fd5b50620001126000826200011a565b505062000262565b6200012682826200012a565b5050565b6000828152602081815260408083206001600160a01b038516845290915290205460ff1662000126576000828152602081815260408083206001600160a01b03851684529091529020805460ff19166001179055620001863390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b6001600160a01b0381168114620001e057600080fd5b50565b600060208284031215620001f657600080fd5b81516200020381620001ca565b9392505050565b600060208083528351808285015260005b8181101562000239578581018301518582016040015282016200021b565b818111156200024c576000604083870101525b50601f01601f1916929092016040019392505050565b608051610e376200027e60003960006102420152610e376000f3fe608060405234801561001057600080fd5b50600436106101fb5760003560e01c8063674b5e4d1161011a5780639a2b96f7116100ad578063b5bfddea1161007c578063b5bfddea14610472578063b8f6dba714610487578063d547741f1461049c578063f83695cb146104af578063fa50f297146104c257600080fd5b80639a2b96f7146104315780639ac9d80b14610444578063a217fddf14610457578063a21bce151461045f57600080fd5b80637a9a93f4116100e95780637a9a93f4146103e55780637be53ca1146103f857806391d148541461040b5780639712fdf81461041e57600080fd5b8063674b5e4d146103955780636e76fc8f146103a8578063726600ce146103bd57806378bb0a43146103d057600080fd5b80632500f2b6116101925780633c5a08e5116101615780633c5a08e5146103455780634f16b425146103585780635577b7a91461036d5780635b9a94e41461038257600080fd5b80632500f2b6146102f9578063253cf9801461030c5780632f2ff15d1461031f57806336568abe1461033257600080fd5b8063179efb09116101ce578063179efb091461028f5780631e4e0091146102a257806322650caf146102b5578063248a9ca3146102c857600080fd5b806301ffc9a71461020057806304df017d146102285780630542975c1461023d57806313ee32e01461027c575b600080fd5b61021361020e366004610b11565b6104d5565b60405190151581526020015b60405180910390f35b61023b610236366004610b57565b61050c565b005b6102647f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161021f565b61021361028a366004610b57565b610527565b61023b61029d366004610b57565b610541565b61023b6102b0366004610b72565b610559565b61023b6102c3366004610b57565b610574565b6102eb6102d6366004610b94565b60009081526020819052604090206001015490565b60405190815260200161021f565b610213610307366004610b57565b61058c565b61023b61031a366004610b57565b6105a6565b61023b61032d366004610bad565b6105be565b61023b610340366004610bad565b6105e4565b61023b610353366004610b57565b610667565b6102eb600080516020610d6283398151915281565b6102eb600080516020610de283398151915281565b61023b610390366004610b57565b61067f565b6102136103a3366004610b57565b610697565b6102eb600080516020610dc283398151915281565b6102136103cb366004610b57565b6106b1565b6102eb600080516020610d8283398151915281565b61023b6103f3366004610b57565b6106cb565b610213610406366004610b57565b6106e3565b610213610419366004610bad565b6106f9565b61023b61042c366004610b57565b610722565b61023b61043f366004610b57565b61073a565b61023b610452366004610b57565b610752565b6102eb600081565b61023b61046d366004610b57565b61076a565b6102eb600080516020610da283398151915281565b6102eb600080516020610d4283398151915281565b61023b6104aa366004610bad565b61077e565b61023b6104bd366004610b57565b6107a4565b6102136104d0366004610b57565b6107bc565b60006001600160e01b03198216637965db0b60e01b148061050657506301ffc9a760e01b6001600160e01b03198316145b92915050565b610524600080516020610da28339815191528261077e565b50565b6000610506600080516020610d82833981519152836106f9565b610524600080516020610dc2833981519152826105be565b600061056581336107d6565b61056f838361083a565b505050565b610524600080516020610d42833981519152826105be565b6000610506600080516020610dc2833981519152836106f9565b610524600080516020610de28339815191528261077e565b6000828152602081905260409020600101546105da81336107d6565b61056f8383610885565b6001600160a01b03811633146106595760405162461bcd60e51b815260206004820152602f60248201527f416363657373436f6e74726f6c3a2063616e206f6e6c792072656e6f756e636560448201526e103937b632b9903337b91039b2b63360891b60648201526084015b60405180910390fd5b6106638282610909565b5050565b610524600080516020610d628339815191528261077e565b610524600080516020610d62833981519152826105be565b6000610506600080516020610d62833981519152836106f9565b6000610506600080516020610da2833981519152836106f9565b610524600080516020610dc28339815191528261077e565b6000610506600080516020610d42833981519152835b6000918252602082815260408084206001600160a01b0393909316845291905290205460ff1690565b610524600080516020610da2833981519152826105be565b610524600080516020610d82833981519152826105be565b610524600080516020610de2833981519152826105be565b610524600080516020610d82833981519152825b60008281526020819052604090206001015461079a81336107d6565b61056f8383610909565b610524600080516020610d428339815191528261077e565b6000610506600080516020610de2833981519152836106f9565b6107e082826106f9565b610663576107f8816001600160a01b0316601461096e565b61080383602061096e565b604051602001610814929190610c09565b60408051601f198184030181529082905262461bcd60e51b825261065091600401610c7e565b600082815260208190526040808220600101805490849055905190918391839186917fbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff9190a4505050565b61088f82826106f9565b610663576000828152602081815260408083206001600160a01b03851684529091529020805460ff191660011790556108c53390565b6001600160a01b0316816001600160a01b0316837f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45050565b61091382826106f9565b15610663576000828152602081815260408083206001600160a01b0385168085529252808320805460ff1916905551339285917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a45050565b6060600061097d836002610cc7565b610988906002610ce6565b67ffffffffffffffff8111156109a0576109a0610cfe565b6040519080825280601f01601f1916602001820160405280156109ca576020820181803683370190505b509050600360fc1b816000815181106109e5576109e5610d14565b60200101906001600160f81b031916908160001a905350600f60fb1b81600181518110610a1457610a14610d14565b60200101906001600160f81b031916908160001a9053506000610a38846002610cc7565b610a43906001610ce6565b90505b6001811115610abb576f181899199a1a9b1b9c1cb0b131b232b360811b85600f1660108110610a7757610a77610d14565b1a60f81b828281518110610a8d57610a8d610d14565b60200101906001600160f81b031916908160001a90535060049490941c93610ab481610d2a565b9050610a46565b508315610b0a5760405162461bcd60e51b815260206004820181905260248201527f537472696e67733a20686578206c656e67746820696e73756666696369656e746044820152606401610650565b9392505050565b600060208284031215610b2357600080fd5b81356001600160e01b031981168114610b0a57600080fd5b80356001600160a01b0381168114610b5257600080fd5b919050565b600060208284031215610b6957600080fd5b610b0a82610b3b565b60008060408385031215610b8557600080fd5b50508035926020909101359150565b600060208284031215610ba657600080fd5b5035919050565b60008060408385031215610bc057600080fd5b82359150610bd060208401610b3b565b90509250929050565b60005b83811015610bf4578181015183820152602001610bdc565b83811115610c03576000848401525b50505050565b7f416363657373436f6e74726f6c3a206163636f756e7420000000000000000000815260008351610c41816017850160208801610bd9565b7001034b99036b4b9b9b4b733903937b6329607d1b6017918401918201528351610c72816028840160208801610bd9565b01602801949350505050565b6020815260008251806020840152610c9d816040850160208701610bd9565b601f01601f19169190910160400192915050565b634e487b7160e01b600052601160045260246000fd5b6000816000190483118215151615610ce157610ce1610cb1565b500290565b60008219821115610cf957610cf9610cb1565b500190565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b600081610d3957610d39610cb1565b50600019019056fe12ad05bde78c5ab75238ce885307f96ecd482bb402ef831f99e7018a0f169b7b8aa855a911518ecfbe5bc3088c8f3dda7badf130faaf8ace33fdc33828e1816719c860a63258efbd0ecb7d55c626237bf5c2044c26c073390b74f0c13c85743308fb31c3e81624356c3314088aa971b73bcc82d22bc3e3b184b4593077ae32785c91514091af31f62f596a314af7d5be40146b2f2355969392f055e12e0982fb939b8dfb57ecef2aea54a93a15e86768b9d4089f1ba61c245e6ec980695f4ca4a2646970667358221220174ee4b3636d382d4df6d1acfb23d08f9ac4046a4455b2ff752b07e4d2fc48e064736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ACLManager<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ACLManager<M> {
        fn clone(&self) -> Self {
            ACLManager(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ACLManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ACLManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ACLManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ACLManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ACLMANAGER_ABI.clone(), client).into()
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
                ACLMANAGER_ABI.clone(),
                ACLMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ASSET_LISTING_ADMIN_ROLE` (0x78bb0a43) function"]
        pub fn asset_listing_admin_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([120, 187, 10, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BRIDGE_ROLE` (0xb5bfddea) function"]
        pub fn bridge_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([181, 191, 221, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `EMERGENCY_ADMIN_ROLE` (0x6e76fc8f) function"]
        pub fn emergency_admin_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([110, 118, 252, 143], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FLASH_BORROWER_ROLE` (0x5577b7a9) function"]
        pub fn flash_borrower_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([85, 119, 183, 169], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_ADMIN_ROLE` (0xb8f6dba7) function"]
        pub fn pool_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 246, 219, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RISK_ADMIN_ROLE` (0x4f16b425) function"]
        pub fn risk_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([79, 22, 180, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addAssetListingAdmin` (0x9a2b96f7) function"]
        pub fn add_asset_listing_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 43, 150, 247], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addBridge` (0x9712fdf8) function"]
        pub fn add_bridge(
            &self,
            bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 18, 253, 248], bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addEmergencyAdmin` (0x179efb09) function"]
        pub fn add_emergency_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 158, 251, 9], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addFlashBorrower` (0x9ac9d80b) function"]
        pub fn add_flash_borrower(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 201, 216, 11], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPoolAdmin` (0x22650caf) function"]
        pub fn add_pool_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 101, 12, 175], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addRiskAdmin` (0x5b9a94e4) function"]
        pub fn add_risk_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 154, 148, 228], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAssetListingAdmin` (0x13ee32e0) function"]
        pub fn is_asset_listing_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 238, 50, 224], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isBridge` (0x726600ce) function"]
        pub fn is_bridge(
            &self,
            bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 102, 0, 206], bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isEmergencyAdmin` (0x2500f2b6) function"]
        pub fn is_emergency_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 0, 242, 182], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isFlashBorrower` (0xfa50f297) function"]
        pub fn is_flash_borrower(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 80, 242, 151], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPoolAdmin` (0x7be53ca1) function"]
        pub fn is_pool_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 229, 60, 161], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isRiskAdmin` (0x674b5e4d) function"]
        pub fn is_risk_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([103, 75, 94, 77], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeAssetListingAdmin` (0xa21bce15) function"]
        pub fn remove_asset_listing_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 27, 206, 21], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeBridge` (0x04df017d) function"]
        pub fn remove_bridge(
            &self,
            bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 223, 1, 125], bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeEmergencyAdmin` (0x7a9a93f4) function"]
        pub fn remove_emergency_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 154, 147, 244], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeFlashBorrower` (0x253cf980) function"]
        pub fn remove_flash_borrower(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 60, 249, 128], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removePoolAdmin` (0xf83695cb) function"]
        pub fn remove_pool_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 54, 149, 203], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeRiskAdmin` (0x3c5a08e5) function"]
        pub fn remove_risk_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 90, 8, 229], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRoleAdmin` (0x1e4e0091) function"]
        pub fn set_role_admin(
            &self,
            role: [u8; 32],
            admin_role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 78, 0, 145], (role, admin_role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ACLManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ACLManager<M> {
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ACLManagerEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ethers::contract::EthLogDecode for ACLManagerEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ACLManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ACLManagerEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                ACLManagerEvents::RoleGrantedFilter(element) => element.fmt(f),
                ACLManagerEvents::RoleRevokedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    #[doc = "Container type for all input parameters for the `ASSET_LISTING_ADMIN_ROLE` function with signature `ASSET_LISTING_ADMIN_ROLE()` and selector `[120, 187, 10, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ASSET_LISTING_ADMIN_ROLE", abi = "ASSET_LISTING_ADMIN_ROLE()")]
    pub struct AssetListingAdminRoleCall;
    #[doc = "Container type for all input parameters for the `BRIDGE_ROLE` function with signature `BRIDGE_ROLE()` and selector `[181, 191, 221, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BRIDGE_ROLE", abi = "BRIDGE_ROLE()")]
    pub struct BridgeRoleCall;
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `EMERGENCY_ADMIN_ROLE` function with signature `EMERGENCY_ADMIN_ROLE()` and selector `[110, 118, 252, 143]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "EMERGENCY_ADMIN_ROLE", abi = "EMERGENCY_ADMIN_ROLE()")]
    pub struct EmergencyAdminRoleCall;
    #[doc = "Container type for all input parameters for the `FLASH_BORROWER_ROLE` function with signature `FLASH_BORROWER_ROLE()` and selector `[85, 119, 183, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "FLASH_BORROWER_ROLE", abi = "FLASH_BORROWER_ROLE()")]
    pub struct FlashBorrowerRoleCall;
    #[doc = "Container type for all input parameters for the `POOL_ADMIN_ROLE` function with signature `POOL_ADMIN_ROLE()` and selector `[184, 246, 219, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL_ADMIN_ROLE", abi = "POOL_ADMIN_ROLE()")]
    pub struct PoolAdminRoleCall;
    #[doc = "Container type for all input parameters for the `RISK_ADMIN_ROLE` function with signature `RISK_ADMIN_ROLE()` and selector `[79, 22, 180, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RISK_ADMIN_ROLE", abi = "RISK_ADMIN_ROLE()")]
    pub struct RiskAdminRoleCall;
    #[doc = "Container type for all input parameters for the `addAssetListingAdmin` function with signature `addAssetListingAdmin(address)` and selector `[154, 43, 150, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addAssetListingAdmin", abi = "addAssetListingAdmin(address)")]
    pub struct AddAssetListingAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addBridge` function with signature `addBridge(address)` and selector `[151, 18, 253, 248]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addBridge", abi = "addBridge(address)")]
    pub struct AddBridgeCall {
        pub bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addEmergencyAdmin` function with signature `addEmergencyAdmin(address)` and selector `[23, 158, 251, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addEmergencyAdmin", abi = "addEmergencyAdmin(address)")]
    pub struct AddEmergencyAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addFlashBorrower` function with signature `addFlashBorrower(address)` and selector `[154, 201, 216, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addFlashBorrower", abi = "addFlashBorrower(address)")]
    pub struct AddFlashBorrowerCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addPoolAdmin` function with signature `addPoolAdmin(address)` and selector `[34, 101, 12, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addPoolAdmin", abi = "addPoolAdmin(address)")]
    pub struct AddPoolAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addRiskAdmin` function with signature `addRiskAdmin(address)` and selector `[91, 154, 148, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addRiskAdmin", abi = "addRiskAdmin(address)")]
    pub struct AddRiskAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isAssetListingAdmin` function with signature `isAssetListingAdmin(address)` and selector `[19, 238, 50, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isAssetListingAdmin", abi = "isAssetListingAdmin(address)")]
    pub struct IsAssetListingAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isBridge` function with signature `isBridge(address)` and selector `[114, 102, 0, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isBridge", abi = "isBridge(address)")]
    pub struct IsBridgeCall {
        pub bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isEmergencyAdmin` function with signature `isEmergencyAdmin(address)` and selector `[37, 0, 242, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isEmergencyAdmin", abi = "isEmergencyAdmin(address)")]
    pub struct IsEmergencyAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isFlashBorrower` function with signature `isFlashBorrower(address)` and selector `[250, 80, 242, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isFlashBorrower", abi = "isFlashBorrower(address)")]
    pub struct IsFlashBorrowerCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isPoolAdmin` function with signature `isPoolAdmin(address)` and selector `[123, 229, 60, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPoolAdmin", abi = "isPoolAdmin(address)")]
    pub struct IsPoolAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isRiskAdmin` function with signature `isRiskAdmin(address)` and selector `[103, 75, 94, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isRiskAdmin", abi = "isRiskAdmin(address)")]
    pub struct IsRiskAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeAssetListingAdmin` function with signature `removeAssetListingAdmin(address)` and selector `[162, 27, 206, 21]`"]
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
        name = "removeAssetListingAdmin",
        abi = "removeAssetListingAdmin(address)"
    )]
    pub struct RemoveAssetListingAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeBridge` function with signature `removeBridge(address)` and selector `[4, 223, 1, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeBridge", abi = "removeBridge(address)")]
    pub struct RemoveBridgeCall {
        pub bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeEmergencyAdmin` function with signature `removeEmergencyAdmin(address)` and selector `[122, 154, 147, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeEmergencyAdmin", abi = "removeEmergencyAdmin(address)")]
    pub struct RemoveEmergencyAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeFlashBorrower` function with signature `removeFlashBorrower(address)` and selector `[37, 60, 249, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeFlashBorrower", abi = "removeFlashBorrower(address)")]
    pub struct RemoveFlashBorrowerCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removePoolAdmin` function with signature `removePoolAdmin(address)` and selector `[248, 54, 149, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removePoolAdmin", abi = "removePoolAdmin(address)")]
    pub struct RemovePoolAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeRiskAdmin` function with signature `removeRiskAdmin(address)` and selector `[60, 90, 8, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeRiskAdmin", abi = "removeRiskAdmin(address)")]
    pub struct RemoveRiskAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setRoleAdmin` function with signature `setRoleAdmin(bytes32,bytes32)` and selector `[30, 78, 0, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setRoleAdmin", abi = "setRoleAdmin(bytes32,bytes32)")]
    pub struct SetRoleAdminCall {
        pub role: [u8; 32],
        pub admin_role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ACLManagerCalls {
        AddressesProvider(AddressesProviderCall),
        AssetListingAdminRole(AssetListingAdminRoleCall),
        BridgeRole(BridgeRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        EmergencyAdminRole(EmergencyAdminRoleCall),
        FlashBorrowerRole(FlashBorrowerRoleCall),
        PoolAdminRole(PoolAdminRoleCall),
        RiskAdminRole(RiskAdminRoleCall),
        AddAssetListingAdmin(AddAssetListingAdminCall),
        AddBridge(AddBridgeCall),
        AddEmergencyAdmin(AddEmergencyAdminCall),
        AddFlashBorrower(AddFlashBorrowerCall),
        AddPoolAdmin(AddPoolAdminCall),
        AddRiskAdmin(AddRiskAdminCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsAssetListingAdmin(IsAssetListingAdminCall),
        IsBridge(IsBridgeCall),
        IsEmergencyAdmin(IsEmergencyAdminCall),
        IsFlashBorrower(IsFlashBorrowerCall),
        IsPoolAdmin(IsPoolAdminCall),
        IsRiskAdmin(IsRiskAdminCall),
        RemoveAssetListingAdmin(RemoveAssetListingAdminCall),
        RemoveBridge(RemoveBridgeCall),
        RemoveEmergencyAdmin(RemoveEmergencyAdminCall),
        RemoveFlashBorrower(RemoveFlashBorrowerCall),
        RemovePoolAdmin(RemovePoolAdminCall),
        RemoveRiskAdmin(RemoveRiskAdminCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetRoleAdmin(SetRoleAdminCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ethers::core::abi::AbiDecode for ACLManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <AssetListingAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AssetListingAdminRole(decoded));
            }
            if let Ok(decoded) =
                <BridgeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::BridgeRole(decoded));
            }
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <EmergencyAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::EmergencyAdminRole(decoded));
            }
            if let Ok(decoded) =
                <FlashBorrowerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::FlashBorrowerRole(decoded));
            }
            if let Ok(decoded) =
                <PoolAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::PoolAdminRole(decoded));
            }
            if let Ok(decoded) =
                <RiskAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RiskAdminRole(decoded));
            }
            if let Ok(decoded) =
                <AddAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddBridge(decoded));
            }
            if let Ok(decoded) =
                <AddEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <AddPoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::AddRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <IsAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsBridge(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <IsPoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::IsRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemoveAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemoveBridge(decoded));
            }
            if let Ok(decoded) =
                <RemoveEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemoveEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemoveFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <RemovePoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemovePoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RemoveRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::SetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLManagerCalls::SupportsInterface(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ACLManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ACLManagerCalls::AddressesProvider(element) => element.encode(),
                ACLManagerCalls::AssetListingAdminRole(element) => element.encode(),
                ACLManagerCalls::BridgeRole(element) => element.encode(),
                ACLManagerCalls::DefaultAdminRole(element) => element.encode(),
                ACLManagerCalls::EmergencyAdminRole(element) => element.encode(),
                ACLManagerCalls::FlashBorrowerRole(element) => element.encode(),
                ACLManagerCalls::PoolAdminRole(element) => element.encode(),
                ACLManagerCalls::RiskAdminRole(element) => element.encode(),
                ACLManagerCalls::AddAssetListingAdmin(element) => element.encode(),
                ACLManagerCalls::AddBridge(element) => element.encode(),
                ACLManagerCalls::AddEmergencyAdmin(element) => element.encode(),
                ACLManagerCalls::AddFlashBorrower(element) => element.encode(),
                ACLManagerCalls::AddPoolAdmin(element) => element.encode(),
                ACLManagerCalls::AddRiskAdmin(element) => element.encode(),
                ACLManagerCalls::GetRoleAdmin(element) => element.encode(),
                ACLManagerCalls::GrantRole(element) => element.encode(),
                ACLManagerCalls::HasRole(element) => element.encode(),
                ACLManagerCalls::IsAssetListingAdmin(element) => element.encode(),
                ACLManagerCalls::IsBridge(element) => element.encode(),
                ACLManagerCalls::IsEmergencyAdmin(element) => element.encode(),
                ACLManagerCalls::IsFlashBorrower(element) => element.encode(),
                ACLManagerCalls::IsPoolAdmin(element) => element.encode(),
                ACLManagerCalls::IsRiskAdmin(element) => element.encode(),
                ACLManagerCalls::RemoveAssetListingAdmin(element) => element.encode(),
                ACLManagerCalls::RemoveBridge(element) => element.encode(),
                ACLManagerCalls::RemoveEmergencyAdmin(element) => element.encode(),
                ACLManagerCalls::RemoveFlashBorrower(element) => element.encode(),
                ACLManagerCalls::RemovePoolAdmin(element) => element.encode(),
                ACLManagerCalls::RemoveRiskAdmin(element) => element.encode(),
                ACLManagerCalls::RenounceRole(element) => element.encode(),
                ACLManagerCalls::RevokeRole(element) => element.encode(),
                ACLManagerCalls::SetRoleAdmin(element) => element.encode(),
                ACLManagerCalls::SupportsInterface(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ACLManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ACLManagerCalls::AddressesProvider(element) => element.fmt(f),
                ACLManagerCalls::AssetListingAdminRole(element) => element.fmt(f),
                ACLManagerCalls::BridgeRole(element) => element.fmt(f),
                ACLManagerCalls::DefaultAdminRole(element) => element.fmt(f),
                ACLManagerCalls::EmergencyAdminRole(element) => element.fmt(f),
                ACLManagerCalls::FlashBorrowerRole(element) => element.fmt(f),
                ACLManagerCalls::PoolAdminRole(element) => element.fmt(f),
                ACLManagerCalls::RiskAdminRole(element) => element.fmt(f),
                ACLManagerCalls::AddAssetListingAdmin(element) => element.fmt(f),
                ACLManagerCalls::AddBridge(element) => element.fmt(f),
                ACLManagerCalls::AddEmergencyAdmin(element) => element.fmt(f),
                ACLManagerCalls::AddFlashBorrower(element) => element.fmt(f),
                ACLManagerCalls::AddPoolAdmin(element) => element.fmt(f),
                ACLManagerCalls::AddRiskAdmin(element) => element.fmt(f),
                ACLManagerCalls::GetRoleAdmin(element) => element.fmt(f),
                ACLManagerCalls::GrantRole(element) => element.fmt(f),
                ACLManagerCalls::HasRole(element) => element.fmt(f),
                ACLManagerCalls::IsAssetListingAdmin(element) => element.fmt(f),
                ACLManagerCalls::IsBridge(element) => element.fmt(f),
                ACLManagerCalls::IsEmergencyAdmin(element) => element.fmt(f),
                ACLManagerCalls::IsFlashBorrower(element) => element.fmt(f),
                ACLManagerCalls::IsPoolAdmin(element) => element.fmt(f),
                ACLManagerCalls::IsRiskAdmin(element) => element.fmt(f),
                ACLManagerCalls::RemoveAssetListingAdmin(element) => element.fmt(f),
                ACLManagerCalls::RemoveBridge(element) => element.fmt(f),
                ACLManagerCalls::RemoveEmergencyAdmin(element) => element.fmt(f),
                ACLManagerCalls::RemoveFlashBorrower(element) => element.fmt(f),
                ACLManagerCalls::RemovePoolAdmin(element) => element.fmt(f),
                ACLManagerCalls::RemoveRiskAdmin(element) => element.fmt(f),
                ACLManagerCalls::RenounceRole(element) => element.fmt(f),
                ACLManagerCalls::RevokeRole(element) => element.fmt(f),
                ACLManagerCalls::SetRoleAdmin(element) => element.fmt(f),
                ACLManagerCalls::SupportsInterface(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for ACLManagerCalls {
        fn from(var: AddressesProviderCall) -> Self {
            ACLManagerCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<AssetListingAdminRoleCall> for ACLManagerCalls {
        fn from(var: AssetListingAdminRoleCall) -> Self {
            ACLManagerCalls::AssetListingAdminRole(var)
        }
    }
    impl ::std::convert::From<BridgeRoleCall> for ACLManagerCalls {
        fn from(var: BridgeRoleCall) -> Self {
            ACLManagerCalls::BridgeRole(var)
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for ACLManagerCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            ACLManagerCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<EmergencyAdminRoleCall> for ACLManagerCalls {
        fn from(var: EmergencyAdminRoleCall) -> Self {
            ACLManagerCalls::EmergencyAdminRole(var)
        }
    }
    impl ::std::convert::From<FlashBorrowerRoleCall> for ACLManagerCalls {
        fn from(var: FlashBorrowerRoleCall) -> Self {
            ACLManagerCalls::FlashBorrowerRole(var)
        }
    }
    impl ::std::convert::From<PoolAdminRoleCall> for ACLManagerCalls {
        fn from(var: PoolAdminRoleCall) -> Self {
            ACLManagerCalls::PoolAdminRole(var)
        }
    }
    impl ::std::convert::From<RiskAdminRoleCall> for ACLManagerCalls {
        fn from(var: RiskAdminRoleCall) -> Self {
            ACLManagerCalls::RiskAdminRole(var)
        }
    }
    impl ::std::convert::From<AddAssetListingAdminCall> for ACLManagerCalls {
        fn from(var: AddAssetListingAdminCall) -> Self {
            ACLManagerCalls::AddAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<AddBridgeCall> for ACLManagerCalls {
        fn from(var: AddBridgeCall) -> Self {
            ACLManagerCalls::AddBridge(var)
        }
    }
    impl ::std::convert::From<AddEmergencyAdminCall> for ACLManagerCalls {
        fn from(var: AddEmergencyAdminCall) -> Self {
            ACLManagerCalls::AddEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<AddFlashBorrowerCall> for ACLManagerCalls {
        fn from(var: AddFlashBorrowerCall) -> Self {
            ACLManagerCalls::AddFlashBorrower(var)
        }
    }
    impl ::std::convert::From<AddPoolAdminCall> for ACLManagerCalls {
        fn from(var: AddPoolAdminCall) -> Self {
            ACLManagerCalls::AddPoolAdmin(var)
        }
    }
    impl ::std::convert::From<AddRiskAdminCall> for ACLManagerCalls {
        fn from(var: AddRiskAdminCall) -> Self {
            ACLManagerCalls::AddRiskAdmin(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for ACLManagerCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            ACLManagerCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for ACLManagerCalls {
        fn from(var: GrantRoleCall) -> Self {
            ACLManagerCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for ACLManagerCalls {
        fn from(var: HasRoleCall) -> Self {
            ACLManagerCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<IsAssetListingAdminCall> for ACLManagerCalls {
        fn from(var: IsAssetListingAdminCall) -> Self {
            ACLManagerCalls::IsAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<IsBridgeCall> for ACLManagerCalls {
        fn from(var: IsBridgeCall) -> Self {
            ACLManagerCalls::IsBridge(var)
        }
    }
    impl ::std::convert::From<IsEmergencyAdminCall> for ACLManagerCalls {
        fn from(var: IsEmergencyAdminCall) -> Self {
            ACLManagerCalls::IsEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<IsFlashBorrowerCall> for ACLManagerCalls {
        fn from(var: IsFlashBorrowerCall) -> Self {
            ACLManagerCalls::IsFlashBorrower(var)
        }
    }
    impl ::std::convert::From<IsPoolAdminCall> for ACLManagerCalls {
        fn from(var: IsPoolAdminCall) -> Self {
            ACLManagerCalls::IsPoolAdmin(var)
        }
    }
    impl ::std::convert::From<IsRiskAdminCall> for ACLManagerCalls {
        fn from(var: IsRiskAdminCall) -> Self {
            ACLManagerCalls::IsRiskAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveAssetListingAdminCall> for ACLManagerCalls {
        fn from(var: RemoveAssetListingAdminCall) -> Self {
            ACLManagerCalls::RemoveAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveBridgeCall> for ACLManagerCalls {
        fn from(var: RemoveBridgeCall) -> Self {
            ACLManagerCalls::RemoveBridge(var)
        }
    }
    impl ::std::convert::From<RemoveEmergencyAdminCall> for ACLManagerCalls {
        fn from(var: RemoveEmergencyAdminCall) -> Self {
            ACLManagerCalls::RemoveEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveFlashBorrowerCall> for ACLManagerCalls {
        fn from(var: RemoveFlashBorrowerCall) -> Self {
            ACLManagerCalls::RemoveFlashBorrower(var)
        }
    }
    impl ::std::convert::From<RemovePoolAdminCall> for ACLManagerCalls {
        fn from(var: RemovePoolAdminCall) -> Self {
            ACLManagerCalls::RemovePoolAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveRiskAdminCall> for ACLManagerCalls {
        fn from(var: RemoveRiskAdminCall) -> Self {
            ACLManagerCalls::RemoveRiskAdmin(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for ACLManagerCalls {
        fn from(var: RenounceRoleCall) -> Self {
            ACLManagerCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for ACLManagerCalls {
        fn from(var: RevokeRoleCall) -> Self {
            ACLManagerCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<SetRoleAdminCall> for ACLManagerCalls {
        fn from(var: SetRoleAdminCall) -> Self {
            ACLManagerCalls::SetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ACLManagerCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ACLManagerCalls::SupportsInterface(var)
        }
    }
    #[doc = "Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressesProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `ASSET_LISTING_ADMIN_ROLE` function with signature `ASSET_LISTING_ADMIN_ROLE()` and selector `[120, 187, 10, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AssetListingAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `BRIDGE_ROLE` function with signature `BRIDGE_ROLE()` and selector `[181, 191, 221, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BridgeRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `EMERGENCY_ADMIN_ROLE` function with signature `EMERGENCY_ADMIN_ROLE()` and selector `[110, 118, 252, 143]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EmergencyAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `FLASH_BORROWER_ROLE` function with signature `FLASH_BORROWER_ROLE()` and selector `[85, 119, 183, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FlashBorrowerRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `POOL_ADMIN_ROLE` function with signature `POOL_ADMIN_ROLE()` and selector `[184, 246, 219, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `RISK_ADMIN_ROLE` function with signature `RISK_ADMIN_ROLE()` and selector `[79, 22, 180, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RiskAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct HasRoleReturn(pub bool);
    #[doc = "Container type for all return fields from the `isAssetListingAdmin` function with signature `isAssetListingAdmin(address)` and selector `[19, 238, 50, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsAssetListingAdminReturn(pub bool);
    #[doc = "Container type for all return fields from the `isBridge` function with signature `isBridge(address)` and selector `[114, 102, 0, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsBridgeReturn(pub bool);
    #[doc = "Container type for all return fields from the `isEmergencyAdmin` function with signature `isEmergencyAdmin(address)` and selector `[37, 0, 242, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsEmergencyAdminReturn(pub bool);
    #[doc = "Container type for all return fields from the `isFlashBorrower` function with signature `isFlashBorrower(address)` and selector `[250, 80, 242, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsFlashBorrowerReturn(pub bool);
    #[doc = "Container type for all return fields from the `isPoolAdmin` function with signature `isPoolAdmin(address)` and selector `[123, 229, 60, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsPoolAdminReturn(pub bool);
    #[doc = "Container type for all return fields from the `isRiskAdmin` function with signature `isRiskAdmin(address)` and selector `[103, 75, 94, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsRiskAdminReturn(pub bool);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
}
