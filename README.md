System Requirements Specification (SRS) for the F1 Track Optimization System
1. Introduction
1.1 Purpose
The purpose of this document is to define the functional and non-functional requirements for the F1 Track Optimization System. This system is a console application designed to simulate and optimize a Formula 1 race strategy by determining the minimum total race time and the optimal pit stop schedule. This SRS serves as a foundational reference for developers, testers, and stakeholders, ensuring a shared understanding of the system's objectives and capabilities.
1.2 Scope
This system will be a stand-alone console application. Its primary function is to solve a constrained shortest path problem on a directed weighted graph to determine an optimal race strategy. The system will take a series of inputs related to the track, car performance, and race conditions, and it will output the minimum race time and the corresponding pit stop and path information. The system will not include a graphical user interface (GUI) or real-time, in-race data feeds. It is designed for pre-race strategic analysis.
1.3 Definitions, Acronyms, and Abbreviations
Term
Definition
SRS
System Requirements Specification
F1
Formula 1
RCSPP
Resource-Constrained Shortest Path Problem
N
Number of nodes (checkpoints) in the track graph
M
Number of edges (paths) in the track graph
K
Total number of laps to be completed
F
Fuel tank capacity (in distance units)
P
Pit stop time penalty (in seconds)
W
Tyre wear distance (in distance units)
B
Number of blocked edges

2. Overall Description
2.1 Product Perspective
The F1 Track Optimization System is a new, self-contained application. It does not interface with any other existing systems (e.g., data telemetry systems, live race data feeds). It will be a console-based application requiring manual input from a file or standard input.
2.2 Product Functions
The system will perform the following core functions:
Read and parse track and race data from a defined input format.
Model the race track as a directed, weighted graph.
Model a car's journey over K laps, considering multiple constraints.
Apply penalties for tyre degradation and blocked edges.
Optimize the path and pit stop schedule to achieve the minimum total race time.
Output the optimal race time, pit stop schedule, and the sequence of nodes for each lap.
2.3 User Characteristics
The primary users of this system are assumed to be F1 race strategists, data analysts, or engineers. These users are expected to have a basic understanding of computer systems and the ability to interact with a console application. They will be responsible for preparing the input data files and interpreting the output.
2.4 General Constraints
The system must run on a standard desktop or server environment.
The solution must be a console-based application.
The system must handle the specified input constraints (N≤105, M≤2×105, etc.) efficiently.
The implementation must be language-agnostic, though a common language like Python, C++, or Java is preferred for clarity and performance.
3. Specific Requirements
3.1 Functional Requirements
FR1: Track and Race Data Input
The system shall accept input data in the following format:
The first line will contain two integers, N and M, representing the number of nodes and edges, respectively.
The next M lines will describe the edges, each with five values: u v time distance penalty.
A line with three integers: F P W.
A single integer K.
A single integer B.
A list of B integers representing the blocked edge indices.
FR2: Graph and State Modeling
The system shall represent the track as a directed weighted graph with N nodes and M edges.
Each edge shall have attributes for time, distance, and penalty.
The system shall model the race state as a composite of (lap, current_node, current_fuel_level, tyre_distance_covered).
FR3: Race Simulation and Path Optimization
The system shall simulate the race for K laps.
The system shall identify the path that minimizes the total race time from the start to the end of the K laps. This problem can be modeled as a Resource-Constrained Shortest Path Problem (RCSPP).
The pathfinding algorithm must account for:
Fuel Constraint: The total distance traveled between pit stops cannot exceed the fuel capacity F.
Tyre Wear Penalty: For every unit of distance traveled over the tyre wear limit W, an additional penalty time must be added to the edge's original time.
Pit Stops: A pit stop at a designated node (lap, pit_node) adds a fixed time P to the total race time. A pit stop resets the fuel and tyre wear counters.
Blocked Edges: The system must treat the specified B blocked edges as impassable.
FR4: Output Generation
The system shall output the following information to the console:
The calculated minimum total race time (as a single numerical value).
A detailed pit stop schedule, listing the lap and checkpoint where each pit stop occurs.
The optimal sequence of nodes for each of the K laps that results in the minimum time.
3.2 Non-Functional Requirements
NFR1: Performance
The system shall be capable of finding a solution for a track with up to nodes and 2×105 edges within a reasonable time, given the NP-hard nature of the problem.
The solution must be computationally efficient to handle the specified constraints (N≤105,M≤2×105,K≤50,etc.). An optimal algorithm like a labeling-based approach for RCSPP is expected.
NFR2: Accuracy
The system shall guarantee that the outputted race time is the mathematically provable minimum, given the provided inputs and constraints.
NFR3: Reliability
The system shall handle invalid input formats gracefully, providing a clear error message and exiting.
The system shall handle cases where no feasible path exists (e.g., all paths are blocked, or a solution violates a constraint) and report this to the user.
NFR4: Usability
The system's interface shall be purely text-based (console).
The output shall be well-formatted and easy to read.
4. Constraints and Assumptions
4.1 Assumptions
Directed Graph: The track is a directed graph, meaning travel from u to v does not imply the reverse.
Unique Start/End: The race starts and ends at specific, pre-defined nodes.
Integer Values: All input parameters (N, M, time, distance, etc.) are positive integers.
Race Strategy: The only strategic decisions are when and where to make pit stops. The optimal path between checkpoints is always the one that minimizes the time cost under the given constraints.
4.2 Input Format
	N - Number of nodes
	NP - Number of pits
	C - Number of cars
	M - Number of edges
	For M times: 
		U—V (U:source, V:destination ; U and V are nodes)
		D (D:Distance between the nodes)
	For NP times:
		P (Nodes which are pits)
	For C times:
		Fcap (Fuel capacity of the car)
		Tc (Ty
5. Use Cases
Use Case 1: Standard Optimization
The user runs the program and provides the path to an input file.
The program reads the track, race, and car parameters.
The program computes the optimal path and pit stop schedule for K laps.
The program outputs the total time, pit stop schedule, and the sequence of nodes per lap.
Use Case 2: Infeasible Solution
The user provides an input file where the fuel capacity is too low to complete a full lap.
The program detects that no feasible solution exists.
The program outputs an error message indicating that no valid race strategy can be found under the given constraints.

