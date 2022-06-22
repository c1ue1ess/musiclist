#![allow(dead_code)]

use cli::Cli;
use musiclist::MusicList;

mod cli;
mod musiclist;

fn main() {

    let mut ml = MusicList::new();
    let mut cli = Cli::new(&mut ml);

    cli.menu();

    // println!("Hello, world!");

    // let mut users = HashMap::new();
    // users.insert(String::from("test1"), User::new(String::from("test1")));
    // users.insert(String::from("test2"), User::new(String::from("test2")));
    // users.insert(String::from("test3"), User::new(String::from("test3")));

    // let mut ml = MusicList { curr_user: None, users };
    // ml.menu();

}



