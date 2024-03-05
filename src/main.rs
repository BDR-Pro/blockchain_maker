use blockchain_maker::blockchain::Blockchain;
 fn main() {
    // Load the blockchain from disk, replacing the new, empty blockchain
    let mut blockchain = Blockchain::load_chain_from_disk().expect("Failed to load chain from disk");

    // Validate the loaded blockchain
    blockchain.validate_chain();

    // Add new blocks to the blockchain
    blockchain.add_block("Block 1 Transactions Data".to_string()).expect("Failed to add block");
    blockchain.add_block("Block 2 Transactions Data".to_string()).expect("Failed to add block");

    // Print out the current state of the blockchain
}

