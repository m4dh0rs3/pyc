# Polycentrics ⤵️⤴️
### Warning unstable!

#### Technical

Polycentric curves are approximated by line seqments of a graph. If the PCCS intersects itselfs, two nodes of the intersector's lines are connected. Every permutation of the graph is used to detect closed paths with colored breadth-first-search to classify areas contained in the PCCS. For aestetic reasons, arcs are generated with quadratic bezier curves.

### Board

The board is a field containing 11 by 11 evenly spaced points.
Such can be free or taken by a player.
### Tiles

The players alternate laying tiles with a specific radius and rotation.
Internally they are an edge of a graph and a set of points making up the curvature.

### Graph

The undirected graph of tiles and their intersections is used to find closed paths.

#### `RGL`: Rust Graph Library

- Detect all cycles in undirected graphs

Based on: [Optimal Listing of Cycles and st-Paths in Undirected Graphs](https://arxiv.org/pdf/1205.2766.pdf) (I don't know if my implementation is optimal)

## Tutorial

Two players alternate choosing unique tiles of radii 1 to 3 and 4 directions
to append to the end of the last tile, shaping a path on the board. If such
intersects itself, all points enclosed by its shape are taken by the player
who closed the path. He aims to accumulate the most points and not to steer
outside the board.

## Technical

## People

- Author Angelo Alessandro Mazzotti (game registered 01.2018)

This is a private clone of Polycentrics, a game by Angelo Mazotti. The original was implemented by `OrionLab` and published by [GAMMAGRAPHICS SRLS](https://www.gammagraphics.eu/)