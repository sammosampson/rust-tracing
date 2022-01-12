use tracing::{info, instrument};
use tracing_subscriber::prelude::*;

mod custom_layer;
use custom_layer::CustomLayer;

fn main() {
    tracing_subscriber::registry().with(CustomLayer).init();

    outer("moulin");
}

#[instrument]
fn outer(jeff: &str) {
    inner(jeff, 1.3);
}

#[instrument]
fn inner(jeff: &str, poogis: f32) {
    info!(a_bool = true, answer = 42, message = "first example");
}