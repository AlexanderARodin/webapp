

pub enum MidiMessage {
  General( i32, i32, i32, i32 ), // channel, command, data1, data2
  NoteOn( i32, i32, i32 ), // channel, 0x90, key, velocity
  NoteOff( i32, i32 ), // channel, 0x80, key, -1
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
            Self::NoteOff( channel, key) => {
                Self::General( *channel, 0x90, *key, -1 )
            },
        }
    }

    pub fn parse_general(&self) -> Self {
        match self {
            Self::General( channel, command, data1, data2 ) => {
                Self::General( *channel, *command, *data1, *data2 )
            },
            Self::NoteOn( channel, key, velocity ) => {
                Self::NoteOn( *channel, *key, *velocity )
            },
            Self::NoteOff( channel, key) => {
                Self::NoteOff( *channel, *key)
            },
        }
    }

}
