use alloy::{
    network::TransactionBuilder,
    primitives::{address, utils::format_ether, Address, U256},
    providers::{ext::AnvilApi, Provider, WalletProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use anvil_alloy_playground::TokenClient;

// use a public RPC url
// https://base-rpc.publicnode.com (https://chainlist.org/chain/84532)
// https://base-sepolia-rpc.publicnode.com (https://chainlist.org/chain/8453)
const RPC_URL: &str = "https://base-rpc.publicnode.com";

#[tokio::test]
async fn test_token() -> eyre::Result<()> {
    let client = TokenClient::new(
        PrivateKeySigner::random().into(),
        RPC_URL.parse()?,
        address!("4200000000000000000000000000000000000006"),
    )
    .await?;

    let (balance, symbol) = client
        .balance(address!("349cCe6Efd353C92445d7a0730D5B24548aE4534"))
        .await?;
    println!("Balance: {} {}", format_ether(balance), symbol);

    Ok(())
}

#[tokio::test]
async fn test_anvil_dummy() -> eyre::Result<()> {
    let client = TokenClient::anvil_new(
        RPC_URL.parse()?,
        address!("4200000000000000000000000000000000000006"),
    )
    .await?;

    // set balance of some dummy address
    let dummy_address = address!("4209998887776665550000000000000000000006");
    let dummy_amount = U256::from(123456);
    assert_eq!(
        client.provider.get_balance(dummy_address).await?,
        U256::ZERO
    );
    client
        .provider
        .anvil_set_balance(dummy_address, dummy_amount)
        .await?;

    assert_eq!(
        client.provider.get_balance(dummy_address).await?,
        dummy_amount
    );

    println!("Bal: {}", client.provider.get_balance(dummy_address).await?);

    // we will impersonate the dummy account and send its money to zero
    client
        .provider
        .anvil_impersonate_account(dummy_address)
        .await?;
    let tx = TransactionRequest::default()
        .with_from(dummy_address)
        .with_to(Address::ZERO)
        .with_value(U256::from(100000));
    let tx = client.provider.send_transaction(tx).await?;
    let _ = tx.get_receipt().await?;
    client
        .provider
        .anvil_stop_impersonating_account(dummy_address)
        .await?;

    // print balance after
    println!("Bal: {}", client.provider.get_balance(dummy_address).await?);

    Ok(())
}
