# Obstacle Avoidance Benchmarking

## Overview

This program benchmarks the the obstacle avoidance [steering behaviours](http://www.red3d.com/cwr/steer/). It does so by randomly-generating interactions between the feeler, which triggers collision avoidance, and a circular obstacle. The feeler is modelled as a rectangle. Three cases are considered:

- Case One: Obstacle and feeler do not intersect.
- Case Two: Obstacle intersects the feeler but its centre does not lie inside the feeler. The boundary of the obstacle intersects either of the longitudinal edges of the feeler at exactly two points.
- Case Three: Obstacle intersects the feeler and its centre lies inside the feeler. It may not intersect either longitudinal edge of the feeler.

Each case corresponds to a different branch of execution in the feeler-obstacle intersection function.
