use nannou::prelude::*;

mod pendulum;
use crate::pendulum::Pendulum;

fn main() {
    nannou::app(model)
        .size(1280, 720)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    pendulum: Pendulum,
}

fn model(app: &App) -> Model {
    let _window = app.
                            new_window()
                            .title("Bouncing Ball")
                            .key_pressed(key_pressed)
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect().pad(30.0);

    //Custom Code
    let pendulum = Pendulum::new(win.x(), win.y(), 150.0, 0.0, -100.0);

    Model { _window, pendulum }
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
    //Custom Code
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Custom Code
    _model.pendulum.logic(_app, _update);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    //Custom Code
    _model.pendulum.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}