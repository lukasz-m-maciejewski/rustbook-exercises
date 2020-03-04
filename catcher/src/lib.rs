use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_cashes_single_value() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);
    }

    #[test]
    fn successfully_two_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let v1 = c.value(2);
        assert_eq!(v1, 2);
    }
}
