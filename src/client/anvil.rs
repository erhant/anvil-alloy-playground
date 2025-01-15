// TODO: feature gate

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::anvil,
    primitives::Address,
    providers::{
        ext::AnvilApi,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        layers::AnvilProvider,
        Identity, PendingTransactionBuilder, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::TransactionRequest,
    transports::{http::reqwest::Url, BoxTransport},
};
use eyre::{Context, Result};

use crate::{TokenClient, ERC20};

/// Core fillers are recommended fillers, along with a wallet filler for Ethereum.
type AnvilFiller = JoinFill<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    WalletFiller<EthereumWallet>,
>;
/// Anvil provider requires a `BoxTransport`.
type AnvilTransport = BoxTransport;
/// We are using Ethereum network.
type AnvilNetwork = Ethereum;

type AnvilFillProvider = FillProvider<
    AnvilFiller,
    AnvilProvider<RootProvider<AnvilTransport>, AnvilTransport>,
    AnvilTransport,
    AnvilNetwork,
>;

impl TokenClient<AnvilTransport, AnvilFillProvider, AnvilNetwork> {
    pub async fn anvil_new(url: Url, token: Address) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet_and_config(|anvil| anvil.fork(url));
        let token = ERC20::new(token, provider.clone());

        Ok(Self { provider, token })
    }

    #[inline]
    pub async fn anvil_impersonated_tx(
        &self,
        tx: TransactionRequest,
        from: Address,
    ) -> Result<PendingTransactionBuilder<AnvilTransport, AnvilNetwork>> {
        // create a provider, without any `WalletFiller` otherwise there is a bug with impersonations
        // see: https://github.com/alloy-rs/alloy/issues/1918
        let anvil_provider = ProviderBuilder::new().on_anvil();

        anvil_provider.anvil_impersonate_account(from).await?;
        let pending_tx = anvil_provider.send_transaction(tx.from(from)).await?;
        anvil_provider
            .anvil_stop_impersonating_account(from)
            .await?;

        Ok(pending_tx)
    }
}
