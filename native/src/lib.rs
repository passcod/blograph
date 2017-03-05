extern crate colored;
extern crate env_logger;
extern crate list;
#[macro_use] extern crate log;
#[macro_use] extern crate neon;
extern crate post;
extern crate walkdir;
extern crate yaml_rust;

use jslist::JsList;
use list::List;
use neon::js::{JsArray, JsFunction, JsNull, JsString};
use neon::vm::{Call, JsResult, Lock};
use std::path::PathBuf;

mod all;
mod jsmetadata;
mod jspost;
mod jslist;

fn load(call: Call) -> JsResult<JsList> {
    let scope = call.scope;
    let args = call.arguments;
    let base = args.require(scope, 0)?.check::<JsString>()?.value();

    let posts = all::load(PathBuf::from(base)).to_vec();

    let farg = vec![JsArray::new(scope, 0)];
    let mut list = JsFunction::new(scope, jslist::new)?
        .call(scope, JsNull::new(), farg)?
        .check::<JsList>()?;

    list.grab(|list| list.0 = List::new(posts));
    Ok(list)
}

register_module!(m, {
    m.export("load", load)?;
    m.export("List", jslist::new)?;
    m.export("Metadata", jsmetadata::new)?;
    m.export("Post", jspost::new)?;
    Ok(())
});
