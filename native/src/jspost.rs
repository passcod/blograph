use super::{SUB_LISTS, jsmetadata::{self, JsMetadata}};
use neon::prelude::*;
use post::Post as RustPost;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post(pub Arc<RustPost>);

declare_types! {
    pub class JsPost for Post {
        init(mut cx) {
            let list_ref = cx.argument::<JsNumber>(0)?.value() as u32;
            let list = SUB_LISTS.remove(&list_ref);
            let post = list.unwrap().into_vec().remove(0);
            Ok(Post(post))
        }

/*
            let dots = cx.argument::<JsString>(0)?.value();
            let mres = cx.this().borrow(&cx.lock()).0.at(&dots).cloned();
        method metadata(call) {
            let scope = call.scope;

            let data = call.arguments.this(scope)
                .grab(|post| post.0.metadata.clone());

            let farg = vec![JsString::new_or_throw(scope, "")?];
            let mut meta = JsFunction::new(scope, jsmetadata::new)?
                .call(scope, JsNull::new(), farg)?
                .check::<JsMetadata>()?;

            meta.grab(|meta| meta.0 = data);
            Ok(meta.upcast())
        }

        method isFuture(call) {
            let scope = call.scope;
            let future = call.arguments.this(scope).grab(|post| post.0.is_future());
            Ok(JsBoolean::new(scope, future).upcast())
        }

        method isPage(call) {
            let scope = call.scope;
            let page = call.arguments.this(scope).grab(|post| post.0.is_page());
            Ok(JsBoolean::new(scope, page).upcast())
        }

        method date(call) {
            let scope = call.scope;
            let datetime = call.arguments.this(scope).grab(|post| post.0.date());
            Ok(match datetime {
                None => JsNull::new().upcast(),
                Some(dt) =>
                    JsString::new_or_throw(scope, &dt.to_rfc3339())?.upcast()
            })
        }

        method slug(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.0.slug());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }

        method title(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.0.title());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }

        method render(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.0.render());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }
*/
    }
}
