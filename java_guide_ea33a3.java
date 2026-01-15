// Learning Objective: This tutorial teaches how to generate intricate fractal art
// in Java using recursive algorithms. We will focus on creating a simple
// fractal tree, demonstrating the core principles of recursion for generating
// self-similar patterns.

import javax.swing.*;
import java.awt.*;

public class FractalTreeGenerator extends JPanel {

    // --- Configuration ---
    private static final int WIDTH = 800; // Width of the drawing panel
    private static final int HEIGHT = 700; // Height of the drawing panel
    private static final int MAX_DEPTH = 10; // Maximum recursion depth (controls detail)
    private static final double BRANCH_LENGTH_FACTOR = 0.7; // How much shorter each branch gets
    private static final double ANGLE_OFFSET = Math.PI / 4; // Angle deviation for branches (45 degrees)

    // --- Main Application Setup ---
    public static void main(String[] args) {
        // Create a window (JFrame) to hold our fractal art
        JFrame frame = new JFrame("Fractal Tree Generator");
        // Set the window to close when the user clicks the close button
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        // Create an instance of our fractal generator panel
        FractalTreeGenerator fractalPanel = new FractalTreeGenerator();
        // Add the fractal panel to the frame
        frame.add(fractalPanel);
        // Size the frame to fit its components
        frame.pack();
        // Center the window on the screen
        frame.setLocationRelativeTo(null);
        // Make the window visible
        frame.setVisible(true);
    }

    // --- JPanel Override for Drawing ---
    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g); // Call the parent class's paintComponent to handle background etc.
        Graphics2D g2d = (Graphics2D) g; // Cast to Graphics2D for more advanced drawing capabilities

        // Enable anti-aliasing for smoother lines
        g2d.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

        // Define the starting point of the tree (bottom center of the panel)
        int startX = getWidth() / 2;
        int startY = getHeight() - 50; // Slightly above the bottom edge

        // Define the initial length of the main trunk
        int initialLength = 150;

        // Define the initial angle of the main trunk (pointing straight up)
        double initialAngle = -Math.PI / 2; // -90 degrees or pointing upwards

        // Draw the fractal tree by calling the recursive function
        // The initial call starts the process from the trunk
        drawBranch(g2d, startX, startY, initialLength, initialAngle, MAX_DEPTH);
    }

    // --- Recursive Function to Draw a Branch ---
    private void drawBranch(Graphics2D g2d, double x1, double y1, double length, double angle, int depth) {
        // --- Base Case: Stopping Condition ---
        // If we've reached the maximum recursion depth or the branch is too short, stop drawing.
        // This is crucial to prevent infinite recursion.
        if (depth == 0 || length < 5) {
            return;
        }

        // --- Calculate Endpoint of the Current Branch ---
        // Using trigonometry:
        // x2 = x1 + length * cos(angle)
        // y2 = y1 + length * sin(angle)
        // Note: In Java's Graphics coordinate system, positive Y is downwards, hence the negative sign for y.
        double x2 = x1 + length * Math.cos(angle);
        double y2 = y1 + length * Math.sin(angle);

        // --- Draw the Current Branch ---
        g2d.setStroke(new BasicStroke(Math.max(1, depth / 2))); // Thicker lines for deeper branches
        g2d.drawLine((int) x1, (int) y1, (int) x2, (int) y2); // Draw the line segment

        // --- Recursive Calls: Creating Sub-Branches ---

        // Branch 1: Left branch
        // The new length is the current length reduced by the factor.
        // The new angle is the current angle plus an offset (turning left).
        drawBranch(g2d, x2, y2, length * BRANCH_LENGTH_FACTOR, angle - ANGLE_OFFSET, depth - 1);

        // Branch 2: Right branch
        // Similar to the left branch, but the angle offset is added (turning right).
        drawBranch(g2d, x2, y2, length * BRANCH_LENGTH_FACTOR, angle + ANGLE_OFFSET, depth - 1);

        // Optional: Add a third, straighter branch for more complexity
        if (depth > MAX_DEPTH / 3) { // Only add if depth is significant
            drawBranch(g2d, x2, y2, length * BRANCH_LENGTH_FACTOR * 0.8, angle, depth - 1);
        }
    }

    // --- JPanel Configuration ---
    @Override
    public Dimension getPreferredSize() {
        return new Dimension(WIDTH, HEIGHT); // Define the preferred size of our panel
    }
}