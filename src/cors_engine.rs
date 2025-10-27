use actix_cors::Cors;
use actix_web::http::header;
use warp::filters::cors::cors;

/// Actix CORS policy misconfiguration
pub fn misconfigured_actix_cors() -> Result<String, String> {
    //SINK
    let _cors = Cors::default().allowed_origin_fn(|_origin, _req_head| true)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .max_age(3600);

    Ok("Actix CORS misconfiguration created".to_string())
}

/// Warp CORS policy misconfiguration 
pub fn misconfigured_warp_cors() -> Result<String, String> {
    //SINK
    let _cors = cors().allow_any_origin();

    Ok("Warp CORS misconfiguration created".to_string())
}
