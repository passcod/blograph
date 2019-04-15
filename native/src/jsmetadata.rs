use neon::js::class::Class;
use neon::js::{
    JsArray, JsBoolean, JsInteger, JsNull, JsNumber, JsObject, JsString, JsValue, Object, Value,
};
use neon::mem::Handle;
use neon::scope::Scope;
use neon::vm::{Call, JsResult, Lock};
use post::Metadata as RustMetadata;
use std::i32;
use std::ops::DerefMut;
use yaml_rust::Yaml;

fn i64_to_js<'a, T: Scope<'a>>(scope: &mut T, i: i64) -> Handle<'a, JsValue> {
    match i {
        -2147483648 ... 2147483647 // Hardcoded i32::MIN and i32::MAX
            => JsInteger::new(scope, i as i32).as_value(scope),
        f @ _ => JsNumber::new(scope, f as f64).as_value(scope)
    }
}

fn str_to_js<'a, T: Scope<'a>>(scope: &mut T, s: &str) -> Handle<'a, JsValue> {
    match JsString::new(scope, s) {
        None => JsNull::new().as_value(scope),
        Some(s) => s.as_value(scope),
    }
}

fn yaml_to_js<'a, T: Scope<'a>>(scope: &mut T, yaml: &Yaml) -> Handle<'a, JsValue> {
    match yaml {
        &Yaml::Boolean(b) => JsBoolean::new(scope, b).as_value(scope),

        &Yaml::Integer(i) => i64_to_js(scope, i),

        r @ &Yaml::Real(_) => JsNumber::new(scope, r.as_f64().unwrap_or(0f64)).as_value(scope),

        &Yaml::String(ref s) => str_to_js(scope, &s),

        &Yaml::Array(ref v) => {
            let mut array: Handle<JsArray> = JsArray::new(scope, v.len() as u32);

            {
                let raw_array = array.deref_mut();
                let mut i: u32 = 0;
                for val in v.iter().map(|y| yaml_to_js(scope, y)) {
                    if let Err(_) = raw_array.set(i, val) {
                        warn!("Couldn't set array index {}, skipping", i);
                    } else {
                        i += 1;
                    }
                }
            }

            array.as_value(scope)
        }

        &Yaml::Hash(ref h) => {
            let mut hash: Handle<JsObject> = JsObject::new(scope);

            {
                let raw_hash = hash.deref_mut();
                for (key, val) in h
                    .iter()
                    .map(|(yk, yv)| (yaml_to_js(scope, yk), yaml_to_js(scope, yv)))
                {
                    if let Err(_) = raw_hash.set(key, val) {
                        warn!("Couldn't set hash key, skipping");
                    }
                }
            }

            hash.as_value(scope)
        }

        _ => JsNull::new().as_value(scope),
    }
}

pub struct Metadata(pub RustMetadata);

declare_types! {
    pub class JsMetadata for Metadata {
        init(call) {
            let scope = call.scope;
            let args = call.arguments;
            let yaml = args.require(scope, 0)?.check::<JsString>()?.value();

            Ok(Metadata(RustMetadata::parse(&yaml)))
        }

        method at(call) {
            let scope = call.scope;
            let args = call.arguments;
            let dots = args.require(scope, 0)?.check::<JsString>()?.value();

            Ok(match args.this(scope).grab(|meta| meta.0.at(&dots)) {
                None => JsNull::new().upcast(),
                Some(y) => yaml_to_js(scope, y)
            })
        }

        method bool(call) {
            let scope = call.scope;
            let args = call.arguments;
            let dots = args.require(scope, 0)?.check::<JsString>()?.value();

            Ok(match args.this(scope).grab(|meta| meta.0.bool(&dots)) {
                None => JsNull::new().upcast(),
                Some(b) => JsBoolean::new(scope, b).upcast()
            })
        }

        method int(call) {
            let scope = call.scope;
            let args = call.arguments;
            let dots = args.require(scope, 0)?.check::<JsString>()?.value();

            Ok(match args.this(scope).grab(|meta| meta.0.int(&dots)) {
                None => JsNull::new().upcast(),
                Some(i) => i64_to_js(scope, i)
            })
        }

        method string(call) {
            let scope = call.scope;
            let args = call.arguments;
            let dots = args.require(scope, 0)?.check::<JsString>()?.value();

            Ok(match args.this(scope).grab(|meta| meta.0.str(&dots)) {
                None => JsNull::new().upcast(),
                Some(s) => str_to_js(scope, s)
            })
        }

        method page(call) {
            let scope = call.scope;
            let args = call.arguments;
            let b = args.this(scope).grab(|meta| meta.0.page());

            Ok(JsBoolean::new(scope, b).upcast())
        }

        method date(call) {
            let scope = call.scope;
            let datetime = call.arguments.this(scope).grab(|meta| meta.0.date());

            Ok(match datetime {
                None => JsNull::new().upcast(),
                Some(dt) =>
                    JsString::new_or_throw(scope, &dt.to_rfc3339())?.upcast()
            })
        }

        method parents(call) {
            let scope = call.scope;
            let ps = call.arguments.this(scope).grab(|meta| meta.0.parents());

            let mut array: Handle<JsArray> = JsArray::new(scope, ps.len() as u32);

            {
                let raw_array = array.deref_mut();
                let mut i: u32 = 0;
                for val in ps.iter().map(|s| str_to_js(scope, &s)) {
                    raw_array.set(i, val)?;
                    i += 1;
                }
            }

            Ok(array.as_value(scope))
        }

        method toJSON(call) {
            let scope = call.scope;
            let args = call.arguments;

            let yaml = args.this(scope).grab(|meta| meta.0.yaml.clone());
            Ok(yaml_to_js(scope, &yaml))
        }

        // #author, #tags, and #title are just straight queries using the
        // normal #at/#string methods, so instead of binding them in Rust,
        // they're rewritten trivially in JS.
    }
}

pub fn new(call: Call) -> JsResult<JsMetadata> {
    let scope = call.scope;
    let args = call.arguments;
    let arg0 = args.require(scope, 0)?;

    let meta_class = JsMetadata::class(scope)?;
    let meta_ctor = meta_class.constructor(scope)?;
    meta_ctor.construct(scope, vec![arg0])
}
