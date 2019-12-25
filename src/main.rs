mod element;
mod random_range;
mod int_color;
mod profiles;
use nannou::prelude::*;

use profiles::{ElementCollection, OrangeSun, BlueNight, Sparkle, Smoke};

enum AppState {
    OrangeSun,
    BlueNight,
    Sparkle,
    Smoke,
}

struct Model {
    triangles: Vec<ElementCollection>,
    app_state: AppState,
    switch: bool
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
}

fn model(_app: &App) -> Model {
    Model {
        triangles: profiles::OrangeSun::profile(),
        app_state: AppState::OrangeSun,
        switch: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let duration = app.duration.since_prev_update.as_secs_f32();
    for collection in model.triangles.iter_mut() {
        collection.update(duration);
    }
    let space_pressed = app.keys.down.contains(&Key::Space);
    if !model.switch && space_pressed {
        match model.app_state {
            AppState::OrangeSun => {
                model.triangles = BlueNight::profile();
                model.app_state = AppState::BlueNight;
            },
            AppState::BlueNight => {
                model.triangles = Sparkle::profile();
                model.app_state = AppState::Sparkle;
            },
            AppState::Sparkle => {
                model.triangles = Smoke::profile();
                model.app_state = AppState::Smoke;
            },
            AppState::Smoke => {
                model.triangles = OrangeSun::profile();
                model.app_state = AppState::OrangeSun;
            },
        }
        model.switch = true;
    } else if !space_pressed {
        model.switch = false;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for collection in &model.triangles {
        collection.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
