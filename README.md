# Polycentrics – Rust Backend

This Rust binary handles the board–logic of PYC.

### Board

The board is an 2–dimensional array of points.
They can be free or taken up by one of the players

### Tiles

The players alternate laying tiles with a specific radius and rotation.
Internally they are an edge of a graph and a set of points making up the curvature.

### Graph

The undirected graph of tiles and their intersections is used to find closed paths.

A closed path is a cycle, which can be reduced to $st$-Paths, as described in
[](https://arxiv.org/pdf/1205.2766.pdf)