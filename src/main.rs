#![allow(dead_code)]
use chrono::{TimeZone, Utc};
use iced::{
    executor, Align, Application, Background, Button, Color, Column, Command, Container, Length,
    Row, Scrollable, Settings, Space, Text,
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
type Id = i64;
const BASE: &str = "https://hacker-news.firebaseio.com/v0";
#[derive(Debug)]
enum Mode {
    HomePage,
    Comment(Id),
}
#[derive(Debug)]
struct State {
    scroll: iced::scrollable::State,
    refresh: iced::button::State,
    back_button: iced::button::State,

    comment_button: Vec<(iced::button::State, iced::button::State)>,
}
#[derive(Debug)]
struct App {
    state: State,
    top_stories: Vec<Item>,
    max_stories: usize,
    mode: Mode,
}
struct Comment {
    by: Option<String>,
    text: Option<String>,
    time: i64,
    replies: Vec<Comment>,
    orphan: bool,
}
impl Item {
    fn get(id: Id) -> Self {
        let test =
            reqwest::blocking::get(&format!("{}/item/{}.json?print=pretty", BASE, id)).unwrap();
        let text = test.text().unwrap();
        serde_json::from_str(&text).unwrap()
    }
    fn getr(id: &Id) -> Self {
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
    score: Option<i32>,
    title: Option<String>,
    /// Not meant for serde to deserialize
    replies___: Option<Vec<Item>>,
}
fn top_stories(max: usize) -> Vec<Item> {
    let req = reqwest::blocking::get(&format!("{}/topstories.json", BASE)).unwrap();
    let text = req.text().unwrap();
    let mut ids: Vec<Id> = serde_json::from_str(&text).unwrap();

    ids.truncate(max);

    ids.iter().map(|i| Item::get(*i)).collect()
}

fn children_as_items(root: &mut Item) -> Vec<Item> {
    match &root.kids {
        Some(kids) => {
            let mut kids_items: Vec<Item> = kids.iter().map(|i| Item::get(*i)).collect();
            for item in kids_items.iter_mut() {
                item.replies___ = Some(children_as_items(item));
            }
            // println!("kids items {:#?}", kids_items);
            kids_items
        }
        None => vec![],
    }
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
                    comment_button: vec![
                        (
                            iced::button::State::default(),
                            iced::button::State::default()
                        );
                        255
                    ],
                    refresh: iced::button::State::default(),
                },
                top_stories: top_stories(3),
                max_stories: 25,
                mode: Mode::HomePage,
            },
            Command::none(),
        );
        println!("created app");
        app
    }

    fn title(&self) -> String {
        format!("Hacker news - {} stories.", self.top_stories.len())
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::LinkPressed(i) => {
                let post = self.top_stories.iter().find(|it| it.id == i).unwrap();
                let url = post.url.as_ref().unwrap();
                webbrowser::open(url).unwrap();
            }

            Message::Refresh => self.top_stories = top_stories(self.max_stories),
            Message::GotoComments(id) => {
                self.mode = Mode::Comment(id);
            }
            Message::GotoHome => {
                self.mode = Mode::HomePage;
            }
        }
        Command::none()
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
                let stories = self
                    .top_stories
                    .iter()
                    .zip(self.state.comment_button.iter_mut())
                    .zip(1..self.max_stories)
                    .fold(
                        Column::new().spacing(20),
                        |mut col: Column<Message>, ((it, (state, comment_state)), i)| {
                            if it.url.is_some() && it.by.is_some() && it.score.is_some() {
                                let label = Text::new(format!(
                                    "{}. {}",
                                    i as i32,
                                    it.title.as_ref().unwrap()
                                ))
                                .size(24);
                                col = col.push(label);
                                let url = Text::new(it.url.as_ref().unwrap())
                                    .size(15)
                                    .horizontal_alignment(iced::HorizontalAlignment::Center);

                                let mut r = Row::new();
                                let comments =
                                    Button::new(comment_state, Text::new("comments").size(15))
                                        .style(ButtonStyle {})
                                        .on_press(Message::GotoComments(it.id));

                                let butt = Button::new(state, url)
                                    .on_press(Message::LinkPressed(it.id))
                                    .style(ButtonStyle {});

                                r = r.push(comments).push(butt);
                                col = col.push(r);
                                let under = format!(
                                    "{} points by {}",
                                    it.score.unwrap(),
                                    it.by.as_ref().unwrap()
                                );
                                col = col.push(Text::new(under).size(13));
                                col
                            } else if it.text.is_some() {
                                let label = Text::new(it.text.as_ref().unwrap()).size(17);
                                col.push(label)
                            } else {
                                col
                            }
                        },
                    );
                content = content.push(stories);
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
                let post = Item::get(id);
                fn item_to_el<'a>(depth: i32, item_id: Id) -> Column<'a, Message> {
                    let mut item = Item::get(item_id);
                    let mut r = Column::new().spacing(15);
                    if item.text.is_some() {
                        if item.by.as_ref().is_some() {
                            let d = Utc.timestamp(item.time, 0);
                            r = r.push(
                                Text::new(format!("{} {}", item.by.as_ref().unwrap(), d))
                                    .color(Color::from_rgb8(255, 165, 0)),
                            );
                            r = r.push(Text::new(item.text.as_ref().unwrap()));
                        }

                        for i in children_as_items(&mut item) {
                            let mut row = Row::new();
                            row = row.push(Space::with_height(Length::Units(15)));
                            row = row.push(Space::with_width(Length::Units(
                                (depth * 15).try_into().unwrap(),
                            )));
                            row = row.push(item_to_el(depth + 1, i.id));
                            row = row.push(Space::with_height(Length::Units(15)));

                            r = r.push(row)
                        }
                    }
                    r
                }
                for kid in post.kids.as_ref().unwrap() {
                    let mut col = Column::new();
                    col = col.push(item_to_el(0, *kid));

                    content = content.push(col);
                }
                // let kids = post.kids.as_ref().unwrap().get(0);
                // let mut col = Column::new();
                // col = col.push(item_to_el(0, *kids.unwrap()));

                // content = content.push(col);
                // println!("Recusrive kids: {:#?}", children_as_items(&mut i));
                // if i.kids.is_some() {
                //     let resolved: Vec<Item> = i.kids.unwrap().iter().map(|i| Item::getr(i)).collect();
                //     for item in resolved {
                //         println!("{:#?}", item);
                //         content = content.push(Text::new(item.text.unwrap()));
                //         let d = Utc.timestamp(item.time, 0);

                //         content = content.push(Text::new(d.to_string()))
                //     }
                // }

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    LinkPressed(Id),
    GotoComments(Id),
    GotoHome,
    Refresh,
}
fn main() {
    App::run(Settings::default()).unwrap();
}
