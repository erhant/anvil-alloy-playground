use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    transports::http::{reqwest::Url, Client, Http},
};
use eyre::Result;

use crate::{TokenClient, ERC20};

/// Core fillers are recommended fillers, along with a wallet filler for Ethereum.
type CoreFiller = JoinFill<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    WalletFiller<EthereumWallet>,
>;
/// We are using HTTP transport, not WebSockets.
type CoreTransport = Http<Client>;
/// We are using Ethereum network.
type CoreNetwork = Ethereum;

impl
    TokenClient<
        CoreTransport,
        FillProvider<CoreFiller, RootProvider<CoreTransport>, CoreTransport, CoreNetwork>,
        CoreNetwork,
    >
{
    pub async fn new(wallet: EthereumWallet, url: Url, token: Address) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url);
        let token = ERC20::new(token, provider.clone());

        Ok(Self { provider, token })
    }
}
