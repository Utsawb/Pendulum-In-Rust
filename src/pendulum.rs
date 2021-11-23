use nannou::prelude::*;

pub struct Pendulum {
    pub orgin_x: f32,
    pub orgin_y: f32,
    pub string_length: f32,
    pub angle: f32,
    pub gravity: f32,

    pub bob_x: f32,
    pub bob_y: f32,
    pub angle_vel: f32,
    pub angle_acc: f32,
}

impl Pendulum {
    pub fn new(orgin_x: f32, orgin_y: f32, string_length: f32, angle: f32, gravity: f32) -> Self {
        let bob_x: f32 = string_length * angle.sin() + orgin_x;
        let bob_y: f32 = string_length * -1.0 * angle.cos() + orgin_y;
        let angle_vel: f32 = 0.0;
        let angle_acc: f32 = gravity * angle.sin() / string_length;

        Pendulum {orgin_x, orgin_y, string_length, angle, gravity, bob_x, bob_y, angle_vel, angle_acc}
    }
    pub fn logic(&mut self, _app: &App, _update: Update) {
        if _app.mouse.buttons.right().is_down() {
            self.orgin_x = _app.mouse.x;
            self.orgin_y = _app.mouse.y;
        }

        if _app.mouse.buttons.left().is_down() {
            self.bob_x = _app.mouse.x;
            self.bob_y = _app.mouse.y;
            self.angle_vel = 0.0;

            let a = self.bob_y - self.orgin_y;
            let b = self.bob_x - self.orgin_x;

            self.angle =  -1.0 * (b.atan2(a) - PI);

            //self.string_length = a / self.angle.cos();
            self.string_length = (a.powi(2) + b.powi(2)).sqrt();
        }
        else {
            let delta_time = _update.since_last.as_secs_f32();
            self.angle_acc = self.gravity * self.angle.sin() / self.string_length;
            self.angle_vel = self.angle_vel + self.angle_acc * delta_time;
            self.angle = self.angle + self.angle_vel * delta_time;
    
            self.bob_x = self.string_length * self.angle.sin() + self.orgin_x;
            self.bob_y = self.string_length * -1.0 * self.angle.cos() + self.orgin_y;
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let orgin = pt2(self.orgin_x, self.orgin_y);
        let bob = pt2(self.bob_x, self.bob_y);
        draw.ellipse().xy(orgin).radius(10.0).color(BLACK);
        draw.line().start(orgin).end(bob).weight(10.0).color(BLACK);
        draw.ellipse().xy(bob).radius(25.0).color(BLACK);
    }
}