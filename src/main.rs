use sqlite::{State, Connection};
use text_io::read;

// use gtk::prelude::*;
// use gtk::{Application, ApplicationWindow, Button};

mod model;
use crate::model::*;

fn main() {
    let db = sqlite::open("/home/george/Documents/progs/musiclistdb/musiclist.db").unwrap();

    menu(&db);

    println!("bye");


}

fn set_uid(db: &Connection) -> usize {
    let mut users = Vec::new();

    let mut statement = db
        .prepare("SELECT * FROM USERS")
        .unwrap();
    
    while let State::Row = statement.next().unwrap() {
        let uid = statement.read::<i64>(0).unwrap();
        let name = statement.read::<String>(1).unwrap();
        
        users.push((uid, name));
    }

    println!("Choose user:");
    for (uid, name) in users {
        println!("{}) {}", uid, name);
    }

    let uid: usize = read!(); 

    uid
}

fn menu(db: &Connection) {
    let mut uid = set_uid(db);
    println!("{}", uid);
    
    loop {
        println!("\n\nChoose an option:");
        println!("1) Add Song");
        println!("2) Add Snippit to Song");
        println!("3) List All Songs");
        println!("4) Find Song By");
        println!("5) Remove Song");
        println!("6) Switch User");
        println!("7) Quit");

        let option: usize = read!();

        match option {
            1 => add_song(db, uid),
            
            2 => add_song_snippit(db, uid),
            
            3 => list_all_songs(db, uid),
            
            4 => find_song_by(db, uid),
            
            5 => remove_song(db, uid),
            
            6 => uid = set_uid(db),
            
            7 => break,
            
            _ => println!("Please enter a correct option")
        }

    }
}

fn add_song(db: &Connection, uid: usize) {

}

fn add_song_snippit(db: &Connection, uid: usize) {

}

fn list_all_songs(db: &Connection, uid: usize) {
    let mut statement = db
        .prepare("SELECT * FROM SONGS WHERE user_id = :uid;")
        .unwrap();

    statement.bind_by_name(":uid", uid as i64).unwrap();

    while let State::Row = statement.next().unwrap() {
        println!("Song ID: {} ", statement.read::<i64>(0).unwrap());
        println!("USER ID: {} ", statement.read::<i64>(1).unwrap());
        println!("Title: {} ", statement.read::<String>(2).unwrap());
        println!("Artist: {} ", statement.read::<String>(3).unwrap());
        
        println!("Link: {} ", statement.read::<String>(4).unwrap());
    }
}

fn find_song_by(db: &Connection, uid: usize) {

}

fn remove_song(db: &Connection, uid: usize) {

}


#[test]
fn songs() {
    let db = sqlite::open("/home/george/Documents/progs/musiclistdb/musiclist.db").unwrap();
    let song = Song::get(&db, 1);
    if let Some(song) = song {
        song.show();
    }

}