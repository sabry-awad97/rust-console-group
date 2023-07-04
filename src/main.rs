struct LogGroup {
    indentation_level: u32,
}

impl LogGroup {
    fn new(indentation_level: u32) -> LogGroup {
        LogGroup { indentation_level }
    }

    fn log(&self, message: &str) {
        let indentation = "  ".repeat(self.indentation_level as usize);
        println!("{}{}", indentation, message);
    }
}

fn main() {
    let log_group = LogGroup::new(1);
    log_group.log("Hello world!");
}
