use super::linalg::vector2d::*;

use std::f64::consts::PI;

use super::rand::thread_rng;
use super::rand::distributions::{IndependentSample, Range};

// Returns a random f64 between 0 and 1 using the thread's random number
// generator.
fn random_unity() -> f64 {
    Range::new(0f64, 1f64).ind_sample(&mut thread_rng())
}

// Data describing approach between two vehicles.
#[derive(Copy, Clone)]
pub struct Interaction {
    vehicle_position: Vec2D,
    relative_position: Vec2D,
    relative_velocity: Vec2D,
    time_to_collision: f64,
    min_separation: f64,
    distance: f64
}

// Defines vehicle capable of avoiding other vehicles.
pub struct Vehicle {
    pub position: Vec2D,
    pub velocity: Vec2D,
    radius: f64,
    max_acceleration: f64
}

impl Vehicle {
    // Creates a vehicle from the given values.
    pub fn new(pos: Vec2D, vel: Vec2D, radius: f64, max_acc: f64 ) -> Vehicle {
        Vehicle { position: pos
                , velocity: vel
                , radius: radius
                , max_acceleration: max_acc }
    }

    // Returns the interaction between this vehicle and the given vehicle.
    pub fn interaction(&self, vehicle: &Vehicle) -> Option<Interaction> {
        // Determine relative position.
        let relative_position = vehicle.position.sub(self.position);
        let distance = relative_position.mag();

        // Determine time to collision, skipping this vehicle if
        // velocities are equal.
        let relative_velocity = vehicle.velocity.sub(self.velocity);
        let relative_speed = relative_velocity.mag();

        let numerator = relative_position.dot(relative_velocity);
        let denominator = relative_speed * relative_speed;
        if denominator < EPSILON { return None; }
        let time_to_collision = -numerator / denominator;

        // Check if collision will occur.
        if time_to_collision < EPSILON { return None; }
        let min_separation = distance - relative_speed * time_to_collision;
        if min_separation > 2f64 * self.radius { return None; }

        // Return result.
        let interaction = Interaction { vehicle_position: vehicle.position
                                      , relative_position: relative_position
                                      , relative_velocity: relative_velocity
                                      , time_to_collision: time_to_collision
                                      , min_separation: min_separation
                                      , distance: distance };
        Some(interaction)
    }

    // Returns a force intended to prevent collision between the vehicle and a
    // collection of other vehicles.
    pub fn vehicle_avoidance(&self, vehicles: &Vec<Vehicle>) -> Option<Vec2D> {

        // Collect interactions between this and other vehicles.
        let mut soonest: Option<Interaction> = None;
        for vehicle in vehicles.iter() {

            // Check if interaction is sooner than known soonest.
            let interaction = self.interaction(vehicle);
            if let Some(int) = interaction {
                if let Some(soon) = soonest {
                    if int.time_to_collision < soon.time_to_collision {
                        soonest = interaction
                    }
                } else {
                    soonest = interaction
                }
            }
        }

        // Determine collision avoidance from soonest interaction.
        if soonest.is_none() { return None; }
        let interaction = soonest.unwrap();

        let colliding = interaction.distance < 2f64 * self.radius;
        let exact = interaction.min_separation <= EPSILON;
        let relative_position = if colliding || exact {
            interaction.vehicle_position.sub(self.position)
        } else {
            let rel_pos = interaction.relative_position;
            let rel_vel = interaction.relative_velocity;
            rel_pos.add(rel_vel.mul(interaction.time_to_collision))
        };

        // Determine avoidance force. If the two vehicles share the same
        // position a random vector is returned. (In a full implementation
        // another mechanism would be required to resolve this case.)
        let min_separation = relative_position.mag();
        if min_separation < EPSILON {
            let angle = 2f64 * PI * random_unity();
            return Some(Vec2D::polar(angle, self.max_acceleration));
        }
        let normalised = relative_position.mul(1f64 / min_separation);
        Some(normalised.mul(self.max_acceleration))
    }
}
