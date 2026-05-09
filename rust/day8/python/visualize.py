#!/usr/bin/env python3
"""
3D Visualization Script for Day 8 Coordinates

This script reads 3D coordinate data from a text file and creates
an interactive 3D scatter plot using matplotlib.

Usage:
    python visualize.py --input ../test_input.txt
    python visualize.py --input ../test_input.txt --output plot.png
"""

import argparse
import sys
import os
from pathlib import Path
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np


def parse_coordinates(file_path):
    """
    Parse 3D coordinates from a CSV file.

    Args:
        file_path (str): Path to the input file

    Returns:
        tuple: (x_coords, y_coords, z_coords) as numpy arrays
    """
    try:
        coordinates = []
        with open(file_path, "r") as file:
            for line_num, line in enumerate(file, 1):
                line = line.strip()
                if not line:  # Skip empty lines
                    continue

                parts = line.split(",")
                if len(parts) != 3:
                    print(
                        f"Warning: Line {line_num} has {len(parts)} values, expected 3"
                    )
                    continue

                try:
                    x, y, z = map(float, parts)
                    coordinates.append((x, y, z))
                except ValueError as e:
                    print(f"Warning: Line {line_num} contains non-numeric data: {e}")
                    continue

        if not coordinates:
            raise ValueError("No valid coordinates found in file")

        # Convert to numpy arrays
        coords_array = np.array(coordinates)
        return coords_array[:, 0], coords_array[:, 1], coords_array[:, 2]

    except FileNotFoundError:
        raise FileNotFoundError(f"Input file not found: {file_path}")
    except Exception as e:
        raise Exception(f"Error parsing file: {e}")


def create_3d_plot(x, y, z, output_file=None):
    """
    Create a 3D scatter plot of the coordinates.

    Args:
        x, y, z (array-like): Coordinate arrays
        output_file (str, optional): Path to save the plot
    """
    # Calculate distances from origin for color mapping
    distances = np.sqrt(x**2 + y**2 + z**2)

    # Create figure and 3D axis
    fig = plt.figure(figsize=(10, 8))
    ax = fig.add_subplot(111, projection="3d")

    # Create scatter plot with color based on distance from origin
    scatter = ax.scatter(
        x,
        y,
        z,
        c=distances,
        cmap="viridis",
        s=60,
        alpha=0.7,
        edgecolors="black",
        linewidth=0.5,
    )

    # Add colorbar
    cbar = plt.colorbar(scatter, ax=ax, shrink=0.8, aspect=20)
    cbar.set_label("Distance from Origin", rotation=270, labelpad=20)

    # Set labels and title
    ax.set_xlabel("X Coordinate")
    ax.set_ylabel("Y Coordinate")
    ax.set_zlabel("Z Coordinate")
    ax.set_title("3D Visualization of Day 8 Coordinates")

    # Add grid
    ax.grid(True, alpha=0.3)

    # Set viewing angle for better perspective
    ax.view_init(elev=20, azim=45)

    # Add text annotations for extreme points
    max_dist_idx = np.argmax(distances)
    min_dist_idx = np.argmin(distances)

    ax.text(
        x[max_dist_idx],
        y[max_dist_idx],
        z[max_dist_idx],
        "  Farthest",
        fontsize=8,
        color="red",
    )
    ax.text(
        x[min_dist_idx],
        y[min_dist_idx],
        z[min_dist_idx],
        "  Closest",
        fontsize=8,
        color="blue",
    )

    plt.tight_layout()

    # Save or show the plot
    if output_file:
        plt.savefig(output_file, dpi=300, bbox_inches="tight")
        print(f"Plot saved to: {output_file}")
    else:
        plt.show()


def main():
    """Main function to handle command line arguments and orchestrate visualization."""
    parser = argparse.ArgumentParser(
        description="3D Visualization of Day 8 Coordinates",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python visualize.py --input ../test_input.txt
  python visualize.py --input ../test_input.txt --output plot.png
  python visualize.py --input custom_data.txt --output result.png
        """,
    )

    parser.add_argument(
        "--input",
        "-i",
        required=True,
        help="Path to input file containing 3D coordinates (format: x,y,z per line)",
    )

    parser.add_argument(
        "--output",
        "-o",
        help="Path to save the plot (PNG format). If not provided, displays interactively.",
    )

    args = parser.parse_args()

    try:
        # Validate input file exists
        input_path = Path(args.input)
        if not input_path.exists():
            print(f"Error: Input file '{args.input}' does not exist.")
            sys.exit(1)

        # Parse coordinates
        print(f"Parsing coordinates from: {args.input}")
        x, y, z = parse_coordinates(args.input)
        print(f"Successfully parsed {len(x)} coordinates")

        # Print statistics
        distances = np.sqrt(x**2 + y**2 + z**2)
        print(f"Coordinate ranges:")
        print(f"  X: {x.min():.1f} to {x.max():.1f}")
        print(f"  Y: {y.min():.1f} to {y.max():.1f}")
        print(f"  Z: {z.min():.1f} to {z.max():.1f}")
        print(f"  Distance from origin: {distances.min():.1f} to {distances.max():.1f}")

        # Create visualization
        print("Creating 3D visualization...")
        create_3d_plot(x, y, z, args.output)

        if not args.output:
            print("Interactive plot displayed. Close the window to exit.")

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
