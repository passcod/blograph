use neon::js::class::Class;
use neon::js::{JsBoolean, JsFunction, JsNull, JsString};
use neon::vm::{Call, JsResult, Lock};
use post::Post as RustPost;
use std::sync::Arc;
use super::jsmetadata::{self, JsMetadata};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Post(pub Arc<RustPost>);

declare_types! {
    pub class JsPost for Post {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;

            let path = args.require(scope, 0)?.check::<JsString>()?.value();
            let metadata = args
                .require(scope, 1)?
                .check::<JsMetadata>()?
                .grab(|meta| meta.0.clone());
            let content = args.require(scope, 2)?.check::<JsString>()?.value();

            Ok(Post(Arc::new(RustPost::from(&path, metadata, &content))))
        }

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
    }
}

pub fn new(call: Call) -> JsResult<JsPost> {
    let scope = call.scope;
    let args = call.arguments;
    let arg0 = args.require(scope, 0)?;
    let arg1 = args.require(scope, 1)?;
    let arg2 = args.require(scope, 2)?;

    let post_class = JsPost::class(scope)?;
    let post_ctor = post_class.constructor(scope)?;
    post_ctor.construct(scope, vec![arg0, arg1, arg2])
}
