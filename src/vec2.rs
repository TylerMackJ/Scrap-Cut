struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn move_towards(&mut self, &to: Vec2, step: f32) {
        let distance = ((to.x - self.x) ** 2 + (to.y - self.y) ** 2).sqrt();
        let t = step / distance;
        if t >= 1 {
            self = to;
        } else {
            self.x = ((1 - t) * self.x) + (t * to.x);
            self.y = ((1 - t) * self.y) + (t * to.y);
        }
    }

    fn curve_towards(&mut self, &to: Vec2, &center_point: Vec2, step: f32, clockwise: bool) {
        // Radians
        let radius = ((center_point.x - self.x) ** 2 + (center_point.y - self.y) ** 2).sqrt();
        // Consider angles positive
        let central_angle = todo!();
        let step_angle = step / radius;
        if central_angle >= step_angle {
            self = to;
        } else {
            let x_rotation = Vec2 {
                x: cos(step_angle),
                y: sin(step_angle),
            };

            let y_rotation = Vec2 {
                x: cos(step_angle + (PI / 2)),
                y: sin(step_angle + (PI / 2)),
            };

            if !clockwise {
                x_rotation.x = -x_rotation.x;
                x_rotation.y = -x_rotation.y;
                y_rotation.x = -y_rotation.x;
                y_rotation.y = -y_rotation.y;
            }

            let translated_point = Vec2 {
                x: self.x - center_point.x,
                y: self.y - center_point.y,
            };

            let rotated_point = Vec2 {
                x: translated_point.x * x_rotation.x + translated_point.y * y_rotation.x,
                y: translated_point.x * x_rotation.y + translated_point.y * y_rotation.y,
            };

            self.x = rotated_point.x + center_point.x;
            self.y = rotated_point.y + center_point.y;
        }

    }
}
