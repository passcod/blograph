extern crate colored;
extern crate env_logger;
extern crate list;
#[macro_use] extern crate log;
#[macro_use] extern crate neon;
extern crate post;
extern crate walkdir;

mod jspost;

register_module!(m, {
    try!(m.export("Post", jspost::new));
    Ok(())
});
