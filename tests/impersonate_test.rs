use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{ext::AnvilApi, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};

const RPC_URL: &str = "https://base-rpc.publicnode.com";

// issue: https://github.com/alloy-rs/alloy/issues/1918

// Taken from: https://github.com/alloy-rs/alloy/blob/main/crates/provider/src/ext/anvil.rs#L334
#[tokio::test]
async fn test_anvil_impersonate_account_stop_impersonating_account() -> eyre::Result<()> {
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet(); //(|anvil| anvil.fork(RPC_URL));

    let impersonate = address!("4209998887776665550000000000000000000006");
    let to = address!("4209998887776665550000000000000000000007");
    let val = U256::from(1337);
    let funding = U256::from(1e18 as u64);

    provider.anvil_set_balance(impersonate, funding).await?;

    let balance = provider.get_balance(impersonate).await?;
    assert_eq!(balance, funding);

    let tx = TransactionRequest::default()
        .with_from(impersonate)
        .with_to(to)
        .with_value(val);

    let res = provider.send_transaction(tx.clone()).await;
    res.unwrap_err();

    provider
        .anvil_impersonate_account(impersonate)
        .await
        .unwrap();
    assert!(provider
        .get_accounts()
        .await
        .unwrap()
        .contains(&impersonate));

    let res = provider
        .send_transaction(tx.clone())
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    assert_eq!(res.from, impersonate);

    let nonce = provider.get_transaction_count(impersonate).await.unwrap();
    assert_eq!(nonce, 1);

    let balance = provider.get_balance(to).await.unwrap();
    assert_eq!(balance, val);

    provider
        .anvil_stop_impersonating_account(impersonate)
        .await
        .unwrap();
    let res = provider.send_transaction(tx).await;
    res.unwrap_err();

    Ok(())
}

// Taken from: https://github.com/alloy-rs/alloy/blob/main/crates/provider/src/ext/anvil.rs#L370
#[tokio::test]
async fn test_anvil_auto_impersonate_account() {
    let provider = ProviderBuilder::new().on_anvil();

    let impersonate = address!("4209998887776665550000000000000000000006");
    let to = address!("4209998887776665550000000000000000000007");
    let val = U256::from(1337);
    let funding = U256::from(1e18 as u64);

    provider
        .anvil_set_balance(impersonate, funding)
        .await
        .unwrap();

    let balance = provider.get_balance(impersonate).await.unwrap();
    assert_eq!(balance, funding);

    let tx = TransactionRequest::default()
        .with_from(impersonate)
        .with_to(to)
        .with_value(val);

    let res = provider.send_transaction(tx.clone()).await;
    res.unwrap_err();

    provider.anvil_auto_impersonate_account(true).await.unwrap();

    let res = provider
        .send_transaction(tx.clone())
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    assert_eq!(res.from, impersonate);

    let nonce = provider.get_transaction_count(impersonate).await.unwrap();
    assert_eq!(nonce, 1);

    let balance = provider.get_balance(to).await.unwrap();
    assert_eq!(balance, val);

    provider
        .anvil_auto_impersonate_account(false)
        .await
        .unwrap();
    let res = provider.send_transaction(tx).await;
    res.unwrap_err();

    provider
        .anvil_impersonate_account(impersonate)
        .await
        .unwrap();
    assert!(provider
        .get_accounts()
        .await
        .unwrap()
        .contains(&impersonate));
}
