mod anvil;
mod connect;
mod core;
mod token;

/// An ERC20 token client.
#[derive(Debug, Clone)]
pub struct TokenClient<T, P, N>
where
    T: alloy::transports::Transport + ::core::clone::Clone,
    P: alloy::providers::Provider<T, N>,
    N: alloy::network::Network + alloy::providers::fillers::RecommendedFillers,
{
    /// Underlying provider type.
    pub provider: P,
    pub token: crate::ERC20::ERC20Instance<T, P, N>,
}
