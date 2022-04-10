pub use errors_mod::*;
#[allow(clippy::too_many_arguments)]
mod errors_mod {
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
    #[doc = "Errors was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERRORS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ACL_ADMIN_CANNOT_BE_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER_ALREADY_ADDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER_NOT_REGISTERED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ASSET_NOT_BORROWABLE_IN_ISOLATION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ASSET_NOT_LISTED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ATOKEN_SUPPLY_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BORROWING_NOT_ENABLED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BORROW_CAP_EXCEEDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BRIDGE_PROTOCOL_FEE_INVALID\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_MUST_BE_POOL\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_ATOKEN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_BRIDGE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_EMERGENCY_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_POOL_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_POOL_CONFIGURATOR\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_POOL_OR_EMERGENCY_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CALLER_NOT_RISK_OR_POOL_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"COLLATERAL_BALANCE_IS_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"COLLATERAL_CANNOT_BE_LIQUIDATED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"COLLATERAL_CANNOT_COVER_NEW_BORROW\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"COLLATERAL_SAME_AS_BORROWING_CURRENCY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEBT_CEILING_EXCEEDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEBT_CEILING_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EMODE_CATEGORY_RESERVED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FLASHLOAN_PREMIUM_INVALID\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HEALTH_FACTOR_NOT_BELOW_THRESHOLD\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCONSISTENT_EMODE_CATEGORY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCONSISTENT_FLASHLOAN_PARAMS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCONSISTENT_PARAMS_LENGTH\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_ADDRESSES_PROVIDER_ID\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_AMOUNT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_BORROW_CAP\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_BURN_AMOUNT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_DEBT_CEILING\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_DECIMALS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_EMODE_CATEGORY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_EMODE_CATEGORY_ASSIGNMENT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_EMODE_CATEGORY_PARAMS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_EXPIRATION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_FLASHLOAN_EXECUTOR_RETURN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_INTEREST_RATE_MODE_SELECTED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_LIQUIDATION_PROTOCOL_FEE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_LIQ_BONUS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_LIQ_THRESHOLD\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_LTV\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_MINT_AMOUNT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_OPTIMAL_USAGE_RATIO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_RESERVE_FACTOR\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_RESERVE_INDEX\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_RESERVE_PARAMS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_SIGNATURE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_SUPPLY_CAP\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INVALID_UNBACKED_MINT_CAP\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"LTV_VALIDATION_FAILED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NOT_CONTRACT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NOT_ENOUGH_AVAILABLE_USER_BALANCE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NO_DEBT_OF_SELECTED_TYPE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NO_MORE_RESERVES_ALLOWED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NO_OUTSTANDING_STABLE_DEBT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NO_OUTSTANDING_VARIABLE_DEBT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"OPERATION_NOT_SUPPORTED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_ADDRESSES_DO_NOT_MATCH\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PRICE_ORACLE_SENTINEL_CHECK_FAILED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_ALREADY_ADDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_ALREADY_INITIALIZED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_DEBT_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_FROZEN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_INACTIVE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_LIQUIDITY_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"RESERVE_PAUSED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SAME_BLOCK_BORROW_REPAY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SILOED_BORROWING_VIOLATION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"STABLE_BORROWING_ENABLED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"STABLE_BORROWING_NOT_ENABLED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"STABLE_DEBT_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"SUPPLY_CAP_EXCEEDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNBACKED_MINT_CAP_EXCEEDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_BALANCE_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"UNDERLYING_CANNOT_BE_RESCUED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"USER_IN_ISOLATION_MODE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"VARIABLE_DEBT_SUPPLY_NOT_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_ADDRESS_NOT_VALID\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERRORS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61112f61003a600b82828239805160001a60731461002d57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe73000000000000000000000000000000000000000030146080604052600436106104f85760003560e01c806389c5d45f11610298578063bad8308c11610171578063dd1dd95f116100e3578063f07f67851161009c578063f07f678514610fde578063f10727db14610fff578063f479ea1114611020578063fa163a8314611041578063fae8279114611062578063fd1828ff1461108357600080fd5b8063dd1dd95f14610f19578063de24948c14610f3a578063e02f07ee14610f5b578063e3fa20f514610f7b578063e4dd8b7414610f9c578063e981483a14610fbd57600080fd5b8063d14bb17a11610135578063d14bb17a14610e53578063d1cd8b1d14610e74578063d6f9fcde14610e95578063d9adda8514610eb6578063dc191bd914610ed7578063dcc56db614610ef857600080fd5b8063bad8308c14610dae578063c08a114614610dcf578063c863808214610df0578063c899301a14610e11578063cd23367c14610e3257600080fd5b8063a4868dca1161020a578063b0510054116101ce578063b051005414610ce9578063b4a4573014610d0a578063b5e7936614610d2b578063b68774e914610d4b578063b7f5e22414610d6c578063b87041c214610d8d57600080fd5b8063a4868dca14610c45578063a8c9785314610c66578063ab883ca014610c87578063abd351b114610ca8578063ac75323614610cc957600080fd5b8063952633c51161025c578063952633c514610b7f5780639527e9d914610ba057806399ce53f314610bc1578063a2797c8014610be2578063a2e976c614610c03578063a3402a3814610c2457600080fd5b806389c5d45f14610ada5780638a34400014610afb5780638b8b98d714610b1c5780638eda46bd14610b3d5780638f7722b214610b5e57600080fd5b80634e3aed37116103d55780636b3f7cc711610347578063747fa55611610300578063747fa55614610a1457806376ae8fca14610a355780637aa0767e14610a565780637fea6f3614610a775780638596aad514610a98578063895f7dc814610ab957600080fd5b80636b3f7cc71461094e5780636cd3cfbc1461096f578063712f536a1461099057806373dea5e3146109b1578063744465cc146109d257806374459b14146109f357600080fd5b8063570e354711610399578063570e3547146108895780635d9c76c0146108aa57806360c3de80146108cb57806361c111d2146108eb57806365a83bab1461090c57806365e7ef4c1461092d57600080fd5b80634e3aed37146107e55780634ef999ff146108065780634f77647b14610827578063512674501461084757806352ba9dbe1461086857600080fd5b80632eed17e81161046e578063471df68511610432578063471df6851461072057806347ba93d81461074157806347cf152314610762578063485c8ff6146107835780634d86f393146107a35780634e01e3c1146107c457600080fd5b80632eed17e81461067b578063335763de1461069c578063366eb54d146106bd57806337930782146106de57806343e97c6b146106ff57600080fd5b80631abbb001116104c05780631abbb001146105b757806322a73446146105d857806326bbd053146105f957806326e7b3121461061a5780632926c9711461063a5780632c8e3b4c1461065b57600080fd5b8063084dfa0d146104fd57806311d7b0061461053457806312dcade81461055457806314dcfbbc14610575578063198d6a6b14610596575b600080fd5b61051e60405180604001604052806002815260200161062760f31b81525081565b60405161052b91906110a4565b60405180910390f35b61051e604051806040016040528060018152602001603960f81b81525081565b61051e604051806040016040528060028152602001610c4d60f21b81525081565b61051e604051806040016040528060028152602001611c1b60f11b81525081565b61051e60405180604001604052806002815260200161070760f31b81525081565b61051e60405180604001604052806002815260200161383760f01b81525081565b61051e60405180604001604052806002815260200161343760f01b81525081565b61051e60405180604001604052806002815260200161363960f01b81525081565b61051e604051806040016040528060018152602001603360f81b81525081565b61051e604051806040016040528060028152602001610d0d60f21b81525081565b61051e604051806040016040528060018152602001603560f81b81525081565b61051e60405180604001604052806002815260200161035360f41b81525081565b61051e60405180604001604052806002815260200161032360f41b81525081565b61051e60405180604001604052806002815260200161333560f01b81525081565b61051e60405180604001604052806002815260200161189960f11b81525081565b61051e604051806040016040528060028152602001610d4d60f21b81525081565b61051e60405180604001604052806002815260200161323360f01b81525081565b61051e604051806040016040528060028152602001611b9960f11b81525081565b61051e60405180604001604052806002815260200161323160f01b81525081565b61051e604051806040016040528060018152602001601960f91b81525081565b61051e60405180604001604052806002815260200161333160f01b81525081565b61051e604051806040016040528060028152602001610ccd60f21b81525081565b61051e60405180604001604052806002815260200161383360f01b81525081565b61051e60405180604001604052806002815260200161033360f41b81525081565b61051e604051806040016040528060018152602001601b60f91b81525081565b61051e60405180604001604052806002815260200161323560f01b81525081565b61051e60405180604001604052806002815260200161323760f01b81525081565b61051e60405180604001604052806002815260200161068760f31b81525081565b61051e60405180604001604052806002815260200161313760f01b81525081565b61051e604051806040016040528060018152602001600760fb1b81525081565b61051e60405180604001604052806002815260200161031360f41b81525081565b61051e60405180604001604052806002815260200161353360f01b81525081565b61051e60405180604001604052806002815260200161353560f01b81525081565b61051e604051806040016040528060028152602001611a9960f11b81525081565b61051e60405180604001604052806002815260200161064760f31b81525081565b61051e60405180604001604052806002815260200161034360f41b81525081565b61051e60405180604001604052806002815260200161343960f01b81525081565b61051e604051806040016040528060028152602001611b1960f11b81525081565b61051e60405180604001604052806002815260200161343160f01b81525081565b61051e60405180604001604052806002815260200161313960f01b81525081565b61051e60405180604001604052806002815260200161313560f01b81525081565b61051e60405180604001604052806002815260200161191960f11b81525081565b61051e60405180604001604052806002815260200161313360f01b81525081565b61051e60405180604001604052806002815260200161036360f41b81525081565b61051e604051806040016040528060028152602001611a1b60f11b81525081565b61051e60405180604001604052806002815260200161333360f01b81525081565b61051e60405180604001604052806002815260200161333760f01b81525081565b61051e60405180604001604052806002815260200161038360f41b81525081565b61051e60405180604001604052806002815260200161037360f41b81525081565b61051e6040518060400160405280600281526020016106a760f31b81525081565b61051e60405180604001604052806002815260200161343560f01b81525081565b61051e60405180604001604052806002815260200161363560f01b81525081565b61051e60405180604001604052806002815260200161363360f01b81525081565b61051e60405180604001604052806002815260200161343360f01b81525081565b61051e60405180604001604052806002815260200161313160f01b81525081565b61051e60405180604001604052806002815260200161373960f01b81525081565b61051e60405180604001604052806002815260200161363760f01b81525081565b61051e60405180604001604052806002815260200161373160f01b81525081565b61051e60405180604001604052806002815260200161383560f01b81525081565b61051e604051806040016040528060028152602001610c8d60f21b81525081565b61051e604051806040016040528060018152602001603160f81b81525081565b61051e60405180604001604052806002815260200161353160f01b81525081565b61051e604051806040016040528060028152602001611a1960f11b81525081565b61051e604051806040016040528060018152602001600d60fa1b81525081565b61051e60405180604001604052806002815260200161323960f01b81525081565b61051e60405180604001604052806002815260200161199960f11b81525081565b61051e60405180604001604052806002815260200161353760f01b81525081565b61051e604051806040016040528060028152602001611b9b60f11b81525081565b61051e6040518060400160405280600281526020016106e760f31b81525081565b61051e60405180604001604052806002815260200161353960f01b81525081565b61051e604051806040016040528060028152602001610e0d60f21b81525081565b61051e604051806040016040528060028152602001611c1960f11b81525081565b61051e60405180604001604052806002815260200161373760f01b81525081565b61051e604051806040016040528060028152602001610dcd60f21b81525081565b61051e6040518060400160405280600281526020016106c760f31b81525081565b61051e60405180604001604052806002815260200161363160f01b81525081565b61051e60405180604001604052806002815260200161333960f01b81525081565b61051e60405180604001604052806002815260200161373360f01b81525081565b61051e604051806040016040528060028152602001610d8d60f21b81525081565b61051e60405180604001604052806002815260200161383960f01b81525081565b61051e604051806040016040528060018152602001603760f81b81525081565b61051e60405180604001604052806002815260200161199b60f11b81525081565b61051e60405180604001604052806002815260200161383160f01b81525081565b61051e60405180604001604052806002815260200161039360f41b81525081565b61051e60405180604001604052806002815260200161066760f31b81525081565b61051e604051806040016040528060028152602001611a9b60f11b81525081565b61051e60405180604001604052806002815260200161189b60f11b81525081565b61051e604051806040016040528060028152602001611b1b60f11b81525081565b61051e60405180604001604052806002815260200161191b60f11b81525081565b61051e60405180604001604052806002815260200161373560f01b81525081565b600060208083528351808285015260005b818110156110d1578581018301518582016040015282016110b5565b818111156110e3576000604083870101525b50601f01601f191692909201604001939250505056fea264697066735822122071ab11db712d5342731b76d998abf9456d911ce887c0df7d39f71bf19ca6f1e764736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Errors<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Errors<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Errors<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Errors))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Errors<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERRORS_ABI.clone(), client).into()
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
                ERRORS_ABI.clone(),
                ERRORS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ACL_ADMIN_CANNOT_BE_ZERO` (0xfd1828ff) function"]
        pub fn acl_admin_cannot_be_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([253, 24, 40, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER_ALREADY_ADDED` (0x14dcfbbc) function"]
        pub fn addresses_provider_already_added(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([20, 220, 251, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER_NOT_REGISTERED` (0xe02f07ee) function"]
        pub fn addresses_provider_not_registered(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([224, 47, 7, 238], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE` (0xf07f6785) function"]
        pub fn amount_bigger_than_max_loan_size_stable(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([240, 127, 103, 133], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ASSET_NOT_BORROWABLE_IN_ISOLATION` (0x8596aad5) function"]
        pub fn asset_not_borrowable_in_isolation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([133, 150, 170, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ASSET_NOT_LISTED` (0xcd23367c) function"]
        pub fn asset_not_listed(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([205, 35, 54, 124], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ATOKEN_SUPPLY_NOT_ZERO` (0x43e97c6b) function"]
        pub fn atoken_supply_not_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([67, 233, 124, 107], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BORROWING_NOT_ENABLED` (0x4ef999ff) function"]
        pub fn borrowing_not_enabled(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([78, 249, 153, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BORROW_CAP_EXCEEDED` (0x2eed17e8) function"]
        pub fn borrow_cap_exceeded(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([46, 237, 23, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `BRIDGE_PROTOCOL_FEE_INVALID` (0x7aa0767e) function"]
        pub fn bridge_protocol_fee_invalid(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([122, 160, 118, 126], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_MUST_BE_POOL` (0x471df685) function"]
        pub fn caller_must_be_pool(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([71, 29, 246, 133], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN` (0x2c8e3b4c) function"]
        pub fn caller_not_asset_listing_or_pool_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([44, 142, 59, 76], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_ATOKEN` (0xa2e976c6) function"]
        pub fn caller_not_atoken(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([162, 233, 118, 198], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_BRIDGE` (0x4f77647b) function"]
        pub fn caller_not_bridge(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([79, 119, 100, 123], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_EMERGENCY_ADMIN` (0x485c8ff6) function"]
        pub fn caller_not_emergency_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([72, 92, 143, 246], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_POOL_ADMIN` (0xac753236) function"]
        pub fn caller_not_pool_admin(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([172, 117, 50, 54], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_POOL_CONFIGURATOR` (0x61c111d2) function"]
        pub fn caller_not_pool_configurator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([97, 193, 17, 210], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN` (0x26e7b312) function"]
        pub fn caller_not_pool_or_emergency_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([38, 231, 179, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CALLER_NOT_RISK_OR_POOL_ADMIN` (0xb5e79366) function"]
        pub fn caller_not_risk_or_pool_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([181, 231, 147, 102], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `COLLATERAL_BALANCE_IS_ZERO` (0x4e01e3c1) function"]
        pub fn collateral_balance_is_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([78, 1, 227, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `COLLATERAL_CANNOT_BE_LIQUIDATED` (0x895f7dc8) function"]
        pub fn collateral_cannot_be_liquidated(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([137, 95, 125, 200], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `COLLATERAL_CANNOT_COVER_NEW_BORROW` (0xe3fa20f5) function"]
        pub fn collateral_cannot_cover_new_borrow(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([227, 250, 32, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `COLLATERAL_SAME_AS_BORROWING_CURRENCY` (0x8a344000) function"]
        pub fn collateral_same_as_borrowing_currency(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([138, 52, 64, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DEBT_CEILING_EXCEEDED` (0x65a83bab) function"]
        pub fn debt_ceiling_exceeded(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([101, 168, 59, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DEBT_CEILING_NOT_ZERO` (0xe4dd8b74) function"]
        pub fn debt_ceiling_not_zero(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([228, 221, 139, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `EMODE_CATEGORY_RESERVED` (0xf479ea11) function"]
        pub fn emode_category_reserved(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([244, 121, 234, 17], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FLASHLOAN_PREMIUM_INVALID` (0x747fa556) function"]
        pub fn flashloan_premium_invalid(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([116, 127, 165, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD` (0x366eb54d) function"]
        pub fn health_factor_lower_than_liquidation_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([54, 110, 181, 77], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `HEALTH_FACTOR_NOT_BELOW_THRESHOLD` (0x952633c5) function"]
        pub fn health_factor_not_below_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 38, 51, 197], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCONSISTENT_EMODE_CATEGORY` (0x8f7722b2) function"]
        pub fn inconsistent_emode_category(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([143, 119, 34, 178], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCONSISTENT_FLASHLOAN_PARAMS` (0x73dea5e3) function"]
        pub fn inconsistent_flashloan_params(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([115, 222, 165, 227], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCONSISTENT_PARAMS_LENGTH` (0xbad8308c) function"]
        pub fn inconsistent_params_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([186, 216, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET` (0x2926c971) function"]
        pub fn interest_rate_rebalance_conditions_not_met(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([41, 38, 201, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_ADDRESSES_PROVIDER` (0x37930782) function"]
        pub fn invalid_addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([55, 147, 7, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_ADDRESSES_PROVIDER_ID` (0x60c3de80) function"]
        pub fn invalid_addresses_provider_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([96, 195, 222, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_AMOUNT` (0xfae82791) function"]
        pub fn invalid_amount(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([250, 232, 39, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_BORROW_CAP` (0xd6f9fcde) function"]
        pub fn invalid_borrow_cap(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([214, 249, 252, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_BURN_AMOUNT` (0x51267450) function"]
        pub fn invalid_burn_amount(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([81, 38, 116, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_DEBT_CEILING` (0xdcc56db6) function"]
        pub fn invalid_debt_ceiling(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([220, 197, 109, 182], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_DECIMALS` (0xfa163a83) function"]
        pub fn invalid_decimals(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([250, 22, 58, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_EMODE_CATEGORY` (0xa8c97853) function"]
        pub fn invalid_emode_category(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([168, 201, 120, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_EMODE_CATEGORY_ASSIGNMENT` (0x5d9c76c0) function"]
        pub fn invalid_emode_category_assignment(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([93, 156, 118, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_EMODE_CATEGORY_PARAMS` (0x47cf1523) function"]
        pub fn invalid_emode_category_params(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([71, 207, 21, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_EXPIRATION` (0xc08a1146) function"]
        pub fn invalid_expiration(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([192, 138, 17, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_FLASHLOAN_EXECUTOR_RETURN` (0x7fea6f36) function"]
        pub fn invalid_flashloan_executor_return(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([127, 234, 111, 54], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_INTEREST_RATE_MODE_SELECTED` (0x89c5d45f) function"]
        pub fn invalid_interest_rate_mode_selected(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([137, 197, 212, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_LIQUIDATION_PROTOCOL_FEE` (0x8eda46bd) function"]
        pub fn invalid_liquidation_protocol_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([142, 218, 70, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_LIQ_BONUS` (0x9527e9d9) function"]
        pub fn invalid_liq_bonus(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 39, 233, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_LIQ_THRESHOLD` (0xdd1dd95f) function"]
        pub fn invalid_liq_threshold(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([221, 29, 217, 95], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_LTV` (0x99ce53f3) function"]
        pub fn invalid_ltv(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([153, 206, 83, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_MINT_AMOUNT` (0xabd351b1) function"]
        pub fn invalid_mint_amount(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([171, 211, 81, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` (0xc899301a) function"]
        pub fn invalid_optimal_stable_to_total_debt_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 153, 48, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_OPTIMAL_USAGE_RATIO` (0x4e3aed37) function"]
        pub fn invalid_optimal_usage_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([78, 58, 237, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_RESERVE_FACTOR` (0xa4868dca) function"]
        pub fn invalid_reserve_factor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([164, 134, 141, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_RESERVE_INDEX` (0xd1cd8b1d) function"]
        pub fn invalid_reserve_index(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([209, 205, 139, 29], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_RESERVE_PARAMS` (0x335763de) function"]
        pub fn invalid_reserve_params(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([51, 87, 99, 222], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_SIGNATURE` (0xa3402a38) function"]
        pub fn invalid_signature(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([163, 64, 42, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_SUPPLY_CAP` (0x26bbd053) function"]
        pub fn invalid_supply_cap(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([38, 187, 208, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INVALID_UNBACKED_MINT_CAP` (0x47ba93d8) function"]
        pub fn invalid_unbacked_mint_cap(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([71, 186, 147, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LTV_VALIDATION_FAILED` (0xb87041c2) function"]
        pub fn ltv_validation_failed(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([184, 112, 65, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NOT_CONTRACT` (0x11d7b006) function"]
        pub fn not_contract(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([17, 215, 176, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NOT_ENOUGH_AVAILABLE_USER_BALANCE` (0xb7f5e224) function"]
        pub fn not_enough_available_user_balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([183, 245, 226, 36], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NO_DEBT_OF_SELECTED_TYPE` (0xdc191bd9) function"]
        pub fn no_debt_of_selected_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([220, 25, 27, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF` (0x712f536a) function"]
        pub fn no_explicit_amount_to_repay_on_behalf(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([113, 47, 83, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NO_MORE_RESERVES_ALLOWED` (0x76ae8fca) function"]
        pub fn no_more_reserves_allowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([118, 174, 143, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NO_OUTSTANDING_STABLE_DEBT` (0x74459b14) function"]
        pub fn no_outstanding_stable_debt(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([116, 69, 155, 20], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NO_OUTSTANDING_VARIABLE_DEBT` (0xb4a45730) function"]
        pub fn no_outstanding_variable_debt(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([180, 164, 87, 48], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `OPERATION_NOT_SUPPORTED` (0x8b8b98d7) function"]
        pub fn operation_not_supported(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([139, 139, 152, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_ADDRESSES_DO_NOT_MATCH` (0x1abbb001) function"]
        pub fn pool_addresses_do_not_match(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([26, 187, 176, 1], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRICE_ORACLE_SENTINEL_CHECK_FAILED` (0xc8638082) function"]
        pub fn price_oracle_sentinel_check_failed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 99, 128, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_ALREADY_ADDED` (0x12dcade8) function"]
        pub fn reserve_already_added(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([18, 220, 173, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_ALREADY_INITIALIZED` (0xd9adda85) function"]
        pub fn reserve_already_initialized(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([217, 173, 218, 133], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_DEBT_NOT_ZERO` (0xe981483a) function"]
        pub fn reserve_debt_not_zero(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([233, 129, 72, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_FROZEN` (0x6cd3cfbc) function"]
        pub fn reserve_frozen(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([108, 211, 207, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_INACTIVE` (0x52ba9dbe) function"]
        pub fn reserve_inactive(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([82, 186, 157, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_LIQUIDITY_NOT_ZERO` (0x084dfa0d) function"]
        pub fn reserve_liquidity_not_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([8, 77, 250, 13], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RESERVE_PAUSED` (0xb68774e9) function"]
        pub fn reserve_paused(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([182, 135, 116, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SAME_BLOCK_BORROW_REPAY` (0x570e3547) function"]
        pub fn same_block_borrow_repay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([87, 14, 53, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SILOED_BORROWING_VIOLATION` (0xde24948c) function"]
        pub fn siloed_borrowing_violation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([222, 36, 148, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER` (0x22a73446) function"]
        pub fn specified_currency_not_borrowed_by_user(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([34, 167, 52, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `STABLE_BORROWING_ENABLED` (0x198d6a6b) function"]
        pub fn stable_borrowing_enabled(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([25, 141, 106, 107], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `STABLE_BORROWING_NOT_ENABLED` (0x4d86f393) function"]
        pub fn stable_borrowing_not_enabled(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([77, 134, 243, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `STABLE_DEBT_NOT_ZERO` (0x65e7ef4c) function"]
        pub fn stable_debt_not_zero(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([101, 231, 239, 76], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SUPPLY_CAP_EXCEEDED` (0xb0510054) function"]
        pub fn supply_cap_exceeded(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([176, 81, 0, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNBACKED_MINT_CAP_EXCEEDED` (0x6b3f7cc7) function"]
        pub fn unbacked_mint_cap_exceeded(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([107, 63, 124, 199], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNDERLYING_BALANCE_ZERO` (0xa2797c80) function"]
        pub fn underlying_balance_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([162, 121, 124, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNDERLYING_CANNOT_BE_RESCUED` (0xab883ca0) function"]
        pub fn underlying_cannot_be_rescued(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([171, 136, 60, 160], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `USER_IN_ISOLATION_MODE` (0x744465cc) function"]
        pub fn user_in_isolation_mode(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([116, 68, 101, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `VARIABLE_DEBT_SUPPLY_NOT_ZERO` (0xf10727db) function"]
        pub fn variable_debt_supply_not_zero(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([241, 7, 39, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_ADDRESS_NOT_VALID` (0xd14bb17a) function"]
        pub fn zero_address_not_valid(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([209, 75, 177, 122], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Errors<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ACL_ADMIN_CANNOT_BE_ZERO`function with signature `ACL_ADMIN_CANNOT_BE_ZERO()` and selector `[253, 24, 40, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ACL_ADMIN_CANNOT_BE_ZERO", abi = "ACL_ADMIN_CANNOT_BE_ZERO()")]
    pub struct AclAdminCannotBeZeroCall;
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER_ALREADY_ADDED`function with signature `ADDRESSES_PROVIDER_ALREADY_ADDED()` and selector `[20, 220, 251, 188]`"]
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
        name = "ADDRESSES_PROVIDER_ALREADY_ADDED",
        abi = "ADDRESSES_PROVIDER_ALREADY_ADDED()"
    )]
    pub struct AddressesProviderAlreadyAddedCall;
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER_NOT_REGISTERED`function with signature `ADDRESSES_PROVIDER_NOT_REGISTERED()` and selector `[224, 47, 7, 238]`"]
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
        name = "ADDRESSES_PROVIDER_NOT_REGISTERED",
        abi = "ADDRESSES_PROVIDER_NOT_REGISTERED()"
    )]
    pub struct AddressesProviderNotRegisteredCall;
    #[doc = "Container type for all input parameters for the `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE`function with signature `AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()` and selector `[240, 127, 103, 133]`"]
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
        name = "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE",
        abi = "AMOUNT_BIGGER_THAN_MAX_LOAN_SIZE_STABLE()"
    )]
    pub struct AmountBiggerThanMaxLoanSizeStableCall;
    #[doc = "Container type for all input parameters for the `ASSET_NOT_BORROWABLE_IN_ISOLATION`function with signature `ASSET_NOT_BORROWABLE_IN_ISOLATION()` and selector `[133, 150, 170, 213]`"]
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
        name = "ASSET_NOT_BORROWABLE_IN_ISOLATION",
        abi = "ASSET_NOT_BORROWABLE_IN_ISOLATION()"
    )]
    pub struct AssetNotBorrowableInIsolationCall;
    #[doc = "Container type for all input parameters for the `ASSET_NOT_LISTED`function with signature `ASSET_NOT_LISTED()` and selector `[205, 35, 54, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ASSET_NOT_LISTED", abi = "ASSET_NOT_LISTED()")]
    pub struct AssetNotListedCall;
    #[doc = "Container type for all input parameters for the `ATOKEN_SUPPLY_NOT_ZERO`function with signature `ATOKEN_SUPPLY_NOT_ZERO()` and selector `[67, 233, 124, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ATOKEN_SUPPLY_NOT_ZERO", abi = "ATOKEN_SUPPLY_NOT_ZERO()")]
    pub struct AtokenSupplyNotZeroCall;
    #[doc = "Container type for all input parameters for the `BORROWING_NOT_ENABLED`function with signature `BORROWING_NOT_ENABLED()` and selector `[78, 249, 153, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BORROWING_NOT_ENABLED", abi = "BORROWING_NOT_ENABLED()")]
    pub struct BorrowingNotEnabledCall;
    #[doc = "Container type for all input parameters for the `BORROW_CAP_EXCEEDED`function with signature `BORROW_CAP_EXCEEDED()` and selector `[46, 237, 23, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "BORROW_CAP_EXCEEDED", abi = "BORROW_CAP_EXCEEDED()")]
    pub struct BorrowCapExceededCall;
    #[doc = "Container type for all input parameters for the `BRIDGE_PROTOCOL_FEE_INVALID`function with signature `BRIDGE_PROTOCOL_FEE_INVALID()` and selector `[122, 160, 118, 126]`"]
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
        name = "BRIDGE_PROTOCOL_FEE_INVALID",
        abi = "BRIDGE_PROTOCOL_FEE_INVALID()"
    )]
    pub struct BridgeProtocolFeeInvalidCall;
    #[doc = "Container type for all input parameters for the `CALLER_MUST_BE_POOL`function with signature `CALLER_MUST_BE_POOL()` and selector `[71, 29, 246, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CALLER_MUST_BE_POOL", abi = "CALLER_MUST_BE_POOL()")]
    pub struct CallerMustBePoolCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN`function with signature `CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()` and selector `[44, 142, 59, 76]`"]
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
        name = "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN",
        abi = "CALLER_NOT_ASSET_LISTING_OR_POOL_ADMIN()"
    )]
    pub struct CallerNotAssetListingOrPoolAdminCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_ATOKEN`function with signature `CALLER_NOT_ATOKEN()` and selector `[162, 233, 118, 198]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CALLER_NOT_ATOKEN", abi = "CALLER_NOT_ATOKEN()")]
    pub struct CallerNotAtokenCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_BRIDGE`function with signature `CALLER_NOT_BRIDGE()` and selector `[79, 119, 100, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CALLER_NOT_BRIDGE", abi = "CALLER_NOT_BRIDGE()")]
    pub struct CallerNotBridgeCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_EMERGENCY_ADMIN`function with signature `CALLER_NOT_EMERGENCY_ADMIN()` and selector `[72, 92, 143, 246]`"]
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
        name = "CALLER_NOT_EMERGENCY_ADMIN",
        abi = "CALLER_NOT_EMERGENCY_ADMIN()"
    )]
    pub struct CallerNotEmergencyAdminCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_POOL_ADMIN`function with signature `CALLER_NOT_POOL_ADMIN()` and selector `[172, 117, 50, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CALLER_NOT_POOL_ADMIN", abi = "CALLER_NOT_POOL_ADMIN()")]
    pub struct CallerNotPoolAdminCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_POOL_CONFIGURATOR`function with signature `CALLER_NOT_POOL_CONFIGURATOR()` and selector `[97, 193, 17, 210]`"]
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
        name = "CALLER_NOT_POOL_CONFIGURATOR",
        abi = "CALLER_NOT_POOL_CONFIGURATOR()"
    )]
    pub struct CallerNotPoolConfiguratorCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN`function with signature `CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()` and selector `[38, 231, 179, 18]`"]
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
        name = "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN",
        abi = "CALLER_NOT_POOL_OR_EMERGENCY_ADMIN()"
    )]
    pub struct CallerNotPoolOrEmergencyAdminCall;
    #[doc = "Container type for all input parameters for the `CALLER_NOT_RISK_OR_POOL_ADMIN`function with signature `CALLER_NOT_RISK_OR_POOL_ADMIN()` and selector `[181, 231, 147, 102]`"]
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
        name = "CALLER_NOT_RISK_OR_POOL_ADMIN",
        abi = "CALLER_NOT_RISK_OR_POOL_ADMIN()"
    )]
    pub struct CallerNotRiskOrPoolAdminCall;
    #[doc = "Container type for all input parameters for the `COLLATERAL_BALANCE_IS_ZERO`function with signature `COLLATERAL_BALANCE_IS_ZERO()` and selector `[78, 1, 227, 193]`"]
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
        name = "COLLATERAL_BALANCE_IS_ZERO",
        abi = "COLLATERAL_BALANCE_IS_ZERO()"
    )]
    pub struct CollateralBalanceIsZeroCall;
    #[doc = "Container type for all input parameters for the `COLLATERAL_CANNOT_BE_LIQUIDATED`function with signature `COLLATERAL_CANNOT_BE_LIQUIDATED()` and selector `[137, 95, 125, 200]`"]
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
        name = "COLLATERAL_CANNOT_BE_LIQUIDATED",
        abi = "COLLATERAL_CANNOT_BE_LIQUIDATED()"
    )]
    pub struct CollateralCannotBeLiquidatedCall;
    #[doc = "Container type for all input parameters for the `COLLATERAL_CANNOT_COVER_NEW_BORROW`function with signature `COLLATERAL_CANNOT_COVER_NEW_BORROW()` and selector `[227, 250, 32, 245]`"]
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
        name = "COLLATERAL_CANNOT_COVER_NEW_BORROW",
        abi = "COLLATERAL_CANNOT_COVER_NEW_BORROW()"
    )]
    pub struct CollateralCannotCoverNewBorrowCall;
    #[doc = "Container type for all input parameters for the `COLLATERAL_SAME_AS_BORROWING_CURRENCY`function with signature `COLLATERAL_SAME_AS_BORROWING_CURRENCY()` and selector `[138, 52, 64, 0]`"]
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
        name = "COLLATERAL_SAME_AS_BORROWING_CURRENCY",
        abi = "COLLATERAL_SAME_AS_BORROWING_CURRENCY()"
    )]
    pub struct CollateralSameAsBorrowingCurrencyCall;
    #[doc = "Container type for all input parameters for the `DEBT_CEILING_EXCEEDED`function with signature `DEBT_CEILING_EXCEEDED()` and selector `[101, 168, 59, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEBT_CEILING_EXCEEDED", abi = "DEBT_CEILING_EXCEEDED()")]
    pub struct DebtCeilingExceededCall;
    #[doc = "Container type for all input parameters for the `DEBT_CEILING_NOT_ZERO`function with signature `DEBT_CEILING_NOT_ZERO()` and selector `[228, 221, 139, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEBT_CEILING_NOT_ZERO", abi = "DEBT_CEILING_NOT_ZERO()")]
    pub struct DebtCeilingNotZeroCall;
    #[doc = "Container type for all input parameters for the `EMODE_CATEGORY_RESERVED`function with signature `EMODE_CATEGORY_RESERVED()` and selector `[244, 121, 234, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "EMODE_CATEGORY_RESERVED", abi = "EMODE_CATEGORY_RESERVED()")]
    pub struct EmodeCategoryReservedCall;
    #[doc = "Container type for all input parameters for the `FLASHLOAN_PREMIUM_INVALID`function with signature `FLASHLOAN_PREMIUM_INVALID()` and selector `[116, 127, 165, 86]`"]
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
        name = "FLASHLOAN_PREMIUM_INVALID",
        abi = "FLASHLOAN_PREMIUM_INVALID()"
    )]
    pub struct FlashloanPremiumInvalidCall;
    #[doc = "Container type for all input parameters for the `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD`function with signature `HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()` and selector `[54, 110, 181, 77]`"]
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
        name = "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD",
        abi = "HEALTH_FACTOR_LOWER_THAN_LIQUIDATION_THRESHOLD()"
    )]
    pub struct HealthFactorLowerThanLiquidationThresholdCall;
    #[doc = "Container type for all input parameters for the `HEALTH_FACTOR_NOT_BELOW_THRESHOLD`function with signature `HEALTH_FACTOR_NOT_BELOW_THRESHOLD()` and selector `[149, 38, 51, 197]`"]
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
        name = "HEALTH_FACTOR_NOT_BELOW_THRESHOLD",
        abi = "HEALTH_FACTOR_NOT_BELOW_THRESHOLD()"
    )]
    pub struct HealthFactorNotBelowThresholdCall;
    #[doc = "Container type for all input parameters for the `INCONSISTENT_EMODE_CATEGORY`function with signature `INCONSISTENT_EMODE_CATEGORY()` and selector `[143, 119, 34, 178]`"]
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
        name = "INCONSISTENT_EMODE_CATEGORY",
        abi = "INCONSISTENT_EMODE_CATEGORY()"
    )]
    pub struct InconsistentEmodeCategoryCall;
    #[doc = "Container type for all input parameters for the `INCONSISTENT_FLASHLOAN_PARAMS`function with signature `INCONSISTENT_FLASHLOAN_PARAMS()` and selector `[115, 222, 165, 227]`"]
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
        name = "INCONSISTENT_FLASHLOAN_PARAMS",
        abi = "INCONSISTENT_FLASHLOAN_PARAMS()"
    )]
    pub struct InconsistentFlashloanParamsCall;
    #[doc = "Container type for all input parameters for the `INCONSISTENT_PARAMS_LENGTH`function with signature `INCONSISTENT_PARAMS_LENGTH()` and selector `[186, 216, 48, 140]`"]
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
        name = "INCONSISTENT_PARAMS_LENGTH",
        abi = "INCONSISTENT_PARAMS_LENGTH()"
    )]
    pub struct InconsistentParamsLengthCall;
    #[doc = "Container type for all input parameters for the `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET`function with signature `INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()` and selector `[41, 38, 201, 113]`"]
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
        name = "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET",
        abi = "INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET()"
    )]
    pub struct InterestRateRebalanceConditionsNotMetCall;
    #[doc = "Container type for all input parameters for the `INVALID_ADDRESSES_PROVIDER`function with signature `INVALID_ADDRESSES_PROVIDER()` and selector `[55, 147, 7, 130]`"]
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
        name = "INVALID_ADDRESSES_PROVIDER",
        abi = "INVALID_ADDRESSES_PROVIDER()"
    )]
    pub struct InvalidAddressesProviderCall;
    #[doc = "Container type for all input parameters for the `INVALID_ADDRESSES_PROVIDER_ID`function with signature `INVALID_ADDRESSES_PROVIDER_ID()` and selector `[96, 195, 222, 128]`"]
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
        name = "INVALID_ADDRESSES_PROVIDER_ID",
        abi = "INVALID_ADDRESSES_PROVIDER_ID()"
    )]
    pub struct InvalidAddressesProviderIdCall;
    #[doc = "Container type for all input parameters for the `INVALID_AMOUNT`function with signature `INVALID_AMOUNT()` and selector `[250, 232, 39, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_AMOUNT", abi = "INVALID_AMOUNT()")]
    pub struct InvalidAmountCall;
    #[doc = "Container type for all input parameters for the `INVALID_BORROW_CAP`function with signature `INVALID_BORROW_CAP()` and selector `[214, 249, 252, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_BORROW_CAP", abi = "INVALID_BORROW_CAP()")]
    pub struct InvalidBorrowCapCall;
    #[doc = "Container type for all input parameters for the `INVALID_BURN_AMOUNT`function with signature `INVALID_BURN_AMOUNT()` and selector `[81, 38, 116, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_BURN_AMOUNT", abi = "INVALID_BURN_AMOUNT()")]
    pub struct InvalidBurnAmountCall;
    #[doc = "Container type for all input parameters for the `INVALID_DEBT_CEILING`function with signature `INVALID_DEBT_CEILING()` and selector `[220, 197, 109, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_DEBT_CEILING", abi = "INVALID_DEBT_CEILING()")]
    pub struct InvalidDebtCeilingCall;
    #[doc = "Container type for all input parameters for the `INVALID_DECIMALS`function with signature `INVALID_DECIMALS()` and selector `[250, 22, 58, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_DECIMALS", abi = "INVALID_DECIMALS()")]
    pub struct InvalidDecimalsCall;
    #[doc = "Container type for all input parameters for the `INVALID_EMODE_CATEGORY`function with signature `INVALID_EMODE_CATEGORY()` and selector `[168, 201, 120, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_EMODE_CATEGORY", abi = "INVALID_EMODE_CATEGORY()")]
    pub struct InvalidEmodeCategoryCall;
    #[doc = "Container type for all input parameters for the `INVALID_EMODE_CATEGORY_ASSIGNMENT`function with signature `INVALID_EMODE_CATEGORY_ASSIGNMENT()` and selector `[93, 156, 118, 192]`"]
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
        name = "INVALID_EMODE_CATEGORY_ASSIGNMENT",
        abi = "INVALID_EMODE_CATEGORY_ASSIGNMENT()"
    )]
    pub struct InvalidEmodeCategoryAssignmentCall;
    #[doc = "Container type for all input parameters for the `INVALID_EMODE_CATEGORY_PARAMS`function with signature `INVALID_EMODE_CATEGORY_PARAMS()` and selector `[71, 207, 21, 35]`"]
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
        name = "INVALID_EMODE_CATEGORY_PARAMS",
        abi = "INVALID_EMODE_CATEGORY_PARAMS()"
    )]
    pub struct InvalidEmodeCategoryParamsCall;
    #[doc = "Container type for all input parameters for the `INVALID_EXPIRATION`function with signature `INVALID_EXPIRATION()` and selector `[192, 138, 17, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_EXPIRATION", abi = "INVALID_EXPIRATION()")]
    pub struct InvalidExpirationCall;
    #[doc = "Container type for all input parameters for the `INVALID_FLASHLOAN_EXECUTOR_RETURN`function with signature `INVALID_FLASHLOAN_EXECUTOR_RETURN()` and selector `[127, 234, 111, 54]`"]
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
        name = "INVALID_FLASHLOAN_EXECUTOR_RETURN",
        abi = "INVALID_FLASHLOAN_EXECUTOR_RETURN()"
    )]
    pub struct InvalidFlashloanExecutorReturnCall;
    #[doc = "Container type for all input parameters for the `INVALID_INTEREST_RATE_MODE_SELECTED`function with signature `INVALID_INTEREST_RATE_MODE_SELECTED()` and selector `[137, 197, 212, 95]`"]
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
        name = "INVALID_INTEREST_RATE_MODE_SELECTED",
        abi = "INVALID_INTEREST_RATE_MODE_SELECTED()"
    )]
    pub struct InvalidInterestRateModeSelectedCall;
    #[doc = "Container type for all input parameters for the `INVALID_LIQUIDATION_PROTOCOL_FEE`function with signature `INVALID_LIQUIDATION_PROTOCOL_FEE()` and selector `[142, 218, 70, 189]`"]
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
        name = "INVALID_LIQUIDATION_PROTOCOL_FEE",
        abi = "INVALID_LIQUIDATION_PROTOCOL_FEE()"
    )]
    pub struct InvalidLiquidationProtocolFeeCall;
    #[doc = "Container type for all input parameters for the `INVALID_LIQ_BONUS`function with signature `INVALID_LIQ_BONUS()` and selector `[149, 39, 233, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_LIQ_BONUS", abi = "INVALID_LIQ_BONUS()")]
    pub struct InvalidLiqBonusCall;
    #[doc = "Container type for all input parameters for the `INVALID_LIQ_THRESHOLD`function with signature `INVALID_LIQ_THRESHOLD()` and selector `[221, 29, 217, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_LIQ_THRESHOLD", abi = "INVALID_LIQ_THRESHOLD()")]
    pub struct InvalidLiqThresholdCall;
    #[doc = "Container type for all input parameters for the `INVALID_LTV`function with signature `INVALID_LTV()` and selector `[153, 206, 83, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_LTV", abi = "INVALID_LTV()")]
    pub struct InvalidLtvCall;
    #[doc = "Container type for all input parameters for the `INVALID_MINT_AMOUNT`function with signature `INVALID_MINT_AMOUNT()` and selector `[171, 211, 81, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_MINT_AMOUNT", abi = "INVALID_MINT_AMOUNT()")]
    pub struct InvalidMintAmountCall;
    #[doc = "Container type for all input parameters for the `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO`function with signature `INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `[200, 153, 48, 26]`"]
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
        name = "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "INVALID_OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct InvalidOptimalStableToTotalDebtRatioCall;
    #[doc = "Container type for all input parameters for the `INVALID_OPTIMAL_USAGE_RATIO`function with signature `INVALID_OPTIMAL_USAGE_RATIO()` and selector `[78, 58, 237, 55]`"]
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
        name = "INVALID_OPTIMAL_USAGE_RATIO",
        abi = "INVALID_OPTIMAL_USAGE_RATIO()"
    )]
    pub struct InvalidOptimalUsageRatioCall;
    #[doc = "Container type for all input parameters for the `INVALID_RESERVE_FACTOR`function with signature `INVALID_RESERVE_FACTOR()` and selector `[164, 134, 141, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_RESERVE_FACTOR", abi = "INVALID_RESERVE_FACTOR()")]
    pub struct InvalidReserveFactorCall;
    #[doc = "Container type for all input parameters for the `INVALID_RESERVE_INDEX`function with signature `INVALID_RESERVE_INDEX()` and selector `[209, 205, 139, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_RESERVE_INDEX", abi = "INVALID_RESERVE_INDEX()")]
    pub struct InvalidReserveIndexCall;
    #[doc = "Container type for all input parameters for the `INVALID_RESERVE_PARAMS`function with signature `INVALID_RESERVE_PARAMS()` and selector `[51, 87, 99, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_RESERVE_PARAMS", abi = "INVALID_RESERVE_PARAMS()")]
    pub struct InvalidReserveParamsCall;
    #[doc = "Container type for all input parameters for the `INVALID_SIGNATURE`function with signature `INVALID_SIGNATURE()` and selector `[163, 64, 42, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_SIGNATURE", abi = "INVALID_SIGNATURE()")]
    pub struct InvalidSignatureCall;
    #[doc = "Container type for all input parameters for the `INVALID_SUPPLY_CAP`function with signature `INVALID_SUPPLY_CAP()` and selector `[38, 187, 208, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INVALID_SUPPLY_CAP", abi = "INVALID_SUPPLY_CAP()")]
    pub struct InvalidSupplyCapCall;
    #[doc = "Container type for all input parameters for the `INVALID_UNBACKED_MINT_CAP`function with signature `INVALID_UNBACKED_MINT_CAP()` and selector `[71, 186, 147, 216]`"]
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
        name = "INVALID_UNBACKED_MINT_CAP",
        abi = "INVALID_UNBACKED_MINT_CAP()"
    )]
    pub struct InvalidUnbackedMintCapCall;
    #[doc = "Container type for all input parameters for the `LTV_VALIDATION_FAILED`function with signature `LTV_VALIDATION_FAILED()` and selector `[184, 112, 65, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "LTV_VALIDATION_FAILED", abi = "LTV_VALIDATION_FAILED()")]
    pub struct LtvValidationFailedCall;
    #[doc = "Container type for all input parameters for the `NOT_CONTRACT`function with signature `NOT_CONTRACT()` and selector `[17, 215, 176, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "NOT_CONTRACT", abi = "NOT_CONTRACT()")]
    pub struct NotContractCall;
    #[doc = "Container type for all input parameters for the `NOT_ENOUGH_AVAILABLE_USER_BALANCE`function with signature `NOT_ENOUGH_AVAILABLE_USER_BALANCE()` and selector `[183, 245, 226, 36]`"]
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
        name = "NOT_ENOUGH_AVAILABLE_USER_BALANCE",
        abi = "NOT_ENOUGH_AVAILABLE_USER_BALANCE()"
    )]
    pub struct NotEnoughAvailableUserBalanceCall;
    #[doc = "Container type for all input parameters for the `NO_DEBT_OF_SELECTED_TYPE`function with signature `NO_DEBT_OF_SELECTED_TYPE()` and selector `[220, 25, 27, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "NO_DEBT_OF_SELECTED_TYPE", abi = "NO_DEBT_OF_SELECTED_TYPE()")]
    pub struct NoDebtOfSelectedTypeCall;
    #[doc = "Container type for all input parameters for the `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF`function with signature `NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()` and selector `[113, 47, 83, 106]`"]
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
        name = "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF",
        abi = "NO_EXPLICIT_AMOUNT_TO_REPAY_ON_BEHALF()"
    )]
    pub struct NoExplicitAmountToRepayOnBehalfCall;
    #[doc = "Container type for all input parameters for the `NO_MORE_RESERVES_ALLOWED`function with signature `NO_MORE_RESERVES_ALLOWED()` and selector `[118, 174, 143, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "NO_MORE_RESERVES_ALLOWED", abi = "NO_MORE_RESERVES_ALLOWED()")]
    pub struct NoMoreReservesAllowedCall;
    #[doc = "Container type for all input parameters for the `NO_OUTSTANDING_STABLE_DEBT`function with signature `NO_OUTSTANDING_STABLE_DEBT()` and selector `[116, 69, 155, 20]`"]
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
        name = "NO_OUTSTANDING_STABLE_DEBT",
        abi = "NO_OUTSTANDING_STABLE_DEBT()"
    )]
    pub struct NoOutstandingStableDebtCall;
    #[doc = "Container type for all input parameters for the `NO_OUTSTANDING_VARIABLE_DEBT`function with signature `NO_OUTSTANDING_VARIABLE_DEBT()` and selector `[180, 164, 87, 48]`"]
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
        name = "NO_OUTSTANDING_VARIABLE_DEBT",
        abi = "NO_OUTSTANDING_VARIABLE_DEBT()"
    )]
    pub struct NoOutstandingVariableDebtCall;
    #[doc = "Container type for all input parameters for the `OPERATION_NOT_SUPPORTED`function with signature `OPERATION_NOT_SUPPORTED()` and selector `[139, 139, 152, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "OPERATION_NOT_SUPPORTED", abi = "OPERATION_NOT_SUPPORTED()")]
    pub struct OperationNotSupportedCall;
    #[doc = "Container type for all input parameters for the `POOL_ADDRESSES_DO_NOT_MATCH`function with signature `POOL_ADDRESSES_DO_NOT_MATCH()` and selector `[26, 187, 176, 1]`"]
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
        name = "POOL_ADDRESSES_DO_NOT_MATCH",
        abi = "POOL_ADDRESSES_DO_NOT_MATCH()"
    )]
    pub struct PoolAddressesDoNotMatchCall;
    #[doc = "Container type for all input parameters for the `PRICE_ORACLE_SENTINEL_CHECK_FAILED`function with signature `PRICE_ORACLE_SENTINEL_CHECK_FAILED()` and selector `[200, 99, 128, 130]`"]
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
        name = "PRICE_ORACLE_SENTINEL_CHECK_FAILED",
        abi = "PRICE_ORACLE_SENTINEL_CHECK_FAILED()"
    )]
    pub struct PriceOracleSentinelCheckFailedCall;
    #[doc = "Container type for all input parameters for the `RESERVE_ALREADY_ADDED`function with signature `RESERVE_ALREADY_ADDED()` and selector `[18, 220, 173, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_ALREADY_ADDED", abi = "RESERVE_ALREADY_ADDED()")]
    pub struct ReserveAlreadyAddedCall;
    #[doc = "Container type for all input parameters for the `RESERVE_ALREADY_INITIALIZED`function with signature `RESERVE_ALREADY_INITIALIZED()` and selector `[217, 173, 218, 133]`"]
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
        name = "RESERVE_ALREADY_INITIALIZED",
        abi = "RESERVE_ALREADY_INITIALIZED()"
    )]
    pub struct ReserveAlreadyInitializedCall;
    #[doc = "Container type for all input parameters for the `RESERVE_DEBT_NOT_ZERO`function with signature `RESERVE_DEBT_NOT_ZERO()` and selector `[233, 129, 72, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_DEBT_NOT_ZERO", abi = "RESERVE_DEBT_NOT_ZERO()")]
    pub struct ReserveDebtNotZeroCall;
    #[doc = "Container type for all input parameters for the `RESERVE_FROZEN`function with signature `RESERVE_FROZEN()` and selector `[108, 211, 207, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_FROZEN", abi = "RESERVE_FROZEN()")]
    pub struct ReserveFrozenCall;
    #[doc = "Container type for all input parameters for the `RESERVE_INACTIVE`function with signature `RESERVE_INACTIVE()` and selector `[82, 186, 157, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_INACTIVE", abi = "RESERVE_INACTIVE()")]
    pub struct ReserveInactiveCall;
    #[doc = "Container type for all input parameters for the `RESERVE_LIQUIDITY_NOT_ZERO`function with signature `RESERVE_LIQUIDITY_NOT_ZERO()` and selector `[8, 77, 250, 13]`"]
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
        name = "RESERVE_LIQUIDITY_NOT_ZERO",
        abi = "RESERVE_LIQUIDITY_NOT_ZERO()"
    )]
    pub struct ReserveLiquidityNotZeroCall;
    #[doc = "Container type for all input parameters for the `RESERVE_PAUSED`function with signature `RESERVE_PAUSED()` and selector `[182, 135, 116, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RESERVE_PAUSED", abi = "RESERVE_PAUSED()")]
    pub struct ReservePausedCall;
    #[doc = "Container type for all input parameters for the `SAME_BLOCK_BORROW_REPAY`function with signature `SAME_BLOCK_BORROW_REPAY()` and selector `[87, 14, 53, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "SAME_BLOCK_BORROW_REPAY", abi = "SAME_BLOCK_BORROW_REPAY()")]
    pub struct SameBlockBorrowRepayCall;
    #[doc = "Container type for all input parameters for the `SILOED_BORROWING_VIOLATION`function with signature `SILOED_BORROWING_VIOLATION()` and selector `[222, 36, 148, 140]`"]
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
        name = "SILOED_BORROWING_VIOLATION",
        abi = "SILOED_BORROWING_VIOLATION()"
    )]
    pub struct SiloedBorrowingViolationCall;
    #[doc = "Container type for all input parameters for the `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER`function with signature `SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()` and selector `[34, 167, 52, 70]`"]
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
        name = "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER",
        abi = "SPECIFIED_CURRENCY_NOT_BORROWED_BY_USER()"
    )]
    pub struct SpecifiedCurrencyNotBorrowedByUserCall;
    #[doc = "Container type for all input parameters for the `STABLE_BORROWING_ENABLED`function with signature `STABLE_BORROWING_ENABLED()` and selector `[25, 141, 106, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "STABLE_BORROWING_ENABLED", abi = "STABLE_BORROWING_ENABLED()")]
    pub struct StableBorrowingEnabledCall;
    #[doc = "Container type for all input parameters for the `STABLE_BORROWING_NOT_ENABLED`function with signature `STABLE_BORROWING_NOT_ENABLED()` and selector `[77, 134, 243, 147]`"]
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
        name = "STABLE_BORROWING_NOT_ENABLED",
        abi = "STABLE_BORROWING_NOT_ENABLED()"
    )]
    pub struct StableBorrowingNotEnabledCall;
    #[doc = "Container type for all input parameters for the `STABLE_DEBT_NOT_ZERO`function with signature `STABLE_DEBT_NOT_ZERO()` and selector `[101, 231, 239, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "STABLE_DEBT_NOT_ZERO", abi = "STABLE_DEBT_NOT_ZERO()")]
    pub struct StableDebtNotZeroCall;
    #[doc = "Container type for all input parameters for the `SUPPLY_CAP_EXCEEDED`function with signature `SUPPLY_CAP_EXCEEDED()` and selector `[176, 81, 0, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "SUPPLY_CAP_EXCEEDED", abi = "SUPPLY_CAP_EXCEEDED()")]
    pub struct SupplyCapExceededCall;
    #[doc = "Container type for all input parameters for the `UNBACKED_MINT_CAP_EXCEEDED`function with signature `UNBACKED_MINT_CAP_EXCEEDED()` and selector `[107, 63, 124, 199]`"]
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
        name = "UNBACKED_MINT_CAP_EXCEEDED",
        abi = "UNBACKED_MINT_CAP_EXCEEDED()"
    )]
    pub struct UnbackedMintCapExceededCall;
    #[doc = "Container type for all input parameters for the `UNDERLYING_BALANCE_ZERO`function with signature `UNDERLYING_BALANCE_ZERO()` and selector `[162, 121, 124, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "UNDERLYING_BALANCE_ZERO", abi = "UNDERLYING_BALANCE_ZERO()")]
    pub struct UnderlyingBalanceZeroCall;
    #[doc = "Container type for all input parameters for the `UNDERLYING_CANNOT_BE_RESCUED`function with signature `UNDERLYING_CANNOT_BE_RESCUED()` and selector `[171, 136, 60, 160]`"]
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
        name = "UNDERLYING_CANNOT_BE_RESCUED",
        abi = "UNDERLYING_CANNOT_BE_RESCUED()"
    )]
    pub struct UnderlyingCannotBeRescuedCall;
    #[doc = "Container type for all input parameters for the `USER_IN_ISOLATION_MODE`function with signature `USER_IN_ISOLATION_MODE()` and selector `[116, 68, 101, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "USER_IN_ISOLATION_MODE", abi = "USER_IN_ISOLATION_MODE()")]
    pub struct UserInIsolationModeCall;
    #[doc = "Container type for all input parameters for the `VARIABLE_DEBT_SUPPLY_NOT_ZERO`function with signature `VARIABLE_DEBT_SUPPLY_NOT_ZERO()` and selector `[241, 7, 39, 219]`"]
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
        name = "VARIABLE_DEBT_SUPPLY_NOT_ZERO",
        abi = "VARIABLE_DEBT_SUPPLY_NOT_ZERO()"
    )]
    pub struct VariableDebtSupplyNotZeroCall;
    #[doc = "Container type for all input parameters for the `ZERO_ADDRESS_NOT_VALID`function with signature `ZERO_ADDRESS_NOT_VALID()` and selector `[209, 75, 177, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ZERO_ADDRESS_NOT_VALID", abi = "ZERO_ADDRESS_NOT_VALID()")]
    pub struct ZeroAddressNotValidCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ErrorsCalls {
        AclAdminCannotBeZero(AclAdminCannotBeZeroCall),
        AddressesProviderAlreadyAdded(AddressesProviderAlreadyAddedCall),
        AddressesProviderNotRegistered(AddressesProviderNotRegisteredCall),
        AmountBiggerThanMaxLoanSizeStable(AmountBiggerThanMaxLoanSizeStableCall),
        AssetNotBorrowableInIsolation(AssetNotBorrowableInIsolationCall),
        AssetNotListed(AssetNotListedCall),
        AtokenSupplyNotZero(AtokenSupplyNotZeroCall),
        BorrowingNotEnabled(BorrowingNotEnabledCall),
        BorrowCapExceeded(BorrowCapExceededCall),
        BridgeProtocolFeeInvalid(BridgeProtocolFeeInvalidCall),
        CallerMustBePool(CallerMustBePoolCall),
        CallerNotAssetListingOrPoolAdmin(CallerNotAssetListingOrPoolAdminCall),
        CallerNotAtoken(CallerNotAtokenCall),
        CallerNotBridge(CallerNotBridgeCall),
        CallerNotEmergencyAdmin(CallerNotEmergencyAdminCall),
        CallerNotPoolAdmin(CallerNotPoolAdminCall),
        CallerNotPoolConfigurator(CallerNotPoolConfiguratorCall),
        CallerNotPoolOrEmergencyAdmin(CallerNotPoolOrEmergencyAdminCall),
        CallerNotRiskOrPoolAdmin(CallerNotRiskOrPoolAdminCall),
        CollateralBalanceIsZero(CollateralBalanceIsZeroCall),
        CollateralCannotBeLiquidated(CollateralCannotBeLiquidatedCall),
        CollateralCannotCoverNewBorrow(CollateralCannotCoverNewBorrowCall),
        CollateralSameAsBorrowingCurrency(CollateralSameAsBorrowingCurrencyCall),
        DebtCeilingExceeded(DebtCeilingExceededCall),
        DebtCeilingNotZero(DebtCeilingNotZeroCall),
        EmodeCategoryReserved(EmodeCategoryReservedCall),
        FlashloanPremiumInvalid(FlashloanPremiumInvalidCall),
        HealthFactorLowerThanLiquidationThreshold(HealthFactorLowerThanLiquidationThresholdCall),
        HealthFactorNotBelowThreshold(HealthFactorNotBelowThresholdCall),
        InconsistentEmodeCategory(InconsistentEmodeCategoryCall),
        InconsistentFlashloanParams(InconsistentFlashloanParamsCall),
        InconsistentParamsLength(InconsistentParamsLengthCall),
        InterestRateRebalanceConditionsNotMet(InterestRateRebalanceConditionsNotMetCall),
        InvalidAddressesProvider(InvalidAddressesProviderCall),
        InvalidAddressesProviderId(InvalidAddressesProviderIdCall),
        InvalidAmount(InvalidAmountCall),
        InvalidBorrowCap(InvalidBorrowCapCall),
        InvalidBurnAmount(InvalidBurnAmountCall),
        InvalidDebtCeiling(InvalidDebtCeilingCall),
        InvalidDecimals(InvalidDecimalsCall),
        InvalidEmodeCategory(InvalidEmodeCategoryCall),
        InvalidEmodeCategoryAssignment(InvalidEmodeCategoryAssignmentCall),
        InvalidEmodeCategoryParams(InvalidEmodeCategoryParamsCall),
        InvalidExpiration(InvalidExpirationCall),
        InvalidFlashloanExecutorReturn(InvalidFlashloanExecutorReturnCall),
        InvalidInterestRateModeSelected(InvalidInterestRateModeSelectedCall),
        InvalidLiquidationProtocolFee(InvalidLiquidationProtocolFeeCall),
        InvalidLiqBonus(InvalidLiqBonusCall),
        InvalidLiqThreshold(InvalidLiqThresholdCall),
        InvalidLtv(InvalidLtvCall),
        InvalidMintAmount(InvalidMintAmountCall),
        InvalidOptimalStableToTotalDebtRatio(InvalidOptimalStableToTotalDebtRatioCall),
        InvalidOptimalUsageRatio(InvalidOptimalUsageRatioCall),
        InvalidReserveFactor(InvalidReserveFactorCall),
        InvalidReserveIndex(InvalidReserveIndexCall),
        InvalidReserveParams(InvalidReserveParamsCall),
        InvalidSignature(InvalidSignatureCall),
        InvalidSupplyCap(InvalidSupplyCapCall),
        InvalidUnbackedMintCap(InvalidUnbackedMintCapCall),
        LtvValidationFailed(LtvValidationFailedCall),
        NotContract(NotContractCall),
        NotEnoughAvailableUserBalance(NotEnoughAvailableUserBalanceCall),
        NoDebtOfSelectedType(NoDebtOfSelectedTypeCall),
        NoExplicitAmountToRepayOnBehalf(NoExplicitAmountToRepayOnBehalfCall),
        NoMoreReservesAllowed(NoMoreReservesAllowedCall),
        NoOutstandingStableDebt(NoOutstandingStableDebtCall),
        NoOutstandingVariableDebt(NoOutstandingVariableDebtCall),
        OperationNotSupported(OperationNotSupportedCall),
        PoolAddressesDoNotMatch(PoolAddressesDoNotMatchCall),
        PriceOracleSentinelCheckFailed(PriceOracleSentinelCheckFailedCall),
        ReserveAlreadyAdded(ReserveAlreadyAddedCall),
        ReserveAlreadyInitialized(ReserveAlreadyInitializedCall),
        ReserveDebtNotZero(ReserveDebtNotZeroCall),
        ReserveFrozen(ReserveFrozenCall),
        ReserveInactive(ReserveInactiveCall),
        ReserveLiquidityNotZero(ReserveLiquidityNotZeroCall),
        ReservePaused(ReservePausedCall),
        SameBlockBorrowRepay(SameBlockBorrowRepayCall),
        SiloedBorrowingViolation(SiloedBorrowingViolationCall),
        SpecifiedCurrencyNotBorrowedByUser(SpecifiedCurrencyNotBorrowedByUserCall),
        StableBorrowingEnabled(StableBorrowingEnabledCall),
        StableBorrowingNotEnabled(StableBorrowingNotEnabledCall),
        StableDebtNotZero(StableDebtNotZeroCall),
        SupplyCapExceeded(SupplyCapExceededCall),
        UnbackedMintCapExceeded(UnbackedMintCapExceededCall),
        UnderlyingBalanceZero(UnderlyingBalanceZeroCall),
        UnderlyingCannotBeRescued(UnderlyingCannotBeRescuedCall),
        UserInIsolationMode(UserInIsolationModeCall),
        VariableDebtSupplyNotZero(VariableDebtSupplyNotZeroCall),
        ZeroAddressNotValid(ZeroAddressNotValidCall),
    }
    impl ethers::core::abi::AbiDecode for ErrorsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AclAdminCannotBeZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::AclAdminCannotBeZero(decoded));
            }
            if let Ok(decoded) =
                <AddressesProviderAlreadyAddedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AddressesProviderAlreadyAdded(decoded));
            }
            if let Ok(decoded) =
                <AddressesProviderNotRegisteredCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AddressesProviderNotRegistered(decoded));
            }
            if let Ok(decoded) =
                <AmountBiggerThanMaxLoanSizeStableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AmountBiggerThanMaxLoanSizeStable(decoded));
            }
            if let Ok(decoded) =
                <AssetNotBorrowableInIsolationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AssetNotBorrowableInIsolation(decoded));
            }
            if let Ok(decoded) =
                <AssetNotListedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::AssetNotListed(decoded));
            }
            if let Ok(decoded) =
                <AtokenSupplyNotZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::AtokenSupplyNotZero(decoded));
            }
            if let Ok(decoded) =
                <BorrowingNotEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::BorrowingNotEnabled(decoded));
            }
            if let Ok(decoded) =
                <BorrowCapExceededCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::BorrowCapExceeded(decoded));
            }
            if let Ok(decoded) =
                <BridgeProtocolFeeInvalidCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::BridgeProtocolFeeInvalid(decoded));
            }
            if let Ok(decoded) =
                <CallerMustBePoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CallerMustBePool(decoded));
            }
            if let Ok(decoded) =
                <CallerNotAssetListingOrPoolAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CallerNotAssetListingOrPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <CallerNotAtokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CallerNotAtoken(decoded));
            }
            if let Ok(decoded) =
                <CallerNotBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CallerNotBridge(decoded));
            }
            if let Ok(decoded) =
                <CallerNotEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CallerNotEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <CallerNotPoolAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CallerNotPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <CallerNotPoolConfiguratorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CallerNotPoolConfigurator(decoded));
            }
            if let Ok(decoded) =
                <CallerNotPoolOrEmergencyAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CallerNotPoolOrEmergencyAdmin(decoded));
            }
            if let Ok(decoded) =
                <CallerNotRiskOrPoolAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CallerNotRiskOrPoolAdmin(decoded));
            }
            if let Ok(decoded) =
                <CollateralBalanceIsZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CollateralBalanceIsZero(decoded));
            }
            if let Ok(decoded) =
                <CollateralCannotBeLiquidatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CollateralCannotBeLiquidated(decoded));
            }
            if let Ok(decoded) =
                <CollateralCannotCoverNewBorrowCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CollateralCannotCoverNewBorrow(decoded));
            }
            if let Ok(decoded) =
                <CollateralSameAsBorrowingCurrencyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CollateralSameAsBorrowingCurrency(decoded));
            }
            if let Ok(decoded) =
                <DebtCeilingExceededCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::DebtCeilingExceeded(decoded));
            }
            if let Ok(decoded) =
                <DebtCeilingNotZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::DebtCeilingNotZero(decoded));
            }
            if let Ok(decoded) =
                <EmodeCategoryReservedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::EmodeCategoryReserved(decoded));
            }
            if let Ok(decoded) =
                <FlashloanPremiumInvalidCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::FlashloanPremiumInvalid(decoded));
            }
            if let Ok (decoded) = < HealthFactorLowerThanLiquidationThresholdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ErrorsCalls :: HealthFactorLowerThanLiquidationThreshold (decoded)) }
            if let Ok(decoded) =
                <HealthFactorNotBelowThresholdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::HealthFactorNotBelowThreshold(decoded));
            }
            if let Ok(decoded) =
                <InconsistentEmodeCategoryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InconsistentEmodeCategory(decoded));
            }
            if let Ok(decoded) =
                <InconsistentFlashloanParamsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InconsistentFlashloanParams(decoded));
            }
            if let Ok(decoded) =
                <InconsistentParamsLengthCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InconsistentParamsLength(decoded));
            }
            if let Ok(decoded) =
                <InterestRateRebalanceConditionsNotMetCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InterestRateRebalanceConditionsNotMet(decoded));
            }
            if let Ok(decoded) =
                <InvalidAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidAddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <InvalidAddressesProviderIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidAddressesProviderId(decoded));
            }
            if let Ok(decoded) =
                <InvalidAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidAmount(decoded));
            }
            if let Ok(decoded) =
                <InvalidBorrowCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidBorrowCap(decoded));
            }
            if let Ok(decoded) =
                <InvalidBurnAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidBurnAmount(decoded));
            }
            if let Ok(decoded) =
                <InvalidDebtCeilingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidDebtCeiling(decoded));
            }
            if let Ok(decoded) =
                <InvalidDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidDecimals(decoded));
            }
            if let Ok(decoded) =
                <InvalidEmodeCategoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidEmodeCategory(decoded));
            }
            if let Ok(decoded) =
                <InvalidEmodeCategoryAssignmentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidEmodeCategoryAssignment(decoded));
            }
            if let Ok(decoded) =
                <InvalidEmodeCategoryParamsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidEmodeCategoryParams(decoded));
            }
            if let Ok(decoded) =
                <InvalidExpirationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidExpiration(decoded));
            }
            if let Ok(decoded) =
                <InvalidFlashloanExecutorReturnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidFlashloanExecutorReturn(decoded));
            }
            if let Ok(decoded) =
                <InvalidInterestRateModeSelectedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidInterestRateModeSelected(decoded));
            }
            if let Ok(decoded) =
                <InvalidLiquidationProtocolFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <InvalidLiqBonusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidLiqBonus(decoded));
            }
            if let Ok(decoded) =
                <InvalidLiqThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidLiqThreshold(decoded));
            }
            if let Ok(decoded) =
                <InvalidLtvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidLtv(decoded));
            }
            if let Ok(decoded) =
                <InvalidMintAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidMintAmount(decoded));
            }
            if let Ok(decoded) =
                <InvalidOptimalStableToTotalDebtRatioCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidOptimalStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded) =
                <InvalidOptimalUsageRatioCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::InvalidOptimalUsageRatio(decoded));
            }
            if let Ok(decoded) =
                <InvalidReserveFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidReserveFactor(decoded));
            }
            if let Ok(decoded) =
                <InvalidReserveIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidReserveIndex(decoded));
            }
            if let Ok(decoded) =
                <InvalidReserveParamsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidReserveParams(decoded));
            }
            if let Ok(decoded) =
                <InvalidSignatureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidSignature(decoded));
            }
            if let Ok(decoded) =
                <InvalidSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <InvalidUnbackedMintCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::InvalidUnbackedMintCap(decoded));
            }
            if let Ok(decoded) =
                <LtvValidationFailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::LtvValidationFailed(decoded));
            }
            if let Ok(decoded) =
                <NotContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::NotContract(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughAvailableUserBalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::NotEnoughAvailableUserBalance(decoded));
            }
            if let Ok(decoded) =
                <NoDebtOfSelectedTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::NoDebtOfSelectedType(decoded));
            }
            if let Ok(decoded) =
                <NoExplicitAmountToRepayOnBehalfCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::NoExplicitAmountToRepayOnBehalf(decoded));
            }
            if let Ok(decoded) =
                <NoMoreReservesAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::NoMoreReservesAllowed(decoded));
            }
            if let Ok(decoded) =
                <NoOutstandingStableDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::NoOutstandingStableDebt(decoded));
            }
            if let Ok(decoded) =
                <NoOutstandingVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::NoOutstandingVariableDebt(decoded));
            }
            if let Ok(decoded) =
                <OperationNotSupportedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::OperationNotSupported(decoded));
            }
            if let Ok(decoded) =
                <PoolAddressesDoNotMatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::PoolAddressesDoNotMatch(decoded));
            }
            if let Ok(decoded) =
                <PriceOracleSentinelCheckFailedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PriceOracleSentinelCheckFailed(decoded));
            }
            if let Ok(decoded) =
                <ReserveAlreadyAddedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReserveAlreadyAdded(decoded));
            }
            if let Ok(decoded) =
                <ReserveAlreadyInitializedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::ReserveAlreadyInitialized(decoded));
            }
            if let Ok(decoded) =
                <ReserveDebtNotZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReserveDebtNotZero(decoded));
            }
            if let Ok(decoded) =
                <ReserveFrozenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReserveFrozen(decoded));
            }
            if let Ok(decoded) =
                <ReserveInactiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReserveInactive(decoded));
            }
            if let Ok(decoded) =
                <ReserveLiquidityNotZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReserveLiquidityNotZero(decoded));
            }
            if let Ok(decoded) =
                <ReservePausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ReservePaused(decoded));
            }
            if let Ok(decoded) =
                <SameBlockBorrowRepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::SameBlockBorrowRepay(decoded));
            }
            if let Ok(decoded) =
                <SiloedBorrowingViolationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::SiloedBorrowingViolation(decoded));
            }
            if let Ok(decoded) =
                <SpecifiedCurrencyNotBorrowedByUserCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::SpecifiedCurrencyNotBorrowedByUser(decoded));
            }
            if let Ok(decoded) =
                <StableBorrowingEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::StableBorrowingEnabled(decoded));
            }
            if let Ok(decoded) =
                <StableBorrowingNotEnabledCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::StableBorrowingNotEnabled(decoded));
            }
            if let Ok(decoded) =
                <StableDebtNotZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::StableDebtNotZero(decoded));
            }
            if let Ok(decoded) =
                <SupplyCapExceededCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::SupplyCapExceeded(decoded));
            }
            if let Ok(decoded) =
                <UnbackedMintCapExceededCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::UnbackedMintCapExceeded(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingBalanceZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::UnderlyingBalanceZero(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCannotBeRescuedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::UnderlyingCannotBeRescued(decoded));
            }
            if let Ok(decoded) =
                <UserInIsolationModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::UserInIsolationMode(decoded));
            }
            if let Ok(decoded) =
                <VariableDebtSupplyNotZeroCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::VariableDebtSupplyNotZero(decoded));
            }
            if let Ok(decoded) =
                <ZeroAddressNotValidCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ZeroAddressNotValid(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ErrorsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ErrorsCalls::AclAdminCannotBeZero(element) => element.encode(),
                ErrorsCalls::AddressesProviderAlreadyAdded(element) => element.encode(),
                ErrorsCalls::AddressesProviderNotRegistered(element) => element.encode(),
                ErrorsCalls::AmountBiggerThanMaxLoanSizeStable(element) => element.encode(),
                ErrorsCalls::AssetNotBorrowableInIsolation(element) => element.encode(),
                ErrorsCalls::AssetNotListed(element) => element.encode(),
                ErrorsCalls::AtokenSupplyNotZero(element) => element.encode(),
                ErrorsCalls::BorrowingNotEnabled(element) => element.encode(),
                ErrorsCalls::BorrowCapExceeded(element) => element.encode(),
                ErrorsCalls::BridgeProtocolFeeInvalid(element) => element.encode(),
                ErrorsCalls::CallerMustBePool(element) => element.encode(),
                ErrorsCalls::CallerNotAssetListingOrPoolAdmin(element) => element.encode(),
                ErrorsCalls::CallerNotAtoken(element) => element.encode(),
                ErrorsCalls::CallerNotBridge(element) => element.encode(),
                ErrorsCalls::CallerNotEmergencyAdmin(element) => element.encode(),
                ErrorsCalls::CallerNotPoolAdmin(element) => element.encode(),
                ErrorsCalls::CallerNotPoolConfigurator(element) => element.encode(),
                ErrorsCalls::CallerNotPoolOrEmergencyAdmin(element) => element.encode(),
                ErrorsCalls::CallerNotRiskOrPoolAdmin(element) => element.encode(),
                ErrorsCalls::CollateralBalanceIsZero(element) => element.encode(),
                ErrorsCalls::CollateralCannotBeLiquidated(element) => element.encode(),
                ErrorsCalls::CollateralCannotCoverNewBorrow(element) => element.encode(),
                ErrorsCalls::CollateralSameAsBorrowingCurrency(element) => element.encode(),
                ErrorsCalls::DebtCeilingExceeded(element) => element.encode(),
                ErrorsCalls::DebtCeilingNotZero(element) => element.encode(),
                ErrorsCalls::EmodeCategoryReserved(element) => element.encode(),
                ErrorsCalls::FlashloanPremiumInvalid(element) => element.encode(),
                ErrorsCalls::HealthFactorLowerThanLiquidationThreshold(element) => element.encode(),
                ErrorsCalls::HealthFactorNotBelowThreshold(element) => element.encode(),
                ErrorsCalls::InconsistentEmodeCategory(element) => element.encode(),
                ErrorsCalls::InconsistentFlashloanParams(element) => element.encode(),
                ErrorsCalls::InconsistentParamsLength(element) => element.encode(),
                ErrorsCalls::InterestRateRebalanceConditionsNotMet(element) => element.encode(),
                ErrorsCalls::InvalidAddressesProvider(element) => element.encode(),
                ErrorsCalls::InvalidAddressesProviderId(element) => element.encode(),
                ErrorsCalls::InvalidAmount(element) => element.encode(),
                ErrorsCalls::InvalidBorrowCap(element) => element.encode(),
                ErrorsCalls::InvalidBurnAmount(element) => element.encode(),
                ErrorsCalls::InvalidDebtCeiling(element) => element.encode(),
                ErrorsCalls::InvalidDecimals(element) => element.encode(),
                ErrorsCalls::InvalidEmodeCategory(element) => element.encode(),
                ErrorsCalls::InvalidEmodeCategoryAssignment(element) => element.encode(),
                ErrorsCalls::InvalidEmodeCategoryParams(element) => element.encode(),
                ErrorsCalls::InvalidExpiration(element) => element.encode(),
                ErrorsCalls::InvalidFlashloanExecutorReturn(element) => element.encode(),
                ErrorsCalls::InvalidInterestRateModeSelected(element) => element.encode(),
                ErrorsCalls::InvalidLiquidationProtocolFee(element) => element.encode(),
                ErrorsCalls::InvalidLiqBonus(element) => element.encode(),
                ErrorsCalls::InvalidLiqThreshold(element) => element.encode(),
                ErrorsCalls::InvalidLtv(element) => element.encode(),
                ErrorsCalls::InvalidMintAmount(element) => element.encode(),
                ErrorsCalls::InvalidOptimalStableToTotalDebtRatio(element) => element.encode(),
                ErrorsCalls::InvalidOptimalUsageRatio(element) => element.encode(),
                ErrorsCalls::InvalidReserveFactor(element) => element.encode(),
                ErrorsCalls::InvalidReserveIndex(element) => element.encode(),
                ErrorsCalls::InvalidReserveParams(element) => element.encode(),
                ErrorsCalls::InvalidSignature(element) => element.encode(),
                ErrorsCalls::InvalidSupplyCap(element) => element.encode(),
                ErrorsCalls::InvalidUnbackedMintCap(element) => element.encode(),
                ErrorsCalls::LtvValidationFailed(element) => element.encode(),
                ErrorsCalls::NotContract(element) => element.encode(),
                ErrorsCalls::NotEnoughAvailableUserBalance(element) => element.encode(),
                ErrorsCalls::NoDebtOfSelectedType(element) => element.encode(),
                ErrorsCalls::NoExplicitAmountToRepayOnBehalf(element) => element.encode(),
                ErrorsCalls::NoMoreReservesAllowed(element) => element.encode(),
                ErrorsCalls::NoOutstandingStableDebt(element) => element.encode(),
                ErrorsCalls::NoOutstandingVariableDebt(element) => element.encode(),
                ErrorsCalls::OperationNotSupported(element) => element.encode(),
                ErrorsCalls::PoolAddressesDoNotMatch(element) => element.encode(),
                ErrorsCalls::PriceOracleSentinelCheckFailed(element) => element.encode(),
                ErrorsCalls::ReserveAlreadyAdded(element) => element.encode(),
                ErrorsCalls::ReserveAlreadyInitialized(element) => element.encode(),
                ErrorsCalls::ReserveDebtNotZero(element) => element.encode(),
                ErrorsCalls::ReserveFrozen(element) => element.encode(),
                ErrorsCalls::ReserveInactive(element) => element.encode(),
                ErrorsCalls::ReserveLiquidityNotZero(element) => element.encode(),
                ErrorsCalls::ReservePaused(element) => element.encode(),
                ErrorsCalls::SameBlockBorrowRepay(element) => element.encode(),
                ErrorsCalls::SiloedBorrowingViolation(element) => element.encode(),
                ErrorsCalls::SpecifiedCurrencyNotBorrowedByUser(element) => element.encode(),
                ErrorsCalls::StableBorrowingEnabled(element) => element.encode(),
                ErrorsCalls::StableBorrowingNotEnabled(element) => element.encode(),
                ErrorsCalls::StableDebtNotZero(element) => element.encode(),
                ErrorsCalls::SupplyCapExceeded(element) => element.encode(),
                ErrorsCalls::UnbackedMintCapExceeded(element) => element.encode(),
                ErrorsCalls::UnderlyingBalanceZero(element) => element.encode(),
                ErrorsCalls::UnderlyingCannotBeRescued(element) => element.encode(),
                ErrorsCalls::UserInIsolationMode(element) => element.encode(),
                ErrorsCalls::VariableDebtSupplyNotZero(element) => element.encode(),
                ErrorsCalls::ZeroAddressNotValid(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ErrorsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ErrorsCalls::AclAdminCannotBeZero(element) => element.fmt(f),
                ErrorsCalls::AddressesProviderAlreadyAdded(element) => element.fmt(f),
                ErrorsCalls::AddressesProviderNotRegistered(element) => element.fmt(f),
                ErrorsCalls::AmountBiggerThanMaxLoanSizeStable(element) => element.fmt(f),
                ErrorsCalls::AssetNotBorrowableInIsolation(element) => element.fmt(f),
                ErrorsCalls::AssetNotListed(element) => element.fmt(f),
                ErrorsCalls::AtokenSupplyNotZero(element) => element.fmt(f),
                ErrorsCalls::BorrowingNotEnabled(element) => element.fmt(f),
                ErrorsCalls::BorrowCapExceeded(element) => element.fmt(f),
                ErrorsCalls::BridgeProtocolFeeInvalid(element) => element.fmt(f),
                ErrorsCalls::CallerMustBePool(element) => element.fmt(f),
                ErrorsCalls::CallerNotAssetListingOrPoolAdmin(element) => element.fmt(f),
                ErrorsCalls::CallerNotAtoken(element) => element.fmt(f),
                ErrorsCalls::CallerNotBridge(element) => element.fmt(f),
                ErrorsCalls::CallerNotEmergencyAdmin(element) => element.fmt(f),
                ErrorsCalls::CallerNotPoolAdmin(element) => element.fmt(f),
                ErrorsCalls::CallerNotPoolConfigurator(element) => element.fmt(f),
                ErrorsCalls::CallerNotPoolOrEmergencyAdmin(element) => element.fmt(f),
                ErrorsCalls::CallerNotRiskOrPoolAdmin(element) => element.fmt(f),
                ErrorsCalls::CollateralBalanceIsZero(element) => element.fmt(f),
                ErrorsCalls::CollateralCannotBeLiquidated(element) => element.fmt(f),
                ErrorsCalls::CollateralCannotCoverNewBorrow(element) => element.fmt(f),
                ErrorsCalls::CollateralSameAsBorrowingCurrency(element) => element.fmt(f),
                ErrorsCalls::DebtCeilingExceeded(element) => element.fmt(f),
                ErrorsCalls::DebtCeilingNotZero(element) => element.fmt(f),
                ErrorsCalls::EmodeCategoryReserved(element) => element.fmt(f),
                ErrorsCalls::FlashloanPremiumInvalid(element) => element.fmt(f),
                ErrorsCalls::HealthFactorLowerThanLiquidationThreshold(element) => element.fmt(f),
                ErrorsCalls::HealthFactorNotBelowThreshold(element) => element.fmt(f),
                ErrorsCalls::InconsistentEmodeCategory(element) => element.fmt(f),
                ErrorsCalls::InconsistentFlashloanParams(element) => element.fmt(f),
                ErrorsCalls::InconsistentParamsLength(element) => element.fmt(f),
                ErrorsCalls::InterestRateRebalanceConditionsNotMet(element) => element.fmt(f),
                ErrorsCalls::InvalidAddressesProvider(element) => element.fmt(f),
                ErrorsCalls::InvalidAddressesProviderId(element) => element.fmt(f),
                ErrorsCalls::InvalidAmount(element) => element.fmt(f),
                ErrorsCalls::InvalidBorrowCap(element) => element.fmt(f),
                ErrorsCalls::InvalidBurnAmount(element) => element.fmt(f),
                ErrorsCalls::InvalidDebtCeiling(element) => element.fmt(f),
                ErrorsCalls::InvalidDecimals(element) => element.fmt(f),
                ErrorsCalls::InvalidEmodeCategory(element) => element.fmt(f),
                ErrorsCalls::InvalidEmodeCategoryAssignment(element) => element.fmt(f),
                ErrorsCalls::InvalidEmodeCategoryParams(element) => element.fmt(f),
                ErrorsCalls::InvalidExpiration(element) => element.fmt(f),
                ErrorsCalls::InvalidFlashloanExecutorReturn(element) => element.fmt(f),
                ErrorsCalls::InvalidInterestRateModeSelected(element) => element.fmt(f),
                ErrorsCalls::InvalidLiquidationProtocolFee(element) => element.fmt(f),
                ErrorsCalls::InvalidLiqBonus(element) => element.fmt(f),
                ErrorsCalls::InvalidLiqThreshold(element) => element.fmt(f),
                ErrorsCalls::InvalidLtv(element) => element.fmt(f),
                ErrorsCalls::InvalidMintAmount(element) => element.fmt(f),
                ErrorsCalls::InvalidOptimalStableToTotalDebtRatio(element) => element.fmt(f),
                ErrorsCalls::InvalidOptimalUsageRatio(element) => element.fmt(f),
                ErrorsCalls::InvalidReserveFactor(element) => element.fmt(f),
                ErrorsCalls::InvalidReserveIndex(element) => element.fmt(f),
                ErrorsCalls::InvalidReserveParams(element) => element.fmt(f),
                ErrorsCalls::InvalidSignature(element) => element.fmt(f),
                ErrorsCalls::InvalidSupplyCap(element) => element.fmt(f),
                ErrorsCalls::InvalidUnbackedMintCap(element) => element.fmt(f),
                ErrorsCalls::LtvValidationFailed(element) => element.fmt(f),
                ErrorsCalls::NotContract(element) => element.fmt(f),
                ErrorsCalls::NotEnoughAvailableUserBalance(element) => element.fmt(f),
                ErrorsCalls::NoDebtOfSelectedType(element) => element.fmt(f),
                ErrorsCalls::NoExplicitAmountToRepayOnBehalf(element) => element.fmt(f),
                ErrorsCalls::NoMoreReservesAllowed(element) => element.fmt(f),
                ErrorsCalls::NoOutstandingStableDebt(element) => element.fmt(f),
                ErrorsCalls::NoOutstandingVariableDebt(element) => element.fmt(f),
                ErrorsCalls::OperationNotSupported(element) => element.fmt(f),
                ErrorsCalls::PoolAddressesDoNotMatch(element) => element.fmt(f),
                ErrorsCalls::PriceOracleSentinelCheckFailed(element) => element.fmt(f),
                ErrorsCalls::ReserveAlreadyAdded(element) => element.fmt(f),
                ErrorsCalls::ReserveAlreadyInitialized(element) => element.fmt(f),
                ErrorsCalls::ReserveDebtNotZero(element) => element.fmt(f),
                ErrorsCalls::ReserveFrozen(element) => element.fmt(f),
                ErrorsCalls::ReserveInactive(element) => element.fmt(f),
                ErrorsCalls::ReserveLiquidityNotZero(element) => element.fmt(f),
                ErrorsCalls::ReservePaused(element) => element.fmt(f),
                ErrorsCalls::SameBlockBorrowRepay(element) => element.fmt(f),
                ErrorsCalls::SiloedBorrowingViolation(element) => element.fmt(f),
                ErrorsCalls::SpecifiedCurrencyNotBorrowedByUser(element) => element.fmt(f),
                ErrorsCalls::StableBorrowingEnabled(element) => element.fmt(f),
                ErrorsCalls::StableBorrowingNotEnabled(element) => element.fmt(f),
                ErrorsCalls::StableDebtNotZero(element) => element.fmt(f),
                ErrorsCalls::SupplyCapExceeded(element) => element.fmt(f),
                ErrorsCalls::UnbackedMintCapExceeded(element) => element.fmt(f),
                ErrorsCalls::UnderlyingBalanceZero(element) => element.fmt(f),
                ErrorsCalls::UnderlyingCannotBeRescued(element) => element.fmt(f),
                ErrorsCalls::UserInIsolationMode(element) => element.fmt(f),
                ErrorsCalls::VariableDebtSupplyNotZero(element) => element.fmt(f),
                ErrorsCalls::ZeroAddressNotValid(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AclAdminCannotBeZeroCall> for ErrorsCalls {
        fn from(var: AclAdminCannotBeZeroCall) -> Self {
            ErrorsCalls::AclAdminCannotBeZero(var)
        }
    }
    impl ::std::convert::From<AddressesProviderAlreadyAddedCall> for ErrorsCalls {
        fn from(var: AddressesProviderAlreadyAddedCall) -> Self {
            ErrorsCalls::AddressesProviderAlreadyAdded(var)
        }
    }
    impl ::std::convert::From<AddressesProviderNotRegisteredCall> for ErrorsCalls {
        fn from(var: AddressesProviderNotRegisteredCall) -> Self {
            ErrorsCalls::AddressesProviderNotRegistered(var)
        }
    }
    impl ::std::convert::From<AmountBiggerThanMaxLoanSizeStableCall> for ErrorsCalls {
        fn from(var: AmountBiggerThanMaxLoanSizeStableCall) -> Self {
            ErrorsCalls::AmountBiggerThanMaxLoanSizeStable(var)
        }
    }
    impl ::std::convert::From<AssetNotBorrowableInIsolationCall> for ErrorsCalls {
        fn from(var: AssetNotBorrowableInIsolationCall) -> Self {
            ErrorsCalls::AssetNotBorrowableInIsolation(var)
        }
    }
    impl ::std::convert::From<AssetNotListedCall> for ErrorsCalls {
        fn from(var: AssetNotListedCall) -> Self {
            ErrorsCalls::AssetNotListed(var)
        }
    }
    impl ::std::convert::From<AtokenSupplyNotZeroCall> for ErrorsCalls {
        fn from(var: AtokenSupplyNotZeroCall) -> Self {
            ErrorsCalls::AtokenSupplyNotZero(var)
        }
    }
    impl ::std::convert::From<BorrowingNotEnabledCall> for ErrorsCalls {
        fn from(var: BorrowingNotEnabledCall) -> Self {
            ErrorsCalls::BorrowingNotEnabled(var)
        }
    }
    impl ::std::convert::From<BorrowCapExceededCall> for ErrorsCalls {
        fn from(var: BorrowCapExceededCall) -> Self {
            ErrorsCalls::BorrowCapExceeded(var)
        }
    }
    impl ::std::convert::From<BridgeProtocolFeeInvalidCall> for ErrorsCalls {
        fn from(var: BridgeProtocolFeeInvalidCall) -> Self {
            ErrorsCalls::BridgeProtocolFeeInvalid(var)
        }
    }
    impl ::std::convert::From<CallerMustBePoolCall> for ErrorsCalls {
        fn from(var: CallerMustBePoolCall) -> Self {
            ErrorsCalls::CallerMustBePool(var)
        }
    }
    impl ::std::convert::From<CallerNotAssetListingOrPoolAdminCall> for ErrorsCalls {
        fn from(var: CallerNotAssetListingOrPoolAdminCall) -> Self {
            ErrorsCalls::CallerNotAssetListingOrPoolAdmin(var)
        }
    }
    impl ::std::convert::From<CallerNotAtokenCall> for ErrorsCalls {
        fn from(var: CallerNotAtokenCall) -> Self {
            ErrorsCalls::CallerNotAtoken(var)
        }
    }
    impl ::std::convert::From<CallerNotBridgeCall> for ErrorsCalls {
        fn from(var: CallerNotBridgeCall) -> Self {
            ErrorsCalls::CallerNotBridge(var)
        }
    }
    impl ::std::convert::From<CallerNotEmergencyAdminCall> for ErrorsCalls {
        fn from(var: CallerNotEmergencyAdminCall) -> Self {
            ErrorsCalls::CallerNotEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<CallerNotPoolAdminCall> for ErrorsCalls {
        fn from(var: CallerNotPoolAdminCall) -> Self {
            ErrorsCalls::CallerNotPoolAdmin(var)
        }
    }
    impl ::std::convert::From<CallerNotPoolConfiguratorCall> for ErrorsCalls {
        fn from(var: CallerNotPoolConfiguratorCall) -> Self {
            ErrorsCalls::CallerNotPoolConfigurator(var)
        }
    }
    impl ::std::convert::From<CallerNotPoolOrEmergencyAdminCall> for ErrorsCalls {
        fn from(var: CallerNotPoolOrEmergencyAdminCall) -> Self {
            ErrorsCalls::CallerNotPoolOrEmergencyAdmin(var)
        }
    }
    impl ::std::convert::From<CallerNotRiskOrPoolAdminCall> for ErrorsCalls {
        fn from(var: CallerNotRiskOrPoolAdminCall) -> Self {
            ErrorsCalls::CallerNotRiskOrPoolAdmin(var)
        }
    }
    impl ::std::convert::From<CollateralBalanceIsZeroCall> for ErrorsCalls {
        fn from(var: CollateralBalanceIsZeroCall) -> Self {
            ErrorsCalls::CollateralBalanceIsZero(var)
        }
    }
    impl ::std::convert::From<CollateralCannotBeLiquidatedCall> for ErrorsCalls {
        fn from(var: CollateralCannotBeLiquidatedCall) -> Self {
            ErrorsCalls::CollateralCannotBeLiquidated(var)
        }
    }
    impl ::std::convert::From<CollateralCannotCoverNewBorrowCall> for ErrorsCalls {
        fn from(var: CollateralCannotCoverNewBorrowCall) -> Self {
            ErrorsCalls::CollateralCannotCoverNewBorrow(var)
        }
    }
    impl ::std::convert::From<CollateralSameAsBorrowingCurrencyCall> for ErrorsCalls {
        fn from(var: CollateralSameAsBorrowingCurrencyCall) -> Self {
            ErrorsCalls::CollateralSameAsBorrowingCurrency(var)
        }
    }
    impl ::std::convert::From<DebtCeilingExceededCall> for ErrorsCalls {
        fn from(var: DebtCeilingExceededCall) -> Self {
            ErrorsCalls::DebtCeilingExceeded(var)
        }
    }
    impl ::std::convert::From<DebtCeilingNotZeroCall> for ErrorsCalls {
        fn from(var: DebtCeilingNotZeroCall) -> Self {
            ErrorsCalls::DebtCeilingNotZero(var)
        }
    }
    impl ::std::convert::From<EmodeCategoryReservedCall> for ErrorsCalls {
        fn from(var: EmodeCategoryReservedCall) -> Self {
            ErrorsCalls::EmodeCategoryReserved(var)
        }
    }
    impl ::std::convert::From<FlashloanPremiumInvalidCall> for ErrorsCalls {
        fn from(var: FlashloanPremiumInvalidCall) -> Self {
            ErrorsCalls::FlashloanPremiumInvalid(var)
        }
    }
    impl ::std::convert::From<HealthFactorLowerThanLiquidationThresholdCall> for ErrorsCalls {
        fn from(var: HealthFactorLowerThanLiquidationThresholdCall) -> Self {
            ErrorsCalls::HealthFactorLowerThanLiquidationThreshold(var)
        }
    }
    impl ::std::convert::From<HealthFactorNotBelowThresholdCall> for ErrorsCalls {
        fn from(var: HealthFactorNotBelowThresholdCall) -> Self {
            ErrorsCalls::HealthFactorNotBelowThreshold(var)
        }
    }
    impl ::std::convert::From<InconsistentEmodeCategoryCall> for ErrorsCalls {
        fn from(var: InconsistentEmodeCategoryCall) -> Self {
            ErrorsCalls::InconsistentEmodeCategory(var)
        }
    }
    impl ::std::convert::From<InconsistentFlashloanParamsCall> for ErrorsCalls {
        fn from(var: InconsistentFlashloanParamsCall) -> Self {
            ErrorsCalls::InconsistentFlashloanParams(var)
        }
    }
    impl ::std::convert::From<InconsistentParamsLengthCall> for ErrorsCalls {
        fn from(var: InconsistentParamsLengthCall) -> Self {
            ErrorsCalls::InconsistentParamsLength(var)
        }
    }
    impl ::std::convert::From<InterestRateRebalanceConditionsNotMetCall> for ErrorsCalls {
        fn from(var: InterestRateRebalanceConditionsNotMetCall) -> Self {
            ErrorsCalls::InterestRateRebalanceConditionsNotMet(var)
        }
    }
    impl ::std::convert::From<InvalidAddressesProviderCall> for ErrorsCalls {
        fn from(var: InvalidAddressesProviderCall) -> Self {
            ErrorsCalls::InvalidAddressesProvider(var)
        }
    }
    impl ::std::convert::From<InvalidAddressesProviderIdCall> for ErrorsCalls {
        fn from(var: InvalidAddressesProviderIdCall) -> Self {
            ErrorsCalls::InvalidAddressesProviderId(var)
        }
    }
    impl ::std::convert::From<InvalidAmountCall> for ErrorsCalls {
        fn from(var: InvalidAmountCall) -> Self {
            ErrorsCalls::InvalidAmount(var)
        }
    }
    impl ::std::convert::From<InvalidBorrowCapCall> for ErrorsCalls {
        fn from(var: InvalidBorrowCapCall) -> Self {
            ErrorsCalls::InvalidBorrowCap(var)
        }
    }
    impl ::std::convert::From<InvalidBurnAmountCall> for ErrorsCalls {
        fn from(var: InvalidBurnAmountCall) -> Self {
            ErrorsCalls::InvalidBurnAmount(var)
        }
    }
    impl ::std::convert::From<InvalidDebtCeilingCall> for ErrorsCalls {
        fn from(var: InvalidDebtCeilingCall) -> Self {
            ErrorsCalls::InvalidDebtCeiling(var)
        }
    }
    impl ::std::convert::From<InvalidDecimalsCall> for ErrorsCalls {
        fn from(var: InvalidDecimalsCall) -> Self {
            ErrorsCalls::InvalidDecimals(var)
        }
    }
    impl ::std::convert::From<InvalidEmodeCategoryCall> for ErrorsCalls {
        fn from(var: InvalidEmodeCategoryCall) -> Self {
            ErrorsCalls::InvalidEmodeCategory(var)
        }
    }
    impl ::std::convert::From<InvalidEmodeCategoryAssignmentCall> for ErrorsCalls {
        fn from(var: InvalidEmodeCategoryAssignmentCall) -> Self {
            ErrorsCalls::InvalidEmodeCategoryAssignment(var)
        }
    }
    impl ::std::convert::From<InvalidEmodeCategoryParamsCall> for ErrorsCalls {
        fn from(var: InvalidEmodeCategoryParamsCall) -> Self {
            ErrorsCalls::InvalidEmodeCategoryParams(var)
        }
    }
    impl ::std::convert::From<InvalidExpirationCall> for ErrorsCalls {
        fn from(var: InvalidExpirationCall) -> Self {
            ErrorsCalls::InvalidExpiration(var)
        }
    }
    impl ::std::convert::From<InvalidFlashloanExecutorReturnCall> for ErrorsCalls {
        fn from(var: InvalidFlashloanExecutorReturnCall) -> Self {
            ErrorsCalls::InvalidFlashloanExecutorReturn(var)
        }
    }
    impl ::std::convert::From<InvalidInterestRateModeSelectedCall> for ErrorsCalls {
        fn from(var: InvalidInterestRateModeSelectedCall) -> Self {
            ErrorsCalls::InvalidInterestRateModeSelected(var)
        }
    }
    impl ::std::convert::From<InvalidLiquidationProtocolFeeCall> for ErrorsCalls {
        fn from(var: InvalidLiquidationProtocolFeeCall) -> Self {
            ErrorsCalls::InvalidLiquidationProtocolFee(var)
        }
    }
    impl ::std::convert::From<InvalidLiqBonusCall> for ErrorsCalls {
        fn from(var: InvalidLiqBonusCall) -> Self {
            ErrorsCalls::InvalidLiqBonus(var)
        }
    }
    impl ::std::convert::From<InvalidLiqThresholdCall> for ErrorsCalls {
        fn from(var: InvalidLiqThresholdCall) -> Self {
            ErrorsCalls::InvalidLiqThreshold(var)
        }
    }
    impl ::std::convert::From<InvalidLtvCall> for ErrorsCalls {
        fn from(var: InvalidLtvCall) -> Self {
            ErrorsCalls::InvalidLtv(var)
        }
    }
    impl ::std::convert::From<InvalidMintAmountCall> for ErrorsCalls {
        fn from(var: InvalidMintAmountCall) -> Self {
            ErrorsCalls::InvalidMintAmount(var)
        }
    }
    impl ::std::convert::From<InvalidOptimalStableToTotalDebtRatioCall> for ErrorsCalls {
        fn from(var: InvalidOptimalStableToTotalDebtRatioCall) -> Self {
            ErrorsCalls::InvalidOptimalStableToTotalDebtRatio(var)
        }
    }
    impl ::std::convert::From<InvalidOptimalUsageRatioCall> for ErrorsCalls {
        fn from(var: InvalidOptimalUsageRatioCall) -> Self {
            ErrorsCalls::InvalidOptimalUsageRatio(var)
        }
    }
    impl ::std::convert::From<InvalidReserveFactorCall> for ErrorsCalls {
        fn from(var: InvalidReserveFactorCall) -> Self {
            ErrorsCalls::InvalidReserveFactor(var)
        }
    }
    impl ::std::convert::From<InvalidReserveIndexCall> for ErrorsCalls {
        fn from(var: InvalidReserveIndexCall) -> Self {
            ErrorsCalls::InvalidReserveIndex(var)
        }
    }
    impl ::std::convert::From<InvalidReserveParamsCall> for ErrorsCalls {
        fn from(var: InvalidReserveParamsCall) -> Self {
            ErrorsCalls::InvalidReserveParams(var)
        }
    }
    impl ::std::convert::From<InvalidSignatureCall> for ErrorsCalls {
        fn from(var: InvalidSignatureCall) -> Self {
            ErrorsCalls::InvalidSignature(var)
        }
    }
    impl ::std::convert::From<InvalidSupplyCapCall> for ErrorsCalls {
        fn from(var: InvalidSupplyCapCall) -> Self {
            ErrorsCalls::InvalidSupplyCap(var)
        }
    }
    impl ::std::convert::From<InvalidUnbackedMintCapCall> for ErrorsCalls {
        fn from(var: InvalidUnbackedMintCapCall) -> Self {
            ErrorsCalls::InvalidUnbackedMintCap(var)
        }
    }
    impl ::std::convert::From<LtvValidationFailedCall> for ErrorsCalls {
        fn from(var: LtvValidationFailedCall) -> Self {
            ErrorsCalls::LtvValidationFailed(var)
        }
    }
    impl ::std::convert::From<NotContractCall> for ErrorsCalls {
        fn from(var: NotContractCall) -> Self {
            ErrorsCalls::NotContract(var)
        }
    }
    impl ::std::convert::From<NotEnoughAvailableUserBalanceCall> for ErrorsCalls {
        fn from(var: NotEnoughAvailableUserBalanceCall) -> Self {
            ErrorsCalls::NotEnoughAvailableUserBalance(var)
        }
    }
    impl ::std::convert::From<NoDebtOfSelectedTypeCall> for ErrorsCalls {
        fn from(var: NoDebtOfSelectedTypeCall) -> Self {
            ErrorsCalls::NoDebtOfSelectedType(var)
        }
    }
    impl ::std::convert::From<NoExplicitAmountToRepayOnBehalfCall> for ErrorsCalls {
        fn from(var: NoExplicitAmountToRepayOnBehalfCall) -> Self {
            ErrorsCalls::NoExplicitAmountToRepayOnBehalf(var)
        }
    }
    impl ::std::convert::From<NoMoreReservesAllowedCall> for ErrorsCalls {
        fn from(var: NoMoreReservesAllowedCall) -> Self {
            ErrorsCalls::NoMoreReservesAllowed(var)
        }
    }
    impl ::std::convert::From<NoOutstandingStableDebtCall> for ErrorsCalls {
        fn from(var: NoOutstandingStableDebtCall) -> Self {
            ErrorsCalls::NoOutstandingStableDebt(var)
        }
    }
    impl ::std::convert::From<NoOutstandingVariableDebtCall> for ErrorsCalls {
        fn from(var: NoOutstandingVariableDebtCall) -> Self {
            ErrorsCalls::NoOutstandingVariableDebt(var)
        }
    }
    impl ::std::convert::From<OperationNotSupportedCall> for ErrorsCalls {
        fn from(var: OperationNotSupportedCall) -> Self {
            ErrorsCalls::OperationNotSupported(var)
        }
    }
    impl ::std::convert::From<PoolAddressesDoNotMatchCall> for ErrorsCalls {
        fn from(var: PoolAddressesDoNotMatchCall) -> Self {
            ErrorsCalls::PoolAddressesDoNotMatch(var)
        }
    }
    impl ::std::convert::From<PriceOracleSentinelCheckFailedCall> for ErrorsCalls {
        fn from(var: PriceOracleSentinelCheckFailedCall) -> Self {
            ErrorsCalls::PriceOracleSentinelCheckFailed(var)
        }
    }
    impl ::std::convert::From<ReserveAlreadyAddedCall> for ErrorsCalls {
        fn from(var: ReserveAlreadyAddedCall) -> Self {
            ErrorsCalls::ReserveAlreadyAdded(var)
        }
    }
    impl ::std::convert::From<ReserveAlreadyInitializedCall> for ErrorsCalls {
        fn from(var: ReserveAlreadyInitializedCall) -> Self {
            ErrorsCalls::ReserveAlreadyInitialized(var)
        }
    }
    impl ::std::convert::From<ReserveDebtNotZeroCall> for ErrorsCalls {
        fn from(var: ReserveDebtNotZeroCall) -> Self {
            ErrorsCalls::ReserveDebtNotZero(var)
        }
    }
    impl ::std::convert::From<ReserveFrozenCall> for ErrorsCalls {
        fn from(var: ReserveFrozenCall) -> Self {
            ErrorsCalls::ReserveFrozen(var)
        }
    }
    impl ::std::convert::From<ReserveInactiveCall> for ErrorsCalls {
        fn from(var: ReserveInactiveCall) -> Self {
            ErrorsCalls::ReserveInactive(var)
        }
    }
    impl ::std::convert::From<ReserveLiquidityNotZeroCall> for ErrorsCalls {
        fn from(var: ReserveLiquidityNotZeroCall) -> Self {
            ErrorsCalls::ReserveLiquidityNotZero(var)
        }
    }
    impl ::std::convert::From<ReservePausedCall> for ErrorsCalls {
        fn from(var: ReservePausedCall) -> Self {
            ErrorsCalls::ReservePaused(var)
        }
    }
    impl ::std::convert::From<SameBlockBorrowRepayCall> for ErrorsCalls {
        fn from(var: SameBlockBorrowRepayCall) -> Self {
            ErrorsCalls::SameBlockBorrowRepay(var)
        }
    }
    impl ::std::convert::From<SiloedBorrowingViolationCall> for ErrorsCalls {
        fn from(var: SiloedBorrowingViolationCall) -> Self {
            ErrorsCalls::SiloedBorrowingViolation(var)
        }
    }
    impl ::std::convert::From<SpecifiedCurrencyNotBorrowedByUserCall> for ErrorsCalls {
        fn from(var: SpecifiedCurrencyNotBorrowedByUserCall) -> Self {
            ErrorsCalls::SpecifiedCurrencyNotBorrowedByUser(var)
        }
    }
    impl ::std::convert::From<StableBorrowingEnabledCall> for ErrorsCalls {
        fn from(var: StableBorrowingEnabledCall) -> Self {
            ErrorsCalls::StableBorrowingEnabled(var)
        }
    }
    impl ::std::convert::From<StableBorrowingNotEnabledCall> for ErrorsCalls {
        fn from(var: StableBorrowingNotEnabledCall) -> Self {
            ErrorsCalls::StableBorrowingNotEnabled(var)
        }
    }
    impl ::std::convert::From<StableDebtNotZeroCall> for ErrorsCalls {
        fn from(var: StableDebtNotZeroCall) -> Self {
            ErrorsCalls::StableDebtNotZero(var)
        }
    }
    impl ::std::convert::From<SupplyCapExceededCall> for ErrorsCalls {
        fn from(var: SupplyCapExceededCall) -> Self {
            ErrorsCalls::SupplyCapExceeded(var)
        }
    }
    impl ::std::convert::From<UnbackedMintCapExceededCall> for ErrorsCalls {
        fn from(var: UnbackedMintCapExceededCall) -> Self {
            ErrorsCalls::UnbackedMintCapExceeded(var)
        }
    }
    impl ::std::convert::From<UnderlyingBalanceZeroCall> for ErrorsCalls {
        fn from(var: UnderlyingBalanceZeroCall) -> Self {
            ErrorsCalls::UnderlyingBalanceZero(var)
        }
    }
    impl ::std::convert::From<UnderlyingCannotBeRescuedCall> for ErrorsCalls {
        fn from(var: UnderlyingCannotBeRescuedCall) -> Self {
            ErrorsCalls::UnderlyingCannotBeRescued(var)
        }
    }
    impl ::std::convert::From<UserInIsolationModeCall> for ErrorsCalls {
        fn from(var: UserInIsolationModeCall) -> Self {
            ErrorsCalls::UserInIsolationMode(var)
        }
    }
    impl ::std::convert::From<VariableDebtSupplyNotZeroCall> for ErrorsCalls {
        fn from(var: VariableDebtSupplyNotZeroCall) -> Self {
            ErrorsCalls::VariableDebtSupplyNotZero(var)
        }
    }
    impl ::std::convert::From<ZeroAddressNotValidCall> for ErrorsCalls {
        fn from(var: ZeroAddressNotValidCall) -> Self {
            ErrorsCalls::ZeroAddressNotValid(var)
        }
    }
}
