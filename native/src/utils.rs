use neon::prelude::{
    Context,JsArray,JsResult
};
use neon::object::Object;

pub fn vec_to_array<'a, C: Context<'a>>(vec: &Vec<String>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}