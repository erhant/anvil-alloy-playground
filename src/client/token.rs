use alloy::primitives::{Address, U256};
use eyre::Result;

use super::TokenClient;

impl<T, P, N> TokenClient<T, P, N>
where
    T: alloy::transports::Transport + ::core::clone::Clone,
    P: alloy::providers::Provider<T, N>,
    N: alloy::network::Network + alloy::providers::fillers::RecommendedFillers,
{
    /// Transfer tokens from one address to another, calls `transferFrom` of the ERC20 contract.
    ///
    /// Assumes that approvals are made priorly.
    pub async fn transfer_from(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<N::ReceiptResponse> {
        let req = self.token.transferFrom(from, to, amount);
        let tx = req.send().await?;
        tx.get_receipt().await.map_err(Into::into)
    }

    /// Approves the `spender` to spend `amount` tokens on behalf of the caller.
    pub async fn approve(&self, spender: Address, amount: U256) -> Result<N::ReceiptResponse> {
        let req = self.token.approve(spender, amount);
        let tx = req.send().await?;
        tx.get_receipt().await.map_err(Into::into)
    }

    /// Returns the token balance of a given address.
    pub async fn balance(&self, address: Address) -> Result<(U256, String)> {
        let balance = self.token.balanceOf(address).call().await?._0;
        let symbol = self.token.symbol().call().await?._0;
        Ok((balance, symbol))
    }

    /// Returns the allowance of a given `spender` address to spend tokens on behalf of `owner` address.
    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<(U256, String)> {
        let symbol = self.token.symbol().call().await?._0;
        let allowance = self.token.allowance(owner, spender).call().await?._0;
        Ok((allowance, symbol))
    }
}
