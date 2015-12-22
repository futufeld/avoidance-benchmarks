# Obstacle Avoidance Benchmarks

## Overview

This program benchmarks the obstacle avoidance [steering behaviour](http://www.red3d.com/cwr/steer/). It does so by randomly generating interactions between a rectangular feeler, which triggers collision avoidance, and a number of circular obstacles. Two interactions are considered:

- Case One: No obstacles intersect the feeler.
- Case Two: All obstacles intersect the feeler.

The benchmark consists of:

- Updating the matrices for transforming in and out of feeler's local space.
- Determining whether each obstacle intersects with the feeler.
- Identifying the nearest intersecting obstacle.
- Calculating the steering force required to avoid that obstacle.
