# Rust Blockchain Maker Package 🚀

Welcome to the Rust Blockchain Maker Package! This is your ultimate toolkit for building, validating, and managing a blockchain in Rust. Below, you'll find everything you need to know to integrate and leverage our blockchain functionality in your Rust projects.

## Dependencies 📦

Before diving into the blockchain wonders, make sure to include the following dependencies in your `Cargo.toml`:

```toml
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
sha2 = "0.10"
openssl = "0.10"
serde_json = "1.0"
```

These dependencies will ensure proper handling of time, serialization, cryptographic functions, and JSON formatting.

## Getting Started 🌟

Import the necessary modules from our package and standard Rust libraries to start crafting your blockchain:

```rust
use blockchain_maker::Blockchain; // Our primary Blockchain structure.
//instead of all of this
use chrono::prelude::*; // For handling timestamps.
use serde::{Deserialize, Serialize}; // For data serialization.
use sha2::{Sha256, Digest}; // For creating cryptographic hashes.
use openssl::ec::{EcGroup, EcKey}; // For elliptic curve cryptography.
```

## Core Functionalities 🛠

Our package provides a variety of functions to support your blockchain adventures:

### `get_block_hash_from_file(path: P) -> Result<String, Box<dyn std::error::Error>>`

Retrieves the hash of a blockchain block from a specified file.

- **Parameters**: `path: P` - A reference to the file path containing the block.
- **Returns**: A `Result` containing the block hash as a `String` if successful.

### `count_files_in_folder(path: P) -> std::io::Result<usize>`

Counts the files in a specified directory, aiding in block management.

- **Parameters**: `path: P` - A reference to the directory path.
- **Returns**: A `Result` with the count of files.

### `sign(message: &str, reward: u64, block_number: u64) -> (Vec<u8>, Vec<u8>)`

Generates a digital signature for a given message using ECDSA.

- **Parameters**: 
    - `message: &str` - The message to sign.
    - `reward: u64` - The mining reward.
    - `block_number: u64` - The number of the block.
- **Returns**: A tuple containing the signature and public key.

## Structures 🏗

Our primary structures include `Block` and `Blockchain`, designed to encapsulate all necessary blockchain data and functionalities:

### `Block`

Represents a single block in the blockchain, containing attributes like `timestamp, data, previous_hash, etc.`

### `Blockchain`

Manages the chain of blocks, providing methods for adding `new blocks, validating the chain, and calculating rewards.`

## Usage Example 📝

Here's how you can utilize our package to create and manage a blockchain:

```rust
fn main() {
    let mut blockchain: Blockchain = Blockchain::new();

    // Load the blockchain from disk (if available)
    match Blockchain::load_chain_from_disk("my_blocks".to_string()) {
        Ok(chain) => blockchain = chain,
        Err(e) => println!("Failed to load chain from disk: {}", e),
    };

    // Validate the blockchain
    if blockchain.validate_chain("my_blocks".to_string()) {
        println!("Blockchain validated successfully.");
    } else {
        println!("Blockchain validation failed.");
    }

    // Add new blocks
    blockchain.add_block("Block 1 Transactions Data".to_string()).expect("Failed to add block");
    blockchain.add_block("Block 2 Transactions Data".to_string()).expect("Failed to add block");

    // Print the current size of the blockchain
    println!("Current blockchain size: {}", blockchain.chain.len());
}
```

This example demonstrates loading a blockchain from disk, validating it, adding new blocks, and displaying the blockchain size.

## Contribution and Support 🤝

Contributions are welcome! If you'd like to contribute or have found bugs, please open an issue or pull request on our GitHub repository. For support, feel free to reach out via our support channels.

Dive into blockchain development with our Rust Blockchain Maker Package! Happy coding! 🚀