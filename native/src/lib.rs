extern crate colored;
extern crate env_logger;
extern crate list;
#[macro_use] extern crate log;
#[macro_use] extern crate neon;
extern crate neon_runtime;
extern crate post;
extern crate walkdir;
extern crate yaml_rust;

mod jslist;
mod jsmetadata;
mod jspost;

register_module!(m, {
    m.export("List", jslist::new)?;
    m.export("Metadata", jsmetadata::new)?;
    m.export("Post", jspost::new)?;
    Ok(())
});
