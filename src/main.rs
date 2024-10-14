pub mod server_struct::Config;

use actix_cors;
use actix_web_httpauth;
use dotenv;
use llm::Model;
use openssl;
use rand;
use serde;
use serde_json;
use tokio;
use tokio_macros;


 

fn main() {
    tokio::run(fetch());
}

#[actix_web::main]  
#[cfg(feature = "server")] 
async fn fetch() -> std::io::Result<()> {  
    let config: Config = Config::init();  
    let model_path = PathBuf::from(&config.llm_model);  
    let now = std::time::Instant::now();  
    let model_architecture = match_model_architecture(&config.llm_model_architecture)  
        .unwrap_or_else(|| {  
            panic!(  
                "Failed to find model architecture {} for model: {}.\n",  
                config.llm_model_architecture, &config.llm_model  
            );  
        });  

    let model = llm::load_dynamic(  
        model_architecture,  
        &model_path,  
        Default::default(),  
        llm::load_progress_callback_stdout,  
    )  
    .unwrap_or_else(|err| {  
        panic!(  
            "Failed to load {} model from {:?}: {}",  
            config.llm_model, model_path, err  
        );  
    });  

    println!(  
        "{} model ({}) has been started!\nElapsed: {}ms",  
        config.llm_model,  
        config.llm_model_architecture,  
        now.elapsed().as_millis()  
    );  

    println!(  
        "Starting server at https://{}:{}.\n",  
        config.server_address, config.server_port  
    );  

    let config: Config = Config::init();  
    let app_state = web::Data::new(AppState {  
        model,  
        config: config.clone(),  
    });  

    let complete_address = format!("{}:{}", config.server_address, config.server_port);  
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();  

    ssl_builder  
        .set_private_key_file("certs/key.pem", SslFiletype::PEM)  
        .unwrap();  

    ssl_builder  
        .set_certificate_chain_file("certs/cert.pem")  
        .unwrap();  

    HttpServer::new(move || {  
        App::new()  
            .app_data(app_state.clone())  
            .wrap(middleware::Logger::default())  
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))  
            .wrap(middleware::Compress::default())  
            .wrap(  
                Cors::default()  
                    .allowed_origin(&config.allowed_origin)  
                    .allowed_methods(vec!["GET", "POST"])  
                    .allowed_headers(vec![  
                        http::header::AUTHORIZATION,  
                        http::header::ACCEPT,  
                        http::header::CONTENT_TYPE,  
                    ])  
                    .max_age(config.max_age as usize),  
            )  
            .route("/", web::get().to(server_info_handler))  
            .service(  
                web::scope("/api")  
                    .route("/generate", web::post().to(generate_handler))  
                    .route("/health", web::get().to(health_handler)),  
            )  
    })  
    .bind_openssl(complete_address, ssl_builder)?  
    .run()  
    .await 
}
