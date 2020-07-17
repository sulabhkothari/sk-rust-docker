#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_oauth2::{OAuth2, TokenResponse};
use rocket::response::Redirect;
use rocket::http::{Cookie,Cookies, SameSite};
// This struct will only be used as a type-level key. Multiple
// instances of OAuth2 can be used in the same application by
// using different key types.
struct Google;

// This route calls `get_redirect`, which sets up a token request and
// returns a `Redirect` to the authorization endpoint.
#[get("/login/google")]
fn google_login(oauth2: OAuth2<Google>, mut cookies: Cookies<'_>) -> Redirect {
    oauth2.get_redirect(&mut cookies, &["email"]).unwrap()
}

// This route, mounted at the application's Redirect URI, uses the
// `TokenResponse` request guard to complete the token exchange and obtain
// the token.
//
// The order is important here! If Cookies is positioned before
// TokenResponse, TokenResponse will be unable to verify the token exchange
// and return a failure.
#[get("/auth/google")]
fn google_callback(token: TokenResponse<Google>, mut cookies: Cookies<'_>) -> Redirect
{
    println!("{}", token.access_token());
    // Set a private cookie with the access token
    cookies.add_private(
        Cookie::build("token", token.access_token().to_string())
            .same_site(SameSite::Lax)
            .finish()
    );
    Redirect::to("/")
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Per directory rust nightly build override
// rustup override set nightly
fn main() {
    rocket::ignite()
        .mount("/", routes![google_callback, google_login, index])
        // The string "github" here matches [global.oauth2.github] in `Rocket.toml`
        .attach(OAuth2::<Google>::fairing("google"))
        .launch();
}

// linking with `cc` failed docker build rust:
// https://users.rust-lang.org/t/linking-error-in-docker/29563
// You will need to install libpq inside the container, as diesel depends on it to connect to a PostgreSQL database.