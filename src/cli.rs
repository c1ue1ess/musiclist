use text_io::read;

use crate::musiclist::{MusicList, Song, Snippit};

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
    
    pub fn menu(ml: &mut MusicList) {
        let mut line = String::new();
        let stdin = std::io::stdin();

        loop {
            print!("> ");
            stdin.read_line(&mut line).unwrap();

            let split: Vec<&str> = line.split(' ').collect();
            match split[0] {
                "add" => Cli::add(&split, ml),
                "list" =>Cli::list(&split, ml),
                "select" => Cli::select(&split, ml),
                "edit" => Cli::edit(&split, ml),
                "remove" => Cli::remove(&split, ml),
                
                "help" => Cli::help(),
                "quit" => break,
                "" => continue,
                _ => println!("Unrecognised command, type \"help\" for help")
            }
        }
    }

    fn help() {
        println!(  "Commands are:\n
                    add\n
                    list\n
                    select\n
                    edit\n
                    remove\n
                    Usage is <command> <user/song/snippit>\n\n
                    
                    Other:\n
                    help\n
                    quit");
    }

    fn add(split: &Vec<&str>, ml: &mut MusicList) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" =>   Cli::add_user(ml),
                    "song" =>   Cli::add_song(ml),
                    "snippit" => Cli::add_snippit(ml),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }

    fn add_user(ml: &mut MusicList) {
        println!("Enter Username (leave empty to exit):");
        let username: String = read!();

        if username == "" {
            return;
        }

        // if username exists loop and ask for a different username
        match ml.add_user(username) {
            Ok(()) => return,
            Err(error) => println!("{:?}", error)
        }
    }

    fn add_song(ml: &mut MusicList) {
        match ml.get_curr_user() {
            Ok(user) => {
                println!("Enter title: ");
                let title: String = read!();
                
                println!("Enter artist: ");
                let artist: String = read!();

                println!("Enter link:");
                let link: String = read!();

                let mut song = Song::new(title, artist, link);

                println!("Do you want to add a genre? [y/n]");
                let mut add_genre: String = read!();
                while add_genre == "y" {
                    println!("Enter genre:");
                    let genre: String = read!();
                    song.add_genre(genre);

                    println!("Do you want to add another genre? [y/n]");
                    add_genre = read!();
                }

                user.add_song(song);
            }
                
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }         
    }

    fn add_snippit(ml: &mut MusicList) {
        match ml.get_curr_song() {
            Ok(song) => {
                println!("Enter snippit start (in seconds):");
                let start: u32 = read!();
                
                println!("Enter snippit end (in seconds):");
                let end: u32 = read!();

                println!("Enter Comment:");
                let comment = read!("{}\n");

                let mut snippit = Snippit::new(start, end, comment);

                println!("Do you want to add a theme? [y/n]");
                let mut add_theme: String = read!();
                while add_theme == "y" {
                    println!("Enter theme:");
                    let theme: String = read!();
                    snippit.add_theme(theme);

                    println!("Do you want to add another theme? [y/n]");
                    add_theme = read!();
                }

                song.add_snippit(snippit)
            }
            
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        } 
    }

    fn list(split: &Vec<&str>, ml: &mut MusicList) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" => Cli::add_user(ml),
                    "song" => Cli::add_song(ml),
                    "snippit" => Cli::add_snippit(ml),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }

    fn list_users(ml: &MusicList) {

    }   

    fn list_songs(ml: &MusicList) {

    }   

    fn list_snippits(ml: &MusicList) {

    }   

    
    





    fn select(split: &Vec<&str>, ml: &mut MusicList) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" => Cli::add_user(ml),
                    "song" => Cli::add_song(ml),
                    "snippit" => Cli::add_snippit(ml),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }
    
    fn edit(split: &Vec<&str>, ml: &mut MusicList) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" => Cli::add_user(ml),
                    "song" => Cli::add_song(ml),
                    "snippit" => Cli::add_snippit(ml),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }
    
    fn remove(split: &Vec<&str>, ml: &mut MusicList) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" => Cli::add_user(ml),
                    "song" => Cli::add_song(ml),
                    "snippit" => Cli::add_snippit(ml),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }
}