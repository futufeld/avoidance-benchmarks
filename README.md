# Avoidance Benchmarks

## Overview

This project includes multiple programs that benchmark two collision avoidance approaches used in autonomous steering: [steering behaviours](http://www.red3d.com/cwr/steer/gdc99/) and xetrov fields. Scenarios involving disk, wall and vehicle obstacles are considered.

## Project structure

A breakdown of this project:

- `xetrov_field_function`: Contains projects that benchmark the xetrov field function.
    - `common`: Functionality common to the xetrov field benchmark programs.
- `avoidance_behaviours`: Contains subprojects that benchmark the algorithms in the steering behaviours framework intended to produce collision avoidance behaviour: *avoid block*, *avoid wall* and *avoid vehicle*.
    - `common`: Functionality common to the avoidance behaviour benchmark programs.
- `linalg`: Linear algebra functionality common to all projects.
- `utilities`: Various benchmarking utilities.

The folders `disk_avoidance`, `wall_avoidance` and `vehicle_avoidance` in both the `xetrov_field_function` and `avoidance_behaviours` folders contain programs that can be run as follows:
```
cargo run FILENAME
```
where `FILENAME` specifies the file in which to write benchmark results. The output format is JSON.

## Benchmarking strategy

Each program times the evaluation of an algorithm using randomly generated scenarios. Each scenario contains a navigating vehicle and a number of obstacles. They represent the state of a steering simulation in a single iteration. Each scenario represents an environment containing a predefined number of obstacles. Two types of scenarios are considered:

* **Type A**: Each obstacle poses a collision risk; that is to say, if the avoidance algorithm were to be evaluated for each obstacle individually, each would produce a non-null result. In **Type A** scenarios, the avoidance algorithm must assess the collision risk posed by each obstacle and determine an appropriate result.

* **Type B**: No obstacle poses a collision risk. That is to say, if the avoidance algorithm were to be evaluated for each obstacle individually, each would produce a null result. In **Type B** scenarios, the avoidance algorithm must assess the collision risk posed by each obstacle but can terminate once it has determine that no action is required.

1. Assessment of collision risk: whether the vehicle may collide with the obstacle if it does not modify its trajectory.
2. Evaluation of steering force: if any obstacles present a collision risk, what is the appropriate steering response?

This benchmarking strategy provides some insight into the relative computational cost of assessing obstacle risk and evaluating a result using each algorithm.

Each algorithm is evaluated 1,000,000 times using independently-generated scenarios. The algorithms' implementations are isolated from the compiler's optimiser.

## Running the benchmarks

The software uses compiler plugins and therefore depends on the nightly version of the Rust compiler.
