#[macro_use]
extern crate glium;

#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
extern crate conrod_winit;
extern crate image;

pub mod curve;
mod settings;
mod support;

use conrod_glium::Renderer;
use glium::Surface;

fn main_draw(
    ref mut ui: conrod_core::UiCell,
    ids: &settings::Ids,
    curve: &mut curve::CubicBezierCurve,
) {
    use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
    use std::iter::once;

    const MARGIN: conrod_core::Scalar = 5.0;
    const KEYCONFIG_SIZE: conrod_core::FontSize = 12;

    // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    // By default, its size is the size of the window. We'll use this as a background for the
    // following widgets, as well as a scrollable container for the children widgets.
    widget::Canvas::new()
        .scroll_kids_vertically()
        .pad(MARGIN)
        .set(ids.canvas, ui);

    const KEYCONFIG: &'static str = "Key [I]: Initialize all control points\n\
     Key [L]: Make the line dotted\n\
     Key [C]: Draw control mesh\n\
     Key [R]: Draw full control circles\n\
     Key [1-4]: Draw biarc (2, 4, 8, 16)";
    widget::Text::new(KEYCONFIG)
        .font_size(KEYCONFIG_SIZE)
        .bottom_left_with_margin_on(ids.canvas, MARGIN)
        .line_spacing(5.0)
        .set(ids.keyconfig, ui);

    
}

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_title("Geometric Modeling HW1")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(WIDTH, HEIGHT));
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(settings::theme())
        .build();

    let ids = settings::Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut renderer = Renderer::new(&display).unwrap();

    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    // MAIN CURVE
    let mut curve = curve::CubicBezierCurve::new();

    support::run_loop(display, event_loop, move |request, display| {
        match request {
            support::Request::Event {
                event,
                should_update_ui,
                should_exit,
            } => {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = support::convert_event(&event, &display.gl_window().window()) {
                    ui.handle_event(event);
                    *should_update_ui = true;
                }

                match event {
                    glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::event::WindowEvent::CloseRequested
                        | glium::glutin::event::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::event::KeyboardInput {
                                    virtual_keycode:
                                        Some(glium::glutin::event::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *should_exit = true,
                        _ => {}
                    },
                    _ => {}
                }
            }
            support::Request::SetUi { needs_redraw } => {
                // Instantiate a GUI demonstrating every widget type provided by conrod.
                main_draw(ui.set_widgets(), &ids, &mut curve);

                *needs_redraw = ui.has_changed();
            }
            support::Request::Redraw => {
                // Render the `Ui` and then display it on the screen.
                let primitives = ui.draw();

                renderer.fill(display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    })
}
