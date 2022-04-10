pub use iaclmanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaclmanager_mod {
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
    #[doc = "IACLManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACLMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ASSET_LISTING_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BRIDGE_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EMERGENCY_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FLASH_BORROWER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RISK_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAssetListingAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addEmergencyAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addFlashBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPoolAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addRiskAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAssetListingAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBridge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isFlashBorrower\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPoolAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isRiskAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeAssetListingAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeEmergencyAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeFlashBorrower\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removePoolAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRiskAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"adminRole\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRoleAdmin\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACLMANAGER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IACLManager<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IACLManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IACLManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IACLManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IACLManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IACLMANAGER_ABI.clone(), client).into()
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
                IACLMANAGER_ABI.clone(),
                IACLMANAGER_BYTECODE.clone().into(),
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IACLManager<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER`function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
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
    #[doc = "Container type for all input parameters for the `ASSET_LISTING_ADMIN_ROLE`function with signature `ASSET_LISTING_ADMIN_ROLE()` and selector `[120, 187, 10, 67]`"]
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
    #[doc = "Container type for all input parameters for the `BRIDGE_ROLE`function with signature `BRIDGE_ROLE()` and selector `[181, 191, 221, 234]`"]
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
    #[doc = "Container type for all input parameters for the `EMERGENCY_ADMIN_ROLE`function with signature `EMERGENCY_ADMIN_ROLE()` and selector `[110, 118, 252, 143]`"]
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
    #[doc = "Container type for all input parameters for the `FLASH_BORROWER_ROLE`function with signature `FLASH_BORROWER_ROLE()` and selector `[85, 119, 183, 169]`"]
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
    #[doc = "Container type for all input parameters for the `POOL_ADMIN_ROLE`function with signature `POOL_ADMIN_ROLE()` and selector `[184, 246, 219, 167]`"]
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
    #[doc = "Container type for all input parameters for the `RISK_ADMIN_ROLE`function with signature `RISK_ADMIN_ROLE()` and selector `[79, 22, 180, 37]`"]
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
    #[doc = "Container type for all input parameters for the `addAssetListingAdmin`function with signature `addAssetListingAdmin(address)` and selector `[154, 43, 150, 247]`"]
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
    #[doc = "Container type for all input parameters for the `addBridge`function with signature `addBridge(address)` and selector `[151, 18, 253, 248]`"]
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
    #[doc = "Container type for all input parameters for the `addEmergencyAdmin`function with signature `addEmergencyAdmin(address)` and selector `[23, 158, 251, 9]`"]
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
    #[doc = "Container type for all input parameters for the `addFlashBorrower`function with signature `addFlashBorrower(address)` and selector `[154, 201, 216, 11]`"]
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
    #[doc = "Container type for all input parameters for the `addPoolAdmin`function with signature `addPoolAdmin(address)` and selector `[34, 101, 12, 175]`"]
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
    #[doc = "Container type for all input parameters for the `addRiskAdmin`function with signature `addRiskAdmin(address)` and selector `[91, 154, 148, 228]`"]
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
    #[doc = "Container type for all input parameters for the `isAssetListingAdmin`function with signature `isAssetListingAdmin(address)` and selector `[19, 238, 50, 224]`"]
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
    #[doc = "Container type for all input parameters for the `isBridge`function with signature `isBridge(address)` and selector `[114, 102, 0, 206]`"]
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
    #[doc = "Container type for all input parameters for the `isEmergencyAdmin`function with signature `isEmergencyAdmin(address)` and selector `[37, 0, 242, 182]`"]
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
    #[doc = "Container type for all input parameters for the `isFlashBorrower`function with signature `isFlashBorrower(address)` and selector `[250, 80, 242, 151]`"]
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
    #[doc = "Container type for all input parameters for the `isPoolAdmin`function with signature `isPoolAdmin(address)` and selector `[123, 229, 60, 161]`"]
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
    #[doc = "Container type for all input parameters for the `isRiskAdmin`function with signature `isRiskAdmin(address)` and selector `[103, 75, 94, 77]`"]
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
    #[doc = "Container type for all input parameters for the `removeAssetListingAdmin`function with signature `removeAssetListingAdmin(address)` and selector `[162, 27, 206, 21]`"]
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
    #[doc = "Container type for all input parameters for the `removeBridge`function with signature `removeBridge(address)` and selector `[4, 223, 1, 125]`"]
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
    #[doc = "Container type for all input parameters for the `removeEmergencyAdmin`function with signature `removeEmergencyAdmin(address)` and selector `[122, 154, 147, 244]`"]
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
    #[doc = "Container type for all input parameters for the `removeFlashBorrower`function with signature `removeFlashBorrower(address)` and selector `[37, 60, 249, 128]`"]
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
    #[doc = "Container type for all input parameters for the `removePoolAdmin`function with signature `removePoolAdmin(address)` and selector `[248, 54, 149, 203]`"]
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
    #[doc = "Container type for all input parameters for the `removeRiskAdmin`function with signature `removeRiskAdmin(address)` and selector `[60, 90, 8, 229]`"]
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
    #[doc = "Container type for all input parameters for the `setRoleAdmin`function with signature `setRoleAdmin(bytes32,bytes32)` and selector `[30, 78, 0, 145]`"]
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IACLManagerCalls {
        AddressesProvider(AddressesProviderCall),
        AssetListingAdminRole(AssetListingAdminRoleCall),
        BridgeRole(BridgeRoleCall),
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
        SetRoleAdmin(SetRoleAdminCall),
    }
    impl ethers::core::abi::AbiDecode for IACLManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <AssetListingAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AssetListingAdminRole(decoded));
            }
            if let Ok(decoded) =
                <BridgeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::BridgeRole(decoded));
            }
            if let Ok(decoded) =
                <EmergencyAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::EmergencyAdminRole(decoded));
            }
            if let Ok(decoded) =
                <FlashBorrowerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::FlashBorrowerRole(decoded));
            }
            if let Ok(decoded) =
                <PoolAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::PoolAdminRole(decoded));
            }
            if let Ok(decoded) =
                <RiskAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RiskAdminRole(decoded));
            }
            if let Ok(decoded) =
                <AddAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddBridge(decoded));
            }
            if let Ok(decoded) =
                <AddEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <AddPoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::AddRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsBridge(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <IsPoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::IsRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveAssetListingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemoveAssetListingAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemoveBridge(decoded));
            }
            if let Ok(decoded) =
                <RemoveEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemoveEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveFlashBorrowerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemoveFlashBorrower(decoded));
            }
            if let Ok(decoded) =
                <RemovePoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemovePoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveRiskAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::RemoveRiskAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLManagerCalls::SetRoleAdmin(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IACLManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IACLManagerCalls::AddressesProvider(element) => element.encode(),
                IACLManagerCalls::AssetListingAdminRole(element) => element.encode(),
                IACLManagerCalls::BridgeRole(element) => element.encode(),
                IACLManagerCalls::EmergencyAdminRole(element) => element.encode(),
                IACLManagerCalls::FlashBorrowerRole(element) => element.encode(),
                IACLManagerCalls::PoolAdminRole(element) => element.encode(),
                IACLManagerCalls::RiskAdminRole(element) => element.encode(),
                IACLManagerCalls::AddAssetListingAdmin(element) => element.encode(),
                IACLManagerCalls::AddBridge(element) => element.encode(),
                IACLManagerCalls::AddEmergencyAdmin(element) => element.encode(),
                IACLManagerCalls::AddFlashBorrower(element) => element.encode(),
                IACLManagerCalls::AddPoolAdmin(element) => element.encode(),
                IACLManagerCalls::AddRiskAdmin(element) => element.encode(),
                IACLManagerCalls::IsAssetListingAdmin(element) => element.encode(),
                IACLManagerCalls::IsBridge(element) => element.encode(),
                IACLManagerCalls::IsEmergencyAdmin(element) => element.encode(),
                IACLManagerCalls::IsFlashBorrower(element) => element.encode(),
                IACLManagerCalls::IsPoolAdmin(element) => element.encode(),
                IACLManagerCalls::IsRiskAdmin(element) => element.encode(),
                IACLManagerCalls::RemoveAssetListingAdmin(element) => element.encode(),
                IACLManagerCalls::RemoveBridge(element) => element.encode(),
                IACLManagerCalls::RemoveEmergencyAdmin(element) => element.encode(),
                IACLManagerCalls::RemoveFlashBorrower(element) => element.encode(),
                IACLManagerCalls::RemovePoolAdmin(element) => element.encode(),
                IACLManagerCalls::RemoveRiskAdmin(element) => element.encode(),
                IACLManagerCalls::SetRoleAdmin(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IACLManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IACLManagerCalls::AddressesProvider(element) => element.fmt(f),
                IACLManagerCalls::AssetListingAdminRole(element) => element.fmt(f),
                IACLManagerCalls::BridgeRole(element) => element.fmt(f),
                IACLManagerCalls::EmergencyAdminRole(element) => element.fmt(f),
                IACLManagerCalls::FlashBorrowerRole(element) => element.fmt(f),
                IACLManagerCalls::PoolAdminRole(element) => element.fmt(f),
                IACLManagerCalls::RiskAdminRole(element) => element.fmt(f),
                IACLManagerCalls::AddAssetListingAdmin(element) => element.fmt(f),
                IACLManagerCalls::AddBridge(element) => element.fmt(f),
                IACLManagerCalls::AddEmergencyAdmin(element) => element.fmt(f),
                IACLManagerCalls::AddFlashBorrower(element) => element.fmt(f),
                IACLManagerCalls::AddPoolAdmin(element) => element.fmt(f),
                IACLManagerCalls::AddRiskAdmin(element) => element.fmt(f),
                IACLManagerCalls::IsAssetListingAdmin(element) => element.fmt(f),
                IACLManagerCalls::IsBridge(element) => element.fmt(f),
                IACLManagerCalls::IsEmergencyAdmin(element) => element.fmt(f),
                IACLManagerCalls::IsFlashBorrower(element) => element.fmt(f),
                IACLManagerCalls::IsPoolAdmin(element) => element.fmt(f),
                IACLManagerCalls::IsRiskAdmin(element) => element.fmt(f),
                IACLManagerCalls::RemoveAssetListingAdmin(element) => element.fmt(f),
                IACLManagerCalls::RemoveBridge(element) => element.fmt(f),
                IACLManagerCalls::RemoveEmergencyAdmin(element) => element.fmt(f),
                IACLManagerCalls::RemoveFlashBorrower(element) => element.fmt(f),
                IACLManagerCalls::RemovePoolAdmin(element) => element.fmt(f),
                IACLManagerCalls::RemoveRiskAdmin(element) => element.fmt(f),
                IACLManagerCalls::SetRoleAdmin(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for IACLManagerCalls {
        fn from(var: AddressesProviderCall) -> Self {
            IACLManagerCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<AssetListingAdminRoleCall> for IACLManagerCalls {
        fn from(var: AssetListingAdminRoleCall) -> Self {
            IACLManagerCalls::AssetListingAdminRole(var)
        }
    }
    impl ::std::convert::From<BridgeRoleCall> for IACLManagerCalls {
        fn from(var: BridgeRoleCall) -> Self {
            IACLManagerCalls::BridgeRole(var)
        }
    }
    impl ::std::convert::From<EmergencyAdminRoleCall> for IACLManagerCalls {
        fn from(var: EmergencyAdminRoleCall) -> Self {
            IACLManagerCalls::EmergencyAdminRole(var)
        }
    }
    impl ::std::convert::From<FlashBorrowerRoleCall> for IACLManagerCalls {
        fn from(var: FlashBorrowerRoleCall) -> Self {
            IACLManagerCalls::FlashBorrowerRole(var)
        }
    }
    impl ::std::convert::From<PoolAdminRoleCall> for IACLManagerCalls {
        fn from(var: PoolAdminRoleCall) -> Self {
            IACLManagerCalls::PoolAdminRole(var)
        }
    }
    impl ::std::convert::From<RiskAdminRoleCall> for IACLManagerCalls {
        fn from(var: RiskAdminRoleCall) -> Self {
            IACLManagerCalls::RiskAdminRole(var)
        }
    }
    impl ::std::convert::From<AddAssetListingAdminCall> for IACLManagerCalls {
        fn from(var: AddAssetListingAdminCall) -> Self {
            IACLManagerCalls::AddAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<AddBridgeCall> for IACLManagerCalls {
        fn from(var: AddBridgeCall) -> Self {
            IACLManagerCalls::AddBridge(var)
        }
    }
    impl ::std::convert::From<AddEmergencyAdminCall> for IACLManagerCalls {
        fn from(var: AddEmergencyAdminCall) -> Self {
            IACLManagerCalls::AddEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<AddFlashBorrowerCall> for IACLManagerCalls {
        fn from(var: AddFlashBorrowerCall) -> Self {
            IACLManagerCalls::AddFlashBorrower(var)
        }
    }
    impl ::std::convert::From<AddPoolAdminCall> for IACLManagerCalls {
        fn from(var: AddPoolAdminCall) -> Self {
            IACLManagerCalls::AddPoolAdmin(var)
        }
    }
    impl ::std::convert::From<AddRiskAdminCall> for IACLManagerCalls {
        fn from(var: AddRiskAdminCall) -> Self {
            IACLManagerCalls::AddRiskAdmin(var)
        }
    }
    impl ::std::convert::From<IsAssetListingAdminCall> for IACLManagerCalls {
        fn from(var: IsAssetListingAdminCall) -> Self {
            IACLManagerCalls::IsAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<IsBridgeCall> for IACLManagerCalls {
        fn from(var: IsBridgeCall) -> Self {
            IACLManagerCalls::IsBridge(var)
        }
    }
    impl ::std::convert::From<IsEmergencyAdminCall> for IACLManagerCalls {
        fn from(var: IsEmergencyAdminCall) -> Self {
            IACLManagerCalls::IsEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<IsFlashBorrowerCall> for IACLManagerCalls {
        fn from(var: IsFlashBorrowerCall) -> Self {
            IACLManagerCalls::IsFlashBorrower(var)
        }
    }
    impl ::std::convert::From<IsPoolAdminCall> for IACLManagerCalls {
        fn from(var: IsPoolAdminCall) -> Self {
            IACLManagerCalls::IsPoolAdmin(var)
        }
    }
    impl ::std::convert::From<IsRiskAdminCall> for IACLManagerCalls {
        fn from(var: IsRiskAdminCall) -> Self {
            IACLManagerCalls::IsRiskAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveAssetListingAdminCall> for IACLManagerCalls {
        fn from(var: RemoveAssetListingAdminCall) -> Self {
            IACLManagerCalls::RemoveAssetListingAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveBridgeCall> for IACLManagerCalls {
        fn from(var: RemoveBridgeCall) -> Self {
            IACLManagerCalls::RemoveBridge(var)
        }
    }
    impl ::std::convert::From<RemoveEmergencyAdminCall> for IACLManagerCalls {
        fn from(var: RemoveEmergencyAdminCall) -> Self {
            IACLManagerCalls::RemoveEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveFlashBorrowerCall> for IACLManagerCalls {
        fn from(var: RemoveFlashBorrowerCall) -> Self {
            IACLManagerCalls::RemoveFlashBorrower(var)
        }
    }
    impl ::std::convert::From<RemovePoolAdminCall> for IACLManagerCalls {
        fn from(var: RemovePoolAdminCall) -> Self {
            IACLManagerCalls::RemovePoolAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveRiskAdminCall> for IACLManagerCalls {
        fn from(var: RemoveRiskAdminCall) -> Self {
            IACLManagerCalls::RemoveRiskAdmin(var)
        }
    }
    impl ::std::convert::From<SetRoleAdminCall> for IACLManagerCalls {
        fn from(var: SetRoleAdminCall) -> Self {
            IACLManagerCalls::SetRoleAdmin(var)
        }
    }
}
