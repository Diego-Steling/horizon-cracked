//! Every built-in address of an outside service vanilla doesn't contact, in one place
//! (docs/layers.md#built-in-addresses). `tools/check_addresses.py` keeps new ones from going
//! anywhere else.
//!
//! This is a Horizon-layer crate: the engine and the platform never depend on it. Platform code
//! that talks to a service gets the address passed in from a Horizon crate.

/// This fork's website, where the launcher is downloaded: the game started without its prepared
/// files sends the player there.
pub const WEBSITE: &str = "https://github.com/Diego-Steling/horizon-cracked";

/// Minecraft's product links, offered by the official build's Mods screen.
pub const MINECRAFT_WEBSITE: &str = "https://minecraft.net";
pub const MINECRAFT_ISSUES: &str = "https://bugs.mojang.com";

/// This fork's license and issue tracker, offered by the build's Mods screen.
pub const LICENSE: &str = "https://github.com/Diego-Steling/horizon-cracked/blob/main/LICENSE";
pub const ISSUES: &str = "https://github.com/Diego-Steling/horizon-cracked/issues";

/// Modpack file hosts, including GitHub's release redirects and the loader's Maven repository.
/// The launcher and porting CLI pass this policy to the platform's checked downloader.
pub const MODPACK_DOWNLOAD_HOSTS: &[&str] = &[
    "cdn.modrinth.com",
    "github.com",
    "raw.githubusercontent.com",
    "objects.githubusercontent.com",
    "release-assets.githubusercontent.com",
    "gitlab.com",
    "maven.neoforged.net",
];

/// The cosmetics service: cosmetics, emotes, "Show me as a Horizon player"
/// (`HORIZON_COSMETICS_API` overrides it).
pub const COSMETICS_API: &str = "https://cosmetics.playhorizon.dev";

/// The performance viewer page a saved capture opens in (`HORIZON_PERF_VIEWER` overrides it).
pub const PERF_VIEWER: &str = "https://playhorizon.dev/perf";

/// The capture upload API behind Share (`HORIZON_PERF_CAPTURES` overrides it).
pub const PERF_CAPTURES: &str = "https://playhorizon.dev/api/perf/captures";

/// Public STUN servers for Play with Friends' NAT discovery: two independent operators, so one
/// outage doesn't break discovery (`HORIZON_STUN_SERVERS` overrides them).
pub const STUN_SERVERS: &[&str] = &["stun.cloudflare.com:3478", "stun.l.google.com:19302"];

/// Play with Friends' troubleshooting page, shown to hosts nobody can reach.
pub const FRIENDS_HELP: &str =
    "https://github.com/horizon-rust/horizon/blob/main/docs/play-with-friends.md#troubleshooting";

/// Horizon community invite used by launcher defaults.
pub const COMMUNITY_DISCORD: &str = "https://discord.gg/playhorizon";

/// The sponsoring hosting provider’s referral page.
pub const HOSTING_PARTNER: &str = "https://my.sculkhosting.com/ref/horizon";

/// The official cosmetics shop.
pub const SHOP: &str = "https://playhorizon.dev/shop";

/// The GitHub profile of who made the launcher's look, credited in Settings' About (its picture is
/// `<this>.png`).
pub const LAUNCHER_LOOK_CREDIT: &str = "https://github.com/Xmilon";

/// The official launcher installer's product home.
pub const LAUNCHER_ABOUT: &str = "https://playhorizon.dev";

/// Cargo's canonical source identity for the crates.io registry.
pub const CARGO_CRATES_IO_SOURCE: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// The pinned horizon-mixins archive the launcher downloads for `[[native.mixin]]` builds.
pub const HORIZON_MIXINS_SOURCE: &str =
    "https://codeload.github.com/horizon-rust/horizon-mixins/tar.gz";

/// Where the fork releases the prebuilt horizon-mixins tool; a release's tag and file follow.
pub const MIXINS_TOOL_RELEASES: &str = "https://github.com/horizon-rust/horizon/releases/download";

/// Where the fork publishes each game build's mod SDK (`sdk-<build id>`) and the game releases
/// whose `sdk.json` names their build; a release's tag and file follow. `HORIZON_SDK_BASE`
/// overrides it.
pub const SDK_RELEASES: &str = "https://github.com/horizon-rust/horizon/releases/download";

/// The fork's newest releases, which `cargo horizon-mod` reads to find the one SDK a short sha names.
pub const SDK_RELEASE_LIST: &str =
    "https://api.github.com/repos/horizon-rust/horizon/releases?per_page=100";
