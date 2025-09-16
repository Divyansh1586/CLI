use colored::*;
use crate::input_parser::RaceData;
use petgraph::graph::Graph as PetGraph;
use petgraph::dot::{Dot, Config};
use std::process::Command;

pub fn print_graph_visualization(race_data: &RaceData) {
    println!("\n{}", "=== TRACK VISUALIZATION ===".bold().cyan());
    
    // Print basic track info
    println!("Track: {} nodes, {} edges, {} pit nodes", 
        race_data.n.to_string().green(), 
        race_data.m.to_string().blue(),
        race_data.pit_nodes.len().to_string().red()
    );
    
    // Create adjacency list for visualization
    let mut adj: Vec<Vec<(usize, u32)>> = vec![Vec::new(); race_data.n];
    for edge in &race_data.edges {
        adj[edge.u].push((edge.v, edge.distance));
    }
    
    // Print nodes with their connections
    println!("\n{}", "Track Layout:".bold().yellow());
    for i in 0..race_data.n {
        let node_color = if race_data.pit_nodes.contains(&i) {
            i.to_string().red().bold()
        } else {
            i.to_string().green()
        };
        
        print!("Node {}: ", node_color);
        
        if adj[i].is_empty() {
            println!("{}", "DEAD END".red());
        } else {
            let mut connections = Vec::new();
            for &(neighbor, dist) in &adj[i] {
                let neighbor_color = if race_data.pit_nodes.contains(&neighbor) {
                    neighbor.to_string().red().bold()
                } else {
                    neighbor.to_string().green()
                };
                connections.push(format!("{} (dist:{})", neighbor_color, dist.to_string().cyan()));
            }
            println!("{}", connections.join(" -> "));
        }
    }
    
    // Print pit nodes as detours
    if !race_data.pit_nodes.is_empty() {
        println!("\n{}", "Pit Stop Locations:".bold().red());
        for &pit_node in &race_data.pit_nodes {
            println!("  Node {}: [PIT] - refuel and change tires", pit_node.to_string().red().bold());
        }
    }
    
    // Print a simple ASCII track representation
    println!("\n{}", "ASCII Track Map:".bold().yellow());
    print_ascii_track(race_data, &adj);
    
    // Print car configurations
    println!("\n{}", "Car Configurations:".bold().magenta());
    for (i, car) in race_data.cars.iter().enumerate() {
        let car_color = match i % 3 {
            0 => Color::Green,
            1 => Color::Blue,
            _ => Color::Magenta,
        };
        println!("  Car {}: Fuel={}, Tyre Cost={}", 
            (i + 1).to_string().color(car_color),
            car.fuel_capacity.to_string().color(car_color),
            car.tyre_cost.to_string().color(car_color)
        );
    }
}

fn print_ascii_track(race_data: &RaceData, _adj: &Vec<Vec<(usize, u32)>>) {
    // Simple linear representation for small tracks
    if race_data.n <= 10 {
        let mut track_line = String::new();
        for i in 0..race_data.n {
            if race_data.pit_nodes.contains(&i) {
                track_line.push_str(&format!("[P{}]", i));
            } else {
                track_line.push_str(&format!("[{}]", i));
            }
            if i < race_data.n - 1 {
                track_line.push_str(" -> ");
            }
        }
        println!("  {}", track_line);
    } else {
        // For larger tracks, show a more compact representation
        println!("  Track too large for ASCII visualization ({} nodes)", race_data.n);
        println!("  Use the detailed node connections above for reference");
    }
    
    // Show edge distances
    println!("\n  Edge Distances:");
    for edge in &race_data.edges {
        let from_color = if race_data.pit_nodes.contains(&edge.u) {
            edge.u.to_string().red().bold()
        } else {
            edge.u.to_string().green()
        };
        let to_color = if race_data.pit_nodes.contains(&edge.v) {
            edge.v.to_string().red().bold()
        } else {
            edge.v.to_string().green()
        };
        println!("    {} -> {} (distance: {})", 
            from_color, 
            to_color, 
            edge.distance.to_string().cyan()
        );
    }
}

