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
                            .title("Pendulum")
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect();

    //Custom Code
    let pendulum = Pendulum::new(win.x(), win.y(), 100.0, PI/4.0, -1000.0);

    Model { _window, pendulum }
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