use blockchain_maker::Blockchain;
use blockchain_maker::count_files_in_folder;

fn main() {
    // Attempt to load the blockchain from disk
    let mut blockchain: Blockchain = match Blockchain::load_chain_from_disk("my_blocks".to_string()) {
        Ok(chain) => chain,
        Err(e) => {
            // Handle the error e.g., by logging or creating a new, empty blockchain
            println!("Failed to load chain from disk, error: {}", e);
            // Potentially initialize a new, empty blockchain here if desired
            Blockchain::new() // This assumes you have a `new` method to create an empty blockchain
        },
    };

    // Validate the loaded or new blockchain
    if blockchain.validate_chain("my_blocks".to_string()) {
        println!("Blockchain validated successfully.");
    } else {
        println!("Blockchain validation failed.");
    }

    // Add new blocks to the blockchain
    if let Err(e) = blockchain.add_block("Block 1 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }

    if let Err(e) = blockchain.add_block("Block 2 Transactions Data".to_string()) {
        println!("Failed to add block: {}", e);
    }

    // Print out the current state of the blockchain or other relevant information
    // This might involve iterating over the blocks and printing them out, 
    // or simply printing out the number of blocks in the chain
    println!("Current blockchain size: {}", count_files_in_folder("my_blocks".to_string()).unwrap());
}
