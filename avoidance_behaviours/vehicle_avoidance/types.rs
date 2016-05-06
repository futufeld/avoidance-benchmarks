use super::linalg::vector2d::{EPSILON, Vec2D};

// Data describing approach between two vehicles.
#[derive(Copy, Clone)]
pub struct Interaction { vehicle_position:  Vec2D
                       , relative_position: Vec2D
                       , relative_velocity: Vec2D
                       , time_to_collision: f64
                       , min_separation:    f64
                       , distance:          f64 }

// Defines a vehicle capable of avoiding other vehicles.
pub struct Vehicle { position:         Vec2D
                   , velocity:         Vec2D
                   , radius:           f64
                   , max_acceleration: f64 }

impl Vehicle {
    // Creates a vehicle from the given values.
    pub fn new(pos: Vec2D, vel: Vec2D, radius: f64, max_acc: f64 ) -> Vehicle {
        Vehicle { position:         pos
                , velocity:         vel
                , radius:           radius
                , max_acceleration: max_acc }
    }

    // Returns the interaction between this vehicle and the given vehicle.
    pub fn interaction(&self, vehicle: &Vehicle) -> Option<Interaction> {
        // Determine relative position.
        let relative_position = self.position.sub(vehicle.position);
        let distance = relative_position.mag();

        // Determine time to collision, skipping this vehicle if
        // velocities are equal.
        let relative_velocity = vehicle.velocity.sub(self.velocity);
        let relative_speed = relative_velocity.mag();

        let numerator = relative_position.dot(relative_velocity);
        let denominator = relative_speed * relative_speed;
        if denominator < EPSILON { return None; }
        let time_to_collision = numerator / denominator;

        // Check if collision will occur.
        if time_to_collision < EPSILON { return None; }
        let min_separation = distance - relative_speed * time_to_collision;
        if min_separation > 2f64 * self.radius { return None; }

        // Return result.
        let interaction = Interaction { vehicle_position:  vehicle.position
                                      , relative_position: relative_position
                                      , relative_velocity: relative_velocity
                                      , time_to_collision: time_to_collision
                                      , min_separation:    min_separation
                                      , distance:          distance };
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

        // Determine avoidance force.
        let factor = self.max_acceleration / interaction.min_separation;
        Some(relative_position.mul(factor))
    }
}
