use crate::*;
use std::f32::consts::*;

// Adopted from: https://pwy.io/posts/learning-to-fly-pt4/#prerequisites | Section: Eye Of The Birdie
/// How far our eye can see:
///
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
///
/// If @ marks our agent and % marks input, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = agent sees no inputs (at least in this case)
/// - 0.5 = 50% of the map = agent sees one of the inputs
/// - 1.0 = 100% of the map = agent sees both inputs
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// @> marks our agent (rotated to the right) and . marks the area
/// our agent sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
///
/// - PI = 180° =
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
///
/// - 2 * PI = 360° =
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our agents will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

// ----------------------- Definitions ---------------------------
#[derive(Debug)]
pub struct Eye {
    fov_angle: f32,
    fov_range: f32,
    cells: usize
}
// ---------------------------------------------------------------


// --------------------- Eye Implementation  ---------------------
impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        inputs: &[Input],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for input in inputs {
            let input_vec = input.position - position;

            let distance_to_input = input_vec.norm();
            if distance_to_input >= self.fov_range {
                continue;
            }

            // Angle of the input
            let mut angle = na::Rotation2::rotation_between(
                &na::Vector2::y(),
                &input_vec,
            ).angle();
            
            // Include agents rotation
            angle -= rotation.angle();

            // Ensure angle is between +- 3.14 radians (0 - 360 degs)
            angle = na::wrap(angle, -PI, PI);
            
            // If current input angle is outside agents fov, go to next input
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Makes angle relative to agents FOV - that is:
            // transforms it from <-FOV_ANGLE/2,+FOV_ANGLE/2> to <0,FOV_ANGLE>.
            angle += self.fov_angle / 2.0;

            // Create a cell within 0 - 1 range
            let cell = angle / self.fov_angle;

            // Convert so it's between 0 - cells range
            let cell = cell * (self.cells as f32);

            // Make it usize
            let cell = (cell as usize).min(self.cells - 1);

            // Energy is inversely proportional to the distance between our
            // agent and the currently checked food
            let energy = (self.fov_range - distance_to_input) / self.fov_range;

            // Update all the cells
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
// ---------------------------------------------------------------