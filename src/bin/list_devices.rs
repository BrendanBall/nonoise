use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SourceController;

fn main() {
    // create handler that calls functions on playback devices and apps
    let mut handler = SourceController::create();
    let devices = handler
        .list_devices()
        .expect("Could not get list of playback devices");
    println!("Playback Devices");
    for dev in devices.clone() {
        println!(
            "[{}] {}: {}, [Volume: {}]",
            dev.index,
            dev.name.as_ref().unwrap(),
            dev.description.as_ref().unwrap(),
            dev.volume.print(),
        );
    }
}
