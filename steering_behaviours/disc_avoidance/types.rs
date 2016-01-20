use super::common::types::Frame;
use super::linalg::vector2d::Vec2D;

// Weighting factor for obstacle avoidance steering force.
const BRAKING_WEIGHT: f64 = 2f64;

// Defines a disc.
pub struct Disc { pub centre: Vec2D
                , pub radius: f64 }

impl Disc {
    // Creates a disc from the given centre and radius.
    pub fn new(centre: Vec2D, radius: f64) -> Disc {
        Disc { centre: centre, radius: radius }
    }
}

// Result of interaction between feeler and disc.
pub enum FeelerResult {
    Case1,
    Case2(Interaction),
    Case3(Interaction)
}

// Pairs a disc with a distance. Used to capture interactions between vehicle
// feeler and discs.
#[derive(Copy, Clone)]
pub struct Interaction { dist:   f64
                       , centre: Vec2D
                       , radius: f64 }

impl Interaction {
    // Creates an interaction containing the provided values.
    fn new(distance: f64, centre: Vec2D, radius: f64) -> Interaction {
        Interaction { dist: distance, centre: centre, radius: radius }
    }
}

// Defines a vehicle with a single feeler volume.
pub struct Vehicle { pub frame:  Frame
                   ,     length: f64
                   ,     width:  f64 }

impl Vehicle {
    // Creates a new vehicle using the given values.
    pub fn new(frame: Frame, length: f64, width: f64) -> Vehicle {
        Vehicle { frame: frame, length: length, width: width }
    }

    // Updates the matrices of the underlying frame.
    pub fn update(&mut self) {
        self.frame.update_matrices();
    }

    // Returns the interaction between the vehicle's feeler and the given
    // disc.
    pub fn interaction(&self, disc: &Disc) -> Option<Interaction> {
        let local_centre = self.frame.to_local.transform(disc.centre);
        if local_centre.x > self.length { return None; }
        if local_centre.y.abs() > disc.radius + self.width { return None; }

        let r2 = (disc.radius + self.width) * (disc.radius + self.width);
        let y2 = local_centre.y * local_centre.y;
        let mut x = -self.length * (local_centre.x + (r2 - y2).sqrt());
        if x < 0f64 { x = 0f64; }
        Some(Interaction::new(x, local_centre, disc.radius))
    }

    // Returns a force intended to prevent collision between the vehicle and a
    // collection of discs.
    pub fn disc_avoidance(&self, discs: &Vec<Disc>) -> Option<Vec2D> {

        // Collect interactions between vehicle's feeler and discs.
        let mut nearest: Option<Interaction> = None;
        for disc in discs.iter() {

            // Check if interaction is closer than known nearest.
            let interaction = self.interaction(disc);
            if let Some(int) = interaction {
                if let Some(near) = nearest {
                    if int.dist < near.dist { nearest = interaction }
                } else {
                    nearest = interaction
                }
            }
        }
        if nearest.is_none() { return None; }
        let near: Interaction = nearest.unwrap();

        // Determine steering force.
        let multiplier = 1f64 + (self.length - near.centre.x) / self.length;
        let force_x = (near.radius - near.centre.x) * BRAKING_WEIGHT;
        let force_y = (near.radius - near.centre.y) * multiplier;
        Some(self.frame.to_world.transform(Vec2D::new(force_x, force_y)))
    }
}
