// TODO: feature gate

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        layers::AnvilProvider,
        Identity, ProviderBuilder, RootProvider,
    },
    transports::{http::reqwest::Url, BoxTransport},
};
use eyre::Result;

use crate::{TokenClient, ERC20};

impl
    TokenClient<
        BoxTransport,
        FillProvider<
            JoinFill<
                JoinFill<
                    Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            AnvilProvider<RootProvider<BoxTransport>, BoxTransport>,
            BoxTransport,
            Ethereum,
        >,
        Ethereum,
    >
{
    pub async fn anvil_new(url: Url, token: Address) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet_and_config(|anvil| anvil.fork(url));
        let token = ERC20::new(token, provider.clone());

        Ok(Self { provider, token })
    }
}
