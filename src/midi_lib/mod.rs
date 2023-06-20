

pub enum MidiMessage {
  general( channel: i32, command: i32, data1: i32, data2: i32 ),
  note_on( channel: i32, key: i32, velocity: i32 ),
  note_off( channel: i32, key: i32 ),
}

impl MidiMessage {
  pub fn to_general(&self) -> Self {
    match self {
      general( channel: i32, command: i32, data1: i32, data2: i32 ) => {
            Self::general( channel, command, data1, data2 )
          }
  }
}

