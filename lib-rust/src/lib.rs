#[macro_use]
extern crate derive_builder;

pub mod any;
pub mod config;
pub mod debugger;
pub mod emit;
pub mod ffmpeg;
pub mod logging;
pub mod main_thread;
pub mod photon;
pub mod pipeline;
mod pipeline_simple;
pub mod runner;
pub mod tensorflow;
pub mod tesseract;
pub mod threshold;
pub mod trigger;
