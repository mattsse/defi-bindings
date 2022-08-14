pub use comp::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod comp {
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
    #[doc = "Comp was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static COMP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"fromDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DelegateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"previousBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelegateVotesChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"fromBlock\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint96\",\"name\":\"votes\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegateBySig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriorVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numCheckpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COMP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162001bd038038062001bd08339810160408190526200003491620000bc565b6001600160a01b03811660008181526001602052604080822080546001600160601b0319166a084595161401484a00000090811790915590517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef916200009a91620000f6565b60405180910390a35062000135565b8051620000b6816200011b565b92915050565b600060208284031215620000cf57600080fd5b6000620000dd8484620000a9565b949350505050565b620000f08162000118565b82525050565b60208101620000b68284620000e5565b60006001600160a01b038216620000b6565b90565b620001268162000106565b81146200013257600080fd5b50565b611a8b80620001456000396000f3fe608060405234801561001057600080fd5b50600436106101215760003560e01c806370a08231116100ad578063b4b5ea5711610071578063b4b5ea571461025f578063c3cda52014610272578063dd62ed3e14610285578063e7a324dc14610298578063f1127ed8146102a057610121565b806370a08231146101fe578063782d6fe1146102115780637ecebe001461023157806395d89b4114610244578063a9059cbb1461024c57610121565b806323b872dd116100f457806323b872dd14610181578063313ce56714610194578063587cde1e146101a95780635c19a95c146101c95780636fcfff45146101de57610121565b806306fdde0314610126578063095ea7b31461014457806318160ddd1461016457806320606b7014610179575b600080fd5b61012e6102c1565b60405161013b919061173c565b60405180910390f35b610157610152366004611205565b6102e5565b60405161013b9190611692565b61016c6103a2565b60405161013b91906116a0565b61016c6103b1565b61015761018f3660046111b8565b6103c8565b61019c61050d565b60405161013b91906117d6565b6101bc6101b7366004611158565b610512565b60405161013b9190611684565b6101dc6101d7366004611158565b61052d565b005b6101f16101ec366004611158565b61053a565b60405161013b91906117ad565b61016c61020c366004611158565b610552565b61022461021f366004611205565b610576565b60405161013b91906117f2565b61016c61023f366004611158565b61078d565b61012e61079f565b61015761025a366004611205565b6107bf565b61022461026d366004611158565b6107fb565b6101dc610280366004611235565b61086b565b61016c61029336600461117e565b610a55565b61016c610a87565b6102b36102ae3660046112bc565b610a93565b60405161013b9291906117bb565b6040518060400160405280600881526020016710dbdb5c1bdd5b9960c21b81525081565b6000806000198314156102fb5750600019610320565b61031d8360405180606001604052806025815260200161190e60259139610ac8565b90505b336000818152602081815260408083206001600160a01b03891680855292529182902080546001600160601b0319166001600160601b03861617905590519091907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9259061038e9085906117e4565b60405180910390a360019150505b92915050565b6a084595161401484a00000081565b6040516103bd9061166e565b604051809103902081565b6001600160a01b0383166000908152602081815260408083203380855290835281842054825160608101909352602580845291936001600160601b0390911692859261041e928892919061190e90830139610ac8565b9050866001600160a01b0316836001600160a01b03161415801561044b57506001600160601b0382811614155b156104f357600061047583836040518060600160405280603d81526020016119e5603d9139610af7565b6001600160a01b03898116600081815260208181526040808320948a16808452949091529081902080546001600160601b0319166001600160601b0386161790555192935090917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925906104e99085906117e4565b60405180910390a3505b6104fe878783610b36565b600193505050505b9392505050565b601281565b6002602052600090815260409020546001600160a01b031681565b6105373382610ce1565b50565b60046020526000908152604090205463ffffffff1681565b6001600160a01b03166000908152600160205260409020546001600160601b031690565b60004382106105a05760405162461bcd60e51b81526004016105979061176d565b60405180910390fd5b6001600160a01b03831660009081526004602052604090205463ffffffff16806105ce57600091505061039c565b6001600160a01b038416600090815260036020908152604080832063ffffffff60001986018116855292529091205416831061064a576001600160a01b03841660009081526003602090815260408083206000199490940163ffffffff1683529290522054600160201b90046001600160601b0316905061039c565b6001600160a01b038416600090815260036020908152604080832083805290915290205463ffffffff1683101561068557600091505061039c565b600060001982015b8163ffffffff168163ffffffff16111561074857600282820363ffffffff160481036106b7611115565b506001600160a01b038716600090815260036020908152604080832063ffffffff858116855290835292819020815180830190925254928316808252600160201b9093046001600160601b031691810191909152908714156107235760200151945061039c9350505050565b805163ffffffff1687111561073a57819350610741565b6001820392505b505061068d565b506001600160a01b038516600090815260036020908152604080832063ffffffff909416835292905220546001600160601b03600160201b9091041691505092915050565b60056020526000908152604090205481565b604051806040016040528060048152602001630434f4d560e41b81525081565b6000806107e48360405180606001604052806026815260200161193360269139610ac8565b90506107f1338583610b36565b5060019392505050565b6001600160a01b03811660009081526004602052604081205463ffffffff1680610826576000610506565b6001600160a01b0383166000908152600360209081526040808320600019850163ffffffff168452909152902054600160201b90046001600160601b03169392505050565b60006040516108799061166e565b60408051918290038220828201909152600882526710dbdb5c1bdd5b9960c21b6020909201919091527f561ca898cce9f021c15a441ef41899706e923541cee724530075d1a1144761c76108cb610d6b565b306040516020016108df94939291906116ec565b604051602081830303815290604052805190602001209050600060405161090590611679565b604051908190038120610920918a908a908a906020016116ae565b6040516020818303038152906040528051906020012090506000828260405160200161094d92919061163d565b60405160208183030381529060405280519060200120905060006001828888886040516000815260200160405260405161098a9493929190611721565b6020604051602081039080840390855afa1580156109ac573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166109df5760405162461bcd60e51b81526004016105979061174d565b6001600160a01b03811660009081526005602052604090208054600181019091558914610a1e5760405162461bcd60e51b81526004016105979061177d565b87421115610a3e5760405162461bcd60e51b81526004016105979061175d565b610a48818b610ce1565b505050505b505050505050565b6001600160a01b039182166000908152602081815260408083209390941682529190915220546001600160601b031690565b6040516103bd90611679565b600360209081526000928352604080842090915290825290205463ffffffff811690600160201b90046001600160601b031682565b600081600160601b8410610aef5760405162461bcd60e51b8152600401610597919061173c565b509192915050565b6000836001600160601b0316836001600160601b031611158290610b2e5760405162461bcd60e51b8152600401610597919061173c565b505050900390565b6001600160a01b038316610b5c5760405162461bcd60e51b81526004016105979061179d565b6001600160a01b038216610b825760405162461bcd60e51b81526004016105979061178d565b6001600160a01b038316600090815260016020908152604091829020548251606081019093526036808452610bcd936001600160601b0390921692859291906118d890830139610af7565b6001600160a01b03848116600090815260016020908152604080832080546001600160601b0319166001600160601b03968716179055928616825290829020548251606081019093526030808452610c3594919091169285929091906119b590830139610d6f565b6001600160a01b038381166000818152600160205260409081902080546001600160601b0319166001600160601b0395909516949094179093559151908516907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90610ca29085906117e4565b60405180910390a36001600160a01b03808416600090815260026020526040808220548584168352912054610cdc92918216911683610dab565b505050565b6001600160a01b03808316600081815260026020818152604080842080546001845282862054949093528787166001600160a01b031984168117909155905191909516946001600160601b039092169391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4610d65828483610dab565b50505050565b4690565b6000838301826001600160601b038087169083161015610da25760405162461bcd60e51b8152600401610597919061173c565b50949350505050565b816001600160a01b0316836001600160a01b031614158015610dd657506000816001600160601b0316115b15610cdc576001600160a01b03831615610e8e576001600160a01b03831660009081526004602052604081205463ffffffff169081610e16576000610e55565b6001600160a01b0385166000908152600360209081526040808320600019860163ffffffff168452909152902054600160201b90046001600160601b03165b90506000610e7c828560405180606001604052806028815260200161198d60289139610af7565b9050610e8a86848484610f39565b5050505b6001600160a01b03821615610cdc576001600160a01b03821660009081526004602052604081205463ffffffff169081610ec9576000610f08565b6001600160a01b0384166000908152600360209081526040808320600019860163ffffffff168452909152902054600160201b90046001600160601b03165b90506000610f2f8285604051806060016040528060278152602001611a2260279139610d6f565b9050610a4d858484845b6000610f5d43604051806060016040528060348152602001611959603491396110ee565b905060008463ffffffff16118015610fa657506001600160a01b038516600090815260036020908152604080832063ffffffff6000198901811685529252909120548282169116145b15611005576001600160a01b0385166000908152600360209081526040808320600019880163ffffffff168452909152902080546fffffffffffffffffffffffff000000001916600160201b6001600160601b038516021790556110a4565b60408051808201825263ffffffff80841682526001600160601b0380861660208085019182526001600160a01b038b166000818152600383528781208c871682528352878120965187549451909516600160201b026fffffffffffffffffffffffff000000001995871663ffffffff19958616179590951694909417909555938252600490935292909220805460018801909316929091169190911790555b846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72484846040516110df929190611800565b60405180910390a25050505050565b600081600160201b8410610aef5760405162461bcd60e51b8152600401610597919061173c565b604080518082019091526000808252602082015290565b803561039c816118a8565b803561039c816118bc565b803561039c816118c5565b803561039c816118ce565b60006020828403121561116a57600080fd5b6000611176848461112c565b949350505050565b6000806040838503121561119157600080fd5b600061119d858561112c565b92505060206111ae8582860161112c565b9150509250929050565b6000806000606084860312156111cd57600080fd5b60006111d9868661112c565b93505060206111ea8682870161112c565b92505060406111fb86828701611137565b9150509250925092565b6000806040838503121561121857600080fd5b6000611224858561112c565b92505060206111ae85828601611137565b60008060008060008060c0878903121561124e57600080fd5b600061125a898961112c565b965050602061126b89828a01611137565b955050604061127c89828a01611137565b945050606061128d89828a0161114d565b935050608061129e89828a01611137565b92505060a06112af89828a01611137565b9150509295509295509295565b600080604083850312156112cf57600080fd5b60006112db858561112c565b92505060206111ae85828601611142565b6112f58161182d565b82525050565b6112f581611838565b6112f58161183d565b6112f56113198261183d565b61183d565b60006113298261181b565b611333818561181f565b9350611343818560208601611872565b61134c8161189e565b9093019392505050565b600061136360268361181f565b7f436f6d703a3a64656c656761746542795369673a20696e76616c6964207369678152656e617475726560d01b602082015260400192915050565b60006113ab60268361181f565b7f436f6d703a3a64656c656761746542795369673a207369676e617475726520658152651e1c1a5c995960d21b602082015260400192915050565b60006113f3600283611828565b61190160f01b815260020192915050565b600061141160278361181f565b7f436f6d703a3a6765745072696f72566f7465733a206e6f742079657420646574815266195c9b5a5b995960ca1b602082015260400192915050565b600061145a60228361181f565b7f436f6d703a3a64656c656761746542795369673a20696e76616c6964206e6f6e815261636560f01b602082015260400192915050565b600061149e603a8361181f565b7f436f6d703a3a5f7472616e73666572546f6b656e733a2063616e6e6f7420747281527f616e7366657220746f20746865207a65726f2061646472657373000000000000602082015260400192915050565b60006114fd604383611828565b7f454950373132446f6d61696e28737472696e67206e616d652c75696e7432353681527f20636861696e49642c6164647265737320766572696679696e67436f6e74726160208201526263742960e81b604082015260430192915050565b6000611568603c8361181f565b7f436f6d703a3a5f7472616e73666572546f6b656e733a2063616e6e6f7420747281527f616e736665722066726f6d20746865207a65726f206164647265737300000000602082015260400192915050565b60006115c7603a83611828565b7f44656c65676174696f6e28616464726573732064656c6567617465652c75696e81527f74323536206e6f6e63652c75696e7432353620657870697279290000000000006020820152603a0192915050565b6112f58161184c565b6112f581611855565b6112f581611867565b6112f58161185b565b6000611648826113e6565b9150611654828561130d565b602082019150611664828461130d565b5060200192915050565b600061039c826114f0565b600061039c826115ba565b6020810161039c82846112ec565b6020810161039c82846112fb565b6020810161039c8284611304565b608081016116bc8287611304565b6116c960208301866112ec565b6116d66040830185611304565b6116e36060830184611304565b95945050505050565b608081016116fa8287611304565b6117076020830186611304565b6117146040830185611304565b6116e360608301846112ec565b6080810161172f8287611304565b6116c96020830186611622565b60208082528101610506818461131e565b6020808252810161039c81611356565b6020808252810161039c8161139e565b6020808252810161039c81611404565b6020808252810161039c8161144d565b6020808252810161039c81611491565b6020808252810161039c8161155b565b6020810161039c8284611619565b604081016117c98285611619565b6105066020830184611634565b6020810161039c8284611622565b6020810161039c828461162b565b6020810161039c8284611634565b6040810161180e828561162b565b610506602083018461162b565b5190565b90815260200190565b919050565b600061039c82611840565b151590565b90565b6001600160a01b031690565b63ffffffff1690565b60ff1690565b6001600160601b031690565b600061039c8261185b565b60005b8381101561188d578181015183820152602001611875565b83811115610d655750506000910152565b601f01601f191690565b6118b18161182d565b811461053757600080fd5b6118b18161183d565b6118b18161184c565b6118b18161185556fe436f6d703a3a5f7472616e73666572546f6b656e733a207472616e7366657220616d6f756e7420657863656564732062616c616e6365436f6d703a3a617070726f76653a20616d6f756e7420657863656564732039362062697473436f6d703a3a7472616e736665723a20616d6f756e7420657863656564732039362062697473436f6d703a3a5f7772697465436865636b706f696e743a20626c6f636b206e756d62657220657863656564732033322062697473436f6d703a3a5f6d6f7665566f7465733a20766f746520616d6f756e7420756e646572666c6f7773436f6d703a3a5f7472616e73666572546f6b656e733a207472616e7366657220616d6f756e74206f766572666c6f7773436f6d703a3a7472616e7366657246726f6d3a207472616e7366657220616d6f756e742065786365656473207370656e64657220616c6c6f77616e6365436f6d703a3a5f6d6f7665566f7465733a20766f746520616d6f756e74206f766572666c6f7773a365627a7a72315820a621318828ddcbe52b182296ba9ebbd377258b22901d1d8727b706ba16ac2fa76c6578706572696d656e74616cf564736f6c63430005110040" . parse () . expect ("invalid bytecode")
        });
    pub struct Comp<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Comp<M> {
        fn clone(&self) -> Self {
            Comp(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Comp<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Comp<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Comp))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Comp<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COMP_ABI.clone(), client).into()
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
                COMP_ABI.clone(),
                COMP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DELEGATION_TYPEHASH` (0xe7a324dc) function"]
        pub fn delegation_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([231, 163, 36, 220], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function"]
        pub fn domain_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            account: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (account, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, raw_amount))
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
        #[doc = "Calls the contract's `checkpoints` (0xf1127ed8) function"]
        pub fn checkpoints(
            &self,
            p0: ethers::core::types::Address,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, (u32, u128)> {
            self.0
                .method_hash([241, 18, 126, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegate` (0x5c19a95c) function"]
        pub fn delegate(
            &self,
            delegatee: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateBySig` (0xc3cda520) function"]
        pub fn delegate_by_sig(
            &self,
            delegatee: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegates` (0x587cde1e) function"]
        pub fn delegates(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([88, 124, 222, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentVotes` (0xb4b5ea57) function"]
        pub fn get_current_votes(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([180, 181, 234, 87], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriorVotes` (0x782d6fe1) function"]
        pub fn get_prior_votes(
            &self,
            account: ethers::core::types::Address,
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([120, 45, 111, 225], (account, block_number))
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
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numCheckpoints` (0x6fcfff45) function"]
        pub fn num_checkpoints(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 207, 255, 69], p0)
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
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateChanged` event"]
        pub fn delegate_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateVotesChanged` event"]
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateVotesChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CompEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Comp<M> {
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
        name = "DelegateChanged",
        abi = "DelegateChanged(address,address,address)"
    )]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ethers::core::types::Address,
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
        name = "DelegateVotesChanged",
        abi = "DelegateVotesChanged(address,uint256,uint256)"
    )]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ethers::core::types::Address,
        pub previous_balance: ethers::core::types::U256,
        pub new_balance: ethers::core::types::U256,
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
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CompEvents {
        ApprovalFilter(ApprovalFilter),
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for CompEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CompEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(CompEvents::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(CompEvents::DelegateVotesChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CompEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CompEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CompEvents::ApprovalFilter(element) => element.fmt(f),
                CompEvents::DelegateChangedFilter(element) => element.fmt(f),
                CompEvents::DelegateVotesChangedFilter(element) => element.fmt(f),
                CompEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DELEGATION_TYPEHASH` function with signature `DELEGATION_TYPEHASH()` and selector `[231, 163, 36, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DELEGATION_TYPEHASH", abi = "DELEGATION_TYPEHASH()")]
    pub struct DelegationTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
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
    pub struct AllowanceCall {
        pub account: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
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
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `[241, 18, 126, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "checkpoints", abi = "checkpoints(address,uint32)")]
    pub struct CheckpointsCall(pub ethers::core::types::Address, pub u32);
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
    #[doc = "Container type for all input parameters for the `delegate` function with signature `delegate(address)` and selector `[92, 25, 169, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `delegateBySig` function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[195, 205, 165, 32]`"]
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `delegates` function with signature `delegates(address)` and selector `[88, 124, 222, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getCurrentVotes` function with signature `getCurrentVotes(address)` and selector `[180, 181, 234, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentVotes", abi = "getCurrentVotes(address)")]
    pub struct GetCurrentVotesCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPriorVotes` function with signature `getPriorVotes(address,uint256)` and selector `[120, 45, 111, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriorVotes", abi = "getPriorVotes(address,uint256)")]
    pub struct GetPriorVotesCall {
        pub account: ethers::core::types::Address,
        pub block_number: ethers::core::types::U256,
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
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `[111, 207, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(address)")]
    pub struct NumCheckpointsCall(pub ethers::core::types::Address);
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
    pub struct TransferCall {
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
    }
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
    pub struct TransferFromCall {
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CompCalls {
        DelegationTypehash(DelegationTypehashCall),
        DomainTypehash(DomainTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Checkpoints(CheckpointsCall),
        Decimals(DecimalsCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        GetCurrentVotes(GetCurrentVotesCall),
        GetPriorVotes(GetPriorVotesCall),
        Name(NameCall),
        Nonces(NoncesCall),
        NumCheckpoints(NumCheckpointsCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for CompCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DelegationTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::DelegationTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Checkpoints(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Delegate(decoded));
            }
            if let Ok(decoded) =
                <DelegateBySigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::DelegateBySig(decoded));
            }
            if let Ok(decoded) =
                <DelegatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Delegates(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::GetCurrentVotes(decoded));
            }
            if let Ok(decoded) =
                <GetPriorVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::GetPriorVotes(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CompCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <NumCheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::NumCheckpoints(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CompCalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CompCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CompCalls::DelegationTypehash(element) => element.encode(),
                CompCalls::DomainTypehash(element) => element.encode(),
                CompCalls::Allowance(element) => element.encode(),
                CompCalls::Approve(element) => element.encode(),
                CompCalls::BalanceOf(element) => element.encode(),
                CompCalls::Checkpoints(element) => element.encode(),
                CompCalls::Decimals(element) => element.encode(),
                CompCalls::Delegate(element) => element.encode(),
                CompCalls::DelegateBySig(element) => element.encode(),
                CompCalls::Delegates(element) => element.encode(),
                CompCalls::GetCurrentVotes(element) => element.encode(),
                CompCalls::GetPriorVotes(element) => element.encode(),
                CompCalls::Name(element) => element.encode(),
                CompCalls::Nonces(element) => element.encode(),
                CompCalls::NumCheckpoints(element) => element.encode(),
                CompCalls::Symbol(element) => element.encode(),
                CompCalls::TotalSupply(element) => element.encode(),
                CompCalls::Transfer(element) => element.encode(),
                CompCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CompCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CompCalls::DelegationTypehash(element) => element.fmt(f),
                CompCalls::DomainTypehash(element) => element.fmt(f),
                CompCalls::Allowance(element) => element.fmt(f),
                CompCalls::Approve(element) => element.fmt(f),
                CompCalls::BalanceOf(element) => element.fmt(f),
                CompCalls::Checkpoints(element) => element.fmt(f),
                CompCalls::Decimals(element) => element.fmt(f),
                CompCalls::Delegate(element) => element.fmt(f),
                CompCalls::DelegateBySig(element) => element.fmt(f),
                CompCalls::Delegates(element) => element.fmt(f),
                CompCalls::GetCurrentVotes(element) => element.fmt(f),
                CompCalls::GetPriorVotes(element) => element.fmt(f),
                CompCalls::Name(element) => element.fmt(f),
                CompCalls::Nonces(element) => element.fmt(f),
                CompCalls::NumCheckpoints(element) => element.fmt(f),
                CompCalls::Symbol(element) => element.fmt(f),
                CompCalls::TotalSupply(element) => element.fmt(f),
                CompCalls::Transfer(element) => element.fmt(f),
                CompCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DelegationTypehashCall> for CompCalls {
        fn from(var: DelegationTypehashCall) -> Self {
            CompCalls::DelegationTypehash(var)
        }
    }
    impl ::std::convert::From<DomainTypehashCall> for CompCalls {
        fn from(var: DomainTypehashCall) -> Self {
            CompCalls::DomainTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CompCalls {
        fn from(var: AllowanceCall) -> Self {
            CompCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CompCalls {
        fn from(var: ApproveCall) -> Self {
            CompCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CompCalls {
        fn from(var: BalanceOfCall) -> Self {
            CompCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CheckpointsCall> for CompCalls {
        fn from(var: CheckpointsCall) -> Self {
            CompCalls::Checkpoints(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CompCalls {
        fn from(var: DecimalsCall) -> Self {
            CompCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DelegateCall> for CompCalls {
        fn from(var: DelegateCall) -> Self {
            CompCalls::Delegate(var)
        }
    }
    impl ::std::convert::From<DelegateBySigCall> for CompCalls {
        fn from(var: DelegateBySigCall) -> Self {
            CompCalls::DelegateBySig(var)
        }
    }
    impl ::std::convert::From<DelegatesCall> for CompCalls {
        fn from(var: DelegatesCall) -> Self {
            CompCalls::Delegates(var)
        }
    }
    impl ::std::convert::From<GetCurrentVotesCall> for CompCalls {
        fn from(var: GetCurrentVotesCall) -> Self {
            CompCalls::GetCurrentVotes(var)
        }
    }
    impl ::std::convert::From<GetPriorVotesCall> for CompCalls {
        fn from(var: GetPriorVotesCall) -> Self {
            CompCalls::GetPriorVotes(var)
        }
    }
    impl ::std::convert::From<NameCall> for CompCalls {
        fn from(var: NameCall) -> Self {
            CompCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CompCalls {
        fn from(var: NoncesCall) -> Self {
            CompCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<NumCheckpointsCall> for CompCalls {
        fn from(var: NumCheckpointsCall) -> Self {
            CompCalls::NumCheckpoints(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CompCalls {
        fn from(var: SymbolCall) -> Self {
            CompCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CompCalls {
        fn from(var: TotalSupplyCall) -> Self {
            CompCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CompCalls {
        fn from(var: TransferCall) -> Self {
            CompCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CompCalls {
        fn from(var: TransferFromCall) -> Self {
            CompCalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `DELEGATION_TYPEHASH` function with signature `DELEGATION_TYPEHASH()` and selector `[231, 163, 36, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DelegationTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DomainTypehashReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `checkpoints` function with signature `checkpoints(address,uint32)` and selector `[241, 18, 126, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CheckpointsReturn {
        pub from_block: u32,
        pub votes: u128,
    }
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
    #[doc = "Container type for all return fields from the `delegates` function with signature `delegates(address)` and selector `[88, 124, 222, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DelegatesReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getCurrentVotes` function with signature `getCurrentVotes(address)` and selector `[180, 181, 234, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCurrentVotesReturn(pub u128);
    #[doc = "Container type for all return fields from the `getPriorVotes` function with signature `getPriorVotes(address,uint256)` and selector `[120, 45, 111, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriorVotesReturn(pub u128);
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
    #[doc = "Container type for all return fields from the `numCheckpoints` function with signature `numCheckpoints(address)` and selector `[111, 207, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct NumCheckpointsReturn(pub u32);
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
