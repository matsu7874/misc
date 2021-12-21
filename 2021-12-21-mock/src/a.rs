fn main() {
    println!("automock sample");
}

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
trait Api {
    fn call_a(&self, arg_a: i32) -> i32;
    fn call_b(&self, arg_b: i32) -> i32;
    fn call_c(&self, arg_c: i32) -> i32;
}

fn call_api(api: &dyn Api, v: i32) -> i32 {
    let mut total = 0;
    for i in 0..api.call_a(v) {
        total += api.call_b(i);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mytest() {
        let mut mock = MockApi::new();
        mock.expect_call_a()
            .with(eq(5)) // 引数5で
            .times(1) // 1回呼ばれる
            .returning(|arg| arg * 2);
        mock.expect_call_b().times(10).returning(|arg| arg + 1);
        mock.expect_call_c().times(0); // 0回だけ呼ばれる
        assert_eq!(55, call_api(&mock, 5));
    }
}
