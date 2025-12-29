# Ferricord

A high-performance Discord API wrapper combining Python's ease of use with Rust's speed and memory efficiency.

## Features

- **discord.py-compatible API**: Familiar interface for easy migration from discord.py
- **Memory efficient**: 50-70% lower memory footprint compared to pure Python implementations
- **Scalable**: Designed for large-scale bots with thousands of guilds
- **Type safe**: Strict type checking on both Rust and Python sides
- **Modern**: Supports Discord API v10, Gateway Intents, Slash Commands

## Requirements

- Python 3.11 or higher
- No additional dependencies required (Rust binary is bundled)

## Installation

```bash
pip install ferricord
```

## Quick Start

```python
from ferricord import Client, Intents

# Create intents
intents = Intents.default()
intents.message_content = True

# Create client
client = Client(intents=intents)

@client.event
async def on_ready():
    print(f"Logged in as {client.user}")

@client.event
async def on_message(message):
    if message.content == "!ping":
        await message.channel.send("Pong!")

# Run the bot
client.run("YOUR_BOT_TOKEN")
```

## Async Usage

```python
import asyncio
from ferricord import Client, Intents

async def main():
    intents = Intents.default()
    client = Client(intents=intents)

    @client.event
    async def on_ready():
        print(f"Logged in as {client.user}")

    await client.start("YOUR_BOT_TOKEN")

    # Do other async work...

    await client.close()

asyncio.run(main())
```

## Gateway Intents

Ferricord supports all Discord Gateway Intents:

```python
from ferricord import Intents

# Default intents (non-privileged)
intents = Intents.default()

# All intents (including privileged)
intents = Intents.all()

# Custom intents
intents = Intents.none()
intents.guilds = True
intents.guild_messages = True
intents.message_content = True  # Privileged intent
```

## Architecture

Ferricord is built as a hybrid Rust/Python library:

```
Python User Code
       |
Python Helper Layer (decorators, event loop)
       |
PyO3 Binding Layer (Client, Message, Guild...)
       |
Rust Core (Gateway, HTTP, Cache)
       |
Tokio Async Runtime
       |
Discord API
```

### Crates

| Crate | Description |
|-------|-------------|
| `ferricord-core` | Common utilities and error types |
| `ferricord-model` | Discord API model definitions |
| `ferricord-http` | REST API client with rate limiting |
| `ferricord-gateway` | WebSocket Gateway client |
| `ferricord-cache` | Flexible caching layer |
| `ferricord-voice` | Voice support (Phase 3) |
| `ferricord-py` | PyO3 Python bindings |

## Development

### Prerequisites

- Rust 1.75+
- Python 3.11+
- maturin

### Building from Source

```bash
# Clone the repository
git clone https://github.com/FreeWiFi7749/ferricord.git
cd ferricord

# Build and install in development mode
pip install maturin
maturin develop

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings

# Format code
cargo fmt --all
```

### Project Structure

```
ferricord/
├── Cargo.toml              # Workspace configuration
├── pyproject.toml          # Python package configuration
├── ferricord-core/         # Core utilities
├── ferricord-model/        # Discord API models
├── ferricord-http/         # HTTP client
├── ferricord-gateway/      # Gateway client
├── ferricord-cache/        # Cache layer
├── ferricord-voice/        # Voice support (stub)
├── ferricord-py/           # Python bindings
└── python/                 # Python helper modules
```

## Current Status (Phase 1 MVP)

### Available Features
- Gateway connection (single shard)
- Event handling: `on_ready`, `on_message`, `on_guild_join`, `on_channel_create/update/delete`, `on_member_join/remove`, `on_typing`
- Guild, Channel, User, Message caching
- HTTP client with rate limiting

### Coming Soon (Phase 2+)
- Message sending via REST API
- Slash commands, components, modals
- Multi-shard support (AutoShardedClient)
- Cogs system
- Voice connection with DAVE protocol

## Roadmap

### Phase 1 (MVP) - Current
- Basic Gateway connection
- Core models (User, Guild, Channel, Message)
- HTTP client with rate limiting
- Python bindings with @client.event decorator

### Phase 2 (Feature Expansion)
- Multi-shard support (AutoShardedClient)
- Slash commands, components, modals
- Cogs system
- Advanced cache policies
- Metrics and monitoring

### Phase 3 (Voice & Optimization)
- Voice connection with DAVE protocol support
- Performance optimization
- Documentation

### Phase 4 (Stabilization)
- Beta testing
- Bug fixes
- 1.0 release

## Platform Support

| Platform | Architecture |
|----------|--------------|
| Linux | x86_64, aarch64 |
| macOS | x86_64, Apple Silicon |
| Windows | x86_64 |

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Discord Developer Portal](https://discord.com/developers/docs)
- [PyO3 Documentation](https://pyo3.rs)
