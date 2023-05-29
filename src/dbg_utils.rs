use std::sync::Mutex;

static interLog: Mutex<String> = Mutex::new( String::new() );
pub fn getInterLog() -> String {
    let res = interLog.lock().unwrap();
    res.clone()
}
pub fn appendInterLog(str: &str){
    let mut log = interLog.lock().unwrap();
    *log = format!( "\n{}\n{}",log,str );
}
