
mod domain;
mod application;
mod infrastructure;
mod schema;
mod presentation;

use dotenv::dotenv;
use env_logger::Env;
use infrastructure::web::run;

#[actix_web::main]
async fn main() ->std::io::Result<()>{

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();

    run().await

}
