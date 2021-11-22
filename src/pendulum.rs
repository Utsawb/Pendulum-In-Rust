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

    pub fn key_logic(&mut self, _key: Key) {
    }

    pub fn logic(&mut self, _app: &App, _update: Update) {
        let delta_time = _update.since_last.as_secs_f32();
        self.angle = self.angle + 100.0 * delta_time;
    }

    pub fn draw(&self, draw: &Draw) {
        let orgin = pt2(self.orgin_x, self.orgin_y);
        let bob = pt2(self.bob_x, self.bob_y);
        draw.line().start(orgin).end(bob).weight(10.0).color(BLACK);
        draw.ellipse().xy(bob).radius(25.0).color(BLACK);
    }
}