use ethers::prelude::*;

#[tokio::main]
async fn main() {
    // Connect to Ethereum
    let http = Http::new("http://localhost:8545").expect("failed to connect to Ethereum node");
    let provider = Provider::<Http>::new(http);
    let wallet = Wallet::new(provider, LocalWallet::new(&PrivateKey::gen()));

    // Deploy and interact with smart contracts
    let token_factory = ContractFactory::new(
        include_bytes!("../artifacts/Token.json"),
        wallet.clone(),
    )
    .deploy("CustomToken".to_string(), "CT".to_string())
    .await
    .expect("failed to deploy token contract");

    let nft_factory = ContractFactory::new(
        include_bytes!("../artifacts/NFT.json"),
        wallet.clone(),
    )
    .deploy("CustomNFT".to_string(), "CNFT".to_string())
    .await
    .expect("failed to deploy NFT contract");

}
