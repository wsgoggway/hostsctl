use askama::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct HostEntry {
    pub host: String,
    pub address: String,
}

#[derive(Template)]
#[template(path = "hosts.j2")]
pub struct HostsTemplate<'a> {
    pub entries: &'a [HostEntry],
    pub profile: String,
}
