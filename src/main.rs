use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

pub mod input_parser;
pub mod graph_rcspp;

// Struct to hold the data that will be sent to the frontend
#[derive(Serialize, Deserialize, Clone)]
pub struct WebOutput {
    pub race_data: input_parser::RaceData,
    pub graph_edges: Vec<(usize, usize, u32)>, // (u, v, distance)
    pub optimal_paths: Vec<Option<graph_rcspp::PathInfo>>,
}

// Shared application data
pub struct AppData {
    pub web_output: WebOutput,
}

// Handler for the /data endpoint
#[get("/data")]
async fn get_data(app_data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&app_data.web_output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("F1 Track Optimization System - Web Server");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file_path>", args[0]);
        std::process::exit(1);
    }
    let input_file_path = &args[1];

    let race_data = match input_parser::parse_input(input_file_path) {
        Ok(data) => {
            println!("Successfully parsed input: Nodes: {}, Edges: {}, Pit Nodes: {}, Cars: {}", data.n, data.m, data.pit_nodes.len(), data.cars.len());
            data
        },
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            std::process::exit(1);
        }
    };

    let graph = graph_rcspp::Graph::new(race_data.n, &race_data.edges);
    println!("Graph created with {} nodes and {} edges.", race_data.n, race_data.m);

    if race_data.cars.is_empty() {
        eprintln!("Error: No car configurations provided in the input.");
        std::process::exit(1);
    }

    let mut optimal_paths: Vec<Option<graph_rcspp::PathInfo>> = Vec::new();

    for (i, car_config) in race_data.cars.iter().enumerate() {
        let start_node = 0;
        let end_node = race_data.n - 1;
        let total_laps = 1; 
        let pit_stop_penalty = 60;
        let tyre_wear_distance = 100;

        let path_info = graph_rcspp::find_optimal_path(
            &race_data,
            &graph,
            start_node,
            end_node,
            total_laps,
            car_config,
            pit_stop_penalty,
            tyre_wear_distance,
        );
        optimal_paths.push(path_info);
    }

    // Convert graph.adj to a simpler format for JSON serialization
    let mut graph_edges_for_web: Vec<(usize, usize, u32)> = Vec::new();
    for (u, edges) in &graph.adj {
        for (v, distance) in edges {
            graph_edges_for_web.push((*u, *v, *distance));
        }
    }

    let web_output = WebOutput {
        race_data: race_data.clone(), // Clone race_data if needed for other parts
        graph_edges: graph_edges_for_web,
        optimal_paths,
    };

    let app_data = web::Data::new(AppData { web_output });

    println!("Starting web server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_data)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
