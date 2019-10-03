use actix_web::{web, web::Path, App, HttpServer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct MyData {
    // hashmap containing game_id -> blockchain_url mappings
    chain_maps: Mutex<HashMap<String, String>>,
}

/// Use `Data<T>` extractor to access data in handler.
fn index(data: web::Data<Arc<MyData>>, path: Path<String>) {
    let game_id: String = path.into_inner();
    let chain_maps = data.chain_maps.lock().unwrap();
    let blockchain_url = chain_maps.get(&game_id);
    if blockchain_url.is_some() {
        println!("Found blockchain_url: {}", blockchain_url.unwrap());
    } else {
        println!("No blockchain_url associated. Pulling from DB.");
        data.chain_maps
            .lock()
            .unwrap()
            .insert(game_id, "http://sample:8545".to_string());
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            // Store `MyData` in application storage.
            .data(Arc::new(MyData {
                chain_maps: Mutex::new(HashMap::new()),
            }))
            .service(web::resource("/{id}").route(web::get().to(index)))
    })
    .bind("127.0.0.1:1337")
    .expect("Cannot bind to port 1337 :(")
    .run()
    .unwrap();
}
