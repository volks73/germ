use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

const TEST_SHELL: &str = "/bin/sh";
const TEST_TERM: &str = "xterm-256color";
const HELLO_WORLD_ASCIICAST_OUTPUT: &str = r#"{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
[0.0,"o","$ "]
[0.75,"o","e"]
[0.785,"o","c"]
[0.82,"o","h"]
[0.855,"o","o"]
[0.89,"o"," "]
[0.925,"o","H"]
[0.96,"o","e"]
[0.995,"o","l"]
[1.03,"o","l"]
[1.065,"o","o"]
[1.1,"o"," "]
[1.135,"o","W"]
[1.17,"o","o"]
[1.205,"o","r"]
[1.24,"o","l"]
[1.275,"o","d"]
[2.16,"o","\r\n"]
[2.16,"o","Hello World\r\n"]
[3.16,"o",""]
"#;
const HELLO_WORLD_GERM_OUTPUT: &str = r#"{"version":1,"timings":{"begin":0.0,"end":1.0,"type_start":750,"type_char":35,"type_submit":350,"output_line":500,"speed":1.0},"commands":[{"prompt":"$ ","input":"echo Hello World","outputs":["Hello World\n"]}]}"#;

fn test_cmd() -> Command {
    let mut cmd = Command::cargo_bin("germ").unwrap();
    cmd.env("SHELL", TEST_SHELL).env("TERM", TEST_TERM);
    cmd
}

#[test]
fn input_arg_only_works() {
    let mut cmd = test_cmd();
    let assert = cmd.arg("echo Hello World").assert();
    assert.success().stdout(HELLO_WORLD_ASCIICAST_OUTPUT);
}

#[test]
fn input_arg_with_one_outputs_arg_works() {
    let mut cmd = test_cmd();
    let assert = cmd.args(&["echo Hello World", "Hello World"]).assert();
    assert.success().stdout(HELLO_WORLD_ASCIICAST_OUTPUT);
}

#[test]
fn output_file_works() {
    let tmp_dir = TempDir::new().unwrap();
    let output_file = tmp_dir.child("test.cast");
    let mut cmd = test_cmd();
    let assert = cmd
        .arg("-o")
        .arg(output_file.path())
        .args(&["echo Hello World", "Hello World"])
        .assert();
    assert.success();
    output_file.assert(HELLO_WORLD_ASCIICAST_OUTPUT);
}

#[test]
fn germ_output_format_works() {
    let mut cmd = test_cmd();
    let assert = cmd
        .arg("--output-format")
        .arg("germ")
        .arg("echo Hello World")
        .assert();
    assert.success().stdout(HELLO_WORLD_GERM_OUTPUT);
}

#[test]
fn germ_output_format_shortcut_works() {
    let mut cmd = test_cmd();
    let assert = cmd.arg("-G").arg("echo Hello World").assert();
    assert.success().stdout(HELLO_WORLD_GERM_OUTPUT);
}

#[test]
fn germ_timestamp_works() {
    let mut cmd = test_cmd();
    let assert = cmd
        .arg("--timestamp")
        .arg("123456789")
        .arg("echo Hello World")
        .assert();
    assert
        .success()
        .stdout(predicate::str::contains(r#""timestamp":123456789"#));
}

#[test]
fn germ_timestamp_now_works() {
    let mut cmd = test_cmd();
    let assert = cmd
        .arg("--timestamp")
        .arg("now")
        .arg("echo Hello World")
        .assert();
    assert
        .success()
        .stdout(predicate::str::contains("timestamp"));
}
