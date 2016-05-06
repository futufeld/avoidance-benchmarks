use super::common::types::Frame;
use super::linalg::vector2d::Vec2D;

// Weighting factor for obstacle avoidance steering force.
const BRAKING_WEIGHT: f64 = 2f64;

// Defines a disk.
pub struct Disk { pub centre: Vec2D
                , pub radius: f64 }

impl Disk {
    // Creates a disk from the given centre and radius.
    pub fn new(centre: Vec2D, radius: f64) -> Disk {
        Disk { centre: centre, radius: radius }
    }
}

// Result of interaction between feeler and disk.
pub enum FeelerResult {
    Case1,
    Case2(Interaction),
    Case3(Interaction)
}

// Pairs a disk with a distance. Used to capture interactions between vehicle
// feeler and disks.
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
    // disk.
    pub fn interaction(&self, disk: &Disk) -> Option<Interaction> {
        let local_centre = self.frame.to_local.transform(disk.centre);
        if local_centre.x > self.length { return None; }

        let expanded_radius = disk.radius + self.width;
        if local_centre.y.abs() > expanded_radius { return None; }

        let r2 = expanded_radius * expanded_radius;
        let y2 = local_centre.y * local_centre.y;
        let sqrt_part = (r2 - y2).sqrt();

        let mut x = local_centre.x - sqrt_part;
        if x < 0f64 { x = local_centre.x + sqrt_part; }
        Some(Interaction::new(x, local_centre, disk.radius))
    }

    // Returns a force intended to prevent collision between the vehicle and a
    // collection of disks.
    pub fn disk_avoidance(&self, disks: &Vec<Disk>) -> Option<Vec2D> {

        // Collect interactions between vehicle's feeler and disks.
        let mut nearest: Option<Interaction> = None;
        for disk in disks.iter() {

            // Check if interaction is closer than known nearest.
            let interaction = self.interaction(disk);
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
        let multiplier = 1f64 + (self.length - near.dist) / self.length;
        let force_x = (near.radius - near.centre.x) * BRAKING_WEIGHT;
        let force_y = (near.radius - near.centre.y) * multiplier;
        Some(self.frame.to_world.transform(Vec2D::new(force_x, force_y)))
    }
}
