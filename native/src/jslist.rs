use super::{SUB_LISTS, jsmetadata::{self, JsMetadata}};
//use super::jspost::{self, JsPost};
use list::List as RustList;
use neon::prelude::*;
use post::Post;
use std::ops::DerefMut;
use std::sync::Arc;

pub struct List(pub RustList);

declare_types! {
    pub class JsList for List {
        init(mut cx) {
            let list_ref = cx.argument::<JsNumber>(0)?.value() as u32;
            let list = SUB_LISTS.remove(&list_ref).unwrap();
            Ok(List(list))
        }

        method length(mut cx) {
            let length = cx.this().borrow(&cx.lock()).0.len();
            Ok(cx.number(length as f64).as_value(&mut cx))
        }

/*
        method toArray(call) {
            let scope = call.scope;
            let args = call.arguments;

            let vec = args.this(scope).grab(|list| list.0.to_vec());
            let mut array = JsArray::new(scope, vec.len() as u32);

            {
                let mut i = 0u32;
                let raw_array = array.deref_mut();
                for post in vec {
                    let marg = vec![JsString::new_or_throw(scope, "")?];
                    let meta = JsFunction::new(scope, jsmetadata::new)?
                        .call(scope, JsNull::new(), marg)?
                        .check::<JsMetadata>()?;

                    let parg: Vec<Handle<JsValue>> = vec![
                        JsString::new_or_throw(scope, "")?.upcast(),
                        meta.upcast(),
                        JsString::new_or_throw(scope, "")?.upcast(),
                    ];

                    let mut newpost = JsFunction::new(scope, jspost::new)?
                        .call(scope, JsNull::new(), parg)?
                        .check::<JsPost>()?;

                    newpost.grab(|p| p.0 = post);
                    raw_array.set(i, newpost)?;
                    i += 1;
                }
            }

            Ok(array.upcast())
        }

        method iter(call) {
            let scope = call.scope;
            let args = call.arguments;
            let fun = args.require(scope, 0)?.check::<JsFunction>()?;

            let mut this = args.this(scope);
            let iter = this.grab(|list| list.0.iter());

            for item in iter {
                let mut post = {
                    let marg = vec![JsString::new_or_throw(scope, "")?];
                    let meta = JsFunction::new(scope, jsmetadata::new)?
                        .call(scope, JsNull::new(), marg)?
                        .check::<JsMetadata>()?;

                    let parg: Vec<Handle<JsValue>> = vec![
                        JsString::new_or_throw(scope, "")?.upcast(),
                        meta.upcast(),
                        JsString::new_or_throw(scope, "")?.upcast(),
                    ];

                    JsFunction::new(scope, jspost::new)?
                        .call(scope, JsNull::new(), parg)?
                        .check::<JsPost>()?
                };

                let mut prev = {
                    let marg = vec![JsString::new_or_throw(scope, "")?];
                    let meta = JsFunction::new(scope, jsmetadata::new)?
                        .call(scope, JsNull::new(), marg)?
                        .check::<JsMetadata>()?;

                    let parg: Vec<Handle<JsValue>> = vec![
                        JsString::new_or_throw(scope, "")?.upcast(),
                        meta.upcast(),
                        JsString::new_or_throw(scope, "")?.upcast(),
                    ];

                    JsFunction::new(scope, jspost::new)?
                        .call(scope, JsNull::new(), parg)?
                        .check::<JsPost>()?
                };

                let mut next = {
                    let marg = vec![JsString::new_or_throw(scope, "")?];
                    let meta = JsFunction::new(scope, jsmetadata::new)?
                        .call(scope, JsNull::new(), marg)?
                        .check::<JsMetadata>()?;

                    let parg: Vec<Handle<JsValue>> = vec![
                        JsString::new_or_throw(scope, "")?.upcast(),
                        meta.upcast(),
                        JsString::new_or_throw(scope, "")?.upcast(),
                    ];

                    JsFunction::new(scope, jspost::new)?
                        .call(scope, JsNull::new(), parg)?
                        .check::<JsPost>()?
                };

                post.grab(|p| p.0 = item.post.clone());

                let args: Vec<Handle<JsValue>> = vec![
                    post.upcast(),
                    match item.previous {
                        None => JsNull::new().upcast(),
                        Some(pre) => {
                            prev.grab(|p| p.0 = pre.clone());
                            prev.upcast()
                        }
                    },
                    match item.next {
                        None => JsNull::new().upcast(),
                        Some(nxt) => {
                            next.grab(|p| p.0 = nxt.clone());
                            next.upcast()
                        }
                    }
                ];

                fun.call(scope, JsNull::new(), args)?;
            }

            Ok(JsUndefined::new().upcast())
        }
*/

        // The rest of the methods can be reimplemented in JS with much better
        // aesthetics and performance than bridging them in Rust.
    }
}

