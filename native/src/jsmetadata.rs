use log::warn;
use neon::prelude::*;
use neon::types::{JsArray, JsBoolean, JsNull, JsNumber, JsObject, JsString, JsValue, Value};
use post::Metadata as RustMetadata;
use std::i32;
use std::ops::DerefMut;
use yaml_rust::Yaml;

fn i64_to_js<'a, T: Context<'a>>(cx: &mut T, i: i64) -> Handle<'a, JsValue> {
    JsNumber::new(cx, i as f64).as_value(cx)
}

fn str_to_js<'a, T: Context<'a>>(cx: &mut T, s: &str) -> Handle<'a, JsValue> {
    JsString::try_new(cx, s)
        .map(|s| s.as_value(cx))
        .unwrap_or_else(|_| JsNull::new().as_value(cx))
}

fn yaml_to_js<'a, T: Context<'a>>(cx: &mut T, yaml: &Yaml) -> Handle<'a, JsValue> {
    match yaml {
        &Yaml::Boolean(b) => JsBoolean::new(cx, b).as_value(cx),

        &Yaml::Integer(i) => i64_to_js(cx, i),

        r @ &Yaml::Real(_) => JsNumber::new(cx, r.as_f64().unwrap_or(0f64)).as_value(cx),

        &Yaml::String(ref s) => str_to_js(cx, &s),

        &Yaml::Array(ref v) => {
            let mut array: Handle<JsArray> = JsArray::new(cx, v.len() as u32);

            let raw_array = array.deref_mut();
            let mut i: u32 = 0;
            let jsvals: Vec<Handle<'a, JsValue>> =
                v.iter().map(|y| yaml_to_js(cx, y)).collect();
            for val in jsvals {
                if let Err(_) = raw_array.set(cx, i, val) {
                    warn!("Couldn't set array index {}, skipping", i);
                } else {
                    i += 1;
                }
            }

            array.as_value(cx)
        }

        &Yaml::Hash(ref h) => {
            let mut hash: Handle<JsObject> = JsObject::new(cx);

            let raw_hash = hash.deref_mut();
            let jsvals: Vec<(Handle<'a, JsValue>, Handle<'a, JsValue>)> = h
                .iter()
                .map(|(yk, yv)| (yaml_to_js(cx, yk), yaml_to_js(cx, yv)))
                .collect();
            for (key, val) in jsvals {
                if let Err(_) = raw_hash.set(cx, key, val) {
                    warn!("Couldn't set hash key, skipping");
                }
            }

            hash.as_value(cx)
        }

        _ => JsNull::new().as_value(cx),
    }
}

pub struct Metadata(pub RustMetadata);

declare_types! {
    pub class JsMetadata for Metadata {
        init(mut cx) {
            let yaml: String = cx.argument::<JsString>(0)?.value();
            Ok(Metadata(RustMetadata::parse(&yaml)))
        }

        method at(mut cx) {
            let dots = cx.argument::<JsString>(0)?.value();
            let mres = cx.this().borrow(&cx.lock()).0.at(&dots).cloned();

            Ok(match mres {
                None => JsNull::new().upcast(),
                Some(y) => yaml_to_js(&mut cx, &y)
            })
        }

        method bool(mut cx) {
            let dots = cx.argument::<JsString>(0)?.value();
            let mres = cx.this().borrow(&cx.lock()).0.bool(&dots);

            Ok(match mres {
                None => JsNull::new().upcast(),
                Some(b) => JsBoolean::new(&mut cx, b).upcast()
            })
        }

        method int(mut cx) {
            let dots = cx.argument::<JsString>(0)?.value();
            let mres = cx.this().borrow(&cx.lock()).0.int(&dots);

            Ok(match mres {
                None => JsNull::new().upcast(),
                Some(i) => i64_to_js(&mut cx, i)
            })
        }

        method string(mut cx) {
            let dots = cx.argument::<JsString>(0)?.value();
            let mres = cx.this().borrow(&cx.lock()).0.str(&dots).map(|s| s.to_string());

            Ok(match mres {
                None => JsNull::new().upcast(),
                Some(s) => str_to_js(&mut cx, &s)
            })
        }

        method page(mut cx) {
            let is_page = cx.this().borrow(&cx.lock()).0.page();
            Ok(JsBoolean::new(&mut cx, is_page).upcast())
        }

        method date(mut cx) {
            let datetime = cx.this().borrow(&cx.lock()).0.date();

            Ok(match datetime {
                None => JsNull::new().upcast(),
                Some(dt) => cx.string(&dt.to_rfc3339()).upcast()
            })
        }

        method parents(mut cx) {
            let ps = cx.this().borrow(&cx.lock()).0.parents();
            let mut array: Handle<JsArray> = JsArray::new(&mut cx, ps.len() as u32);

            let jsvals: Vec<Handle<JsValue>> = ps.iter().map(|s| str_to_js(&mut cx, &s)).collect();
            let raw_array = array.deref_mut();
            let mut i: u32 = 0;
            for val in jsvals {
                raw_array.set(&mut cx, i, val)?;
                i += 1;
            }

            Ok(array.as_value(&mut cx))
        }

        method toJSON(mut cx) {
            let yaml = cx.this().borrow(&cx.lock()).0.yaml.clone();
            Ok(yaml_to_js(&mut cx, &yaml))
        }

        // #author, #tags, and #title are just straight queries using the
        // normal #at/#string methods, so instead of binding them in Rust,
        // they're rewritten trivially in JS.
    }
}
