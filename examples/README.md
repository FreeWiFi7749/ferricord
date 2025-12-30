# Ferricord Examples

This directory contains example bots demonstrating Ferricord usage.

## Prerequisites

1. Build and install Ferricord:
   ```bash
   pip install maturin
   maturin develop
   ```

2. Set your Discord bot token:
   ```bash
   export DISCORD_TOKEN="your_bot_token_here"
   ```

## Examples

### basic_bot.py

A simple bot that connects to Discord and logs all events.

```bash
python examples/basic_bot.py
```

Features demonstrated:
- Creating a Client with Intents
- Event handlers (`on_ready`, `on_message`, `on_guild_join`, etc.)
- Blocking `client.run()` method

### async_bot.py

An async version using `asyncio` for more control over the bot lifecycle.

```bash
python examples/async_bot.py
```

Features demonstrated:
- Async `client.start()` method
- Manual event loop control
- Graceful shutdown with `client.close()`

## Current Limitations (Phase 1 MVP)

These examples can only **receive** events. The following features are not yet implemented:

- Sending messages
- Slash commands
- Reactions
- Voice

These will be added in Phase 2.

## Getting a Bot Token

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Go to "Bot" section and create a bot
4. Copy the token
5. Enable required intents (MESSAGE CONTENT INTENT for message content)

## Inviting Your Bot

Use this URL format to invite your bot to a server:
```
https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=0&scope=bot
```

Replace `YOUR_CLIENT_ID` with your application's client ID.
