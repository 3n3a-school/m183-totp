use google_authenticator::{ErrorCorrectionLevel, GoogleAuthenticator};
use rocket::{State, form::*, get, post, response::Redirect, routes};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use serde_json::json;
use sqlx::{postgres::PgPool, *};

use std::result::Result;
use std::*;

use once_cell::sync::Lazy;

const DATABASE_URL: Lazy<String> = Lazy::new(||env::var("ASSETS_PATH")
    .unwrap_or(String::from("postgres://postgres:postgres@localhost/")) );

#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form, false).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
async fn get_signup() -> Template {
    Template::render("signup", json!({}))
}

#[post("/signup", data = "<form>")]
async fn post_signup(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into(), true).await?;

    Ok(Redirect::to("/show_totp"))
}

#[get("/")]
async fn index(user: Option<User>) -> Template {
    Template::render("index", json!({ "user": user }))
}

#[get("/logout")]
fn logout(auth: Auth<'_>) -> Result<Template, Error> {
    auth.logout()?;
    Ok(Template::render("logout", json!({})))
}

#[get("/delete")]
async fn delete(auth: Auth<'_>) -> Result<Template, Error> {
    auth.delete().await?;
    Ok(Template::render("deleted", json!({})))
}

#[get("/show_all_users")]
async fn show_all_users(conn: &State<PgPool>, user: Option<User>) -> Result<Template, Error> {
    let users: Vec<User> = query_as("select * from users;").fetch_all(&**conn).await?;
    println!("{:?}", users);
    Ok(Template::render(
        "users",
        json!({"users": users, "user": user}),
    ))
}

#[get("/show_totp")]
async fn show_totp(user: User) -> Template {
    let secret = user.totp_secret();
    let g_auth = GoogleAuthenticator::new();
    let title = "Rocket Auth TOTP";
    let account_name = user.email();
    let url = (&g_auth).qr_code_url(&secret, account_name, title, 200, 200, ErrorCorrectionLevel::High).clone().to_owned();
    Template::render("totp", json!({ "user": user, "totp_url": &url }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = PgPool::connect(&(*DATABASE_URL).clone()).await?;
    let users: Users = conn.clone().into();
    users.create_table().await?; 
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_login,
                post_signup,
                get_signup,
                post_login,
                logout,
                delete,
                show_all_users,
                show_totp
            ],
        )
        .manage(conn)
        .manage(users)
        .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
    Ok(())
}