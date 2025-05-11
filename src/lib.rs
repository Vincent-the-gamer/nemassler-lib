#![deny(clippy::all)]

pub mod utils;
mod ncm2mp3;

use ncm2mp3::ncm2mp3;

#[macro_use]
extern crate napi_derive;

#[napi(js_name="ncm2mp3")]
pub fn binding_ncm2mp3(ncm_dir: String, out_dir: String) -> Vec<String> {
  let ncm_dir = &ncm_dir;
  let out_dir = &out_dir;
  ncm2mp3(ncm_dir, out_dir)
}