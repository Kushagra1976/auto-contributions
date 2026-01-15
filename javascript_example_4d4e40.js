// app.js (Main Thread Script)

// Learning Objective:
// Understand how to offload heavy computations (like image processing) to Web Workers
// to keep the main browser UI thread responsive and prevent freezing.
// We will implement client-side grayscale and sepia filters for images.

// 1. Get references to our HTML elements.
// These elements would typically be defined in an index.html file like this:
// <input type="file" id="imageLoader">
// <canvas id="originalCanvas"></canvas>
// <canvas id="processedCanvas"></canvas>
// <button id="grayscaleBtn">Grayscale</button>
// <button id="sepiaBtn">Sepia</button>
const fileInput = document.getElementById('imageLoader');
const originalCanvas = document.getElementById('originalCanvas');
const processedCanvas = document.getElementById('processedCanvas');
const grayscaleBtn = document.getElementById('grayscaleBtn');
const sepiaBtn = document.getElementById('sepiaBtn');

const ctxOriginal = originalCanvas.getContext('2d');
const ctxProcessed = processedCanvas.getContext('2d');

let originalImageData = null; // Stores the original image data for re-applying filters

// 2. Initialize the Web Worker.
// WHAT: `new Worker()` creates a new JavaScript thread running concurrently with the main thread.
// WHY: A Web Worker runs in a separate thread, preventing long-running scripts
// from blocking the main UI thread and keeping the user interface responsive.
const worker = new Worker('worker.js'); // The path to our worker script.

// 3. Handle messages coming back from the Web Worker.
// WHAT: `worker.onmessage` is an event handler that fires when the worker sends a message.
// WHY: When the worker finishes processing, it sends the new image data back.
// The main thread then updates the canvas.
worker.onmessage = function(event) {
    // We receive the processed ImageData object from the worker.
    // The `data` property of the event contains the data sent by `postMessage` in the worker.
    const { imageData } = event.data;

    // Set the dimensions of the processed canvas to match the image data.
    processedCanvas.width = imageData.width;
    processedCanvas.height = imageData.height;

    // Put the processed image data onto the processed canvas.
    // This is the final step where the UI is updated with the worker's result.
    ctxProcessed.putImageData(imageData, 0, 0);
    console.log('Image processing complete on processedCanvas!');
};

// 4. Handle potential errors from the Web Worker.
// WHAT: `worker.onerror` catches any uncaught errors that occur within the worker script.
// WHY: Essential for debugging and providing feedback to the user if something goes wrong
// during the background processing.
worker.onerror = function(error) {
    console.error('Web Worker Error:', error);
    alert('An error occurred in the image processing worker. Check console for details.');
};

// 5. Function to load an image from the file input onto the original canvas.
// WHAT: An event listener on the file input detects when a user selects a file.
// WHY: This is the entry point for getting an image into our application.
fileInput.addEventListener('change', function(e) {
    const file = e.target.files[0];
    if (!file) return; // Exit if no file was selected.

    // WHAT: FileReader is an API to read the contents of files.
    // WHY: We need to get the image data from the selected file so it can be drawn on the canvas.
    const reader = new FileReader();

    reader.onload = function(event) {
        // Create a new Image object.
        const img = new Image();

        img.onload = function() {
            // Set canvas dimensions to match the image.
            originalCanvas.width = img.width;
            originalCanvas.height = img.height;
            processedCanvas.width = img.width;
            processedCanvas.height = img.height;

            // Draw the original image onto the original canvas.
            ctxOriginal.drawImage(img, 0, 0);

            // Get the ImageData object from the original canvas.
            // WHAT: `getImageData` captures all the pixel data (RGBA values) from a specified area.
            // WHY: This raw pixel data is what we will send to the Web Worker for processing.
            originalImageData = ctxOriginal.getImageData(0, 0, img.width, img.height);

            // Clear the processed canvas initially.
            ctxProcessed.clearRect(0, 0, processedCanvas.width, processedCanvas.height);
            console.log('Original image loaded.');
        };
        // Set the image source to the result of the FileReader (a Data URL, Base64 encoded string).
        img.src = event.target.result;
    };

    // Read the file as a Data URL.
    reader.readAsDataURL(file);
});

