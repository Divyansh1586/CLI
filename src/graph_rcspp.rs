use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Represents a state in the RCSPP algorithm
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub lap: u32,
    pub current_node: usize,
    pub current_fuel: u32,
    pub tyre_distance: u32,
}

// Represents information about a path leading to a state
#[derive(Debug, Clone)]
pub struct PathInfo {
    pub total_time: u32,
    pub pit_stops: Vec<(u32, usize)>,
    pub node_sequence: Vec<usize>,
}

// A label used in the Dijkstra-like algorithm
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Label {
    pub time: u32,
    pub state: State,
    pub path: Vec<usize>,
    pub pit_stops_taken: Vec<(u32, usize)>,
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        // Invert the ordering to make BinaryHeap a min-heap
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Graph {
    pub adj: HashMap<usize, Vec<(usize, u32)>>,
}

impl Graph {
    pub fn new(n: usize, edges: &Vec<crate::input_parser::Edge>) -> Self {
        let mut adj = HashMap::with_capacity(n);
        for i in 0..n {
            adj.insert(i, Vec::new());
        }
        for edge in edges {
            adj.get_mut(&edge.u).unwrap().push((edge.v, edge.distance));
        }
        Graph { adj }
    }
}

pub fn find_optimal_path(
    race_data: &crate::input_parser::RaceData,
    graph: &Graph,
    start_node: usize,
    end_node: usize,
    total_laps: u32,
    car_config: &crate::input_parser::CarConfig,
    pit_stop_penalty: u32, // Assuming a default pit stop penalty
    tyre_wear_distance: u32, // Assuming a default tyre wear distance
) -> Option<PathInfo> {
    let mut min_heap = BinaryHeap::new();
    let mut best_times: HashMap<State, u32> = HashMap::new();
    let mut best_paths: HashMap<State, Vec<usize>> = HashMap::new();
    let mut best_pit_stops: HashMap<State, Vec<(u32, usize)>> = HashMap::new();

    // Initial state: lap 0, start_node, full fuel, 0 tyre distance, 0 time
    let initial_state = State {
        lap: 0,
        current_node: start_node,
        current_fuel: car_config.fuel_capacity,
        tyre_distance: 0,
    };
    let initial_label = Label {
        time: 0,
        state: initial_state.clone(),
        path: vec![start_node],
        pit_stops_taken: Vec::new(),
    };
    min_heap.push(initial_label);
    best_times.insert(initial_state.clone(), 0);
    best_paths.insert(initial_state.clone(), vec![start_node]);
    best_pit_stops.insert(initial_state, Vec::new());

    let mut overall_min_time = u32::MAX;
    let mut overall_optimal_path = None;
    let mut overall_optimal_pit_stops = None;

    while let Some(label) = min_heap.pop() {
        let current_time = label.time;
        let current_state = label.state;
        let current_path = label.path;
        let current_pit_stops = label.pit_stops_taken;

        // Dominance check
        
        if let Some(&existing_best_time) = best_times.get(&current_state) {
            if current_time > existing_best_time {
                continue;
            }
        }
        

        // Check if we reached the end of the race
        if current_state.current_node == end_node && current_state.lap + 1 == total_laps {
            if current_time < overall_min_time {
                overall_min_time = current_time;
                overall_optimal_path = Some(current_path.clone());
                overall_optimal_pit_stops = Some(current_pit_stops.clone());
            }
            continue; // Continue to explore other paths that might be better for other states
        }

        // Explore neighbors
        if let Some(neighbors) = graph.adj.get(&current_state.current_node) {
            for &(neighbor_node, edge_distance) in neighbors {
                // Case 1: Attempt to move without a pit stop
                if current_state.current_fuel >= edge_distance {
                    let new_fuel = current_state.current_fuel - edge_distance;
                    let new_tyre_distance = current_state.tyre_distance + edge_distance;
                    let travel_time = edge_distance;
                    let tyre_penalty = if new_tyre_distance > tyre_wear_distance {
                        (new_tyre_distance - tyre_wear_distance) * car_config.tyre_cost
                    } else {
                        0
                    };
                    let new_time = current_time + travel_time + tyre_penalty;

                    let new_state = State {
                        lap: if neighbor_node == start_node && current_state.current_node != start_node { current_state.lap + 1 } else { current_state.lap },
                        current_node: neighbor_node,
                        current_fuel: new_fuel,
                        tyre_distance: new_tyre_distance,
                    };

                    if new_time < *best_times.get(&new_state).unwrap_or(&u32::MAX) {
                        let mut new_path = current_path.clone();
                        new_path.push(neighbor_node);
                        min_heap.push(Label {
                            time: new_time,
                            state: new_state.clone(),
                            path: new_path.clone(),
                            pit_stops_taken: current_pit_stops.clone(),
                        });
                        // Re-enable for debugging
                        best_times.insert(new_state.clone(), new_time);
                        best_paths.insert(new_state.clone(), new_path);
                        best_pit_stops.insert(new_state, current_pit_stops.clone());
                        
                    }
                }

                // Case 2: Consider a pit stop if at a pit node, then move
                if race_data.pit_nodes.contains(&current_state.current_node) {
                    // After a pit stop, fuel is reset. Check if new fuel is enough for this edge.
                    if car_config.fuel_capacity >= edge_distance {
                        let new_fuel_after_pit_and_travel = car_config.fuel_capacity - edge_distance;
                        let new_tyre_distance_after_pit_and_travel = edge_distance; // Tyre resets, then covers edge distance
                        let travel_time = edge_distance; // Assuming time is equal to distance

                        // Calculate tyre wear penalty for this segment after pit
                        let tyre_penalty = if new_tyre_distance_after_pit_and_travel > tyre_wear_distance {
                            (new_tyre_distance_after_pit_and_travel - tyre_wear_distance) * car_config.tyre_cost
                        } else {
                            0
                        };

                        let new_time_after_pit = current_time + pit_stop_penalty + travel_time + tyre_penalty;

                        let new_state = State {
                            lap: if neighbor_node == start_node && current_state.current_node != start_node { current_state.lap + 1 } else { current_state.lap },
                            current_node: neighbor_node,
                            current_fuel: new_fuel_after_pit_and_travel,
                            tyre_distance: new_tyre_distance_after_pit_and_travel,
                        };

                        if new_time_after_pit < *best_times.get(&new_state).unwrap_or(&u32::MAX) {
                            let mut new_path = current_path.clone();
                            new_path.push(neighbor_node);
                            let mut new_pit_stops = current_pit_stops.clone();
                            new_pit_stops.push((current_state.lap, current_state.current_node));
                            min_heap.push(Label {
                                time: new_time_after_pit,
                                state: new_state.clone(),
                                path: new_path.clone(),
                                pit_stops_taken: new_pit_stops.clone(),
                            });
                            // Re-enable for debugging
                            best_times.insert(new_state.clone(), new_time_after_pit);
                            best_paths.insert(new_state.clone(), new_path);
                            best_pit_stops.insert(new_state, new_pit_stops);
                            
                        }
                    }
                }
            }
        }
    }

    if let (Some(path), Some(pit_stops)) = (overall_optimal_path, overall_optimal_pit_stops) {
        Some(PathInfo {
            total_time: overall_min_time,
            pit_stops: pit_stops,
            node_sequence: path,
        })
    } else {
        None
    }
}
