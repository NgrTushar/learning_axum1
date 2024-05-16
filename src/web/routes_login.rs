use crate::{Error,Result};
use serde::Deserialize; 
use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json,Value};
use tower_cookies::Cookies;
use tower_cookies::Cookie;
use crate::web;


pub fn routes() -> Router{
Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies:Cookies ,payload: Json<LoginPayload>)-> Result<Json<Value>>{
println!("->> {:<12} - api_login","HANDLER");

//ToDo: implement real db/auth logic.
if payload.username != "demo1" || payload.pwd !="Welcome" {
return Err(Error::LoginFail);
}

//FIXME: Implementreal auth-token generation/signature.
//ToDo :Set Cookies
//cookies.add(Cookie::new("web::AUTH_TOKEN","user-1.exp.sign"));
let mut cookie = Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign");
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);

//Create success Body
let body=Json(json!({
"result":{
"success": true
}
}));

Ok(body)
}
#[derive(Debug,Deserialize)]
struct LoginPayload {
username: String,
pwd: String,
}
