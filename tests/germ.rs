use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;

const TEST_SHELL: &str = "/bin/sh";
const TEST_TERM: &str = "xterm-256color";
const HELLO_WORLD_OUTPUT: &str = r#"{"version":2,"width":80,"height":24,"env":{"SHELL":"/bin/sh","TERM":"xterm-256color"}}
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

#[test]
fn input_arg_with_no_outputs_arg_works() {
    let mut cmd = Command::cargo_bin("germ").unwrap();
    let assert = cmd
        .env("SHELL", TEST_SHELL)
        .env("TERM", TEST_TERM)
        .arg("echo Hello World")
        .assert();
    assert.success().stdout(HELLO_WORLD_OUTPUT);
}

#[test]
fn input_arg_with_one_outputs_arg_works() {
    let mut cmd = Command::cargo_bin("germ").unwrap();
    let assert = cmd
        .env("SHELL", TEST_SHELL)
        .env("TERM", TEST_TERM)
        .args(&["echo Hello World", "Hello World"])
        .assert();
    assert.success().stdout(HELLO_WORLD_OUTPUT);
}

#[test]
fn output_file_works() {
    let tmp_dir = TempDir::new().unwrap();
    let output_file = tmp_dir.child("test.cast");
    let mut cmd = Command::cargo_bin("germ").unwrap();
    let assert = cmd
        .env("SHELL", TEST_SHELL)
        .env("TERM", TEST_TERM)
        .arg("-o")
        .arg(output_file.path())
        .args(&["echo Hello World", "Hello World"])
        .assert();
    assert.success();
    output_file.assert(HELLO_WORLD_OUTPUT);
}
