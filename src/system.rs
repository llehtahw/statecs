extern crate procs;
pub use procs::system;

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_one() {
        #[system]
        fn test_system_gen() -> i32 {
            233
        }

        #[system]
        fn test_system_take(_: i32) {}

        // 0,1
        let e = test_system_gen(());
        assert_eq!(&(233,), &e);
        assert_eq!((), test_system_take(e));
        // 1,1
        let e = test_system_gen((666u32,));
        assert_eq!(&233, e.get() as &i32);
        assert_eq!((666u32,), test_system_take(e));
        // 2,1
        let e = test_system_gen((666u32, 789f32));
        assert_eq!(&233, e.get() as &i32);
        assert_eq!((666u32, 789f32), test_system_take(e));
    }
    #[test]
    fn test_two() {
        #[system]
        fn test_system_gen() -> (i32, u32) {
            (233, 666)
        }
        #[system]
        fn test_system_take(_: u32, _: i32) {}

        // 0,2
        let e = test_system_gen(());
        assert_eq!(&233, e.get() as &i32);
        assert_eq!(&666, e.get() as &u32);
        assert_eq!((), test_system_take(e));
        // 1,2
        let e = test_system_gen((789f32,));
        assert_eq!(&233, e.get() as &i32);
        assert_eq!(&666, e.get() as &u32);
        assert_eq!((789f32,), test_system_take(e));
        // 2,2
        let e = test_system_gen((789f32, 987f64));
        assert_eq!(&233, e.get() as &i32);
        assert_eq!(&666, e.get() as &u32);
        assert_eq!((789f32, 987f64), test_system_take(e));
    }

    #[test]
    fn test_async() {
        #[system]
        async fn test_system_gen() -> (i32, u32) {
            (233, 666)
        }
        #[system]
        async fn test_system_take(a: u32, b: i32) -> String {
            std::format!("({a}, {b})")
        }
        async fn _test() {
            let res = test_system_take(test_system_gen(()).await).await;
            let _: (String,) = res;
        }
    }
}