use nannou::color::RgbHue;
use nannou::glam::Vec2Swizzles;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

#[derive(Copy, Clone, Default, Debug)]
struct Model {
    mouse_x: f32,
    mouse_y: f32,
}

// The application state: The "Model in MVC"
fn model(_app: &App) -> Model {
    Model::default()
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(window_event)
        } => {
            match window_event {
                WindowEvent::MouseMoved(point) => {
                        model.mouse_x = point.x;
                        model.mouse_y = point.y;
                    }
                _ => (),
            }
        },
        // Event::DeviceEvent(_, _) => {},
        Event::Update(x) => update(app, model, x),
        // Event::Suspended => {},
        // Event::Resumed => {},
        _ => (),
    }
}


fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // frame.clear(PURPLE);

    let draw = app.draw();

    draw.background().color(Hsv::new(RgbHue::from_degrees(model.mouse_y), 1.0, 1.0));

    draw.rect().color(Hsv::new(RgbHue::from_degrees(360.0 - model.mouse_y), 1.0, 1.0)).wh(vec2(model.mouse_x, model.mouse_x));

    draw.to_frame(app, &frame).unwrap();
    println!("{model:?}");
}
