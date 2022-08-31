use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vec2 {
    pub fn distance(v1: &Vec2, v2: &Vec2) -> f32 {
        ((v2.x - v1.x).powf(2.0) as f32 + (v2.y - v1.y).powf(2.0) as f32).sqrt()
    }

    pub fn move_towards(&mut self, to: Vec2, step: f32) {
        // Find the distance between the points
        let distance = Self::distance(&self, &to);

        // Figure out the ratio to step by
        let t = step / distance;

        // If we will step over the end then just go to end
        if t >= 1.0 {
            *self = to;
        } else {
            // Translate by the ratio
            self.x = ((1.0 - t) * self.x) + (t * to.x);
            self.y = ((1.0 - t) * self.y) + (t * to.y);
        }
    }

    pub fn curve_towards(&mut self, to: Vec2, center_point: Vec2, step: f32, clockwise: bool) {
        // Consider angles positive and use radians

        // Get the radius of the curve
        let radius = Self::distance(&self, &center_point);
        // Find what degree results in an arclength = step
        let step_angle = step / radius;

        // Get the angle of center_point->self to center_point->to
        let mut central_angle: f32 = (((self.x - center_point.x) * (to.x - center_point.x) + (self.y) * (center_point.y))/(radius * radius)).acos();
        // Account for clockwise
        central_angle = if clockwise { PI - central_angle } else { central_angle };

        // If we will step passed the end just move to end
        if central_angle >= step_angle {
            *self = to;
        } else {
            // Find the rotation we make in the x
            let mut x_rotation = Vec2 {
                x: step_angle.cos(),
                y: step_angle.sin(),
            };

            // Find the rotation we make in the y
            let mut y_rotation = Vec2 {
                x: (step_angle + (PI / 2.0)).cos(),
                y: (step_angle + (PI / 2.0)).sin(),
            };

            // Account for clockwise
            if clockwise {
                x_rotation.x = -x_rotation.x;
                x_rotation.y = -x_rotation.y;
                y_rotation.x = -y_rotation.x;
                y_rotation.y = -y_rotation.y;
            }

            // Translate point to origin
            let translated_point = Vec2 {
                x: self.x - center_point.x,
                y: self.y - center_point.y,
            };

            // Make rotation
            let rotated_point = Vec2 {
                x: translated_point.x * x_rotation.x + translated_point.y * y_rotation.x,
                y: translated_point.x * x_rotation.y + translated_point.y * y_rotation.y,
            };

            // Translate back to relative position
            self.x = rotated_point.x + center_point.x;
            self.y = rotated_point.y + center_point.y;
        }
    }
}
