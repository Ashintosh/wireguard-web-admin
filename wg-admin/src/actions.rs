use crate::utils;
use anyhow::Result;

// Add a peer using `wg` commands
pub fn add_peer(name: &str, ip: &str, allowed_ips: &str) -> Result<()> {
    println!("Adding peer: {} -> {}", name, ip);

    let private_key = utils::run_command("wg", &["genkey"])?;
    let public_key = utils::run_command_with_input("wg", &["pubkey"], &private_key)?;

    utils::run_command(
        "doas", // TODO: Detect if Linux or OpenBSD
        &["wg", "set", "wg0", "peer", &public_key, "allowed-ips", allowed_ips],
    )?;

    println!("Peer {} added successfully.", name);
    Ok(())
}

pub fn remove_peer(name: &str) -> Result<()> {
    println!("Removing peer: {}", name);

    // Get public key from local store
    let public_key = utils::get_peer_public_key(name)?;
    utils::run_command("doas", &["wg", "set", "wg0", "peer", &public_key, "remove"])?;

    println!("Peer {} removed.", name);
    Ok(())
}

pub fn edit_peer(name: &str, ip: Option<&str>, allowed_ips: Option<&str>) -> Result<()> {
    println!("Editing peer: {}", name);

    let public_key = utils::get_peer_public_key(name)?;

    if let Some(allowed) = allowed_ips {
        utils::run_command(
            "doas",
            &["wg", "set", "wg0", "peer", &public_key, "allowed-ips", allowed],
        )?;
    }

    println!("Peer {} updated", name);
    Ok(())
}

pub fn list_peers() -> Result<()> {
    let output = utils::run_command("wg", &["show"])?;
    println!("{}", output);
    Ok(())
}