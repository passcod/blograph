extern crate colored;
extern crate env_logger;
extern crate list;
#[macro_use] extern crate log;
#[macro_use] extern crate neon;
extern crate post;
extern crate walkdir;
extern crate yaml_rust;

mod jsmetadata;
mod jspost;

register_module!(m, {
    m.export("Metadata", jsmetadata::new)?;
    m.export("Post", jspost::new)?;
    Ok(())
});
