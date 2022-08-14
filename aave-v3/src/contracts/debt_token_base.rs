pub use debt_token_base::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod debt_token_base {
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
    #[doc = "DebtTokenBase was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DEBTTOKENBASE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowAllowanceDelegated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_WITH_SIG_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EIP712_REVISION\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveDelegation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"fromUser\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"toUser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowAllowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegationWithSig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct DebtTokenBase<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DebtTokenBase<M> {
        fn clone(&self) -> Self {
            DebtTokenBase(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DebtTokenBase<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DebtTokenBase<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DebtTokenBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DebtTokenBase<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DEBTTOKENBASE_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BorrowAllowanceDelegated` event"]
        pub fn borrow_allowance_delegated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowAllowanceDelegatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowAllowanceDelegatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for DebtTokenBase<M> {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DebtTokenBaseCalls {
        DelegationWithSigTypehash(DelegationWithSigTypehashCall),
        DomainSeparator(DomainSeparatorCall),
        Eip712Revision(Eip712RevisionCall),
        ApproveDelegation(ApproveDelegationCall),
        BorrowAllowance(BorrowAllowanceCall),
        DelegationWithSig(DelegationWithSigCall),
        Nonces(NoncesCall),
    }
    impl ethers::core::abi::AbiDecode for DebtTokenBaseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DelegationWithSigTypehashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DebtTokenBaseCalls::DelegationWithSigTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <Eip712RevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::Eip712Revision(decoded));
            }
            if let Ok(decoded) =
                <ApproveDelegationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::ApproveDelegation(decoded));
            }
            if let Ok(decoded) =
                <BorrowAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::BorrowAllowance(decoded));
            }
            if let Ok(decoded) =
                <DelegationWithSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::DelegationWithSig(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DebtTokenBaseCalls::Nonces(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DebtTokenBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DebtTokenBaseCalls::DelegationWithSigTypehash(element) => element.encode(),
                DebtTokenBaseCalls::DomainSeparator(element) => element.encode(),
                DebtTokenBaseCalls::Eip712Revision(element) => element.encode(),
                DebtTokenBaseCalls::ApproveDelegation(element) => element.encode(),
                DebtTokenBaseCalls::BorrowAllowance(element) => element.encode(),
                DebtTokenBaseCalls::DelegationWithSig(element) => element.encode(),
                DebtTokenBaseCalls::Nonces(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DebtTokenBaseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DebtTokenBaseCalls::DelegationWithSigTypehash(element) => element.fmt(f),
                DebtTokenBaseCalls::DomainSeparator(element) => element.fmt(f),
                DebtTokenBaseCalls::Eip712Revision(element) => element.fmt(f),
                DebtTokenBaseCalls::ApproveDelegation(element) => element.fmt(f),
                DebtTokenBaseCalls::BorrowAllowance(element) => element.fmt(f),
                DebtTokenBaseCalls::DelegationWithSig(element) => element.fmt(f),
                DebtTokenBaseCalls::Nonces(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DelegationWithSigTypehashCall> for DebtTokenBaseCalls {
        fn from(var: DelegationWithSigTypehashCall) -> Self {
            DebtTokenBaseCalls::DelegationWithSigTypehash(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for DebtTokenBaseCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            DebtTokenBaseCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<Eip712RevisionCall> for DebtTokenBaseCalls {
        fn from(var: Eip712RevisionCall) -> Self {
            DebtTokenBaseCalls::Eip712Revision(var)
        }
    }
    impl ::std::convert::From<ApproveDelegationCall> for DebtTokenBaseCalls {
        fn from(var: ApproveDelegationCall) -> Self {
            DebtTokenBaseCalls::ApproveDelegation(var)
        }
    }
    impl ::std::convert::From<BorrowAllowanceCall> for DebtTokenBaseCalls {
        fn from(var: BorrowAllowanceCall) -> Self {
            DebtTokenBaseCalls::BorrowAllowance(var)
        }
    }
    impl ::std::convert::From<DelegationWithSigCall> for DebtTokenBaseCalls {
        fn from(var: DelegationWithSigCall) -> Self {
            DebtTokenBaseCalls::DelegationWithSig(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for DebtTokenBaseCalls {
        fn from(var: NoncesCall) -> Self {
            DebtTokenBaseCalls::Nonces(var)
        }
    }
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
}
