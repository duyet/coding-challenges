use assert_cmd::Command;
use predicates::prelude::*;

macro_rules! run_test {
    ($($name:ident: $args:expr, $expected:expr)*) => {
    $(
        #[test]
        fn $name() {

            Command::cargo_bin("wc")
                .unwrap()
                .args($args)
                .assert()
                .success()
                .stdout(predicate::str::contains($expected));

        }
    )*
    }
}

run_test!(test_wc_c: ["-c", "test.txt"], "342190 test.txt");
run_test!(test_wc_l: ["-l", "test.txt"], "7145 test.txt");
run_test!(test_wc_m: ["-m", "test.txt"], "339292 test.txt");
run_test!(test_wc_w: ["-w", "test.txt"], "58164 test.txt");

// Multiple options
run_test!(test_c_l: ["-c", "-l", "test.txt"], "7145  342190 test.txt");
run_test!(test_c_m: ["-c","-m", "test.txt"], "339292 test.txt");
run_test!(test_c_w: ["-c", "-w", "test.txt"], "58164  342190 test.txt");

// Combine options
run_test!(test_cl: ["-cl", "test.txt"], "7145  342190 test.txt");
run_test!(test_cm: ["-cm", "test.txt"], "339292 test.txt");
run_test!(test_cw: ["-cw", "test.txt"], "58164  342190 test.txt");

// Swap options
run_test!(test_lc: ["-lc", "test.txt"], "7145  342190 test.txt");
run_test!(test_mc: ["-mc", "test.txt"], "339292 test.txt");
run_test!(test_wc: ["-wc", "test.txt"], "58164  342190 test.txt");

// No options
run_test!(test_no_options: ["test.txt"], "   7145   58164  342190 test.txt");
