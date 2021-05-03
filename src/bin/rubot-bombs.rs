use rubot::config::Config;
use rubot::robot::Action;
use rubot::robot::Block;
use rubot::robot::RobotBombs;
use rubot::util;

use std::convert::TryFrom;
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
                .required(false),
        )
        .get_matches();
    let config: &'static Config = (|| {
        let config_path = match matches.value_of("config") {
            None | Some("wwc_bombs") => return Box::leak(Box::new(Config::wwc_bombs())),
            Some(x) => x,
        };
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
    })();
    // load the list of actions in memory
    let _ = Action::get_list(Block::I, Block::I);
    let mut robot: Option<RobotBombs> = None;
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        match iter.next() {
            None => {} // ignore empty line
            Some("block") => {
                let r = robot.as_mut().unwrap();
                while let Some(x) = iter.next() {
                    let block = Block::try_from(x.parse::<char>().unwrap()).unwrap();
                    r.add_block(block);
                }
            }
            Some("go") => match robot.as_ref() {
                None => println!("Create a new game first"),
                Some(r) => {
                    let action = r.next_action();
                    let next_block: char = r.next_block(action.hold).into();
                    println!(
                        "{} {} {} {} {}",
                        next_block,
                        if action.hold { 1 } else { 0 },
                        action.rotation,
                        action.translation,
                        action.spin
                    );
                }
            },
            Some("handicap") => {
                robot.as_mut().unwrap().add_handicap(
                    &iter
                        .map(|x| {
                            let column = x.parse::<u8>().unwrap();
                            if column >= 10 {
                                panic!("Column index should be < 10");
                            }
                            column
                        })
                        .collect::<Vec<u8>>(),
                );
            }
            Some("isready") => println!("readyok"),
            Some("ko") => robot.as_mut().unwrap().ko(),
            Some("newgame") => {
                let blocks: Vec<Block> = iter
                    .next()
                    .expect("Expected block list")
                    .as_bytes()
                    .iter()
                    .map(|&x| Block::try_from(x as char).expect("Invalid block name"))
                    .collect();
                robot = Some(RobotBombs::new(&blocks, config));
            }
            Some("play") => match robot.as_mut() {
                None => println!("Create a new game first"),
                Some(r) => {
                    let hold: i8 = util::read(&mut iter);
                    let rotation = util::read(&mut iter);
                    let translation = util::read(&mut iter);
                    let spin = util::try_read(&mut iter).unwrap_or(0);
                    r.play(Action {
                        hold: hold != 0,
                        rotation,
                        translation,
                        spin,
                    });
                }
            },
            Some("print") => println!("{:x?}", serde_json::to_string(&robot)),
            Some("exit") => break,
            Some(_) => println!("invalid command"),
        }
    }

    unsafe {
        Box::from_raw(config as *const Config as *mut Config);
    }
}
