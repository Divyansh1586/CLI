use colored::*;
use std::thread;
use std::time::Duration;
pub mod input_parser;
pub mod graph_rcspp;
pub mod visual;

fn main() {
    println!("F1 Track Optimization System");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file_path>", args[0]);
        std::process::exit(1);
    }
    let input_file_path = &args[1];

    match input_parser::parse_input(input_file_path) {
        Ok(race_data) => {
            println!("Successfully parsed input:");
            println!("Nodes: {}, Edges: {}, Pit Nodes: {}, Cars: {}", race_data.n, race_data.m, race_data.pit_nodes.len(), race_data.cars.len());

            // Print graph visualization and export DOT (graphviz)
            visual::print_graph_visualization(&race_data);
            if let Err(e) = visual::export_dot(&race_data, "track.dot") {
                eprintln!("Failed to export DOT: {}", e);
            } else {
                println!("Exported track graph DOT to {} (render with: dot -Tsvg track.dot -o track.svg)", "track.dot".green());
            }

            let graph = graph_rcspp::Graph::new(race_data.n, &race_data.edges);
            println!("\nGraph created with {} nodes and {} edges.", race_data.n, race_data.m);

            if race_data.cars.is_empty() {
                eprintln!("Error: No car configurations provided in the input.");
                std::process::exit(1);
            }
            
            for (i, car_config) in race_data.cars.iter().enumerate() {
                let car_color = match i % 3 {
                    0 => Color::Green,
                    1 => Color::Blue,
                    _ => Color::Magenta,
                };

                println!("\n--- Processing Car Configuration {} ---", (i + 1).to_string().color(car_color));
                println!("  Fuel Capacity: {}, Tyre Cost: {}", car_config.fuel_capacity.to_string().color(car_color), car_config.tyre_cost.to_string().color(car_color));

                let start_node = 0;
                let end_node = race_data.n - 1;
                let total_laps = 1; // Default to 1 lap as K is not in Section 4.2 input
                let pit_stop_penalty = 60; // Realistic pit stop penalty
                let tyre_wear_distance = 100; // Realistic tyre wear distance

                match graph_rcspp::find_optimal_path(
                    &race_data,
                    &graph,
                    start_node,
                    end_node,
                    total_laps,
                    car_config,
                    pit_stop_penalty,
                    tyre_wear_distance,
                ) {
                    Some(path_info) => {
                        // Print race strategy visualization
                        visual::print_race_strategy(i, &path_info, &race_data);
                        // Export per-lap highlighted DOTs to images/
                        let prefix = format!("car{}_laps", i + 1);
                        if let Err(e) = visual::export_highlighted_dots(&race_data, &path_info, start_node, total_laps, &prefix) {
                            eprintln!("Failed to export per-lap DOTs for car {}: {}", i + 1, e);
                        } else {
                            println!("Saved per-lap DOTs under {}", "images/".green());
                            println!("Render with: dot -Tsvg images/{}_lap1.dot -o images/{}_lap1.svg", prefix, prefix);
                        }
                        
                        println!("\n--- Visualizing Car {}'s Journey ---", (i + 1).to_string().color(car_color));
                        let mut current_fuel = car_config.fuel_capacity;
                        let mut current_tyre_distance = 0;
                        let mut current_lap = 0; // Assuming starting lap 0

                        for (idx, &node) in path_info.node_sequence.iter().enumerate() {
                            let is_pit_stop_node = race_data.pit_nodes.contains(&node);
                            let mut performed_pit_stop_this_step = false;

                            if is_pit_stop_node && idx > 0 { // Check if a pit stop occurred at this node
                                // Check if this node was actually part of the recorded pit stops
                                if path_info.pit_stops.iter().any(|&(_lap, p_node)| p_node == node) {
                                    println!("  {} at Node {}. Refueling and changing tires.", "PIT STOP!".color(Color::Red), node.to_string().color(Color::Red));
                                    current_fuel = car_config.fuel_capacity; // Refuel
                                    current_tyre_distance = 0; // Reset tyre wear
                                    performed_pit_stop_this_step = true;
                                    thread::sleep(Duration::from_millis(500)); // Short pause for pit stop
                                }
                            }

                            if idx > 0 {
                                // Simulate fuel and tyre usage for the segment just completed
                                // This part is tricky as optimal_path doesn't store per-segment fuel/tyre usage history
                                // For visualization, we'll re-calculate based on previous node to current node
                                let prev_node = path_info.node_sequence[idx - 1];
                                if let Some(neighbors) = graph.adj.get(&prev_node) {
                                    for &(neighbor_node, edge_distance) in neighbors {
                                        if neighbor_node == node {
                                            if !performed_pit_stop_this_step {
                                                current_fuel = current_fuel.saturating_sub(edge_distance);
                                                current_tyre_distance += edge_distance;
                                            }
                                            break;
                                        }
                                    }
                                }
                            }

                            let node_display = if is_pit_stop_node {
                                node.to_string().color(Color::Red).to_string()
                            } else {
                                node.to_string().color(car_color).to_string()
                            };

                            println!("  Car {} at Node {}. Fuel: {}, Tyre Distance: {}", 
                                (i + 1).to_string().color(car_color),
                                node_display,
                                current_fuel.to_string().color(Color::Cyan),
                                current_tyre_distance.to_string().color(Color::Yellow)
                            );

                            if node == start_node && idx > 0 { // Increment lap if returned to start node
                                current_lap += 1;
                                println!("  --- Entering Lap {} ---", (current_lap + 1).to_string().color(Color::White));
                            }

                            thread::sleep(Duration::from_millis(700)); // Pause for readability
                        }
                        println!("--- Journey Complete for Car {} ---", (i + 1).to_string().color(car_color));
                        
                    },
                    None => {
                        println!("No feasible race strategy found for this car configuration.");
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            std::process::exit(1);
        }
    }
}
