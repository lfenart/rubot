use std::io::{BufRead, BufReader, LineWriter, Write};
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

    // sets a new game, first blocks are I, J, L, O, S, T, Z
    stdin.write(b"newgame IJLOSTZ\n").unwrap();

    // reply: "readyok"
    stdin.write(b"isready\n").unwrap();
    let _ = read(&mut stdout);

    // start searching and returns the best move
    stdin.write(b"go\n").unwrap();
    let best_move = read(&mut stdout);

    // play the best move
    let mut play = String::from("play ");
    play.push_str(&best_move);
    stdin.write(play.as_bytes()).unwrap();

    // play move hold = false, rotation = 1, translation = 2, spin = 3
    stdin.write(b"play 0 1 2 3\n").unwrap();

    // add handicap line with a hole (or bomb) in column 0
    stdin.write(b"handicap 0\n").unwrap();

    // add handicap lines with a hold in columns 1 (top) and 2 (bottom)
    stdin.write(b"handicap 1 2\n").unwrap();

    // add block I
    stdin.write(b"block I\n").unwrap();

    // add blocks O and S
    stdin.write(b"block O S\n").unwrap();

    // ko, removes all handicap lines
    stdin.write(b"ko\n").unwrap();

    // print the state of the robot
    stdin.write(b"print\n").unwrap();
    let _ = read(&mut stdout);

    // exit the processus
    stdin.write(b"exit\n").unwrap();
}
