extern crate colored;
extern crate env_logger;
extern crate list;
#[macro_use] extern crate log;
#[macro_use] extern crate neon;
extern crate post;
extern crate walkdir;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});
