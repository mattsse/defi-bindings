pub use versioned_initializable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod versioned_initializable {
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
    #[doc = "VersionedInitializable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VERSIONEDINITIALIZABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str("[]").expect("invalid abi")
        });
    pub struct VersionedInitializable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VersionedInitializable<M> {
        fn clone(&self) -> Self {
            VersionedInitializable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VersionedInitializable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VersionedInitializable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VersionedInitializable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VersionedInitializable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                VERSIONEDINITIALIZABLE_ABI.clone(),
                client,
            )
            .into()
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for VersionedInitializable<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
}
