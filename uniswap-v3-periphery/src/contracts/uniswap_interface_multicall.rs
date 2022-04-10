pub use uniswapinterfacemulticall_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapinterfacemulticall_mod {
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
    #[doc = "UniswapInterfaceMulticall was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPINTERFACEMULTICALL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct UniswapInterfaceMulticall.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct UniswapInterfaceMulticall.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasUsed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNISWAPINTERFACEMULTICALL_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b5061050f806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80630f28c97d146100465780631749e1e3146100645780634d2301cc14610085575b600080fd5b61004e610098565b60405161005b91906103da565b60405180910390f35b610077610072366004610280565b61009c565b60405161005b9291906103e3565b61004e61009336600461025f565b610213565b4290565b8051439060609067ffffffffffffffff811180156100b957600080fd5b506040519080825280602002602001820160405280156100f357816020015b6100e0610220565b8152602001906001900390816100d85790505b50905060005b835181101561020d57600080600086848151811061011357fe5b60200260200101516000015187858151811061012b57fe5b60200260200101516020015188868151811061014357fe5b60200260200101516040015192509250925060005a9050600080856001600160a01b0316858560405161017691906103be565b60006040518083038160008787f1925050503d80600081146101b4576040519150601f19603f3d011682016040523d82523d6000602084013e6101b9565b606091505b509150915060005a8403905060405180606001604052808415158152602001828152602001838152508989815181106101ee57fe5b60200260200101819052505050505050505080806001019150506100f9565b50915091565b6001600160a01b03163190565b604051806060016040528060001515815260200160008152602001606081525090565b80356001600160a01b038116811461025a57600080fd5b919050565b600060208284031215610270578081fd5b61027982610243565b9392505050565b60006020808385031215610292578182fd5b823567ffffffffffffffff808211156102a9578384fd5b818501915085601f8301126102bc578384fd5b8135818111156102c857fe5b6102d58485830201610485565b81815284810190848601875b848110156103af5781358701601f196060828e0382011215610301578a8bfd5b60408051606081018181108b8211171561031757fe5b8252610324848d01610243565b8152818401358c82015260608401358a81111561033f578d8efd5b8085019450508e603f850112610353578c8dfd5b8b8401358a81111561036157fe5b6103718d85601f84011601610485565b93508084528f83828701011115610386578d8efd5b808386018e86013783018c018d90529081019190915285525092870192908701906001016102e1565b50909998505050505050505050565b600082516103d08184602087016104a9565b9190910192915050565b90815260200190565b600060408083018584526020828186015281865180845260609350838701915083838202880101838901875b8381101561047557898303605f19018552815180511515845286810151878501528801518884018890528051888501819052608061045282828801858c016104a9565b96880196601f91909101601f19169490940190930192509085019060010161040f565b50909a9950505050505050505050565b60405181810167ffffffffffffffff811182821017156104a157fe5b604052919050565b60005b838110156104c45781810151838201526020016104ac565b838111156104d3576000848401525b5050505056fea2646970667358221220df4359d717046def576d2ec43085566394087c24c3271799677dce5d24c4b07f64736f6c63430007060033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct UniswapInterfaceMulticall<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapInterfaceMulticall<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapInterfaceMulticall<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapInterfaceMulticall))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapInterfaceMulticall<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                UNISWAPINTERFACEMULTICALL_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function"]
        pub fn get_current_block_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEthBalance` (0x4d2301cc) function"]
        pub fn get_eth_balance(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0x1749e1e3) function"]
        pub fn multicall(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([23, 73, 225, 227], calls)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for UniswapInterfaceMulticall<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getCurrentBlockTimestamp`function with signature `getCurrentBlockTimestamp()` and selector `[15, 40, 201, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    #[doc = "Container type for all input parameters for the `getEthBalance`function with signature `getEthBalance(address)` and selector `[77, 35, 1, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `multicall`function with signature `multicall((address,uint256,bytes)[])` and selector `[23, 73, 225, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "multicall", abi = "multicall((address,uint256,bytes)[])")]
    pub struct MulticallCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapInterfaceMulticallCalls {
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        Multicall(MulticallCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapInterfaceMulticallCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCurrentBlockTimestampCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapInterfaceMulticallCalls::GetCurrentBlockTimestamp(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetEthBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapInterfaceMulticallCalls::GetEthBalance(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapInterfaceMulticallCalls::Multicall(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapInterfaceMulticallCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapInterfaceMulticallCalls::GetCurrentBlockTimestamp(element) => {
                    element.encode()
                }
                UniswapInterfaceMulticallCalls::GetEthBalance(element) => element.encode(),
                UniswapInterfaceMulticallCalls::Multicall(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapInterfaceMulticallCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapInterfaceMulticallCalls::GetCurrentBlockTimestamp(element) => element.fmt(f),
                UniswapInterfaceMulticallCalls::GetEthBalance(element) => element.fmt(f),
                UniswapInterfaceMulticallCalls::Multicall(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCurrentBlockTimestampCall> for UniswapInterfaceMulticallCalls {
        fn from(var: GetCurrentBlockTimestampCall) -> Self {
            UniswapInterfaceMulticallCalls::GetCurrentBlockTimestamp(var)
        }
    }
    impl ::std::convert::From<GetEthBalanceCall> for UniswapInterfaceMulticallCalls {
        fn from(var: GetEthBalanceCall) -> Self {
            UniswapInterfaceMulticallCalls::GetEthBalance(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for UniswapInterfaceMulticallCalls {
        fn from(var: MulticallCall) -> Self {
            UniswapInterfaceMulticallCalls::Multicall(var)
        }
    }
    #[doc = "`Call(address,uint256,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Call {
        pub target: ethers::core::types::Address,
        pub gas_limit: ethers::core::types::U256,
        pub call_data: ethers::core::types::Bytes,
    }
    #[doc = "`Result(bool,uint256,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Result {
        pub success: bool,
        pub gas_used: ethers::core::types::U256,
        pub return_data: ethers::core::types::Bytes,
    }
}
