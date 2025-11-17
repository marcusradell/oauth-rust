use axum::response::{IntoResponse, Redirect};
use tower_cookies::{Cookie, Cookies};
mod authorization_callback;
mod landing_page;
mod router;
pub use router::router;

async fn log_out(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::from("client_access_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    let mut cookie = Cookie::from("client_refresh_token");
    cookie.set_path("/client");
    cookies.remove(cookie);

    Redirect::to("/client")
}
