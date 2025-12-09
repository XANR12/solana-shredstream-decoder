# Solana Shredstream Decoder - Multi-Protocol Edition

This is an high-performance Rust client that decodes Solana shred packets directly from the Turbine block propagation layer. Unlike Geyser RPC or standard RPC requests, this decoder ensures you receive transactions the instant they are propagated by the leader validator, providing an unmatched speed advantage for time-sensitive trading operations.

This advanced version supports **11 major Solana DeFi protocols** with comprehensive instruction decoding, Address Lookup Table (ALT) resolution, and intelligent caching mechanisms.

# Supported Protocols & Instructions

## 1. **Pumpfun** (`6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`)
## 2. **Raydium AMM** (`675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8`)
## 3. **Raydium CPMM** (`CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C`)
## 4. **Raydium LaunchLab** (`LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj`)
## 5. **Moonit** (`MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG`)
## 6. **Boop** (`boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4`)
## 7. **PumpAMM** (`pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`)
## 8. **Meteora VCurve** (`dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN`)
## 9. **Meteora DYN** (`Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB`)
## 10. **Meteora AMM V2** (`cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG`)
## 11. **Orca Whirlpool** (`whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc`)

# Advanced Features

## Core Performance Optimizations

- **Jemalloc Memory Allocator** – Memory management with reduced fragmentation and improved allocation performance compared to default allocator
- **Optimized UDP Buffer (256KB)** – Prevents packet loss by configuring `SO_RCVBUF` to handle high-volume shred bursts without dropping packets
- **Physical CPU Thread Pool** – Rayon thread pool configured to use `num_cpus - 1` physical cores, reserving one for system operations
- **Parallel Transaction Processing** – Leverages Rayon for CPU-bound deserialization for maximum throughput
- **Async I/O with Tokio** – Non-blocking network operations for receiving shreds and serving gRPC streams
- **DashMap Concurrent Storage** – Lock-free concurrent hash maps for FEC blocks and processed block tracking
- **Zero-Copy Shred Handling** – Efficient `Box<[u8]>` usage to minimize memory allocations

## Reed-Solomon FEC Recovery

- **Automatic Data Shred Recovery** – When data shreds are missing, the decoder uses coding shreds to reconstruct the complete payload
- **Shredder Integration** – Uses Solana's ReedSolomon recovery erasure code processing
- **Complete Block Reconstruction** – Ensures 99%+ transaction capture rate even with packet loss
- **Intelligent Recovery Logging** – Detailed logs showing recovery attempts and success rates per FEC set

## Address Lookup Table (ALT) Cache System

- **RPC-Backed Cache** – Fetches and caches Address Lookup Tables from RPC endpoint with configurable TTL (default: 300 seconds)
- **Automatic Refresh on Extension** – Detects `ExtendLookupTable` instructions and invalidates cache to prevent stale data
- **Out-of-Bounds Protection** – Automatically refreshes lookup tables when indices exceed current size
- **Fallback Mechanism** – Uses stale cache entries if RPC fetch fails, preventing transaction loss
- **Async Resolution** – Pre-resolves all lookup tables before parallel processing to maintain async context
- **V0 Transaction Support** – Full support for versioned transactions with address table lookups

## Intelligent Garbage Collection

- **Periodic FEC Block Cleanup** – Removes FEC blocks older than 20 seconds every 10 seconds
- **Processed Block Tracking** – DashSet-based tracking prevents duplicate processing

## gRPC Streaming Server

- **Sub-Millisecond Latency** – Broadcasts decoded transactions via gRPC with <0.1ms latency on localhost
- **Broadcast Channel Architecture** – 1000-message buffer with `tokio::sync::broadcast` for multiple subscribers
- **Tonic Framework** – Production-ready gRPC implementation with Protocol Buffers
- **Automatic Stream Management** – Handles client connections, disconnections, and backpressure
- **Timestamp Precision** – Microsecond-precision timestamps for each transaction

## Comprehensive Logging & Debugging

- **Debug Mode Enhancements** – Thread IDs, names, line numbers, and targets in debug builds
- **Performance Metrics** – Detailed timing for shred collection, FEC completion, and transaction extraction
- **Slot Statistics** – Tracks FEC blocks processed, transactions extracted, and completion rates per slot
- **Error Context** – Rich error messages with full context for debugging protocol-specific issues

## Transaction Deserialization

- **Standardized Output Format** – Uniform JSON structure across all protocols with:
  - Program ID and instruction name
  - Protocol identifier
  - Raw instruction data (hex)
  - Mapped accounts with signer/writable flags
  - Parsed instruction arguments
