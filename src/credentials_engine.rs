use crate::credentials_handler::{get_neo4j_credentials, get_influxdb_credentials};

/// Neo4j connection sink using hardcoded credentials
pub fn connect_neo4j_with_hardcoded_creds() -> Result<String, String> {
    smol::block_on(async {
        let (host, user, pass) = get_neo4j_credentials();

        //SINK
        let _graph = neo4rs::Graph::new(&host, &user, &pass)
            .await
            .map_err(|_| "Failed to create Neo4j Graph".to_string())?;

        Ok::<_, String>("Neo4j connection established with hardcoded credentials".to_string())
    })
}

/// InfluxDB connection sink using hardcoded credentials
pub fn connect_influx_with_hardcoded_creds() -> Result<String, String> {
    let (url, org, token) = get_influxdb_credentials();

    //SINK
    let _client = influxdb2::Client::new(&url, &org, &token);

    Ok("InfluxDB connection established with hardcoded credentials".to_string())
}
