use std::fs;
use std::path::Path;

pub fn read_hosts() -> Result<String, Box<dyn std::error::Error>> {
    fs::read_to_string("/etc/hosts").map_err(|e| e.into())
}

pub fn write_hosts(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("/etc/hosts").exists() {
        return Err("Hosts file doesn't exist".into());
    }

    eprintln!("⚠️  This will overwrite /etc/hosts. Run with sudo!");
    fs::write("/etc/hosts", content)?;
    Ok(())
}

pub fn dry_run(content: &str) {
    eprintln!("=== DRY-RUN OUTPUT (would write to /etc/hosts): ===");
    println!("{}", content);
}
