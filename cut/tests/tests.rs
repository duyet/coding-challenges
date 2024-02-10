use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;

#[test]
fn test_1() {
    Command::cargo_bin("cut")
        .unwrap()
        .args(["-f2", "sample.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            indoc! {"
                f1
                1
                6
                11
                16
                21"
            }
            .trim(),
        ));
}

#[test]
fn test_2() {
    Command::cargo_bin("cut")
        .unwrap()
        .args(["-f1", "-d,", "fourchords.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            indoc! {r#"
                Song title
                "10000 Reasons (Bless the Lord)"
                "20 Good Reasons"
                "Adore You"
                "Africa"
                "#
            }
            .trim(),
        ));

    Command::cargo_bin("cut")
        .unwrap()
        .args(["-f1", "sample.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            indoc! {"
                f0
                0
                5
                10
                15
                20" }
            .trim(),
        ));
}

#[test]
fn test_3() {
    Command::cargo_bin("cut")
        .unwrap()
        .args(["-f1,2", "sample.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            indoc! {"
                f0\tf1
                0\t1
                5\t6
                10\t11
                15\t16
                20\t21
            " }
            .trim(),
        ));

    Command::cargo_bin("cut")
        .unwrap()
        .args(["-d,", "-f", "1 2", "fourchords.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            indoc! {"
                Song title,Artist
            " }
            .trim(),
        ));
}
