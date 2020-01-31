use std::error::Error;

pub struct MySQLPinger {}

impl MySQLPinger {
    pub fn new() -> Self {
        Self{}
    }

    pub fn ping() -> Result<(), Box<dyn Error>> {
        println!("ping");
        Ok(())
    }
}