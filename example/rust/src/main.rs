use std::io::{LineWriter, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const PATH: &'static str = "./rubot-holes";
const CONFIG: &'static str = "holes.json";

fn read(stdout: &mut impl BufRead) -> String {
    let mut buffer = String::new();
    stdout.read_line(&mut buffer).unwrap();
    println!("{:?}", buffer);
    buffer
}

fn main() {
    let rubot = Command::new(PATH)
        .arg("-c")
        .arg(CONFIG)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut stdin = LineWriter::new(rubot.stdin.unwrap());
    let mut stdout = BufReader::new(rubot.stdout.unwrap());

    // reply: "nok"
    stdin.write(b"isready\n").unwrap();
    let _ = read(&mut stdout);

    // sets a new game, first blocks are I, J, L, O, S, T, Z
    stdin.write(b"newgame IJLOSTZ\n").unwrap();

    // reply: "readyok"
    stdin.write(b"isready\n").unwrap();
    let _ = read(&mut stdout);

    // start the game
    stdin.write(b"go\n").unwrap();
    let _ = read(&mut stdout);

    // no ko, next block is I, 3 handicap lines added on columns 3, 4 and 5
    stdin.write(b"0 I 3 4 5\n").unwrap();
    let _ = read(&mut stdout);

    // ko, next block is O, no handicap line added
    stdin.write(b"1 O\n").unwrap();
    let _ = read(&mut stdout);

    // quit the game
    stdin.write(b"q\n").unwrap();
    let _ = read(&mut stdout);
    
    // exit the processus
    stdin.write(b"exit\n").unwrap();
}
