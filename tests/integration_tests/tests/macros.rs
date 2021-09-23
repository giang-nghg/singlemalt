macro_rules! await_test {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}