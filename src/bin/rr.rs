use rillrate::prime::*;

fn main() {
    rillrate::install("my-app").expect("error");

    let _pulse_empty = Pulse::new(
        "app.issues.all.pulse-empty",
        Default::default(),
        PulseOpts::default(),
    );
}