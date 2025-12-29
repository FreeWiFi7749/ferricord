#!/usr/bin/env python3
"""
Ferricord Phase 1 Full Features Example Bot

This example demonstrates ALL Phase 1 features of Ferricord:
- Client initialization with custom Intents
- All available event handlers
- Cache access methods
- Model properties and methods

Note: Phase 1 is receive-only. Message sending will be available in Phase 2.

Usage:
    export DISCORD_TOKEN="your_bot_token"
    python full_features_bot.py
"""

import os
import asyncio
from datetime import datetime

# Import Ferricord components
from ferricord import Client, Intents

# =============================================================================
# Configuration
# =============================================================================

def get_token() -> str:
    """Get the bot token from environment variable."""
    token = os.environ.get("DISCORD_TOKEN")
    if not token:
        raise ValueError(
            "DISCORD_TOKEN environment variable is not set.\n"
            "Please set it with: export DISCORD_TOKEN='your_bot_token'"
        )
    return token

# =============================================================================
# Intents Configuration
# =============================================================================

def create_intents() -> Intents:
    """
    Create and configure Gateway Intents.
    
    Intents control which events your bot receives from Discord.
    Some intents are privileged and require verification for bots in 100+ guilds.
    
    Available intents:
    - guilds: Guild create/update/delete events
    - members: Member join/remove events (PRIVILEGED)
    - moderation: Ban add/remove events
    - emojis_and_stickers: Emoji/sticker update events
    - integrations: Integration update events
    - webhooks: Webhook update events
    - invites: Invite create/delete events
    - voice_states: Voice state update events
    - presences: Presence update events (PRIVILEGED)
    - guild_messages: Message events in guilds
    - guild_reactions: Reaction events in guilds
    - guild_typing: Typing events in guilds
    - dm_messages: Message events in DMs
    - dm_reactions: Reaction events in DMs
    - dm_typing: Typing events in DMs
    - message_content: Access to message content (PRIVILEGED)
    - scheduled_events: Scheduled event updates
    - auto_moderation_configuration: AutoMod config updates
    - auto_moderation_execution: AutoMod execution events
    """
    # Start with default intents (all non-privileged)
    intents = Intents.default()
    
    # Enable privileged intents (requires Discord Developer Portal configuration)
    intents.message_content = True  # Required to read message content
    intents.members = True          # Required for member join/remove events
    # intents.presences = True      # Uncomment if you need presence updates
    
    # You can also use:
    # intents = Intents.all()   # All intents including privileged
    # intents = Intents.none()  # No intents (minimal)
    
    # Check intent values
    print(f"[Config] Intents value: {intents.value}")
    print(f"[Config] Message content enabled: {intents.message_content}")
    print(f"[Config] Members enabled: {intents.members}")
    print(f"[Config] Guild messages enabled: {intents.guild_messages}")
    
    return intents

# =============================================================================
# Create Client
# =============================================================================

intents = create_intents()
client = Client(intents=intents)

# =============================================================================
# Event Handlers - All Phase 1 Events
# =============================================================================

@client.event
async def on_ready():
    """
    Called when the bot has successfully connected to Discord.
    
    Available client properties:
    - client.user: The bot's User object
    - client.guilds: List of all guilds (expensive for large bots)
    - client.guild_ids(): List of guild IDs (lightweight)
    - client.guild_count: Number of guilds
    - client.cache_stats(): Cache statistics string
    """
    print("\n" + "=" * 60)
    print("BOT IS READY!")
    print("=" * 60)
    
    # Access bot user information
    user = client.user
    if user:
        print(f"\n[User Info]")
        print(f"  ID: {user.id}")
        print(f"  Username: {user.username}")
        print(f"  Discriminator: {user.discriminator}")
        print(f"  Display Name: {user.display_name}")
        print(f"  Global Name: {user.global_name}")
        print(f"  Tag: {user.tag()}")
        print(f"  Is Bot: {user.bot}")
        print(f"  Is System: {user.system}")
        print(f"  Avatar: {user.avatar}")
        print(f"  Avatar URL: {user.avatar_url()}")
        print(f"  Default Avatar URL: {user.default_avatar_url()}")
        print(f"  Display Avatar URL: {user.display_avatar_url()}")
        print(f"  Mention: {user.mention()}")
    
    # Access guild information
    print(f"\n[Guild Info]")
    print(f"  Guild Count: {client.guild_count}")
    
    # Lightweight access - just IDs
    guild_ids = client.guild_ids()
    print(f"  Guild IDs: {guild_ids[:5]}{'...' if len(guild_ids) > 5 else ''}")
    
    # Full guild access (use sparingly for large bots)
    guilds = client.guilds
    for guild in guilds[:3]:  # Show first 3 guilds
        print(f"\n  [Guild: {guild.name}]")
        print(f"    ID: {guild.id}")
        print(f"    Owner ID: {guild.owner_id}")
        print(f"    Is Owner: {guild.owner}")
        print(f"    Description: {guild.description}")
        print(f"    Preferred Locale: {guild.preferred_locale}")
        print(f"    Premium Tier: {guild.premium_tier}")
        print(f"    Boost Count: {guild.premium_subscription_count}")
        print(f"    Vanity URL: {guild.vanity_url_code}")
        print(f"    Icon: {guild.icon}")
        print(f"    Icon URL: {guild.icon_url()}")
        print(f"    Banner URL: {guild.banner_url()}")
    
    if len(guilds) > 3:
        print(f"\n  ... and {len(guilds) - 3} more guilds")
    
    # Get specific guild by ID
    if guild_ids:
        specific_guild = client.get_guild(guild_ids[0])
        if specific_guild:
            print(f"\n[Specific Guild Lookup]")
            print(f"  Found guild: {specific_guild.name} (ID: {specific_guild.id})")
    
    # Cache statistics
    print(f"\n[Cache Stats]")
    print(f"  {client.cache_stats()}")
    
    print("\n" + "=" * 60)
    print("Listening for events... (Ctrl+C to stop)")
    print("=" * 60 + "\n")


