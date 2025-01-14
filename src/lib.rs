mod contracts {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        ERC20,
        "abi/erc20.json"
    );
}
pub(crate) use contracts::*;

mod client;
pub use client::TokenClient;
