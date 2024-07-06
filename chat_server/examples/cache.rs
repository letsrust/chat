struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn get(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut cache = Cacher::new(|arg| arg);
    let v = cache.get(1);
    assert_eq!(v, 1);

    let mut cacher = Cacher::new(|arg| arg);
    let v = cacher.get("hello");
    assert_eq!(v, "hello");

    // Bad case: String is not Copy
    // let mut cacher = Cacher::new(|arg| arg);
    // let v = cacher.get(String::from("hello"));
    // assert_eq!(v, "hello");
}
