"""
Basic Bot Example for Ferricord

This example demonstrates the basic usage of Ferricord to connect to Discord
and listen for events. Currently in Phase 1 MVP, this bot can:
- Connect to Discord Gateway
- Receive and log events (ready, message, guild_join, etc.)

Note: Message sending is not yet implemented (coming in Phase 2).

Usage:
    1. Set your bot token as an environment variable:
       export DISCORD_TOKEN="your_bot_token_here"
    
    2. Run the bot:
       python examples/basic_bot.py
"""

import os
from ferricord import Client, Intents


def main():
    # Get token from environment variable
    token = os.environ.get("DISCORD_TOKEN")
    if not token:
        print("Error: DISCORD_TOKEN environment variable not set")
        print("Usage: export DISCORD_TOKEN='your_bot_token_here'")
        return

    # Create intents - enable message_content to receive message content
    intents = Intents.default()
    intents.message_content = True
    intents.guild_messages = True
    intents.guilds = True

    # Create client
    client = Client(intents=intents)

    @client.event
    async def on_ready():
        """Called when the bot successfully connects to Discord."""
        print("=" * 50)
        print("Bot is ready!")
        if client.user:
            print(f"Logged in as: {client.user}")
        print(f"Connected to {client.guild_count} guild(s)")
        print("=" * 50)

    @client.event
    async def on_message(message):
        """Called when a message is received."""
        print(f"[MESSAGE] #{message.channel_id}: {message.content}")
        
        # Example: Detect a command (sending response not yet implemented)
        if message.content == "!ping":
            print("  -> Received ping command! (Response not yet implemented)")

    @client.event
    async def on_guild_join(guild):
        """Called when the bot joins a new guild."""
        print(f"[GUILD JOIN] Joined guild: {guild.name} (ID: {guild.id})")

    @client.event
    async def on_guild_update(guild):
        """Called when a guild is updated."""
        print(f"[GUILD UPDATE] Guild updated: {guild.name}")

    @client.event
    async def on_channel_create(channel):
        """Called when a channel is created."""
        print(f"[CHANNEL CREATE] New channel: {channel.name} (ID: {channel.id})")

    @client.event
    async def on_channel_update(channel):
        """Called when a channel is updated."""
        print(f"[CHANNEL UPDATE] Channel updated: {channel.name}")

    @client.event
    async def on_channel_delete(channel):
        """Called when a channel is deleted."""
        print(f"[CHANNEL DELETE] Channel deleted: {channel.name}")

    @client.event
    async def on_member_join(guild_id):
        """Called when a member joins a guild."""
        print(f"[MEMBER JOIN] New member in guild {guild_id}")

    @client.event
    async def on_member_remove(guild_id, user):
        """Called when a member leaves a guild."""
        print(f"[MEMBER REMOVE] Member left guild {guild_id}: {user}")

    @client.event
    async def on_typing(channel_id, user_id):
        """Called when a user starts typing."""
        print(f"[TYPING] User {user_id} is typing in channel {channel_id}")

    print("Starting bot...")
    print("Press Ctrl+C to stop")
    
    try:
        client.run(token)
    except KeyboardInterrupt:
        print("\nBot stopped by user")
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
