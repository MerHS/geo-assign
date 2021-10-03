use iced::{
    button, executor, slider, window, Align, Application, Button, Canvas, Checkbox, Clipboard,
    Column, Command, Element, Length, Row, Settings, Slider, Text,
};

pub mod bezier;
pub mod biarc;
pub mod tree;
pub mod util;

pub fn main() -> iced::Result {
    Bezier::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (720, 480),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Bezier {
    canvas: bezier::State,
    init_state: button::State,
    dot_state: button::State,
    mesh_state: button::State,
    arc_slider_state: slider::State,
    aabb_slider_state: slider::State,
    checkbox_state: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Initialize,
    ToggleDotted,
    ToggleMesh,
    ToggleAABB(bool),
    SetBiarc(u8),
    SetAABBDepth(u8),
}

impl Application for Bezier {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Bezier {
                canvas: bezier::State::new(),
                init_state: Default::default(),
                dot_state: Default::default(),
                mesh_state: Default::default(),
                arc_slider_state: Default::default(),
                aabb_slider_state: Default::default(),
                checkbox_state: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("GeoModel Assignment 2")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Initialize => {
                self.canvas = bezier::State::new();
            }
            Message::ToggleDotted => {
                self.canvas.toggle_dotted();
            }
            Message::ToggleMesh => {
                self.canvas.toggle_meshed();
            }
            Message::SetBiarc(split_biarc) => {
                self.canvas.set_num_biarc(split_biarc as usize);
            }
            Message::SetAABBDepth(aabb_depth) => {
                self.canvas.set_aabb_depth(aabb_depth as usize);
            }
            Message::ToggleAABB(checked) => {
                self.checkbox_state = checked;
                self.canvas.set_bezier_aabb(checked);
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let num_split = self.canvas.num_split;
        let aabb_depth = self.canvas.aabb_depth;
        let num_string = num_split.to_string();
        let aabb_string = aabb_depth.to_string();

        Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Align::Start)
            .push(
                Canvas::new(&mut self.canvas)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                Row::new()
                    .padding(5)
                    .spacing(10)
                    .align_items(Align::Start)
                    .push(
                        Column::new()
                            .spacing(10)
                            .align_items(Align::Start)
                            .push(
                                Row::new()
                                    .spacing(10)
                                    .align_items(Align::Start)
                                    .push(
                                        Button::new(&mut self.init_state, Text::new("Initialize"))
                                            .padding(8)
                                            .on_press(Message::Initialize),
                                    )
                                    .push(
                                        Button::new(&mut self.mesh_state, Text::new("Draw Mesh"))
                                            .padding(8)
                                            .on_press(Message::ToggleMesh),
                                    )
                                    .push(
                                        Button::new(&mut self.dot_state, Text::new("Dashed"))
                                            .padding(8)
                                            .on_press(Message::ToggleDotted),
                                    ),
                            )
                            .push(Checkbox::new(
                                self.checkbox_state,
                                "Use Bezier AABB",
                                Message::ToggleAABB,
                            )),
                    )
                    .push(
                        Column::new()
                            .spacing(3)
                            .align_items(Align::Start)
                            .push(
                                Row::new()
                                    .padding(5)
                                    .spacing(5)
                                    .align_items(Align::Center)
                                    .push(Text::new("Arc Split # ").width(Length::Units(130)))
                                    .push(Text::new(num_string).width(Length::Units(10)))
                                    .push(Slider::new(
                                        &mut self.arc_slider_state,
                                        1..=5,
                                        num_split as u8,
                                        Message::SetBiarc,
                                    )),
                            )
                            .push(
                                Row::new()
                                    .padding(5)
                                    .spacing(5)
                                    .align_items(Align::Center)
                                    .push(Text::new("AABB Depth # ").width(Length::Units(130)))
                                    .push(Text::new(aabb_string).width(Length::Units(10)))
                                    .push(Slider::new(
                                        &mut self.aabb_slider_state,
                                        0..=((num_split + 2) as u8),
                                        aabb_depth as u8,
                                        Message::SetAABBDepth,
                                    )),
                            ),
                    ),
            )
            .into()
    }
}
