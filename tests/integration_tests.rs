mod common;

#[cfg(test)]
mod tests {
    use crate::common::server;

    #[test]
    fn insert_an_element() {
        let greeter = server();
        assert_eq!(2 + 2, 4);
    }
}