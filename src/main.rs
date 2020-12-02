#![allow(dead_code)]
#![feature(async_closure)]
use chrono::{TimeZone, Utc};
use iced::{
    executor, Align, Application, Background, Button, Color, Column, Command, Container, Length,
    Row, Scrollable, Settings, Space, Text, TextInput,
};
use std::convert::TryInto;
type Id = i64;
mod api;
#[derive(Debug)]
enum Mode {
    HomePage,
    Comment(Id),
}
async fn test_a() -> Message {
    // println!("async!!!");
    std::thread::sleep_ms(2000);
    Message::Add("Aa".to_string())
}
#[derive(Debug)]
struct State {
    scroll: iced::scrollable::State,
    refresh: iced::button::State,
    back_button: iced::button::State,
    texts: Vec<String>, // text: iced::text_input::State,
                        // comment_button: Vec<(iced::button::State, iced::button::State)>,
                        // downloads: Vec<iced_futures::S
}
#[derive(Debug)]
struct App {
    state: State,
    // top_stories: Vec<Item>,
    max_stories: usize,
    mode: Mode,
}

struct ButtonStyle;
impl iced::button::StyleSheet for ButtonStyle {
    fn hovered(&self) -> iced::button::Style {
        let active = self.active();

        iced::button::Style {
            shadow_offset: active.shadow_offset + iced::Vector::new(1.0, 1.0),
            ..active
        }
    }

    fn pressed(&self) -> iced::button::Style {
        iced::button::Style {
            shadow_offset: iced::Vector::new(1f32, 1f32),
            ..self.active()
        }
    }

    fn disabled(&self) -> iced::button::Style {
        let active = self.active();

        iced::button::Style {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }

    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            shadow_offset: iced::Vector::new(0f32, 0f32),
            background: Some(Background::Color(Color::WHITE)),
            text_color: Color::from_rgba8(0x00, 0x80, 0xff, 0.8),
            ..Default::default()
        }
    }
}
impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        println!("Creating new App;");
        let app = (
            App {
                state: State {
                    back_button: iced::button::State::default(),
                    scroll: iced::scrollable::State::default(),
                    refresh: iced::button::State::default(),
                    texts: vec![],
                },
                // top_stories: top_stories(3),
                max_stories: 25,
                mode: Mode::HomePage,
            },
            Command::none(),
        );
        println!("created app");
        app
    }

    fn title(&self) -> String {
        // format!("Hacker news - {} stories.", self.top_stories.len())
        format!("temp title")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::LinkPressed(i) => {
                // let post = self.top_stories.iter().find(|it| it.id == i).unwrap();
                // let url = post.url.as_ref().unwrap();
                // webbrowser::open("bing.com").unwrap();
            }
            Message::Refresh => {
                println!("Refreshed.");
            }
            // Message::Refresh => self.top_stories = top_stories(self.max_stories),
            Message::GotoComments(id) => {
                self.mode = Mode::Comment(id);
            }
            Message::GotoHome => {
                self.mode = Mode::HomePage;
            }
            Message::Add(s) => self.state.texts.push(s),
        };
        Command::perform(test_a(), |v| {
            println!("Perfomrmedf");
            v
        })
        // Command::perform(iced_futures::futures::future::pending(), async |f: Message|  {
        //     println!("performing");
        //     return Message::Refresh
        // })
        // Command::perform::<Mes(iced_futures::futures::future::pending(), |_| {
        //     println!("performed {:?}", 2);
        //     Message::Refresh
        // })
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self.mode {
            Mode::HomePage => {
                let cheems = Text::new("Hacker News Stories:");
                let refresh = Button::new(&mut self.state.refresh, Text::new("Refresh"))
                    .on_press(Message::Refresh);
                let mut content = Scrollable::new(&mut self.state.scroll)
                    .width(Length::Fill)
                    .align_items(Align::Start)
                    .padding(24)
                    .spacing(10)
                    .push(cheems)
                    .push(refresh);

                for text in &self.state.texts {
                    content = content.push(Text::new(text))
                }

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            Mode::Comment(id) => {
                let mut content = Scrollable::new(&mut self.state.scroll)
                    .width(Length::Fill)
                    .align_items(Align::Start)
                    .padding(24)
                    .spacing(10);
                content = content.push(
                    Button::new(&mut self.state.back_button, Text::new("←"))
                        .style(ButtonStyle {})
                        .on_press(Message::GotoHome),
                );

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }

    // fn subscription(&self) -> iced::Subscription<Self::Message> {
    //     iced::Subscription::
    //     // iced::futures::future::pending()

    //     // iced::Subscription::none()
    // }
}
#[derive(Debug, Clone)]
pub enum Message {
    LinkPressed(Id),
    GotoComments(Id),
    GotoHome,
    Refresh,
    Add(String),
}
fn main() {
    App::run(Settings::default()).unwrap();
}
