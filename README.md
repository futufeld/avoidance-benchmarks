# Avoidance Benchmarks

## Overview

This project includes multiple programs that benchmark various collision avoidance algorithms used in autonomous steering. Four algorithms are considered:

- Goal-aligned xetrov avoidance (disc and line segment avoidance)
- [Steering behaviours](http://www.red3d.com/cwr/steer/)
    - Obstacle avoidance (circular obstacles)
    - Wall avoidance (line segment obstacles)
    - Vehicle avoidance (moving obstacles)

## Structure

A breakdown of this project:

- `goal_aligned_xetrov`: Contains subprojects that benchmark collision avoidance using the goal-aligned xetrov algorithm.
    - `obstacle_avoidance`*: Avoidance of disc obstacles.
- `steering_behaviours`: Contains subprojects that benchmark collision avoidance _steering behaviours_.
    - `obstacle_avoidance`*: Avoidance of disc obstacles.
    - `wall_avoidance`*: Avoidance of wall segments.
    - `common`: Functionality common to the steering behaviours benchmark programs.
- `linalg`: Linear algebra functionality common to all subprojects.
- `utilities`: Various benchmarking utilities.

Folders marked with an asterisk contain benchmark programs which can be run as follows:

```
cargo run FILENAME
```

where `FILENAME` specifies the file to write benchmark results.

## Strategy

Broadly, each algorithm is tested with randomly generated inputs contrived to involve a specific number of collisions. Due to the speed of the algorithms, each benchmark consists of thousands of executions, called a _run_. Multiple runs are performed to produce a _batch_. The results of a batch are written, in JSON, to the file specified on the command line of the benchmark program. Refer to READMEs in subprojects for more details.
