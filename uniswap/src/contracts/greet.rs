pub use greet_mod::*;
#[allow(clippy::too_many_arguments)]
mod greet_mod {
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
    #[doc = "Greet was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GREET_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testCanSetGreeting\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testCannotGm\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"greeting\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testWorksForAllGreetings\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static GREET_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805460ff1916600117905534801561001d57600080fd5b506114038061002d6000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c80630a9254e4146100675780635306d34d146100715780636721f71814610079578063ba414fa61461008c578063e0747015146100b2578063fa7626d4146100ba575b600080fd5b61006f6100c7565b005b61006f61023a565b61006f6100873660046106eb565b610329565b60005461009e90610100900460ff1681565b604051901515815260200160405180910390f35b61006f61040f565b60005461009e9060ff1681565b6040516100d390610666565b604051809103906000f0801580156100ef573d6000803e3d6000fd5b506000805462010000600160b01b031916620100006001600160a01b039384168102919091179182905560405191049091169061012b90610673565b6001600160a01b039091168152602001604051809103906000f080158015610157573d6000803e3d6000fd5b50600180546001600160a01b0319166001600160a01b03928316179055600054604051620100009091049091169061018e90610673565b6001600160a01b039091168152602001604051809103906000f0801580156101ba573d6000803e3d6000fd5b50600280546001600160a01b0319166001600160a01b0392831617905560005460015460405163f2fde38b60e01b81529083166004820152620100009091049091169063f2fde38b90602401600060405180830381600087803b15801561022057600080fd5b505af1158015610234573d6000803e3d6000fd5b50505050565b600154604051633ab5c43160e21b8152602060048201526002602482015261676d60f01b60448201526001600160a01b039091169063ead710c490606401600060405180830381600087803b15801561029257600080fd5b505af19250505080156102a3575060015b610313576102af610771565b806308c379a00361030757506102c361078d565b806102ce5750610309565b610304816040518060400160405280601481526020017363616e6e6f74206772656574207769746820676d60601b815250610519565b50565b505b3d6000803e3d6000fd5b6103276000805461ff001916610100179055565b565b600154604051633ab5c43160e21b81526001600160a01b039091169063ead710c49061035990849060040161086f565b600060405180830381600087803b15801561037357600080fd5b505af1158015610387573d6000803e3d6000fd5b50505050610304600060029054906101000a90046001600160a01b03166001600160a01b031663ef690cc06040518163ffffffff1660e01b8152600401600060405180830381865afa1580156103e1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104099190810190610889565b82610519565b600154604051633ab5c43160e21b8152602060048201526002602482015261686960f01b60448201526001600160a01b039091169063ead710c490606401600060405180830381600087803b15801561046757600080fd5b505af115801561047b573d6000803e3d6000fd5b50505050610327600060029054906101000a90046001600160a01b03166001600160a01b031663ef690cc06040518163ffffffff1660e01b8152600401600060405180830381865afa1580156104d5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104fd9190810190610889565b60405180604001604052806002815260200161686960f01b8152505b8060405160200161052a919061090b565b6040516020818303038152906040528051906020012082604051602001610551919061090b565b6040516020818303038152906040528051906020012014610662577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f506040516105d89060208082526024908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b737472604082015263696e675d60e01b606082015260800190565b60405180910390a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf35838260405161060f9190610927565b60405180910390a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358381604051610646919061095a565b60405180910390a16106626000805461ff001916610100179055565b5050565b6107438061098e83390190565b6102fd806110d183390190565b634e487b7160e01b600052604160045260246000fd5b601f8201601f1916810167ffffffffffffffff811182821017156106bc576106bc610680565b6040525050565b600067ffffffffffffffff8211156106dd576106dd610680565b50601f01601f191660200190565b6000602082840312156106fd57600080fd5b813567ffffffffffffffff81111561071457600080fd5b8201601f8101841361072557600080fd5b8035610730816106c3565b60405161073d8282610696565b82815286602084860101111561075257600080fd5b8260208501602083013760009281016020019290925250949350505050565b600060033d111561078a5760046000803e5060005160e01c5b90565b600060443d101561079b5790565b6040516003193d81016004833e81513d67ffffffffffffffff81602484011181841117156107cb57505050505090565b82850191508151818111156107e35750505050505090565b843d87010160208285010111156107fd5750505050505090565b61080c60208286010187610696565b509095945050505050565b60005b8381101561083257818101518382015260200161081a565b838111156102345750506000910152565b6000815180845261085b816020860160208601610817565b601f01601f19169290920160200192915050565b6020815260006108826020830184610843565b9392505050565b60006020828403121561089b57600080fd5b815167ffffffffffffffff8111156108b257600080fd5b8201601f810184136108c357600080fd5b80516108ce816106c3565b6040516108db8282610696565b8281528660208486010111156108f057600080fd5b610901836020830160208701610817565b9695505050505050565b6000825161091d818460208701610817565b9190910192915050565b604081526009604082015268202056616c7565206160b81b60608201526080602082015260006108826080830184610843565b60408152600960408201526810102b30b63ab2903160b91b6060820152608060208201526000610882608083018461084356fe608060405234801561001057600080fd5b5061001a3361001f565b61006f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6106c58061007e6000396000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c8063715018a6146100675780638da5cb5b14610071578063c0129d4314610091578063ead710c414610099578063ef690cc0146100ac578063f2fde38b146100c1575b600080fd5b61006f6100d4565b005b6000546040516001600160a01b0390911681526020015b60405180910390f35b61006f610113565b61006f6100a736600461047d565b6101af565b6100b461025c565b604051610088919061055e565b61006f6100cf366004610591565b6102ea565b6000546001600160a01b031633146101075760405162461bcd60e51b81526004016100fe906105c1565b60405180910390fd5b610111600061037e565b565b6000546001600160a01b0316331461013d5760405162461bcd60e51b81526004016100fe906105c1565b610148600a436105f6565b60001460405180606001604052806021815260200161066f60219139906101825760405162461bcd60e51b81526004016100fe919061055e565b5060408051808201909152600280825261676d60f01b60209092019182526101ac916001916103ce565b50565b7f71b78290913af2addd8fcbe5766de306af2c8afbc466ca891e207f73638c7270816040516020016101e19190610618565b6040516020818303038152906040528051906020012014156040518060400160405280601481526020017363616e6e6f74206772656574207769746820676d60601b815250906102445760405162461bcd60e51b81526004016100fe919061055e565b5080516102589060019060208401906103ce565b5050565b6001805461026990610634565b80601f016020809104026020016040519081016040528092919081815260200182805461029590610634565b80156102e25780601f106102b7576101008083540402835291602001916102e2565b820191906000526020600020905b8154815290600101906020018083116102c557829003601f168201915b505050505081565b6000546001600160a01b031633146103145760405162461bcd60e51b81526004016100fe906105c1565b6001600160a01b0381166103795760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016100fe565b6101ac815b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b8280546103da90610634565b90600052602060002090601f0160209004810192826103fc5760008555610442565b82601f1061041557805160ff1916838001178555610442565b82800160010185558215610442579182015b82811115610442578251825591602001919060010190610427565b5061044e929150610452565b5090565b5b8082111561044e5760008155600101610453565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561048f57600080fd5b813567ffffffffffffffff808211156104a757600080fd5b818401915084601f8301126104bb57600080fd5b8135818111156104cd576104cd610467565b604051601f8201601f19908116603f011681019083821181831017156104f5576104f5610467565b8160405282815287602084870101111561050e57600080fd5b826020860160208301376000928101602001929092525095945050505050565b60005b83811015610549578181015183820152602001610531565b83811115610558576000848401525b50505050565b602081526000825180602084015261057d81604085016020870161052e565b601f01601f19169190910160400192915050565b6000602082840312156105a357600080fd5b81356001600160a01b03811681146105ba57600080fd5b9392505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b60008261061357634e487b7160e01b600052601260045260246000fd5b500690565b6000825161062a81846020870161052e565b9190910192915050565b600181811c9082168061064857607f821691505b60208210810361066857634e487b7160e01b600052602260045260246000fd5b5091905056fe696e76616c696420626c6f636b206e756d6265722c20706c656173652077616974a26469706673582212208a080f4ab1cd9aef0205233bb6e8b8c046d113364b1c43a48f5eae3ea64599da64736f6c634300080d0033608060405234801561001057600080fd5b506040516102fd3803806102fd83398101604081905261002f91610054565b600080546001600160a01b0319166001600160a01b0392909216919091179055610084565b60006020828403121561006657600080fd5b81516001600160a01b038116811461007d57600080fd5b9392505050565b61026a806100936000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063c0129d431461003b578063ead710c414610045575b600080fd5b610043610058565b005b61004361005336600461012e565b6100b3565b600080546040805163c0129d4360e01b815290516001600160a01b039092169263c0129d439260048084019382900301818387803b15801561009957600080fd5b505af11580156100ad573d6000803e3d6000fd5b50505050565b600054604051633ab5c43160e21b81526001600160a01b039091169063ead710c4906100e39084906004016101df565b600060405180830381600087803b1580156100fd57600080fd5b505af1158015610111573d6000803e3d6000fd5b5050505050565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561014057600080fd5b813567ffffffffffffffff8082111561015857600080fd5b818401915084601f83011261016c57600080fd5b81358181111561017e5761017e610118565b604051601f8201601f19908116603f011681019083821181831017156101a6576101a6610118565b816040528281528760208487010111156101bf57600080fd5b826020860160208301376000928101602001929092525095945050505050565b600060208083528351808285015260005b8181101561020c578581018301518582016040015282016101f0565b8181111561021e576000604083870101525b50601f01601f191692909201604001939250505056fea264697066735822122049ff2658e52a98cbd947f398cfa42dfa7d701f70b871200a7a1ffbbb8af77a7b64736f6c634300080d0033a2646970667358221220d22df26a8678d450d3849879559929e4262e2c165978925677db814a1ccd432864736f6c634300080d0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Greet<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Greet<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Greet<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Greet))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Greet<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GREET_ABI.clone(), client).into()
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
                GREET_ABI.clone(),
                GREET_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `testCanSetGreeting` (0xe0747015) function"]
        pub fn test_can_set_greeting(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 116, 112, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testCannotGm` (0x5306d34d) function"]
        pub fn test_cannot_gm(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 6, 211, 77], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testWorksForAllGreetings` (0x6721f718) function"]
        pub fn test_works_for_all_greetings(
            &self,
            greeting: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 33, 247, 24], greeting)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, GreetEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Greet<M> {
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
    pub enum GreetEvents {
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
    impl ethers::contract::EthLogDecode for GreetEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(GreetEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(GreetEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(GreetEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(GreetEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(GreetEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(GreetEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(GreetEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(GreetEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(GreetEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(GreetEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GreetEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GreetEvents::LogFilter(element) => element.fmt(f),
                GreetEvents::LogAddressFilter(element) => element.fmt(f),
                GreetEvents::LogBytesFilter(element) => element.fmt(f),
                GreetEvents::LogBytes32Filter(element) => element.fmt(f),
                GreetEvents::LogIntFilter(element) => element.fmt(f),
                GreetEvents::LogNamedAddressFilter(element) => element.fmt(f),
                GreetEvents::LogNamedBytesFilter(element) => element.fmt(f),
                GreetEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                GreetEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                GreetEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                GreetEvents::LogNamedIntFilter(element) => element.fmt(f),
                GreetEvents::LogNamedStringFilter(element) => element.fmt(f),
                GreetEvents::LogNamedUintFilter(element) => element.fmt(f),
                GreetEvents::LogStringFilter(element) => element.fmt(f),
                GreetEvents::LogUintFilter(element) => element.fmt(f),
                GreetEvents::LogsFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `testCanSetGreeting`function with signature `testCanSetGreeting()` and selector `[224, 116, 112, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testCanSetGreeting", abi = "testCanSetGreeting()")]
    pub struct TestCanSetGreetingCall;
    #[doc = "Container type for all input parameters for the `testCannotGm`function with signature `testCannotGm()` and selector `[83, 6, 211, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testCannotGm", abi = "testCannotGm()")]
    pub struct TestCannotGmCall;
    #[doc = "Container type for all input parameters for the `testWorksForAllGreetings`function with signature `testWorksForAllGreetings(string)` and selector `[103, 33, 247, 24]`"]
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
        name = "testWorksForAllGreetings",
        abi = "testWorksForAllGreetings(string)"
    )]
    pub struct TestWorksForAllGreetingsCall {
        pub greeting: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GreetCalls {
        IsTest(IsTestCall),
        Failed(FailedCall),
        SetUp(SetUpCall),
        TestCanSetGreeting(TestCanSetGreetingCall),
        TestCannotGm(TestCannotGmCall),
        TestWorksForAllGreetings(TestWorksForAllGreetingsCall),
    }
    impl ethers::core::abi::AbiDecode for GreetCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreetCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreetCalls::Failed(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreetCalls::SetUp(decoded));
            }
            if let Ok(decoded) =
                <TestCanSetGreetingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreetCalls::TestCanSetGreeting(decoded));
            }
            if let Ok(decoded) =
                <TestCannotGmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GreetCalls::TestCannotGm(decoded));
            }
            if let Ok(decoded) =
                <TestWorksForAllGreetingsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GreetCalls::TestWorksForAllGreetings(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GreetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GreetCalls::IsTest(element) => element.encode(),
                GreetCalls::Failed(element) => element.encode(),
                GreetCalls::SetUp(element) => element.encode(),
                GreetCalls::TestCanSetGreeting(element) => element.encode(),
                GreetCalls::TestCannotGm(element) => element.encode(),
                GreetCalls::TestWorksForAllGreetings(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GreetCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GreetCalls::IsTest(element) => element.fmt(f),
                GreetCalls::Failed(element) => element.fmt(f),
                GreetCalls::SetUp(element) => element.fmt(f),
                GreetCalls::TestCanSetGreeting(element) => element.fmt(f),
                GreetCalls::TestCannotGm(element) => element.fmt(f),
                GreetCalls::TestWorksForAllGreetings(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for GreetCalls {
        fn from(var: IsTestCall) -> Self {
            GreetCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<FailedCall> for GreetCalls {
        fn from(var: FailedCall) -> Self {
            GreetCalls::Failed(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for GreetCalls {
        fn from(var: SetUpCall) -> Self {
            GreetCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<TestCanSetGreetingCall> for GreetCalls {
        fn from(var: TestCanSetGreetingCall) -> Self {
            GreetCalls::TestCanSetGreeting(var)
        }
    }
    impl ::std::convert::From<TestCannotGmCall> for GreetCalls {
        fn from(var: TestCannotGmCall) -> Self {
            GreetCalls::TestCannotGm(var)
        }
    }
    impl ::std::convert::From<TestWorksForAllGreetingsCall> for GreetCalls {
        fn from(var: TestWorksForAllGreetingsCall) -> Self {
            GreetCalls::TestWorksForAllGreetings(var)
        }
    }
}
