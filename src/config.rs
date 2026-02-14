use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    // let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    // Path::new(&home).join(".config/hostctl/db.sqlite")
    //
    PathBuf::from("/etc/hostctl/db.sqlite")
}
