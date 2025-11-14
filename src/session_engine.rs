use poem::session::{CookieConfig, CookieSession, ServerSession, MemoryStorage, Session};
use warp_sessions::{MemoryStore, SessionWithStore, CookieOptions, SameSiteCookieOption};
use warp::{Filter, Rejection};

pub fn poem_cookie_session() -> Result<String, String> {
    //SINK
    let _session = CookieSession::new(CookieConfig::default().http_only(false));
    Ok("Poem CookieSession created".to_string())
}

pub fn warp_with_session() -> Result<String, String> {
    let store = MemoryStore::new();
    //SINK
    let _vuln = warp::path!("warp_sessions" / "http_only_false")
            .and(warp_sessions::request::with_session(
                store.clone(),
                Some(CookieOptions {
                    cookie_name: "warp-session-vuln",
                    cookie_value: Some("verySensitiveCookieValue123!".to_string()),
                    max_age: Some(60),
                    domain: None,
                    path: None,
                    secure: true,
                    http_only: false,
                    same_site: Some(SameSiteCookieOption::Strict),
                }),
            ))
            .and_then(|mut sess: SessionWithStore<MemoryStore>| async move {
                sess.session.insert("user", "alice").unwrap();
                Ok::<_, warp::Rejection>((warp::reply::html("Vulnerable cookie set"), sess))
            })
            .untuple_one()
            .and_then(warp_sessions::reply::with_session);

    Ok("Warp route with http_only=false created".to_string())
}