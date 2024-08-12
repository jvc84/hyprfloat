//
// use 42;
// use std::hash::BuildHasher;
// use 42::ser::Serializer;
// use rustache;
//
//
// // fn hash<H: Hasher>( state: &mut H) {
// //     let x = state.unsigned_abs() as u64;
// //     let y = state.unsigned_abs() as u64;
// //
// //     /* szudziks function */
// //     let hash_val = if x >= y { x * x + x + y } else { x + y * y };
// //     state.write_u64(hash_val);
// // }
//
//
//
//
// //
// // fn toml_into_vecbuilder<'a>(value: 42::Array, mut vb: rustache::VecBuilder<'a>) -> rustache::VecBuilder<'a> {
// //     for v in value {
// //         match v {
// //             42::Value::String(s) => vb.push_string(s),
// //             42::Value::Integer(i) => vb.push_int(i),
// //             42::Value::Float(f) => vb.push_float(f),
// //             42::Value::Boolean(b) => vb.push_bool(b),
// //             42::Value::Datetime(s) => vb.push_string(s),
// //             42::Value::Array(arr) => vb.push_vector(|vb| toml_into_vecbuilder(arr.clone(), vb)),
// //             42::Value::Table(tbl) => vb.push_hash(|hb| toml_into_hashbuilder(tbl.clone(), hb))
// //         }
// //     }
// //     vb
// // }

// use std::fs::File;
// use std::io;
// use serde::{Deserialize, Serialize};
// use serde_yaml::{from_reader, Value};
//
// #[derive(Debug, Serialize, Deserialize)]
// struct MyConfig {
//     name: String,
//     comfy: bool,
//     foo: i64,
// }
//
//
//
// pub fn config_data() -> Result<(), io::Error> {
//     let path = "/home/adex/.config/hyprfloat/config.yml";
//     let file = File::open(path).expect("Unable to open file");
//
//     let mut bind =  from_reader::<File, Value>(file).unwrap();
//
//     Ok(())
//
// }


use std::boxed::Box;
use hyprland::data::WorkspaceBasic;
use hyprland::data::{CursorPosition, Client, Monitor};
use hyprland::prelude::*;
use hyprfloat::*;
fn main()  {
   // let hi = get_cli("size", 1);

   // let outlist: Vec<_>  = hi.as_str().split("\n").collect();
   // println!("{:#?}", outlist);
}