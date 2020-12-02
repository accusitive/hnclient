#![allow(dead_code)]
use iced::{
    executor, Align, Application, Background, Button, Color, Column, Command, Container, Element,
    Length, Point, Rectangle, Row, Sandbox, Scrollable, Settings, Size, Space, Text,
};
use serde::{Deserialize, Serialize};
type Id = u32;
const BASE: &str = "https://hacker-news.firebaseio.com/v0";
#[derive(Debug, Default)]
struct State {
    scroll: iced::scrollable::State,
    comment_button: Vec<iced::button::State>,
}
#[derive(Debug, Default)]
struct App {
    state: State,
    top_stories: Vec<Item>,
    max_stories: usize,
}
impl Item {
    fn get(id: Id) -> Self {
        let test =
            reqwest::blocking::get(&format!("{}/item/{}.json?print=pretty", BASE, id)).unwrap();
        let text = test.text().unwrap();
        serde_json::from_str(&text).unwrap()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
enum ItemTy {
    #[serde(rename = "job")]
    Job,
    #[serde(rename = "story")]
    Story,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "pollopt")]
    Pollopt,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: Id,
    deleted: Option<bool>,
    #[serde(rename = "type")]
    ty: ItemTy,
    by: Option<String>,
    time: i64,
    text: Option<String>,
    dead: Option<bool>,
    parent: Option<Id>,
    kids: Option<Vec<Id>>,
    url: Option<String>,
    score: Option<u32>,
    title: String,
}
fn top_stories(max: usize) -> Vec<Item> {
    let req = reqwest::blocking::get(&format!("{}/topstories.json", BASE)).unwrap();
    let text = req.text().unwrap();
    let mut ids: Vec<u32> = serde_json::from_str(&text).unwrap();

    ids.truncate(max);

    ids.iter().map(|i| Item::get(*i)).collect()
}
struct ButtonStyle;
impl iced::button::StyleSheet for ButtonStyle {
    fn hovered(&self) -> iced::button::Style {
        let active = self.active();

        iced::button::Style {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.3, 0.3),
            ..active
        }
    }

    fn pressed(&self) -> iced::button::Style {
        iced::button::Style {
            shadow_offset: iced::Vector::new(3f32, 3f32),
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

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                state: State {
                    scroll: iced::scrollable::State::default(),
                    comment_button: vec![iced::button::State::default(); 255],
                },
                top_stories: top_stories(25),
                max_stories: 25,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Hacker News")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::LinkPressed(i) => {
                let post = self.top_stories.iter().find(|it| it.id == i).unwrap();
                let url = post.url.as_ref().unwrap();
                webbrowser::open(url).unwrap();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let cheems = Text::new("Hacker News Stories:");
        let mut content = Scrollable::new(&mut self.state.scroll)
            .width(Length::Fill)
            .align_items(Align::Start)
            .padding(24)
            .spacing(10)
            .push(cheems);
        let max = 10;
        let stories = self
            .top_stories
            .iter()
            .zip(self.state.comment_button.iter_mut())
            .zip(1..max)
            .fold(
                Column::new().spacing(20),
                |mut col: Column<Message>, ((it, state), i)| {
                    if it.url.is_some() && it.by.is_some() && it.score.is_some() {
                        let label = Text::new(format!("{}. {}", i as i32, it.title)).size(24);
                        col = col.push(label);
                        let url = Text::new(it.url.as_ref().unwrap())
                            .size(15)
                            .horizontal_alignment(iced::HorizontalAlignment::Center);
                        let butt = Button::new(state, url)
                            .on_press(Message::LinkPressed(it.id))
                            .style(ButtonStyle {});
                        col = col.push(butt);

                        let under = format!(
                            "{} points by {}",
                            it.score.unwrap(),
                            it.by.as_ref().unwrap()
                        );
                        col = col.push(Text::new(under).size(13));
                        col
                    } else {
                        let label = Text::new(it.url.as_ref().unwrap());
                        col.push(label)
                    }
                },
            );
        content = content.push(stories);
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            // .center_y()
            .into()
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    LinkPressed(u32),
}
fn main() {
    App::run(Settings::default()).unwrap();
}
