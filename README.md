# Shaggy

Shaggy is a silly Discord bot for the Mushroom Farm server.

Shaggy does a couple of things:
- Sends us random cooking recipes in the morning.
- Offers us recipes when asked, based on categories or ingredients... or totally random ones.
- Gives you video game songs to listen to, by folder/series/entry.
- Joins a voice channel and uses Text-to-Speech (TTS) to speak messages posted in that voice channel's text chat (only for users who are in the call and opted-in).
- Replies to @ mentions.
- And some other hidden Easter eggs.

## TTS Usage
- /tts signup — opt-in to having your messages read while you are in a voice call.
- /tts signout — opt-out from TTS.
- /tts join — have the bot join your current voice channel (required before TTS starts speaking).
- /tts leave — have the bot leave the voice channel.

Notes:
- TTS only triggers for messages posted in a voice channel's text chat, from users currently connected to that same voice channel, and who have signed up.
- System dependencies: Piper TTS must be installed on the host, along with a Piper voice model (.onnx). Optionally set env vars: PIPER_BIN (path to piper binary), PIPER_MODEL (path to model .onnx) and PIPER_SPEAKER (speaker id for multi‑speaker models). If PIPER_BIN is not set, the bot will attempt to use `piper` from PATH. PIPER_MODEL is required.
