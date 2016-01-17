use super::linalg::vector2d::Vec2D;

// For scenarios that are testable without needing to access internals.
pub trait HasScenario {
    fn interactions(&self) -> u32;
    fn avoidance(&mut self) -> Option<Vec2D>;
}

// Contains details about obstacle interactions.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Obstacles { pub total:         u32
                     , pub insignificant: u32
                     , pub significant:   u32 }

impl Obstacles {
    // Returns an Obstacles populated by the given values.
    pub fn new(insignificant: u32, significant: u32) -> Obstacles {
        Obstacles { total:         insignificant + significant
                  , insignificant: insignificant
                  , significant:   significant }
    }

    // Returns a tuple containing the number of insignificant and significant
    // obstacles.
    pub fn details(&self) -> (u32, u32) {
        (self.insignificant, self.significant)
    }
}
