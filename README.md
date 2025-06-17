🕵️‍♂️ Spynode
Spynode is a lightweight Solana node designed to simulate a validator for the purpose of accessing block and transaction data with minimal delay — without participating in consensus or block production. This allows faster relaying of real-time transaction data to downstream services like RPC nodes.

🚨 Problem
Traditional RPC nodes receive data with a slight delay compared to validator nodes due to their passive role in the network. This latency creates a gap for applications that rely on fast, real-time access to Solana transactions (e.g., bots, dashboards, monitoring tools).

✅ Solution
Spynode solves this by mimicking the networking behavior of a validator node to gain early access to transaction and block data — but without voting, staking, or participating in block production. This data is then forwarded to downstream services (e.g., RPC endpoints) in near real-time.

🧪 Simulation Design
The node:

Joins the Solana gossip network to observe transactions and block metadata.

Forwards the captured transaction data through a local channel to an external RPC handler.

Avoids all consensus participation: no voting, no staking, no leader scheduling.

🧠 Architecture Overview
text
Copy
Edit
 +----------------------+         +------------------+     
 | Solana Validator Set | <--->  |   Spynode (This) | -->  [RPC Listener / Forwarder]
 +----------------------+         +------------------+     
                                        |
                              [Transaction Channel]
                                        |
                              +--------------------+
                              |  RPC/Webhook Sink  |
                              +--------------------+
🧾 Code Overview
main.rs
Sets up logging and environment.

Parses entrypoint and determines public IP.

Initializes the GossipService to join the Solana cluster.

Creates a channel (tx, rx) for transferring transactions.

Spawns a forwarding thread to relay received data to an RPC/Webhook endpoint.

utils.rs
Utility functions including:

parse_host_port() for extracting IP and port from CLI input or config.

Other future helpers for IP resolution, timestamp formatting, etc.

⚙️ How to Use
1. Clone the repository:
bash
Copy
Edit
git clone https://github.com/vict0rcarvalh0/spynode.git
cd spynode
2. Build the project:
bash
Copy
Edit
cargo build --release
3. Run the node:
bash
Copy
Edit
cargo run --release
🔧 You may configure the RPC endpoint or entrypoint via command-line args or environment variables (WIP: CLI support).

🧠 Use Cases
Real-time transaction indexing

Trading bots needing fast mempool data

On-chain monitoring dashboards

Transaction stream analyzers

DeFi bots or arbitrage scanners

🔒 Security Considerations
This node does not stake or vote, and therefore holds no funds or private keys.

Ensure that forwarded data over the network is protected (e.g., use HTTPS or secure websockets for RPC).

Limit exposure to external RPC endpoints to avoid leaking transaction flow unnecessarily.

📌 Roadmap
 CLI support for custom RPC/webhook endpoints

 Transaction filtering (e.g., by program ID or address)

 Optional TLS/WebSocket support for secure forwarding

 JSON output option

 Dashboard or log viewer

 Docker container and deployment guide

🤝 Contributing
Contributions are welcome! Feel free to:

Open issues for bugs or suggestions

Fork the repo and submit pull requests

Add logging, metrics, filters, or CLI tools

🪪 License
This project is licensed under the MIT License.
