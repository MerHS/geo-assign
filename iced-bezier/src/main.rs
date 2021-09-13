use iced::{
    executor, keyboard, window, Align, Application, Canvas, Clipboard, Column, Command, Element,
    Length, Settings, Subscription, Text,
};
use iced_native::{subscription, Event};

pub mod biarc;
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
    canvas: biarc::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Initialize,
    ToggleDotted,
    ToggleMesh,
    SetBiarc(usize),
}

impl Application for Bezier {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Bezier {
                canvas: biarc::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("GeoModel Assignment 1 - 2020-29856")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Initialize => {
                self.canvas = biarc::State::new();
            }
            Message::ToggleDotted => {
                self.canvas.toggle_dotted();
            }
            Message::ToggleMesh => {
                self.canvas.toggle_meshed();
            }
            Message::SetBiarc(num_biarc) => {
                self.canvas.set_num_biarc(num_biarc);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
                return match key_code {
                    keyboard::KeyCode::I => Some(Message::Initialize),
                    keyboard::KeyCode::L => Some(Message::ToggleDotted),
                    keyboard::KeyCode::C => Some(Message::ToggleMesh),
                    keyboard::KeyCode::Key1 => Some(Message::SetBiarc(1)),
                    keyboard::KeyCode::Key2 => Some(Message::SetBiarc(2)),
                    keyboard::KeyCode::Key3 => Some(Message::SetBiarc(3)),
                    keyboard::KeyCode::Key4 => Some(Message::SetBiarc(4)),
                    keyboard::KeyCode::Key5 => Some(Message::SetBiarc(5)),
                    _ => None,
                };
            }
            None
        })
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(10)
            .align_items(Align::Start)
            .push(
                Canvas::new(&mut self.canvas)
                  .width(Length::Fill)
                  .height(Length::Fill)
            )
            .push(
                Text::new("[I]: Initialize control points / [L]: Make line dotted / [C]: Draw control mesh / [1-5]: Draw biarcs (2^n splits)")
                    .width(Length::Shrink)
                    .size(20),
            )
            .into()
    }
}
