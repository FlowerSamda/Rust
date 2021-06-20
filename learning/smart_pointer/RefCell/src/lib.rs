// 내부 가변성에 대한 용례: mock 객체
// 테스트 더블(test double)은 테스트하는 동안 또다른 타입을 대신하여 사용되는 타입을 위한 일반적인 프로그래밍 개념
// 목(mock) 객체는 테스트 중 어떤 일이 일어났는지 기록하여 정확한 동작이 일어났음을 단언할 수 있는 테스트 더블의 특정한 타입

// RefCell<T>의 메소드 ( 두 타입 모두 Deref 구현 -> 보통의 참조자처럼 다룰 수 있다! )
// 1. borrow      ->  스마트 포인터 타입 Ref<T>를 반환
// 2. borrow_mut  ->  스마트 포인터 타입 RefMut<T>를 반환

// RefCell<T>는 Ref<T>와 RefMut<T>가 현재 몇개나 있는지 추적
// -> 빌림 규칙에 따라, 여러개의 불변 빌림 혹은 하나의 가변 빌림을 가질 수 있음! 아닐시 패닉!

pub trait Messenger {
    fn send(&self, msg: &str);  // 목 객체가 가질 필요가 있는 인터페이스
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // 내부 가변성을 위해 RefCell<T> 사용
        sent_messages: RefCell<Vec<String>>,  // 보내질 메세지를 추적하기 위한 String값의 Vec
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }  // new -> 빈 벡터
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));  // send시 보내진 메시지 구조체에 push
            // RefCell 객체이기에, 
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);  // RefCell<Vec<String>> 상의 borrow호출 -> 벡터에 대한 불변 참조자 얻기


    }
}


