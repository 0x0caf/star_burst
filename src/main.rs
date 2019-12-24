use nannou::prelude::*;

struct Model {
    rotate: f32
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
}

fn model(_app: &App) -> Model {
    Model {
        rotate: 0.0
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotate = model.rotate + 1.0;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(srgb(0.25, 0.25, 0.25));
    let radians = deg_to_rad(model.rotate);
    let offset_radians = deg_to_rad(model.rotate + 45.0);
    draw.tri()
        .rotate(radians)
        .x_y(0.0, 0.0)
        .height(100.0)
        .width(100.0)
        .color(srgb(1.00,0.49,0.00));

    draw.tri()
        .rotate(-radians)
        .x_y(0.0, 0.0)
        .height(100.0)
        .width(100.0)
        .color(srgb(1.00,0.49,0.00));

    draw.tri()
        .rotate(-offset_radians)
        .x_y(0.0, 0.0)
        .height(100.0)
        .width(100.0)
        .color(srgb(1.00,0.49,0.00));

    draw.tri()
        .rotate(offset_radians)
        .x_y(0.0, 0.0)
        .height(100.0)
        .width(100.0)
        .color(srgb(1.00,0.49,0.00));

    draw.to_frame(app, &frame).unwrap();
}
