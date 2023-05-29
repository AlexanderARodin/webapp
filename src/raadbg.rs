use std::sync::Mutex;

static INTER_LOG: Mutex<String> = Mutex::new( String::new() );

pub fn getLog() -> String {
    let res = INTER_LOG.lock().unwrap();
    res.clone()
}
pub fn log(status: Status){
    let mut log = INTER_LOG.lock().unwrap();
    let newItem: String = match status {
        Simple(msg) =>       format!("> {msg}"),
        New(name) =>         format!("[+ {name}]"),
        Drop(name) =>        format!("[- {name}]"),
        Infor(name, info) => format!("[  {name}]: {info}"),
        Error(name, error) => format!("[E {name}]: {error}"),
    }
    *log = format!( "\n{}\n{}",log, newItem );
}

pub enum Status {
    Simple(String),
    New(String),
    Drop(String),
    Info(String, String),
    Error(String, String),
}
