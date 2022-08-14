pub use pool_addresses_provider::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod pool_addresses_provider {
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
    #[doc = "PoolAddressesProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static POOLADDRESSESPROVIDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"marketId\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ACLAdminUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ACLManagerUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldImplementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newImplementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSetAsProxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"oldMarketId\",\"type\":\"string\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"newMarketId\",\"type\":\"string\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MarketIdSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolConfiguratorUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolDataProviderUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PoolUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleSentinelUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementationAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ProxyCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACLAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACLManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMarketId\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolConfigurator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolDataProvider\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracleSentinel\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAclAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setACLAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAclManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setACLManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"id\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newImplementationAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAddressAsProxy\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newMarketId\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMarketId\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPoolConfiguratorImpl\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolConfiguratorImpl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newDataProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolDataProvider\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPoolImpl\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolImpl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracleSentinel\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceOracleSentinel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static POOLADDRESSESPROVIDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b5060405162001ffb38038062001ffb8339810160408190526200003491620003aa565b600080546001600160a01b0319163390811782556040519091829160008051602062001fdb833981519152908290a3506200006f8262000082565b6200007a816200018d565b5050620004d2565b600060018054620000939062000477565b80601f0160208091040260200160405190810160405280929190818152602001828054620000c19062000477565b8015620001125780601f10620000e65761010080835404028352916020019162000112565b820191906000526020600020905b815481529060010190602001808311620000f457829003601f168201915b5050855193945062000130936001935060208701925090506200029e565b5081604051620001419190620004b4565b604051809103902081604051620001599190620004b4565b604051908190038120907fe685c8cdecc6030c45030fd54778812cb84ed8e4467c38294403d68ba786082390600090a35050565b6000546001600160a01b03163314620001ed5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064015b60405180910390fd5b6001600160a01b038116620002545760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401620001e4565b600080546040516001600160a01b038085169392169160008051602062001fdb83398151915291a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b828054620002ac9062000477565b90600052602060002090601f016020900481019282620002d057600085556200031b565b82601f10620002eb57805160ff19168380011785556200031b565b828001600101855582156200031b579182015b828111156200031b578251825591602001919060010190620002fe565b50620003299291506200032d565b5090565b5b808211156200032957600081556001016200032e565b634e487b7160e01b600052604160045260246000fd5b60005b83811015620003775781810151838201526020016200035d565b8381111562000387576000848401525b50505050565b80516001600160a01b0381168114620003a557600080fd5b919050565b60008060408385031215620003be57600080fd5b82516001600160401b0380821115620003d657600080fd5b818501915085601f830112620003eb57600080fd5b81518181111562000400576200040062000344565b604051601f8201601f19908116603f011681019083821181831017156200042b576200042b62000344565b816040528281528860208487010111156200044557600080fd5b620004588360208301602088016200035a565b80965050505050506200046e602084016200038d565b90509250929050565b600181811c908216806200048c57607f821691505b60208210811415620004ae57634e487b7160e01b600052602260045260246000fd5b50919050565b60008251620004c88184602087016200035a565b9190910192915050565b611af980620004e26000396000f3fe608060405234801561001057600080fd5b50600436106101425760003560e01c806376d84ffc116100b8578063e4ca28b71161007c578063e4ca28b714610254578063e860accb14610267578063ed301ca91461026f578063f2fde38b14610282578063f67b184714610295578063fca513a8146102a857600080fd5b806376d84ffc146101f75780638da5cb5b1461020a578063a15644061461021b578063ca446dd91461022e578063e44e9ed11461024157600080fd5b80635dcc528c1161010a5780635dcc528c146101b15780635eb88d3d146101c4578063631adfca146101cc578063707cd716146101d4578063715018a6146101dc57806374944cec146101e457600080fd5b8063026b1d5f146101475780630e67178c1461016c57806321f8a72114610174578063530e784f14610187578063568ef4701461019c575b600080fd5b61014f6102b0565b6040516001600160a01b0390911681526020015b60405180910390f35b61014f6102c7565b61014f610182366004610fb7565b6102da565b61019a610195366004610fe5565b6102f5565b005b6101a46103b0565b6040516101639190611065565b61019a6101bf366004611078565b610442565b61014f6104e7565b61014f61050a565b61014f610529565b61019a610542565b61019a6101f2366004610fe5565b6105b6565b61019a610205366004610fe5565b610671565b6000546001600160a01b031661014f565b61019a610229366004610fe5565b610720565b61019a61023c366004611078565b6107b3565b61019a61024f366004610fe5565b61083b565b61019a610262366004610fe5565b6108ee565b61014f61099b565b61019a61027d366004610fe5565b6109b6565b61019a610290366004610fe5565b610a67565b61019a6102a33660046110be565b610b51565b61014f610b87565b60006102c2631413d3d360e21b6102da565b905090565b60006102c26820a1a62fa0a226a4a760b91b5b6000908152600260205260409020546001600160a01b031690565b6000546001600160a01b031633146103285760405162461bcd60e51b815260040161031f9061116f565b60405180910390fd5b6b50524943455f4f5241434c4560a01b600090815260026020527f740f710666bd7a12af42df98311e541e47f7fd33d382d11602457a6d540cbd6380546001600160a01b038481166001600160a01b03198316811790935560405191169283917f56b5f80d8cac1479698aa7d01605fd6111e90b15fc4d2b377417f46034876cbd9190a35050565b6060600180546103bf906111a4565b80601f01602080910402602001604051908101604052809291908181526020018280546103eb906111a4565b80156104385780601f1061040d57610100808354040283529160200191610438565b820191906000526020600020905b81548152906001019060200180831161041b57829003601f168201915b5050505050905090565b6000546001600160a01b0316331461046c5760405162461bcd60e51b815260040161031f9061116f565b6000828152600260205260408120546001600160a01b03169061048e84610ba1565b905061049a8484610c3e565b6040516001600160a01b038281168252808516919084169086907f3bbd45b5429b385e3fb37ad5cd1cd1435a3c8ec32196c7937597365a3fd3e99c9060200160405180910390a450505050565b60006102c27414149250d157d3d49050d31157d4d1539512539153605a1b6102da565b60006102c2702827a7a62fa1a7a72324a3aaa920aa27a960791b6102da565b60006102c26a20a1a62fa6a0a720a3a2a960a91b6102da565b6000546001600160a01b0316331461056c5760405162461bcd60e51b815260040161031f9061116f565b600080546040516001600160a01b03909116907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908390a3600080546001600160a01b0319169055565b6000546001600160a01b031633146105e05760405162461bcd60e51b815260040161031f9061116f565b7414149250d157d3d49050d31157d4d1539512539153605a1b600090815260026020527f0d2c1bcee56447b4f46248272f34207a580a5c40f666a31f4e2fbb470ea53ab880546001600160a01b038481166001600160a01b03198316811790935560405191169283917f5326514eeca90494a14bedabcff812a0e683029ee85d1e23824d44fd14cd6ae79190a35050565b6000546001600160a01b0316331461069b5760405162461bcd60e51b815260040161031f9061116f565b6820a1a62fa0a226a4a760b91b600090815260026020527ffab167ad2009dcb80ee379700bb4bd029d97c1181ed9d961625632c8a6f051c680546001600160a01b038481166001600160a01b03198316811790935560405191169283917fe9cf53972264dc95304fd424458745019ddfca0e37ae8f703d74772c41ad115b9190a35050565b6000546001600160a01b0316331461074a5760405162461bcd60e51b815260040161031f9061116f565b600061075c631413d3d360e21b610ba1565b905061076f631413d3d360e21b83610c3e565b816001600160a01b0316816001600160a01b03167f90affc163f1a2dfedcd36aa02ed992eeeba8100a4014f0b4cdc20ea265a6662760405160405180910390a35050565b6000546001600160a01b031633146107dd5760405162461bcd60e51b815260040161031f9061116f565b60008281526002602052604080822080546001600160a01b031981166001600160a01b038681169182179093559251911692839186917f9ef0e8c8e52743bb38b83b17d9429141d494b8041ca6d616a6c77cebae9cd8b791a4505050565b6000546001600160a01b031633146108655760405162461bcd60e51b815260040161031f9061116f565b6c2220aa20afa82927ab24a222a960991b600090815260026020527fcd7944601aaa5cd7ccdae1bebec659e98c6aac8f12486b30e59db0d39698051f80546001600160a01b038481166001600160a01b03198316811790935560405191169283917fc853974cfbf81487a14a23565917bee63f527853bcb5fa54f2ae1cdf8a38356d9190a35050565b6000546001600160a01b031633146109185760405162461bcd60e51b815260040161031f9061116f565b6000610937702827a7a62fa1a7a72324a3aaa920aa27a960791b610ba1565b9050610957702827a7a62fa1a7a72324a3aaa920aa27a960791b83610c3e565b816001600160a01b0316816001600160a01b03167f8932892569eba59c8382a089d9b732d1f49272878775235761a2a6b0309cd46560405160405180910390a35050565b60006102c26c2220aa20afa82927ab24a222a960991b6102da565b6000546001600160a01b031633146109e05760405162461bcd60e51b815260040161031f9061116f565b6a20a1a62fa6a0a720a3a2a960a91b600090815260026020527f9edef266ef35fd0c6e131df0f31a330f3dd4c4d19dd31ed615c21d005c68116b80546001600160a01b038481166001600160a01b03198316811790935560405191169283917fb30efa04327bb8a537d61cc1e5c48095345ad18ef7cc04e6bacf7dfb6caaf5079190a35050565b6000546001600160a01b03163314610a915760405162461bcd60e51b815260040161031f9061116f565b6001600160a01b038116610af65760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161031f565b600080546040516001600160a01b03808516939216917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e091a3600080546001600160a01b0319166001600160a01b0392909216919091179055565b6000546001600160a01b03163314610b7b5760405162461bcd60e51b815260040161031f9061116f565b610b8481610e14565b50565b60006102c26b50524943455f4f5241434c4560a01b6102da565b6000818152600260205260408120546001600160a01b031680610bc75750600092915050565b6000819050806001600160a01b0316635c60da1b6040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610c0c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c3091906111d9565b949350505050565b50919050565b6000828152600260205260408082205490513060248201526001600160a01b039091169190819060440160408051601f198184030181529190526020810180516001600160e01b031663189acdbd60e31b17905290506001600160a01b038316610da95730604051610caf90610f11565b6001600160a01b039091168152602001604051809103906000f080158015610cdb573d6000803e3d6000fd5b506000868152600260205260409081902080546001600160a01b0319166001600160a01b038416908117909155905163347d5e2560e21b81529194508493509063d1f5789490610d3190879085906004016111f6565b600060405180830381600087803b158015610d4b57600080fd5b505af1158015610d5f573d6000803e3d6000fd5b50505050836001600160a01b0316836001600160a01b0316867f4a465a9bd819d9662563c1e11ae958f8109e437e7f4bf1c6ef0b9a7b3f35d47860405160405180910390a4610e0d565b60405163278f794360e11b81528392506001600160a01b03831690634f1ef28690610dda90879085906004016111f6565b600060405180830381600087803b158015610df457600080fd5b505af1158015610e08573d6000803e3d6000fd5b505050505b5050505050565b600060018054610e23906111a4565b80601f0160208091040260200160405190810160405280929190818152602001828054610e4f906111a4565b8015610e9c5780601f10610e7157610100808354040283529160200191610e9c565b820191906000526020600020905b815481529060010190602001808311610e7f57829003601f168201915b50508551939450610eb893600193506020870192509050610f1e565b5081604051610ec7919061121a565b604051809103902081604051610edd919061121a565b604051908190038120907fe685c8cdecc6030c45030fd54778812cb84ed8e4467c38294403d68ba786082390600090a35050565b61088d8061123783390190565b828054610f2a906111a4565b90600052602060002090601f016020900481019282610f4c5760008555610f92565b82601f10610f6557805160ff1916838001178555610f92565b82800160010185558215610f92579182015b82811115610f92578251825591602001919060010190610f77565b50610f9e929150610fa2565b5090565b5b80821115610f9e5760008155600101610fa3565b600060208284031215610fc957600080fd5b5035919050565b6001600160a01b0381168114610b8457600080fd5b600060208284031215610ff757600080fd5b813561100281610fd0565b9392505050565b60005b8381101561102457818101518382015260200161100c565b83811115611033576000848401525b50505050565b60008151808452611051816020860160208601611009565b601f01601f19169290920160200192915050565b6020815260006110026020830184611039565b6000806040838503121561108b57600080fd5b82359150602083013561109d81610fd0565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b6000602082840312156110d057600080fd5b813567ffffffffffffffff808211156110e857600080fd5b818401915084601f8301126110fc57600080fd5b81358181111561110e5761110e6110a8565b604051601f8201601f19908116603f01168101908382118183101715611136576111366110a8565b8160405282815287602084870101111561114f57600080fd5b826020860160208301376000928101602001929092525095945050505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b600181811c908216806111b857607f821691505b60208210811415610c3857634e487b7160e01b600052602260045260246000fd5b6000602082840312156111eb57600080fd5b815161100281610fd0565b6001600160a01b0383168152604060208201819052600090610c3090830184611039565b6000825161122c818460208701611009565b919091019291505056fe60a060405234801561001057600080fd5b5060405161088d38038061088d83398101604081905261002f91610040565b6001600160a01b0316608052610070565b60006020828403121561005257600080fd5b81516001600160a01b038116811461006957600080fd5b9392505050565b6080516107df6100ae600039600081816101130152818161015801528181610211015281816103510152818161037a01526104a501526107df6000f3fe60806040526004361061004a5760003560e01c80633659cfe6146100545780634f1ef286146100745780635c60da1b14610087578063d1f57894146100b8578063f851a440146100cb575b6100526100e0565b005b34801561006057600080fd5b5061005261006f366004610586565b610108565b6100526100823660046105a8565b61014d565b34801561009357600080fd5b5061009c610204565b6040516001600160a01b03909116815260200160405180910390f35b6100526100c6366004610641565b610256565b3480156100d757600080fd5b5061009c610344565b6100e861039c565b61010661010160008051602061078a8339815191525490565b6103a4565b565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561014557610142816103c8565b50565b6101426100e0565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101f757610187836103c8565b6000836001600160a01b031683836040516101a3929190610703565b600060405180830381855af49150503d80600081146101de576040519150601f19603f3d011682016040523d82523d6000602084013e6101e3565b606091505b50509050806101f157600080fd5b50505050565b6101ff6100e0565b505050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561024b575060008051602061078a8339815191525490565b6102536100e0565b90565b600061026e60008051602061078a8339815191525490565b6001600160a01b03161461028157600080fd5b6102ac60017f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbd610713565b60008051602061078a833981519152146102c8576102c8610738565b6102d182610408565b805115610340576000826001600160a01b0316826040516102f2919061074e565b600060405180830381855af49150503d806000811461032d576040519150601f19603f3d011682016040523d82523d6000602084013e610332565b606091505b50509050806101ff57600080fd5b5050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016141561024b57507f000000000000000000000000000000000000000000000000000000000000000090565b61010661049a565b3660008037600080366000845af43d6000803e8080156103c3573d6000f35b3d6000fd5b6103d181610408565b6040516001600160a01b038216907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b90600090a250565b6104118161052e565b6104885760405162461bcd60e51b815260206004820152603b60248201527f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f60448201527f6e20746f2061206e6f6e2d636f6e74726163742061646472657373000000000060648201526084015b60405180910390fd5b60008051602061078a83398151915255565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614156101065760405162461bcd60e51b815260206004820152603260248201527f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667260448201527137b6903a343290383937bc3c9030b236b4b760711b606482015260840161047f565b6000813f7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47081811480159061056257508115155b949350505050565b80356001600160a01b038116811461058157600080fd5b919050565b60006020828403121561059857600080fd5b6105a18261056a565b9392505050565b6000806000604084860312156105bd57600080fd5b6105c68461056a565b9250602084013567ffffffffffffffff808211156105e357600080fd5b818601915086601f8301126105f757600080fd5b81358181111561060657600080fd5b87602082850101111561061857600080fd5b6020830194508093505050509250925092565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561065457600080fd5b61065d8361056a565b9150602083013567ffffffffffffffff8082111561067a57600080fd5b818501915085601f83011261068e57600080fd5b8135818111156106a0576106a061062b565b604051601f8201601f19908116603f011681019083821181831017156106c8576106c861062b565b816040528281528860208487010111156106e157600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b8183823760009101908152919050565b60008282101561073357634e487b7160e01b600052601160045260246000fd5b500390565b634e487b7160e01b600052600160045260246000fd5b6000825160005b8181101561076f5760208186018101518583015201610755565b8181111561077e576000828501525b50919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbca264697066735822122093ac75a9551727db7c726cbbc41613e76a776b55293117a0cf140b1f86c9b22e64736f6c634300080a0033a26469706673582212200ba63322d85bcea66a6fd353d296185c2d909702b8572e9384e46a843e28b6fb64736f6c634300080a00338be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0" . parse () . expect ("invalid bytecode")
        });
    pub struct PoolAddressesProvider<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PoolAddressesProvider<M> {
        fn clone(&self) -> Self {
            PoolAddressesProvider(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PoolAddressesProvider<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PoolAddressesProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PoolAddressesProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PoolAddressesProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                POOLADDRESSESPROVIDER_ABI.clone(),
                client,
            )
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
                POOLADDRESSESPROVIDER_ABI.clone(),
                POOLADDRESSESPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getACLAdmin` (0x0e67178c) function"]
        pub fn get_acl_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([14, 103, 23, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getACLManager` (0x707cd716) function"]
        pub fn get_acl_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([112, 124, 215, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddress` (0x21f8a721) function"]
        pub fn get_address(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 248, 167, 33], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMarketId` (0x568ef470) function"]
        pub fn get_market_id(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([86, 142, 244, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPool` (0x026b1d5f) function"]
        pub fn get_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([2, 107, 29, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolConfigurator` (0x631adfca) function"]
        pub fn get_pool_configurator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 26, 223, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolDataProvider` (0xe860accb) function"]
        pub fn get_pool_data_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 96, 172, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracle` (0xfca513a8) function"]
        pub fn get_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracleSentinel` (0x5eb88d3d) function"]
        pub fn get_price_oracle_sentinel(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 184, 141, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACLAdmin` (0x76d84ffc) function"]
        pub fn set_acl_admin(
            &self,
            new_acl_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 216, 79, 252], new_acl_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACLManager` (0xed301ca9) function"]
        pub fn set_acl_manager(
            &self,
            new_acl_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 48, 28, 169], new_acl_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddress` (0xca446dd9) function"]
        pub fn set_address(
            &self,
            id: [u8; 32],
            new_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 68, 109, 217], (id, new_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddressAsProxy` (0x5dcc528c) function"]
        pub fn set_address_as_proxy(
            &self,
            id: [u8; 32],
            new_implementation_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 204, 82, 140], (id, new_implementation_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMarketId` (0xf67b1847) function"]
        pub fn set_market_id(
            &self,
            new_market_id: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 123, 24, 71], new_market_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolConfiguratorImpl` (0xe4ca28b7) function"]
        pub fn set_pool_configurator_impl(
            &self,
            new_pool_configurator_impl: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 202, 40, 183], new_pool_configurator_impl)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolDataProvider` (0xe44e9ed1) function"]
        pub fn set_pool_data_provider(
            &self,
            new_data_provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 78, 158, 209], new_data_provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolImpl` (0xa1564406) function"]
        pub fn set_pool_impl(
            &self,
            new_pool_impl: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 86, 68, 6], new_pool_impl)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracle` (0x530e784f) function"]
        pub fn set_price_oracle(
            &self,
            new_price_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], new_price_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracleSentinel` (0x74944cec) function"]
        pub fn set_price_oracle_sentinel(
            &self,
            new_price_oracle_sentinel: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 148, 76, 236], new_price_oracle_sentinel)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ACLAdminUpdated` event"]
        pub fn acl_admin_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AcladminUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ACLManagerUpdated` event"]
        pub fn acl_manager_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AclmanagerUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers::contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AddressSetAsProxy` event"]
        pub fn address_set_as_proxy_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddressSetAsProxyFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketIdSet` event"]
        pub fn market_id_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MarketIdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolConfiguratorUpdated` event"]
        pub fn pool_configurator_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolConfiguratorUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolDataProviderUpdated` event"]
        pub fn pool_data_provider_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolDataProviderUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PoolUpdated` event"]
        pub fn pool_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PoolUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleSentinelUpdated` event"]
        pub fn price_oracle_sentinel_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceOracleSentinelUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleUpdated` event"]
        pub fn price_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProxyCreated` event"]
        pub fn proxy_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProxyCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PoolAddressesProviderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for PoolAddressesProvider<M>
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
    #[ethevent(name = "ACLAdminUpdated", abi = "ACLAdminUpdated(address,address)")]
    pub struct AcladminUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "ACLManagerUpdated", abi = "ACLManagerUpdated(address,address)")]
    pub struct AclmanagerUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "AddressSetAsProxy",
        abi = "AddressSetAsProxy(bytes32,address,address,address)"
    )]
    pub struct AddressSetAsProxyFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ethers::core::types::Address,
        pub old_implementation_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_implementation_address: ethers::core::types::Address,
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
    #[ethevent(name = "MarketIdSet", abi = "MarketIdSet(string,string)")]
    pub struct MarketIdSetFilter {
        #[ethevent(indexed)]
        pub old_market_id: ethers::core::types::H256,
        #[ethevent(indexed)]
        pub new_market_id: ethers::core::types::H256,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        name = "PoolConfiguratorUpdated",
        abi = "PoolConfiguratorUpdated(address,address)"
    )]
    pub struct PoolConfiguratorUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PoolDataProviderUpdated",
        abi = "PoolDataProviderUpdated(address,address)"
    )]
    pub struct PoolDataProviderUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "PoolUpdated", abi = "PoolUpdated(address,address)")]
    pub struct PoolUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PriceOracleSentinelUpdated",
        abi = "PriceOracleSentinelUpdated(address,address)"
    )]
    pub struct PriceOracleSentinelUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
        name = "PriceOracleUpdated",
        abi = "PriceOracleUpdated(address,address)"
    )]
    pub struct PriceOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[ethevent(name = "ProxyCreated", abi = "ProxyCreated(bytes32,address,address)")]
    pub struct ProxyCreatedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolAddressesProviderEvents {
        AcladminUpdatedFilter(AcladminUpdatedFilter),
        AclmanagerUpdatedFilter(AclmanagerUpdatedFilter),
        AddressSetFilter(AddressSetFilter),
        AddressSetAsProxyFilter(AddressSetAsProxyFilter),
        MarketIdSetFilter(MarketIdSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PoolConfiguratorUpdatedFilter(PoolConfiguratorUpdatedFilter),
        PoolDataProviderUpdatedFilter(PoolDataProviderUpdatedFilter),
        PoolUpdatedFilter(PoolUpdatedFilter),
        PriceOracleSentinelUpdatedFilter(PriceOracleSentinelUpdatedFilter),
        PriceOracleUpdatedFilter(PriceOracleUpdatedFilter),
        ProxyCreatedFilter(ProxyCreatedFilter),
    }
    impl ethers::contract::EthLogDecode for PoolAddressesProviderEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AcladminUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AcladminUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AclmanagerUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AclmanagerUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = AddressSetAsProxyFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AddressSetAsProxyFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MarketIdSetFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::MarketIdSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PoolConfiguratorUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PoolDataProviderUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PoolUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleSentinelUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PriceOracleUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ProxyCreatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::ProxyCreatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PoolAddressesProviderEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolAddressesProviderEvents::AcladminUpdatedFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::AclmanagerUpdatedFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::AddressSetFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::AddressSetAsProxyFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::MarketIdSetFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderEvents::PoolUpdatedFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(element) => {
                    element.fmt(f)
                }
                PoolAddressesProviderEvents::PriceOracleUpdatedFilter(element) => element.fmt(f),
                PoolAddressesProviderEvents::ProxyCreatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `getACLAdmin` function with signature `getACLAdmin()` and selector `[14, 103, 23, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACLAdmin", abi = "getACLAdmin()")]
    pub struct GetACLAdminCall;
    #[doc = "Container type for all input parameters for the `getACLManager` function with signature `getACLManager()` and selector `[112, 124, 215, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACLManager", abi = "getACLManager()")]
    pub struct GetACLManagerCall;
    #[doc = "Container type for all input parameters for the `getAddress` function with signature `getAddress(bytes32)` and selector `[33, 248, 167, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAddress", abi = "getAddress(bytes32)")]
    pub struct GetAddressCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getMarketId` function with signature `getMarketId()` and selector `[86, 142, 244, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMarketId", abi = "getMarketId()")]
    pub struct GetMarketIdCall;
    #[doc = "Container type for all input parameters for the `getPool` function with signature `getPool()` and selector `[2, 107, 29, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPool", abi = "getPool()")]
    pub struct GetPoolCall;
    #[doc = "Container type for all input parameters for the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `[99, 26, 223, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolConfigurator", abi = "getPoolConfigurator()")]
    pub struct GetPoolConfiguratorCall;
    #[doc = "Container type for all input parameters for the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `[232, 96, 172, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolDataProvider", abi = "getPoolDataProvider()")]
    pub struct GetPoolDataProviderCall;
    #[doc = "Container type for all input parameters for the `getPriceOracle` function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracle", abi = "getPriceOracle()")]
    pub struct GetPriceOracleCall;
    #[doc = "Container type for all input parameters for the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `[94, 184, 141, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracleSentinel", abi = "getPriceOracleSentinel()")]
    pub struct GetPriceOracleSentinelCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setACLAdmin` function with signature `setACLAdmin(address)` and selector `[118, 216, 79, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setACLAdmin", abi = "setACLAdmin(address)")]
    pub struct SetACLAdminCall {
        pub new_acl_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setACLManager` function with signature `setACLManager(address)` and selector `[237, 48, 28, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setACLManager", abi = "setACLManager(address)")]
    pub struct SetACLManagerCall {
        pub new_acl_manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAddress` function with signature `setAddress(bytes32,address)` and selector `[202, 68, 109, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAddress", abi = "setAddress(bytes32,address)")]
    pub struct SetAddressCall {
        pub id: [u8; 32],
        pub new_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAddressAsProxy` function with signature `setAddressAsProxy(bytes32,address)` and selector `[93, 204, 82, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAddressAsProxy", abi = "setAddressAsProxy(bytes32,address)")]
    pub struct SetAddressAsProxyCall {
        pub id: [u8; 32],
        pub new_implementation_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMarketId` function with signature `setMarketId(string)` and selector `[246, 123, 24, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMarketId", abi = "setMarketId(string)")]
    pub struct SetMarketIdCall {
        pub new_market_id: String,
    }
    #[doc = "Container type for all input parameters for the `setPoolConfiguratorImpl` function with signature `setPoolConfiguratorImpl(address)` and selector `[228, 202, 40, 183]`"]
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
        name = "setPoolConfiguratorImpl",
        abi = "setPoolConfiguratorImpl(address)"
    )]
    pub struct SetPoolConfiguratorImplCall {
        pub new_pool_configurator_impl: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPoolDataProvider` function with signature `setPoolDataProvider(address)` and selector `[228, 78, 158, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolDataProvider", abi = "setPoolDataProvider(address)")]
    pub struct SetPoolDataProviderCall {
        pub new_data_provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPoolImpl` function with signature `setPoolImpl(address)` and selector `[161, 86, 68, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolImpl", abi = "setPoolImpl(address)")]
    pub struct SetPoolImplCall {
        pub new_pool_impl: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceOracle` function with signature `setPriceOracle(address)` and selector `[83, 14, 120, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPriceOracle", abi = "setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub new_price_oracle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceOracleSentinel` function with signature `setPriceOracleSentinel(address)` and selector `[116, 148, 76, 236]`"]
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
        name = "setPriceOracleSentinel",
        abi = "setPriceOracleSentinel(address)"
    )]
    pub struct SetPriceOracleSentinelCall {
        pub new_price_oracle_sentinel: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolAddressesProviderCalls {
        GetACLAdmin(GetACLAdminCall),
        GetACLManager(GetACLManagerCall),
        GetAddress(GetAddressCall),
        GetMarketId(GetMarketIdCall),
        GetPool(GetPoolCall),
        GetPoolConfigurator(GetPoolConfiguratorCall),
        GetPoolDataProvider(GetPoolDataProviderCall),
        GetPriceOracle(GetPriceOracleCall),
        GetPriceOracleSentinel(GetPriceOracleSentinelCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetACLAdmin(SetACLAdminCall),
        SetACLManager(SetACLManagerCall),
        SetAddress(SetAddressCall),
        SetAddressAsProxy(SetAddressAsProxyCall),
        SetMarketId(SetMarketIdCall),
        SetPoolConfiguratorImpl(SetPoolConfiguratorImplCall),
        SetPoolDataProvider(SetPoolDataProviderCall),
        SetPoolImpl(SetPoolImplCall),
        SetPriceOracle(SetPriceOracleCall),
        SetPriceOracleSentinel(SetPriceOracleSentinelCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for PoolAddressesProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetACLAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetACLAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetACLManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetACLManager(decoded));
            }
            if let Ok(decoded) =
                <GetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetAddress(decoded));
            }
            if let Ok(decoded) =
                <GetMarketIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetMarketId(decoded));
            }
            if let Ok(decoded) =
                <GetPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetPool(decoded));
            }
            if let Ok(decoded) =
                <GetPoolConfiguratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetPoolConfigurator(decoded));
            }
            if let Ok(decoded) =
                <GetPoolDataProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetPoolDataProvider(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleSentinelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::GetPriceOracleSentinel(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetACLAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetACLAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetACLManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetACLManager(decoded));
            }
            if let Ok(decoded) =
                <SetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetAddress(decoded));
            }
            if let Ok(decoded) =
                <SetAddressAsProxyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetAddressAsProxy(decoded));
            }
            if let Ok(decoded) =
                <SetMarketIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetMarketId(decoded));
            }
            if let Ok(decoded) =
                <SetPoolConfiguratorImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetPoolConfiguratorImpl(decoded));
            }
            if let Ok(decoded) =
                <SetPoolDataProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetPoolDataProvider(decoded));
            }
            if let Ok(decoded) =
                <SetPoolImplCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetPoolImpl(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleSentinelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::SetPriceOracleSentinel(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolAddressesProviderCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PoolAddressesProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PoolAddressesProviderCalls::GetACLAdmin(element) => element.encode(),
                PoolAddressesProviderCalls::GetACLManager(element) => element.encode(),
                PoolAddressesProviderCalls::GetAddress(element) => element.encode(),
                PoolAddressesProviderCalls::GetMarketId(element) => element.encode(),
                PoolAddressesProviderCalls::GetPool(element) => element.encode(),
                PoolAddressesProviderCalls::GetPoolConfigurator(element) => element.encode(),
                PoolAddressesProviderCalls::GetPoolDataProvider(element) => element.encode(),
                PoolAddressesProviderCalls::GetPriceOracle(element) => element.encode(),
                PoolAddressesProviderCalls::GetPriceOracleSentinel(element) => element.encode(),
                PoolAddressesProviderCalls::Owner(element) => element.encode(),
                PoolAddressesProviderCalls::RenounceOwnership(element) => element.encode(),
                PoolAddressesProviderCalls::SetACLAdmin(element) => element.encode(),
                PoolAddressesProviderCalls::SetACLManager(element) => element.encode(),
                PoolAddressesProviderCalls::SetAddress(element) => element.encode(),
                PoolAddressesProviderCalls::SetAddressAsProxy(element) => element.encode(),
                PoolAddressesProviderCalls::SetMarketId(element) => element.encode(),
                PoolAddressesProviderCalls::SetPoolConfiguratorImpl(element) => element.encode(),
                PoolAddressesProviderCalls::SetPoolDataProvider(element) => element.encode(),
                PoolAddressesProviderCalls::SetPoolImpl(element) => element.encode(),
                PoolAddressesProviderCalls::SetPriceOracle(element) => element.encode(),
                PoolAddressesProviderCalls::SetPriceOracleSentinel(element) => element.encode(),
                PoolAddressesProviderCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PoolAddressesProviderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolAddressesProviderCalls::GetACLAdmin(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetACLManager(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetAddress(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetMarketId(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetPool(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetPoolConfigurator(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetPoolDataProvider(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetPriceOracle(element) => element.fmt(f),
                PoolAddressesProviderCalls::GetPriceOracleSentinel(element) => element.fmt(f),
                PoolAddressesProviderCalls::Owner(element) => element.fmt(f),
                PoolAddressesProviderCalls::RenounceOwnership(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetACLAdmin(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetACLManager(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetAddress(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetAddressAsProxy(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetMarketId(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetPoolConfiguratorImpl(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetPoolDataProvider(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetPoolImpl(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetPriceOracle(element) => element.fmt(f),
                PoolAddressesProviderCalls::SetPriceOracleSentinel(element) => element.fmt(f),
                PoolAddressesProviderCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetACLAdminCall> for PoolAddressesProviderCalls {
        fn from(var: GetACLAdminCall) -> Self {
            PoolAddressesProviderCalls::GetACLAdmin(var)
        }
    }
    impl ::std::convert::From<GetACLManagerCall> for PoolAddressesProviderCalls {
        fn from(var: GetACLManagerCall) -> Self {
            PoolAddressesProviderCalls::GetACLManager(var)
        }
    }
    impl ::std::convert::From<GetAddressCall> for PoolAddressesProviderCalls {
        fn from(var: GetAddressCall) -> Self {
            PoolAddressesProviderCalls::GetAddress(var)
        }
    }
    impl ::std::convert::From<GetMarketIdCall> for PoolAddressesProviderCalls {
        fn from(var: GetMarketIdCall) -> Self {
            PoolAddressesProviderCalls::GetMarketId(var)
        }
    }
    impl ::std::convert::From<GetPoolCall> for PoolAddressesProviderCalls {
        fn from(var: GetPoolCall) -> Self {
            PoolAddressesProviderCalls::GetPool(var)
        }
    }
    impl ::std::convert::From<GetPoolConfiguratorCall> for PoolAddressesProviderCalls {
        fn from(var: GetPoolConfiguratorCall) -> Self {
            PoolAddressesProviderCalls::GetPoolConfigurator(var)
        }
    }
    impl ::std::convert::From<GetPoolDataProviderCall> for PoolAddressesProviderCalls {
        fn from(var: GetPoolDataProviderCall) -> Self {
            PoolAddressesProviderCalls::GetPoolDataProvider(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleCall> for PoolAddressesProviderCalls {
        fn from(var: GetPriceOracleCall) -> Self {
            PoolAddressesProviderCalls::GetPriceOracle(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleSentinelCall> for PoolAddressesProviderCalls {
        fn from(var: GetPriceOracleSentinelCall) -> Self {
            PoolAddressesProviderCalls::GetPriceOracleSentinel(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PoolAddressesProviderCalls {
        fn from(var: OwnerCall) -> Self {
            PoolAddressesProviderCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PoolAddressesProviderCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PoolAddressesProviderCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetACLAdminCall> for PoolAddressesProviderCalls {
        fn from(var: SetACLAdminCall) -> Self {
            PoolAddressesProviderCalls::SetACLAdmin(var)
        }
    }
    impl ::std::convert::From<SetACLManagerCall> for PoolAddressesProviderCalls {
        fn from(var: SetACLManagerCall) -> Self {
            PoolAddressesProviderCalls::SetACLManager(var)
        }
    }
    impl ::std::convert::From<SetAddressCall> for PoolAddressesProviderCalls {
        fn from(var: SetAddressCall) -> Self {
            PoolAddressesProviderCalls::SetAddress(var)
        }
    }
    impl ::std::convert::From<SetAddressAsProxyCall> for PoolAddressesProviderCalls {
        fn from(var: SetAddressAsProxyCall) -> Self {
            PoolAddressesProviderCalls::SetAddressAsProxy(var)
        }
    }
    impl ::std::convert::From<SetMarketIdCall> for PoolAddressesProviderCalls {
        fn from(var: SetMarketIdCall) -> Self {
            PoolAddressesProviderCalls::SetMarketId(var)
        }
    }
    impl ::std::convert::From<SetPoolConfiguratorImplCall> for PoolAddressesProviderCalls {
        fn from(var: SetPoolConfiguratorImplCall) -> Self {
            PoolAddressesProviderCalls::SetPoolConfiguratorImpl(var)
        }
    }
    impl ::std::convert::From<SetPoolDataProviderCall> for PoolAddressesProviderCalls {
        fn from(var: SetPoolDataProviderCall) -> Self {
            PoolAddressesProviderCalls::SetPoolDataProvider(var)
        }
    }
    impl ::std::convert::From<SetPoolImplCall> for PoolAddressesProviderCalls {
        fn from(var: SetPoolImplCall) -> Self {
            PoolAddressesProviderCalls::SetPoolImpl(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for PoolAddressesProviderCalls {
        fn from(var: SetPriceOracleCall) -> Self {
            PoolAddressesProviderCalls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleSentinelCall> for PoolAddressesProviderCalls {
        fn from(var: SetPriceOracleSentinelCall) -> Self {
            PoolAddressesProviderCalls::SetPriceOracleSentinel(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PoolAddressesProviderCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PoolAddressesProviderCalls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `getACLAdmin` function with signature `getACLAdmin()` and selector `[14, 103, 23, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetACLAdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getACLManager` function with signature `getACLManager()` and selector `[112, 124, 215, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetACLManagerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAddress` function with signature `getAddress(bytes32)` and selector `[33, 248, 167, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMarketId` function with signature `getMarketId()` and selector `[86, 142, 244, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetMarketIdReturn(pub String);
    #[doc = "Container type for all return fields from the `getPool` function with signature `getPool()` and selector `[2, 107, 29, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `[99, 26, 223, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolConfiguratorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `[232, 96, 172, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPoolDataProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPriceOracle` function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriceOracleReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `[94, 184, 141, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetPriceOracleSentinelReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
}
