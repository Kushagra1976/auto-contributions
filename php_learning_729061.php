<?php
/**
 * LEARNING OBJECTIVE:
 * This tutorial demonstrates how to build a dynamic PHP-powered data visualization tool
 * using Chart.js to display real-time sensor readings. We will focus on fetching data
 * from a simulated source (a PHP array in this example, but could be a database or API)
 * and preparing it in a format suitable for Chart.js to render a line graph.
 *
 * Key concepts covered:
 * - Basic HTML structure for a web page.
 * - Including JavaScript libraries (Chart.js).
 * - Using PHP to generate dynamic JavaScript data.
 * - Understanding how Chart.js datasets and labels work.
 * - Simulating real-time data for demonstration.
 */

// --- PHP Section: Data Simulation and Preparation ---

// In a real application, this data would come from a sensor, database, or API.
// For demonstration purposes, we'll use a PHP array.
// This array simulates sensor readings over time.
$sensorReadings = [
    // Each element represents a reading with a timestamp and a value.
    ['timestamp' => '2023-10-27 10:00:00', 'value' => 22.5],
    ['timestamp' => '2023-10-27 10:01:00', 'value' => 23.1],
    ['timestamp' => '2023-10-27 10:02:00', 'value' => 22.8],
    ['timestamp' => '2023-10-27 10:03:00', 'value' => 23.5],
    ['timestamp' => '2023-10-27 10:04:00', 'value' => 24.0],
    ['timestamp' => '2023-10-27 10:05:00', 'value' => 23.8],
    ['timestamp' => '2023-10-27 10:06:00', 'value' => 24.2],
    ['timestamp' => '2023-10-27 10:07:00', 'value' => 24.5],
    ['timestamp' => '2023-10-27 10:08:00', 'value' => 24.1],
    ['timestamp' => '2023-10-27 10:09:00', 'value' => 24.8],
    ['timestamp' => '2023-10-27 10:10:00', 'value' => 25.0],
];

// We need to extract the 'labels' (timestamps) and 'data' (values)
// into separate arrays that Chart.js can understand.

$labels = []; // Array to hold the timestamps for the X-axis.
$data = [];   // Array to hold the sensor values for the Y-axis.

// Loop through our simulated sensor readings to populate the arrays.
foreach ($sensorReadings as $reading) {
    // For simplicity, we'll format the timestamp to be more readable on the chart.
    // In a production environment, you might use a JavaScript date library for better formatting.
    $labels[] = date('H:i:s', strtotime($reading['timestamp'])); // Extract time part.
    $data[] = $reading['value']; // Add the sensor value.
}

// To make this data available to JavaScript, we'll convert it to a JSON string.
// JSON is a universally understood format for data exchange between server (PHP) and client (JavaScript).
$jsonDataLabels = json_encode($labels);
$jsonDataData = json_encode($data);

?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Real-time Sensor Readings</title>
    <!-- Include Chart.js library from a CDN (Content Delivery Network).
         This is the JavaScript charting library we will use. -->
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        /* Basic styling for the chart container to give it some size. */
        .chart-container {
            width: 80%;
            margin: 20px auto; /* Center the chart */
            height: 400px; /* Set a fixed height for the chart */
        }
    </style>
</head>
<body>

    <h1>Live Sensor Data Visualization</h1>
    <p>This chart displays simulated real-time temperature readings.</p>

    <div class="chart-container">
        <!-- This is where our chart will be rendered.
             The 'canvas' element is a requirement for Chart.js. -->
        <canvas id="sensorChart"></canvas>
    </div>

    <script>
        // --- JavaScript Section: Chart.js Configuration ---

        // Get the canvas element where the chart will be drawn.
        const ctx = document.getElementById('sensorChart').getContext('2d');

        // The Chart.js library uses a configuration object to define the chart.
        const sensorChart = new Chart(ctx, {
            // 'type' defines the kind of chart. 'line' is suitable for time-series data.
            type: 'line',
            // 'data' holds all the information about the chart's data and appearance.
            data: {
                // 'labels' are the categories or points on the X-axis.
                // We are using the PHP-generated JSON data here.
                // The <?php echo ... ?> syntax embeds PHP output directly into the JavaScript.
                labels: <?php echo $jsonDataLabels; ?>,
                // 'datasets' is an array of objects, where each object represents a data series.
                datasets: [{
                    label: 'Temperature (°C)', // A descriptive label for this data series.
                    // 'data' is an array of the actual data points for this series.
                    // Again, we embed the PHP-generated JSON data.
                    data: <?php echo $jsonDataData; ?>,
                    // Styling options for the line.
                    borderColor: 'rgb(75, 192, 192)', // Color of the line.
                    tension: 0.1 // Adds a slight curve to the line.
                }]
            },
            // 'options' allows for customization of the chart's appearance and behavior.
            options: {
                responsive: true, // Makes the chart resize with the container.
                maintainAspectRatio: false, // Allows the chart to fill its container height.
                scales: {
                    y: {
                        beginAtZero: false, // Don't force the Y-axis to start at zero.
                        title: {
                            display: true,
                            text: 'Temperature (°C)' // Label for the Y-axis.
                        }
                    },
                    x: {
                        title: {
                            display: true,
                            text: 'Time' // Label for the X-axis.
                        }
                    }
                },
                plugins: {
                    title: {
                        display: true,
                        text: 'Temperature Trends' // Title for the entire chart.
                    }
                }
            }
        });

        // --- Example Usage and Explanation ---
        // To run this code:
        // 1. Save it as an `.php` file (e.g., `sensor_dashboard.php`).
        // 2. Place it in a web server environment with PHP support (e.g., XAMPP, WAMP, MAMP, or a live server).
        // 3. Access the file through your web browser (e.g., `http://localhost/sensor_dashboard.php`).
        //
        // This example uses a static PHP array for sensor data. In a real-time scenario,
        // you would typically use AJAX (Asynchronous JavaScript and XML) to periodically fetch
        // new data from a PHP script (that might query a database or live sensor feed)
        // and then update the chart using Chart.js's `chart.update()` method without
        // a full page reload. This is how true "real-time" visualization is achieved.
    </script>

</body>
</html>