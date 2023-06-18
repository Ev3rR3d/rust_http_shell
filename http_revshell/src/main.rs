use actix_web::{web, App, HttpResponse, HttpServer};
use std::io::{self, Read};
use std::process;
use std::sync::{Arc, Mutex};

async fn index(data: web::Data<Arc<Mutex<String>>>) -> HttpResponse {
    let input_command = data.lock().unwrap();
    let command = input_command.as_str();
    HttpResponse::Ok().body(format!("{}", command))
}

async fn process_command(payload: String, data: web::Data<Arc<Mutex<String>>>) -> HttpResponse {
    let mut input_command = data.lock().unwrap();
    *input_command = payload.clone();
    println!("Comando recebido: {}", payload);
    HttpResponse::Ok().body("Comando recebido")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let input_command = Arc::new(Mutex::new("".to_string()));

    let input_command_clone = input_command.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(input_command_clone.clone()))
            .route("/", web::get().to(index))
            .route("/command", web::post().to(process_command))
    })
    .bind("0.0.0.0:8080")?
    .run();

    tokio::spawn(async move {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Falha ao ler o comando");

            let input = buffer.trim().to_string();

            if input == "exit" || input == "quit" {
                process::exit(0);
            }

            let mut input_command = input_command.lock().unwrap();
            *input_command = input;
        }
    });

    server.await
}
