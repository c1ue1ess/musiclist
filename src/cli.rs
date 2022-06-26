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

            if let Some(cmd) = split.get(0) {    
                if *cmd == "help" {
                    Cli::help();
                    continue;
                } else if *cmd == "quit" || *cmd == "exit" || *cmd == "q" {
                    break;
                } else if *cmd == "" {
                    continue;
                }

                let mut action = |cmd: &str| -> Result<(), String> {
                    match cmd {
                        "add" => self.add(&split),
                        "list" => self.list(&split),
                        "find" => self.find(&split),
                        "select" => self.select(&split),
                        "edit" => self.edit(&split),
                        "remove" => self.remove(&split),
                        _ => Err("Unrecognised command, type \"help\" for help".to_string())
                    }
                };

                if let Err(err) = action(*cmd) {
                    println!("{err}");
                }
            }
        }
    }

    fn help() {
        println!(  "Commands are:
    add <user/song/snippit>
    list <users/songs/snippits>
    select <user/song/snippit>
    edit <user/song/snippit>
    remove <user/song/snippit>
    find <artist/genres/themes>
    
    help
    quit");
    }

    fn add(&mut self, split: &Vec<&str>) -> Result<(), String> {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "user" =>   self.add_user(),
                    "song" =>   self.add_song(),
                    "snippit" => self.add_snippit(),
                    _ => Err("Usage: add <user/song/snippit>".to_string()),
                }
            }
            
            None => Err("Usage: add <user/song/snippit>".to_string())
        }
    }

    fn add_user(&mut self) -> Result<(), String> {
        println!("Enter Username:");
        let username: String = read!("{}\n");
        self.ml.add_user(username)
            
    }

    fn add_song(&mut self) -> Result<(), String> {
        let user = self.ml.get_curr_user()?;

        println!("Enter title: ");
        let title: String = read!();
        
        println!("Enter artist: ");
        let artist: String = read!();

        println!("Enter link:");
        let link: String = read!();

        let mut genres = Vec::new();
        println!("Do you want to add a genre? [y/n]");
        let mut add_genre: String = read!();
        while add_genre == "y" {
            println!("Enter genre:");
            let genre: String = read!();
            genres.push(genre);

            println!("Do you want to add another genre? [y/n]");
            add_genre = read!();
        }

        user.add_song(title, artist, link, genres)         
    }

    fn add_snippit(&mut self) -> Result<(), String> {
        let song = self.ml.get_curr_song()?; 
        
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

    fn list(&mut self, split: &Vec<&str>) -> Result<(), String>{
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "users" => self.list_users(),
                    "songs" => self.list_songs(),
                    "snippits" => self.list_snippits(),
                    _ => Err("Usage: list <users/songs/snippits>".to_string()),
                }
            }
            
            None => Err("Usage: list <users/songs/snippits>".to_string())
        }
    }

    fn list_users(&self) -> Result<(), String>{
        self.ml.get_all_users().iter()
            .for_each(|user| println!("USER: {}", user.get_username()));
        
        Ok(())
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

    fn list_songs(&mut self) -> Result<(), String> {
        self.ml.get_curr_user()?
            .get_all_songs().iter()
            .for_each(|song| Cli::show_song(song));

        Ok(())
    }   

    fn show_snippit(snippit: &Snippit) {
        print!("ID: {}\tSTART: {}\tEND: {}", 
            snippit.get_id(), 
            snippit.get_start(), 
            snippit.get_end());
        
        print!("\tTHEMES:");
        for theme in snippit.get_themes() {
            print!("{theme} ");
        }

        println!("\tCOMMENT: {}", snippit.get_comment());
    }

    fn list_snippits(&mut self) -> Result<(), String> { 
        self.ml.get_curr_song()?
            .get_all_snippits().iter()
            .for_each(|snippit| Cli::show_snippit(snippit));

        Ok(())
    }   

    //     find <username/title/artist/genres/themes>
    fn find(&mut self, split: &Vec<&str>) -> Result<(), String> {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "artist" => self.find_by_artist(),
                    "genres" => self.find_by_genres(),
                    "themes" => self.find_by_themes(),
                    _ => Err("Usage: find <artist/genres/themes>".to_string())
                }
            }

            None => Err("Usage: find <artist/genres/themes>".to_string())
        }
    }  

    fn find_by_artist(&mut self) -> Result<(), String> {
        println!("Enter the artist you want to search for:");
        let artist: String = read!("{}\n");

        self.ml.find_songs_by_artist(artist)?.iter()
            .for_each(|song| Cli::show_song(song));
        
        Ok(())
    }

    fn find_by_genres(&mut self) -> Result<(), String> {
        let mut genres = Vec::new();
        let mut add_genre = String::from("y");
        while add_genre == "y" {
            println!("Enter the genre you want to search for:");
            let genre: String = read!();
            genres.push(genre);

            println!("Do you want to add another genre? [y/n]");
            add_genre = read!();
        }

        self.ml.find_songs_by_genres(genres)?.iter()
            .for_each(|song| Cli::show_song(song));
        
        Ok(())
    }

    fn find_by_themes(&mut self) -> Result<(), String> {
        let mut themes = Vec::new();
        let mut add_theme = String::from("y");
        while add_theme == "y" {
            println!("Enter the theme you want to search for:");
            let theme: String = read!();
            themes.push(theme);

            println!("Do you want to add another theme? [y/n]");
            add_theme = read!();
        }

        self.ml.find_songs_by_themes(themes)?.iter()
            .for_each(|song| Cli::show_song(song));
        
        Ok(())
    }   

    
    fn select(&mut self, split: &Vec<&str>) -> Result<(), String> {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "user" => self.select_user(),
                    "song" => self.select_song(),
                    "snippit" => self.select_snippit(),
                    _ => Err("Usage: select <user/song/snippit>".to_string()),
                }
            }
            
            None => Err("Usage: select <user/song/snippit>".to_string())
        }
    }

    fn select_user(&mut self) -> Result<(), String> {
        println!("Enter Username:");
        let username: String = read!("{}\n");

        self.ml.set_curr_user(&username)
    }
    
    fn select_song(&mut self) -> Result<(), String> {
        println!("Enter Song title:");
        let title: String = read!("{}\n");

        self.ml.set_curr_song(&title)
    }
    
    fn select_snippit(&mut self) -> Result<(), String> {
        println!("Enter the index of the snippit (leave empty to exit):");
        let snip_idx_str: String = read!("{}\n");

        match snip_idx_str.parse::<usize>() {
            Ok(idx) => {
                self.ml.set_curr_snippit(idx)
            }

            Err(err) => Err(format!("{err}\nPlease enter a number."))
        }
    }
    
    fn edit(&mut self, split: &Vec<&str>) -> Result<(), String> {
        match split.get(1) {
            Some(arg) => {
                match *arg {
                    "user" =>  self.edit_user(),
                    "song" =>  self.edit_song(),
                    "snippit" => self.edit_snippit(),
                    _ => Err("Usage: edit <song/snippit>".to_string()),
                }
            }
            
            None => Err("Usage: edit <song/snippit>".to_string())
        }
    }

    fn edit_user(&mut self) -> Result<(), String> {
        let user = self.ml.get_curr_user()?;
        
        println!("Current Username: {}", user.get_username());
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans != "y" {
            return Ok(())
        }

        println!("Enter new username:");
        let username: String = read!("{}\n");

        self.ml.update_user(username)

    }

    fn edit_song(&mut self) -> Result<(), String> {
        let song = self.ml.get_curr_song()?;
        let mut title = song.get_title().to_string();
        let mut artist = song.get_artist().to_string();
        let mut link = song.get_link().to_string();

        println!("Current Title: {}", title);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            println!("Enter new song title:");
            title = read!("{}\n");
            
        }

        println!("Current Artist: {}", artist);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            println!("Enter new artist:");
            artist = read!("{}\n");
        }

        println!("Current Link: {}", link);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            println!("Enter new link:");
            link = read!("{}\n");
        }

        self.ml.update_song(title, artist, link)
    }

    fn edit_snippit(&mut self) -> Result<(), String> {
        let snippit = self.ml.get_curr_snippit()?;
        let mut start = snippit.get_start();
        let mut end = snippit.get_end();
        let mut comment = snippit.get_comment().to_string();

        println!("Current Start: {}", start);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            loop {
                println!("Enter new start (in seconds):");
                let start_str: String = read!();
            
                if let Ok(num) = start_str.parse::<u32>() {
                    start = num;
                    break;
                }
    
                println!("Invalid format. Enter a number");
            }
        }

        println!("Current End: {}", end);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            loop {
                println!("Enter snippit end (in seconds):");
                let end_str: String = read!();
            
                if let Ok(num) = end_str.parse::<u32>() {
                    end = num;
                    break;
                }
    
                println!("Invalid format. Enter a number");
            }
        }

        println!("Current Comment: \"{}\"", comment);
        println!("Do you want to change it? [y/n]");
        let ans: String = read!("{}\n");
        if ans == "y" {
            println!("Enter new link:");
            comment = read!("{}\n");
        }
        
        self.ml.update_snippit(start, end, comment)
    }

    fn remove(&mut self, split: &Vec<&str>) -> Result<(), String>{
        match split.get(1) {
            Some(arg) => {
                println!("Are you sure you want to remove {arg}? [y/n]");

                let confirm: String = read!("{}\n");
                
                if confirm != "y" {   
                    return Ok(())
                }

                match *arg {
                    "user" => {
                        self.ml.remove_curr_user()
                    }
                    "song" => {
                        self.ml.remove_curr_song()
                    }
                    "snippit" => {
                        self.ml.remove_curr_snippit()
                    }
                    _ => Err("Usage: remove <user/song/snippit>".to_string()),
                }
                
            }
            
            None => Err("Usage: remove <user/song/snippit>".to_string())
        }
    }
}