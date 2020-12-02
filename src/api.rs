// use super::Id;
// use iced::futures;
// use serde::{Deserialize, Serialize};
// const BASE: &str = "https://hacker-news.firebaseio.com/v0";

// pub struct Download {
//     id: Id,
// }

// impl<H, I> iced_native::subscription::Recipe<H, I> for Download
// where
//     H: std::hash::Hasher,
// {
//     type Output = Progress;

//     fn hash(&self, state: &mut H) {
//         use std::hash::Hash;

//         std::any::TypeId::of::<Self>().hash(state);
//         self.id.hash(state);
//     }

//     fn stream(
//         self: Box<Self>,
//         _input: futures::stream::BoxStream<'static, I>,
//     ) -> futures::stream::BoxStream<'static, Self::Output> {
//         Box::pin(futures::stream::unfold(
//             State::Ready(self.url),
//             |state| async move {
//                 match state {
//                     State::Ready(url) => {
//                         let response = reqwest::get(&url).await;

//                         match response {
//                             Ok(response) => {
//                                 if let Some(total) = response.content_length() {
//                                     Some((
//                                         Progress::Started,
//                                         State::Downloading {
//                                             response,
//                                             total,
//                                             downloaded: 0,
//                                         },
//                                     ))
//                                 } else {
//                                     Some((Progress::Errored, State::Finished))
//                                 }
//                             }
//                             Err(_) => {
//                                 Some((Progress::Errored, State::Finished))
//                             }
//                         }
//                     }
//                     State::Downloading {
//                         mut response,
//                         total,
//                         downloaded,
//                     } => match response.chunk().await {
//                         Ok(Some(chunk)) => {
//                             let downloaded = downloaded + chunk.len() as u64;

//                             let percentage =
//                                 (downloaded as f32 / total as f32) * 100.0;

//                             Some((
//                                 Progress::Advanced(percentage),
//                                 State::Downloading {
//                                     response,
//                                     total,
//                                     downloaded,
//                                 },
//                             ))
//                         }
//                         Ok(None) => Some((Progress::Finished, State::Finished)),
//                         Err(_) => Some((Progress::Errored, State::Finished)),
//                     },
//                     State::Finished => {
//                         // We do not let the stream die, as it would start a
//                         // new download repeatedly if the user is not careful
//                         // in case of errors.
//                         let _: () = iced::futures::future::pending().await;

//                         None
//                     }
//                 }
//             },
//         ))
//     }
// }

// struct Comment {
//     by: Option<String>,
//     text: Option<String>,
//     time: i64,
//     replies: Vec<Comment>,
//     orphan: bool,
// }
// impl Item {
//     // fn get(id: Id) -> Self {
//     //     Box::pin
//     //     println!("Sending request... {}", id);
//     //     let test =
//     //         reqwest::blocking::get(&format!("{}/item/{}.json?print=pretty", BASE, id)).unwrap();
//     //         println!("Got response... {}", id);

//     //         let text = test.text().unwrap();
//     //     serde_json::from_str(&text).unwrap()
//     // }
// }
// #[derive(Debug, Serialize, Deserialize, Clone)]
// enum ItemTy {
//     #[serde(rename = "job")]
//     Job,
//     #[serde(rename = "story")]
//     Story,
//     #[serde(rename = "comment")]
//     Comment,
//     #[serde(rename = "poll")]
//     Poll,
//     #[serde(rename = "pollopt")]
//     Pollopt,
// }
// #[derive(Debug, Serialize, Deserialize, Clone)]
// struct Item {
//     id: Id,
//     deleted: Option<bool>,
//     #[serde(rename = "type")]
//     ty: ItemTy,
//     by: Option<String>,
//     time: i64,
//     text: Option<String>,
//     dead: Option<bool>,
//     parent: Option<Id>,
//     kids: Option<Vec<Id>>,
//     url: Option<String>,
//     score: Option<i32>,
//     title: Option<String>,
//     /// Not meant for serde to deserialize
//     replies___: Option<Vec<Item>>,
// }
// // fn top_stories(max: usize) -> Vec<Item> {
// //     let req = reqwest::blocking::get(&format!("{}/topstories.json", BASE)).unwrap();
// //     let text = req.text().unwrap();
// //     let mut ids: Vec<Id> = serde_json::from_str(&text).unwrap();

// //     ids.truncate(max);

// //     ids.iter().map(|i| Item::get(*i)).collect()
// // }

// // fn children_as_items(root: &mut Item) -> Vec<Item> {
// //     match &root.kids {
// //         Some(kids) => {
// //             let mut kids_items: Vec<Item> = kids.iter().map(|i| Item::get(*i)).collect();
// //             for item in kids_items.iter_mut() {
// //                 item.replies___ = Some(children_as_items(item));
// //             }
// //             // println!("kids items {:#?}", kids_items);
// //             kids_items
// //         }
// //         None => vec![],
// //     }
// // }