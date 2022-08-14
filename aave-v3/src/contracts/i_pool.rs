pub use i_pool::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_pool {
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
    #[doc = "IPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"backer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BackUnbacked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"enum DataTypes.InterestRateMode\",\"name\":\"interestRateMode\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"borrowRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"initiator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"enum DataTypes.InterestRateMode\",\"name\":\"interestRateMode\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"premium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"FlashLoan\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"totalDebt\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IsolationModeTotalDebtUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"debtAsset\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"debtToCover\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidatedCollateralAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"receiveAToken\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidationCall\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MintUnbacked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amountMinted\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MintedToTreasury\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RebalanceStableBorrowRate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"repayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"useATokens\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Repay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"liquidityRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"stableBorrowRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"variableBorrowRate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidityIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"variableBorrowIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReserveDataUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ReserveUsedAsCollateralDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ReserveUsedAsCollateralEnabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Supply\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum DataTypes.InterestRateMode\",\"name\":\"interestRateMode\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SwapBorrowRateMode\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"categoryId\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserEModeSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"reserve\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Withdraw\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BRIDGE_PROTOCOL_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FLASHLOAN_PREMIUM_TOTAL\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FLASHLOAN_PREMIUM_TO_PROTOCOL\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_NUMBER_RESERVES\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_STABLE_RATE_BORROW_SIZE_PERCENT\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"backUnbacked\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"borrow\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"id\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"struct DataTypes.EModeCategory\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"ltv\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationThreshold\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationBonus\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"priceSource\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"label\",\"type\":\"string\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureEModeCategory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dropReserve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balanceFromBefore\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balanceToBefore\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"receiverAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"interestRateModes\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flashLoan\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"receiverAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"flashLoanSimple\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getConfiguration\",\"outputs\":[{\"internalType\":\"struct DataTypes.ReserveConfigurationMap\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"id\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEModeCategoryData\",\"outputs\":[{\"internalType\":\"struct DataTypes.EModeCategory\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"ltv\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationThreshold\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"liquidationBonus\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"priceSource\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"label\",\"type\":\"string\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"id\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveAddressById\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveData\",\"outputs\":[{\"internalType\":\"struct DataTypes.ReserveData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct DataTypes.ReserveConfigurationMap\",\"name\":\"configuration\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint128\",\"name\":\"liquidityIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentLiquidityRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"variableBorrowIndex\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentVariableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"currentStableBorrowRate\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"lastUpdateTimestamp\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"id\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"variableDebtTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedToTreasury\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"unbacked\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"isolationModeTotalDebt\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedIncome\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserveNormalizedVariableDebt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReservesList\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserAccountData\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalCollateralBase\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalDebtBase\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableBorrowsBase\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"currentLiquidationThreshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ltv\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"healthFactor\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserConfiguration\",\"outputs\":[{\"internalType\":\"struct DataTypes.UserConfigurationMap\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserEMode\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"aTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stableDebtAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"variableDebtAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"interestRateStrategyAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initReserve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"collateralAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"debtAsset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"debtToCover\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"receiveAToken\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidationCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintToTreasury\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintUnbacked\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rebalanceStableBorrowRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayWithATokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"permitV\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitS\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"resetIsolationModeTotalDebt\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct DataTypes.ReserveConfigurationMap\",\"name\":\"configuration\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"data\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setConfiguration\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rateStrategyAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReserveInterestRateStrategyAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"categoryId\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUserEMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"useAsCollateral\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUserUseReserveAsCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"supply\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"permitV\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"permitS\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"supplyWithPermit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"interestRateMode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapBorrowRateMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"bridgeProtocolFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateBridgeProtocolFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"flashLoanPremiumTotal\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"flashLoanPremiumToProtocol\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateFlashloanPremiums\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IPool<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IPool<M> {
        fn clone(&self) -> Self {
            IPool(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOL_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BRIDGE_PROTOCOL_FEE` (0x272d9072) function"]
        pub fn bridge_protocol_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([39, 45, 144, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FLASHLOAN_PREMIUM_TOTAL` (0x074b2e43) function"]
        pub fn flashloan_premium_total(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([7, 75, 46, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FLASHLOAN_PREMIUM_TO_PROTOCOL` (0x6a99c036) function"]
        pub fn flashloan_premium_to_protocol(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([106, 153, 192, 54], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_NUMBER_RESERVES` (0xf8119d51) function"]
        pub fn max_number_reserves(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([248, 17, 157, 81], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` (0xe82fec2f) function"]
        pub fn max_stable_rate_borrow_size_percent(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([232, 47, 236, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `backUnbacked` (0xd65dc7a1) function"]
        pub fn back_unbacked(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 93, 199, 161], (asset, amount, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0xa415bcad) function"]
        pub fn borrow(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            referral_code: u16,
            on_behalf_of: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 21, 188, 173],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        referral_code,
                        on_behalf_of,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureEModeCategory` (0xd579ea7d) function"]
        pub fn configure_e_mode_category(
            &self,
            id: u8,
            config: EmodeCategory,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 121, 234, 125], (id, config))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xe8eda9df) function"]
        pub fn deposit(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 237, 169, 223],
                    (asset, amount, on_behalf_of, referral_code),
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
        #[doc = "Calls the contract's `finalizeTransfer` (0xd5ed3933) function"]
        pub fn finalize_transfer(
            &self,
            asset: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            balance_from_before: ethers::core::types::U256,
            balance_to_before: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 237, 57, 51],
                    (
                        asset,
                        from,
                        to,
                        amount,
                        balance_from_before,
                        balance_to_before,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoan` (0xab9c4b5d) function"]
        pub fn flash_loan(
            &self,
            receiver_address: ethers::core::types::Address,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            interest_rate_modes: ::std::vec::Vec<ethers::core::types::U256>,
            on_behalf_of: ethers::core::types::Address,
            params: ethers::core::types::Bytes,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 156, 75, 93],
                    (
                        receiver_address,
                        assets,
                        amounts,
                        interest_rate_modes,
                        on_behalf_of,
                        params,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoanSimple` (0x42b0b77c) function"]
        pub fn flash_loan_simple(
            &self,
            receiver_address: ethers::core::types::Address,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            params: ethers::core::types::Bytes,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 176, 183, 124],
                    (receiver_address, asset, amount, params, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfiguration` (0xc44b11f7) function"]
        pub fn get_configuration(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ReserveConfigurationMap> {
            self.0
                .method_hash([196, 75, 17, 247], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEModeCategoryData` (0x6c6f6ae1) function"]
        pub fn get_e_mode_category_data(
            &self,
            id: u8,
        ) -> ethers::contract::builders::ContractCall<M, EmodeCategory> {
            self.0
                .method_hash([108, 111, 106, 225], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveAddressById` (0x52751797) function"]
        pub fn get_reserve_address_by_id(
            &self,
            id: u16,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([82, 117, 23, 151], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveData` (0x35ea6a75) function"]
        pub fn get_reserve_data(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ReserveData> {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedIncome` (0xd15e0053) function"]
        pub fn get_reserve_normalized_income(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([209, 94, 0, 83], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedVariableDebt` (0x386497fd) function"]
        pub fn get_reserve_normalized_variable_debt(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([56, 100, 151, 253], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReservesList` (0xd1946dbc) function"]
        pub fn get_reserves_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([209, 148, 109, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserAccountData` (0xbf92857c) function"]
        pub fn get_user_account_data(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 146, 133, 124], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserConfiguration` (0x4417a583) function"]
        pub fn get_user_configuration(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, UserConfigurationMap> {
            self.0
                .method_hash([68, 23, 165, 131], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserEMode` (0xeddf1b79) function"]
        pub fn get_user_e_mode(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([237, 223, 27, 121], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initReserve` (0x7a708e92) function"]
        pub fn init_reserve(
            &self,
            asset: ethers::core::types::Address,
            a_token_address: ethers::core::types::Address,
            stable_debt_address: ethers::core::types::Address,
            variable_debt_address: ethers::core::types::Address,
            interest_rate_strategy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 112, 142, 146],
                    (
                        asset,
                        a_token_address,
                        stable_debt_address,
                        variable_debt_address,
                        interest_rate_strategy_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationCall` (0x00a718a9) function"]
        pub fn liquidation_call(
            &self,
            collateral_asset: ethers::core::types::Address,
            debt_asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
            debt_to_cover: ethers::core::types::U256,
            receive_a_token: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 167, 24, 169],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        receive_a_token,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintToTreasury` (0x9cd19996) function"]
        pub fn mint_to_treasury(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 209, 153, 150], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintUnbacked` (0x69a933a5) function"]
        pub fn mint_unbacked(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 169, 51, 165],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalanceStableBorrowRate` (0xcd112382) function"]
        pub fn rebalance_stable_borrow_rate(
            &self,
            asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 17, 35, 130], (asset, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repay` (0x573ade81) function"]
        pub fn repay(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [87, 58, 222, 129],
                    (asset, amount, interest_rate_mode, on_behalf_of),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayWithATokens` (0x2dad97d4) function"]
        pub fn repay_with_a_tokens(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 173, 151, 212], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayWithPermit` (0xee3e210b) function"]
        pub fn repay_with_permit(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [238, 62, 33, 11],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        on_behalf_of,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueTokens` (0xcea9d26f) function"]
        pub fn rescue_tokens(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 169, 210, 111], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `resetIsolationModeTotalDebt` (0xe43e88a1) function"]
        pub fn reset_isolation_mode_total_debt(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 62, 136, 161], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setConfiguration` (0xf51e435b) function"]
        pub fn set_configuration(
            &self,
            asset: ethers::core::types::Address,
            configuration: ReserveConfigurationMap,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 30, 67, 91], (asset, configuration))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function"]
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            asset: ethers::core::types::Address,
            rate_strategy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (asset, rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserEMode` (0x28530a47) function"]
        pub fn set_user_e_mode(
            &self,
            category_id: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 83, 10, 71], category_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserUseReserveAsCollateral` (0x5a3b74b9) function"]
        pub fn set_user_use_reserve_as_collateral(
            &self,
            asset: ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 59, 116, 185], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supply` (0x617ba037) function"]
        pub fn supply(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [97, 123, 160, 55],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supplyWithPermit` (0x02c205f0) function"]
        pub fn supply_with_permit(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
            deadline: ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [2, 194, 5, 240],
                    (
                        asset,
                        amount,
                        on_behalf_of,
                        referral_code,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapBorrowRateMode` (0x94ba89a2) function"]
        pub fn swap_borrow_rate_mode(
            &self,
            asset: ethers::core::types::Address,
            interest_rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 186, 137, 162], (asset, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBridgeProtocolFee` (0x3036b439) function"]
        pub fn update_bridge_protocol_fee(
            &self,
            bridge_protocol_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 54, 180, 57], bridge_protocol_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateFlashloanPremiums` (0xbcb6e522) function"]
        pub fn update_flashloan_premiums(
            &self,
            flash_loan_premium_total: u128,
            flash_loan_premium_to_protocol: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 182, 229, 34],
                    (flash_loan_premium_total, flash_loan_premium_to_protocol),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x69328dec) function"]
        pub fn withdraw(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([105, 50, 141, 236], (asset, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `BackUnbacked` event"]
        pub fn back_unbacked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BackUnbackedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers::contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FlashLoan` event"]
        pub fn flash_loan_filter(&self) -> ethers::contract::builders::Event<M, FlashLoanFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IsolationModeTotalDebtUpdated` event"]
        pub fn isolation_mode_total_debt_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IsolationModeTotalDebtUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidationCall` event"]
        pub fn liquidation_call_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidationCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MintUnbacked` event"]
        pub fn mint_unbacked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MintUnbackedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MintedToTreasury` event"]
        pub fn minted_to_treasury_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MintedToTreasuryFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RebalanceStableBorrowRate` event"]
        pub fn rebalance_stable_borrow_rate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RebalanceStableBorrowRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers::contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveDataUpdated` event"]
        pub fn reserve_data_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveDataUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralDisabled` event"]
        pub fn reserve_used_as_collateral_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveUsedAsCollateralDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralEnabled` event"]
        pub fn reserve_used_as_collateral_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveUsedAsCollateralEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Supply` event"]
        pub fn supply_filter(&self) -> ethers::contract::builders::Event<M, SupplyFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SwapBorrowRateMode` event"]
        pub fn swap_borrow_rate_mode_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SwapBorrowRateModeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UserEModeSet` event"]
        pub fn user_e_mode_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UserEModeSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPoolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPool<M> {
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
        name = "BackUnbacked",
        abi = "BackUnbacked(address,address,uint256,uint256)"
    )]
    pub struct BackUnbackedFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub backer: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub fee: ethers::core::types::U256,
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
        name = "Borrow",
        abi = "Borrow(address,address,address,uint256,uint8,uint256,uint16)"
    )]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: u8,
        pub borrow_rate: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "FlashLoan",
        abi = "FlashLoan(address,address,address,uint256,uint8,uint256,uint16)"
    )]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub initiator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: u8,
        pub premium: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "IsolationModeTotalDebtUpdated",
        abi = "IsolationModeTotalDebtUpdated(address,uint256)"
    )]
    pub struct IsolationModeTotalDebtUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub total_debt: ethers::core::types::U256,
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
        name = "LiquidationCall",
        abi = "LiquidationCall(address,address,address,uint256,uint256,address,bool)"
    )]
    pub struct LiquidationCallFilter {
        #[ethevent(indexed)]
        pub collateral_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub debt_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub debt_to_cover: ethers::core::types::U256,
        pub liquidated_collateral_amount: ethers::core::types::U256,
        pub liquidator: ethers::core::types::Address,
        pub receive_a_token: bool,
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
        name = "MintUnbacked",
        abi = "MintUnbacked(address,address,address,uint256,uint16)"
    )]
    pub struct MintUnbackedFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
    #[ethevent(name = "MintedToTreasury", abi = "MintedToTreasury(address,uint256)")]
    pub struct MintedToTreasuryFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub amount_minted: ethers::core::types::U256,
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
        name = "RebalanceStableBorrowRate",
        abi = "RebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "Repay", abi = "Repay(address,address,address,uint256,bool)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub repayer: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub use_a_tokens: bool,
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
        name = "ReserveDataUpdated",
        abi = "ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ReserveDataUpdatedFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub liquidity_rate: ethers::core::types::U256,
        pub stable_borrow_rate: ethers::core::types::U256,
        pub variable_borrow_rate: ethers::core::types::U256,
        pub liquidity_index: ethers::core::types::U256,
        pub variable_borrow_index: ethers::core::types::U256,
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
        name = "ReserveUsedAsCollateralDisabled",
        abi = "ReserveUsedAsCollateralDisabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralDisabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
        name = "ReserveUsedAsCollateralEnabled",
        abi = "ReserveUsedAsCollateralEnabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralEnabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
        name = "Supply",
        abi = "Supply(address,address,address,uint256,uint16)"
    )]
    pub struct SupplyFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "SwapBorrowRateMode",
        abi = "SwapBorrowRateMode(address,address,uint8)"
    )]
    pub struct SwapBorrowRateModeFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub interest_rate_mode: u8,
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
    #[ethevent(name = "UserEModeSet", abi = "UserEModeSet(address,uint8)")]
    pub struct UserEModeSetFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub category_id: u8,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolEvents {
        BackUnbackedFilter(BackUnbackedFilter),
        BorrowFilter(BorrowFilter),
        FlashLoanFilter(FlashLoanFilter),
        IsolationModeTotalDebtUpdatedFilter(IsolationModeTotalDebtUpdatedFilter),
        LiquidationCallFilter(LiquidationCallFilter),
        MintUnbackedFilter(MintUnbackedFilter),
        MintedToTreasuryFilter(MintedToTreasuryFilter),
        RebalanceStableBorrowRateFilter(RebalanceStableBorrowRateFilter),
        RepayFilter(RepayFilter),
        ReserveDataUpdatedFilter(ReserveDataUpdatedFilter),
        ReserveUsedAsCollateralDisabledFilter(ReserveUsedAsCollateralDisabledFilter),
        ReserveUsedAsCollateralEnabledFilter(ReserveUsedAsCollateralEnabledFilter),
        SupplyFilter(SupplyFilter),
        SwapBorrowRateModeFilter(SwapBorrowRateModeFilter),
        UserEModeSetFilter(UserEModeSetFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BackUnbackedFilter::decode_log(log) {
                return Ok(IPoolEvents::BackUnbackedFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(IPoolEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(IPoolEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = IsolationModeTotalDebtUpdatedFilter::decode_log(log) {
                return Ok(IPoolEvents::IsolationModeTotalDebtUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationCallFilter::decode_log(log) {
                return Ok(IPoolEvents::LiquidationCallFilter(decoded));
            }
            if let Ok(decoded) = MintUnbackedFilter::decode_log(log) {
                return Ok(IPoolEvents::MintUnbackedFilter(decoded));
            }
            if let Ok(decoded) = MintedToTreasuryFilter::decode_log(log) {
                return Ok(IPoolEvents::MintedToTreasuryFilter(decoded));
            }
            if let Ok(decoded) = RebalanceStableBorrowRateFilter::decode_log(log) {
                return Ok(IPoolEvents::RebalanceStableBorrowRateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(IPoolEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = ReserveDataUpdatedFilter::decode_log(log) {
                return Ok(IPoolEvents::ReserveDataUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralDisabledFilter::decode_log(log) {
                return Ok(IPoolEvents::ReserveUsedAsCollateralDisabledFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralEnabledFilter::decode_log(log) {
                return Ok(IPoolEvents::ReserveUsedAsCollateralEnabledFilter(decoded));
            }
            if let Ok(decoded) = SupplyFilter::decode_log(log) {
                return Ok(IPoolEvents::SupplyFilter(decoded));
            }
            if let Ok(decoded) = SwapBorrowRateModeFilter::decode_log(log) {
                return Ok(IPoolEvents::SwapBorrowRateModeFilter(decoded));
            }
            if let Ok(decoded) = UserEModeSetFilter::decode_log(log) {
                return Ok(IPoolEvents::UserEModeSetFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(IPoolEvents::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolEvents::BackUnbackedFilter(element) => element.fmt(f),
                IPoolEvents::BorrowFilter(element) => element.fmt(f),
                IPoolEvents::FlashLoanFilter(element) => element.fmt(f),
                IPoolEvents::IsolationModeTotalDebtUpdatedFilter(element) => element.fmt(f),
                IPoolEvents::LiquidationCallFilter(element) => element.fmt(f),
                IPoolEvents::MintUnbackedFilter(element) => element.fmt(f),
                IPoolEvents::MintedToTreasuryFilter(element) => element.fmt(f),
                IPoolEvents::RebalanceStableBorrowRateFilter(element) => element.fmt(f),
                IPoolEvents::RepayFilter(element) => element.fmt(f),
                IPoolEvents::ReserveDataUpdatedFilter(element) => element.fmt(f),
                IPoolEvents::ReserveUsedAsCollateralDisabledFilter(element) => element.fmt(f),
                IPoolEvents::ReserveUsedAsCollateralEnabledFilter(element) => element.fmt(f),
                IPoolEvents::SupplyFilter(element) => element.fmt(f),
                IPoolEvents::SwapBorrowRateModeFilter(element) => element.fmt(f),
                IPoolEvents::UserEModeSetFilter(element) => element.fmt(f),
                IPoolEvents::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
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
    #[doc = "Container type for all input parameters for the `BRIDGE_PROTOCOL_FEE` function with signature `BRIDGE_PROTOCOL_FEE()` and selector `[39, 45, 144, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BRIDGE_PROTOCOL_FEE", abi = "BRIDGE_PROTOCOL_FEE()")]
    pub struct BridgeProtocolFeeCall;
    #[doc = "Container type for all input parameters for the `FLASHLOAN_PREMIUM_TOTAL` function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `[7, 75, 46, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "FLASHLOAN_PREMIUM_TOTAL", abi = "FLASHLOAN_PREMIUM_TOTAL()")]
    pub struct FlashloanPremiumTotalCall;
    #[doc = "Container type for all input parameters for the `FLASHLOAN_PREMIUM_TO_PROTOCOL` function with signature `FLASHLOAN_PREMIUM_TO_PROTOCOL()` and selector `[106, 153, 192, 54]`"]
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
        name = "FLASHLOAN_PREMIUM_TO_PROTOCOL",
        abi = "FLASHLOAN_PREMIUM_TO_PROTOCOL()"
    )]
    pub struct FlashloanPremiumToProtocolCall;
    #[doc = "Container type for all input parameters for the `MAX_NUMBER_RESERVES` function with signature `MAX_NUMBER_RESERVES()` and selector `[248, 17, 157, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_NUMBER_RESERVES", abi = "MAX_NUMBER_RESERVES()")]
    pub struct MaxNumberReservesCall;
    #[doc = "Container type for all input parameters for the `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `[232, 47, 236, 47]`"]
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
        name = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
        abi = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT()"
    )]
    pub struct MaxStableRateBorrowSizePercentCall;
    #[doc = "Container type for all input parameters for the `backUnbacked` function with signature `backUnbacked(address,uint256,uint256)` and selector `[214, 93, 199, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "backUnbacked", abi = "backUnbacked(address,uint256,uint256)")]
    pub struct BackUnbackedCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `borrow` function with signature `borrow(address,uint256,uint256,uint16,address)` and selector `[164, 21, 188, 173]`"]
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
        name = "borrow",
        abi = "borrow(address,uint256,uint256,uint16,address)"
    )]
    pub struct BorrowCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub referral_code: u16,
        pub on_behalf_of: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `configureEModeCategory` function with signature `configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))` and selector `[213, 121, 234, 125]`"]
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
        name = "configureEModeCategory",
        abi = "configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))"
    )]
    pub struct ConfigureEModeCategoryCall {
        pub id: u8,
        pub config: EmodeCategory,
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(address,uint256,address,uint16)` and selector `[232, 237, 169, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,uint256,address,uint16)")]
    pub struct DepositCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
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
    #[doc = "Container type for all input parameters for the `finalizeTransfer` function with signature `finalizeTransfer(address,address,address,uint256,uint256,uint256)` and selector `[213, 237, 57, 51]`"]
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
        name = "finalizeTransfer",
        abi = "finalizeTransfer(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct FinalizeTransferCall {
        pub asset: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub balance_from_before: ethers::core::types::U256,
        pub balance_to_before: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)` and selector `[171, 156, 75, 93]`"]
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
        name = "flashLoan",
        abi = "flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)"
    )]
    pub struct FlashLoanCall {
        pub receiver_address: ethers::core::types::Address,
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub interest_rate_modes: ::std::vec::Vec<ethers::core::types::U256>,
        pub on_behalf_of: ethers::core::types::Address,
        pub params: ethers::core::types::Bytes,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `flashLoanSimple` function with signature `flashLoanSimple(address,address,uint256,bytes,uint16)` and selector `[66, 176, 183, 124]`"]
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
        name = "flashLoanSimple",
        abi = "flashLoanSimple(address,address,uint256,bytes,uint16)"
    )]
    pub struct FlashLoanSimpleCall {
        pub receiver_address: ethers::core::types::Address,
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub params: ethers::core::types::Bytes,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `getConfiguration` function with signature `getConfiguration(address)` and selector `[196, 75, 17, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getConfiguration", abi = "getConfiguration(address)")]
    pub struct GetConfigurationCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getEModeCategoryData` function with signature `getEModeCategoryData(uint8)` and selector `[108, 111, 106, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEModeCategoryData", abi = "getEModeCategoryData(uint8)")]
    pub struct GetEModeCategoryDataCall {
        pub id: u8,
    }
    #[doc = "Container type for all input parameters for the `getReserveAddressById` function with signature `getReserveAddressById(uint16)` and selector `[82, 117, 23, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReserveAddressById", abi = "getReserveAddressById(uint16)")]
    pub struct GetReserveAddressByIdCall {
        pub id: u16,
    }
    #[doc = "Container type for all input parameters for the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReserveData", abi = "getReserveData(address)")]
    pub struct GetReserveDataCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `[209, 94, 0, 83]`"]
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
        name = "getReserveNormalizedIncome",
        abi = "getReserveNormalizedIncome(address)"
    )]
    pub struct GetReserveNormalizedIncomeCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `[56, 100, 151, 253]`"]
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
        name = "getReserveNormalizedVariableDebt",
        abi = "getReserveNormalizedVariableDebt(address)"
    )]
    pub struct GetReserveNormalizedVariableDebtCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReservesList` function with signature `getReservesList()` and selector `[209, 148, 109, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReservesList", abi = "getReservesList()")]
    pub struct GetReservesListCall;
    #[doc = "Container type for all input parameters for the `getUserAccountData` function with signature `getUserAccountData(address)` and selector `[191, 146, 133, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserAccountData", abi = "getUserAccountData(address)")]
    pub struct GetUserAccountDataCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserConfiguration` function with signature `getUserConfiguration(address)` and selector `[68, 23, 165, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserConfiguration", abi = "getUserConfiguration(address)")]
    pub struct GetUserConfigurationCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserEMode` function with signature `getUserEMode(address)` and selector `[237, 223, 27, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getUserEMode", abi = "getUserEMode(address)")]
    pub struct GetUserEModeCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initReserve` function with signature `initReserve(address,address,address,address,address)` and selector `[122, 112, 142, 146]`"]
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
        name = "initReserve",
        abi = "initReserve(address,address,address,address,address)"
    )]
    pub struct InitReserveCall {
        pub asset: ethers::core::types::Address,
        pub a_token_address: ethers::core::types::Address,
        pub stable_debt_address: ethers::core::types::Address,
        pub variable_debt_address: ethers::core::types::Address,
        pub interest_rate_strategy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `liquidationCall` function with signature `liquidationCall(address,address,address,uint256,bool)` and selector `[0, 167, 24, 169]`"]
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
        name = "liquidationCall",
        abi = "liquidationCall(address,address,address,uint256,bool)"
    )]
    pub struct LiquidationCallCall {
        pub collateral_asset: ethers::core::types::Address,
        pub debt_asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        pub debt_to_cover: ethers::core::types::U256,
        pub receive_a_token: bool,
    }
    #[doc = "Container type for all input parameters for the `mintToTreasury` function with signature `mintToTreasury(address[])` and selector `[156, 209, 153, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mintToTreasury", abi = "mintToTreasury(address[])")]
    pub struct MintToTreasuryCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `mintUnbacked` function with signature `mintUnbacked(address,uint256,address,uint16)` and selector `[105, 169, 51, 165]`"]
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
        name = "mintUnbacked",
        abi = "mintUnbacked(address,uint256,address,uint16)"
    )]
    pub struct MintUnbackedCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `rebalanceStableBorrowRate` function with signature `rebalanceStableBorrowRate(address,address)` and selector `[205, 17, 35, 130]`"]
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateCall {
        pub asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repay` function with signature `repay(address,uint256,uint256,address)` and selector `[87, 58, 222, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "repay", abi = "repay(address,uint256,uint256,address)")]
    pub struct RepayCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repayWithATokens` function with signature `repayWithATokens(address,uint256,uint256)` and selector `[45, 173, 151, 212]`"]
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
        name = "repayWithATokens",
        abi = "repayWithATokens(address,uint256,uint256)"
    )]
    pub struct RepayWithATokensCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `repayWithPermit` function with signature `repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[238, 62, 33, 11]`"]
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
        name = "repayWithPermit",
        abi = "repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct RepayWithPermitCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `rescueTokens` function with signature `rescueTokens(address,address,uint256)` and selector `[206, 169, 210, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rescueTokens", abi = "rescueTokens(address,address,uint256)")]
    pub struct RescueTokensCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `resetIsolationModeTotalDebt` function with signature `resetIsolationModeTotalDebt(address)` and selector `[228, 62, 136, 161]`"]
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
        name = "resetIsolationModeTotalDebt",
        abi = "resetIsolationModeTotalDebt(address)"
    )]
    pub struct ResetIsolationModeTotalDebtCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setConfiguration` function with signature `setConfiguration(address,(uint256))` and selector `[245, 30, 67, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setConfiguration", abi = "setConfiguration(address,(uint256))")]
    pub struct SetConfigurationCall {
        pub asset: ethers::core::types::Address,
        pub configuration: ReserveConfigurationMap,
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
        pub rate_strategy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setUserEMode` function with signature `setUserEMode(uint8)` and selector `[40, 83, 10, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUserEMode", abi = "setUserEMode(uint8)")]
    pub struct SetUserEModeCall {
        pub category_id: u8,
    }
    #[doc = "Container type for all input parameters for the `setUserUseReserveAsCollateral` function with signature `setUserUseReserveAsCollateral(address,bool)` and selector `[90, 59, 116, 185]`"]
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(address,bool)"
    )]
    pub struct SetUserUseReserveAsCollateralCall {
        pub asset: ethers::core::types::Address,
        pub use_as_collateral: bool,
    }
    #[doc = "Container type for all input parameters for the `supply` function with signature `supply(address,uint256,address,uint16)` and selector `[97, 123, 160, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supply", abi = "supply(address,uint256,address,uint16)")]
    pub struct SupplyCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `supplyWithPermit` function with signature `supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)` and selector `[2, 194, 5, 240]`"]
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
        name = "supplyWithPermit",
        abi = "supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SupplyWithPermitCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
        pub deadline: ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapBorrowRateMode` function with signature `swapBorrowRateMode(address,uint256)` and selector `[148, 186, 137, 162]`"]
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
        name = "swapBorrowRateMode",
        abi = "swapBorrowRateMode(address,uint256)"
    )]
    pub struct SwapBorrowRateModeCall {
        pub asset: ethers::core::types::Address,
        pub interest_rate_mode: ethers::core::types::U256,
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
        pub bridge_protocol_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateFlashloanPremiums` function with signature `updateFlashloanPremiums(uint128,uint128)` and selector `[188, 182, 229, 34]`"]
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
        name = "updateFlashloanPremiums",
        abi = "updateFlashloanPremiums(uint128,uint128)"
    )]
    pub struct UpdateFlashloanPremiumsCall {
        pub flash_loan_premium_total: u128,
        pub flash_loan_premium_to_protocol: u128,
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `[105, 50, 141, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256,address)")]
    pub struct WithdrawCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolCalls {
        AddressesProvider(AddressesProviderCall),
        BridgeProtocolFee(BridgeProtocolFeeCall),
        FlashloanPremiumTotal(FlashloanPremiumTotalCall),
        FlashloanPremiumToProtocol(FlashloanPremiumToProtocolCall),
        MaxNumberReserves(MaxNumberReservesCall),
        MaxStableRateBorrowSizePercent(MaxStableRateBorrowSizePercentCall),
        BackUnbacked(BackUnbackedCall),
        Borrow(BorrowCall),
        ConfigureEModeCategory(ConfigureEModeCategoryCall),
        Deposit(DepositCall),
        DropReserve(DropReserveCall),
        FinalizeTransfer(FinalizeTransferCall),
        FlashLoan(FlashLoanCall),
        FlashLoanSimple(FlashLoanSimpleCall),
        GetConfiguration(GetConfigurationCall),
        GetEModeCategoryData(GetEModeCategoryDataCall),
        GetReserveAddressById(GetReserveAddressByIdCall),
        GetReserveData(GetReserveDataCall),
        GetReserveNormalizedIncome(GetReserveNormalizedIncomeCall),
        GetReserveNormalizedVariableDebt(GetReserveNormalizedVariableDebtCall),
        GetReservesList(GetReservesListCall),
        GetUserAccountData(GetUserAccountDataCall),
        GetUserConfiguration(GetUserConfigurationCall),
        GetUserEMode(GetUserEModeCall),
        InitReserve(InitReserveCall),
        LiquidationCall(LiquidationCallCall),
        MintToTreasury(MintToTreasuryCall),
        MintUnbacked(MintUnbackedCall),
        RebalanceStableBorrowRate(RebalanceStableBorrowRateCall),
        Repay(RepayCall),
        RepayWithATokens(RepayWithATokensCall),
        RepayWithPermit(RepayWithPermitCall),
        RescueTokens(RescueTokensCall),
        ResetIsolationModeTotalDebt(ResetIsolationModeTotalDebtCall),
        SetConfiguration(SetConfigurationCall),
        SetReserveInterestRateStrategyAddress(SetReserveInterestRateStrategyAddressCall),
        SetUserEMode(SetUserEModeCall),
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateralCall),
        Supply(SupplyCall),
        SupplyWithPermit(SupplyWithPermitCall),
        SwapBorrowRateMode(SwapBorrowRateModeCall),
        UpdateBridgeProtocolFee(UpdateBridgeProtocolFeeCall),
        UpdateFlashloanPremiums(UpdateFlashloanPremiumsCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <BridgeProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::BridgeProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <FlashloanPremiumTotalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::FlashloanPremiumTotal(decoded));
            }
            if let Ok(decoded) =
                <FlashloanPremiumToProtocolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::FlashloanPremiumToProtocol(decoded));
            }
            if let Ok(decoded) =
                <MaxNumberReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::MaxNumberReserves(decoded));
            }
            if let Ok(decoded) =
                <MaxStableRateBorrowSizePercentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::MaxStableRateBorrowSizePercent(decoded));
            }
            if let Ok(decoded) =
                <BackUnbackedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::BackUnbacked(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <ConfigureEModeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::ConfigureEModeCategory(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DropReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::DropReserve(decoded));
            }
            if let Ok(decoded) =
                <FinalizeTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::FinalizeTransfer(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanSimpleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::FlashLoanSimple(decoded));
            }
            if let Ok(decoded) =
                <GetConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetConfiguration(decoded));
            }
            if let Ok(decoded) =
                <GetEModeCategoryDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetEModeCategoryData(decoded));
            }
            if let Ok(decoded) =
                <GetReserveAddressByIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetReserveAddressById(decoded));
            }
            if let Ok(decoded) =
                <GetReserveDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetReserveData(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedIncomeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::GetReserveNormalizedIncome(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::GetReserveNormalizedVariableDebt(decoded));
            }
            if let Ok(decoded) =
                <GetReservesListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetReservesList(decoded));
            }
            if let Ok(decoded) =
                <GetUserAccountDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetUserAccountData(decoded));
            }
            if let Ok(decoded) =
                <GetUserConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetUserConfiguration(decoded));
            }
            if let Ok(decoded) =
                <GetUserEModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::GetUserEMode(decoded));
            }
            if let Ok(decoded) =
                <InitReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::InitReserve(decoded));
            }
            if let Ok(decoded) =
                <LiquidationCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::LiquidationCall(decoded));
            }
            if let Ok(decoded) =
                <MintToTreasuryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::MintToTreasury(decoded));
            }
            if let Ok(decoded) =
                <MintUnbackedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::MintUnbacked(decoded));
            }
            if let Ok(decoded) =
                <RebalanceStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::RebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded) = <RepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Repay(decoded));
            }
            if let Ok(decoded) =
                <RepayWithATokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::RepayWithATokens(decoded));
            }
            if let Ok(decoded) =
                <RepayWithPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::RepayWithPermit(decoded));
            }
            if let Ok(decoded) =
                <RescueTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::RescueTokens(decoded));
            }
            if let Ok(decoded) =
                <ResetIsolationModeTotalDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::ResetIsolationModeTotalDebt(decoded));
            }
            if let Ok(decoded) =
                <SetConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetConfiguration(decoded));
            }
            if let Ok(decoded) =
                <SetReserveInterestRateStrategyAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::SetReserveInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded) =
                <SetUserEModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SetUserEMode(decoded));
            }
            if let Ok(decoded) =
                <SetUserUseReserveAsCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IPoolCalls::SetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded) = <SupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Supply(decoded));
            }
            if let Ok(decoded) =
                <SupplyWithPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SupplyWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapBorrowRateModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::SwapBorrowRateMode(decoded));
            }
            if let Ok(decoded) =
                <UpdateBridgeProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::UpdateBridgeProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <UpdateFlashloanPremiumsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::UpdateFlashloanPremiums(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolCalls::AddressesProvider(element) => element.encode(),
                IPoolCalls::BridgeProtocolFee(element) => element.encode(),
                IPoolCalls::FlashloanPremiumTotal(element) => element.encode(),
                IPoolCalls::FlashloanPremiumToProtocol(element) => element.encode(),
                IPoolCalls::MaxNumberReserves(element) => element.encode(),
                IPoolCalls::MaxStableRateBorrowSizePercent(element) => element.encode(),
                IPoolCalls::BackUnbacked(element) => element.encode(),
                IPoolCalls::Borrow(element) => element.encode(),
                IPoolCalls::ConfigureEModeCategory(element) => element.encode(),
                IPoolCalls::Deposit(element) => element.encode(),
                IPoolCalls::DropReserve(element) => element.encode(),
                IPoolCalls::FinalizeTransfer(element) => element.encode(),
                IPoolCalls::FlashLoan(element) => element.encode(),
                IPoolCalls::FlashLoanSimple(element) => element.encode(),
                IPoolCalls::GetConfiguration(element) => element.encode(),
                IPoolCalls::GetEModeCategoryData(element) => element.encode(),
                IPoolCalls::GetReserveAddressById(element) => element.encode(),
                IPoolCalls::GetReserveData(element) => element.encode(),
                IPoolCalls::GetReserveNormalizedIncome(element) => element.encode(),
                IPoolCalls::GetReserveNormalizedVariableDebt(element) => element.encode(),
                IPoolCalls::GetReservesList(element) => element.encode(),
                IPoolCalls::GetUserAccountData(element) => element.encode(),
                IPoolCalls::GetUserConfiguration(element) => element.encode(),
                IPoolCalls::GetUserEMode(element) => element.encode(),
                IPoolCalls::InitReserve(element) => element.encode(),
                IPoolCalls::LiquidationCall(element) => element.encode(),
                IPoolCalls::MintToTreasury(element) => element.encode(),
                IPoolCalls::MintUnbacked(element) => element.encode(),
                IPoolCalls::RebalanceStableBorrowRate(element) => element.encode(),
                IPoolCalls::Repay(element) => element.encode(),
                IPoolCalls::RepayWithATokens(element) => element.encode(),
                IPoolCalls::RepayWithPermit(element) => element.encode(),
                IPoolCalls::RescueTokens(element) => element.encode(),
                IPoolCalls::ResetIsolationModeTotalDebt(element) => element.encode(),
                IPoolCalls::SetConfiguration(element) => element.encode(),
                IPoolCalls::SetReserveInterestRateStrategyAddress(element) => element.encode(),
                IPoolCalls::SetUserEMode(element) => element.encode(),
                IPoolCalls::SetUserUseReserveAsCollateral(element) => element.encode(),
                IPoolCalls::Supply(element) => element.encode(),
                IPoolCalls::SupplyWithPermit(element) => element.encode(),
                IPoolCalls::SwapBorrowRateMode(element) => element.encode(),
                IPoolCalls::UpdateBridgeProtocolFee(element) => element.encode(),
                IPoolCalls::UpdateFlashloanPremiums(element) => element.encode(),
                IPoolCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolCalls::AddressesProvider(element) => element.fmt(f),
                IPoolCalls::BridgeProtocolFee(element) => element.fmt(f),
                IPoolCalls::FlashloanPremiumTotal(element) => element.fmt(f),
                IPoolCalls::FlashloanPremiumToProtocol(element) => element.fmt(f),
                IPoolCalls::MaxNumberReserves(element) => element.fmt(f),
                IPoolCalls::MaxStableRateBorrowSizePercent(element) => element.fmt(f),
                IPoolCalls::BackUnbacked(element) => element.fmt(f),
                IPoolCalls::Borrow(element) => element.fmt(f),
                IPoolCalls::ConfigureEModeCategory(element) => element.fmt(f),
                IPoolCalls::Deposit(element) => element.fmt(f),
                IPoolCalls::DropReserve(element) => element.fmt(f),
                IPoolCalls::FinalizeTransfer(element) => element.fmt(f),
                IPoolCalls::FlashLoan(element) => element.fmt(f),
                IPoolCalls::FlashLoanSimple(element) => element.fmt(f),
                IPoolCalls::GetConfiguration(element) => element.fmt(f),
                IPoolCalls::GetEModeCategoryData(element) => element.fmt(f),
                IPoolCalls::GetReserveAddressById(element) => element.fmt(f),
                IPoolCalls::GetReserveData(element) => element.fmt(f),
                IPoolCalls::GetReserveNormalizedIncome(element) => element.fmt(f),
                IPoolCalls::GetReserveNormalizedVariableDebt(element) => element.fmt(f),
                IPoolCalls::GetReservesList(element) => element.fmt(f),
                IPoolCalls::GetUserAccountData(element) => element.fmt(f),
                IPoolCalls::GetUserConfiguration(element) => element.fmt(f),
                IPoolCalls::GetUserEMode(element) => element.fmt(f),
                IPoolCalls::InitReserve(element) => element.fmt(f),
                IPoolCalls::LiquidationCall(element) => element.fmt(f),
                IPoolCalls::MintToTreasury(element) => element.fmt(f),
                IPoolCalls::MintUnbacked(element) => element.fmt(f),
                IPoolCalls::RebalanceStableBorrowRate(element) => element.fmt(f),
                IPoolCalls::Repay(element) => element.fmt(f),
                IPoolCalls::RepayWithATokens(element) => element.fmt(f),
                IPoolCalls::RepayWithPermit(element) => element.fmt(f),
                IPoolCalls::RescueTokens(element) => element.fmt(f),
                IPoolCalls::ResetIsolationModeTotalDebt(element) => element.fmt(f),
                IPoolCalls::SetConfiguration(element) => element.fmt(f),
                IPoolCalls::SetReserveInterestRateStrategyAddress(element) => element.fmt(f),
                IPoolCalls::SetUserEMode(element) => element.fmt(f),
                IPoolCalls::SetUserUseReserveAsCollateral(element) => element.fmt(f),
                IPoolCalls::Supply(element) => element.fmt(f),
                IPoolCalls::SupplyWithPermit(element) => element.fmt(f),
                IPoolCalls::SwapBorrowRateMode(element) => element.fmt(f),
                IPoolCalls::UpdateBridgeProtocolFee(element) => element.fmt(f),
                IPoolCalls::UpdateFlashloanPremiums(element) => element.fmt(f),
                IPoolCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for IPoolCalls {
        fn from(var: AddressesProviderCall) -> Self {
            IPoolCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<BridgeProtocolFeeCall> for IPoolCalls {
        fn from(var: BridgeProtocolFeeCall) -> Self {
            IPoolCalls::BridgeProtocolFee(var)
        }
    }
    impl ::std::convert::From<FlashloanPremiumTotalCall> for IPoolCalls {
        fn from(var: FlashloanPremiumTotalCall) -> Self {
            IPoolCalls::FlashloanPremiumTotal(var)
        }
    }
    impl ::std::convert::From<FlashloanPremiumToProtocolCall> for IPoolCalls {
        fn from(var: FlashloanPremiumToProtocolCall) -> Self {
            IPoolCalls::FlashloanPremiumToProtocol(var)
        }
    }
    impl ::std::convert::From<MaxNumberReservesCall> for IPoolCalls {
        fn from(var: MaxNumberReservesCall) -> Self {
            IPoolCalls::MaxNumberReserves(var)
        }
    }
    impl ::std::convert::From<MaxStableRateBorrowSizePercentCall> for IPoolCalls {
        fn from(var: MaxStableRateBorrowSizePercentCall) -> Self {
            IPoolCalls::MaxStableRateBorrowSizePercent(var)
        }
    }
    impl ::std::convert::From<BackUnbackedCall> for IPoolCalls {
        fn from(var: BackUnbackedCall) -> Self {
            IPoolCalls::BackUnbacked(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for IPoolCalls {
        fn from(var: BorrowCall) -> Self {
            IPoolCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<ConfigureEModeCategoryCall> for IPoolCalls {
        fn from(var: ConfigureEModeCategoryCall) -> Self {
            IPoolCalls::ConfigureEModeCategory(var)
        }
    }
    impl ::std::convert::From<DepositCall> for IPoolCalls {
        fn from(var: DepositCall) -> Self {
            IPoolCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DropReserveCall> for IPoolCalls {
        fn from(var: DropReserveCall) -> Self {
            IPoolCalls::DropReserve(var)
        }
    }
    impl ::std::convert::From<FinalizeTransferCall> for IPoolCalls {
        fn from(var: FinalizeTransferCall) -> Self {
            IPoolCalls::FinalizeTransfer(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for IPoolCalls {
        fn from(var: FlashLoanCall) -> Self {
            IPoolCalls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<FlashLoanSimpleCall> for IPoolCalls {
        fn from(var: FlashLoanSimpleCall) -> Self {
            IPoolCalls::FlashLoanSimple(var)
        }
    }
    impl ::std::convert::From<GetConfigurationCall> for IPoolCalls {
        fn from(var: GetConfigurationCall) -> Self {
            IPoolCalls::GetConfiguration(var)
        }
    }
    impl ::std::convert::From<GetEModeCategoryDataCall> for IPoolCalls {
        fn from(var: GetEModeCategoryDataCall) -> Self {
            IPoolCalls::GetEModeCategoryData(var)
        }
    }
    impl ::std::convert::From<GetReserveAddressByIdCall> for IPoolCalls {
        fn from(var: GetReserveAddressByIdCall) -> Self {
            IPoolCalls::GetReserveAddressById(var)
        }
    }
    impl ::std::convert::From<GetReserveDataCall> for IPoolCalls {
        fn from(var: GetReserveDataCall) -> Self {
            IPoolCalls::GetReserveData(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedIncomeCall> for IPoolCalls {
        fn from(var: GetReserveNormalizedIncomeCall) -> Self {
            IPoolCalls::GetReserveNormalizedIncome(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedVariableDebtCall> for IPoolCalls {
        fn from(var: GetReserveNormalizedVariableDebtCall) -> Self {
            IPoolCalls::GetReserveNormalizedVariableDebt(var)
        }
    }
    impl ::std::convert::From<GetReservesListCall> for IPoolCalls {
        fn from(var: GetReservesListCall) -> Self {
            IPoolCalls::GetReservesList(var)
        }
    }
    impl ::std::convert::From<GetUserAccountDataCall> for IPoolCalls {
        fn from(var: GetUserAccountDataCall) -> Self {
            IPoolCalls::GetUserAccountData(var)
        }
    }
    impl ::std::convert::From<GetUserConfigurationCall> for IPoolCalls {
        fn from(var: GetUserConfigurationCall) -> Self {
            IPoolCalls::GetUserConfiguration(var)
        }
    }
    impl ::std::convert::From<GetUserEModeCall> for IPoolCalls {
        fn from(var: GetUserEModeCall) -> Self {
            IPoolCalls::GetUserEMode(var)
        }
    }
    impl ::std::convert::From<InitReserveCall> for IPoolCalls {
        fn from(var: InitReserveCall) -> Self {
            IPoolCalls::InitReserve(var)
        }
    }
    impl ::std::convert::From<LiquidationCallCall> for IPoolCalls {
        fn from(var: LiquidationCallCall) -> Self {
            IPoolCalls::LiquidationCall(var)
        }
    }
    impl ::std::convert::From<MintToTreasuryCall> for IPoolCalls {
        fn from(var: MintToTreasuryCall) -> Self {
            IPoolCalls::MintToTreasury(var)
        }
    }
    impl ::std::convert::From<MintUnbackedCall> for IPoolCalls {
        fn from(var: MintUnbackedCall) -> Self {
            IPoolCalls::MintUnbacked(var)
        }
    }
    impl ::std::convert::From<RebalanceStableBorrowRateCall> for IPoolCalls {
        fn from(var: RebalanceStableBorrowRateCall) -> Self {
            IPoolCalls::RebalanceStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<RepayCall> for IPoolCalls {
        fn from(var: RepayCall) -> Self {
            IPoolCalls::Repay(var)
        }
    }
    impl ::std::convert::From<RepayWithATokensCall> for IPoolCalls {
        fn from(var: RepayWithATokensCall) -> Self {
            IPoolCalls::RepayWithATokens(var)
        }
    }
    impl ::std::convert::From<RepayWithPermitCall> for IPoolCalls {
        fn from(var: RepayWithPermitCall) -> Self {
            IPoolCalls::RepayWithPermit(var)
        }
    }
    impl ::std::convert::From<RescueTokensCall> for IPoolCalls {
        fn from(var: RescueTokensCall) -> Self {
            IPoolCalls::RescueTokens(var)
        }
    }
    impl ::std::convert::From<ResetIsolationModeTotalDebtCall> for IPoolCalls {
        fn from(var: ResetIsolationModeTotalDebtCall) -> Self {
            IPoolCalls::ResetIsolationModeTotalDebt(var)
        }
    }
    impl ::std::convert::From<SetConfigurationCall> for IPoolCalls {
        fn from(var: SetConfigurationCall) -> Self {
            IPoolCalls::SetConfiguration(var)
        }
    }
    impl ::std::convert::From<SetReserveInterestRateStrategyAddressCall> for IPoolCalls {
        fn from(var: SetReserveInterestRateStrategyAddressCall) -> Self {
            IPoolCalls::SetReserveInterestRateStrategyAddress(var)
        }
    }
    impl ::std::convert::From<SetUserEModeCall> for IPoolCalls {
        fn from(var: SetUserEModeCall) -> Self {
            IPoolCalls::SetUserEMode(var)
        }
    }
    impl ::std::convert::From<SetUserUseReserveAsCollateralCall> for IPoolCalls {
        fn from(var: SetUserUseReserveAsCollateralCall) -> Self {
            IPoolCalls::SetUserUseReserveAsCollateral(var)
        }
    }
    impl ::std::convert::From<SupplyCall> for IPoolCalls {
        fn from(var: SupplyCall) -> Self {
            IPoolCalls::Supply(var)
        }
    }
    impl ::std::convert::From<SupplyWithPermitCall> for IPoolCalls {
        fn from(var: SupplyWithPermitCall) -> Self {
            IPoolCalls::SupplyWithPermit(var)
        }
    }
    impl ::std::convert::From<SwapBorrowRateModeCall> for IPoolCalls {
        fn from(var: SwapBorrowRateModeCall) -> Self {
            IPoolCalls::SwapBorrowRateMode(var)
        }
    }
    impl ::std::convert::From<UpdateBridgeProtocolFeeCall> for IPoolCalls {
        fn from(var: UpdateBridgeProtocolFeeCall) -> Self {
            IPoolCalls::UpdateBridgeProtocolFee(var)
        }
    }
    impl ::std::convert::From<UpdateFlashloanPremiumsCall> for IPoolCalls {
        fn from(var: UpdateFlashloanPremiumsCall) -> Self {
            IPoolCalls::UpdateFlashloanPremiums(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IPoolCalls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddressesProviderReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `BRIDGE_PROTOCOL_FEE` function with signature `BRIDGE_PROTOCOL_FEE()` and selector `[39, 45, 144, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BridgeProtocolFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `FLASHLOAN_PREMIUM_TOTAL` function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `[7, 75, 46, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FlashloanPremiumTotalReturn(pub u128);
    #[doc = "Container type for all return fields from the `FLASHLOAN_PREMIUM_TO_PROTOCOL` function with signature `FLASHLOAN_PREMIUM_TO_PROTOCOL()` and selector `[106, 153, 192, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FlashloanPremiumToProtocolReturn(pub u128);
    #[doc = "Container type for all return fields from the `MAX_NUMBER_RESERVES` function with signature `MAX_NUMBER_RESERVES()` and selector `[248, 17, 157, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxNumberReservesReturn(pub u16);
    #[doc = "Container type for all return fields from the `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `[232, 47, 236, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MaxStableRateBorrowSizePercentReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getConfiguration` function with signature `getConfiguration(address)` and selector `[196, 75, 17, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetConfigurationReturn(pub ReserveConfigurationMap);
    #[doc = "Container type for all return fields from the `getEModeCategoryData` function with signature `getEModeCategoryData(uint8)` and selector `[108, 111, 106, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetEModeCategoryDataReturn(pub EmodeCategory);
    #[doc = "Container type for all return fields from the `getReserveAddressById` function with signature `getReserveAddressById(uint16)` and selector `[82, 117, 23, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveAddressByIdReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getReserveData` function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveDataReturn(pub ReserveData);
    #[doc = "Container type for all return fields from the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `[209, 94, 0, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveNormalizedIncomeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `[56, 100, 151, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReserveNormalizedVariableDebtReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReservesList` function with signature `getReservesList()` and selector `[209, 148, 109, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReservesListReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getUserAccountData` function with signature `getUserAccountData(address)` and selector `[191, 146, 133, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserAccountDataReturn {
        pub total_collateral_base: ethers::core::types::U256,
        pub total_debt_base: ethers::core::types::U256,
        pub available_borrows_base: ethers::core::types::U256,
        pub current_liquidation_threshold: ethers::core::types::U256,
        pub ltv: ethers::core::types::U256,
        pub health_factor: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getUserConfiguration` function with signature `getUserConfiguration(address)` and selector `[68, 23, 165, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserConfigurationReturn(pub UserConfigurationMap);
    #[doc = "Container type for all return fields from the `getUserEMode` function with signature `getUserEMode(address)` and selector `[237, 223, 27, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetUserEModeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repay` function with signature `repay(address,uint256,uint256,address)` and selector `[87, 58, 222, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repayWithATokens` function with signature `repayWithATokens(address,uint256,uint256)` and selector `[45, 173, 151, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayWithATokensReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `repayWithPermit` function with signature `repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `[238, 62, 33, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RepayWithPermitReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `[105, 50, 141, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WithdrawReturn(pub ethers::core::types::U256);
}
