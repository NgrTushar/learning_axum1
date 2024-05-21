use crate::{Error,Result};
use crate::web::AUTH_TOKEN;
use axum::body::Body;
use axum::response::Response;
use axum::http::Request;
use axum::middleware::Next;
use tower_cookies::Cookies;

pub async fn mw_require_auth(
    mut cookies: Cookies,
    req:Request<Body>,
    next:Next,) 
    ->Result<Response> {
    println!("->> {:<12} - mw_require_auth","MIDDLEWARE");
    println!("->> {:<12} - mw_require_auth","MIDDLEWARE");  
    let auth_token=cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

/*    //TODO: Real aith-token parsing and validation
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
   // if auth_token.is_none() {
           // return Err(Error::AuthFailNoAuthTokenCookie)
	    //}

Ok(next.run(req).await)*/

match auth_token {
        Some(token) => {
            // Validate the token here
            // If valid, proceed with the request
            Ok(next.run(req).await)
        }
        None => {
            // If the token is not present or not valid, return an error
            Err(Error::AuthFailNoAuthTokenCookie)
        }
    }
}
