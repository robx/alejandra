#[cfg(any(
    all(
        target_arch = "aarch64",
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musl"
    ),
    all(
        any(target_arch = "armv6l", target_arch = "armv7l",),
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musleabihf"
    ),
    all(
        target_arch = "i686",
        target_vendor = "unknown",
        target_os = "linux",
        target_env = "musl"
    ),
    all(
        target_arch = "x86_64",
        target_vendor = "unknown",
        target_os = "linux",
        any(target_env = "gnu", target_env = "musl")
    ),
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod format;
pub(crate) mod formatter;
pub(crate) mod formatting_rules;
pub(crate) mod trivia;
pub(crate) mod utils;
pub mod version;
