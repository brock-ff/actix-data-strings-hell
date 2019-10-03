use actix_web::{web, web::Path, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type SyncHashMap = Arc<Mutex<HashMap<String, String>>>;
type KeyValData = web::Data<SyncHashMap>;

/// Use `Data<T>` extractor to access data in handler.
fn index(data: KeyValData, path: Path<String>) {
    let game_id: String = path.into_inner();
    let mut chain_map = data.lock().unwrap();
    let blockchain_url = chain_map.get(&game_id);
    if blockchain_url.is_some() {
        println!("blockchain_url: {}", blockchain_url.unwrap());
    } else {
        println!("No blockchain_url found for this game. Fetching from DB...");
        chain_map.insert(game_id, "http://sampleurl:8545".to_owned());
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            // Store `MyData` in application storage.
            .data(Arc::new(Mutex::new(HashMap::<String, String>::new())))
            .service(web::resource("/{id}").route(web::get().to(index)))
    })
    .bind("127.0.0.1:1337")
    .expect("Cannot bind to port 1337 :(")
    .run()
    .unwrap();
}
