use std::{
    collections::HashSet,
    net::SocketAddr,
    process::exit,
    sync::{Arc, atomic::AtomicBool},
};
mod utils;
use solana_client::connection_cache::ConnectionCache;
use solana_gossip::{
    cluster_info::{ClusterInfo, Node},
    contact_info::ContactInfo,
    gossip_service::GossipService,
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, timing::timestamp};
use solana_streamer::socket::SocketAddrSpace;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, prelude::*};

/// Sets up the logging system for the application.
/// This function creates the "logs" directory, configures a file appender with daily rotation,
/// sets up an environment filter to capture logs at different levels and from specific modules,
/// and initializes a subscriber that outputs logs to both the console and a file.
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create the "logs" directory if it doesn't exist
    std::fs::create_dir_all("logs")?;

    // Create a rolling file appender that writes logs to "logs/solana-gossip.log" and rotates daily
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "solana-gossip.log");

    // Set up an environment filter for logging; if no environment variable is set, use a default filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,solana_gossip=trace,solana_cluster_info=trace"));

    // Build a subscriber with two layers:
    // 1. A layer that writes detailed logs (including file, line, thread ID, target) to the file.
    // 2. A compact layer for console output.
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true) // Include the file name in logs
                .with_line_number(true) // Include line numbers
                .with_thread_ids(true) // Include thread IDs
                .with_target(true) // Include target (module) information
                .with_writer(file_appender) // Write logs to the configured file appender
                .with_ansi(false), // Disable ANSI colors for file logs
        )
        .with(tracing_subscriber::fmt::layer().compact()) // Compact formatting for console logs
        .init();

    Ok(())
}

/// Creates and configures a UDP socket for the gossip service.
/// The socket is bound to "127.0.0.1:8001" and is set to non-blocking mode.
fn create_gossip_socket() -> std::net::UdpSocket {
    // Bind the UDP socket to the specified address and port
    let socket = std::net::UdpSocket::bind("127.0.0.1:8001").expect("Failed to bind gossip socket");
    // Set the socket to non-blocking mode for asynchronous operations
    socket.set_nonblocking(true).unwrap();
    socket
}

/// Main function: initializes and runs the Solana gossip service.
fn main() {
    // Initialize the logging system; if an error occurs, print it and exit the application.
    setup_logging().unwrap_or_else(|err| {
        eprintln!("Failed to set up logging: {}", err);
        exit(1);
    });

    // Log an informational message indicating that the gossip service is starting.
    info!("Starting Solana gossip service...");

    // Generate a new keypair for the validator and wrap it in an Arc for thread-safe sharing.
    let id_pair = Arc::new(Keypair::new());
    // Define the shred version for the node (used in Solana's data transmission protocol)
    let shred_version: u16 = 27799;
    // List of entrypoint URLs for the testnet cluster
    let entrypoint_urls: Vec<&str> = vec![
        "entrypoint.testnet.solana.com:8001",
        "entrypoint2.testnet.solana.com:8001",
        "entrypoint3.testnet.solana.com:8001",
    ];

    // Convert entrypoint URLs to SocketAddr using a helper function from the utils module
    let entrypoint_addrs = entrypoint_urls
        .iter()
        .map(|addr| utils::parse_host_port(addr).unwrap())
        .collect::<Vec<SocketAddr>>();

    // Create a vector of indices corresponding to each entrypoint address
    let mut order: Vec<_> = (0..entrypoint_addrs.len()).collect();

    // Attempt to determine the validator's public IP address by contacting each entrypoint.
    let gossip_host = order.into_iter().find_map(|i| {
        let entrypoint_addr = &entrypoint_addrs[i];
        // Log which entrypoint is being contacted to determine the public IP address.
        info!(
            "Contacting {} to determine validator's public IP address",
            entrypoint_addr
        );

        // Use a utility function to get the public IP address.
        // If it fails, print an error message and continue to the next entrypoint.
        solana_net_utils::get_public_ip_addr(entrypoint_addr).map_or_else(
            |err| {
                eprintln!("Failed to contact cluster entrypoint {entrypoint_addr}: {err}");
                None
            },
            Some,
        )
    });

    // If no public IP address could be determined, print an error and exit.
    let gossip_host = gossip_host.unwrap_or_else(|| {
        eprintln!("Unable to determine the validator's public IP address");
        exit(1)
    });

    // Create a local Node instance using localhost settings and the validator's public key.
    let mut node = Node::new_localhost_with_pubkey(&id_pair.try_pubkey().unwrap());
    // Update the node's information with the current public key.
    node.info.hot_swap_pubkey(id_pair.pubkey());
    // Set the shred version in the node's information.
    node.info.set_shred_version(shred_version);
    // Set the node's wall clock to the current timestamp.
    node.info.set_wallclock(timestamp());

    // Create an independent UDP socket for gossip communication.
    let gossip_socket = create_gossip_socket();
    // Get the local address of the gossip socket.
    let gossip_addr = gossip_socket.local_addr().unwrap();
    // Update the node's information with the gossip socket address.
    node.info.set_gossip(gossip_addr).unwrap();

    // Create a ClusterInfo instance to maintain information about the cluster.
    // It uses the node's info, the validator's keypair, and a socket address space configuration.
    let mut cluster_info = ClusterInfo::new(node.info, id_pair, SocketAddrSpace::new(false));

    // Build a list of entrypoint contact information from the entrypoint addresses.
    let cluster_entrypoints = entrypoint_addrs
        .iter()
        .map(ContactInfo::new_gossip_entry_point)
        .collect::<Vec<_>>();
    // Set the interval (in milliseconds) for contact information debug logging.
    cluster_info.set_contact_debug_interval(120000);
    // Configure the cluster's entrypoints.
    cluster_info.set_entrypoints(cluster_entrypoints);
    // Wrap cluster_info in an Arc for thread-safe sharing.
    let cluster_info = Arc::new(cluster_info);
    // Log the peers known in the cluster.
    info!("Cluster info peers: {:?}", cluster_info.all_peers());

    // Create a connection cache using the QUIC protocol with a specified cache name and capacity.
    let connection_cache = ConnectionCache::new_quic("connection_cache_banking_bench_quic", 1);
    // Create an atomic flag (initialized to false) that will signal when the service should exit.
    let exit = Arc::new(AtomicBool::new(false));

    // Initialize an empty set for storing the public keys of gossip validators (currently empty).
    let gossip_validators: HashSet<Pubkey> = HashSet::new();
    info!("Starting gossip service...");

    // Start the gossip service:
    // - Pass in the cluster_info so the service knows about cluster peers.
    // - Use the independent gossip socket for communication.
    // - The 'true' flag indicates that the service should connect to entrypoints if necessary.
    // - Pass the exit flag to allow for graceful shutdown of the service.
    let gossip_service = GossipService::new(
        &cluster_info,
        None,
        gossip_socket, // Use the dedicated gossip socket
        None,
        true,
        None,
        exit.clone(),
    );

    // Block the main thread until the gossip service terminates.
    gossip_service.join();
}
