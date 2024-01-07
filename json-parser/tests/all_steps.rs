use assert_cmd::Command;
use paste::paste;

macro_rules! run_test {
    ($($name:ident: $file:expr, expect: $assert:tt)*) => {
    $(

        paste! {
            #[test]
            fn [< $name _ $assert >]() {

                Command::cargo_bin("json-parser")
                    .unwrap()
                    .args([$file])
                    .assert()
                    .$assert();

            }
        }
    )*
    }
}

run_test!(test_step1: "tests/step1/valid.json", expect: success);
run_test!(test_step1: "tests/step1/invalid.json", expect: failure);

run_test!(test_step2_1: "tests/step2/valid.json", expect: success);
run_test!(test_step2_1: "tests/step2/invalid.json", expect: failure);
run_test!(test_step2_2: "tests/step2/valid2.json", expect: success);
run_test!(test_step2_2: "tests/step2/invalid2.json", expect: failure);

run_test!(test_step3: "tests/step3/valid.json", expect: success);
run_test!(test_step3: "tests/step3/invalid.json", expect: failure);

run_test!(test_step4: "tests/step4/valid.json", expect: success);
run_test!(test_step4_invalid1: "tests/step4/invalid.json", expect: failure);
run_test!(test_step4_invalid2: "tests/step4/invalid2.json", expect: failure);
