use std::sync::Mutex;

static INTER_LOG: Mutex<String> = Mutex::new( String::new() );

pub fn getInterLog() -> String {
    let res = INTER_LOG.lock().unwrap();
    res.clone()
}
pub fn appendInterLog(str: &str){
    let mut log = INTER_LOG.lock().unwrap();
    *log = format!( "\n{}\n{}",log,str );
}
