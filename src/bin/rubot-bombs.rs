use rubot::config::Config;
use rubot::robot::Block;
use rubot::robot::RobotBombs;

use std::fs;
use std::io::Read;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Rubot-bombs")
        .version("1.0")
        .author("Lucas F. <lucas.fenart@protonmail.com>")
        .about("Tetris robot for bomb handicap")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let config_path = matches.value_of("config").unwrap();
    let config: &'static Config = {
        let mut file = match fs::File::open(config_path) {
            Ok(file) => file,
            Err(err) => panic!("Could not open {}: {:?}", config_path, err),
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => {}
            Err(err) => panic!("Error while reading {}: {:?}", config_path, err),
        };
        let config: Config = match serde_json::from_str(&data) {
            Ok(x) => x,
            Err(err) => panic!("Error while unmarshalling {}: {:?}", config_path, err),
        };
        Box::leak(Box::new(config))
    };
    // load the list of actions in memory
    let _ = rubot::robot::Action::get_list(Block::I, Block::I);
    let mut robot: Option<RobotBombs> = None;
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        match iter.next() {
            None => {} // ignore empty line
            Some("go") => {
                play(robot.as_mut().expect("Not ready, use newgame first"));
                robot = None;
                println!("game ended");
            }
            Some("isready") => match robot {
                None => println!("nok"),
                Some(_) => println!("readyok"),
            },
            Some("newgame") => {
                let blocks: Vec<Block> = iter
                    .next()
                    .expect("Expected block list")
                    .as_bytes()
                    .iter()
                    .map(|&x| Block::from_byte(x).expect("Invalid block name"))
                    .collect();
                robot = Some(RobotBombs::new(&blocks, config));
            }
            Some("print") => println!("{:x?}", robot),
            Some("exit") | Some("quit") => break,
            Some(_) => println!("invalid command"),
        }
    }

    unsafe {
        Box::from_raw(config as *const Config as *mut Config);
    }
}

fn play(robot: &mut RobotBombs) {
    let mut buffer = String::new();
    loop {
        let action = robot.next_action();
        println!(
            "{} {} {} {}",
            if action.hold { 1 } else { 0 },
            action.rotation,
            action.translation,
            action.spin
        );
        robot.play(action);
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        match iter.next().expect("No empty line allowed") {
            "0" => {}
            "1" => robot.ko(),
            "q" => break,
            _ => panic!("Should be 0, 1 or q"),
        }
        for &block in iter.next().expect("Expected block list").as_bytes() {
            robot.add_block(Block::from_byte(block).expect("Invalid block name"));
        }
        robot.add_handicap(
            &iter
                .map(|x| {
                    let column = x
                        .parse()
                        .expect("Column index should be an integer in [3, 12]");
                    if column < 3 || column > 12 {
                        panic!("Column index should be an integer in [3, 12]")
                    } else {
                        column
                    }
                })
                .collect::<Vec<u8>>(),
        );
    }
}
