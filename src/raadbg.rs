use std::sync::Mutex;

static INTER_LOG: Mutex<String> = Mutex::new( String::new() );

pub mod log {
    pub fn simple(msg: &str){
        super::add_log_line( format!( "> {msg}") );
    }
    pub fn create(name: &str){
        super::add_log_line( format!( "[+ {name}]") );
    }
    pub fn drop(name: &str){
    }
    pub fn info(name: &str, info: &str){
    }
    pub fn error(name: &str, error: &str){
    }
    
    pub fn get() -> String {
        let res = super::INTER_LOG.lock().unwrap();
        res.clone()
    }
}

fn add_log_line(line: String) {
    let mut log = INTER_LOG.lock().unwrap();
    *log += &line;
    println!("{line}");
}

/*pub fn log(status: Status){
    let newItem: String = match status {
        Simple(msg) =>       format!("> {msg}"),
        New(name) =>         format!("[+ {name}]"),
        Drop(name) =>        format!("[- {name}]"),
        Info(name, info) => format!("[  {name}]: {info}"),
        Error(name, error) => format!("[E {name}]: {error}"),
    }
    let mut log = INTER_LOG.lock().unwrap();
    *log = format!( "\n{}\n{}",log, newItem );
}

pub enum Status {
    Simple(&str),
    New(&str),
    Drop(&str),
    Info(&str, &str),
    Error(&str, &str),
}
*/
