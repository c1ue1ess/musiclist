use std::{collections::HashMap};

pub struct MusicList {
    curr_user: Option<String>,

    // curr song/snippit is the index of the song/snippit in their vectors
    curr_song: Option<usize>,
    curr_snippit: Option<usize>,
    
    users: HashMap<String, User>
}

impl MusicList {
    
    pub fn get_curr_user(&mut self) -> Result<&mut User, String> {
        match self.curr_user {
            Some(username) => Ok(self.users.get_mut(&username).unwrap()),
            None => Err(String::from("No current user selected!"))
        }
    }

    pub fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        match self.get_curr_user() {
            Ok(user) => {
                match self.curr_song {
                    Some(idx) => Ok(user.songs.get_mut(idx).unwrap()),
                    None => Err(String::from("No current song"))
                }
            }

            Err(e) => Err(e)
        }    
    }
    
    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
        match self.get_curr_song() {
            Ok(song) => {
                match self.curr_snippit {
                    Some(idx) => Ok(song.snippits.get_mut(idx).unwrap()),
                    None => Err(String::from("No current song"))
                }
            }

            Err(e) => Err(e)
        } 
    }



    pub fn add_user(&mut self, username: String) -> Result<(), String>{
        match self.users.insert(username, User::new(username)) {
            Some(_) => Ok(()),
            None => Err(String::from("User already exists"))
        }
    }

    fn choose_user(&mut self) {
        // loop {

            
        //     println!("Please select user (or type \"add user\" to make a new one, leave blank to exit):");
        //     for (username, _) in self.users.iter() {
        //         println!("({})", username)
        //     }
            
        //     println!("[add user]");
            
        //     let choice: String = read!("{}\n");
            
        //     if choice == "" {
        //         return;
        //     }

        //     if choice == "add user" {
        //         self.add_user();
                
        //         // call choose user again but return before the user can be set twice
        //         self.choose_user();
        //         return;
        //     }
            
        //     // if choice exists in the map set and return, otherwise loop
        //     if let Some(_) = self.users.get_mut(&choice) {
        //         self.curr_user = Some(choice);
        //         return;
        //     }
            
        // }
            
    }
}

struct User {
    username: String,
    songs: Vec<Song>
}

impl User {
    fn new(username: String) -> User {
        User { username, songs: Vec::new() }
    } 

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn edit_song(&mut self) {

    }

    fn list_all_songs(&self) {
        for song in &self.songs {
            song.show();
        }
    }

    fn find_song_by(&self) {

    }

    fn remove_song(&mut self) {

    }


}

#[derive(Debug)]
pub struct Song {
    title: String,
    artist: String,
    link: String, 
    genres: Vec<String>,
    snippits: Vec<Snippit>,
}

impl Song {
    pub fn new(title: String, artist: String, link: String) -> Song {
        Song {  title,
                artist,
                link,
                genres: Vec::new(),
                snippits: Vec::new()
        }
    }

    pub fn add_genre(&mut self, genre: String) {
        self.genres.push(genre);
    }

    pub fn add_snippit(&mut self, snippit: Snippit) {
        self.snippits.push(snippit);
    }

    pub fn show(&self) {
        println!("TITLE: {}\nARTIST: {}", 
            self.title, self.artist);
        
        println!("GENRES:");

        for genre in &self.genres {
            println!("\t{}", genre);
        }

        println!("SNIPPITS:");

        for snippit in &self.snippits {
            snippit.show();
        }

        println!("LINK: {}", self.link);
    }
}

#[derive(Debug)]
pub struct Snippit {
    start: u32,
    end: u32,
    comment: String,
    themes: Vec<String>,
}

impl Snippit {
    fn show(&self) {
        println!("\tSTART: {}\n\tEND: {}", self.start, self.end);
        
        println!("\tTHEMES:");
        for theme in &self.themes {
            println!("\t\t{}", theme);
        }

        println!("\n\tCOMMENT: {}", self.comment);
    }

    pub fn new(start: u32, end: u32, comment: String) -> Snippit {
        Snippit { start, end, comment, themes: Vec::new() }
    }

    pub fn add_theme(&mut self, theme: String) {
        self.themes.push(theme);
    }
}