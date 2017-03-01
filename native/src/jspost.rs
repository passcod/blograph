use neon::vm::{Call, JsResult, Throw};
use neon::js::class::Class;
use neon::js::Value;
use post::Post;
use std::ops::Deref;
use std::path::PathBuf;

declare_types!{
    pub class JsPost for Post {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;

            let base = PathBuf::from(
                try!(try!(args.require(scope, 0))
                    .deref()
                    .to_string(scope))
                    .value()
            );

            let path = PathBuf::from(
                try!(try!(args.require(scope, 1))
                    .deref()
                    .to_string(scope))
                    .value()
            );

            match Post::new(&base, path) {
                Ok(p) => Ok(p),
                Err(e) => {
                    error!("{}", e);
                    Err(Throw)
                }
            }
        }
    }
}

pub fn new(call: Call) -> JsResult<JsPost> {
    let mut scope = call.scope;
    let args = call.arguments;
    let arg0 = try!(args.require(scope, 0));
    let arg1 = try!(args.require(scope, 1));

    let post_class = try!(JsPost::class(scope));
    let post_ctor = try!(post_class.constructor(scope));
    post_ctor.construct(scope, vec![arg0, arg1])
}
