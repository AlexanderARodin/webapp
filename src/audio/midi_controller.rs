

pub trait MidiReceiver {
    fn reset(&mut self) {}
    pub fn process_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {}
}

pub trait MidiSender {
    fn invoke_reset(&mut self) {}
    //pub fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {}
    pub fn invoke_note_on(&mut self, channel: i32, key: i32, velocity: i32)
    pub fn invoke_note_off(&mut self, channel: i32, key: i32)
}

