use env_logger::Env;
use infrastructure::web::run;
use dotenv::dotenv;

mod schema;

mod domain;

mod application;

mod infrastructure;

mod presentation;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Charge les variables d'environnement depuis .env

    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();
    run().await
}
