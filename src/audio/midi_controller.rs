

trait MidiController {
    pub fn reset(&mut self) {}
    pub fn note_on(&mut self, channel: i32, key: i32, velocity: i32) {}
    pub fn note_off(&mut self, channel: i32, key: i32) {}
}

