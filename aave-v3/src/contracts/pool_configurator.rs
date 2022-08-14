pub use pool_configurator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod pool_configurator {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "PoolConfigurator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static POOLCONFIGURATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ATokenUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldBorrowCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBorrowCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowCapChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"borrowable\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BorrowableInIsolationChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"oldBridgeProtocolFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBridgeProtocolFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BridgeProtocolFeeUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"ltv\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationBonus\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CollateralConfigurationChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldDebtCeiling\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newDebtCeiling\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DebtCeilingChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"oldCategoryId\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"newCategoryId\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EModeAssetCategoryChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"categoryId\",\"type\":\"uint8\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"ltv\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationBonus\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"label\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EModeCategoryAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"oldFlashloanPremiumToProtocol\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"newFlashloanPremiumToProtocol\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FlashloanPremiumToProtocolUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"oldFlashloanPremiumTotal\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"newFlashloanPremiumTotal\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FlashloanPremiumTotalUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidationProtocolFeeChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"active\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveActive\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveBorrowing\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ReserveDropped\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldReserveFactor\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newReserveFactor\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveFactorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"frozen\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveFrozen\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"aToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"stableDebtToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"variableDebtToken\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveInitialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldStrategy\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"newStrategy\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveInterestRateStrategyChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservePaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveStableRateBorrowing\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"oldState\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"newState\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SiloedBorrowingChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"StableDebtTokenUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldSupplyCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newSupplyCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SupplyCapChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldUnbackedMintCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newUnbackedMintCap\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UnbackedMintCapChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proxy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VariableDebtTokenUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CONFIGURATOR_REVISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ltv\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationBonus\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureReserveAsCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropReserve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ConfiguratorInputTypes.InitReserveInput[]\",\"name\":\"input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"aTokenImpl\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stableDebtTokenImpl\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"variableDebtTokenImpl\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"underlyingAssetDecimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlyingAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"aTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"variableDebtTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"variableDebtTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"stableDebtTokenName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"stableDebtTokenSymbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initReserves\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"provider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"newCategoryId\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAssetEModeCategory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newBorrowCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"borrowable\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBorrowableInIsolation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newDebtCeiling\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDebtCeiling\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"categoryId\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"ltv\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationThreshold\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationBonus\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"label\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEModeCategory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidationProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolPause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"active\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveActive\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveBorrowing\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newReserveFactor\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveFactor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"freeze\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveFreeze\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newRateStrategyAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveInterestRateStrategyAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReservePause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveStableRateBorrowing\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"newSiloed\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSiloedBorrowing\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newSupplyCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSupplyCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newUnbackedMintCap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUnbackedMintCap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ConfiguratorInputTypes.UpdateATokenInput\",\"name\":\"input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"treasury\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateAToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newBridgeProtocolFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateBridgeProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newFlashloanPremiumToProtocol\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFlashloanPremiumToProtocol\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newFlashloanPremiumTotal\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFlashloanPremiumTotal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ConfiguratorInputTypes.UpdateDebtTokenInput\",\"name\":\"input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateStableDebtToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ConfiguratorInputTypes.UpdateDebtTokenInput\",\"name\":\"input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"incentivesController\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementation\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateVariableDebtToken\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct PoolConfigurator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PoolConfigurator<M> {
        fn clone(&self) -> Self {
            PoolConfigurator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PoolConfigurator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PoolConfigurator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PoolConfigurator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PoolConfigurator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), POOLCONFIGURATOR_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `CONFIGURATOR_REVISION` (0x7af635a6) function"]
        pub fn configurator_revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 246, 53, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureReserveAsCollateral` (0x7c4e560b) function"]
        pub fn configure_reserve_as_collateral(
            &self,
            asset: ethers::core::types::Address,
            ltv: ethers::core::types::U256,
            liquidation_threshold: ethers::core::types::U256,
            liquidation_bonus: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [124, 78, 86, 11],
                    (asset, ltv, liquidation_threshold, liquidation_bonus),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dropReserve` (0x63c9b860) function"]
        pub fn drop_reserve(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 201, 184, 96], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initReserves` (0x02fb45e6) function"]
        pub fn init_reserves(
            &self,
            input: ::std::vec::Vec<InitReserveInput>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 251, 69, 230], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAssetEModeCategory` (0xd4fe3f99) function"]
        pub fn set_asset_e_mode_category(
            &self,
            asset: ethers::core::types::Address,
            new_category_id: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 254, 63, 153], (asset, new_category_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowCap` (0xd14a0983) function"]
        pub fn set_borrow_cap(
            &self,
            asset: ethers::core::types::Address,
            new_borrow_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 74, 9, 131], (asset, new_borrow_cap))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setBorrowableInIsolation` (0x38ae0cc3) function"]
        pub fn set_borrowable_in_isolation(
            &self,
            asset: ethers::core::types::Address,
            borrowable: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 174, 12, 195], (asset, borrowable))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDebtCeiling` (0xaeb4fcc1) function"]
        pub fn set_debt_ceiling(
            &self,
            asset: ethers::core::types::Address,
            new_debt_ceiling: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 180, 252, 193], (asset, new_debt_ceiling))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEModeCategory` (0xc19d61e4) function"]
        pub fn set_e_mode_category(
            &self,
            category_id: u8,
            ltv: u16,
            liquidation_threshold: u16,
            liquidation_bonus: u16,
            oracle: ethers::core::types::Address,
            label: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 157, 97, 228],
                    (
                        category_id,
                        ltv,
                        liquidation_threshold,
                        liquidation_bonus,
                        oracle,
                        label,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationProtocolFee` (0x26d2cec2) function"]
        pub fn set_liquidation_protocol_fee(
            &self,
            asset: ethers::core::types::Address,
            new_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 210, 206, 194], (asset, new_fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolPause` (0x7641f3d9) function"]
        pub fn set_pool_pause(
            &self,
            paused: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 65, 243, 217], paused)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveActive` (0xb736aaeb) function"]
        pub fn set_reserve_active(
            &self,
            asset: ethers::core::types::Address,
            active: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 54, 170, 235], (asset, active))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveBorrowing` (0x682cf264) function"]
        pub fn set_reserve_borrowing(
            &self,
            asset: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 44, 242, 100], (asset, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveFactor` (0x4b4e6753) function"]
        pub fn set_reserve_factor(
            &self,
            asset: ethers::core::types::Address,
            new_reserve_factor: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 78, 103, 83], (asset, new_reserve_factor))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveFreeze` (0x96e957c4) function"]
        pub fn set_reserve_freeze(
            &self,
            asset: ethers::core::types::Address,
            freeze: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 233, 87, 196], (asset, freeze))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function"]
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            asset: ethers::core::types::Address,
            new_rate_strategy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (asset, new_rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReservePause` (0x48d9fba9) function"]
        pub fn set_reserve_pause(
            &self,
            asset: ethers::core::types::Address,
            paused: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 217, 251, 169], (asset, paused))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveStableRateBorrowing` (0x8a751a60) function"]
        pub fn set_reserve_stable_rate_borrowing(
            &self,
            asset: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 117, 26, 96], (asset, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSiloedBorrowing` (0xa7fa83b7) function"]
        pub fn set_siloed_borrowing(
            &self,
            asset: ethers::core::types::Address,
            new_siloed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 250, 131, 183], (asset, new_siloed))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSupplyCap` (0x571f03e5) function"]
        pub fn set_supply_cap(
            &self,
            asset: ethers::core::types::Address,
            new_supply_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 31, 3, 229], (asset, new_supply_cap))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUnbackedMintCap` (0x145f5892) function"]
        pub fn set_unbacked_mint_cap(
            &self,
            asset: ethers::core::types::Address,
            new_unbacked_mint_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 95, 88, 146], (asset, new_unbacked_mint_cap))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateAToken` (0xbb01c37c) function"]
        pub fn update_a_token(
            &self,
            input: UpdateATokenInput,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 1, 195, 124], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBridgeProtocolFee` (0x3036b439) function"]
        pub fn update_bridge_protocol_fee(
            &self,
            new_bridge_protocol_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 54, 180, 57], new_bridge_protocol_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFlashloanPremiumToProtocol` (0x1df970bd) function"]
        pub fn update_flashloan_premium_to_protocol(
            &self,
            new_flashloan_premium_to_protocol: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 249, 112, 189], new_flashloan_premium_to_protocol)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFlashloanPremiumTotal` (0x8a493676) function"]
        pub fn update_flashloan_premium_total(
            &self,
            new_flashloan_premium_total: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 73, 54, 118], new_flashloan_premium_total)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateStableDebtToken` (0x7626cde3) function"]
        pub fn update_stable_debt_token(
            &self,
            input: UpdateDebtTokenInput,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 38, 205, 227], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateVariableDebtToken` (0xad4e6432) function"]
        pub fn update_variable_debt_token(
            &self,
            input: UpdateDebtTokenInput,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 78, 100, 50], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ATokenUpgraded` event"]
        pub fn a_token_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AtokenUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowCapChanged` event"]
        pub fn borrow_cap_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowCapChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowableInIsolationChanged` event"]
        pub fn borrowable_in_isolation_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowableInIsolationChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BridgeProtocolFeeUpdated` event"]
        pub fn bridge_protocol_fee_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BridgeProtocolFeeUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CollateralConfigurationChanged` event"]
        pub fn collateral_configuration_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CollateralConfigurationChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DebtCeilingChanged` event"]
        pub fn debt_ceiling_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DebtCeilingChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EModeAssetCategoryChanged` event"]
        pub fn e_mode_asset_category_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmodeAssetCategoryChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EModeCategoryAdded` event"]
        pub fn e_mode_category_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmodeCategoryAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FlashloanPremiumToProtocolUpdated` event"]
        pub fn flashloan_premium_to_protocol_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FlashloanPremiumToProtocolUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FlashloanPremiumTotalUpdated` event"]
        pub fn flashloan_premium_total_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FlashloanPremiumTotalUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidationProtocolFeeChanged` event"]
        pub fn liquidation_protocol_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidationProtocolFeeChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveActive` event"]
        pub fn reserve_active_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveActiveFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveBorrowing` event"]
        pub fn reserve_borrowing_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveBorrowingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveDropped` event"]
        pub fn reserve_dropped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveDroppedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveFactorChanged` event"]
        pub fn reserve_factor_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveFactorChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveFrozen` event"]
        pub fn reserve_frozen_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveFrozenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveInitialized` event"]
        pub fn reserve_initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveInitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveInterestRateStrategyChanged` event"]
        pub fn reserve_interest_rate_strategy_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveInterestRateStrategyChangedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReservePaused` event"]
        pub fn reserve_paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReservePausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveStableRateBorrowing` event"]
        pub fn reserve_stable_rate_borrowing_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveStableRateBorrowingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SiloedBorrowingChanged` event"]
        pub fn siloed_borrowing_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SiloedBorrowingChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StableDebtTokenUpgraded` event"]
        pub fn stable_debt_token_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StableDebtTokenUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SupplyCapChanged` event"]
        pub fn supply_cap_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SupplyCapChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UnbackedMintCapChanged` event"]
        pub fn unbacked_mint_cap_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnbackedMintCapChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VariableDebtTokenUpgraded` event"]
        pub fn variable_debt_token_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VariableDebtTokenUpgradedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PoolConfiguratorEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PoolConfigurator<M> {
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
        name = "ATokenUpgraded",
        abi = "ATokenUpgraded(address,address,address)"
    )]
    pub struct AtokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
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
        name = "BorrowCapChanged",
        abi = "BorrowCapChanged(address,uint256,uint256)"
    )]
    pub struct BorrowCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_borrow_cap: ethers::core::types::U256,
        pub new_borrow_cap: ethers::core::types::U256,
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
        name = "BorrowableInIsolationChanged",
        abi = "BorrowableInIsolationChanged(address,bool)"
    )]
    pub struct BorrowableInIsolationChangedFilter {
        pub asset: ethers::core::types::Address,
        pub borrowable: bool,
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
        name = "BridgeProtocolFeeUpdated",
        abi = "BridgeProtocolFeeUpdated(uint256,uint256)"
    )]
    pub struct BridgeProtocolFeeUpdatedFilter {
        pub old_bridge_protocol_fee: ethers::core::types::U256,
        pub new_bridge_protocol_fee: ethers::core::types::U256,
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
        name = "CollateralConfigurationChanged",
        abi = "CollateralConfigurationChanged(address,uint256,uint256,uint256)"
    )]
    pub struct CollateralConfigurationChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub ltv: ethers::core::types::U256,
        pub liquidation_threshold: ethers::core::types::U256,
        pub liquidation_bonus: ethers::core::types::U256,
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
        name = "DebtCeilingChanged",
        abi = "DebtCeilingChanged(address,uint256,uint256)"
    )]
    pub struct DebtCeilingChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_debt_ceiling: ethers::core::types::U256,
        pub new_debt_ceiling: ethers::core::types::U256,
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
        name = "EModeAssetCategoryChanged",
        abi = "EModeAssetCategoryChanged(address,uint8,uint8)"
    )]
    pub struct EmodeAssetCategoryChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_category_id: u8,
        pub new_category_id: u8,
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
        name = "EModeCategoryAdded",
        abi = "EModeCategoryAdded(uint8,uint256,uint256,uint256,address,string)"
    )]
    pub struct EmodeCategoryAddedFilter {
        #[ethevent(indexed)]
        pub category_id: u8,
        pub ltv: ethers::core::types::U256,
        pub liquidation_threshold: ethers::core::types::U256,
        pub liquidation_bonus: ethers::core::types::U256,
        pub oracle: ethers::core::types::Address,
        pub label: String,
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
        name = "FlashloanPremiumToProtocolUpdated",
        abi = "FlashloanPremiumToProtocolUpdated(uint128,uint128)"
    )]
    pub struct FlashloanPremiumToProtocolUpdatedFilter {
        pub old_flashloan_premium_to_protocol: u128,
        pub new_flashloan_premium_to_protocol: u128,
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
        name = "FlashloanPremiumTotalUpdated",
        abi = "FlashloanPremiumTotalUpdated(uint128,uint128)"
    )]
    pub struct FlashloanPremiumTotalUpdatedFilter {
        pub old_flashloan_premium_total: u128,
        pub new_flashloan_premium_total: u128,
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
        name = "LiquidationProtocolFeeChanged",
        abi = "LiquidationProtocolFeeChanged(address,uint256,uint256)"
    )]
    pub struct LiquidationProtocolFeeChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_fee: ethers::core::types::U256,
        pub new_fee: ethers::core::types::U256,
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
    #[ethevent(name = "ReserveActive", abi = "ReserveActive(address,bool)")]
    pub struct ReserveActiveFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub active: bool,
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
    #[ethevent(name = "ReserveBorrowing", abi = "ReserveBorrowing(address,bool)")]
    pub struct ReserveBorrowingFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "ReserveDropped", abi = "ReserveDropped(address)")]
    pub struct ReserveDroppedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
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
        name = "ReserveFactorChanged",
        abi = "ReserveFactorChanged(address,uint256,uint256)"
    )]
    pub struct ReserveFactorChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_reserve_factor: ethers::core::types::U256,
        pub new_reserve_factor: ethers::core::types::U256,
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
    #[ethevent(name = "ReserveFrozen", abi = "ReserveFrozen(address,bool)")]
    pub struct ReserveFrozenFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub frozen: bool,
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
        name = "ReserveInitialized",
        abi = "ReserveInitialized(address,address,address,address,address)"
    )]
    pub struct ReserveInitializedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub a_token: ethers::core::types::Address,
        pub stable_debt_token: ethers::core::types::Address,
        pub variable_debt_token: ethers::core::types::Address,
        pub interest_rate_strategy_address: ethers::core::types::Address,
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
        name = "ReserveInterestRateStrategyChanged",
        abi = "ReserveInterestRateStrategyChanged(address,address,address)"
    )]
    pub struct ReserveInterestRateStrategyChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_strategy: ethers::core::types::Address,
        pub new_strategy: ethers::core::types::Address,
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
    #[ethevent(name = "ReservePaused", abi = "ReservePaused(address,bool)")]
    pub struct ReservePausedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub paused: bool,
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
        name = "ReserveStableRateBorrowing",
        abi = "ReserveStableRateBorrowing(address,bool)"
    )]
    pub struct ReserveStableRateBorrowingFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub enabled: bool,
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
        name = "SiloedBorrowingChanged",
        abi = "SiloedBorrowingChanged(address,bool,bool)"
    )]
    pub struct SiloedBorrowingChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_state: bool,
        pub new_state: bool,
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
        name = "StableDebtTokenUpgraded",
        abi = "StableDebtTokenUpgraded(address,address,address)"
    )]
    pub struct StableDebtTokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
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
        name = "SupplyCapChanged",
        abi = "SupplyCapChanged(address,uint256,uint256)"
    )]
    pub struct SupplyCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_supply_cap: ethers::core::types::U256,
        pub new_supply_cap: ethers::core::types::U256,
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
        name = "UnbackedMintCapChanged",
        abi = "UnbackedMintCapChanged(address,uint256,uint256)"
    )]
    pub struct UnbackedMintCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub old_unbacked_mint_cap: ethers::core::types::U256,
        pub new_unbacked_mint_cap: ethers::core::types::U256,
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
        name = "VariableDebtTokenUpgraded",
        abi = "VariableDebtTokenUpgraded(address,address,address)"
    )]
    pub struct VariableDebtTokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolConfiguratorEvents {
        AtokenUpgradedFilter(AtokenUpgradedFilter),
        BorrowCapChangedFilter(BorrowCapChangedFilter),
        BorrowableInIsolationChangedFilter(BorrowableInIsolationChangedFilter),
        BridgeProtocolFeeUpdatedFilter(BridgeProtocolFeeUpdatedFilter),
        CollateralConfigurationChangedFilter(CollateralConfigurationChangedFilter),
        DebtCeilingChangedFilter(DebtCeilingChangedFilter),
        EmodeAssetCategoryChangedFilter(EmodeAssetCategoryChangedFilter),
        EmodeCategoryAddedFilter(EmodeCategoryAddedFilter),
        FlashloanPremiumToProtocolUpdatedFilter(FlashloanPremiumToProtocolUpdatedFilter),
        FlashloanPremiumTotalUpdatedFilter(FlashloanPremiumTotalUpdatedFilter),
        LiquidationProtocolFeeChangedFilter(LiquidationProtocolFeeChangedFilter),
        ReserveActiveFilter(ReserveActiveFilter),
        ReserveBorrowingFilter(ReserveBorrowingFilter),
        ReserveDroppedFilter(ReserveDroppedFilter),
        ReserveFactorChangedFilter(ReserveFactorChangedFilter),
        ReserveFrozenFilter(ReserveFrozenFilter),
        ReserveInitializedFilter(ReserveInitializedFilter),
        ReserveInterestRateStrategyChangedFilter(ReserveInterestRateStrategyChangedFilter),
        ReservePausedFilter(ReservePausedFilter),
        ReserveStableRateBorrowingFilter(ReserveStableRateBorrowingFilter),
        SiloedBorrowingChangedFilter(SiloedBorrowingChangedFilter),
        StableDebtTokenUpgradedFilter(StableDebtTokenUpgradedFilter),
        SupplyCapChangedFilter(SupplyCapChangedFilter),
        UnbackedMintCapChangedFilter(UnbackedMintCapChangedFilter),
        VariableDebtTokenUpgradedFilter(VariableDebtTokenUpgradedFilter),
    }
    impl ethers::contract::EthLogDecode for PoolConfiguratorEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AtokenUpgradedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::AtokenUpgradedFilter(decoded));
            }
            if let Ok(decoded) = BorrowCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::BorrowCapChangedFilter(decoded));
            }
            if let Ok(decoded) = BorrowableInIsolationChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::BorrowableInIsolationChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = BridgeProtocolFeeUpdatedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::BridgeProtocolFeeUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CollateralConfigurationChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::CollateralConfigurationChangedFilter(decoded));
            }
            if let Ok(decoded) = DebtCeilingChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::DebtCeilingChangedFilter(decoded));
            }
            if let Ok(decoded) = EmodeAssetCategoryChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::EmodeAssetCategoryChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EmodeCategoryAddedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::EmodeCategoryAddedFilter(decoded));
            }
            if let Ok(decoded) = FlashloanPremiumToProtocolUpdatedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::FlashloanPremiumToProtocolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FlashloanPremiumTotalUpdatedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::FlashloanPremiumTotalUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LiquidationProtocolFeeChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::LiquidationProtocolFeeChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReserveActiveFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveActiveFilter(decoded));
            }
            if let Ok(decoded) = ReserveBorrowingFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveBorrowingFilter(decoded));
            }
            if let Ok(decoded) = ReserveDroppedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveDroppedFilter(decoded));
            }
            if let Ok(decoded) = ReserveFactorChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveFactorChangedFilter(decoded));
            }
            if let Ok(decoded) = ReserveFrozenFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveFrozenFilter(decoded));
            }
            if let Ok(decoded) = ReserveInitializedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveInitializedFilter(decoded));
            }
            if let Ok(decoded) = ReserveInterestRateStrategyChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::ReserveInterestRateStrategyChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ReservePausedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReservePausedFilter(decoded));
            }
            if let Ok(decoded) = ReserveStableRateBorrowingFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveStableRateBorrowingFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SiloedBorrowingChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::SiloedBorrowingChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StableDebtTokenUpgradedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::StableDebtTokenUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SupplyCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::SupplyCapChangedFilter(decoded));
            }
            if let Ok(decoded) = UnbackedMintCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::UnbackedMintCapChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = VariableDebtTokenUpgradedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::VariableDebtTokenUpgradedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PoolConfiguratorEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolConfiguratorEvents::AtokenUpgradedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::BorrowCapChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::BorrowableInIsolationChangedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::BridgeProtocolFeeUpdatedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::CollateralConfigurationChangedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::DebtCeilingChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::EmodeAssetCategoryChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::EmodeCategoryAddedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::FlashloanPremiumToProtocolUpdatedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::FlashloanPremiumTotalUpdatedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::LiquidationProtocolFeeChangedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::ReserveActiveFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveBorrowingFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveDroppedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveFactorChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveFrozenFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveInitializedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveInterestRateStrategyChangedFilter(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorEvents::ReservePausedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::ReserveStableRateBorrowingFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::SiloedBorrowingChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::StableDebtTokenUpgradedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::SupplyCapChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::UnbackedMintCapChangedFilter(element) => element.fmt(f),
                PoolConfiguratorEvents::VariableDebtTokenUpgradedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `CONFIGURATOR_REVISION` function with signature `CONFIGURATOR_REVISION()` and selector `[122, 246, 53, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CONFIGURATOR_REVISION", abi = "CONFIGURATOR_REVISION()")]
    pub struct ConfiguratorRevisionCall;
    #[doc = "Container type for all input parameters for the `configureReserveAsCollateral` function with signature `configureReserveAsCollateral(address,uint256,uint256,uint256)` and selector `[124, 78, 86, 11]`"]
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
        name = "configureReserveAsCollateral",
        abi = "configureReserveAsCollateral(address,uint256,uint256,uint256)"
    )]
    pub struct ConfigureReserveAsCollateralCall {
        pub asset: ethers::core::types::Address,
        pub ltv: ethers::core::types::U256,
        pub liquidation_threshold: ethers::core::types::U256,
        pub liquidation_bonus: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `dropReserve` function with signature `dropReserve(address)` and selector `[99, 201, 184, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dropReserve", abi = "dropReserve(address)")]
    pub struct DropReserveCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initReserves` function with signature `initReserves((address,address,address,uint8,address,address,address,address,string,string,string,string,string,string,bytes)[])` and selector `[2, 251, 69, 230]`"]
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
        name = "initReserves",
        abi = "initReserves((address,address,address,uint8,address,address,address,address,string,string,string,string,string,string,bytes)[])"
    )]
    pub struct InitReservesCall {
        pub input: ::std::vec::Vec<InitReserveInput>,
    }
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `[196, 214, 109, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAssetEModeCategory` function with signature `setAssetEModeCategory(address,uint8)` and selector `[212, 254, 63, 153]`"]
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
        name = "setAssetEModeCategory",
        abi = "setAssetEModeCategory(address,uint8)"
    )]
    pub struct SetAssetEModeCategoryCall {
        pub asset: ethers::core::types::Address,
        pub new_category_id: u8,
    }
    #[doc = "Container type for all input parameters for the `setBorrowCap` function with signature `setBorrowCap(address,uint256)` and selector `[209, 74, 9, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBorrowCap", abi = "setBorrowCap(address,uint256)")]
    pub struct SetBorrowCapCall {
        pub asset: ethers::core::types::Address,
        pub new_borrow_cap: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setBorrowableInIsolation` function with signature `setBorrowableInIsolation(address,bool)` and selector `[56, 174, 12, 195]`"]
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
        name = "setBorrowableInIsolation",
        abi = "setBorrowableInIsolation(address,bool)"
    )]
    pub struct SetBorrowableInIsolationCall {
        pub asset: ethers::core::types::Address,
        pub borrowable: bool,
    }
    #[doc = "Container type for all input parameters for the `setDebtCeiling` function with signature `setDebtCeiling(address,uint256)` and selector `[174, 180, 252, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDebtCeiling", abi = "setDebtCeiling(address,uint256)")]
    pub struct SetDebtCeilingCall {
        pub asset: ethers::core::types::Address,
        pub new_debt_ceiling: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setEModeCategory` function with signature `setEModeCategory(uint8,uint16,uint16,uint16,address,string)` and selector `[193, 157, 97, 228]`"]
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
        name = "setEModeCategory",
        abi = "setEModeCategory(uint8,uint16,uint16,uint16,address,string)"
    )]
    pub struct SetEModeCategoryCall {
        pub category_id: u8,
        pub ltv: u16,
        pub liquidation_threshold: u16,
        pub liquidation_bonus: u16,
        pub oracle: ethers::core::types::Address,
        pub label: String,
    }
    #[doc = "Container type for all input parameters for the `setLiquidationProtocolFee` function with signature `setLiquidationProtocolFee(address,uint256)` and selector `[38, 210, 206, 194]`"]
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
        name = "setLiquidationProtocolFee",
        abi = "setLiquidationProtocolFee(address,uint256)"
    )]
    pub struct SetLiquidationProtocolFeeCall {
        pub asset: ethers::core::types::Address,
        pub new_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPoolPause` function with signature `setPoolPause(bool)` and selector `[118, 65, 243, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolPause", abi = "setPoolPause(bool)")]
    pub struct SetPoolPauseCall {
        pub paused: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveActive` function with signature `setReserveActive(address,bool)` and selector `[183, 54, 170, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setReserveActive", abi = "setReserveActive(address,bool)")]
    pub struct SetReserveActiveCall {
        pub asset: ethers::core::types::Address,
        pub active: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveBorrowing` function with signature `setReserveBorrowing(address,bool)` and selector `[104, 44, 242, 100]`"]
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
        name = "setReserveBorrowing",
        abi = "setReserveBorrowing(address,bool)"
    )]
    pub struct SetReserveBorrowingCall {
        pub asset: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveFactor` function with signature `setReserveFactor(address,uint256)` and selector `[75, 78, 103, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setReserveFactor", abi = "setReserveFactor(address,uint256)")]
    pub struct SetReserveFactorCall {
        pub asset: ethers::core::types::Address,
        pub new_reserve_factor: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReserveFreeze` function with signature `setReserveFreeze(address,bool)` and selector `[150, 233, 87, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setReserveFreeze", abi = "setReserveFreeze(address,bool)")]
    pub struct SetReserveFreezeCall {
        pub asset: ethers::core::types::Address,
        pub freeze: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveInterestRateStrategyAddress` function with signature `setReserveInterestRateStrategyAddress(address,address)` and selector `[29, 33, 24, 249]`"]
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
        name = "setReserveInterestRateStrategyAddress",
        abi = "setReserveInterestRateStrategyAddress(address,address)"
    )]
    pub struct SetReserveInterestRateStrategyAddressCall {
        pub asset: ethers::core::types::Address,
        pub new_rate_strategy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setReservePause` function with signature `setReservePause(address,bool)` and selector `[72, 217, 251, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setReservePause", abi = "setReservePause(address,bool)")]
    pub struct SetReservePauseCall {
        pub asset: ethers::core::types::Address,
        pub paused: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveStableRateBorrowing` function with signature `setReserveStableRateBorrowing(address,bool)` and selector `[138, 117, 26, 96]`"]
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
        name = "setReserveStableRateBorrowing",
        abi = "setReserveStableRateBorrowing(address,bool)"
    )]
    pub struct SetReserveStableRateBorrowingCall {
        pub asset: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setSiloedBorrowing` function with signature `setSiloedBorrowing(address,bool)` and selector `[167, 250, 131, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSiloedBorrowing", abi = "setSiloedBorrowing(address,bool)")]
    pub struct SetSiloedBorrowingCall {
        pub asset: ethers::core::types::Address,
        pub new_siloed: bool,
    }
    #[doc = "Container type for all input parameters for the `setSupplyCap` function with signature `setSupplyCap(address,uint256)` and selector `[87, 31, 3, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setSupplyCap", abi = "setSupplyCap(address,uint256)")]
    pub struct SetSupplyCapCall {
        pub asset: ethers::core::types::Address,
        pub new_supply_cap: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setUnbackedMintCap` function with signature `setUnbackedMintCap(address,uint256)` and selector `[20, 95, 88, 146]`"]
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
        name = "setUnbackedMintCap",
        abi = "setUnbackedMintCap(address,uint256)"
    )]
    pub struct SetUnbackedMintCapCall {
        pub asset: ethers::core::types::Address,
        pub new_unbacked_mint_cap: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateAToken` function with signature `updateAToken((address,address,address,string,string,address,bytes))` and selector `[187, 1, 195, 124]`"]
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
        name = "updateAToken",
        abi = "updateAToken((address,address,address,string,string,address,bytes))"
    )]
    pub struct UpdateATokenCall {
        pub input: UpdateATokenInput,
    }
    #[doc = "Container type for all input parameters for the `updateBridgeProtocolFee` function with signature `updateBridgeProtocolFee(uint256)` and selector `[48, 54, 180, 57]`"]
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
        name = "updateBridgeProtocolFee",
        abi = "updateBridgeProtocolFee(uint256)"
    )]
    pub struct UpdateBridgeProtocolFeeCall {
        pub new_bridge_protocol_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateFlashloanPremiumToProtocol` function with signature `updateFlashloanPremiumToProtocol(uint128)` and selector `[29, 249, 112, 189]`"]
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
        name = "updateFlashloanPremiumToProtocol",
        abi = "updateFlashloanPremiumToProtocol(uint128)"
    )]
    pub struct UpdateFlashloanPremiumToProtocolCall {
        pub new_flashloan_premium_to_protocol: u128,
    }
    #[doc = "Container type for all input parameters for the `updateFlashloanPremiumTotal` function with signature `updateFlashloanPremiumTotal(uint128)` and selector `[138, 73, 54, 118]`"]
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
        name = "updateFlashloanPremiumTotal",
        abi = "updateFlashloanPremiumTotal(uint128)"
    )]
    pub struct UpdateFlashloanPremiumTotalCall {
        pub new_flashloan_premium_total: u128,
    }
    #[doc = "Container type for all input parameters for the `updateStableDebtToken` function with signature `updateStableDebtToken((address,address,string,string,address,bytes))` and selector `[118, 38, 205, 227]`"]
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
        name = "updateStableDebtToken",
        abi = "updateStableDebtToken((address,address,string,string,address,bytes))"
    )]
    pub struct UpdateStableDebtTokenCall {
        pub input: UpdateDebtTokenInput,
    }
    #[doc = "Container type for all input parameters for the `updateVariableDebtToken` function with signature `updateVariableDebtToken((address,address,string,string,address,bytes))` and selector `[173, 78, 100, 50]`"]
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
        name = "updateVariableDebtToken",
        abi = "updateVariableDebtToken((address,address,string,string,address,bytes))"
    )]
    pub struct UpdateVariableDebtTokenCall {
        pub input: UpdateDebtTokenInput,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PoolConfiguratorCalls {
        ConfiguratorRevision(ConfiguratorRevisionCall),
        ConfigureReserveAsCollateral(ConfigureReserveAsCollateralCall),
        DropReserve(DropReserveCall),
        InitReserves(InitReservesCall),
        Initialize(InitializeCall),
        SetAssetEModeCategory(SetAssetEModeCategoryCall),
        SetBorrowCap(SetBorrowCapCall),
        SetBorrowableInIsolation(SetBorrowableInIsolationCall),
        SetDebtCeiling(SetDebtCeilingCall),
        SetEModeCategory(SetEModeCategoryCall),
        SetLiquidationProtocolFee(SetLiquidationProtocolFeeCall),
        SetPoolPause(SetPoolPauseCall),
        SetReserveActive(SetReserveActiveCall),
        SetReserveBorrowing(SetReserveBorrowingCall),
        SetReserveFactor(SetReserveFactorCall),
        SetReserveFreeze(SetReserveFreezeCall),
        SetReserveInterestRateStrategyAddress(SetReserveInterestRateStrategyAddressCall),
        SetReservePause(SetReservePauseCall),
        SetReserveStableRateBorrowing(SetReserveStableRateBorrowingCall),
        SetSiloedBorrowing(SetSiloedBorrowingCall),
        SetSupplyCap(SetSupplyCapCall),
        SetUnbackedMintCap(SetUnbackedMintCapCall),
        UpdateAToken(UpdateATokenCall),
        UpdateBridgeProtocolFee(UpdateBridgeProtocolFeeCall),
        UpdateFlashloanPremiumToProtocol(UpdateFlashloanPremiumToProtocolCall),
        UpdateFlashloanPremiumTotal(UpdateFlashloanPremiumTotalCall),
        UpdateStableDebtToken(UpdateStableDebtTokenCall),
        UpdateVariableDebtToken(UpdateVariableDebtTokenCall),
    }
    impl ethers::core::abi::AbiDecode for PoolConfiguratorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ConfiguratorRevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::ConfiguratorRevision(decoded));
            }
            if let Ok(decoded) =
                <ConfigureReserveAsCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::ConfigureReserveAsCollateral(decoded));
            }
            if let Ok(decoded) =
                <DropReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::DropReserve(decoded));
            }
            if let Ok(decoded) =
                <InitReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::InitReserves(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SetAssetEModeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetAssetEModeCategory(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetBorrowCap(decoded));
            }
            if let Ok(decoded) =
                <SetBorrowableInIsolationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::SetBorrowableInIsolation(decoded));
            }
            if let Ok(decoded) =
                <SetDebtCeilingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetDebtCeiling(decoded));
            }
            if let Ok(decoded) =
                <SetEModeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetEModeCategory(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::SetLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <SetPoolPauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetPoolPause(decoded));
            }
            if let Ok(decoded) =
                <SetReserveActiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetReserveActive(decoded));
            }
            if let Ok(decoded) =
                <SetReserveBorrowingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetReserveBorrowing(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <SetReserveFreezeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetReserveFreeze(decoded));
            }
            if let Ok(decoded) =
                <SetReserveInterestRateStrategyAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::SetReserveInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded) =
                <SetReservePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetReservePause(decoded));
            }
            if let Ok(decoded) =
                <SetReserveStableRateBorrowingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::SetReserveStableRateBorrowing(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetSiloedBorrowingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetSiloedBorrowing(decoded));
            }
            if let Ok(decoded) =
                <SetSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <SetUnbackedMintCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::SetUnbackedMintCap(decoded));
            }
            if let Ok(decoded) =
                <UpdateATokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::UpdateAToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateBridgeProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::UpdateBridgeProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <UpdateFlashloanPremiumToProtocolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::UpdateFlashloanPremiumToProtocol(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdateFlashloanPremiumTotalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PoolConfiguratorCalls::UpdateFlashloanPremiumTotal(decoded));
            }
            if let Ok(decoded) =
                <UpdateStableDebtTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::UpdateStableDebtToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateVariableDebtTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PoolConfiguratorCalls::UpdateVariableDebtToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PoolConfiguratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PoolConfiguratorCalls::ConfiguratorRevision(element) => element.encode(),
                PoolConfiguratorCalls::ConfigureReserveAsCollateral(element) => element.encode(),
                PoolConfiguratorCalls::DropReserve(element) => element.encode(),
                PoolConfiguratorCalls::InitReserves(element) => element.encode(),
                PoolConfiguratorCalls::Initialize(element) => element.encode(),
                PoolConfiguratorCalls::SetAssetEModeCategory(element) => element.encode(),
                PoolConfiguratorCalls::SetBorrowCap(element) => element.encode(),
                PoolConfiguratorCalls::SetBorrowableInIsolation(element) => element.encode(),
                PoolConfiguratorCalls::SetDebtCeiling(element) => element.encode(),
                PoolConfiguratorCalls::SetEModeCategory(element) => element.encode(),
                PoolConfiguratorCalls::SetLiquidationProtocolFee(element) => element.encode(),
                PoolConfiguratorCalls::SetPoolPause(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveActive(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveBorrowing(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveFactor(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveFreeze(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveInterestRateStrategyAddress(element) => {
                    element.encode()
                }
                PoolConfiguratorCalls::SetReservePause(element) => element.encode(),
                PoolConfiguratorCalls::SetReserveStableRateBorrowing(element) => element.encode(),
                PoolConfiguratorCalls::SetSiloedBorrowing(element) => element.encode(),
                PoolConfiguratorCalls::SetSupplyCap(element) => element.encode(),
                PoolConfiguratorCalls::SetUnbackedMintCap(element) => element.encode(),
                PoolConfiguratorCalls::UpdateAToken(element) => element.encode(),
                PoolConfiguratorCalls::UpdateBridgeProtocolFee(element) => element.encode(),
                PoolConfiguratorCalls::UpdateFlashloanPremiumToProtocol(element) => {
                    element.encode()
                }
                PoolConfiguratorCalls::UpdateFlashloanPremiumTotal(element) => element.encode(),
                PoolConfiguratorCalls::UpdateStableDebtToken(element) => element.encode(),
                PoolConfiguratorCalls::UpdateVariableDebtToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PoolConfiguratorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PoolConfiguratorCalls::ConfiguratorRevision(element) => element.fmt(f),
                PoolConfiguratorCalls::ConfigureReserveAsCollateral(element) => element.fmt(f),
                PoolConfiguratorCalls::DropReserve(element) => element.fmt(f),
                PoolConfiguratorCalls::InitReserves(element) => element.fmt(f),
                PoolConfiguratorCalls::Initialize(element) => element.fmt(f),
                PoolConfiguratorCalls::SetAssetEModeCategory(element) => element.fmt(f),
                PoolConfiguratorCalls::SetBorrowCap(element) => element.fmt(f),
                PoolConfiguratorCalls::SetBorrowableInIsolation(element) => element.fmt(f),
                PoolConfiguratorCalls::SetDebtCeiling(element) => element.fmt(f),
                PoolConfiguratorCalls::SetEModeCategory(element) => element.fmt(f),
                PoolConfiguratorCalls::SetLiquidationProtocolFee(element) => element.fmt(f),
                PoolConfiguratorCalls::SetPoolPause(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveActive(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveBorrowing(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveFactor(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveFreeze(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveInterestRateStrategyAddress(element) => {
                    element.fmt(f)
                }
                PoolConfiguratorCalls::SetReservePause(element) => element.fmt(f),
                PoolConfiguratorCalls::SetReserveStableRateBorrowing(element) => element.fmt(f),
                PoolConfiguratorCalls::SetSiloedBorrowing(element) => element.fmt(f),
                PoolConfiguratorCalls::SetSupplyCap(element) => element.fmt(f),
                PoolConfiguratorCalls::SetUnbackedMintCap(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateAToken(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateBridgeProtocolFee(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateFlashloanPremiumToProtocol(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateFlashloanPremiumTotal(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateStableDebtToken(element) => element.fmt(f),
                PoolConfiguratorCalls::UpdateVariableDebtToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ConfiguratorRevisionCall> for PoolConfiguratorCalls {
        fn from(var: ConfiguratorRevisionCall) -> Self {
            PoolConfiguratorCalls::ConfiguratorRevision(var)
        }
    }
    impl ::std::convert::From<ConfigureReserveAsCollateralCall> for PoolConfiguratorCalls {
        fn from(var: ConfigureReserveAsCollateralCall) -> Self {
            PoolConfiguratorCalls::ConfigureReserveAsCollateral(var)
        }
    }
    impl ::std::convert::From<DropReserveCall> for PoolConfiguratorCalls {
        fn from(var: DropReserveCall) -> Self {
            PoolConfiguratorCalls::DropReserve(var)
        }
    }
    impl ::std::convert::From<InitReservesCall> for PoolConfiguratorCalls {
        fn from(var: InitReservesCall) -> Self {
            PoolConfiguratorCalls::InitReserves(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PoolConfiguratorCalls {
        fn from(var: InitializeCall) -> Self {
            PoolConfiguratorCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<SetAssetEModeCategoryCall> for PoolConfiguratorCalls {
        fn from(var: SetAssetEModeCategoryCall) -> Self {
            PoolConfiguratorCalls::SetAssetEModeCategory(var)
        }
    }
    impl ::std::convert::From<SetBorrowCapCall> for PoolConfiguratorCalls {
        fn from(var: SetBorrowCapCall) -> Self {
            PoolConfiguratorCalls::SetBorrowCap(var)
        }
    }
    impl ::std::convert::From<SetBorrowableInIsolationCall> for PoolConfiguratorCalls {
        fn from(var: SetBorrowableInIsolationCall) -> Self {
            PoolConfiguratorCalls::SetBorrowableInIsolation(var)
        }
    }
    impl ::std::convert::From<SetDebtCeilingCall> for PoolConfiguratorCalls {
        fn from(var: SetDebtCeilingCall) -> Self {
            PoolConfiguratorCalls::SetDebtCeiling(var)
        }
    }
    impl ::std::convert::From<SetEModeCategoryCall> for PoolConfiguratorCalls {
        fn from(var: SetEModeCategoryCall) -> Self {
            PoolConfiguratorCalls::SetEModeCategory(var)
        }
    }
    impl ::std::convert::From<SetLiquidationProtocolFeeCall> for PoolConfiguratorCalls {
        fn from(var: SetLiquidationProtocolFeeCall) -> Self {
            PoolConfiguratorCalls::SetLiquidationProtocolFee(var)
        }
    }
    impl ::std::convert::From<SetPoolPauseCall> for PoolConfiguratorCalls {
        fn from(var: SetPoolPauseCall) -> Self {
            PoolConfiguratorCalls::SetPoolPause(var)
        }
    }
    impl ::std::convert::From<SetReserveActiveCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveActiveCall) -> Self {
            PoolConfiguratorCalls::SetReserveActive(var)
        }
    }
    impl ::std::convert::From<SetReserveBorrowingCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveBorrowingCall) -> Self {
            PoolConfiguratorCalls::SetReserveBorrowing(var)
        }
    }
    impl ::std::convert::From<SetReserveFactorCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveFactorCall) -> Self {
            PoolConfiguratorCalls::SetReserveFactor(var)
        }
    }
    impl ::std::convert::From<SetReserveFreezeCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveFreezeCall) -> Self {
            PoolConfiguratorCalls::SetReserveFreeze(var)
        }
    }
    impl ::std::convert::From<SetReserveInterestRateStrategyAddressCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveInterestRateStrategyAddressCall) -> Self {
            PoolConfiguratorCalls::SetReserveInterestRateStrategyAddress(var)
        }
    }
    impl ::std::convert::From<SetReservePauseCall> for PoolConfiguratorCalls {
        fn from(var: SetReservePauseCall) -> Self {
            PoolConfiguratorCalls::SetReservePause(var)
        }
    }
    impl ::std::convert::From<SetReserveStableRateBorrowingCall> for PoolConfiguratorCalls {
        fn from(var: SetReserveStableRateBorrowingCall) -> Self {
            PoolConfiguratorCalls::SetReserveStableRateBorrowing(var)
        }
    }
    impl ::std::convert::From<SetSiloedBorrowingCall> for PoolConfiguratorCalls {
        fn from(var: SetSiloedBorrowingCall) -> Self {
            PoolConfiguratorCalls::SetSiloedBorrowing(var)
        }
    }
    impl ::std::convert::From<SetSupplyCapCall> for PoolConfiguratorCalls {
        fn from(var: SetSupplyCapCall) -> Self {
            PoolConfiguratorCalls::SetSupplyCap(var)
        }
    }
    impl ::std::convert::From<SetUnbackedMintCapCall> for PoolConfiguratorCalls {
        fn from(var: SetUnbackedMintCapCall) -> Self {
            PoolConfiguratorCalls::SetUnbackedMintCap(var)
        }
    }
    impl ::std::convert::From<UpdateATokenCall> for PoolConfiguratorCalls {
        fn from(var: UpdateATokenCall) -> Self {
            PoolConfiguratorCalls::UpdateAToken(var)
        }
    }
    impl ::std::convert::From<UpdateBridgeProtocolFeeCall> for PoolConfiguratorCalls {
        fn from(var: UpdateBridgeProtocolFeeCall) -> Self {
            PoolConfiguratorCalls::UpdateBridgeProtocolFee(var)
        }
    }
    impl ::std::convert::From<UpdateFlashloanPremiumToProtocolCall> for PoolConfiguratorCalls {
        fn from(var: UpdateFlashloanPremiumToProtocolCall) -> Self {
            PoolConfiguratorCalls::UpdateFlashloanPremiumToProtocol(var)
        }
    }
    impl ::std::convert::From<UpdateFlashloanPremiumTotalCall> for PoolConfiguratorCalls {
        fn from(var: UpdateFlashloanPremiumTotalCall) -> Self {
            PoolConfiguratorCalls::UpdateFlashloanPremiumTotal(var)
        }
    }
    impl ::std::convert::From<UpdateStableDebtTokenCall> for PoolConfiguratorCalls {
        fn from(var: UpdateStableDebtTokenCall) -> Self {
            PoolConfiguratorCalls::UpdateStableDebtToken(var)
        }
    }
    impl ::std::convert::From<UpdateVariableDebtTokenCall> for PoolConfiguratorCalls {
        fn from(var: UpdateVariableDebtTokenCall) -> Self {
            PoolConfiguratorCalls::UpdateVariableDebtToken(var)
        }
    }
    #[doc = "Container type for all return fields from the `CONFIGURATOR_REVISION` function with signature `CONFIGURATOR_REVISION()` and selector `[122, 246, 53, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConfiguratorRevisionReturn(pub ethers::core::types::U256);
}
