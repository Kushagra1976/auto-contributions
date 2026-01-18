################################################################################
# Learning Objective:
# This tutorial will teach you how to procedurally generate a simple, text-based
# dungeon using Python. We will focus on the concept of "random walk" to
# create rooms and corridors, and then visualize it using simple characters.
# This is a fundamental technique in procedural content generation.
################################################################################

import random

# Define the dimensions of our dungeon grid.
DUNGEON_WIDTH = 50
DUNGEON_HEIGHT = 20

# Define characters for different dungeon elements.
# '#' represents a wall, '.' represents a floor/room.
WALL = '#'
FLOOR = '.'

def create_empty_dungeon(width, height):
    """
    Creates a 2D list (grid) representing an empty dungeon filled with walls.
    This serves as our canvas before we start carving out rooms and corridors.

    Args:
        width (int): The desired width of the dungeon grid.
        height (int): The desired height of the dungeon grid.

    Returns:
        list[list[str]]: A 2D list filled with WALL characters.
    """
    # We use a list comprehension for a concise way to create a 2D list.
    # The outer list represents rows, and inner lists represent columns.
    return [[WALL for _ in range(width)] for _ in range(height)]

def is_valid_position(x, y, width, height):
    """
    Checks if a given (x, y) coordinate is within the bounds of the dungeon.
    This is crucial to prevent errors when trying to access out-of-bounds indices.

    Args:
        x (int): The x-coordinate (column).
        y (int): The y-coordinate (row).
        width (int): The width of the dungeon.
        height (int): The height of the dungeon.

    Returns:
        bool: True if the position is valid, False otherwise.
    """
    return 0 <= x < width and 0 <= y < height

def carve_room(dungeon, x, y, room_width, room_height):
    """
    Carves out a rectangular room of a given size at a specified position
    by changing WALL characters to FLOOR characters.

    Args:
        dungeon (list[list[str]]): The dungeon grid.
        x (int): The starting x-coordinate (left edge) of the room.
        y (int): The starting y-coordinate (top edge) of the room.
        room_width (int): The width of the room to carve.
        room_height (int): The height of the room to carve.
    """
    # Iterate through the rows and columns that define the room.
    for dy in range(room_height):
        for dx in range(room_width):
            # Calculate the absolute position on the dungeon grid.
            current_x = x + dx
            current_y = y + dy
            # Only carve if the position is valid to avoid going out of bounds.
            if is_valid_position(current_x, current_y, len(dungeon[0]), len(dungeon)):
                dungeon[current_y][current_x] = FLOOR

def generate_dungeon(width, height, num_rooms, max_room_size):
    """
    Generates a text-based dungeon using a randomized approach.
    It first creates an empty grid, then places rooms, and finally connects them.

    Args:
        width (int): The width of the dungeon.
        height (int): The height of the dungeon.
        num_rooms (int): The desired number of rooms to place.
        max_room_size (int): The maximum width/height of a single room.

    Returns:
        list[list[str]]: The generated dungeon grid.
    """
    dungeon = create_empty_dungeon(width, height)
    rooms = [] # Keep track of the rooms we've placed.

    for _ in range(num_rooms):
        # Randomly determine room dimensions.
        room_w = random.randint(3, max_room_size)
        room_h = random.randint(3, max_room_size)
        # Randomly determine room position, ensuring it's within bounds
        # and has some padding from the edges.
        # We subtract room dimensions to ensure the whole room fits.
        # We add 1 to ensure rooms don't start exactly at the edge.
        room_x = random.randint(1, width - room_w - 1)
        room_y = random.randint(1, height - room_h - 1)

        # For simplicity, we're not checking for room overlaps here,
        # but in a more complex generator, you would.
        carve_room(dungeon, room_x, room_y, room_w, room_h)
        # Store the center coordinates of the room for later corridor carving.
        rooms.append((room_x + room_w // 2, room_y + room_h // 2))

    # Connect rooms with corridors (simple straight lines for now).
    # We connect each room to the previous one.
    for i in range(1, len(rooms)):
        # Get the center coordinates of the current and previous room.
        x1, y1 = rooms[i-1]
        x2, y2 = rooms[i]

        # Carve horizontally first.
        # random.randint(min(x1, x2), max(x1, x2)) picks a random point between the two x-coordinates.
        for x in range(min(x1, x2), max(x1, x2) + 1):
            if is_valid_position(x, y1, width, height):
                dungeon[y1][x] = FLOOR

        # Then carve vertically.
        for y in range(min(y1, y2), max(y1, y2) + 1):
            if is_valid_position(x2, y, width, height):
                dungeon[y][x2] = FLOOR

    return dungeon

def visualize_dungeon(dungeon):
    """
    Prints the dungeon grid to the console.

    Args:
        dungeon (list[list[str]]): The dungeon grid to visualize.
    """
    # Iterate through each row in the dungeon.
    for row in dungeon:
        # Join the characters in the row into a single string and print it.
        print("".join(row))

# --- Example Usage ---
if __name__ == "__main__":
    # Set parameters for dungeon generation.
    # These can be adjusted to create different types of dungeons.
    num_rooms_to_generate = 10
    maximum_room_size = 8

    print("Generating a simple text-based dungeon...")

    # Generate the dungeon.
    generated_dungeon = generate_dungeon(
        DUNGEON_WIDTH,
        DUNGEON_HEIGHT,
        num_rooms_to_generate,
        maximum_room_size
    )

    print("\nDungeon Generated:")
    # Visualize the generated dungeon.
    visualize_dungeon(generated_dungeon)