- **Borsh Deserialization** – Efficient binary deserialization for all instruction types
- **Account Safety Checks** – Validates account indices before access to prevent panics
- **Unknown Account Handling** – Gracefully handles missing accounts with "Unknown" placeholders

# Architecture Highlights

## Shred Processing Pipeline

1. **Packet Reception** – Receives raw shred packets from configured source
2. **Validation & Assembly** – Validates and assembles shreds into complete blocks
3. **Data Recovery** – Reconstructs missing data using advanced error correction
4. **Transaction Extraction** – Extracts and deserializes transactions from assembled blocks
5. **Protocol Decoding** – Identifies and decodes protocol-specific instructions
6. **Output Streaming** – Broadcasts decoded transactions via gRPC to connected clients

## Memory Management

- **Jemalloc Global Allocator** – Optimized for multi-threaded workloads
- **Arc-Based Sharing** – Shared ownership of FEC blocks and sockets across tasks
- **Scoped Lifetimes** – Automatic cleanup when FEC blocks complete
- **Bounded Channels** – Prevents unbounded memory growth in broadcast queue

# Performance Characteristics

- **Throughput** – Handles 1000+ transactions per slot without problems
- **CPU Utilization** – Scales linearly with available physical cores
- **Memory Footprint** – ~200-500MB typical usage with automatic GC
- **Cache Hit Rate** – 95%+ ALT cache hits after warm-up period

# Who Should Use This?

- **MEV Searchers** – Fastest possible transaction feed for arbitrage and liquidations
- **Sniper Bots** – Catch token launches across 11 protocols instantly
- **Market Makers** – Real-time pool creation and liquidity events
- **Validators** – Decode shreds locally without external RPC dependencies
- **Analytics Platforms** – Comprehensive multi-protocol transaction indexing
- **Trading Firms** – Production-grade infrastructure for high-frequency strategies

# Setup & Configuration

## Environment Variables

Create a `.env` file with the following variables:

```bash
# RPC endpoint example for Address Lookup Table resolution
RPC_ENDPOINT=https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY

# ALT cache TTL in seconds (default: 300)
LOOKUP_TABLE_CACHE_TTL_SECONDS=300

# UDP socket for receiving shreds
# Use 8001 or 8002 depending on Jito Shredstream Proxy configuration
# Or use your validator's TVU port for direct connection
UDP_BUFFER_SOCKET=0.0.0.0:8002

# gRPC server endpoint for streaming transactions
GRPC_SERVER_ENDPOINT=0.0.0.0:50051
```

## Build & Run

### Optimized Release Build

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Run with Info Logging

```bash
RUST_LOG=info cargo run --release
```

### Run with Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

## Compiler Optimizations

The release profile in `Cargo.toml` is configured for maximum performance:

```toml
[profile.release]
opt-level = 3           # Maximum optimizations
lto = "fat"             # Link-time optimization across all crates
codegen-units = 1       # Single codegen unit for better optimization
panic = "abort"         # Smaller binary, faster panics
strip = true            # Remove debug symbols
debug = false           # No debug info in release
```

# Output Format

Transactions are streamed as JSON with the following structure:

```json
{
  "signatures": ["5Xm..."],
  "slot": 123456789,
  "message": {
    "header": {
      "numRequiredSignatures": 1,
      "numReadonlySignedAccounts": 0,
      "numReadonlyUnsignedAccounts": 5
    },
    "recentBlockhash": "9Xm...",
    "instructions": [
      {
        "program_id": "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
        "instruction_name": "Create",
        "protocol": "Pumpfun",
        "raw_data": "181ec828051c0777...",
        "accounts": [
          {
            "index": 0,
            "pubkey": "7Xm...",
            "signer": true,
            "writable": true
          }
        ],
        "parsed_data": {
          "name": "MyToken",
          "symbol": "MTK",
          "uri": "https://...",
          "creator": "7Xm..."
        }
      }
    ]
  }
}
```

# Monitoring & Debugging

## Log Levels

- **ERROR** – Critical failures (RPC errors, deserialization failures)
- **WARN** – Recoverable issues (missing accounts, cache misses)
- **INFO** – Key events (FEC completion, cache initialization, GC stats)
- **DEBUG** – Detailed tracing (shred collection, timing metrics)

## Performance Tuning

- Increase `UDP_BUFFER_SOCKET` buffer size if experiencing packet loss
- Adjust `LOOKUP_TABLE_CACHE_TTL_SECONDS` based on network latency
- Monitor CPU usage and adjust Rayon thread pool if needed

# Source Code

If you are really interested in the source code, please contact me for details and demo on Discord: `.xanr`
