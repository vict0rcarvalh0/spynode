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
The main components of the code are:
- `setup_logging`: Configures the logging for the application.
- `create_gossip_socket`: Creates and configures a UDP socket for the gossip service.
- `main`: Initializes and runs the Solana gossip service.

### Key Functions
- `setup_logging`: Sets up the logging configuration.
- `create_gossip_socket`: Creates a UDP socket bound to "127.0.0.1:8001" and sets it to non-blocking mode.
- `main`: Initializes the node, sets up the gossip service, and configures the cluster information.

### Key Modules
- `solana-client`: Contains modules for connection caching, non-blocking operations, transaction execution, and RPC communication.
- `solana-gossip`: Contains the gossip service implementation and related tests.

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