use super::{MidiMessage,MidiReceiver};


pub struct MidiSequence {
    current_index: usize,
    elapsed_time: f32,
    list: Vec<TimedMidiMessage>,
}

impl MidiSequence {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            current_index: 0,
            elapsed_time: 0_f32,
            list: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, delay: f32, msg: &MidiMessage) {
        let len = self.list.len();
        let prev_time:f32 = match len {
            0 => {
                0_f32
            },
            _ => {
                self.list[len - 1].time
            }
        };
        let new_value = TimedMidiMessage::new(prev_time+delay, msg.clone() );
        self.list.push( new_value );
    }
    #[allow(dead_code)]
    pub fn restart(&mut self) {
        self.current_index = 0;
        self.elapsed_time = 0_f32;
    }

    #[allow(dead_code)]
    pub fn send_next_sequence(&mut self, tick_time: f32, receiver: &mut dyn MidiReceiver) {
        self.elapsed_time += tick_time;
        for (i, tm_msg) in self.list.iter().enumerate() {
            if i < self.current_index {
                continue;
            }
            if self.elapsed_time < tm_msg.time {
                break;
            }
            let midi = tm_msg.midi_msg.to_midi_general();
            receiver.process_midi_command( midi.channel,
                                           midi.command, 
                                           midi.data1, 
                                           midi.data2 );
            self.current_index += 1;
        }
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        self.current_index >= self.list.len()
    }

}


struct TimedMidiMessage {
    time: f32,
    midi_msg: MidiMessage,
}
impl TimedMidiMessage {
    #[allow(dead_code)]
    fn new(time: f32, midi_msg: MidiMessage) -> Self {
        Self {
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
        assert!( seq.list[0].time == 0.5, "time must be 0.5");
        seq.push( 1.2, &a_note );
        assert!( seq.list.len() == 2, "len must be 2");
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
        assert!( seq.list[0].time == 0.5, "time must be 0.5");
        seq.push( 1.2, &a_note );
        assert!( seq.list.len() == 2, "len must be 2");
        assert!( seq.list[1].time == 1.7, "time must be 1.7");
        
        let mut tst_rec = ReceiverTest::new();
        assert!( tst_rec.buf.is_empty(), "must be empty");
        seq.send_next_sequence( 999999., &mut tst_rec );
        assert!( tst_rec.buf.len() == 2, "must be some content but {}", tst_rec.buf.len());
    }

    #[test]
    fn timing_1() {
        let mut seq = MidiSequence::new();
        let c_on = MidiMessage::NoteOn(1,60,80);
        let c_off = MidiMessage::NoteOff(1,60,80);
        seq.push( 0., &c_on );
        seq.push( 0.5, &c_off );
        let d_on = MidiMessage::NoteOn(1,62,80);
        let d_off = MidiMessage::NoteOff(1,62,80);
        seq.push( 0., &d_on );
        seq.push( 1., &d_off );
        
        let mut tst_rec = ReceiverTest::new();
        seq.send_next_sequence( 0., &mut tst_rec );
        assert!( tst_rec.buf.len() == 1, "A: must be some content but {}", tst_rec.buf.len());
        let midi = tst_rec.buf[0].to_midi_general();
        assert!( midi.channel == 1, "must be <c_on.ch>");
        assert!( midi.command == 0x90, "must be <c_on.cm>");
        assert!( midi.data1 == 60, "must be <c_on.dt1>");
        assert!( midi.data2 == 80, "must be <c_on.dt2>");
        
        let mut tst_re2 = ReceiverTest::new();
        seq.send_next_sequence( 0., &mut tst_re2 );
        assert!( tst_re2.buf.len() == 0, "B: must be zero but {}", tst_re2.buf.len());
        seq.send_next_sequence( 0.5, &mut tst_re2 );
        assert!( tst_re2.buf.len() == 2, "B: must be TWO but {}", tst_re2.buf.len());
        
        let mut tst_re3 = ReceiverTest::new();
        seq.send_next_sequence( 9., &mut tst_re3 );
        assert!( tst_re3.buf.len() == 1, "C: must be ONE but {}", tst_re3.buf.len());
        let mid3 = tst_re3.buf[0].to_midi_general();
        assert!( mid3.channel == 1, "must be <d_off.ch>");
        assert!( mid3.command == 0x80, "must be <d_off.cm>");
        assert!( mid3.data1 == 62, "must be <d_off.dt1>");
        assert!( mid3.data2 == 80, "must be <d_off.dt2>");
    }

}

