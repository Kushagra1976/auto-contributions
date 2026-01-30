// /////////////////////////////////////////////////////////////////////////////
// LEARNING OBJECTIVE:
// This tutorial will guide you through building a basic real-time audio visualizer.
// We will learn how to:
// 1. Access and play audio using the Web Audio API.
// 2. Analyze audio frequencies in real-time.
// 3. Draw dynamic visuals on an HTML Canvas based on the audio data.
// /////////////////////////////////////////////////////////////////////////////

// --- 1. Setting up the Audio Context and Source ---

// Create an AudioContext. This is the main entry point for the Web Audio API.
// It allows us to create and manage audio nodes.
const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

// Get a reference to the audio element from our HTML.
// We'll use this to load and play the audio file.
// IMPORTANT: For this to work, you need an <audio id="audioSource" src="your_audio_file.mp3"></audio> in your HTML.
const audioElement = document.getElementById('audioSource');

// Create an AudioBufferSourceNode. This node will play the audio we load.
const source = audioCtx.createBufferSource();

// Create an AnalyserNode. This node will analyze the audio data in real-time.
// We'll use its data to drive our visualization.
const analyser = audioCtx.createAnalyser();

// Connect the audio source to the analyser.
// This means the audio data will flow through the analyser before being played.
source.connect(analyser);

// Connect the analyser to the destination (your speakers).
// This ensures you can still hear the audio while visualizing it.
analyser.connect(audioCtx.destination);

// Set the FFT (Fast Fourier Transform) size for the analyser.
// This determines the frequency resolution of the data we get back.
// A higher FFT size gives more detailed frequency information but requires more processing.
analyser.fftSize = 2048; // Common value, balancing detail and performance

// --- 2. Preparing the Canvas for Visualization ---

// Get a reference to the canvas element from our HTML.
// You'll need a <canvas id="visualizerCanvas"></canvas> in your HTML.
const canvas = document.getElementById('visualizerCanvas');
// Set the canvas drawing context to 2D. This is what we'll use to draw shapes.
const ctx = canvas.getContext('2d');

// Set the canvas dimensions to fill the browser window.
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

// Calculate the buffer length needed for the analyser.
// This is half of the FFT size, representing the number of frequency bins.
const bufferLength = analyser.frequencyBinCount;
// Create a Uint8Array to store the audio frequency data.
// Uint8Array is an array of 8-bit unsigned integers, perfect for frequency data (0-255).
const dataArray = new Uint8Array(bufferLength);

// --- 3. The Visualization Logic (Animation Loop) ---

// This function will be called repeatedly to draw each frame of our visualization.
function draw() {
    // Request the next animation frame. This creates a smooth, efficient loop.
    requestAnimationFrame(draw);

    // Clear the entire canvas before drawing the new frame.
    // This prevents previous frames from lingering and creating a messy effect.
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Get the audio data from the analyser.
    // getByteFrequencyData populates our dataArray with frequency magnitudes (0-255).
    analyser.getByteFrequencyData(dataArray);

    // --- Drawing the Visualizer ---
    // We'll draw vertical bars representing different frequencies.

    // Define some drawing parameters.
    const barWidth = (canvas.width / bufferLength) * 1.5; // Width of each bar
    let x = 0; // Starting x-coordinate for drawing

    // Iterate through the dataArray to draw each bar.
    for (let i = 0; i < bufferLength; i++) {
        // Get the magnitude of the current frequency bin.
        const barHeight = dataArray[i];

        // Set the fill color of the bar. We'll use a gradient for a nicer effect.
        ctx.fillStyle = `hsl(${i / bufferLength * 360}, 100%, 50%)`; // Hue based on frequency

        // Draw the rectangle (bar) on the canvas.
        // Parameters: x, y, width, height
        // We position the bar from the bottom of the canvas, so y = canvas.height - barHeight.
        ctx.fillRect(x, canvas.height - barHeight, barWidth, barHeight);

        // Move the x-coordinate forward for the next bar.
        x += barWidth + 1; // Add 1 for a small gap between bars
    }
}

// --- 4. Example Usage and Event Handling ---

// This function starts the audio playback and the visualization.
function startVisualization() {
    // Check if the audio context is in a suspended state (common for user interaction reasons).
    // If so, resume it.
    if (audioCtx.state === 'suspended') {
        audioCtx.resume();
    }

    // If the audio source hasn't been connected yet (e.g., first time playing),
    // we need to load the audio file into the source.
    // This is a common pattern when using buffer sources.
    if (!source.buffer) {
        // Fetch the audio file as an ArrayBuffer.
        fetch(audioElement.src)
            .then(response => response.arrayBuffer())
            .then(arrayBuffer => {
                // Decode the ArrayBuffer into an AudioBuffer.
                audioCtx.decodeAudioData(arrayBuffer, (buffer) => {
                    source.buffer = buffer;
                    source.loop = true; // Loop the audio if desired
                    source.start(0); // Start playing the audio from the beginning
                    draw(); // Start the animation loop
                });
            })
            .catch(e => console.error("Error loading or decoding audio file:", e));
    } else {
        // If the audio has already been loaded, just start it again.
        // This handles cases where the user might pause and then restart.
        source.start(0);
        draw(); // Ensure the draw loop is running
    }
}

// Add an event listener to start the visualization when the audio element is ready to play.
// The 'canplaythrough' event fires when the user agent can play through the media
// without requiring further buffering.
audioElement.addEventListener('canplaythrough', () => {
    console.log("Audio is ready to play. Click anywhere to start visualization.");
});

// Add a click listener to the document to start the visualization.
// This is a common requirement for browser autoplay policies,
// which often require user interaction to initiate audio.
document.addEventListener('click', () => {
    startVisualization();
}, { once: true }); // The 'once: true' option means this listener will only run once.

// --- END OF TUTORIAL CODE ---
// To use this:
// 1. Create an HTML file with:
//    <audio id="audioSource" src="path/to/your/audio.mp3"></audio>
//    <canvas id="visualizerCanvas"></canvas>
// 2. Save the above Javascript code in a .js file (e.g., visualizer.js)
// 3. Link the JS file in your HTML: <script src="visualizer.js"></script>
// 4. Open the HTML file in your browser. Click anywhere on the page to start the visualization.