use std::time::Duration;
use rodio::{OutputStream, Sink, dynamic_mixer};
use rodio::source::{SineWave, Source};

fn main() {
  // Construct a dynamic controller and mixer, stream_handle, and sink.
  let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  // Create four unique sources.
  let source_c = SineWave::new(261.63).take_duration(Duration::from_secs_f32(1.)).amplify(0.20);
  let source_e = SineWave::new(329.63).take_duration(Duration::from_secs_f32(1.)).amplify(0.20);
  let source_g = SineWave::new(392.0).take_duration(Duration::from_secs_f32(1.)).amplify(0.20);
  let source_a = SineWave::new(440.0).take_duration(Duration::from_secs_f32(1.)).amplify(0.20);

  // Add sources C, E, G, and A to the mixer controller.
  controller.add(source_c);
  controller.add(source_e);
  controller.add(source_g);
  controller.add(source_a);

  // Append the dynamic mixer to the sink to play a C major 6th chord.
  sink.append(mixer);

  // Sleep the thread until sink is empty.
  sink.sleep_until_end();
}