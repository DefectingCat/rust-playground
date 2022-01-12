use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: HashMap<i32, i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, n: i32) -> i32 {
        match self.value.get(&n) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(n);
                self.value.insert(n, v);
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|x| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        x * x
    });

    println!("value is {}", cacher.value(21));
    println!("value is {}", cacher.value(21));
    println!("value is {}", cacher.value(21));
    println!("value is {}", cacher.value(221));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Cacher;

    #[test]
    fn cache_test() {
        let mut c = Cacher::new(|x| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            x * x
        });

        let v1 = c.value(42);
        let v2 = c.value(42);
        let v3 = c.value(41);

        assert_eq!(1764, v1);
        assert_eq!(v2, v1);
        assert_ne!(v3, v1)
    }

    #[test]
    fn closure_test() {
        let a = 42;

        let equal_to_a = |b: i32| b == a;

        let b = 42;

        assert!(equal_to_a(b));

        let x = vec![1, 2, 3];
        let equal_to_x = move |b: Vec<i32>| b == x;
        println!("{:?}", x)
    }
}