// 6. Function to send image data and filter type to the Web Worker.
// WHAT: This function initiates the image processing by sending data to the worker.
// WHY: It's a common interface for applying different filters.
function applyFilter(filterType) {
    if (!originalImageData) {
        alert('Please load an image first!');
        return;
    }

    // WHAT: `worker.postMessage()` sends data to the worker thread.
    // WHY: The worker needs to know which image to process and what kind of processing to do.
    // ImageData objects can be efficiently transferred between threads using structured cloning,
    // which effectively moves the data rather than copying it, for performance.
    worker.postMessage({
        imageData: originalImageData, // The raw pixel data.
        filter: filterType            // The type of filter to apply ('grayscale' or 'sepia').
    });
    console.log(`Applying ${filterType} filter via Web Worker...`);
}

// 7. Event listeners for filter buttons.
// WHAT: These listeners trigger the `applyFilter` function when buttons are clicked.
// WHY: To allow the user to interact with the application and choose a filter.
grayscaleBtn.addEventListener('click', () => applyFilter('grayscale'));
sepiaBtn.addEventListener('click', () => applyFilter('sepia'));


// worker.js (Web Worker Script)

// Learning Objective:
// Implement heavy computational tasks (like image pixel manipulation) inside a Web Worker
// to avoid blocking the main browser thread.
// This script will receive image data and a filter type, process the pixels,
// and send the modified image data back.

// 1. Listen for messages from the main thread.
// WHAT: `self.onmessage` is the worker's event handler for incoming messages from the main thread.
// WHY: This is how the worker receives tasks (like image data and filter type) to perform.
// The `self` keyword refers to the global scope of the worker itself.
self.onmessage = function(event) {
    // WHAT: We destructure the received object to get `imageData` and `filter`.
    // WHY: The main thread sends us the raw pixel data to process and tells us
    // which filter to apply.
    const { imageData, filter } = event.data;

    // Create a new ImageData object with the same dimensions but for storing processed data.
    // WHAT: `new ImageData()` creates a blank image data object.
    // WHY: We operate on a copy of the pixel data to avoid directly modifying the original if possible,
    // and to ensure we have a structure to return.
    let processedImageData = new ImageData(imageData.width, imageData.height);
    // WHAT: `set()` copies the pixel data from the original to our new ImageData object.
    // WHY: We want to start with the original image's pixels before applying the filter.
    processedImageData.data.set(imageData.data);

    // 2. Apply the chosen filter.
    // WHAT: A `switch` statement is used to call the appropriate filter function based on `filter` type.
    // WHY: To handle different filter requests from the main thread in an organized way.
    switch (filter) {
        case 'grayscale':
            processedImageData = applyGrayscale(processedImageData);
            break;
        case 'sepia':
            processedImageData = applySepia(processedImageData);
            break;
        default:
            console.warn('Unknown filter type:', filter);
            // If an unknown filter is requested, just send back the original data.
            processedImageData = imageData;
            break;
    }

    // 3. Send the processed image data back to the main thread.
    // WHAT: `self.postMessage()` sends data back to the main thread.
    // WHY: The main thread needs the processed pixel data to update the canvas
    // with the filtered image.
    self.postMessage({ imageData: processedImageData });
};

