fn main() {
    let equal_to_x = fun();

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn fun() -> Box<dyn FnOnce(Vec<i32>) -> bool> {
    let x = vec![1, 2, 3];

    Box::new(move |z| z == x)
}
