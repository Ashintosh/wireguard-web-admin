use std::process::{ Command, Stdio };
use anyhow::{ Result, anyhow };

// Run a command and return stdout as string
pub fn run_command(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if ! output.status.success() {
        return Err(anyhow!(
            "Command `{}` failed: {}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Run a command with input piped in
pub fn run_command_with_input(cmd: &str, args: &[&str], input: &str) -> Result<String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = &mut child.stdin {
        use std::io::Write;
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if ! output.status.success() {
        return Err(anyhow!(
            "Command `{}` failed: {}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// Stub: get peer public key by name
pub fn get_peer_public_key(name: &str) -> Result<String> {
    // TODO: implement some sort of mapping or parse wg0.conf
    Err(anyhow!("get_peer_public_key not implemented for {}", name))
}