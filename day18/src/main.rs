pub trait Messager {
    fn sent(&self, msg: &str);
}

pub struct LimitChecker<'a, T> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitChecker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> LimitChecker<T> {
        Self {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_value = self.value as f64 / self.max as f64;

        if percentage_of_value >= 1.0 {
            self.messager.sent("Error: You are over your quota!")
        } else if percentage_of_value >= 0.9 {
            self.messager
                .sent("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_value >= 0.75 {
            self.messager
                .sent("Warning: You've used up over 75% of your quota!")
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessager {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> Self {
            Self {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn sent(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string())
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessager::new();
        let mut checker = LimitChecker::new(&mock, 100);
        checker.set_value(80);

        assert_eq!(mock.messages.borrow().len(), 1);
    }
}
