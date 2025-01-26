use soroban_sdk::{contracttype, Address, BytesN, Env, Symbol};

#[contracttype]
pub struct NFTMetadata {
    pub buyer: Address,
    pub seller: Address,
    pub amount: u64,
    pub product: BytesN<32>,
    pub timestamp: u64,
}

pub fn mint_nft(env: &Env, buyer: &Address, tx_id: BytesN<32>, seller: &Address, amount: u64, product: &BytesN<32>) {
    // Check if NFT already exists
    if env.storage().instance().has(&tx_id) {
        panic!("NFT already exists for this transaction");
    }

    // Validate amount
    if amount == 0 {
        panic!("Amount must be greater than zero");
    }

    let timestamp = env.ledger().timestamp();
    let metadata = NFTMetadata {
        buyer: buyer.clone(),
        seller: seller.clone(),
        amount,
        product: product.clone(),
        timestamp,
    };

    // Store metadata in contract instance storage
    env.storage().instance().set(&tx_id, &metadata);

    // Emit an event for tracking the mint operation
    // Limit sensitive data in public events
    env.events().publish(
        (Symbol::new(env, "nft_minted"), tx_id.clone()),  // Event key
        tx_id.clone(),  // Event data limited to tx_id only
    );
}
