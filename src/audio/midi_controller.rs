

trait MidiController {
    fn reset(&mut self) {}
    fn note_on(&mut self, channel: i32, key: i32, velocity: i32) {}
    fn note_off(&mut self, channel: i32, key: i32) {}
}

