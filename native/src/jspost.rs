use neon::vm::{Call, JsResult, Lock, Throw};
use neon::js::class::Class;
use neon::js::{JsBoolean, JsNull, JsString};
use post::Post;
use std::path::PathBuf;

declare_types! {
    pub class JsPost for Post {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;

            let base = PathBuf::from(
                args.require(scope, 0)?.check::<JsString>()?.value());

            let path = PathBuf::from(
                args.require(scope, 1)?.check::<JsString>()?.value());

            match Post::new(&base, path) {
                Ok(p) => Ok(p),
                Err(e) => {
                    error!("{}", e);
                    Err(Throw)
                }
            }
        }

        // TODO: metadata()

        // method metadata(call) {
        //     let scope = call.scope;
        //     let meta = call.arguments.this(scope)
        //         .grab(|post| post.metadata.clone());
        //     Ok(JsBoolean::new(scope, future).upcast())
        // }

        method isFuture(call) {
            let scope = call.scope;
            let future = call.arguments.this(scope).grab(|post| post.is_future());
            Ok(JsBoolean::new(scope, future).upcast())
        }

        method isPage(call) {
            let scope = call.scope;
            let page = call.arguments.this(scope).grab(|post| post.is_page());
            Ok(JsBoolean::new(scope, page).upcast())
        }

        method date(call) {
            let scope = call.scope;
            let datetime = call.arguments.this(scope).grab(|post| post.date());
            Ok(match datetime {
                None => JsNull::new().upcast(),
                Some(dt) =>
                    JsString::new_or_throw(scope, &dt.to_rfc3339())?.upcast()
            })
        }

        method slug(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.slug());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }

        method title(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.title());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }

        method render(call) {
            let scope = call.scope;
            let s = call.arguments.this(scope).grab(|post| post.render());
            Ok(JsString::new_or_throw(scope, &s)?.upcast())
        }
    }
}

pub fn new(call: Call) -> JsResult<JsPost> {
    let mut scope = call.scope;
    let args = call.arguments;
    let arg0 = args.require(scope, 0)?;
    let arg1 = args.require(scope, 1)?;

    let post_class = JsPost::class(scope)?;
    let post_ctor = post_class.constructor(scope)?;
    post_ctor.construct(scope, vec![arg0, arg1])
}
