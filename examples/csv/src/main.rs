use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use bitmap_trie::dictionary::AttributeSearch;
use csvexample::CsvDictionary;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::error::Error;
use std::io::{Cursor, Read};
use std::sync::Arc;
use std::time::SystemTime;

// Define a serializable wrapper for the search response
#[derive(Serialize)]
struct SearchResponse<'a> {
    results: Vec<SearchResultJson<'a>>,
}

#[derive(Serialize)]
struct SearchResultJson<'a> {
    term: &'a str,
    attribute: &'a str,
    original_entry: &'a str,
    attribute_index: usize,
    position: usize,
    dictionary_index: usize,
}

struct AppState {
    dict: Arc<CsvDictionary>,
}

#[derive(Deserialize)]
struct SearchQuery {
    term: String,
}

#[derive(Deserialize)]
struct IdGet {
    id: String,
}

fn read_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[get("/search")]
async fn srca(data: web::Data<AppState>, query: web::Query<SearchQuery>) -> impl Responder {
    let dict = Arc::clone(&data.dict);
    let resp = dict.search(&query.term);

    // Wrap the response in a serializable structure
    let json_response = SearchResponse {
        results: resp
            .iter()
            .map(|r| SearchResultJson {
                term: &r.term,
                attribute: &r.attribute,
                original_entry: &r.original_entry,
                attribute_index: r.attribute_index,
                position: r.position,
                dictionary_index: r.dictionary_index,
            })
            .collect(),
    };

    let json = serde_json::to_string(&json_response).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[get("/get")]
async fn get(
    data: web::Data<AppState>,
    query: web::Query<IdGet>,
) -> impl Responder {
    let dict = Arc::clone(&data.dict);
    let resp = dict.get(&query.id);
    let json = serde_json::to_string(&resp).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("CSV Dictionary Example");
    println!("======================");
    let attributes = vec![
        ("Title".to_string(), AttributeSearch::Multiple),
        ("Authors".to_string(), AttributeSearch::Multiple),
        ("Description".to_string(), AttributeSearch::Multiple),
        ("Publisher".to_string(), AttributeSearch::Exact),
        ("Publish Date".to_string(), AttributeSearch::None),
        ("Price".to_string(), AttributeSearch::None),
    ];

    // Create and populate dictionary
    let dict = Arc::new(CsvDictionary::new(attributes));
    let app_state = web::Data::new(AppState {
        dict: Arc::clone(&dict), // your CsvDictionary instance
    });
    tokio::task::spawn(async move {
        let dict = Arc::clone(&dict);
        if let Err(e) = load_data(&dict) {
            eprintln!("Failed to load data: {}", e);
        }
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET"])
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(srca)
            .service(get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn load_data(dict: &CsvDictionary) -> Result<(), Box<dyn Error>> {
    println!("Loading data...");
    let now = SystemTime::now();

    // Sample CSV data (in a real application, this would come from a file)

    let csv_data = read_from_file("BooksDataset.csv")?;

    // Configure attributes for different types of searching

    let reader = Cursor::new(csv_data);
    let count = dict.load_from_csv(reader, true)?;
    println!(
        "Loaded {} records from CSV, in {} seconds\n",
        count,
        now.elapsed()?.as_secs_f64()
    );
    Ok(())
}
