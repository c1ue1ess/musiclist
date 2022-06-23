use std::io::{self, Write};

use text_io::read;

use crate::musiclist::{MusicList, Song, Snippit};

pub struct Cli<'a> {
    ml: &'a mut MusicList
}

impl Cli<'_> {
    pub fn new(ml: &mut MusicList) -> Cli {
        Cli { ml }
    }
    
    pub fn menu(&mut self) {
        let stdin = std::io::stdin();
        
        loop {
            let mut line = String::new();
            
            print!(">> ");
            io::stdout().flush().unwrap();
            stdin.read_line(&mut line).unwrap();

            let split: Vec<&str> = line.trim().split(' ').collect();

            match split[0] {
                "add" => self.add(&split),
                "list" => self.list(&split),
                "select" => self.select(&split),
                "edit" => self.edit(&split),
                "remove" => self.remove(&split),
                "help" => Cli::help(),
                "quit" | "exit" | "q" => break,
                "" => continue,
                _ => println!("Unrecognised command, type \"help\" for help")
            }
        }
    }

    fn help() {
        println!(  "Commands are:
    add
    list
    select
    edit
    remove
    Usage is <command> <user/song/snippit>
                    
    Other:
    help
    quit");
    }

    fn add(&mut self, split: &Vec<&str>) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "user" =>   self.add_user(),
                    "song" =>   self.add_song(),
                    "snippit" => self.add_snippit(),
                    _ => println!("Usage: add <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }

    fn add_user(&mut self) {
        println!("Enter Username:");
        let username: String = read!("{}\n");

        // if username exists loop and ask for a different username
        match self.ml.add_user(username) {
            Ok(()) => return,
            Err(error) => println!("{}", error)
        }
    }

    fn add_song(&mut self) {
        match self.ml.get_curr_user() {
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

                if let Err(err) = user.add_song(song) {
                    println!("{err}")
                }
            }
                
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }         
    }

    fn add_snippit(&mut self) {
        match self.ml.get_curr_song() {
            Ok(song) => {
                let start;
                loop {

                    println!("Enter snippit start (in seconds):");
                    let start_str: String = read!("{}\n");
                    
                    if let Ok(num) = start_str.parse::<u32>() {
                        start = num;
                        break;
                    } 

                    println!("Invalid format. Enter a number");
                }
                

                let end;
                loop {

                    println!("Enter snippit end (in seconds):");
                    let end_str: String = read!();
                
                    if let Ok(num) = end_str.parse::<u32>() {
                        end = num;
                        break;
                    }

                    println!("Invalid format. Enter a number");
                }

                println!("Enter Comment:");
                let comment = read!("{}\n");

                println!("Do you want to add a theme? [y/n]");
                let mut add_theme: String = read!();
                let mut themes = Vec::new();
                while add_theme == "y" {

                    println!("Enter theme:");
                    let theme: String = read!();
                    themes.push(theme);

                    println!("Do you want to add another theme? [y/n]");
                    add_theme = read!();
                }

                song.add_snippit(start, end, comment, themes)
            }
            
            Err(e) => {
                println!("{:?}", e);
            }
        } 
    }

    fn list(&mut self, split: &Vec<&str>) {
        match split.get(1) {
            
            Some(arg) => {
                match *arg {
                    "users" => self.list_users(),
                    "songs" => self.list_songs(),
                    "snippits" => self.list_snippits(),
                    _ => println!("Usage: list <users/songs/snippits>"),
                }
            }
            
            None => println!("Usage: add <user/song/snippit>")
        }
    }

    fn list_users(&self) {
        self.ml.get_all_users().iter()
            .for_each(|user| println!("USER: {}", user.get_username()));
    }   

    fn show_song(song: &Song) {
        print!("TITLE: {}\tARTIST: {}", 
            song.get_title(), song.get_artist());
        
        print!("\tGENRES: ");

        for genre in song.get_genres() {
            print!("{} ", genre);
        }

        println!("\tLINK: {}", song.get_link());
    }

    fn list_songs(&mut self) {
        match self.ml.get_curr_user() {
            Ok(user) => user.get_all_songs().iter()
                            .for_each(|song| Cli::show_song(song)),
        
            Err(err) => println!("{err}")
        }
    }   

    fn show_snippit(snippit: &Snippit) {
        print!("ID: {}\tSTART: {}\tEND: {}", snippit.get_id(), snippit.get_start(), snippit.get_end());
        
        print!("\tTHEMES:");
        for theme in snippit.get_themes() {
            print!("{theme} ");
        }

        println!("\tCOMMENT: {}", snippit.get_comment());
    }

    fn list_snippits(&mut self) { 
        match self.ml.get_curr_song() {
            Ok(song) => song.get_all_snippits().iter()
                            .for_each(|snippit| Cli::show_snippit(snippit)),
            
            Err(err) => println!("{err}")
        }
    }   

    
    fn select(&mut self, split: &Vec<&str>) {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "user" => self.select_user(),
                    "song" => self.select_song(),
                    "snippit" => self.select_snippit(),
                    _ => println!("Usage: select <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: select <user/song/snippit>")
        }
    }

    fn select_user(&mut self) {
        println!("Enter Username:");
        let username: String = read!("{}\n");

        if let Err(err) = self.ml.set_curr_user(&username) {
            println!("{err}");
        }
    }
    
    fn select_song(&mut self) {
        println!("Enter Song title:");
        let title: String = read!("{}\n");

        if let Err(err) = self.ml.set_curr_song(&title) {
            println!("{err}");
        }
    }
    
    fn select_snippit(&mut self) {
        println!("Enter the index of the snippit (leave empty to exit):");
        let snip_idx_str: String = read!("{}\n");

        match snip_idx_str.parse::<usize>() {
            Ok(idx) => {
                if let Err(err) = self.ml.set_curr_snippit(idx) {
                    println!("{err}");
                }
            }

            Err(_) => println!("Please enter a number.")
        }
    }
    
    fn edit(&mut self, split: &Vec<&str>,) {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "user" => self.edit_user(),
                    "song" => self.edit_song(),
                    "snippit" => self.edit_snippit(),
                    _ => println!("Usage: edit <user/song/snippit>"),
                }
            }
            
            None => println!("Usage: edit <user/song/snippit>")
        }
    }

    fn edit_user(&mut self) {
        let mut song = match self.ml.get_curr_user() {
            Ok(song) => song,
            
            Err(e) => {
                println!("{e}");
                return;
            }
        };

    }

    fn edit_song(&mut self) {

    }

    fn edit_snippit(&mut self) {

    }


    

    fn remove(&mut self, split: &Vec<&str>) {
        
        
        match split.get(1) {
            Some(arg) => {
                println!("Are you sure you want to remove {arg}? [y/n]");

                let confirm: String = read!("{}\n");
                
                if confirm == "y" {   
                    let res = match *arg {
                        "user" => {
                            self.ml.remove_curr_user()
                        }
                        "song" => {
                            self.ml.remove_curr_song()
                        }
                        "snippit" => {
                            self.ml.remove_curr_snippit()
                        }
                        _ => Err(String::from("Usage: remove <user/song/snippit>")),
                    };

                    if let Err(e) = res {
                        println!("{e}")
                    }
                }
            }
            
            None => println!("Usage: remove <user/song/snippit>")
        }
    }
}