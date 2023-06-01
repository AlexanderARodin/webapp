
use crate::raadbg::log;



pub struct SimpleSynth{
    clck: f32,
}
impl Default for SimpleSynth {
    fn default() -> Self {
        Self::new( 44100, 441*2 )
    }
}
impl Drop for SimpleSynth {
    fn drop(&mut self) {
        //self.stop();
        log::drop("SimpleSynth");
    }
}
impl SimpleSynth {
    pub fn new( sample_rate: usize, block_size: usize ) -> Self {
        log::create("SimpleSynth");
        Self{ 
            clck: 0_f32
        }
    }
}

