# F1 Track Optimization System

A Rust-based system for optimizing F1 race strategies considering fuel consumption, tire wear, and pit stop timing.

## Features

- **Graph-based track modeling** with nodes and edges
- **Optimal path finding** using resource-constrained shortest path algorithms
- **Visual track representation** with ASCII and Graphviz DOT output
- **Multiple car configurations** with different fuel capacities and tire costs
- **Pit stop optimization** at designated track nodes

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
- Optimal race strategies for each car
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

## Algorithm

The system uses a resource-constrained shortest path algorithm that considers:
- Fuel consumption per track segment
- Tire wear accumulation
- Pit stop penalties and benefits
- Multiple lap completion requirements