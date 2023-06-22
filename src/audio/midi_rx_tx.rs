

pub trait MidiSender {
    fn invoke_reset(&mut self);
    fn invoke_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32);
    
    fn invoke_note_on(&mut self, channel: i32, key: i32, velocity: i32) {
        self.invoke_midi_command( channel, 0x90, key, velocity  );
    }
    fn invoke_note_off(&mut self, channel: i32, key: i32) {
        self.invoke_midi_command( channel, 0x80, key, -1 );
    }
}

