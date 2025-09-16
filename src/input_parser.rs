use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct CarConfig {
    pub fuel_capacity: u32,
    pub tyre_cost: u32,
}

pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub distance: u32,
}

pub struct RaceData {
    pub n: usize,
    pub np: usize,
    pub c: usize,
    pub m: usize,
    pub edges: Vec<Edge>,
    pub pit_nodes: Vec<usize>,
    pub cars: Vec<CarConfig>,
}

pub fn parse_input(file_path: &str) -> io::Result<RaceData> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let n: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, String::from("Missing N")))??.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, String::from("Invalid N")))?;
    let np: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, String::from("Missing NP")))??.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, String::from("Invalid NP")))?;
    let c: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, String::from("Missing C")))??.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, String::from("Invalid C")))?;
    let m: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, String::from("Missing M")))??.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, String::from("Invalid M")))?;

    let mut edges = Vec::with_capacity(m);
    for i in 0..m {
        let line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Missing edge line {}", i)))??;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid edge format on line {}", i)));
        }
        let u: usize = parts[0].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid U on line {}", i)))?;
        let v: usize = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid V on line {}", i)))?;
        let distance: u32 = parts[2].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Distance on line {}", i)))?;
        edges.push(Edge { u, v, distance });
    }

    let mut pit_nodes = Vec::with_capacity(np);
    for i in 0..np {
        let pit_node: usize = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Missing pit node line {}", i)))??.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Pit Node on line {}", i)))?;
        pit_nodes.push(pit_node);
    }

    let mut cars = Vec::with_capacity(c);
    for i in 0..c {
        let line = lines.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, format!("Missing car config line {}", i)))??;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid car config format on line {}", i)));
        }
        let fuel_capacity: u32 = parts[0].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Fuel Capacity on line {}", i)))?;
        let tyre_cost: u32 = parts[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid Tyre Cost on line {}", i)))?;
        cars.push(CarConfig { fuel_capacity, tyre_cost });
    }

    Ok(RaceData { n, np, c, m, edges, pit_nodes, cars })
}
