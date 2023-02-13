
    pub enum Type{
        Debug,
        Info,
        Warning,
        Error,
    }

    pub fn pp(msg:&str){
        let level = Type::Info;
        match level {
            Type::Debug => println!("Debug : {msg}"),
            Type::Info => println!("Info : {msg}"),
            Type::Warning => println!("Warning : {msg}"),
            Type::Error => println!("Error : {msg}"),
        }
    }
    