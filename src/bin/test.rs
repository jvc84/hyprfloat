// use serde::{Deserialize, Serialize};
// use serde_json::{Result, from_str};
// use std::fs;
// 
// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }
//
// fn main() -> Result<()> {
//     let file_contents = fs::read_to_string("data.json")?;
//     let people: Vec<Person> = from_str(&file_contents)?;
//
//     for person in people {
//         println!("Person: {:?}", person);
//     }
//     Ok(())
// }
fn main() {
  
}

// 
// use serde::{Deserialize, Serialize};
// use serde_json::{Result, from_reader};
// use std::fs::File;
// 
// // ... (Person struct remains the same) ...
// 
// fn main() -> Result<()> {
//     let file = File::open("data.json");
//     let person: Person = from_reader(file)?;
//     println!("Person: {:?}", person);
//     Ok(())
// }
