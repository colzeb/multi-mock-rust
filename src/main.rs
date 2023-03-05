mod configuration;
mod connection;
mod encoding;
mod random_number;
mod route;

#[tokio::main]
async fn main() {
    let user = route::user::route().await;
    configuration::get_db();
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(user.into_make_service())
        .await
        .unwrap();
}
