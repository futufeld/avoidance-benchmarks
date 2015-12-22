# Wall Avoidance Benchmarks

## Overview

This program benchmarks the wall avoidance [steering behaviour](http://www.red3d.com/cwr/steer/). It does so by randomly generating interactions between feelers (line segments) and walls (line segments). Three feeler configurations are considered:

- Spear: A single feeler projects forward from the vehicle.
- Fork: Two feelers fan out at 45 degree angles from the sides of the vehicle.
- Trident: Combination of Spear and Fork.

Two interactions are considered:

- Case One: No obstacles intersect the feeler.
- Case Two: Exactly one obstacle intersects each feelers.

Note that each scenario involves _k_ x _n_ possible intersections between feelers (_k_) and walls (_n_) but that the scenarios are contrived so that a wall intersects, at most, one wall. As a result, Case Two simulations involve _k_ x _n_ intersection tests of which _k_ x (_n_ - 1) are negative.

The benchmark consists of:

- For each feeler:
    - Translating the feeler into world space (using the position and orientation of the vehicle).
    - Testing whether the feeler has intersected with any wall.
- Identifying the feeler with the deepest wall penetration.
- Calculating the steering force required to avoid the corresponding wall.
