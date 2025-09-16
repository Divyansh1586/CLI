document.addEventListener('DOMContentLoaded', () => {
    const networkElement = document.getElementById('network');
    const carInfoElement = document.getElementById('car-info');

    const carColors = {
        0: '#FF0000', // Red
        1: '#0000FF', // Blue
        2: '#00FF00', // Green
        3: '#FFFF00', // Yellow
        4: '#FF00FF', // Magenta
    };

    const pitStopNodeColor = '#FFA500'; // Orange
    const carAnimationSpeed = 1000; // milliseconds per step

    let network = null;
    let currentCarIndex = 0;
    let animationInterval = null;

    async function fetchData() {
        try {
            const response = await fetch('/data');
            const data = await response.json();
            console.log('Fetched data:', data);
            return data;
        } catch (error) {
            console.error('Error fetching data:', error);
            carInfoElement.innerHTML = '<p>Error loading data. Please ensure the Rust backend is running.</p>';
            return null;
        }
    }

    function drawGraph(graphData) {
        const nodes = new vis.DataSet();
        const edges = new vis.DataSet();

        // Add nodes
        for (let i = 0; i < graphData.race_data.n; i++) {
            let color = '#97C2E5'; // Default node color
            if (graphData.race_data.pit_nodes.includes(i)) {
                color = pitStopNodeColor; // Pit stop nodes are orange
            }
            nodes.add({ id: i, label: `Node ${i}`, color: color });
        }

        // Add edges
        graphData.graph_edges.forEach(edge => {
            edges.add({ from: edge[0], to: edge[1], label: `${edge[2]}`, arrows: 'to' });
        });

        const data = { nodes: nodes, edges: edges };
        const options = {
            physics: {
                enabled: true,
                barnesHut: {
                    gravitationalConstant: -2000,
                    centralGravity: 0.3,
                    springLength: 95,
                    springConstant: 0.04,
                    damping: 0.09,
                    avoidOverlap: 0.5
                },
                solver: 'barnesHut'
            },
            nodes: {
                shape: 'dot',
                size: 20,
                font: { size: 14, color: '#333' },
                borderWidth: 2,
            },
            edges: {
                width: 2,
                font: { align: 'top' },
                color: { inherit: 'from' },
            },
            layout: {
                improvedLayout: true
            }
        };

        network = new vis.Network(networkElement, data, options);

        // Stabilize network after initial layout
        network.once('stabilizationIterationsDone', function () {
            network.setOptions({ physics: { enabled: false } });
        });
    }

    async function startVisualization() {
        const data = await fetchData();
        if (!data) return;

        drawGraph(data);
        visualizeCars(data);
    }

    function visualizeCars(graphData) {
        const cars = graphData.race_data.cars;
        const optimalPaths = graphData.optimal_paths;

        carInfoElement.innerHTML = '';

        currentCarIndex = 0;
        clearInterval(animationInterval);
        animateNextCar(cars, optimalPaths, graphData);
    }

    function animateNextCar(cars, optimalPaths, graphData) {
        if (currentCarIndex >= cars.length) {
            carInfoElement.innerHTML += '<div class="car-section"><h3>All cars visualized!</h3></div>';
            return;
        }

        const car = cars[currentCarIndex];
        const pathInfo = optimalPaths[currentCarIndex];
        const carColor = carColors[currentCarIndex % Object.keys(carColors).length];

        const carSection = document.createElement('div');
        carSection.className = 'car-section';
        carSection.innerHTML = `<h3 class="car-title" style="color: ${carColor};">Car ${currentCarIndex + 1} (Fuel: ${car.fuel_capacity}, Tyre Cost: ${car.tyre_cost})</h3>`;

        if (!pathInfo) {
            carSection.innerHTML += '<p>No feasible race strategy found.</p>';
            carInfoElement.appendChild(carSection);
            currentCarIndex++;
            setTimeout(() => animateNextCar(cars, optimalPaths, graphData), 2000); // Delay before next car
            return;
        }

        carSection.innerHTML += `<p>Total Time: ${pathInfo.total_time}</p>`;
        carSection.innerHTML += `<p>Pit Stops: ${pathInfo.pit_stops.length > 0 ? pathInfo.pit_stops.map(ps => `Node ${ps[1]}`).join(', ') : 'None'}</p>`;
        carInfoElement.appendChild(carSection);

        let pathStep = 0;
        let currentFuel = car.fuel_capacity;
        let currentTyreDistance = 0;

        // Reset node colors before animating new car
        network.body.data.nodes.forEach(node => {
            let defaultColor = '#97C2E5';
            if (graphData.race_data.pit_nodes.includes(node.id)) {
                defaultColor = pitStopNodeColor;
            }
            network.body.data.nodes.update({ id: node.id, color: defaultColor, borderWidth: 2 });
        });

        animationInterval = setInterval(() => {
            if (pathStep < pathInfo.node_sequence.length) {
                const currentNodeId = pathInfo.node_sequence[pathStep];

                // Update fuel and tyre distance (simplified for visualization)
                if (pathStep > 0) {
                    const prevNodeId = pathInfo.node_sequence[pathStep - 1];
                    const edge = graphData.graph_edges.find(e => e[0] === prevNodeId && e[1] === currentNodeId);
                    if (edge) {
                        currentFuel -= edge[2]; // edge[2] is distance/travel_time
                        currentTyreDistance += edge[2];
                    }
                }

                // Check for pit stop at current node
                const isPitStop = graphData.race_data.pit_nodes.includes(currentNodeId) && pathInfo.pit_stops.some(ps => ps[1] === currentNodeId);
                if (isPitStop) {
                    currentFuel = car.fuel_capacity;
                    currentTyreDistance = 0;
                    carSection.innerHTML += `<p class="pit-stop">PIT STOP at Node ${currentNodeId}! Fuel reset, tires changed.</p>`;
                }

                // Highlight current node for the car
                network.body.data.nodes.update({ id: currentNodeId, color: carColor, borderWidth: 5, borderWidthSelected: 5 });

                // Reset color of previous node after it moves away
                if (pathStep > 0) {
                    const prevNodeId = pathInfo.node_sequence[pathStep - 1];
                    if (prevNodeId !== currentNodeId) { // Only reset if actually moved
                        let defaultColor = '#97C2E5';
                        if (graphData.race_data.pit_nodes.includes(prevNodeId)) {
                            defaultColor = pitStopNodeColor;
                        }
                        network.body.data.nodes.update({ id: prevNodeId, color: defaultColor, borderWidth: 2 });
                    }
                }

                carSection.innerHTML += `<p>Moving to Node ${currentNodeId} (Fuel: ${currentFuel}, Tyre: ${currentTyreDistance})</p>`;

                pathStep++;
            } else {
                clearInterval(animationInterval);
                // Reset color of the last node after animation finishes
                const lastNodeId = pathInfo.node_sequence[pathInfo.node_sequence.length - 1];
                let defaultColor = '#97C2E5';
                if (graphData.race_data.pit_nodes.includes(lastNodeId)) {
                    defaultColor = pitStopNodeColor;
                }
                network.body.data.nodes.update({ id: lastNodeId, color: defaultColor, borderWidth: 2 });

                currentCarIndex++;
                setTimeout(() => animateNextCar(cars, optimalPaths, graphData), 2000); // Delay before next car
            }
        }, carAnimationSpeed);
    }

    startVisualization();
});
