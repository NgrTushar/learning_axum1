#![allow(unused)]
pub use self::error::{Error,Result};

//use axum::Router;
use crate::model::ModelController;
use axum::response::{Html,IntoResponse,Response};
use axum::middleware;
use axum::routing::{get,get_service};
use axum::extract::{Path,Query};
use tokio::net::TcpListener;
use axum::routing::Router;
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main()-> Result<()> {
    //Intialize ModelController
    let mc= ModelController::new().await?;
    
    let routes_apis=web::routes_ticket::routes(mc.clone()).route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all=Router::new()
        .merge(routes_hello())
	.merge(web::routes_login::routes())
	.nest("/api",routes_apis)
	.layer(middleware::map_response(main_response_mapper))
	.layer(CookieManagerLayer::new())
	.fallback_service(routes_static());
   
    //-------starting server
    let listener= TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("{:<12} - {:?}","LISTENING",listener.local_addr());
    axum::serve(listener,routes_all.into_make_service())
        .await
	.unwrap();

Ok(())
}
//-------layer
async fn main_response_mapper(res: Response)-> Response{
println!("->> {:<12} - main_response_mapper","RES_MAPPER");

println!();
res
}
//---------Static routing
fn routes_static() ->Router
{
Router::new().nest_service("/",get_service(ServeDir::new("./")))
}
//---------routes /hello
fn routes_hello() -> Router{
    Router::new()
        .route("/hello", get(handler_hello))
	.route("/hello2/:name",get(handler_hello2))
}
#[derive(Debug,Deserialize)]
struct HelloParams
{
name:Option<String>
}
// http://localhost/hello?name=Sharmaji
async fn handler_hello(Query(params):Query<HelloParams>) -> impl IntoResponse
{
println!("{:<12}  - handler_hello - {:?}","HANDLER",params);

let name=params.name.as_deref().unwrap_or("World!");

Html(format!("Hello <strong>{name}</strong>"))
}
// now we want to handle request where value will be in path instead of query parameters
async fn handler_hello2(Path(name) : Path<String>)-> impl IntoResponse
{
println!("{:<12} - handler_hello2 - {:?}","HANDLER",name);

Html(format!("Hello2 <strong>{name} </strong>"))
}
