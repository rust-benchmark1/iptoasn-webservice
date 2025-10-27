use crate::xss_handler::read_untrusted_xss_data;
use actix_web::web::Html;
use rocket::response::content::RawCss;

/// Actix sink for reflected XSS
pub fn actix_reflected_xss() -> Result<String, String> {
    let user_input = read_untrusted_xss_data()?;

    //SINK
    let _html_response = Html::new(user_input.clone());

    Ok("Actix HTML response created from tainted input".to_string())
}

/// Rocket sink for CSS-based XSS
pub fn rocket_css_xss() -> Result<String, String> {
    let user_input = read_untrusted_xss_data()?;

    //SINK
    let _css_response = RawCss(user_input.clone());

    Ok("Rocket CSS response created from tainted input".to_string())
}
