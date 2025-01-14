use alloy::providers::{Provider, WalletProvider};
use alloy::transports::Transport;

use crate::{TokenClient, ERC20};

impl<T, P, N> TokenClient<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + Clone + WalletProvider<N>,
    N: alloy::network::Network + alloy::providers::fillers::RecommendedFillers,
{
    pub fn connect(&mut self, wallet: <P as WalletProvider<N>>::Wallet) {
        let mut provider = self.provider.clone();
        *provider.wallet_mut() = wallet;

        // create a new token with the updated provider
        self.token = ERC20::new(*self.token.address(), provider.clone())
    }

    pub fn connect_new(&self, wallet: <P as WalletProvider<N>>::Wallet) -> Self {
        let mut new = self.clone();
        new.connect(wallet);
        new
    }
}
