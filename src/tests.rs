macro_rules! impl_test {
    ($name:ident, $res:literal) => {
        #[test]
        fn $name() -> anyhow::Result<()> {
            $crate::run_file(concat!("res/tests/", stringify!($name), ".shit"))?;

            // *assert output

            Ok(())
        }
    };
}

impl_test!(binary_ops, "67");
