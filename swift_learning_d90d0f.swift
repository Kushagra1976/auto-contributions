// Objective: Learn to create a basic real-time audio visualizer in Swift using Core Audio for audio analysis and SpriteKit for visual rendering.
// This tutorial focuses on extracting audio amplitude data and displaying it as a simple bar graph.

import SwiftUI
import AVFoundation
import SpriteKit

// MARK: - Audio Analysis Class

// This class handles capturing audio input and analyzing its amplitude.
class AudioAnalyzer {
    private var audioEngine: AVAudioEngine!
    private var audioFormat: AVAudioFormat!
    private var bufferSize: AVAudioFrameCount = 1024 // The number of frames in each audio buffer. Larger buffers can provide more detail but might introduce latency.
    private var amplitude: Float = 0.0 // Stores the calculated average amplitude of the current audio buffer.

    // Callback to notify the visualizer when new amplitude data is available.
    var amplitudeUpdateHandler: ((Float) -> Void)?

    init() {
        setupAudioEngine()
    }

    // Configures the Core Audio engine for input and analysis.
    private func setupAudioEngine() {
        audioEngine = AVAudioEngine() // Initialize the audio engine.
        // Create an audio format object. This specifies the sample rate and channel count for our audio processing.
        audioFormat = audioEngine.inputNode.outputFormat(forBus: 0)

        // Attach a tap to the input node to get audio data.
        audioEngine.inputNode.installTap(onBus: 0, bufferSize: bufferSize, format: audioFormat) { buffer, time in
            self.analyzeAudioBuffer(buffer) // Call our analysis function for each incoming buffer.
        }
    }

    // Analyzes the incoming audio buffer to calculate its average amplitude.
    private func analyzeAudioBuffer(_ buffer: AVAudioPCMBuffer) {
        guard let floatChannelData = buffer.floatChannelData else { return } // Get the audio data as floating-point numbers.
        let frameLength = buffer.frameLength // The number of samples in this buffer.
        var accumulatedAmplitude: Float = 0.0 // To sum up the amplitudes for averaging.

        // Iterate through each sample in the audio buffer.
        for i in 0..<Int(frameLength) {
            // Get the absolute value of the audio sample. We are interested in the magnitude of the sound wave.
            let sample = abs(floatChannelData[0][i]) // Assuming mono input for simplicity.
            accumulatedAmplitude += sample // Add the sample's magnitude to our sum.
        }

        // Calculate the average amplitude by dividing the accumulated sum by the number of frames.
        amplitude = accumulatedAmplitude / Float(frameLength)

        // Clamp the amplitude to a reasonable range (0 to 1) for visualization.
        amplitude = min(amplitude * 2.0, 1.0) // Multiply by 2 to amplify quieter sounds and clamp to 1.

        // Notify the delegate (our visualizer) with the new amplitude value.
        amplitudeUpdateHandler?(amplitude)
    }

    // Starts the audio engine to begin capturing and analyzing audio.
    func startAnalyzing() {
        do {
            try audioEngine.start() // Start the audio engine.
            print("Audio engine started.")
        } catch {
            print("Error starting audio engine: \(error.localizedDescription)")
        }
    }

    // Stops the audio engine.
    func stopAnalyzing() {
        audioEngine.stop() // Stop the audio engine.
        print("Audio engine stopped.")
    }
}

// MARK: - SpriteKit Scene for Visualization

// This SpriteKit scene will display the audio visualization.
class VisualizerScene: SKScene {
    private var bars: [SKShapeNode] = [] // An array to hold our visualization bars.
    private let numberOfBars: Int = 50 // The total number of bars to display.
    private let barSpacing: CGFloat = 2.0 // The space between each bar.
    private let maxBarHeight: CGFloat = 150.0 // The maximum possible height for a bar.

    // Called when the scene is presented. Sets up the initial visualization elements.
    override func didMove(to view: SKView) {
        backgroundColor = SKColor.black // Set the background color of the scene.
        setupBars() // Create and position the initial bars.
    }

    // Creates and positions the individual bars that make up the visualization.
    private func setupBars() {
        // Calculate the total width occupied by the bars and their spacing.
        let totalBarWidth = CGFloat(numberOfBars) * 10.0 // Assuming each bar has a default width of 10.
        let totalSpacing = CGFloat(numberOfBars - 1) * barSpacing
        let totalWidth = totalBarWidth + totalSpacing
        let startX = (size.width - totalWidth) / 2.0 // Calculate the starting X position to center the bars.

        for i in 0..<numberOfBars {
            // Create a rectangular shape node for each bar.
            let bar = SKShapeNode(rectOf: CGSize(width: 10.0, height: 1.0)) // Initial height of 1.
            bar.fillColor = SKColor.green // Set the fill color of the bar.
            bar.strokeColor = SKColor.clear // No outline for the bars.

            // Calculate the X position for the current bar.
            let xPos = startX + CGFloat(i) * (10.0 + barSpacing)
            // Position the bar at the bottom center of the screen.
            bar.position = CGPoint(x: xPos + 5.0, y: size.height / 2.0 - maxBarHeight / 2.0) // Adjust Y to center vertically with base at bottom.
            bar.zPosition = 1.0 // Ensure bars are drawn above the background.

            bars.append(bar) // Add the created bar to our array.
            addChild(bar) // Add the bar to the scene.
        }
    }

