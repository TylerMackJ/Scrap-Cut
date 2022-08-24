#include "line2dLL.h"

enum square = { NotTaken, Taken, Good, Bad };

int main() {
    int sheetW = 48;
    int sheetH = 96;
    int resolution = 2;

    int gridW = sheetW / resolution;
    int gridH = sheetH / resolution;

    // Create Rough Grid
    enum square* square = malloc(sizeof(enum square*) * gridH);
    for (int i = 0; i < gridH; i++) {
        square[i] = malloc(sizeof(enum square) * gridW)
        for (int j = 0; j < gridW; j++) {
            square[i][j] = NotTaken;
        }
    }

    // Place shapes into grid
    // Loop over gcode
        int cutting = 0;
        // if m64
            cutting = 1;
        // if cutting && (G00 || G01)
            // Step over line by 1/2 resolution steps
                // Make stepped over squares Taken
                // Label which shape it belongs to
        // if cutting && G02
            // Step over arc by 1/2 resolution steps
                // Make stepped over squares Taken
                // Label which shape it belongs to
        // if m65
            cutting = 0;

    // Find all the good and bad squares
    for (int i = 0; i < gridH; i++) {
        for (int j = 0; j < gridW; j++) {
            if (grid[i][j] = NotTaken) {
                if ((grid[i + 1][j] != Taken || grid[i - 1][j] != Taken) && (grid[i][j + 1] != Taken || grid[i][j - 1] != Taken)) {
                    grid[i][j] = Good;
                } else {
                    grid[i][j] = Bad;
                }
            }
        }
    }

    // Find all cuts
    line2dLL* cuts = newLine2dLL();
    for (int i = 0; i < gridH; i++) {
        for (int j = 0; j < gridW; j++) {
            // If Bad is next to Good and all by itself change to G?

            // Find Good-Bad cuts
            if (grid[i][j] = Bad && (grid[i + 1][j] == Good || grid[i - 1][j] == Good || grid[i][j + 1] == Good || grid[i][j  1] == Good)) {
                // Find each shape that has a Taken touching the current square
                // Make cut from middle of this square to the closest point of each shape
            }

            // Find Taken-Good-Wall cuts
            // Check top and bottom
            if (i == 0 || i == gridH - 1) {
                if (grid[i][j] == Taken && grid[i][j + 1] == Good) {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }
            // Check left and right
            if (j == 0 || j == gridW - 1) {
                if (grid[i][j] == Taken && grid[i + 1][j] == Good) {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Find TakenX-TakenY-Good cuts
            int goodCount = 0;
            int xTaken = 0;
            int xShape = -1;
            int xTaken = 0;
            int yShape = -1;
            for (int k = 0; k < 2; k++) {
                for (int l = 0; l < 2; l++) {
                    switch(grid[i + k][j + l]) {
                        case Taken:
                            if (xShape == -1) {
                                // Set xShape to current shape
                                xTaken = 1;
                            } else {
                                // Set yShape to current shape
                                yTaken = 1;
                            }
                            break;
                        case Good:
                            goodCount++;
                            break;
                    }
                }
            }
            if (goodCount == 2 && xTaken == 1 && yTaken == 1) {
                // Cut at the thinnest point between xShape and yShape
            }
        }
    }

    // Add the cuts to gcode
}