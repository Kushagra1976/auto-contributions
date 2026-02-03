// Learning Objective: This tutorial demonstrates how to generate
// intricate fractal patterns in Java using recursion and graphical
// drawing. We will focus on the concept of self-similarity inherent
// in fractals and how recursion beautifully models this.

import javax.swing.*;
import java.awt.*;

// FractalPanel is a Swing component where we will draw our fractal.
public class FractalGenerator extends JPanel {

    // The maximum depth of recursion. Higher values create more detailed fractals.
    // This controls the complexity and computation time.
    private final int maxDepth;

    // Constructor to initialize the panel with a specific recursion depth.
    public FractalGenerator(int maxDepth) {
        this.maxDepth = maxDepth;
        // Set a preferred size for the drawing area.
        setPreferredSize(new Dimension(600, 600));
    }

    // This method is called automatically by Swing to paint the component.
    // It's where our drawing logic will go.
    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g); // Always call the superclass method first.
        Graphics2D g2d = (Graphics2D) g; // Cast to Graphics2D for more advanced features.

        // Set rendering hints for smoother drawing.
        g2d.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

        // Define the starting point and size of our initial shape.
        // For a fractal like the Sierpinski triangle, we start with a large triangle.
        int width = getWidth();
        int height = getHeight();
        int size = Math.min(width, height) - 50; // Leave some margin.
        int x = (width - size) / 2;
        int y = (height - size) / 2;

        // Set the initial color for drawing.
        g2d.setColor(Color.BLUE);

        // Start the recursive drawing process.
        // We'll pass the graphics context, the starting coordinates, and the initial size.
        drawSierpinskiTriangle(g2d, x, y, size, maxDepth);
    }

    // This is the core recursive function for drawing the Sierpinski triangle.
    private void drawSierpinskiTriangle(Graphics2D g2d, int x, int y, int size, int depth) {
        // Base case: If the recursion depth reaches 0, stop drawing.
        // This prevents infinite recursion.
        if (depth <= 0) {
            // Optionally, draw a small filled triangle at the deepest level for some visual cue.
            // g2d.fillRect(x + size / 2, y + size / 2, 2, 2); // For example.
            return;
        }

        // Recursive step:
        // We divide the current triangle into three smaller triangles
        // and recursively call this function for each of them.

        // Calculate the coordinates of the three vertices of the current triangle.
        int x1 = x;
        int y1 = y;
        int x2 = x + size;
        int y2 = y;
        int x3 = x + size / 2;
        int y3 = y + size;

        // Draw the outline of the current triangle (optional, but helpful for visualization).
        g2d.drawPolygon(new int[]{x1, x2, x3}, new int[]{y1, y2, y3}, 3);

        // Calculate the size of the smaller triangles.
        int newSize = size / 2;

        // Recursively draw the three smaller Sierpinski triangles.
        // 1. Bottom-left triangle: Starts at (x, y) with newSize.
        drawSierpinskiTriangle(g2d, x, y, newSize, depth - 1);

        // 2. Top-middle triangle: Starts at (x + newSize/2, y) with newSize.
        // Note: The x-coordinate is adjusted to center it above the base of the previous one.
        drawSierpinskiTriangle(g2d, x + newSize / 2, y, newSize, depth - 1);

        // 3. Bottom-right triangle: Starts at (x + newSize, y + newSize) with newSize.
        // Note: The x and y coordinates are adjusted to place it correctly.
        drawSierpinskiTriangle(g2d, x + newSize, y + newSize, newSize, depth - 1);
    }

    // Main method to create and display the fractal.
    public static void main(String[] args) {
        // Set the desired recursion depth for the fractal.
        // Higher values mean more detail but take longer to compute.
        int recursionDepth = 6; // Try values from 1 to 10.

        // Create a JFrame to hold our drawing panel.
        JFrame frame = new JFrame("Sierpinski Triangle Fractal");
        // Set the default close operation for the frame.
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        // Create an instance of our FractalGenerator panel.
        FractalGenerator fractalPanel = new FractalGenerator(recursionDepth);
        // Add the fractal panel to the frame.
        frame.add(fractalPanel);
        // Pack the frame to size itself based on the preferred size of its components.
        frame.pack();
        // Center the frame on the screen.
        frame.setLocationRelativeTo(null);
        // Make the frame visible.
        frame.setVisible(true);
    }
}