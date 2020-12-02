// use reqwest::blocking::Client;
// use serde::{Deserialize, Serialize};

// type Id = u32;

// const BASE: &str = "https://hacker-news.firebaseio.com/v0";

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
//     kids: Option<Vec<Id>>
// }
// impl Item{
//     fn get(id: Id) -> Self {
//         let test = reqwest::blocking::get(&format!("{}/item/{}.json?print=pretty", BASE, id)).unwrap();
//         let text = test.text().unwrap();
//         serde_json::from_str(&text).unwrap()
//     }
//     fn get_children(&self) -> Vec<Self> {
//         println!("Get_Children Called");
//         let mut children = vec![];
//         if self.kids.is_some() {
//             println!("Get_Children Has Kids");
//             for kid in self.kids.as_ref().unwrap(){
//                 println!("Get_Children kid={}", kid);
//                 let k = Item::get(*kid);
//                 for c in k.get_children() {
//                     children.push(c.clone());
//                     println!("Get_Children inserted {:#?} into {:#?}", c, children);

//                 }
//             }
//             // children.insert(0, self.kids.unwrap())
//         }else {
//             println!("Get_Children doesnt have kids??")
//         }
//         children
//     }
// }
// fn main() {
//     let i = Item::get(25270729);
//     println!("i {:#?}", i);
//     println!("I's children: {:#?}", i.get_children());
//     // if i.kids.is_some(){
//     //     for kid_id in &i.kids.unwrap() {
//     //         let kid = Item::get(*kid_id);
//     //         if kid.by.is_none(){
//     //             println!(" {:#?}", kid);
//     //         }
//     //         // println!("kid {} {}: {:?}", kid_id, kid.by, kid.text)
//     //     }
//     // }
    
// }

fn main(){
    let client = hn_api::HnClient::init().unwrap();
    let best_stories =  client.get_best_stories().unwrap();
    let best = best_stories.iter().next().unwrap();
    println!("{:#?}", client.get_item(*best).unwrap());
}