    // Updates the height of each bar based on the provided amplitude.
    func updateBars(with amplitude: Float) {
        // Iterate through each bar and update its height.
        for (index, bar) in bars.enumerated() {
            // Calculate the height of the current bar.
            // The height is proportional to the amplitude and the maximum bar height.
            // We use the index to create a slight variation in responsiveness across bars.
            let barHeight = maxBarHeight * CGFloat(amplitude) * (1.0 + CGFloat(index % 5) * 0.1)
            // Apply a smooth animation to the height change.
            let scaleAction = SKAction.scaleTo(y: barHeight, duration: 0.05) // Short duration for responsiveness.
            bar.run(scaleAction)
        }
    }
}

// MARK: - SwiftUI View to host the SpriteKit Scene

// This SwiftUI View acts as a container for our SpriteKit visualizer.
struct VisualizerView: View {
    @State private var audioAnalyzer = AudioAnalyzer() // Our audio analysis engine.
    @State private var sceneView: SKView? // Reference to the SKView.

    var body: some View {
        SpriteView(scene: createScene()) // Embeds the SpriteKit scene.
            .onAppear {
                // When the view appears, start the audio analysis.
                audioAnalyzer.startAnalyzing()
                // Ensure the scene view is available to pass data.
                if let skView = self.sceneView {
                    // Assign the scene to the SKView so we can access it.
                    if skView.scene == nil {
                        skView.presentScene(createScene())
                    }
                }
            }
            .onDisappear {
                // When the view disappears, stop the audio analysis.
                audioAnalyzer.stopAnalyzing()
            }
            .edgesIgnoringSafeArea(.all) // Extend the visualization to the edges of the screen.
            .background(Color.black) // Ensure the background is black.
            .onTapGesture {
                // Example: Restart analysis on tap.
                audioAnalyzer.stopAnalyzing()
                audioAnalyzer.startAnalyzing()
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity) // Make the view fill available space.
            .coordinateSpace(name: "visualizer") // Define a coordinate space name.
            .background(
                // Attach a GeometryReader to get the frame of the view.
                GeometryReader { geometry in
                    Color.clear // Invisible color to capture frame size.
                        .onAppear {
                            // Store the SKView reference when it's available.
                            self.sceneView = extractSKView(from: geometry)
                        }
                }
            )
    }

    // Creates and configures the SpriteKit scene.
    private func createScene() -> SKScene {
        let scene = VisualizerScene(size: CGSize(width: 300, height: 400)) // Initial size, will adapt.
        scene.scaleMode = .aspectFit // Ensure the scene fits within the view.

        // Set up the amplitude update handler on the audio analyzer.
        audioAnalyzer.amplitudeUpdateHandler = { amplitude in
            // This closure is called whenever new amplitude data is available.
            // We update the bars in the SpriteKit scene on the main thread.
            DispatchQueue.main.async {
                // Ensure the scene is cast to our custom VisualizerScene type.
                if let visualizerScene = scene as? VisualizerScene {
                    visualizerScene.updateBars(with: amplitude) // Update the bars.
                }
            }
        }
        return scene
    }

    // Helper to try and extract the SKView from the GeometryReader's view hierarchy.
    private func extractSKView(from geometry: GeometryProxy) -> SKView? {
        var currentView: Any? = geometry. ? // Start with the current view.
        for _ in 0..<10 { // Iterate a few times to traverse the view hierarchy.
            if let skView = currentView as? SKView {
                return skView // Found the SKView.
            }
            // Attempt to get the superview. This is a bit of a hack and might break in future SwiftUI versions.
            let mirror = Mirror(reflecting: currentView ?? self)
            if let displayPoint = mirror.children.first(where: { $0.label == "displayPoint" })?.value,
               let superview = (displayPoint as AnyObject).?.superview {
                currentView = superview
            } else {
                break
            }
        }
        return nil
    }
}

// MARK: - Example Usage

// A simple SwiftUI App to demonstrate the VisualizerView.
@main
struct AudioVisualizerApp: App {
    var body: some Scene {
        WindowGroup {
            VisualizerView() // Present our visualizer.
        }
    }
}