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

struct LogGroupStack {
    stack: Vec<LogGroup>,
}

impl LogGroupStack {
    fn new() -> LogGroupStack {
        LogGroupStack { stack: Vec::new() }
    }

    fn push(&mut self, group: LogGroup) {
        self.stack.push(group);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn log(&self, message: &str) {
        if let Some(group) = self.stack.last() {
            group.log(message);
        } else {
            // If there are no active log groups, log the message directly
            println!("{}", message);
        }
    }
}
fn main() {
    let mut log_stack = LogGroupStack::new();
    log_stack.log("Outside any group");
    log_stack.push(LogGroup::new(1));
    log_stack.log("Inside group 1");
    log_stack.push(LogGroup::new(2));
    log_stack.log("Inside group 2");
    log_stack.pop();
    log_stack.log("Outside group 2, still inside group 1");
    log_stack.pop();
    log_stack.log("Outside all groups");
}
