use std::{collections::HashMap};

pub struct MusicList {
    curr_user: Option<String>,
    // curr song/snippit is the index of the song/snippit in their vectors    
    users: HashMap<String, User>
}

impl MusicList {
    
    pub fn get_curr_user(&mut self) -> Result<&mut User, String> {
        match &self.curr_user {
            Some(username) => Ok(self.users.get_mut(username).unwrap()),
            None => Err(String::from("No current user selected!"))
        }
    }

    pub fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        match self.get_curr_user() {
            Ok(user) => {
                    match user.get_curr_song() {
                        Ok(song) => return Ok(song),
                        Err(e) => return Err(e)
                    }
            }

            Err(e) => Err(e)
        }
    }

    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
        match self.get_curr_user() {
            Ok(user) => {
                    match user.get_curr_song() {
                        Ok(song) => {
                            match song.get_curr_snippit() {
                                Ok(snippit) => return Ok(snippit),
                                Err(e) => return Err(e)
                            }
                        }

                        Err(e) => return Err(e)
                    }
            }

            Err(e) => Err(e)
        }
    }



    pub fn add_user(&mut self, username: String) -> Result<(), String>{
        match self.users.insert(username.clone(), User::new(username)) {
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

pub struct User {
    username: String,
    songs: Vec<Song>,
    curr_song: Option<usize>,
}

impl User {
    fn new(username: String) -> User {
        User { username, songs: Vec::new(), curr_song: None }
    } 

    fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        match self.curr_song {
            Some(idx) => return Ok(self.songs.get_mut(idx).unwrap()),
            None => return Err(String::from("No current song"))
        }
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
    curr_snippit: Option<usize>,
}

impl Song {
    pub fn new(title: String, artist: String, link: String) -> Song {
        Song {  title,
                artist,
                link,
                genres: Vec::new(),
                snippits: Vec::new(),
                curr_snippit: None
        }
    }

    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
            match self.curr_snippit {
                Some(idx) => Ok(self.snippits.get_mut(idx).unwrap()),
                None => Err(String::from("No current song"))
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