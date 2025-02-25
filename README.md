# Spynode

## Problem
The goal of this project is to set up a node that listens to transactions but doesn’t participate in block building. The problem is that RPC nodes get data slower compared to validator nodes. Therefore, we need to simulate or impersonate a validator node to get block data and then forward it to RPC.

## Solution
The solution involves creating a node that mimics the behavior of a validator node without participating in the voting or block-building process. This node will listen to transactions and forward the data to RPC nodes to ensure they receive the data faster.

## Simulation
The node will:
- Simulate a validator node to get block data.
- Forward the data to RPC nodes.
- Avoid participating in the voting or block-building process.

## Code Overview
- **main.rs**  
  - Initializes logging.  
  - Creates a gossip socket and determines the node’s public IP.  
  - Boots up a Solana gossip service via `GossipService::new`.  
  - Declares a channel (tx, rx) pair of type `Transaction` (string).  
  - Spawns a thread that forwards received transactions to a configured RPC endpoint.  
  - Waits for the gossip service to finish.

- **utils.rs**  
  - Houses helper methods such as `parse_host_port` for entrypoint URLs.

## How to Run
1. Clone the repository:
    ```sh
    git clone https://github.com/vict0rcarvalh0/spynode.git
    cd spynode
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the project:
    ```sh
    cargo run
    ```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License
This project is licensed under the MIT License.