@client.event
async def on_message(message):
    """
    Called when a message is received.
    
    Message properties:
    - id: Message ID
    - channel_id: Channel ID where message was sent
    - guild_id: Guild ID (None for DMs)
    - content: Message content (requires MESSAGE_CONTENT intent)
    - author: User who sent the message
    - timestamp: When the message was sent
    - edited_timestamp: When the message was edited (if edited)
    - tts: Whether this is a TTS message
    - mention_everyone: Whether @everyone was mentioned
    - pinned: Whether the message is pinned
    - type: Message type (0 = default)
    - jump_url(): URL to jump to this message
    """
    # Ignore bot messages to prevent loops
    if message.author.bot:
        return
    
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] MESSAGE RECEIVED")
    print(f"  Message ID: {message.id}")
    print(f"  Channel ID: {message.channel_id}")
    print(f"  Guild ID: {message.guild_id}")
    print(f"  Content: {message.content[:100]}{'...' if len(message.content) > 100 else ''}")
    print(f"  Author: {message.author.tag()} (ID: {message.author.id})")
    print(f"  Timestamp: {message.timestamp}")
    print(f"  Edited: {message.edited_timestamp}")
    print(f"  TTS: {message.tts}")
    print(f"  Mentions Everyone: {message.mention_everyone}")
    print(f"  Pinned: {message.pinned}")
    print(f"  Type: {message.type}")
    print(f"  Jump URL: {message.jump_url()}")
    
    # Example command handling (Phase 1: receive only, no response)
    if message.content.startswith("!"):
        command = message.content[1:].split()[0].lower()
        print(f"  [Command Detected: !{command}]")
        
        if command == "ping":
            print("    -> Would respond with 'Pong!' (Phase 2)")
        elif command == "info":
            print("    -> Would respond with bot info (Phase 2)")
        elif command == "guilds":
            print(f"    -> Bot is in {client.guild_count} guilds")
        elif command == "cache":
            print(f"    -> Cache stats: {client.cache_stats()}")


@client.event
async def on_guild_join(guild):
    """
    Called when the bot joins a new guild.
    
    Guild properties: Same as shown in on_ready
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] GUILD JOINED")
    print(f"  Name: {guild.name}")
    print(f"  ID: {guild.id}")
    print(f"  Owner ID: {guild.owner_id}")
    print(f"  Description: {guild.description}")
    print(f"  Premium Tier: {guild.premium_tier}")
    print(f"  Total Guilds Now: {client.guild_count}")


@client.event
async def on_guild_update(guild):
    """
    Called when a guild is updated (name change, settings change, etc.).
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] GUILD UPDATED")
    print(f"  Name: {guild.name}")
    print(f"  ID: {guild.id}")
    print(f"  Description: {guild.description}")


