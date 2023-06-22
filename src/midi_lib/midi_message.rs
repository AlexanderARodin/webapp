

pub enum MidiMessage {
  General( i32, i32, i32, i32 ), // channel, command, data1, data2
  NoteOn( i32, i32, i32 ), // channel, 0x90, key, velocity
  NoteOff( i32, i32, i32 ), // channel, 0x80, key, velocity
}

impl MidiMessage {

    pub fn to_general(&self) -> Self {
        match self {
            Self::General( channel, command, data1, data2 ) => {
                Self::General( *channel, *command, *data1, *data2 )
            },
            Self::NoteOn( channel, key, velocity ) => {
                Self::General( *channel, 0x90, *key, *velocity )
            },
            Self::NoteOff( channel, key, velocity) => {
                Self::General( *channel, 0x90, *key, *velocity )
            },
        }
    }

    pub fn get_parsed(&self) -> Self {
        match self {
            Self::General( channel, command, data1, data2 ) => {
                match command {
                    0x80 => {
                        Self::NoteOff( *channel, *data1, *data2 )
                    },
                    0x90 => {
                        Self::NoteOn(  *channel, *data1, *data2 )
                    },
                    _ => {
                        Self::General( *channel, *command, *data1, *data2 )
                    }
                }
            },
            Self::NoteOn( channel, key, velocity ) => {
                Self::NoteOn( *channel, *key, *velocity )
            },
            Self::NoteOff( channel, key, velocity) => {
                Self::NoteOff( *channel, *key, *velocity)
            },
        }
    }

}

impl Clone for MidiMessage {
    fn clone(&self) -> Self {
        let general_copy = self.to_general();
        general_copy.get_parsed()
    }
}


//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod test{
    use super::MidiMessage;

    #[test]
    fn note_on_2general() {
        let src_midi_msg = MidiMessage::NoteOn( 1, 2, 3 );
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( channel, command, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( command == 0x90, "wrong command" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn note_off_2general() {
        let src_midi_msg = MidiMessage::NoteOff( 1, 2, 3);
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( channel, command, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( command == 0x90, "wrong command" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn general_2general() {
        let src_midi_msg = MidiMessage::General( 1, 2, 3, 4);
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( channel, command, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( command == 2, "wrong command" );
                assert!( data1 == 3, "wrong key" );
                assert!( data2 == 4, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }

    #[test]
    fn parse_note_on() {
        let src_midi_msg = MidiMessage::NoteOn( 1, 2, 3);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOn( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_note_off() {
        let src_midi_msg = MidiMessage::NoteOff( 1, 2, 3 );
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOff( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_general_2note_off() {
        let src_midi_msg = MidiMessage::General( 1, 0x80, 3, 4);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOff( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 3, "wrong key" );
                assert!( data2 == 4, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_general_2note_on() {
        let src_midi_msg = MidiMessage::General( 1, 0x90, 3, 4);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOn( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 3, "wrong key" );
                assert!( data2 == 4, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }

}

