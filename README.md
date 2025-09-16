# F1 Track Optimization System

A Rust-based system for optimizing F1 race strategies considering fuel consumption, tire wear, and pit stop timing.

## Features

- **Graph-based track modeling** with nodes and edges
- **Optimal path finding** using resource-constrained shortest path algorithms
- **Visual track representation** with ASCII and Graphviz DOT output
- **Multiple car configurations** with different fuel capacities and tire costs
- **Pit stop optimization** at designated track nodes

## Algorithm: Resource-Constrained Shortest Path Problem (RCSPP)

The system uses a **Dijkstra-like state space exploration** algorithm that treats F1 racing as a multi-dimensional optimization problem.

### Core Approach
- **State Representation**: Each racing state is defined by 4 dimensions:
  - `lap`: Current lap number
  - `node`: Current track position
  - `fuel`: Remaining fuel level
  - `tire_wear`: Accumulated tire wear distance

- **Optimization Objective**: Minimize total race time while respecting resource constraints

### Key Algorithm Components

**🔍 State Space Exploration**
- Uses a min-heap (BinaryHeap) to explore states in order of increasing time cost
- Each label contains: total time, state vector, path history, and pit stop records

**⚡ Movement Strategies**
1. **Direct Movement**: Proceed to next node (if sufficient fuel exists)
2. **Pit Stop + Movement**: Refuel and change tires at pit nodes, then proceed

**🎯 Resource Constraints**
- **Fuel Constraint**: Cannot move without sufficient fuel
- **Tire Degradation**: Penalty applied when exceeding tire wear threshold
- **Pit Stop Penalty**: Time cost for refueling and tire changes

**🚀 Performance Optimizations**
- **Dominance Pruning**: Eliminates suboptimal states using best-time tracking
- **Multi-lap Handling**: Properly manages lap transitions at start/finish line
- **Dynamic Programming**: Avoids recomputing optimal paths to visited states

### Time Calculation Formula
```rust
total_time = travel_time + tire_penalty + pit_stop_penalty
```
Where:
- `travel_time = edge_distance`
- `tire_penalty = max(0, (tire_wear - threshold) * tire_cost)`
- `pit_stop_penalty` = fixed time cost when stopping

The algorithm terminates when reaching the finish line after completing all required laps with minimum total time.

## Input Format

The system expects an input file with the following format:

```
N                    # Number of track nodes
NP                   # Number of pit stop nodes  
C                    # Number of car configurations
M                    # Number of directed edges
# M lines of edges:
U V DISTANCE         # Edge from node U to V with distance
# NP lines of pit nodes:
PIT_NODE_INDEX       # Node index where pit stops are allowed
# C lines of car configs:
FUEL_CAPACITY TYRE_COST  # Car fuel capacity and tire cost
```

## Usage

```bash
cargo run input.txt
```

## Example Files

- `input_template.txt` - Template with explanations
- `examples/simple_track.txt` - Basic 4-node track
- `examples/complex_track.txt` - 6-node track with shortcuts
- `examples/oval_track.txt` - Classic oval layout
- `examples/minimal_track.txt` - Minimal 3-node test case

## Output

The system generates:
- Terminal visualization of the track layout
- Optimal race strategies for each car configuration
- Graphviz DOT file (`track.dot`) for visual rendering

To render the track as an image:
```bash
dot -Tsvg track.dot -o track.svg
dot -Tpng track.dot -o track.png
```

## Track Visualization

- **Green nodes**: Regular track checkpoints
- **Red nodes**: Pit stop locations  
- **Edges**: Track segments with distances
- **Car paths**: Color-coded optimal routes

## Technical Implementation

- **Language**: Rust for performance and memory safety
- **Data Structures**: Graph representation with adjacency lists
- **Algorithm Complexity**: O((V×F×T×L) × log(states)) where V=nodes, F=fuel levels, T=tire states, L=laps
- **Memory Optimization**: State pruning and dominance checking reduce memory usage