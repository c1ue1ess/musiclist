use std::{collections::HashMap};

pub struct MusicList {
    curr_user: Option<String>,
    // curr song/snippit is the index of the song/snippit in their vectors    
    users: HashMap<String, User>
}

impl MusicList {
    pub fn new() -> MusicList {
        MusicList { curr_user: None, users: HashMap::new() }
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.iter().map(|(_, u)| u).collect::<Vec<&User>>()
    }

    pub fn get_curr_user(&mut self) -> Result<&mut User, String> {
            match &self.curr_user {
                Some(username) => Ok(self.users.get_mut(username).unwrap()),
                None => Err(String::from("No current user selected!"))
            }
    }

    pub fn set_curr_user(&mut self, username: &str ) -> Result<(), String> {
        match self.users.get(username) {
            Some(_) => {
                self.curr_user = Some(String::from(username));
                Ok(())
            }
            None => Err(format!("User {username} does not exist!"))
        }
    }

    pub fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        self.get_curr_user()?
            .get_curr_song()
    }

    pub fn set_curr_song(&mut self, title: &str) -> Result<(), String> {
        self.get_curr_user()?
            .set_curr_song(title)
    }

    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
        self.get_curr_user()?
            .get_curr_song()?
            .get_curr_snippit()
    }

    pub fn set_curr_snippit(&mut self, snip_idx: usize) -> Result<(), String> {
        self.get_curr_user()?
            .get_curr_song()?
            .set_curr_snippit(snip_idx)
    }


    pub fn add_user(&mut self, username: String) -> Result<(), String>{
        match self.users.insert(username.clone(), User::new(username)) {
            None => Ok(()),
            Some(_) => Err(String::from("User already exists"))
        }
    }
}

pub struct User {
    username: String,
    songs: HashMap<String, Song>,
    curr_song: Option<String>,
}

impl User {
    fn new(username: String) -> User {
        User { username, songs: HashMap::new(), curr_song: None }
    } 

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        match &self.curr_song {
            Some(title) => return Ok(self.songs.get_mut(title).unwrap()),
            None => return Err(String::from("No current song"))
        }
    }
    
    fn set_curr_song(&mut self, title: &str) -> Result<(), String> {
        match self.songs.get(title) {
            Some(_) => {
                self.curr_song = Some(String::from(title));
                Ok(())
            }
            None => Err(format!("Song {title} does not exist!"))
        }
    }

    pub fn add_song(&mut self, song: Song) -> Result<(), String>{
        match self.songs.insert(song.title.clone(), song) {
            None => Ok(()),
            Some(_) => Err(String::from("A song by that title already exists"))
        }
    }

    fn edit_song(&mut self) {

    }

    pub fn get_all_songs(&self) -> Vec<&Song> {
        self.songs.iter().map(|(_, s)| s).collect::<Vec<&Song>>()
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

    pub fn get_title(&self) -> &str { &self.title }

    pub fn get_artist(&self) -> &str { &self.artist }

    pub fn get_genres(&self) -> &Vec<String> { &self.genres }

    pub fn get_link(&self) -> &str { &self.link }    

    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
        match self.curr_snippit {
                Some(idx) => Ok(self.snippits.get_mut(idx).unwrap()),
                None => Err(String::from("No current song"))
            }
    } 

    pub fn set_curr_snippit(&mut self, snip_idx: usize) -> Result<(), String> {
        match self.snippits.get(snip_idx) {
            Some(_) => {
                self.curr_snippit = Some(snip_idx);
                Ok(())
            }
            None => Err(format!("Snippit {snip_idx} does not exist!"))
        }
    }

    pub fn get_all_snippits(&self) -> &Vec<Snippit> {
        &self.snippits
    }
    
    pub fn add_genre(&mut self, genre: String) {
        self.genres.push(genre);
    }

    pub fn add_snippit(&mut self, snippit: Snippit) {
        self.snippits.push(snippit);
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
    pub fn new(start: u32, end: u32, comment: String) -> Snippit {
        Snippit { start, end, comment, themes: Vec::new() }
    }

    pub fn get_start(&self) -> u32 { self.start }
    
    pub fn get_end(&self) -> u32 { self.end }
    
    pub fn get_comment(&self) -> &str { &self.comment }
    
    pub fn get_themes(&self) -> &Vec<String> { &self.themes }

    pub fn add_theme(&mut self, theme: String) {
        self.themes.push(theme);
    }
}