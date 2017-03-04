use list::List;
use neon::js::{JsArray, JsBoolean, JsNull, JsString, Value};
use neon::js::class::Class;
use neon::mem::Managed;
use neon::scope::Scope;
use neon::vm::{Call, JsResult, Lock, Throw};
use neon_runtime::raw;
use post::Post;
use std::path::PathBuf;
use std::sync::Arc;
use super::jspost::JsPost;

declare_types! {
    pub class JsList for List {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;

            let jsvec = args.require(scope, 0)?.check::<JsArray>()?.to_vec(scope)?;
            let mut posts: Vec<Arc<Post>> = vec![];
            for entry in jsvec {
                let post = entry.check::<JsPost>()?;
                post.lock().grab(|handle| handle.grab(|post|
                    posts.push(Arc::new(post.clone()))
                ))
            }

            Ok(List::new(posts))
        }
    }
}

pub fn new(call: Call) -> JsResult<JsList> {
    let mut scope = call.scope;
    let args = call.arguments;
    let arg0 = args.require(scope, 0)?;

    let list_class = JsList::class(scope)?;
    let list_ctor = list_class.constructor(scope)?;
    list_ctor.construct(scope, vec![arg0])
}
