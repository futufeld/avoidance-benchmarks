# Avoidance Benchmarks

## Overview

This project includes multiple programs that benchmark two collision avoidance approaches used in autonomous steering: [steering behaviours](http://www.red3d.com/cwr/steer/gdc99/) and the goal-aligned xetrov system. Scenarios involving disc, wall and vehicle obstacles are considered.

## Project structure

A breakdown of this project:

- `goal_aligned_xetrov`: Contains subprojects that benchmark collision avoidance using the goal-aligned xetrov algorithm.
    - `common`: Functionality common to the goal-aligned xetrov benchmark programs.
- `steering_behaviours`: Contains subprojects that benchmark the steering behaviours intended to produce collision avoidance.
    - `common`: Functionality common to the steering behaviours benchmark programs.
- `linalg`: Linear algebra functionality common to all subprojects.
- `utilities`: Various benchmarking utilities.

The folders `disc_avoidance`, `wall_avoidance` and `vehicle_avoidance` in both the `goal_aligned_xetrov` and `steering_behaviours` contain programs that can be run as follows:
```
cargo run FILENAME
```
where `FILENAME` specifies the file in which to write benchmark results. The output format is JSON.

## Benchmarking strategy

Each program times the evaluation of an algorithm using randomly generated scenarios. Each scenario contains a steering vehicle and a number of obstacles. They represent the state of a steering simulation at a single iteration. Each benchmark program benchmarks the algorithm in scenarios, involving varying numbers of obstacles, in which all obstacles either present a collision risk or do not present a collision risk. Note that each algorithm consists of two phases:

1. Assessment of collision risk: whether the vehicle may collide with the obstacle if it does not modify its trajectory.
2. Evaluation of steering force: if any obstacles present a collision risk, what is the appropriate steering response?

The benchmarking strategy provides some insight into the relative computational cost of these aspects of the algorithms. Each algorithm is evaluated 10,000 times using independently-generated scenarios in an environment isolated from the compiler's optimiser.

## Running the benchmarks

The software uses compiler plugins and therefore depends on the nightly version of the Rust compiler.
