pub use timelock_mod::*;
#[allow(clippy::too_many_arguments)]
mod timelock_mod {
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
    #[doc = "Timelock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TIMELOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"delay_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CancelTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecuteTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewDelay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"QueueTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"GRACE_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAXIMUM_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINIMUM_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTransaction\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queueTransaction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queuedTransactions\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"delay_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDelay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pendingAdmin_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingAdmin\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TIMELOCK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506040516118a63803806118a68339818101604052604081101561003357600080fd5b5080516020909101516202a30081101561007e5760405162461bcd60e51b81526004018080602001828103825260378152602001806118376037913960400191505060405180910390fd5b62278d008111156100c05760405162461bcd60e51b815260040180806020018281038252603881526020018061186e6038913960400191505060405180910390fd5b600080546001600160a01b039093166001600160a01b031990931692909217909155600255611743806100f46000396000f3fe6080604052600436106100c25760003560e01c80636a42b8f81161007f578063c1a287e211610059578063c1a287e2146105dd578063e177246e146105f2578063f2b065371461061c578063f851a4401461065a576100c2565b80636a42b8f81461059e5780637d645fab146105b3578063b1b43ae5146105c8576100c2565b80630825f38f146100c45780630e18b68114610279578063267822471461028e5780633a66f901146102bf5780634dd18bf51461041e578063591fcdfe14610451575b005b610204600480360360a08110156100da57600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b81111561010957600080fd5b82018360208201111561011b57600080fd5b803590602001918460018302840111600160201b8311171561013c57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561018e57600080fd5b8201836020820111156101a057600080fd5b803590602001918460018302840111600160201b831117156101c157600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550509135925061066f915050565b6040805160208082528351818301528351919283929083019185019080838360005b8381101561023e578181015183820152602001610226565b50505050905090810190601f16801561026b5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b34801561028557600080fd5b506100c2610b88565b34801561029a57600080fd5b506102a3610c24565b604080516001600160a01b039092168252519081900360200190f35b3480156102cb57600080fd5b5061040c600480360360a08110156102e257600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b81111561031157600080fd5b82018360208201111561032357600080fd5b803590602001918460018302840111600160201b8311171561034457600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561039657600080fd5b8201836020820111156103a857600080fd5b803590602001918460018302840111600160201b831117156103c957600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505091359250610c33915050565b60408051918252519081900360200190f35b34801561042a57600080fd5b506100c26004803603602081101561044157600080fd5b50356001600160a01b0316610f44565b34801561045d57600080fd5b506100c2600480360360a081101561047457600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b8111156104a357600080fd5b8201836020820111156104b557600080fd5b803590602001918460018302840111600160201b831117156104d657600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295949360208101935035915050600160201b81111561052857600080fd5b82018360208201111561053a57600080fd5b803590602001918460018302840111600160201b8311171561055b57600080fd5b91908080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152509295505091359250610fd2915050565b3480156105aa57600080fd5b5061040c611288565b3480156105bf57600080fd5b5061040c61128e565b3480156105d457600080fd5b5061040c611295565b3480156105e957600080fd5b5061040c61129c565b3480156105fe57600080fd5b506100c26004803603602081101561061557600080fd5b50356112a3565b34801561062857600080fd5b506106466004803603602081101561063f57600080fd5b5035611398565b604080519115158252519081900360200190f35b34801561066657600080fd5b506102a36113ad565b6000546060906001600160a01b031633146106bb5760405162461bcd60e51b81526004018080602001828103825260388152602001806114226038913960400191505060405180910390fd5b6000868686868660405160200180866001600160a01b03166001600160a01b031681526020018581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b8381101561072a578181015183820152602001610712565b50505050905090810190601f1680156107575780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b8381101561078a578181015183820152602001610772565b50505050905090810190601f1680156107b75780820380516001836020036101000a031916815260200191505b5060408051601f1981840301815291815281516020928301206000818152600390935291205490995060ff16975061082896505050505050505760405162461bcd60e51b815260040180806020018281038252603d815260200180611575603d913960400191505060405180910390fd5b826108316113bc565b101561086e5760405162461bcd60e51b81526004018080602001828103825260458152602001806114c46045913960600191505060405180910390fd5b610881836212750063ffffffff6113c016565b6108896113bc565b11156108c65760405162461bcd60e51b81526004018080602001828103825260338152602001806114916033913960400191505060405180910390fd5b6000818152600360205260409020805460ff1916905584516060906108ec575083610979565b85805190602001208560405160200180836001600160e01b0319166001600160e01b031916815260040182805190602001908083835b602083106109415780518252601f199092019160209182019101610922565b6001836020036101000a0380198251168184511680821785525050505050509050019250505060405160208183030381529060405290505b60006060896001600160a01b031689846040518082805190602001908083835b602083106109b85780518252601f199092019160209182019101610999565b6001836020036101000a03801982511681845116808217855250505050505090500191505060006040518083038185875af1925050503d8060008114610a1a576040519150601f19603f3d011682016040523d82523d6000602084013e610a1f565b606091505b509150915081610a605760405162461bcd60e51b815260040180806020018281038252603d815260200180611658603d913960400191505060405180910390fd5b896001600160a01b0316847fa560e3198060a2f10670c1ec5b403077ea6ae93ca8de1c32b451dc1a943cd6e78b8b8b8b604051808581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b83811015610add578181015183820152602001610ac5565b50505050905090810190601f168015610b0a5780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b83811015610b3d578181015183820152602001610b25565b50505050905090810190601f168015610b6a5780820380516001836020036101000a031916815260200191505b50965050505050505060405180910390a39998505050505050505050565b6001546001600160a01b03163314610bd15760405162461bcd60e51b81526004018080602001828103825260388152602001806115b26038913960400191505060405180910390fd5b60008054336001600160a01b031991821617808355600180549092169091556040516001600160a01b03909116917f71614071b88dee5e0b2ae578a9dd7b2ebbe9ae832ba419dc0242cd065a290b6c91a2565b6001546001600160a01b031681565b600080546001600160a01b03163314610c7d5760405162461bcd60e51b81526004018080602001828103825260368152602001806116226036913960400191505060405180910390fd5b610c97600254610c8b6113bc565b9063ffffffff6113c016565b821015610cd55760405162461bcd60e51b81526004018080602001828103825260498152602001806116956049913960600191505060405180910390fd5b6000868686868660405160200180866001600160a01b03166001600160a01b031681526020018581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b83811015610d44578181015183820152602001610d2c565b50505050905090810190601f168015610d715780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b83811015610da4578181015183820152602001610d8c565b50505050905090810190601f168015610dd15780820380516001836020036101000a031916815260200191505b5097505050505050505060405160208183030381529060405280519060200120905060016003600083815260200190815260200160002060006101000a81548160ff021916908315150217905550866001600160a01b0316817f76e2796dc3a81d57b0e8504b647febcbeeb5f4af818e164f11eef8131a6a763f88888888604051808581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b83811015610e9c578181015183820152602001610e84565b50505050905090810190601f168015610ec95780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b83811015610efc578181015183820152602001610ee4565b50505050905090810190601f168015610f295780820380516001836020036101000a031916815260200191505b50965050505050505060405180910390a39695505050505050565b333014610f825760405162461bcd60e51b81526004018080602001828103825260388152602001806115ea6038913960400191505060405180910390fd5b600180546001600160a01b0319166001600160a01b0383811691909117918290556040519116907f69d78e38a01985fbb1462961809b4b2d65531bc93b2b94037f3334b82ca4a75690600090a250565b6000546001600160a01b0316331461101b5760405162461bcd60e51b815260040180806020018281038252603781526020018061145a6037913960400191505060405180910390fd5b6000858585858560405160200180866001600160a01b03166001600160a01b031681526020018581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b8381101561108a578181015183820152602001611072565b50505050905090810190601f1680156110b75780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b838110156110ea5781810151838201526020016110d2565b50505050905090810190601f1680156111175780820380516001836020036101000a031916815260200191505b5097505050505050505060405160208183030381529060405280519060200120905060006003600083815260200190815260200160002060006101000a81548160ff021916908315150217905550856001600160a01b0316817f2fffc091a501fd91bfbff27141450d3acb40fb8e6d8382b243ec7a812a3aaf8787878787604051808581526020018060200180602001848152602001838103835286818151815260200191508051906020019080838360005b838110156111e25781810151838201526020016111ca565b50505050905090810190601f16801561120f5780820380516001836020036101000a031916815260200191505b50838103825285518152855160209182019187019080838360005b8381101561124257818101518382015260200161122a565b50505050905090810190601f16801561126f5780820380516001836020036101000a031916815260200191505b50965050505050505060405180910390a3505050505050565b60025481565b62278d0081565b6202a30081565b6212750081565b3330146112e15760405162461bcd60e51b81526004018080602001828103825260318152602001806116de6031913960400191505060405180910390fd5b6202a3008110156113235760405162461bcd60e51b81526004018080602001828103825260348152602001806115096034913960400191505060405180910390fd5b62278d008111156113655760405162461bcd60e51b815260040180806020018281038252603881526020018061153d6038913960400191505060405180910390fd5b600281905560405181907f948b1f6a42ee138b7e34058ba85a37f716d55ff25ff05a763f15bed6a04c8d2c90600090a250565b60036020526000908152604090205460ff1681565b6000546001600160a01b031681565b4290565b60008282018381101561141a576040805162461bcd60e51b815260206004820152601b60248201527f536166654d6174683a206164646974696f6e206f766572666c6f770000000000604482015290519081900360640190fd5b939250505056fe54696d656c6f636b3a3a657865637574655472616e73616374696f6e3a2043616c6c206d75737420636f6d652066726f6d2061646d696e2e54696d656c6f636b3a3a63616e63656c5472616e73616374696f6e3a2043616c6c206d75737420636f6d652066726f6d2061646d696e2e54696d656c6f636b3a3a657865637574655472616e73616374696f6e3a205472616e73616374696f6e206973207374616c652e54696d656c6f636b3a3a657865637574655472616e73616374696f6e3a205472616e73616374696f6e206861736e2774207375727061737365642074696d65206c6f636b2e54696d656c6f636b3a3a73657444656c61793a2044656c6179206d75737420657863656564206d696e696d756d2064656c61792e54696d656c6f636b3a3a73657444656c61793a2044656c6179206d757374206e6f7420657863656564206d6178696d756d2064656c61792e54696d656c6f636b3a3a657865637574655472616e73616374696f6e3a205472616e73616374696f6e206861736e2774206265656e207175657565642e54696d656c6f636b3a3a61636365707441646d696e3a2043616c6c206d75737420636f6d652066726f6d2070656e64696e6741646d696e2e54696d656c6f636b3a3a73657450656e64696e6741646d696e3a2043616c6c206d75737420636f6d652066726f6d2054696d656c6f636b2e54696d656c6f636b3a3a71756575655472616e73616374696f6e3a2043616c6c206d75737420636f6d652066726f6d2061646d696e2e54696d656c6f636b3a3a657865637574655472616e73616374696f6e3a205472616e73616374696f6e20657865637574696f6e2072657665727465642e54696d656c6f636b3a3a71756575655472616e73616374696f6e3a20457374696d6174656420657865637574696f6e20626c6f636b206d75737420736174697366792064656c61792e54696d656c6f636b3a3a73657444656c61793a2043616c6c206d75737420636f6d652066726f6d2054696d656c6f636b2ea265627a7a723158209feba11764ce0b90f0eeca5e476946f1a31acf46274a683e87e23daf298e36ac64736f6c6343000511003254696d656c6f636b3a3a636f6e7374727563746f723a2044656c6179206d75737420657863656564206d696e696d756d2064656c61792e54696d656c6f636b3a3a73657444656c61793a2044656c6179206d757374206e6f7420657863656564206d6178696d756d2064656c61792e" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Timelock<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Timelock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Timelock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Timelock))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Timelock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TIMELOCK_ABI.clone(), client).into()
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
                TIMELOCK_ABI.clone(),
                TIMELOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `GRACE_PERIOD` (0xc1a287e2) function"]
        pub fn grace_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 162, 135, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAXIMUM_DELAY` (0x7d645fab) function"]
        pub fn maximum_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([125, 100, 95, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_DELAY` (0xb1b43ae5) function"]
        pub fn minimum_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 180, 58, 229], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `acceptAdmin` (0x0e18b681) function"]
        pub fn accept_admin(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 24, 182, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelTransaction` (0x591fcdfe) function"]
        pub fn cancel_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 31, 205, 254], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delay` (0x6a42b8f8) function"]
        pub fn delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([106, 66, 184, 248], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeTransaction` (0x0825f38f) function"]
        pub fn execute_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([8, 37, 243, 143], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingAdmin` (0x26782247) function"]
        pub fn pending_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queueTransaction` (0x3a66f901) function"]
        pub fn queue_transaction(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            signature: String,
            data: ethers::core::types::Bytes,
            eta: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([58, 102, 249, 1], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queuedTransactions` (0xf2b06537) function"]
        pub fn queued_transactions(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([242, 176, 101, 55], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDelay` (0xe177246e) function"]
        pub fn set_delay(
            &self,
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 119, 36, 110], delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPendingAdmin` (0x4dd18bf5) function"]
        pub fn set_pending_admin(
            &self,
            pending_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 209, 139, 245], pending_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CancelTransaction` event"]
        pub fn cancel_transaction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelTransactionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteTransaction` event"]
        pub fn execute_transaction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteTransactionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewAdmin` event"]
        pub fn new_admin_filter(&self) -> ethers::contract::builders::Event<M, NewAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewDelay` event"]
        pub fn new_delay_filter(&self) -> ethers::contract::builders::Event<M, NewDelayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPendingAdmin` event"]
        pub fn new_pending_admin_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPendingAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `QueueTransaction` event"]
        pub fn queue_transaction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, QueueTransactionFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, TimelockEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Timelock<M> {
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
        name = "CancelTransaction",
        abi = "CancelTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct CancelTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
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
        name = "ExecuteTransaction",
        abi = "ExecuteTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct ExecuteTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
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
    #[ethevent(name = "NewAdmin", abi = "NewAdmin(address)")]
    pub struct NewAdminFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "NewDelay", abi = "NewDelay(uint256)")]
    pub struct NewDelayFilter {
        #[ethevent(indexed)]
        pub new_delay: ethers::core::types::U256,
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
    #[ethevent(name = "NewPendingAdmin", abi = "NewPendingAdmin(address)")]
    pub struct NewPendingAdminFilter {
        #[ethevent(indexed)]
        pub new_pending_admin: ethers::core::types::Address,
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
        name = "QueueTransaction",
        abi = "QueueTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct QueueTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TimelockEvents {
        CancelTransactionFilter(CancelTransactionFilter),
        ExecuteTransactionFilter(ExecuteTransactionFilter),
        NewAdminFilter(NewAdminFilter),
        NewDelayFilter(NewDelayFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        QueueTransactionFilter(QueueTransactionFilter),
    }
    impl ethers::contract::EthLogDecode for TimelockEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CancelTransactionFilter::decode_log(log) {
                return Ok(TimelockEvents::CancelTransactionFilter(decoded));
            }
            if let Ok(decoded) = ExecuteTransactionFilter::decode_log(log) {
                return Ok(TimelockEvents::ExecuteTransactionFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(TimelockEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewDelayFilter::decode_log(log) {
                return Ok(TimelockEvents::NewDelayFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(TimelockEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = QueueTransactionFilter::decode_log(log) {
                return Ok(TimelockEvents::QueueTransactionFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TimelockEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TimelockEvents::CancelTransactionFilter(element) => element.fmt(f),
                TimelockEvents::ExecuteTransactionFilter(element) => element.fmt(f),
                TimelockEvents::NewAdminFilter(element) => element.fmt(f),
                TimelockEvents::NewDelayFilter(element) => element.fmt(f),
                TimelockEvents::NewPendingAdminFilter(element) => element.fmt(f),
                TimelockEvents::QueueTransactionFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `GRACE_PERIOD`function with signature `GRACE_PERIOD()` and selector `[193, 162, 135, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "GRACE_PERIOD", abi = "GRACE_PERIOD()")]
    pub struct GracePeriodCall;
    #[doc = "Container type for all input parameters for the `MAXIMUM_DELAY`function with signature `MAXIMUM_DELAY()` and selector `[125, 100, 95, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAXIMUM_DELAY", abi = "MAXIMUM_DELAY()")]
    pub struct MaximumDelayCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_DELAY`function with signature `MINIMUM_DELAY()` and selector `[177, 180, 58, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MINIMUM_DELAY", abi = "MINIMUM_DELAY()")]
    pub struct MinimumDelayCall;
    #[doc = "Container type for all input parameters for the `acceptAdmin`function with signature `acceptAdmin()` and selector `[14, 24, 182, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptAdmin", abi = "acceptAdmin()")]
    pub struct AcceptAdminCall;
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `cancelTransaction`function with signature `cancelTransaction(address,uint256,string,bytes,uint256)` and selector `[89, 31, 205, 254]`"]
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
        name = "cancelTransaction",
        abi = "cancelTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct CancelTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `delay`function with signature `delay()` and selector `[106, 66, 184, 248]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delay", abi = "delay()")]
    pub struct DelayCall;
    #[doc = "Container type for all input parameters for the `executeTransaction`function with signature `executeTransaction(address,uint256,string,bytes,uint256)` and selector `[8, 37, 243, 143]`"]
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
        name = "executeTransaction",
        abi = "executeTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct ExecuteTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `pendingAdmin`function with signature `pendingAdmin()` and selector `[38, 120, 34, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    #[doc = "Container type for all input parameters for the `queueTransaction`function with signature `queueTransaction(address,uint256,string,bytes,uint256)` and selector `[58, 102, 249, 1]`"]
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
        name = "queueTransaction",
        abi = "queueTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct QueueTransactionCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub signature: String,
        pub data: ethers::core::types::Bytes,
        pub eta: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `queuedTransactions`function with signature `queuedTransactions(bytes32)` and selector `[242, 176, 101, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "queuedTransactions", abi = "queuedTransactions(bytes32)")]
    pub struct QueuedTransactionsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `setDelay`function with signature `setDelay(uint256)` and selector `[225, 119, 36, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDelay", abi = "setDelay(uint256)")]
    pub struct SetDelayCall {
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPendingAdmin`function with signature `setPendingAdmin(address)` and selector `[77, 209, 139, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPendingAdmin", abi = "setPendingAdmin(address)")]
    pub struct SetPendingAdminCall {
        pub pending_admin: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TimelockCalls {
        GracePeriod(GracePeriodCall),
        MaximumDelay(MaximumDelayCall),
        MinimumDelay(MinimumDelayCall),
        AcceptAdmin(AcceptAdminCall),
        Admin(AdminCall),
        CancelTransaction(CancelTransactionCall),
        Delay(DelayCall),
        ExecuteTransaction(ExecuteTransactionCall),
        PendingAdmin(PendingAdminCall),
        QueueTransaction(QueueTransactionCall),
        QueuedTransactions(QueuedTransactionsCall),
        SetDelay(SetDelayCall),
        SetPendingAdmin(SetPendingAdminCall),
    }
    impl ethers::core::abi::AbiDecode for TimelockCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GracePeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::GracePeriod(decoded));
            }
            if let Ok(decoded) =
                <MaximumDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::MaximumDelay(decoded));
            }
            if let Ok(decoded) =
                <MinimumDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::MinimumDelay(decoded));
            }
            if let Ok(decoded) =
                <AcceptAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::AcceptAdmin(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <CancelTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::CancelTransaction(decoded));
            }
            if let Ok(decoded) = <DelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::Delay(decoded));
            }
            if let Ok(decoded) =
                <ExecuteTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::ExecuteTransaction(decoded));
            }
            if let Ok(decoded) =
                <PendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::PendingAdmin(decoded));
            }
            if let Ok(decoded) =
                <QueueTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::QueueTransaction(decoded));
            }
            if let Ok(decoded) =
                <QueuedTransactionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::QueuedTransactions(decoded));
            }
            if let Ok(decoded) =
                <SetDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::SetDelay(decoded));
            }
            if let Ok(decoded) =
                <SetPendingAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimelockCalls::SetPendingAdmin(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TimelockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TimelockCalls::GracePeriod(element) => element.encode(),
                TimelockCalls::MaximumDelay(element) => element.encode(),
                TimelockCalls::MinimumDelay(element) => element.encode(),
                TimelockCalls::AcceptAdmin(element) => element.encode(),
                TimelockCalls::Admin(element) => element.encode(),
                TimelockCalls::CancelTransaction(element) => element.encode(),
                TimelockCalls::Delay(element) => element.encode(),
                TimelockCalls::ExecuteTransaction(element) => element.encode(),
                TimelockCalls::PendingAdmin(element) => element.encode(),
                TimelockCalls::QueueTransaction(element) => element.encode(),
                TimelockCalls::QueuedTransactions(element) => element.encode(),
                TimelockCalls::SetDelay(element) => element.encode(),
                TimelockCalls::SetPendingAdmin(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TimelockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TimelockCalls::GracePeriod(element) => element.fmt(f),
                TimelockCalls::MaximumDelay(element) => element.fmt(f),
                TimelockCalls::MinimumDelay(element) => element.fmt(f),
                TimelockCalls::AcceptAdmin(element) => element.fmt(f),
                TimelockCalls::Admin(element) => element.fmt(f),
                TimelockCalls::CancelTransaction(element) => element.fmt(f),
                TimelockCalls::Delay(element) => element.fmt(f),
                TimelockCalls::ExecuteTransaction(element) => element.fmt(f),
                TimelockCalls::PendingAdmin(element) => element.fmt(f),
                TimelockCalls::QueueTransaction(element) => element.fmt(f),
                TimelockCalls::QueuedTransactions(element) => element.fmt(f),
                TimelockCalls::SetDelay(element) => element.fmt(f),
                TimelockCalls::SetPendingAdmin(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GracePeriodCall> for TimelockCalls {
        fn from(var: GracePeriodCall) -> Self {
            TimelockCalls::GracePeriod(var)
        }
    }
    impl ::std::convert::From<MaximumDelayCall> for TimelockCalls {
        fn from(var: MaximumDelayCall) -> Self {
            TimelockCalls::MaximumDelay(var)
        }
    }
    impl ::std::convert::From<MinimumDelayCall> for TimelockCalls {
        fn from(var: MinimumDelayCall) -> Self {
            TimelockCalls::MinimumDelay(var)
        }
    }
    impl ::std::convert::From<AcceptAdminCall> for TimelockCalls {
        fn from(var: AcceptAdminCall) -> Self {
            TimelockCalls::AcceptAdmin(var)
        }
    }
    impl ::std::convert::From<AdminCall> for TimelockCalls {
        fn from(var: AdminCall) -> Self {
            TimelockCalls::Admin(var)
        }
    }
    impl ::std::convert::From<CancelTransactionCall> for TimelockCalls {
        fn from(var: CancelTransactionCall) -> Self {
            TimelockCalls::CancelTransaction(var)
        }
    }
    impl ::std::convert::From<DelayCall> for TimelockCalls {
        fn from(var: DelayCall) -> Self {
            TimelockCalls::Delay(var)
        }
    }
    impl ::std::convert::From<ExecuteTransactionCall> for TimelockCalls {
        fn from(var: ExecuteTransactionCall) -> Self {
            TimelockCalls::ExecuteTransaction(var)
        }
    }
    impl ::std::convert::From<PendingAdminCall> for TimelockCalls {
        fn from(var: PendingAdminCall) -> Self {
            TimelockCalls::PendingAdmin(var)
        }
    }
    impl ::std::convert::From<QueueTransactionCall> for TimelockCalls {
        fn from(var: QueueTransactionCall) -> Self {
            TimelockCalls::QueueTransaction(var)
        }
    }
    impl ::std::convert::From<QueuedTransactionsCall> for TimelockCalls {
        fn from(var: QueuedTransactionsCall) -> Self {
            TimelockCalls::QueuedTransactions(var)
        }
    }
    impl ::std::convert::From<SetDelayCall> for TimelockCalls {
        fn from(var: SetDelayCall) -> Self {
            TimelockCalls::SetDelay(var)
        }
    }
    impl ::std::convert::From<SetPendingAdminCall> for TimelockCalls {
        fn from(var: SetPendingAdminCall) -> Self {
            TimelockCalls::SetPendingAdmin(var)
        }
    }
}
