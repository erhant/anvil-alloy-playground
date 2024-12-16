//! Example of deploying a contract from Solidity code using the `sol!` macro to Anvil and
//! interacting with it.

use alloy::{
    primitives::{address, Address, U256},
    providers::{ext::AnvilApi, ProviderBuilder, WalletProvider},
    sol,
};
use eyre::Result;

// Codegen from embedded Solidity code and precompiled bytecode.
sol! {
    #[allow(missing_docs)]
    // solc ./contracts/Owner.sol --via-ir --optimize --bin
    #[sol(rpc, bytecode="608034607457601f61019a38819003918201601f19168301916001600160401b03831184841017607857808492602094604052833981010312607457516001600160a01b0381169081900360745760046001555f80546001600160a01b03191691909117905560405161010d908161008d8239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60808060405260043610156011575f80fd5b5f3560e01c9081638da5cb5b1460b7575063e73620c314602f575f80fd5b3460b357602036600319011260b3575f546001600160a01b03163303607f576001546004358101809111606b5780602091600155604051908152f35b634e487b7160e01b5f52601160045260245ffd5b60405162461bcd60e51b815260206004820152600c60248201526b3737ba1030b71037bbb732b960a11b6044820152606490fd5b5f80fd5b3460b3575f36600319011260b3575f546001600160a01b03168152602090f3fea26469706673582212203793daad5f3b4b5aff9159607eff1dbd4d232b27d51254769ca08d8338641c2464736f6c634300081a0033")]
    contract Ownable {
        address public owner;
        uint value = 4;

        constructor(address _owner) {
            owner = _owner;
        }

        function callMe(uint val) external returns (uint) {
            require(owner == msg.sender, "not an owner");
            value += val;
            return value;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    const OWNER: Address = address!("aaaabbbbccccddddeeeeffffaaaabbbbccccdddd");
    let value = U256::from(10);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet();

    let contract = Ownable::deploy(&provider, OWNER).await?;
    println!("Contract address:: {}", contract.address());
    assert_eq!(contract.owner().call().await?.owner, OWNER);

    // this is only callable by the owner!
    let result = contract.callMe(value).from(OWNER).call().await?;
    assert_eq!(result._0, value + U256::from(4));

    // this shall fail
    let result = contract.callMe(value).call().await?;
    assert_eq!(result._0, value + U256::from(4));

    Ok(())
}
