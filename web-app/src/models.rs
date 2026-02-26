#[derive(Debug, Clone)]
pub struct Client {
    pub name: String,
    pub description: String,
    pub ip_address: String,
    pub allowed_ips: String,
    pub public_key: String,
    pub endpoint: String,
    pub last_seen: String,
    pub bytes_sent: String,
    pub bytes_received: String,
    pub status: String,
    pub creation_date: String,
}