/// Hardcoded credentials for Neo4j
pub fn get_neo4j_credentials() -> (String, String, String) {
    let host = "localhost:7687".to_string();
    let username = "neo4j_admin".to_string();
    //SOURCE
    let password = "N30HardcodedP@ss!".to_string();
    (host, username, password)
}

/// Hardcoded credentials for InfluxDB
pub fn get_influxdb_credentials() -> (String, String, String) {
    let url = "http://localhost:8086".to_string();
    let org = "metrics_org".to_string();
    //SOURCE
    let token = "InfluxHardcodedT0k3n!".to_string(); 
    (url, org, token)
}
