use assert_cmd::Command;

#[test]
fn input_with_one_output() {
    let mut cmd = Command::cargo_bin("germ").unwrap();
    let assert = cmd.args(&["echo Hello World", "Hello World"]).assert();
    assert.success().stdout(r#"{"version":2,"width":80,"height":24,"env":{"SHELL":"/usr/bin/zsh","TERM":"xterm-256color"}}
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
"#);
}
