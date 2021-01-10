use std::cell::RefCell;

thread_local! {
    // Event call stack
    // Last element is a currently running plugin
    pub static PLUGINS: RefCell<Vec<String>> = RefCell::new(Vec::new())
}

pub fn name(name: String) {
    PLUGINS.with(|stack| stack.borrow_mut().push(name));
}

pub fn clear() {
    PLUGINS.with(|stack| stack.borrow_mut().clear());
}

pub fn pop() {
    PLUGINS.with(|stack| {
        stack.borrow_mut().pop();
    });
}