@client.event
async def on_guild_remove(guild_id):
    """
    Called when the bot is removed from a guild or the guild is deleted.
    
    Note: Only receives guild_id, not the full guild object.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] GUILD REMOVED")
    print(f"  Guild ID: {guild_id}")
    print(f"  Total Guilds Now: {client.guild_count}")


@client.event
async def on_channel_create(channel):
    """
    Called when a channel is created.
    
    Channel properties:
    - id: Channel ID
    - type: Channel type (0=text, 2=voice, 4=category, etc.)
    - guild_id: Guild ID (None for DMs)
    - name: Channel name
    - topic: Channel topic
    - nsfw: Whether the channel is NSFW
    - position: Channel position in the list
    - mention(): Channel mention string
    - is_text_based(): Whether it's a text channel
    - is_voice_based(): Whether it's a voice channel
    - is_thread(): Whether it's a thread
    - is_dm(): Whether it's a DM channel
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] CHANNEL CREATED")
    print(f"  Name: {channel.name}")
    print(f"  ID: {channel.id}")
    print(f"  Type: {channel.type}")
    print(f"  Guild ID: {channel.guild_id}")
    print(f"  Topic: {channel.topic}")
    print(f"  NSFW: {channel.nsfw}")
    print(f"  Position: {channel.position}")
    print(f"  Mention: {channel.mention()}")
    print(f"  Is Text: {channel.is_text_based()}")
    print(f"  Is Voice: {channel.is_voice_based()}")
    print(f"  Is Thread: {channel.is_thread()}")
    print(f"  Is DM: {channel.is_dm()}")


@client.event
async def on_channel_update(channel):
    """
    Called when a channel is updated.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] CHANNEL UPDATED")
    print(f"  Name: {channel.name}")
    print(f"  ID: {channel.id}")
    print(f"  Topic: {channel.topic}")


@client.event
async def on_channel_delete(channel):
    """
    Called when a channel is deleted.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] CHANNEL DELETED")
    print(f"  Name: {channel.name}")
    print(f"  ID: {channel.id}")
    print(f"  Type: {channel.type}")


@client.event
async def on_member_join(guild_id):
    """
    Called when a member joins a guild.
    
    Note: Requires GUILD_MEMBERS privileged intent.
    Currently only receives guild_id. Full member info in Phase 2.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] MEMBER JOINED")
    print(f"  Guild ID: {guild_id}")
    
    # Get guild info
    guild = client.get_guild(guild_id)
    if guild:
        print(f"  Guild Name: {guild.name}")


@client.event
async def on_member_remove(guild_id, user):
    """
    Called when a member leaves a guild.
    
    Note: Requires GUILD_MEMBERS privileged intent.
    
    User properties: Same as message.author
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] MEMBER LEFT")
    print(f"  Guild ID: {guild_id}")
    print(f"  User: {user.tag()} (ID: {user.id})")
    print(f"  Username: {user.username}")
    print(f"  Display Name: {user.display_name}")
    
    # Get guild info
    guild = client.get_guild(guild_id)
    if guild:
        print(f"  Guild Name: {guild.name}")


@client.event
async def on_typing(channel_id, user_id):
    """
    Called when a user starts typing in a channel.
    
    Note: Only receives IDs, not full objects.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] TYPING START")
    print(f"  Channel ID: {channel_id}")
    print(f"  User ID: {user_id}")


@client.event
async def on_resumed():
    """
    Called when the bot resumes a previous session after reconnecting.
    """
    timestamp = datetime.now().strftime("%H:%M:%S")
    
    print(f"\n[{timestamp}] SESSION RESUMED")
    print("  Successfully reconnected to Discord Gateway")


# =============================================================================
# Main Entry Point
# =============================================================================

def main():
    """Main entry point for the bot."""
    print("\n" + "=" * 60)
    print("FERRICORD PHASE 1 - FULL FEATURES EXAMPLE")
    print("=" * 60)
    print("\nPhase 1 Features:")
    print("  - Gateway connection (single shard)")
    print("  - Event receiving (all events listed above)")
    print("  - Cache (guilds, channels, users, messages)")
    print("  - Intents configuration")
    print("\nNot Yet Available (Phase 2+):")
    print("  - Message sending")
    print("  - Slash commands")
    print("  - Multi-shard support")
    print("  - Voice connections")
    print("=" * 60 + "\n")
    
    try:
        token = get_token()
        print("[Starting] Connecting to Discord...")
        
        # Run the bot (blocking call)
        client.run(token)
        
    except ValueError as e:
        print(f"\n[Error] {e}")
        return 1
    except KeyboardInterrupt:
        print("\n\n[Shutdown] Bot stopped by user")
        return 0
    except Exception as e:
        print(f"\n[Error] Unexpected error: {e}")
        return 1
    
    return 0


if __name__ == "__main__":
    exit(main())
