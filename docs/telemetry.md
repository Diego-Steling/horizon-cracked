# Data collection

This fork of Horizon has no telemetry. The `horizon-telemetry` crate, its ingest server, the
anonymous "I'm playing" ping, benchmark uploads and the launcher's consent screen are removed.
Nothing about the player, the machine or a world is collected or sent.

The launcher's Servers page still asks a Minecraft server how many players it has, using the same
status ping a client makes when it joins; that count is drawn locally and nothing is sent anywhere
else.

The code changes are in [launcher/src](../launcher/src) and the game crates; the upstream project's
docs (links from [architecture.md](architecture.md)) describe the telemetry that used to exist.
