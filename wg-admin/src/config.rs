use std::path::PathBuf;

// Path to the WireGuard configuration file
pub fn wg_config_path() -> PathBuf {
    PathBuf::from("/etc/wireguard/wg0.conf")
}