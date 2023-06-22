use super::MidiMessage;
use super::MidiReceiver;


pub struct MidiSequence {
    current_index: usize,
    list: Vec<TimedMidiMessage>,
}

impl MidiSequence {
    pub fn new() -> Self {
        Self {
            current_index: 0,
            list: Vec::new()
        }
    }

    pub fn push(&mut self, delay: f32, msg: &MidiMessage) {
        let len = self.list.len();
        let prev_time:f32 = match len {
            0 => {
                0.
            },
            _ => {
                self.list[len - 1].time
            }
        };
        let new_value = TimedMidiMessage::new(delay, prev_time+delay, msg.clone() );
        self.list.push( new_value );
    }
    pub fn restart(&mut self) {
        self.current_index = 0;
    }

    pub fn send_next_sequence(&mut self, til_time: f32, receiver: &mut dyn MidiReceiver) {
        for tm_msg in &self.list {
            let midi = tm_msg.midi_msg.to_midi_general();
            receiver.process_midi_command( midi.channel,
                                           midi.command, 
                                           midi.data1, 
                                           midi.data2 );
        }
    }

}


struct TimedMidiMessage {
    delay: f32,
    time: f32,
    midi_msg: MidiMessage,
}
impl TimedMidiMessage {
    fn new(delay: f32, time: f32, midi_msg: MidiMessage) -> Self {
        Self {
            delay,
            time,
            midi_msg
        }
    }
} 


//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod test{
    use super::MidiSequence;
    use super::MidiMessage;

    #[test]
    fn create() {
        let seq = MidiSequence::new();
        assert!( seq.list.is_empty(), "is not empty");
        assert!( seq.current_index == 0, "wrong current_index");
    }
    #[test]
    fn push() {
        let mut seq = MidiSequence::new();
        let a_note = MidiMessage::NoteOn(1,2,3);
        seq.push( 0.5, &a_note );
        assert!( seq.list.len() == 1, "len must be 1");
        assert!( seq.list[0].delay == 0.5, "delay must be 0.5");
        assert!( seq.list[0].time == 0.5, "time must be 0.5");
        seq.push( 1.2, &a_note );
        assert!( seq.list.len() == 2, "len must be 2");
        assert!( seq.list[1].delay == 1.2, "delay must be 1.2");
        assert!( seq.list[1].time == 1.7, "time must be 1.7");
    }

    #[test]
    fn restart() {
        let mut seq = MidiSequence::new();
        seq.current_index = 666;
        seq.restart();
        assert!( seq.current_index == 0, "wrong current_index");
    }
}


#[cfg(test)]
mod main_test{
    use super::MidiSequence;
    use super::MidiMessage;

    struct ReceiverTest {
        buf: Vec<MidiMessage>,
    }
    impl ReceiverTest {
        fn new() -> Self {
            Self {
                buf: Vec::new()
            }
        }
    }
    impl super::MidiReceiver for ReceiverTest {
        fn reset(&mut self) {
        }
        fn process_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32) {
            let msg = MidiMessage::new( channel, command, data1, data2 );
            self.buf.push(msg);
            println!("RECEIVER_TEST: got midi message");
        }
    }
    
    #[test]
    fn send_next_sequence() {
        let mut seq = MidiSequence::new();
        let a_note = MidiMessage::NoteOn(1,2,3);
        seq.push( 0.5, &a_note );
        assert!( seq.list.len() == 1, "len must be 1");
        assert!( seq.list[0].delay == 0.5, "delay must be 0.5");
        assert!( seq.list[0].time == 0.5, "time must be 0.5");
        seq.push( 1.2, &a_note );
        assert!( seq.list.len() == 2, "len must be 2");
        assert!( seq.list[1].delay == 1.2, "delay must be 1.2");
        assert!( seq.list[1].time == 1.7, "time must be 1.7");
        
        let mut tst_rec = ReceiverTest::new();
        assert!( tst_rec.buf.is_empty(), "must be empty");
        seq.send_next_sequence( 0., &mut tst_rec );
        assert!( tst_rec.buf.len() == 2, "must be some content but {}", tst_rec.buf.len());
    }

}

