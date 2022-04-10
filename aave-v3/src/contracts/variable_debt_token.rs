pub use variabledebttoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod variabledebttoken_mod {
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
    #[doc = "VariableDebtToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VARIABLEDEBTTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowAllowanceDelegated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"balanceIncrease\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEBT_TOKEN_REVISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_WITH_SIG_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EIP712_REVISION\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_ASSET_ADDRESS\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveDelegation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowAllowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegationWithSig\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getIncentivesController\",\"outputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPreviousIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getScaledUserBalanceAndSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IPool\",\"name\":\"initializingPool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"debtTokenDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"debtTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"scaledTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IAaveIncentivesController\",\"name\":\"controller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setIncentivesController\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static VARIABLEDEBTTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e0604052600080553480156200001557600080fd5b5060405162001fce38038062001fce833981016040819052620000389162000245565b806040518060400160405280601881526020017f5641524941424c455f444542545f544f4b454e5f494d504c00000000000000008152506040518060400160405280601881526020017f5641524941424c455f444542545f544f4b454e5f494d504c0000000000000000815250600083838383838383834660808181525050836001600160a01b0316630542975c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000f6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200011c919062000245565b6001600160a01b031660a05282516200013d90603b90602086019062000186565b5081516200015390603c90602085019062000186565b50603d805460ff191660ff9290921691909117905550506001600160a01b031660c05250620002a9975050505050505050565b82805462000194906200026c565b90600052602060002090601f016020900481019282620001b8576000855562000203565b82601f10620001d357805160ff191683800117855562000203565b8280016001018555821562000203579182015b8281111562000203578251825591602001919060010190620001e6565b506200021192915062000215565b5090565b5b8082111562000211576000815560010162000216565b6001600160a01b03811681146200024257600080fd5b50565b6000602082840312156200025857600080fd5b815162000265816200022c565b9392505050565b600181811c908216806200028157607f821691505b60208210811415620002a357634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c051611ccb6200030360003960008181610302015281816107e1015281816108e80152818161099c01528181610ad801528181610ba70152610da701526000610c3b015260006108710152611ccb6000f3fe608060405234801561001057600080fd5b50600436106101da5760003560e01c80637ecebe0011610104578063b9a7b622116100a2578063e075398611610071578063e075398614610422578063e655dbd81461045b578063f3bfc7381461046e578063f5298aca1461049557600080fd5b8063b9a7b622146103e6578063c04a8a10146103ee578063c222ec8a14610401578063dd62ed3e1461041457600080fd5b8063a9059cbb116100de578063a9059cbb146101fd578063b16a19de146103a3578063b1bf962d146103b4578063b3f1c93d146103bc57600080fd5b80637ecebe001461037257806395d89b411461039b578063a457c2d7146101fd57600080fd5b8063313ce5671161017c57806370a082311161014b57806370a08231146102ea5780637535d246146102fd57806375d264131461033c578063781603761461035257600080fd5b8063313ce567146102945780633644e515146102a957806339509351146101fd5780636bd76d24146102b157600080fd5b80630b52d558116101b85780630b52d5581461024857806318160ddd1461025d5780631da24f3e1461027357806323b872dd1461028657600080fd5b806306fdde03146101df578063095ea7b3146101fd5780630afbcdc914610220575b600080fd5b6101e76104a8565b6040516101f4919061173a565b60405180910390f35b61021061020b366004611775565b61053a565b60405190151581526020016101f4565b61023361022e3660046117a1565b610575565b604080519283526020830191909152016101f4565b61025b6102563660046117cf565b61058d565b005b6102656107b8565b6040519081526020016101f4565b6102656102813660046117a1565b61085c565b61021061020b36600461183d565b603d5460405160ff90911681526020016101f4565b61026561086d565b6102656102bf36600461187e565b6001600160a01b03918216600090815260366020908152604080832093909416825291909152205490565b6102656102f83660046117a1565b6108a6565b6103247f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101f4565b603d5461010090046001600160a01b0316610324565b6101e7604051806040016040528060018152602001603160f81b81525081565b6102656103803660046117a1565b6001600160a01b031660009081526034602052604090205490565b6101e7610961565b6037546001600160a01b0316610324565b610265610970565b6103cf6103ca3660046118b7565b61097b565b6040805192151583526020830191909152016101f4565b610265600181565b61025b6103fc366004611775565b610a28565b61025b61040f3660046119e9565b610a37565b61026561020b36600461187e565b6102656104303660046117a1565b6001600160a01b0316600090815260386020526040902054600160801b90046001600160801b031690565b61025b6104693660046117a1565b610c37565b6102657f323db0410fecc107e39e2af5908671f4c8d106123b35a51501bb805c5fa36aa081565b6102656104a3366004611abe565b610d88565b6060603b80546104b790611af3565b80601f01602080910402602001604051908101604052809291908181526020018280546104e390611af3565b80156105305780601f1061050557610100808354040283529160200191610530565b820191906000526020600020905b81548152906001019060200180831161051357829003601f168201915b5050505050905090565b6040805180820182526002815261038360f41b6020820152905162461bcd60e51b815260009161056c9160040161173a565b60405180910390fd5b60008061058183610e0b565b603a5491509150915091565b604080518082019091526002815261373760f01b60208201526001600160a01b0388166105cd5760405162461bcd60e51b815260040161056c919061173a565b50834211156040518060400160405280600281526020016106e760f31b8152509061060b5760405162461bcd60e51b815260040161056c919061173a565b506001600160a01b0387166000908152603460205260408120549061062e61086d565b604080517f323db0410fecc107e39e2af5908671f4c8d106123b35a51501bb805c5fa36aa060208201526001600160a01b038b1691810191909152606081018990526080810184905260a0810188905260c001604051602081830303815290604052805190602001206040516020016106be92919061190160f01b81526002810192909252602282015260420190565b60408051601f1981840301815282825280516020918201206000845290830180835281905260ff8816918301919091526060820186905260808201859052915060019060a0016020604051602081039080840390855afa158015610726573d6000803e3d6000fd5b505050602060405103516001600160a01b0316896001600160a01b03161460405180604001604052806002815260200161373960f01b8152509061077d5760405162461bcd60e51b815260040161056c919061173a565b50610789826001611b44565b6001600160a01b038a166000908152603460205260409020556107ad898989610e2f565b505050505050505050565b60375460405163386497fd60e01b81526001600160a01b039182166004820152600091610857917f00000000000000000000000000000000000000000000000000000000000000009091169063386497fd90602401602060405180830381865afa15801561082a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061084e9190611b5c565b603a5490610e99565b905090565b600061086782610e0b565b92915050565b60007f000000000000000000000000000000000000000000000000000000000000000046141561089e575060355490565b610857610edd565b6000806108b283610e0b565b9050806108c25750600092915050565b60375460405163386497fd60e01b81526001600160a01b03918216600482015261095a917f0000000000000000000000000000000000000000000000000000000000000000169063386497fd90602401602060405180830381865afa15801561092f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109539190611b5c565b8290610e99565b9392505050565b6060603c80546104b790611af3565b6000610857603a5490565b604080518082019091526002815261323360f01b60208201526000908190337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316146109e25760405162461bcd60e51b815260040161056c919061173a565b50846001600160a01b0316866001600160a01b031614610a0757610a07858786610f86565b610a138686868661102c565b610a1b610970565b9150915094509492505050565b610a33338383610e2f565b5050565b6001805460ff1680610a485750303b155b80610a54575060005481115b610ab75760405162461bcd60e51b815260206004820152602e60248201527f436f6e747261637420696e7374616e63652068617320616c726561647920626560448201526d195b881a5b9a5d1a585b1a5e995960921b606482015260840161056c565b60015460ff16158015610ad6576001805460ff19168117905560008290555b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168a6001600160a01b03161460405180604001604052806002815260200161383760f01b81525090610b445760405162461bcd60e51b815260040161056c919061173a565b50610b4e866111d5565b610b57856111e8565b603d8054603780546001600160a01b038d81166001600160a01b0319909216919091179091558a16610100026001600160a81b031990911660ff8a1617179055610b9f610edd565b6035819055507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316896001600160a01b03167f40251fbfb6656cfa65a00d7879029fec1fad21d28fdcff2f4f68f52795b74f2c8a8a8a8a8a8a604051610c1296959493929190611b75565b60405180910390a38015610c2b576001805460ff191690555b50505050505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663707cd7166040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c97573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cbb9190611bec565b604051637be53ca160e01b81523360048201529091506001600160a01b03821690637be53ca190602401602060405180830381865afa158015610d02573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d269190611c09565b604051806040016040528060018152602001603160f81b81525090610d5e5760405162461bcd60e51b815260040161056c919061173a565b5050603d80546001600160a01b0390921661010002610100600160a81b0319909216919091179055565b604080518082019091526002815261323360f01b6020820152600090337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614610ded5760405162461bcd60e51b815260040161056c919061173a565b50610dfb84600085856111fb565b610e03610970565b949350505050565b6001600160a01b03166000908152603860205260409020546001600160801b031690565b6001600160a01b0383811660008181526036602090815260408083208786168085529083529281902086905560375490518681529416939192917fda919360433220e13b51e8c211e490d148e61a3bd53de8c097194e458b97f3e1910160405180910390a4505050565b600081156b019d971e4fe8401e740000001983900484111517610ebb57600080fd5b506b033b2e3c9fd0803ce800000091026b019d971e4fe8401e74000000010490565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f610f08611459565b805160209182012060408051808201825260018152603160f81b90840152805192830193909352918101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b6001600160a01b038084166000908152603660209081526040808320938616835292905290812054610fb9908390611c2b565b6001600160a01b03808616600081815260366020908152604080832089861680855292529182902085905560375491519495509216927fda919360433220e13b51e8c211e490d148e61a3bd53de8c097194e458b97f3e19061101e9086815260200190565b60405180910390a450505050565b6000806110398484611463565b6040805180820190915260028152610c8d60f21b6020820152909150816110735760405162461bcd60e51b815260040161056c919061173a565b50600061107f86610e0b565b6001600160a01b038716600090815260386020526040812054919250906110b7908390600160801b90046001600160801b0316610e99565b6110c18387610e99565b6110cb9190611c2b565b90506110d6856114a2565b6001600160a01b038816600090815260386020526040902080546001600160801b03928316600160801b02921691909117905561111b87611116856114a2565b61150f565b60006111278288611b44565b9050876001600160a01b031660006001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161116f91815260200190565b60405180910390a360408051828152602081018490529081018790526001600160a01b03808a1691908b16907f458f5fa412d0f69b08dd84872b0215675cc67bc1d5b6fd93300a1c3878b861969060600160405180910390a35050159695505050505050565b8051610a3390603b90602084019061165d565b8051610a3390603c90602084019061165d565b60006112078383611463565b604080518082019091526002815261323560f01b6020820152909150816112415760405162461bcd60e51b815260040161056c919061173a565b50600061124d86610e0b565b6001600160a01b03871660009081526038602052604081205491925090611285908390600160801b90046001600160801b0316610e99565b61128f8386610e99565b6112999190611c2b565b90506112a4846114a2565b6001600160a01b038816600090815260386020526040902080546001600160801b03928316600160801b0292169190911790556112e9876112e4856114a2565b611618565b848111156113a15760006112fd8683611c2b565b9050876001600160a01b031660006001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161134591815260200190565b60405180910390a360408051828152602081018490529081018690526001600160a01b0389169081907f458f5fa412d0f69b08dd84872b0215675cc67bc1d5b6fd93300a1c3878b861969060600160405180910390a350611450565b60006113ad8287611c2b565b905060006001600160a01b0316886001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040516113f591815260200190565b60405180910390a360408051828152602081018490529081018690526001600160a01b0380891691908a16907f4cf25bc1d991c17529c25213d3cc0cda295eeaad5f13f361969b12ea48015f909060600160405180910390a3505b50505050505050565b60606108576104a8565b600081156b033b2e3c9fd0803ce80000006002840419048411171561148757600080fd5b506b033b2e3c9fd0803ce80000009190910260028204010490565b60006001600160801b0382111561150b5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b606482015260840161056c565b5090565b603a546115256001600160801b03831682611b44565b603a556001600160a01b0383166000908152603860205260409020546001600160801b03166115548382611c42565b6001600160a01b03858116600090815260386020526040902080546fffffffffffffffffffffffffffffffff19166001600160801b039390931692909217909155603d546101009004168015611611576040516318c39f1760e11b81526001600160a01b038681166004830152602482018590526001600160801b03841660448301528216906331873e2e90606401600060405180830381600087803b1580156115fd57600080fd5b505af11580156107ad573d6000803e3d6000fd5b5050505050565b603a5461162e6001600160801b03831682611c2b565b603a556001600160a01b0383166000908152603860205260409020546001600160801b03166115548382611c6d565b82805461166990611af3565b90600052602060002090601f01602090048101928261168b57600085556116d1565b82601f106116a457805160ff19168380011785556116d1565b828001600101855582156116d1579182015b828111156116d15782518255916020019190600101906116b6565b5061150b9291505b8082111561150b57600081556001016116d9565b6000815180845260005b81811015611713576020818501810151868301820152016116f7565b81811115611725576000602083870101525b50601f01601f19169290920160200192915050565b60208152600061095a60208301846116ed565b6001600160a01b038116811461176257600080fd5b50565b80356117708161174d565b919050565b6000806040838503121561178857600080fd5b82356117938161174d565b946020939093013593505050565b6000602082840312156117b357600080fd5b813561095a8161174d565b803560ff8116811461177057600080fd5b600080600080600080600060e0888a0312156117ea57600080fd5b87356117f58161174d565b965060208801356118058161174d565b95506040880135945060608801359350611821608089016117be565b925060a0880135915060c0880135905092959891949750929550565b60008060006060848603121561185257600080fd5b833561185d8161174d565b9250602084013561186d8161174d565b929592945050506040919091013590565b6000806040838503121561189157600080fd5b823561189c8161174d565b915060208301356118ac8161174d565b809150509250929050565b600080600080608085870312156118cd57600080fd5b84356118d88161174d565b935060208501356118e88161174d565b93969395505050506040820135916060013590565b634e487b7160e01b600052604160045260246000fd5b600082601f83011261192457600080fd5b813567ffffffffffffffff8082111561193f5761193f6118fd565b604051601f8301601f19908116603f01168101908282118183101715611967576119676118fd565b8160405283815286602085880101111561198057600080fd5b836020870160208301376000602085830101528094505050505092915050565b60008083601f8401126119b257600080fd5b50813567ffffffffffffffff8111156119ca57600080fd5b6020830191508360208285010111156119e257600080fd5b9250929050565b60008060008060008060008060e0898b031215611a0557600080fd5b8835611a108161174d565b97506020890135611a208161174d565b9650611a2e60408a01611765565b9550611a3c60608a016117be565b9450608089013567ffffffffffffffff80821115611a5957600080fd5b611a658c838d01611913565b955060a08b0135915080821115611a7b57600080fd5b611a878c838d01611913565b945060c08b0135915080821115611a9d57600080fd5b50611aaa8b828c016119a0565b999c989b5096995094979396929594505050565b600080600060608486031215611ad357600080fd5b8335611ade8161174d565b95602085013595506040909401359392505050565b600181811c90821680611b0757607f821691505b60208210811415611b2857634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115611b5757611b57611b2e565b500190565b600060208284031215611b6e57600080fd5b5051919050565b6001600160a01b038716815260ff8616602082015260a060408201819052600090611ba2908301876116ed565b8281036060840152611bb481876116ed565b90508281036080840152838152838560208301376000602085830101526020601f19601f860116820101915050979650505050505050565b600060208284031215611bfe57600080fd5b815161095a8161174d565b600060208284031215611c1b57600080fd5b8151801515811461095a57600080fd5b600082821015611c3d57611c3d611b2e565b500390565b60006001600160801b03808316818516808303821115611c6457611c64611b2e565b01949350505050565b60006001600160801b0383811690831681811015611c8d57611c8d611b2e565b03939250505056fea264697066735822122026805bb19c3f45d50e05ec370a3a76cd8fbf81d03f526f5b56d712bb86bc8bd964736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct VariableDebtToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for VariableDebtToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VariableDebtToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VariableDebtToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> VariableDebtToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VARIABLEDEBTTOKEN_ABI.clone(), client)
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
                VARIABLEDEBTTOKEN_ABI.clone(),
                VARIABLEDEBTTOKEN_BYTECODE.clone().into(),
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
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], user)
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
        #[doc = "Calls the contract's `burn` (0xf5298aca) function"]
        pub fn burn(
            &self,
            from: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 41, 138, 202], (from, amount, index))
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
        #[doc = "Calls the contract's `getIncentivesController` (0x75d26413) function"]
        pub fn get_incentives_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 210, 100, 19], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPreviousIndex` (0xe0753986) function"]
        pub fn get_previous_index(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([224, 117, 57, 134], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getScaledUserBalanceAndSupply` (0x0afbcdc9) function"]
        pub fn get_scaled_user_balance_and_supply(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 251, 205, 201], user)
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
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::U256)>
        {
            self.0
                .method_hash([179, 241, 201, 61], (user, on_behalf_of, amount, index))
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
        #[doc = "Calls the contract's `scaledBalanceOf` (0x1da24f3e) function"]
        pub fn scaled_balance_of(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 162, 79, 62], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scaledTotalSupply` (0xb1bf962d) function"]
        pub fn scaled_total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 191, 150, 45], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, VariableDebtTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for VariableDebtToken<M>
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub balance_increase: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
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
    pub enum VariableDebtTokenEvents {
        ApprovalFilter(ApprovalFilter),
        BorrowAllowanceDelegatedFilter(BorrowAllowanceDelegatedFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for VariableDebtTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowAllowanceDelegatedFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::BorrowAllowanceDelegatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(VariableDebtTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VariableDebtTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VariableDebtTokenEvents::ApprovalFilter(element) => element.fmt(f),
                VariableDebtTokenEvents::BorrowAllowanceDelegatedFilter(element) => element.fmt(f),
                VariableDebtTokenEvents::BurnFilter(element) => element.fmt(f),
                VariableDebtTokenEvents::InitializedFilter(element) => element.fmt(f),
                VariableDebtTokenEvents::MintFilter(element) => element.fmt(f),
                VariableDebtTokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEBT_TOKEN_REVISION`function with signature `DEBT_TOKEN_REVISION()` and selector `[185, 167, 182, 34]`"]
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
    #[doc = "Container type for all input parameters for the `DELEGATION_WITH_SIG_TYPEHASH`function with signature `DELEGATION_WITH_SIG_TYPEHASH()` and selector `[243, 191, 199, 56]`"]
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
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
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
    #[doc = "Container type for all input parameters for the `EIP712_REVISION`function with signature `EIP712_REVISION()` and selector `[120, 22, 3, 118]`"]
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
    #[doc = "Container type for all input parameters for the `POOL`function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
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
    #[doc = "Container type for all input parameters for the `UNDERLYING_ASSET_ADDRESS`function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `[177, 106, 25, 222]`"]
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
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[doc = "Container type for all input parameters for the `approveDelegation`function with signature `approveDelegation(address,uint256)` and selector `[192, 74, 138, 16]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowAllowance`function with signature `borrowAllowance(address,address)` and selector `[107, 215, 109, 36]`"]
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
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(address,uint256,uint256)` and selector `[245, 41, 138, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(address,uint256,uint256)")]
    pub struct BurnCall {
        pub from: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
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
    #[doc = "Container type for all input parameters for the `delegationWithSig`function with signature `delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[11, 82, 213, 88]`"]
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
    #[doc = "Container type for all input parameters for the `getIncentivesController`function with signature `getIncentivesController()` and selector `[117, 210, 100, 19]`"]
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
    #[doc = "Container type for all input parameters for the `getPreviousIndex`function with signature `getPreviousIndex(address)` and selector `[224, 117, 57, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPreviousIndex", abi = "getPreviousIndex(address)")]
    pub struct GetPreviousIndexCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getScaledUserBalanceAndSupply`function with signature `getScaledUserBalanceAndSupply(address)` and selector `[10, 251, 205, 201]`"]
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
        name = "getScaledUserBalanceAndSupply",
        abi = "getScaledUserBalanceAndSupply(address)"
    )]
    pub struct GetScaledUserBalanceAndSupplyCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
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
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address,address,address,uint8,string,string,bytes)` and selector `[194, 34, 236, 138]`"]
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
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,address,uint256,uint256)` and selector `[179, 241, 201, 61]`"]
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
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
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
    #[doc = "Container type for all input parameters for the `scaledBalanceOf`function with signature `scaledBalanceOf(address)` and selector `[29, 162, 79, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledBalanceOf", abi = "scaledBalanceOf(address)")]
    pub struct ScaledBalanceOfCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `scaledTotalSupply`function with signature `scaledTotalSupply()` and selector `[177, 191, 150, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "scaledTotalSupply", abi = "scaledTotalSupply()")]
    pub struct ScaledTotalSupplyCall;
    #[doc = "Container type for all input parameters for the `setIncentivesController`function with signature `setIncentivesController(address)` and selector `[230, 85, 219, 216]`"]
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
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
    pub enum VariableDebtTokenCalls {
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
        GetIncentivesController(GetIncentivesControllerCall),
        GetPreviousIndex(GetPreviousIndexCall),
        GetScaledUserBalanceAndSupply(GetScaledUserBalanceAndSupplyCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        ScaledBalanceOf(ScaledBalanceOfCall),
        ScaledTotalSupply(ScaledTotalSupplyCall),
        SetIncentivesController(SetIncentivesControllerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for VariableDebtTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DebtTokenRevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::DebtTokenRevision(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigTypehashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VariableDebtTokenCalls::DelegationWithSigTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <Eip712RevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Eip712Revision(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VariableDebtTokenCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingAssetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::UnderlyingAssetAddress(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveDelegationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::ApproveDelegation(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::BorrowAllowance(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VariableDebtTokenCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::DelegationWithSig(decoded));
            }
            if let Ok(decoded) =
                <GetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::GetIncentivesController(decoded));
            }
            if let Ok(decoded) =
                <GetPreviousIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::GetPreviousIndex(decoded));
            }
            if let Ok(decoded) =
                <GetScaledUserBalanceAndSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VariableDebtTokenCalls::GetScaledUserBalanceAndSupply(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Initialize(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VariableDebtTokenCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VariableDebtTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <ScaledBalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::ScaledBalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ScaledTotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::ScaledTotalSupply(decoded));
            }
            if let Ok(decoded) =
                <SetIncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::SetIncentivesController(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VariableDebtTokenCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VariableDebtTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VariableDebtTokenCalls::DebtTokenRevision(element) => element.encode(),
                VariableDebtTokenCalls::DelegationWithSigTypehash(element) => element.encode(),
                VariableDebtTokenCalls::DomainSeparator(element) => element.encode(),
                VariableDebtTokenCalls::Eip712Revision(element) => element.encode(),
                VariableDebtTokenCalls::Pool(element) => element.encode(),
                VariableDebtTokenCalls::UnderlyingAssetAddress(element) => element.encode(),
                VariableDebtTokenCalls::Allowance(element) => element.encode(),
                VariableDebtTokenCalls::Approve(element) => element.encode(),
                VariableDebtTokenCalls::ApproveDelegation(element) => element.encode(),
                VariableDebtTokenCalls::BalanceOf(element) => element.encode(),
                VariableDebtTokenCalls::BorrowAllowance(element) => element.encode(),
                VariableDebtTokenCalls::Burn(element) => element.encode(),
                VariableDebtTokenCalls::Decimals(element) => element.encode(),
                VariableDebtTokenCalls::DecreaseAllowance(element) => element.encode(),
                VariableDebtTokenCalls::DelegationWithSig(element) => element.encode(),
                VariableDebtTokenCalls::GetIncentivesController(element) => element.encode(),
                VariableDebtTokenCalls::GetPreviousIndex(element) => element.encode(),
                VariableDebtTokenCalls::GetScaledUserBalanceAndSupply(element) => element.encode(),
                VariableDebtTokenCalls::IncreaseAllowance(element) => element.encode(),
                VariableDebtTokenCalls::Initialize(element) => element.encode(),
                VariableDebtTokenCalls::Mint(element) => element.encode(),
                VariableDebtTokenCalls::Name(element) => element.encode(),
                VariableDebtTokenCalls::Nonces(element) => element.encode(),
                VariableDebtTokenCalls::ScaledBalanceOf(element) => element.encode(),
                VariableDebtTokenCalls::ScaledTotalSupply(element) => element.encode(),
                VariableDebtTokenCalls::SetIncentivesController(element) => element.encode(),
                VariableDebtTokenCalls::Symbol(element) => element.encode(),
                VariableDebtTokenCalls::TotalSupply(element) => element.encode(),
                VariableDebtTokenCalls::Transfer(element) => element.encode(),
                VariableDebtTokenCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VariableDebtTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VariableDebtTokenCalls::DebtTokenRevision(element) => element.fmt(f),
                VariableDebtTokenCalls::DelegationWithSigTypehash(element) => element.fmt(f),
                VariableDebtTokenCalls::DomainSeparator(element) => element.fmt(f),
                VariableDebtTokenCalls::Eip712Revision(element) => element.fmt(f),
                VariableDebtTokenCalls::Pool(element) => element.fmt(f),
                VariableDebtTokenCalls::UnderlyingAssetAddress(element) => element.fmt(f),
                VariableDebtTokenCalls::Allowance(element) => element.fmt(f),
                VariableDebtTokenCalls::Approve(element) => element.fmt(f),
                VariableDebtTokenCalls::ApproveDelegation(element) => element.fmt(f),
                VariableDebtTokenCalls::BalanceOf(element) => element.fmt(f),
                VariableDebtTokenCalls::BorrowAllowance(element) => element.fmt(f),
                VariableDebtTokenCalls::Burn(element) => element.fmt(f),
                VariableDebtTokenCalls::Decimals(element) => element.fmt(f),
                VariableDebtTokenCalls::DecreaseAllowance(element) => element.fmt(f),
                VariableDebtTokenCalls::DelegationWithSig(element) => element.fmt(f),
                VariableDebtTokenCalls::GetIncentivesController(element) => element.fmt(f),
                VariableDebtTokenCalls::GetPreviousIndex(element) => element.fmt(f),
                VariableDebtTokenCalls::GetScaledUserBalanceAndSupply(element) => element.fmt(f),
                VariableDebtTokenCalls::IncreaseAllowance(element) => element.fmt(f),
                VariableDebtTokenCalls::Initialize(element) => element.fmt(f),
                VariableDebtTokenCalls::Mint(element) => element.fmt(f),
                VariableDebtTokenCalls::Name(element) => element.fmt(f),
                VariableDebtTokenCalls::Nonces(element) => element.fmt(f),
                VariableDebtTokenCalls::ScaledBalanceOf(element) => element.fmt(f),
                VariableDebtTokenCalls::ScaledTotalSupply(element) => element.fmt(f),
                VariableDebtTokenCalls::SetIncentivesController(element) => element.fmt(f),
                VariableDebtTokenCalls::Symbol(element) => element.fmt(f),
                VariableDebtTokenCalls::TotalSupply(element) => element.fmt(f),
                VariableDebtTokenCalls::Transfer(element) => element.fmt(f),
                VariableDebtTokenCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DebtTokenRevisionCall> for VariableDebtTokenCalls {
        fn from(var: DebtTokenRevisionCall) -> Self {
            VariableDebtTokenCalls::DebtTokenRevision(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigTypehashCall> for VariableDebtTokenCalls {
        fn from(var: DelegationWithSigTypehashCall) -> Self {
            VariableDebtTokenCalls::DelegationWithSigTypehash(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for VariableDebtTokenCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            VariableDebtTokenCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<Eip712RevisionCall> for VariableDebtTokenCalls {
        fn from(var: Eip712RevisionCall) -> Self {
            VariableDebtTokenCalls::Eip712Revision(var)
        }
    }
    impl ::std::convert::From<PoolCall> for VariableDebtTokenCalls {
        fn from(var: PoolCall) -> Self {
            VariableDebtTokenCalls::Pool(var)
        }
    }
    impl ::std::convert::From<UnderlyingAssetAddressCall> for VariableDebtTokenCalls {
        fn from(var: UnderlyingAssetAddressCall) -> Self {
            VariableDebtTokenCalls::UnderlyingAssetAddress(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for VariableDebtTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            VariableDebtTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for VariableDebtTokenCalls {
        fn from(var: ApproveCall) -> Self {
            VariableDebtTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveDelegationCall> for VariableDebtTokenCalls {
        fn from(var: ApproveDelegationCall) -> Self {
            VariableDebtTokenCalls::ApproveDelegation(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for VariableDebtTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            VariableDebtTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowAllowanceCall> for VariableDebtTokenCalls {
        fn from(var: BorrowAllowanceCall) -> Self {
            VariableDebtTokenCalls::BorrowAllowance(var)
        }
    }
    impl ::std::convert::From<BurnCall> for VariableDebtTokenCalls {
        fn from(var: BurnCall) -> Self {
            VariableDebtTokenCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for VariableDebtTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            VariableDebtTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for VariableDebtTokenCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            VariableDebtTokenCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigCall> for VariableDebtTokenCalls {
        fn from(var: DelegationWithSigCall) -> Self {
            VariableDebtTokenCalls::DelegationWithSig(var)
        }
    }
    impl ::std::convert::From<GetIncentivesControllerCall> for VariableDebtTokenCalls {
        fn from(var: GetIncentivesControllerCall) -> Self {
            VariableDebtTokenCalls::GetIncentivesController(var)
        }
    }
    impl ::std::convert::From<GetPreviousIndexCall> for VariableDebtTokenCalls {
        fn from(var: GetPreviousIndexCall) -> Self {
            VariableDebtTokenCalls::GetPreviousIndex(var)
        }
    }
    impl ::std::convert::From<GetScaledUserBalanceAndSupplyCall> for VariableDebtTokenCalls {
        fn from(var: GetScaledUserBalanceAndSupplyCall) -> Self {
            VariableDebtTokenCalls::GetScaledUserBalanceAndSupply(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for VariableDebtTokenCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            VariableDebtTokenCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VariableDebtTokenCalls {
        fn from(var: InitializeCall) -> Self {
            VariableDebtTokenCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<MintCall> for VariableDebtTokenCalls {
        fn from(var: MintCall) -> Self {
            VariableDebtTokenCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for VariableDebtTokenCalls {
        fn from(var: NameCall) -> Self {
            VariableDebtTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for VariableDebtTokenCalls {
        fn from(var: NoncesCall) -> Self {
            VariableDebtTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<ScaledBalanceOfCall> for VariableDebtTokenCalls {
        fn from(var: ScaledBalanceOfCall) -> Self {
            VariableDebtTokenCalls::ScaledBalanceOf(var)
        }
    }
    impl ::std::convert::From<ScaledTotalSupplyCall> for VariableDebtTokenCalls {
        fn from(var: ScaledTotalSupplyCall) -> Self {
            VariableDebtTokenCalls::ScaledTotalSupply(var)
        }
    }
    impl ::std::convert::From<SetIncentivesControllerCall> for VariableDebtTokenCalls {
        fn from(var: SetIncentivesControllerCall) -> Self {
            VariableDebtTokenCalls::SetIncentivesController(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for VariableDebtTokenCalls {
        fn from(var: SymbolCall) -> Self {
            VariableDebtTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for VariableDebtTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            VariableDebtTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for VariableDebtTokenCalls {
        fn from(var: TransferCall) -> Self {
            VariableDebtTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for VariableDebtTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            VariableDebtTokenCalls::TransferFrom(var)
        }
    }
}
