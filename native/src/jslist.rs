use list::List;
use neon::js::{JsArray, JsFunction, JsNull, Object};
use neon::js::class::Class;
use neon::vm::{Call, JsResult, Lock};
use post::Post;
use std::ops::DerefMut;
use std::sync::Arc;
use super::jspost::{self, JsPost};

pub struct WrapList(pub List);

declare_types! {
    pub class JsList for WrapList {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;

            let jsvec = args.require(scope, 0)?.check::<JsArray>()?.to_vec(scope)?;
            let mut posts: Vec<Arc<Post>> = vec![];
            for entry in jsvec {
                let mut post = entry.check::<JsPost>()?;
                let apost = post.grab(|handle| handle.clone());
                posts.push(apost.0);
            }

            Ok(WrapList(List::new(posts)))
        }

        method toArray(call) {
            let scope = call.scope;
            let args = call.arguments;

            let vec = args.this(scope).grab(|list| list.0.to_vec());
            let mut array = JsArray::new(scope, vec.len() as u32);

            {
                let mut i = 0u32;
                let raw_array = array.deref_mut();
                for post in vec {
                    let farg = vec![JsArray::new(scope, 0)];
                    let mut newpost = JsFunction::new(scope, jspost::new)?
                        .call(scope, JsNull::new(), farg)?
                        .check::<JsPost>()?;

                    newpost.grab(|p| p.0 = post);
                    raw_array.set(i, newpost)?;
                    i += 1;
                }
            }

            Ok(array.upcast())
        }

        // The rest of the methods can be reimplemented in JS with much better
        // aesthetics and performance than bridging them in Rust.
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