pub fn print_race_strategy(car_id: usize, path_info: &crate::graph_rcspp::PathInfo, race_data: &RaceData) {
    let car_color = match car_id % 3 {
        0 => Color::Green,
        1 => Color::Blue,
        _ => Color::Magenta,
    };
    
    println!("\n{}", format!("=== CAR {} RACE STRATEGY ===", car_id + 1).bold().color(car_color));
    println!("Total Race Time: {} units", path_info.total_time.to_string().color(car_color));
    
    if path_info.pit_stops.is_empty() {
        println!("Pit Stops: {}", "None".yellow());
    } else {
        println!("Pit Stops:");
        for (lap, node) in &path_info.pit_stops {
            println!("  Lap {} at Node {} (PIT)", 
                lap.to_string().color(Color::Yellow),
                node.to_string().red().bold()
            );
        }
    }
    
    println!("Race Path: {}", 
        path_info.node_sequence.iter()
            .map(|&node| {
                if race_data.pit_nodes.contains(&node) {
                    format!("[P{}]", node).red().bold().to_string()
                } else {
                    format!("[{}]", node).color(car_color).to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" -> ")
    );
}

pub fn export_dot(race_data: &RaceData, output_path: &str) -> std::io::Result<()> {
    let mut g = PetGraph::<String, String>::new();
    let mut nodes = Vec::with_capacity(race_data.n);
    for i in 0..race_data.n {
        let label = if race_data.pit_nodes.contains(&i) { format!("P{}", i) } else { i.to_string() };
        nodes.push(g.add_node(label));
    }
    for e in &race_data.edges {
        g.add_edge(nodes[e.u], nodes[e.v], format!("{}", e.distance));
    }
    let dot = format!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    std::fs::write(output_path, dot)
}

pub fn export_highlighted_dots(
    race_data: &RaceData,
    path_info: &crate::graph_rcspp::PathInfo,
    start_node: usize,
    total_laps: u32,
    output_prefix: &str,
) -> std::io::Result<()> {
    // Build adjacency for quick edge distance lookup
    let mut adj: Vec<std::collections::HashMap<usize, u32>> = vec![Default::default(); race_data.n];
    for e in &race_data.edges {
        adj[e.u].insert(e.v, e.distance);
    }

    // Function to write one DOT with highlighted edges and lap colors
    let write_dot = |segments: &[(usize, usize)], lap_colors: &[&str], file_path: &str| -> std::io::Result<()> {
        let mut dot = String::new();
        dot.push_str("digraph Track {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=circle, style=filled, fillcolor=white];\n");
        // Nodes
        for i in 0..race_data.n {
            if race_data.pit_nodes.contains(&i) {
                dot.push_str(&format!("  {} [shape=box, fillcolor=mistyrose, color=red, label=\"P{}\"];\n", i, i));
            } else {
                dot.push_str(&format!("  {} [label=\"{}\"];\n", i, i));
            }
        }
        // Edges default
        for e in &race_data.edges {
            let lbl = adj[e.u].get(&e.v).cloned().unwrap_or(0);
            dot.push_str(&format!("  {} -> {} [label=\"{}\", color=gray80];\n", e.u, e.v, lbl));
        }
        // Highlighted segments with lap-specific colors and annotations
        for (i, (u, v)) in segments.iter().enumerate() {
            let lbl = adj[*u].get(v).cloned().unwrap_or(0);
            let color = lap_colors.get(i % lap_colors.len()).unwrap_or(&"blue");
            // Add timing and resource info to edge labels
            let edge_label = format!("{} (t:{}, f:-{}, ty:+{})", lbl, lbl, lbl, lbl);
            dot.push_str(&format!("  {} -> {} [label=\"{}\", color={}, penwidth=3.0];\n", u, v, edge_label, color));
        }
        dot.push_str("}\n");
        std::fs::write(file_path, dot)
    };

    // Split the path by laps using start_node boundaries
    let mut laps: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for (idx, &node) in path_info.node_sequence.iter().enumerate() {
        if idx == 0 { current.push(node); continue; }
        current.push(node);
        if node == start_node {
            laps.push(current.clone());
            current.clear();
            current.push(node); // start next lap from start node
            if laps.len() as u32 >= total_laps { break; }
        }
    }
    if !current.is_empty() && (laps.len() as u32) < total_laps { laps.push(current); }

    // Ensure images/ directory exists
    std::fs::create_dir_all("images")?;

    // Lap colors: blue, green, red, orange, purple
    let lap_colors = ["blue", "green", "red", "orange", "purple"];

    // Write per-lap highlighted DOTs
    for (i, lap_nodes) in laps.iter().enumerate() {
        if lap_nodes.len() < 2 { continue; }
        let mut segs: Vec<(usize, usize)> = Vec::new();
        for w in lap_nodes.windows(2) { segs.push((w[0], w[1])); }
        let file = format!("images/{}_lap{}.dot", output_prefix, i + 1);
        let color = lap_colors.get(i).unwrap_or(&"blue");
        write_dot(&segs, &[color], &file)?;
        
        // Auto-render to SVG if dot is available
        let svg_file = file.replace(".dot", ".svg");
        if Command::new("dot").arg("-Tsvg").arg(&file).arg("-o").arg(&svg_file).output().is_ok() {
            println!("Rendered {}", svg_file.green());
        }
    }

    // Also write a full highlighted version with all lap colors
    let mut full_segs: Vec<(usize, usize)> = Vec::new();
    for w in path_info.node_sequence.windows(2) { full_segs.push((w[0], w[1])); }
    let file = format!("images/{}_full.dot", output_prefix);
    write_dot(&full_segs, &lap_colors, &file)?;
    
    // Auto-render full version to SVG
    let svg_file = file.replace(".dot", ".svg");
    if Command::new("dot").arg("-Tsvg").arg(&file).arg("-o").arg(&svg_file).output().is_ok() {
        println!("Rendered {}", svg_file.green());
    }

    Ok(())
}
