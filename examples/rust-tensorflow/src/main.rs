use std::sync::Arc;

use hypetrigger::error::Result;
use hypetrigger::photon::{Crop};
use hypetrigger::pipeline::Hypetrigger;
use hypetrigger::tensorflow::{load_tensorflow_model, TensorflowTrigger};
use hypetrigger::async_trigger::{AsyncTrigger, TriggerThread};


fn main() -> Result<()> {
    println!("Creating a trigger");

    let model_dir = "F:\\Software\\Hypetrigger\\data\\tensorflow-models\\ow2-elim";
    let crop = Some(Crop {
        top_percent: 33.0,
        left_percent: 26.499999999999996,
        width_percent: 28.999999999999996,
        height_percent: 51.55555555555555,
    });

    let input_file = "C:\\Users\\Pieter\\Videos\\Auto-clipEliminations.mp4";

    let (bundle, graph) = load_tensorflow_model(model_dir)?;

    let trigger_thread = TriggerThread::spawn();
    let async_trigger_sender = trigger_thread.tx.clone();

    let trigger = TensorflowTrigger {
        bundle,
        graph,
        crop: crop,
        callback: Some(Arc::new(move |result| {
            let prediction = &result.prediction;
        })),
    };

    let async_trigger = AsyncTrigger::from_trigger(trigger, async_trigger_sender);

    // Start the job
    Hypetrigger::new()
        .set_input(input_file.to_string())
        .set_fps(1)
        .add_trigger(async_trigger)
        .run()?;

    trigger_thread.stop()?;

    Ok(())
}