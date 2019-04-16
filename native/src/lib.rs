#[macro_use]
extern crate neon;

use arc_swap::ArcSwap;
use chashmap::CHashMap;
//use jslist::JsList;
use list::List;
use neon::prelude::*;
use std::path::PathBuf;

mod all;
mod jslist;
mod jsmetadata;
mod jspost;
mod logger;

lazy_static::lazy_static! {
    pub static ref MAIN_LIST: ArcSwap<List> = ArcSwap::default();
    pub static ref SUB_LISTS: CHashMap<u32, List> = CHashMap::new();
}

/*
fn load(mut cx: FunctionContext) -> JsResult<JsList> {
    let scope = cx.scope;
    let args = cx.arguments;
    let base = args.require(scope, 0)?.check::<JsString>()?.value();

    let posts = all::load(PathBuf::from(base)).to_vec();

    let farg = vec![JsArray::new(scope, 0)];
    let mut list = JsFunction::new(scope, jslist::new)?
        .call(scope, JsNull::new(), farg)?
        .check::<JsList>()?;

    list.grab(|list| list.0 = List::new(posts));
    Ok(list)
}
*/

register_module!(mut m, {
    logger::init();
    m.export_class::<jsmetadata::JsMetadata>("Metadata")?;
    m.export_class::<jspost::JsPost>("Post")?;
    m.export_class::<jslist::JsList>("List")?;
    //m.export("load", load)?;
    Ok(())
});
