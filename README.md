# Polycentrics

**TODO: Write description**

### Tutorial

Two players alternate choosing unique tiles of radii 1 to 3 and 4 directions to append to the end of the last tile, shaping a path on the board. If such intersects itself, all points enclosed by its shape are taken by the player who closed the path. He aims to accumulate the most points and not to steer outside the board.

### Technical

#### Steps

1. Offer a set of tiles to the player to choose from.
2. Append the tile a the end of the last tile.
    1. Rotate the tile around the arrow.
    2. The midpoint of the circular arc is the position of the arrow plus a vector of the radius of the arc in the direction of the start angle of the arc.
    3. Move the arrow to the end of the tile.
3. Find all intersections of the last tile with the path.
    1. Find the intersection point of the two circles and check if they are contained by the arc.
4. For every intersection, find the polygon enclosed by the path.
    1. The set of curves between the intersection indecies are the polygon.
5. For every free point on the board, check if the winding number of any polygon is not zero. Then the point can be marked by the player.

### Contributors

This is a private clone of [Polycentrics](https://www.polycentrics.com/), a game by `Angelo Alessandro Mazzotti` (registered `Jan 2018`). It was initially implemented by `OrionLab` and published by [GAMMAGRAPHICS SRLS](https://www.gammagraphics.eu/).