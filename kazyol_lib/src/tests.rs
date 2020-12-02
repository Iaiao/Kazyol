mod tests {
    use crate::tracking;
    use crate::event::EventType;
    use crate::event::EventResult::Handled;

    // TODO split this test to event, data and stack tests
    #[test]
    fn test_events() {
        // it's a bit complicated to test without unsafe code
        static mut HANDLED: bool = false;
        static mut DATA: bool = false;

        let mut event = EventType::<TestEvent>::new();
        tracking::PLUGINS.with(|stack| {
            stack.borrow_mut().push("test".to_string());
        });
        event.add_handler(|event| {
            unsafe { HANDLED = true }

            if event.data == "Event data" {
                unsafe { DATA = true }
            }

            Handled
        });

        event.dispatch_event( &mut create_test_event());

        unsafe {
            assert!(HANDLED);
            assert!(DATA);
        }
    }

    fn create_test_event() -> TestEvent {
        TestEvent {
            data: "Event data".to_string()
        }
    }

    pub struct TestEvent {
        data: String
    }
}
