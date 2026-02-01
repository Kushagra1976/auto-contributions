// This tutorial demonstrates how to programmatically generate beautiful fractal patterns using recursive algorithms in Java.
// We will focus on the concept of recursion and how it can be applied to create self-similar geometric structures.
// By the end of this tutorial, you will understand:
// 1. The core idea of recursive algorithms.
// 2. How to implement a simple fractal (e.g., a Sierpinski Triangle or a Koch Curve) using recursion.
// 3. Basic Java graphics to visualize the generated fractal.

import java.awt.Color;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.BasicStroke;
import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.SwingUtilities;

// A JPanel subclass that will be used to draw our fractal.
public class FractalGenerator extends JPanel {

    // The recursion depth controls how detailed the fractal will be.
    // Higher depth means more iterations and a more complex fractal.
    private int recursionDepth;

    // Constructor: Initializes the panel and sets the desired recursion depth.
    public FractalGenerator(int depth) {
        this.recursionDepth = depth;
    }

    // The paintComponent method is where all custom drawing happens.
    // Swing calls this method whenever the panel needs to be repainted.
    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g); // Always call the superclass method first.
        Graphics2D g2d = (Graphics2D) g; // Cast to Graphics2D for more drawing options.

        // Set rendering hints for smoother lines.
        g2d.setRenderingHint(java.awt.RenderingHints.KEY_ANTIALIASING, java.awt.RenderingHints.VALUE_ANTIALIAS_ON);

        // Set a background color for our drawing surface.
        setBackground(Color.WHITE);

        // Set the color of the fractal lines.
        g2d.setColor(Color.BLUE);

        // Set the stroke (line thickness). A thinner stroke is good for detailed fractals.
        g2d.setStroke(new BasicStroke(1.5f));

        // Define the initial canvas dimensions.
        int width = getWidth();
        int height = getHeight();

        // --- Example: Drawing a Sierpinski Triangle ---
        // A Sierpinski Triangle is a classic fractal. It's formed by starting with
        // an equilateral triangle, and then repeatedly removing smaller equilateral
        // triangles from its center.

        // We define the three initial points of our largest triangle.
        // These points are centered on the panel.
        double x1 = width / 2.0;
        double y1 = 50; // Top vertex
        double x2 = 50; // Left vertex
        double y2 = height - 50;
        double x3 = width - 50; // Right vertex
        double y3 = height - 50;

        // Call the recursive method to draw the Sierpinski Triangle.
        // The method takes the graphics object, the current recursion level,
        // and the coordinates of the three vertices of the triangle to draw.
        drawSierpinski(g2d, recursionDepth, x1, y1, x2, y2, x3, y3);
    }

    // This is the recursive method that draws the Sierpinski Triangle.
    // It breaks down the problem into smaller, self-similar sub-problems.
    private void drawSierpinski(Graphics2D g2d, int depth, double x1, double y1, double x2, double y2, double x3, double y3) {
        // Base Case: If the recursion depth reaches 0, we stop drawing.
        // This is crucial to prevent infinite recursion.
        if (depth == 0) {
            // Draw the current triangle.
            int[] xPoints = {(int) x1, (int) x2, (int) x3};
            int[] yPoints = {(int) y1, (int) y2, (int) y3};
            g2d.drawPolygon(xPoints, yPoints, 3);
            return; // Stop recursion for this branch.
        }

        // Recursive Step: If depth > 0, we divide the current triangle into three
        // smaller triangles and recursively call this method for each of them.

        // Calculate the midpoints of the sides of the current triangle.
        // These midpoints will define the vertices of the three smaller triangles.
        double mid12_x = (x1 + x2) / 2.0; // Midpoint of side 1-2
        double mid12_y = (y1 + y2) / 2.0;
        double mid23_x = (x2 + x3) / 2.0; // Midpoint of side 2-3
        double mid23_y = (y2 + y3) / 2.0;
        double mid31_x = (x3 + x1) / 2.0; // Midpoint of side 3-1
        double mid31_y = (y3 + y1) / 2.0;

        // Recursively draw the three smaller Sierpinski triangles.
        // Each call reduces the recursion depth by 1.

        // Top triangle: Vertices are (x1, y1), midpoint(1,2), midpoint(3,1)
        drawSierpinski(g2d, depth - 1, x1, y1, mid12_x, mid12_y, mid31_x, mid31_y);

        // Left triangle: Vertices are (x2, y2), midpoint(1,2), midpoint(2,3)
        drawSierpinski(g2d, depth - 1, x2, y2, mid12_x, mid12_y, mid23_x, mid23_y);

        // Right triangle: Vertices are (x3, y3), midpoint(3,1), midpoint(2,3)
        drawSierpinski(g2d, depth - 1, x3, y3, mid31_x, mid31_y, mid23_x, mid23_y);

        // Note: We don't draw the middle "inverted" triangle, which is how
        // the characteristic holes of the Sierpinski Triangle are formed.
    }

    // Example Usage: This is how you would run the fractal generator.
    public static void main(String[] args) {
        // The desired level of detail for the fractal.
        // A value of 5-7 is usually a good starting point for visualization.
        int depth = 6;

        // Use SwingUtilities.invokeLater to ensure that GUI creation happens on the Event Dispatch Thread (EDT).
        // This is a best practice in Swing applications to avoid threading issues.
        SwingUtilities.invokeLater(() -> {
            JFrame frame = new JFrame("Recursive Fractal Generator"); // Create a window.
            frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE); // Set what happens when the window is closed.
            frame.setSize(600, 600); // Set the initial size of the window.

            // Create an instance of our FractalGenerator panel.
            FractalGenerator fractalPanel = new FractalGenerator(depth);

            // Add the fractal drawing panel to the frame.
            frame.add(fractalPanel);

            // Make the window visible.
            frame.setVisible(true);
        });
    }
}