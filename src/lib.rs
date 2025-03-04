use ncm2mp3::{
    ncm2mp3,
    process_file
};
use neon::{prelude::ModuleContext, result::NeonResult};

mod ncm2mp3;
mod utils;

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let _ = cx.export_function("process_file", process_file);
    let _ = cx.export_function("ncm2mp3", ncm2mp3);
    Ok(())
}
