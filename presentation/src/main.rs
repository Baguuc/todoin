use crate::prelude::*;

pub mod error;
pub mod prelude;
pub mod config;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    use colored::Colorize;
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;
    use clin::components::{success, error, header};
    use crate::config::Config;
    use authin_sdk::Sdk;   
        
    migrate().await;
    println!("");

    header("Running web server");

    let config = error_if_necessary(Config::read(String::from("./todoin.json")));
    
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
    let server = HttpServer::new(move || {
        let config = error_if_necessary(Config::read(String::from("./todoin.json")));
        let pool = error_if_necessary(block_on(create_pool(config.database.clone())));
        let sdk = error_if_necessary(Sdk::create(config.authin_url.clone()))
            .user();
        
        App::new()
            .app_data(Data::new(pool))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(sdk.clone()))
            .service(crate::routes::project::insert::insert_route)
            .service(crate::routes::project::list::list_route)
            .service(crate::routes::project::retrieve::retrieve_route)
            .service(crate::routes::project::delete::delete_route)
    });

    let binded_server = match server.bind(("0.0.0.0", config.port.clone())) {
        Ok(server) => server,
        Err(_) => {
            error("Cannot bind to port", config.port);
            
            std::process::exit(1);
        }
    };

    let _ = binded_server.run().await;

    return Ok(());
}

async fn migrate() {
    use clin::components::{header,success};
    use crate::config::Config;
    use crate::error::error_if_necessary;
    
    let config = error_if_necessary(Config::read(String::from("./todoin.json")));
    let pool = error_if_necessary(create_pool(config.database.clone()).await);
     
    header("Migrating database");
    
    error_if_necessary(todoin_application::MigrationRepository::migrate(&pool).await);
    success("Migrated");
}

async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
