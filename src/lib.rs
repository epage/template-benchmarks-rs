#[macro_use]
extern crate askama;
#[macro_use]
extern crate horrorshow;
extern crate criterion;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate fomat_macros;

pub mod askama_bench;
pub mod fomat;
pub mod handlebars;
pub mod horrorshow_bench;
pub mod liquid;
pub mod ructe;
pub mod std_write;
pub mod tera;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
