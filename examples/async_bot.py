"""
Async Bot Example for Ferricord

This example demonstrates how to use Ferricord with asyncio for more
control over the bot lifecycle.

Usage:
    1. Set your bot token as an environment variable:
       export DISCORD_TOKEN="your_bot_token_here"
    
    2. Run the bot:
       python examples/async_bot.py
"""

import asyncio
import os
from ferricord import Client, Intents


async def main():
    # Get token from environment variable
    token = os.environ.get("DISCORD_TOKEN")
    if not token:
        print("Error: DISCORD_TOKEN environment variable not set")
        return

    # Create intents
    intents = Intents.default()
    intents.message_content = True

    # Create client
    client = Client(intents=intents)

    @client.event
    async def on_ready():
        print(f"Bot is ready! Connected to {client.guild_count} guild(s)")

    @client.event
    async def on_message(message):
        print(f"Message received: {message.content}")

    print("Starting bot with asyncio...")
    
    # Start the bot asynchronously
    await client.start(token)
    
    # Keep the bot running
    try:
        while True:
            await asyncio.sleep(1)
    except KeyboardInterrupt:
        print("\nStopping bot...")
        await client.close()


if __name__ == "__main__":
    asyncio.run(main())
