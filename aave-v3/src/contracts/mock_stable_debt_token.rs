pub use mock_stable_debt_token::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mock_stable_debt_token {
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
    #[doc = "MockStableDebtToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MOCKSTABLEDEBTTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowAllowanceDelegated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"currentBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"avgStableRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalSupply\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"currentBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"avgStableRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newTotalSupply\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEBT_TOKEN_REVISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_WITH_SIG_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EIP712_REVISION\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_ASSET_ADDRESS\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveDelegation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowAllowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegationWithSig\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAverageStableRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getIncentivesController\",\"outputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSupplyData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalSupplyAndAvgRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalSupplyLastUpdated\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserLastUpdated\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserStableRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"initializingPool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"principalBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIncentivesController\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MOCKSTABLEDEBTTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e0604052600080553480156200001557600080fd5b50604051620023a5380380620023a5833981016040819052620000389162000237565b80806040518060400160405280601681526020017f535441424c455f444542545f544f4b454e5f494d504c000000000000000000008152506040518060400160405280601681526020017f535441424c455f444542545f544f4b454e5f494d504c0000000000000000000081525060004660808181525050836001600160a01b0316630542975c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000ef573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000115919062000237565b6001600160a01b031660a05282516200013690603b90602086019062000178565b5081516200014c90603c90602085019062000178565b50603d805460ff191660ff9290921691909117905550506001600160a01b031660c052506200029b9050565b82805462000186906200025e565b90600052602060002090601f016020900481019282620001aa5760008555620001f5565b82601f10620001c557805160ff1916838001178555620001f5565b82800160010185558215620001f5579182015b82811115620001f5578251825591602001919060010190620001d8565b506200020392915062000207565b5090565b5b8082111562000203576000815560010162000208565b6001600160a01b03811681146200023457600080fd5b50565b6000602082840312156200024a57600080fd5b815162000257816200021e565b9392505050565b600181811c908216806200027357607f821691505b602082108114156200029557634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c0516120be620002e7600039600081816102e8015281816109a301528181610d42015281816111440152611213015260006112b80152600061087101526120be6000f3fe608060405234801561001057600080fd5b50600436106101fb5760003560e01c806390f6fcf21161011a578063c04a8a10116100ad578063e655dbd81161007c578063e655dbd8146104c7578063e7484890146104da578063e78c9b3b146104f0578063f3bfc73814610529578063f731e9be1461055057600080fd5b8063c04a8a1014610480578063c222ec8a14610493578063c634dfaa146104a6578063dd62ed3e146104b957600080fd5b8063a9059cbb116100e9578063a9059cbb1461021e578063b16a19de14610437578063b3f1c93d14610448578063b9a7b6221461047857600080fd5b806390f6fcf2146103f657806395d89b41146104075780639dc29fac1461040f578063a457c2d71461021e57600080fd5b80636bd76d241161019257806378160376116101615780637816037614610338578063797743381461035857806379ce6b8c146103875780637ecebe00146103cd57600080fd5b80636bd76d241461029757806370a08231146102d05780637535d246146102e357806375d264131461032257600080fd5b806323b872dd116101ce57806323b872dd1461026c578063313ce5671461027a5780633644e5151461028f578063395093511461021e57600080fd5b806306fdde0314610200578063095ea7b31461021e5780630b52d5581461024157806318160ddd14610256575b600080fd5b610208610558565b6040516102159190611b30565b60405180910390f35b61023161022c366004611b66565b6105ea565b6040519015158152602001610215565b61025461024f366004611ba3565b610625565b005b61025e610850565b604051908152602001610215565b61023161022c366004611c11565b603d5460405160ff9091168152602001610215565b61025e61086d565b61025e6102a5366004611c52565b6001600160a01b03918216600090815260366020908152604080832093909416825291909152205490565b61025e6102de366004611c8b565b6108a6565b61030a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610215565b603d5461010090046001600160a01b031661030a565b610208604051806040016040528060018152602001603160f81b81525081565b61036061092d565b6040805194855260208501939093529183015264ffffffffff166060820152608001610215565b6103b7610395366004611c8b565b6001600160a01b03166000908152603e602052604090205464ffffffffff1690565b60405164ffffffffff9091168152602001610215565b61025e6103db366004611c8b565b6001600160a01b031660009081526034602052604090205490565b603f546001600160801b031661025e565b610208610973565b61042261041d366004611b66565b610982565b60408051928352602083019190915201610215565b6037546001600160a01b031661030a565b61045b610456366004611ca8565b610d34565b604080519315158452602084019290925290820152606001610215565b61025e600181565b61025461048e366004611b66565b611092565b6102546104a1366004611dd3565b6110a1565b61025e6104b4366004611c8b565b6112a3565b61025e61022c366004611c52565b6102546104d5366004611c8b565b6112b4565b603f54600160801b900464ffffffffff166103b7565b61025e6104fe366004611c8b565b6001600160a01b0316600090815260386020526040902054600160801b90046001600160801b031690565b61025e7f323db0410fecc107e39e2af5908671f4c8d106123b35a51501bb805c5fa36aa081565b610422611405565b6060603b805461056790611ea8565b80601f016020809104026020016040519081016040528092919081815260200182805461059390611ea8565b80156105e05780601f106105b5576101008083540402835291602001916105e0565b820191906000526020600020905b8154815290600101906020018083116105c357829003601f168201915b5050505050905090565b6040805180820182526002815261038360f41b6020820152905162461bcd60e51b815260009161061c91600401611b30565b60405180910390fd5b604080518082019091526002815261373760f01b60208201526001600160a01b0388166106655760405162461bcd60e51b815260040161061c9190611b30565b50834211156040518060400160405280600281526020016106e760f31b815250906106a35760405162461bcd60e51b815260040161061c9190611b30565b506001600160a01b038716600090815260346020526040812054906106c661086d565b604080517f323db0410fecc107e39e2af5908671f4c8d106123b35a51501bb805c5fa36aa060208201526001600160a01b038b1691810191909152606081018990526080810184905260a0810188905260c0016040516020818303038152906040528051906020012060405160200161075692919061190160f01b81526002810192909252602282015260420190565b60408051601f1981840301815282825280516020918201206000845290830180835281905260ff8816918301919091526060820186905260808201859052915060019060a0016020604051602081039080840390855afa1580156107be573d6000803e3d6000fd5b505050602060405103516001600160a01b0316896001600160a01b03161460405180604001604052806002815260200161373960f01b815250906108155760405162461bcd60e51b815260040161061c9190611b30565b50610821826001611ef9565b6001600160a01b038a16600090815260346020526040902055610845898989611427565b505050505050505050565b603f54600090610868906001600160801b0316611491565b905090565b60007f000000000000000000000000000000000000000000000000000000000000000046141561089e575060355490565b6108686114e0565b6000806108b283611589565b6001600160a01b038416600090815260386020526040902054909150600160801b90046001600160801b0316816108ed575060009392505050565b6001600160a01b0384166000908152603e602052604081205461091890839064ffffffffff166115ad565b905061092483826115c1565b95945050505050565b603f546000908190819081906001600160801b031661094b603a5490565b61095482611491565b603f549197909650919450600160801b900464ffffffffff1692509050565b6060603c805461056790611ea8565b604080518082019091526002815261323360f01b60208201526000908190337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316146109e95760405162461bcd60e51b815260040161061c9190611b30565b506000806109f686611605565b92509250506000610a05610850565b6001600160a01b038816600090815260386020526040812054919250908190600160801b90046001600160801b0316888411610a5557603f80546001600160801b03191690556000603a55610b0d565b610a5f8985611f11565b603a81905591506000610a86610a7486611655565b603f546001600160801b0316906115c1565b90506000610a9d610a968c611655565b84906115c1565b9050818110610ac457603f80546001600160801b03191690556000603a8190559450610b0a565b610ae8610ae3610ad386611655565b610add8486611f11565b90611670565b6116af565b603f80546001600160801b0319166001600160801b0392909216918217905594505b50505b85891415610b55576001600160a01b038a16600090815260386020908152604080832080546001600160801b03169055603e9091529020805464ffffffffff19169055610b83565b6001600160a01b038a166000908152603e60205260409020805464ffffffffff19164264ffffffffff161790555b603f805464ffffffffff60801b1916600160801b4264ffffffffff160217905588851115610c6e576000610bb78a87611f11565b9050610bc48b828761171c565b6040518181526001600160a01b038c16906000907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9060200160405180910390a36040805182815260208101899052908101879052606081018390526080810185905260a081018490526001600160a01b038c169081907fc16f4e4ca34d790de4c656c72fd015c667d688f20be64eea360618545c4c530f9060c00160405180910390a350610d24565b6000610c7a868b611f11565b9050610c878b828761181a565b6040518181526000906001600160a01b038d16907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9060200160405180910390a3604080518281526020810189905290810187905260608101859052608081018490526001600160a01b038c16907f44bd20a79e993bdcc7cbedf54a3b4d19fb78490124b6b90d04fe3242eea579e89060a00160405180910390a2505b50955093505050505b9250929050565b600080806001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016336001600160a01b03161460405180604001604052806002815260200161323360f01b81525090610da65760405162461bcd60e51b815260040161061c9190611b30565b50610de06040518060c001604052806000815260200160008152602001600081526020016000815260200160008152602001600081525090565b866001600160a01b0316886001600160a01b031614610e0457610e04878988611854565b600080610e1089611605565b9250925050610e1d610850565b808452603f546001600160801b031660a0850152610e3c908990611ef9565b603a8190556020840152610e4f88611655565b6040848101919091526001600160a01b038a16600090815260386020522054600160801b90046001600160801b03166060840152610ec8610e98610e938a85611ef9565b611655565b6040850151610ea7908a6115c1565b610ebe610eb386611655565b6060880151906115c1565b610add9190611ef9565b60808401819052610ed8906116af565b6001600160a01b038a16600090815260386020908152604080832080546001600160801b03908116600160801b969091168602179055603e8252909120805464ffffffffff19164264ffffffffff16908117909155603f805464ffffffffff60801b19169190930217909155830151610f8390610ae390610f5890611655565b6040860151610f68908b906115c1565b610ebe610f788860000151611655565b60a0890151906115c1565b603f80546001600160801b0319166001600160801b0392909216918217905560a08401526000610fb3828a611ef9565b9050610fc48a82866000015161171c565b6040518181526001600160a01b038b16906000907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef9060200160405180910390a360808085015160a080870151602080890151604080518881529283018a905282018890526060820194909452938401528201526001600160a01b03808c1691908d16907fc16f4e4ca34d790de4c656c72fd015c667d688f20be64eea360618545c4c530f9060c00160405180910390a35050602082015160a0909201519015999198509650945050505050565b61109d338383611427565b5050565b60015460039060ff16806110b45750303b155b806110c0575060005481115b6111235760405162461bcd60e51b815260206004820152602e60248201527f436f6e747261637420696e7374616e63652068617320616c726561647920626560448201526d195b881a5b9a5d1a585b1a5e995960921b606482015260840161061c565b60015460ff16158015611142576001805460ff19168117905560008290555b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168a6001600160a01b03161460405180604001604052806002815260200161383760f01b815250906111b05760405162461bcd60e51b815260040161061c9190611b30565b506111ba866118fa565b6111c38561190d565b603d8054603780546001600160a01b038d81166001600160a01b0319909216919091179091558a16610100026001600160a81b031990911660ff8a161717905561120b6114e0565b6035819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316896001600160a01b03167f40251fbfb6656cfa65a00d7879029fec1fad21d28fdcff2f4f68f52795b74f2c8a8a8a8a8a8a60405161127e96959493929190611f28565b60405180910390a38015611297576001805460ff191690555b50505050505050505050565b60006112ae82611589565b92915050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663707cd7166040518163ffffffff1660e01b8152600401602060405180830381865afa158015611314573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113389190611f9f565b604051637be53ca160e01b81523360048201529091506001600160a01b03821690637be53ca190602401602060405180830381865afa15801561137f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113a39190611fbc565b604051806040016040528060018152602001603160f81b815250906113db5760405162461bcd60e51b815260040161061c9190611b30565b5050603d80546001600160a01b0390921661010002610100600160a81b0319909216919091179055565b603f5460009081906001600160801b031661141f81611491565b939092509050565b6001600160a01b0383811660008181526036602090815260408083208786168085529083529281902086905560375490518681529416939192917fda919360433220e13b51e8c211e490d148e61a3bd53de8c097194e458b97f3e1910160405180910390a4505050565b60008061149d603a5490565b9050806114ad5750600092915050565b60006114cc84603f60109054906101000a900464ffffffffff166115ad565b90506114d882826115c1565b949350505050565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61150b611920565b805160209182012060408051808201825260018152603160f81b90840152805192830193909352918101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b6001600160a01b03166000908152603860205260409020546001600160801b031690565b60006115ba83834261192a565b9392505050565b600081156b019d971e4fe8401e7400000019839004841115176115e357600080fd5b506b033b2e3c9fd0803ce800000091026b019d971e4fe8401e74000000010490565b60008060008061161485611589565b90508061162c5760008060009350935093505061164e565b6000611637866108a6565b905081816116458282611f11565b94509450945050505b9193909250565b633b9aca00818102908104821461166b57600080fd5b919050565b600081156b033b2e3c9fd0803ce80000006002840419048411171561169457600080fd5b506b033b2e3c9fd0803ce80000009190910260028204010490565b60006001600160801b038211156117185760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b606482015260840161061c565b5090565b6000611727836116af565b6001600160a01b0385166000908152603860205260409020549091506001600160801b03166117568282611fde565b6001600160a01b03868116600090815260386020526040902080546001600160801b0319166001600160801b039390931692909217909155603d546101009004161561181357603d546040516318c39f1760e11b81526001600160a01b038781166004830152602482018690526001600160801b0384166044830152610100909204909116906331873e2e90606401600060405180830381600087803b1580156117ff57600080fd5b505af1158015610845573d6000803e3d6000fd5b5050505050565b6000611825836116af565b6001600160a01b0385166000908152603860205260409020549091506001600160801b03166117568282612009565b6001600160a01b038084166000908152603660209081526040808320938616835292905290812054611887908390611f11565b6001600160a01b03808616600081815260366020908152604080832089861680855292529182902085905560375491519495509216927fda919360433220e13b51e8c211e490d148e61a3bd53de8c097194e458b97f3e1906118ec9086815260200190565b60405180910390a450505050565b805161109d90603b906020840190611a53565b805161109d90603c906020840190611a53565b6060610868610558565b60008061193e64ffffffffff851684611f11565b90508061195a576b033b2e3c9fd0803ce80000009150506115ba565b60001981016000808060028511611972576000611977565b600285035b925066038882915c400061198b8a806115c1565b8161199857611998612031565b0491506301e133806119aa838b6115c1565b816119b7576119b7612031565b0490506000826119c78688612047565b6119d19190612047565b600290049050600082856119e5888a612047565b6119ef9190612047565b6119f99190612047565b60069004905080826301e13380611a108a8f612047565b611a1a9190612066565b611a30906b033b2e3c9fd0803ce8000000611ef9565b611a3a9190611ef9565b611a449190611ef9565b9b9a5050505050505050505050565b828054611a5f90611ea8565b90600052602060002090601f016020900481019282611a815760008555611ac7565b82601f10611a9a57805160ff1916838001178555611ac7565b82800160010185558215611ac7579182015b82811115611ac7578251825591602001919060010190611aac565b506117189291505b808211156117185760008155600101611acf565b6000815180845260005b81811015611b0957602081850181015186830182015201611aed565b81811115611b1b576000602083870101525b50601f01601f19169290920160200192915050565b6020815260006115ba6020830184611ae3565b6001600160a01b0381168114611b5857600080fd5b50565b803561166b81611b43565b60008060408385031215611b7957600080fd5b8235611b8481611b43565b946020939093013593505050565b803560ff8116811461166b57600080fd5b600080600080600080600060e0888a031215611bbe57600080fd5b8735611bc981611b43565b96506020880135611bd981611b43565b95506040880135945060608801359350611bf560808901611b92565b925060a0880135915060c0880135905092959891949750929550565b600080600060608486031215611c2657600080fd5b8335611c3181611b43565b92506020840135611c4181611b43565b929592945050506040919091013590565b60008060408385031215611c6557600080fd5b8235611c7081611b43565b91506020830135611c8081611b43565b809150509250929050565b600060208284031215611c9d57600080fd5b81356115ba81611b43565b60008060008060808587031215611cbe57600080fd5b8435611cc981611b43565b93506020850135611cd981611b43565b93969395505050506040820135916060013590565b634e487b7160e01b600052604160045260246000fd5b600082601f830112611d1557600080fd5b813567ffffffffffffffff80821115611d3057611d30611cee565b604051601f8301601f19908116603f01168101908282118183101715611d5857611d58611cee565b81604052838152866020858801011115611d7157600080fd5b836020870160208301376000602085830101528094505050505092915050565b60008083601f840112611da357600080fd5b50813567ffffffffffffffff811115611dbb57600080fd5b602083019150836020828501011115610d2d57600080fd5b60008060008060008060008060e0898b031215611def57600080fd5b8835611dfa81611b43565b97506020890135611e0a81611b43565b9650611e1860408a01611b5b565b9550611e2660608a01611b92565b9450608089013567ffffffffffffffff80821115611e4357600080fd5b611e4f8c838d01611d04565b955060a08b0135915080821115611e6557600080fd5b611e718c838d01611d04565b945060c08b0135915080821115611e8757600080fd5b50611e948b828c01611d91565b999c989b5096995094979396929594505050565b600181811c90821680611ebc57607f821691505b60208210811415611edd57634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115611f0c57611f0c611ee3565b500190565b600082821015611f2357611f23611ee3565b500390565b6001600160a01b038716815260ff8616602082015260a060408201819052600090611f5590830187611ae3565b8281036060840152611f678187611ae3565b90508281036080840152838152838560208301376000602085830101526020601f19601f860116820101915050979650505050505050565b600060208284031215611fb157600080fd5b81516115ba81611b43565b600060208284031215611fce57600080fd5b815180151581146115ba57600080fd5b60006001600160801b0380831681851680830382111561200057612000611ee3565b01949350505050565b60006001600160801b038381169083168181101561202957612029611ee3565b039392505050565b634e487b7160e01b600052601260045260246000fd5b600081600019048311821515161561206157612061611ee3565b500290565b60008261208357634e487b7160e01b600052601260045260246000fd5b50049056fea264697066735822122071d8594e004a5cf64580b390e600e2ea1f96ce723730f1310597f91314c27a1b64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct MockStableDebtToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MockStableDebtToken<M> {
        fn clone(&self) -> Self {
            MockStableDebtToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MockStableDebtToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MockStableDebtToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MockStableDebtToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MockStableDebtToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MOCKSTABLEDEBTTOKEN_ABI.clone(), client)
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MOCKSTABLEDEBTTOKEN_ABI.clone(),
                MOCKSTABLEDEBTTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DEBT_TOKEN_REVISION` (0xb9a7b622) function"]
        pub fn debt_token_revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([185, 167, 182, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DELEGATION_WITH_SIG_TYPEHASH` (0xf3bfc738) function"]
        pub fn delegation_with_sig_typehash(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([243, 191, 199, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `EIP712_REVISION` (0x78160376) function"]
        pub fn eip712_revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([120, 22, 3, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNDERLYING_ASSET_ADDRESS` (0xb16a19de) function"]
        pub fn underlying_asset_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([177, 106, 25, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveDelegation` (0xc04a8a10) function"]
        pub fn approve_delegation(
            &self,
            delegatee: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 74, 138, 16], (delegatee, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowAllowance` (0x6bd76d24) function"]
        pub fn borrow_allowance(
            &self,
            from_user: ethers::core::types::Address,
            to_user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([107, 215, 109, 36], (from_user, to_user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(
            &self,
            from: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 194, 159, 172], (from, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegationWithSig` (0x0b52d558) function"]
        pub fn delegation_with_sig(
            &self,
            delegator: ethers::core::types::Address,
            delegatee: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [11, 82, 213, 88],
                    (delegator, delegatee, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAverageStableRate` (0x90f6fcf2) function"]
        pub fn get_average_stable_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([144, 246, 252, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getIncentivesController` (0x75d26413) function"]
        pub fn get_incentives_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 210, 100, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSupplyData` (0x79774338) function"]
        pub fn get_supply_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u64,
            ),
        > {
            self.0
                .method_hash([121, 119, 67, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalSupplyAndAvgRate` (0xf731e9be) function"]
        pub fn get_total_supply_and_avg_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([247, 49, 233, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalSupplyLastUpdated` (0xe7484890) function"]
        pub fn get_total_supply_last_updated(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 72, 72, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserLastUpdated` (0x79ce6b8c) function"]
        pub fn get_user_last_updated(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([121, 206, 107, 140], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserStableRate` (0xe78c9b3b) function"]
        pub fn get_user_stable_rate(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([231, 140, 155, 59], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc222ec8a) function"]
        pub fn initialize(
            &self,
            initializing_pool: ethers::core::types::Address,
            underlying_asset: ethers::core::types::Address,
            incentives_controller: ethers::core::types::Address,
            debt_token_decimals: u8,
            debt_token_name: String,
            debt_token_symbol: String,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 34, 236, 138],
                    (
                        initializing_pool,
                        underlying_asset,
                        incentives_controller,
                        debt_token_decimals,
                        debt_token_name,
                        debt_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0xb3f1c93d) function"]
        pub fn mint(
            &self,
            user: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (bool, ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([179, 241, 201, 61], (user, on_behalf_of, amount, rate))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `principalBalanceOf` (0xc634dfaa) function"]
        pub fn principal_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 52, 223, 170], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIncentivesController` (0xe655dbd8) function"]
        pub fn set_incentives_controller(
            &self,
            controller: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 85, 219, 216], controller)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowAllowanceDelegated` event"]
        pub fn borrow_allowance_delegated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowAllowanceDelegatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Burn` event"]
        pub fn burn_filter(&self) -> ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Mint` event"]
        pub fn mint_filter(&self) -> ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MockStableDebtTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MockStableDebtToken<M>
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "BorrowAllowanceDelegated",
        abi = "BorrowAllowanceDelegated(address,address,address,uint256)"
    )]
    pub struct BorrowAllowanceDelegatedFilter {
        #[ethevent(indexed)]
        pub from_user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        name = "Burn",
        abi = "Burn(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub current_balance: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub avg_stable_rate: ethers::core::types::U256,
        pub new_total_supply: ethers::core::types::U256,
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
        name = "Initialized",
        abi = "Initialized(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: String,
        pub debt_token_symbol: String,
        pub params: ethers::core::types::Bytes,
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
        name = "Mint",
        abi = "Mint(address,address,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub current_balance: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub new_rate: ethers::core::types::U256,
        pub avg_stable_rate: ethers::core::types::U256,
        pub new_total_supply: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockStableDebtTokenEvents {
        ApprovalFilter(ApprovalFilter),
        BorrowAllowanceDelegatedFilter(BorrowAllowanceDelegatedFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for MockStableDebtTokenEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowAllowanceDelegatedFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::BorrowAllowanceDelegatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockStableDebtTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MockStableDebtTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockStableDebtTokenEvents::ApprovalFilter(element) => element.fmt(f),
                MockStableDebtTokenEvents::BorrowAllowanceDelegatedFilter(element) => {
                    element.fmt(f)
                }
                MockStableDebtTokenEvents::BurnFilter(element) => element.fmt(f),
                MockStableDebtTokenEvents::InitializedFilter(element) => element.fmt(f),
                MockStableDebtTokenEvents::MintFilter(element) => element.fmt(f),
                MockStableDebtTokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEBT_TOKEN_REVISION` function with signature `DEBT_TOKEN_REVISION()` and selector `[185, 167, 182, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEBT_TOKEN_REVISION", abi = "DEBT_TOKEN_REVISION()")]
    pub struct DebtTokenRevisionCall;
    #[doc = "Container type for all input parameters for the `DELEGATION_WITH_SIG_TYPEHASH` function with signature `DELEGATION_WITH_SIG_TYPEHASH()` and selector `[243, 191, 199, 56]`"]
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
        name = "DELEGATION_WITH_SIG_TYPEHASH",
        abi = "DELEGATION_WITH_SIG_TYPEHASH()"
    )]
    pub struct DelegationWithSigTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `EIP712_REVISION` function with signature `EIP712_REVISION()` and selector `[120, 22, 3, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "EIP712_REVISION", abi = "EIP712_REVISION()")]
    pub struct Eip712RevisionCall;
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
    #[doc = "Container type for all input parameters for the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "UNDERLYING_ASSET_ADDRESS", abi = "UNDERLYING_ASSET_ADDRESS()")]
    pub struct UnderlyingAssetAddressCall;
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `approveDelegation` function with signature `approveDelegation(address,uint256)` and selector `[192, 74, 138, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approveDelegation", abi = "approveDelegation(address,uint256)")]
    pub struct ApproveDelegationCall {
        pub delegatee: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowAllowance` function with signature `borrowAllowance(address,address)` and selector `[107, 215, 109, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowAllowance", abi = "borrowAllowance(address,address)")]
    pub struct BorrowAllowanceCall {
        pub from_user: ethers::core::types::Address,
        pub to_user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `delegationWithSig` function with signature `delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[11, 82, 213, 88]`"]
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
        name = "delegationWithSig",
        abi = "delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegationWithSigCall {
        pub delegator: ethers::core::types::Address,
        pub delegatee: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `[144, 246, 252, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAverageStableRate", abi = "getAverageStableRate()")]
    pub struct GetAverageStableRateCall;
    #[doc = "Container type for all input parameters for the `getIncentivesController` function with signature `getIncentivesController()` and selector `[117, 210, 100, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getIncentivesController", abi = "getIncentivesController()")]
    pub struct GetIncentivesControllerCall;
    #[doc = "Container type for all input parameters for the `getSupplyData` function with signature `getSupplyData()` and selector `[121, 119, 67, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getSupplyData", abi = "getSupplyData()")]
    pub struct GetSupplyDataCall;
    #[doc = "Container type for all input parameters for the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `[247, 49, 233, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTotalSupplyAndAvgRate", abi = "getTotalSupplyAndAvgRate()")]
    pub struct GetTotalSupplyAndAvgRateCall;
    #[doc = "Container type for all input parameters for the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `[231, 72, 72, 144]`"]
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
        name = "getTotalSupplyLastUpdated",
        abi = "getTotalSupplyLastUpdated()"
    )]
    pub struct GetTotalSupplyLastUpdatedCall;
    #[doc = "Container type for all input parameters for the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `[121, 206, 107, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserLastUpdated", abi = "getUserLastUpdated(address)")]
    pub struct GetUserLastUpdatedCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `[231, 140, 155, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserStableRate", abi = "getUserStableRate(address)")]
    pub struct GetUserStableRateCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint8,string,string,bytes)` and selector `[194, 34, 236, 138]`"]
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
        name = "initialize",
        abi = "initialize(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub initializing_pool: ethers::core::types::Address,
        pub underlying_asset: ethers::core::types::Address,
        pub incentives_controller: ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: String,
        pub debt_token_symbol: String,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,address,uint256,uint256)")]
    pub struct MintCall {
        pub user: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `[198, 52, 223, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "principalBalanceOf", abi = "principalBalanceOf(address)")]
    pub struct PrincipalBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setIncentivesController` function with signature `setIncentivesController(address)` and selector `[230, 85, 219, 216]`"]
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
        name = "setIncentivesController",
        abi = "setIncentivesController(address)"
    )]
    pub struct SetIncentivesControllerCall {
        pub controller: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MockStableDebtTokenCalls {
        DebtTokenRevision(DebtTokenRevisionCall),
        DelegationWithSigTypehash(DelegationWithSigTypehashCall),
        DomainSeparator(DomainSeparatorCall),
        Eip712Revision(Eip712RevisionCall),
        Pool(PoolCall),
        UnderlyingAssetAddress(UnderlyingAssetAddressCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveDelegation(ApproveDelegationCall),
        BalanceOf(BalanceOfCall),
        BorrowAllowance(BorrowAllowanceCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DelegationWithSig(DelegationWithSigCall),
        GetAverageStableRate(GetAverageStableRateCall),
        GetIncentivesController(GetIncentivesControllerCall),
        GetSupplyData(GetSupplyDataCall),
        GetTotalSupplyAndAvgRate(GetTotalSupplyAndAvgRateCall),
        GetTotalSupplyLastUpdated(GetTotalSupplyLastUpdatedCall),
        GetUserLastUpdated(GetUserLastUpdatedCall),
        GetUserStableRate(GetUserStableRateCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        PrincipalBalanceOf(PrincipalBalanceOfCall),
        SetIncentivesController(SetIncentivesControllerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for MockStableDebtTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DebtTokenRevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::DebtTokenRevision(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigTypehashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockStableDebtTokenCalls::DelegationWithSigTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <Eip712RevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Eip712Revision(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockStableDebtTokenCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::UnderlyingAssetAddress(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveDelegationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::ApproveDelegation(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::BorrowAllowance(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockStableDebtTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::DelegationWithSig(decoded));
            }
            if let Ok(decoded) =
                <GetAverageStableRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::GetAverageStableRate(decoded));
            }
            if let Ok(decoded) =
                <GetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::GetIncentivesController(decoded));
            }
            if let Ok(decoded) =
                <GetSupplyDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::GetSupplyData(decoded));
            }
            if let Ok(decoded) =
                <GetTotalSupplyAndAvgRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockStableDebtTokenCalls::GetTotalSupplyAndAvgRate(decoded));
            }
            if let Ok(decoded) =
                <GetTotalSupplyLastUpdatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MockStableDebtTokenCalls::GetTotalSupplyLastUpdated(decoded));
            }
            if let Ok(decoded) =
                <GetUserLastUpdatedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::GetUserLastUpdated(decoded));
            }
            if let Ok(decoded) =
                <GetUserStableRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::GetUserStableRate(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockStableDebtTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MockStableDebtTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <PrincipalBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::PrincipalBalanceOf(decoded));
            }
            if let Ok(decoded) =
                <SetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::SetIncentivesController(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MockStableDebtTokenCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MockStableDebtTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MockStableDebtTokenCalls::DebtTokenRevision(element) => element.encode(),
                MockStableDebtTokenCalls::DelegationWithSigTypehash(element) => element.encode(),
                MockStableDebtTokenCalls::DomainSeparator(element) => element.encode(),
                MockStableDebtTokenCalls::Eip712Revision(element) => element.encode(),
                MockStableDebtTokenCalls::Pool(element) => element.encode(),
                MockStableDebtTokenCalls::UnderlyingAssetAddress(element) => element.encode(),
                MockStableDebtTokenCalls::Allowance(element) => element.encode(),
                MockStableDebtTokenCalls::Approve(element) => element.encode(),
                MockStableDebtTokenCalls::ApproveDelegation(element) => element.encode(),
                MockStableDebtTokenCalls::BalanceOf(element) => element.encode(),
                MockStableDebtTokenCalls::BorrowAllowance(element) => element.encode(),
                MockStableDebtTokenCalls::Burn(element) => element.encode(),
                MockStableDebtTokenCalls::Decimals(element) => element.encode(),
                MockStableDebtTokenCalls::DecreaseAllowance(element) => element.encode(),
                MockStableDebtTokenCalls::DelegationWithSig(element) => element.encode(),
                MockStableDebtTokenCalls::GetAverageStableRate(element) => element.encode(),
                MockStableDebtTokenCalls::GetIncentivesController(element) => element.encode(),
                MockStableDebtTokenCalls::GetSupplyData(element) => element.encode(),
                MockStableDebtTokenCalls::GetTotalSupplyAndAvgRate(element) => element.encode(),
                MockStableDebtTokenCalls::GetTotalSupplyLastUpdated(element) => element.encode(),
                MockStableDebtTokenCalls::GetUserLastUpdated(element) => element.encode(),
                MockStableDebtTokenCalls::GetUserStableRate(element) => element.encode(),
                MockStableDebtTokenCalls::IncreaseAllowance(element) => element.encode(),
                MockStableDebtTokenCalls::Initialize(element) => element.encode(),
                MockStableDebtTokenCalls::Mint(element) => element.encode(),
                MockStableDebtTokenCalls::Name(element) => element.encode(),
                MockStableDebtTokenCalls::Nonces(element) => element.encode(),
                MockStableDebtTokenCalls::PrincipalBalanceOf(element) => element.encode(),
                MockStableDebtTokenCalls::SetIncentivesController(element) => element.encode(),
                MockStableDebtTokenCalls::Symbol(element) => element.encode(),
                MockStableDebtTokenCalls::TotalSupply(element) => element.encode(),
                MockStableDebtTokenCalls::Transfer(element) => element.encode(),
                MockStableDebtTokenCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MockStableDebtTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MockStableDebtTokenCalls::DebtTokenRevision(element) => element.fmt(f),
                MockStableDebtTokenCalls::DelegationWithSigTypehash(element) => element.fmt(f),
                MockStableDebtTokenCalls::DomainSeparator(element) => element.fmt(f),
                MockStableDebtTokenCalls::Eip712Revision(element) => element.fmt(f),
                MockStableDebtTokenCalls::Pool(element) => element.fmt(f),
                MockStableDebtTokenCalls::UnderlyingAssetAddress(element) => element.fmt(f),
                MockStableDebtTokenCalls::Allowance(element) => element.fmt(f),
                MockStableDebtTokenCalls::Approve(element) => element.fmt(f),
                MockStableDebtTokenCalls::ApproveDelegation(element) => element.fmt(f),
                MockStableDebtTokenCalls::BalanceOf(element) => element.fmt(f),
                MockStableDebtTokenCalls::BorrowAllowance(element) => element.fmt(f),
                MockStableDebtTokenCalls::Burn(element) => element.fmt(f),
                MockStableDebtTokenCalls::Decimals(element) => element.fmt(f),
                MockStableDebtTokenCalls::DecreaseAllowance(element) => element.fmt(f),
                MockStableDebtTokenCalls::DelegationWithSig(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetAverageStableRate(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetIncentivesController(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetSupplyData(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetTotalSupplyAndAvgRate(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetTotalSupplyLastUpdated(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetUserLastUpdated(element) => element.fmt(f),
                MockStableDebtTokenCalls::GetUserStableRate(element) => element.fmt(f),
                MockStableDebtTokenCalls::IncreaseAllowance(element) => element.fmt(f),
                MockStableDebtTokenCalls::Initialize(element) => element.fmt(f),
                MockStableDebtTokenCalls::Mint(element) => element.fmt(f),
                MockStableDebtTokenCalls::Name(element) => element.fmt(f),
                MockStableDebtTokenCalls::Nonces(element) => element.fmt(f),
                MockStableDebtTokenCalls::PrincipalBalanceOf(element) => element.fmt(f),
                MockStableDebtTokenCalls::SetIncentivesController(element) => element.fmt(f),
                MockStableDebtTokenCalls::Symbol(element) => element.fmt(f),
                MockStableDebtTokenCalls::TotalSupply(element) => element.fmt(f),
                MockStableDebtTokenCalls::Transfer(element) => element.fmt(f),
                MockStableDebtTokenCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DebtTokenRevisionCall> for MockStableDebtTokenCalls {
        fn from(var: DebtTokenRevisionCall) -> Self {
            MockStableDebtTokenCalls::DebtTokenRevision(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigTypehashCall> for MockStableDebtTokenCalls {
        fn from(var: DelegationWithSigTypehashCall) -> Self {
            MockStableDebtTokenCalls::DelegationWithSigTypehash(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for MockStableDebtTokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            MockStableDebtTokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<Eip712RevisionCall> for MockStableDebtTokenCalls {
        fn from(var: Eip712RevisionCall) -> Self {
            MockStableDebtTokenCalls::Eip712Revision(var)
        }
    }
    impl ::std::convert::From<PoolCall> for MockStableDebtTokenCalls {
        fn from(var: PoolCall) -> Self {
            MockStableDebtTokenCalls::Pool(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetAddressCall> for MockStableDebtTokenCalls {
        fn from(var: UnderlyingAssetAddressCall) -> Self {
            MockStableDebtTokenCalls::UnderlyingAssetAddress(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for MockStableDebtTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            MockStableDebtTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for MockStableDebtTokenCalls {
        fn from(var: ApproveCall) -> Self {
            MockStableDebtTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveDelegationCall> for MockStableDebtTokenCalls {
        fn from(var: ApproveDelegationCall) -> Self {
            MockStableDebtTokenCalls::ApproveDelegation(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MockStableDebtTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            MockStableDebtTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowAllowanceCall> for MockStableDebtTokenCalls {
        fn from(var: BorrowAllowanceCall) -> Self {
            MockStableDebtTokenCalls::BorrowAllowance(var)
        }
    }
    impl ::std::convert::From<BurnCall> for MockStableDebtTokenCalls {
        fn from(var: BurnCall) -> Self {
            MockStableDebtTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for MockStableDebtTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            MockStableDebtTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for MockStableDebtTokenCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            MockStableDebtTokenCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigCall> for MockStableDebtTokenCalls {
        fn from(var: DelegationWithSigCall) -> Self {
            MockStableDebtTokenCalls::DelegationWithSig(var)
        }
    }
    impl ::std::convert::From<GetAverageStableRateCall> for MockStableDebtTokenCalls {
        fn from(var: GetAverageStableRateCall) -> Self {
            MockStableDebtTokenCalls::GetAverageStableRate(var)
        }
    }
    impl ::std::convert::From<GetIncentivesControllerCall> for MockStableDebtTokenCalls {
        fn from(var: GetIncentivesControllerCall) -> Self {
            MockStableDebtTokenCalls::GetIncentivesController(var)
        }
    }
    impl ::std::convert::From<GetSupplyDataCall> for MockStableDebtTokenCalls {
        fn from(var: GetSupplyDataCall) -> Self {
            MockStableDebtTokenCalls::GetSupplyData(var)
        }
    }
    impl ::std::convert::From<GetTotalSupplyAndAvgRateCall> for MockStableDebtTokenCalls {
        fn from(var: GetTotalSupplyAndAvgRateCall) -> Self {
            MockStableDebtTokenCalls::GetTotalSupplyAndAvgRate(var)
        }
    }
    impl ::std::convert::From<GetTotalSupplyLastUpdatedCall> for MockStableDebtTokenCalls {
        fn from(var: GetTotalSupplyLastUpdatedCall) -> Self {
            MockStableDebtTokenCalls::GetTotalSupplyLastUpdated(var)
        }
    }
    impl ::std::convert::From<GetUserLastUpdatedCall> for MockStableDebtTokenCalls {
        fn from(var: GetUserLastUpdatedCall) -> Self {
            MockStableDebtTokenCalls::GetUserLastUpdated(var)
        }
    }
    impl ::std::convert::From<GetUserStableRateCall> for MockStableDebtTokenCalls {
        fn from(var: GetUserStableRateCall) -> Self {
            MockStableDebtTokenCalls::GetUserStableRate(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for MockStableDebtTokenCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            MockStableDebtTokenCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MockStableDebtTokenCalls {
        fn from(var: InitializeCall) -> Self {
            MockStableDebtTokenCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MintCall> for MockStableDebtTokenCalls {
        fn from(var: MintCall) -> Self {
            MockStableDebtTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for MockStableDebtTokenCalls {
        fn from(var: NameCall) -> Self {
            MockStableDebtTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for MockStableDebtTokenCalls {
        fn from(var: NoncesCall) -> Self {
            MockStableDebtTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PrincipalBalanceOfCall> for MockStableDebtTokenCalls {
        fn from(var: PrincipalBalanceOfCall) -> Self {
            MockStableDebtTokenCalls::PrincipalBalanceOf(var)
        }
    }
    impl ::std::convert::From<SetIncentivesControllerCall> for MockStableDebtTokenCalls {
        fn from(var: SetIncentivesControllerCall) -> Self {
            MockStableDebtTokenCalls::SetIncentivesController(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for MockStableDebtTokenCalls {
        fn from(var: SymbolCall) -> Self {
            MockStableDebtTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for MockStableDebtTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            MockStableDebtTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for MockStableDebtTokenCalls {
        fn from(var: TransferCall) -> Self {
            MockStableDebtTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for MockStableDebtTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            MockStableDebtTokenCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `DEBT_TOKEN_REVISION` function with signature `DEBT_TOKEN_REVISION()` and selector `[185, 167, 182, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DebtTokenRevisionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `DELEGATION_WITH_SIG_TYPEHASH` function with signature `DELEGATION_WITH_SIG_TYPEHASH()` and selector `[243, 191, 199, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DelegationWithSigTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `EIP712_REVISION` function with signature `EIP712_REVISION()` and selector `[120, 22, 3, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Eip712RevisionReturn(pub ethers::core::types::Bytes);
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
    #[doc = "Container type for all return fields from the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UnderlyingAssetAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `borrowAllowance` function with signature `borrowAllowance(address,address)` and selector `[107, 215, 109, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BorrowAllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `burn` function with signature `burn(address,uint256)` and selector `[157, 194, 159, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BurnReturn(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `[144, 246, 252, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAverageStableRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getIncentivesController` function with signature `getIncentivesController()` and selector `[117, 210, 100, 19]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetIncentivesControllerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getSupplyData` function with signature `getSupplyData()` and selector `[121, 119, 67, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSupplyDataReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub u64,
    );
    #[doc = "Container type for all return fields from the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `[247, 49, 233, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTotalSupplyAndAvgRateReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `[231, 72, 72, 144]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTotalSupplyLastUpdatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `[121, 206, 107, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserLastUpdatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `[231, 140, 155, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserStableRateReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MintReturn(
        pub bool,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NoncesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `[198, 52, 223, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PrincipalBalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TransferFromReturn(pub bool);
}