// 4. Implement the Grayscale filter.
// WHAT: This function iterates through all pixels and converts them to grayscale.
// WHY: To visually represent the image using only shades of gray, removing color.
function applyGrayscale(imageData) {
    const data = imageData.data; // This is a Uint8ClampedArray representing RGBA values.

    // Pixels are stored sequentially as [R1, G1, B1, A1, R2, G2, B2, A2, ...].
    // So we loop by 4 to access each pixel's Red, Green, Blue, and Alpha components.
    for (let i = 0; i < data.length; i += 4) {
        // Calculate the average (or weighted average) of RGB values for luminance.
        // WHAT: A common formula for perceived brightness (luminance) is 0.299*R + 0.587*G + 0.114*B.
        // WHY: Simply averaging (R+G+B)/3 can result in a less accurate grayscale appearance
        // because the human eye is more sensitive to green light than red or blue.
        const avg = (data[i] * 0.299 + data[i + 1] * 0.587 + data[i + 2] * 0.114);

        // Set R, G, and B to the calculated average, effectively making it grayscale.
        data[i] = avg;     // Red component
        data[i + 1] = avg; // Green component
        data[i + 2] = avg; // Blue component
        // Alpha (data[i + 3]) remains unchanged for transparency.
    }
    return imageData; // Return the modified ImageData object.
}

// 5. Implement the Sepia filter.
// WHAT: This function applies sepia tones by adjusting RGB values based on a specific matrix calculation.
// WHY: To give the image an old-fashioned, brownish tone, simulating vintage photographs.
function applySepia(imageData) {
    const data = imageData.data;

    for (let i = 0; i < data.length; i += 4) {
        const r = data[i];
        const g = data[i + 1];
        const b = data[i + 2];

        // Apply sepia color transformation matrix.
        // WHAT: These formulas are standard for achieving a sepia effect by mixing R, G, B values.
        // WHY: They shift the color balance towards warmer, brown tones.
        const newR = (r * 0.393) + (g * 0.769) + (b * 0.189);
        const newG = (r * 0.349) + (g * 0.686) + (b * 0.168);
        const newB = (r * 0.272) + (g * 0.534) + (b * 0.131);

        // WHAT: `Math.min(255, ...)` clamps values to ensure they stay within the 0-255 range.
        // WHY: Pixel component values must be between 0 and 255 (inclusive). Values outside
        // this range would be invalid or cause unexpected visual results.
        data[i] = Math.min(255, newR);     // Red component
        data[i + 1] = Math.min(255, newG); // Green component
        data[i + 2] = Math.min(255, newB); // Blue component
        // Alpha (data[i + 3]) remains unchanged.
    }
    return imageData; // Return the modified ImageData object.
}

/*
Example Usage:

To run this tutorial, you would typically have an `index.html` file that links to `app.js`.
The `worker.js` file needs to be in the same directory as `app.js` (or adjust the path in `new Worker()`).

Here's a conceptual `index.html`:

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Web Worker Image Filters</title>
    <style>
        body { font-family: sans-serif; display: flex; flex-direction: column; align-items: center; margin: 20px; }
        .controls { margin-bottom: 20px; }
        button { margin: 5px; padding: 10px 15px; font-size: 1em; cursor: pointer; }
        canvas { border: 1px solid #ccc; margin: 10px; max-width: 48%; height: auto; display: block; }
        .canvas-container { display: flex; justify-content: center; width: 100%; max-width: 1200px; }
        h2 { text-align: center; }
    </style>
</head>
<body>
    <h1>Image Filters with Web Workers</h1>

    <div class="controls">
        <input type="file" id="imageLoader" accept="image/*">
        <button id="grayscaleBtn">Apply Grayscale</button>
        <button id="sepiaBtn">Apply Sepia</button>
    </div>

    <div class="canvas-container">
        <div>
            <h2>Original Image</h2>
            <canvas id="originalCanvas"></canvas>
        </div>
        <div>
            <h2>Processed Image</h2>
            <canvas id="processedCanvas"></canvas>
        </div>
    </div>

    <!-- Link your main script -->
    <script src="app.js"></script>
</body>
</html>

To run:
1. Save the JavaScript code above into two files: `app.js` and `worker.js`.
2. Save the HTML structure above into an `index.html` file.
3. Make sure all three files (`index.html`, `app.js`, `worker.js`) are in the same folder.
4. Open `index.html` in your web browser.
5. Use the "Choose File" button to select an image from your computer.
6. Click "Apply Grayscale" or "Apply Sepia" and observe the processed image without the browser UI freezing!
*/