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

    pub fn new_filled() -> MusicList {
        let mut ml = MusicList::new();

        ml.add_user(String::from("test_user1")).unwrap();
        ml.add_user(String::from("test_user2")).unwrap();
        ml.add_user(String::from("test_user3")).unwrap();
        ml.add_user(String::from("test_user4")).unwrap();
        ml.add_user(String::from("test_user5")).unwrap();

        ml.set_curr_user("test_user1").unwrap();

        let mut song1 = Song::new(  "test_song1".to_string(), 
                                "test_artist1".to_string(), 
                                "test_link1".to_string());
        song1.add_genre("genre1".to_string());
        song1.add_genre("genre2".to_string());
        song1.add_genre("genre3".to_string());

        let song2 = Song::new(  "test_song2".to_string(), 
                                "test_artist2".to_string(), 
                                "test_link2".to_string());

        let song3 = Song::new(  "test_song3".to_string(), 
                                "test_artist3".to_string(), 
                                "test_link3".to_string());

        let song4 = Song::new(  "test_song4".to_string(), 
                                "test_artist4".to_string(), 
                                "test_link4".to_string());
        
        let song5 = Song::new(  "test_song5".to_string(), 
                                "test_artist5".to_string(), 
                                "test_link5".to_string());

        ml.get_curr_user().unwrap().add_song(song1).unwrap();
        ml.get_curr_user().unwrap().add_song(song2).unwrap();
        ml.get_curr_user().unwrap().add_song(song3).unwrap();
        ml.get_curr_user().unwrap().add_song(song4).unwrap();
        ml.get_curr_user().unwrap().add_song(song5).unwrap();

        ml.set_curr_song("test_song1").unwrap();

        ml.get_curr_song().unwrap()
            .add_snippit(1, 1, "comment1".to_string(), vec!["a".to_string()]);
        ml.get_curr_song().unwrap()
            .add_snippit(1, 1, "comment2".to_string(), vec!["b".to_string()]);
        ml.get_curr_song().unwrap()
            .add_snippit(1, 1, "comment3".to_string(), vec!["c".to_string()]);
        ml.get_curr_song().unwrap()
            .add_snippit(1, 1, "comment4".to_string(), vec!["d".to_string()]);
        ml.get_curr_song().unwrap()
            .add_snippit(1, 1, "comment5".to_string(), vec!["e".to_string()]);

        ml
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

    pub fn remove_curr_user(&mut self) -> Result<(), String> {
        let username = String::from(self.get_curr_user()?.get_username());

        self.users.remove(&username);
        self.curr_user = None;
        Ok(())
    }

    pub fn get_curr_song(&mut self) -> Result<&mut Song, String> {
        self.get_curr_user()?
            .get_curr_song()
    }

    pub fn set_curr_song(&mut self, title: &str) -> Result<(), String> {
        self.get_curr_user()?
            .set_curr_song(title)
    }

    pub fn remove_curr_song(&mut self) -> Result<(), String> {
        self.get_curr_user()?
            .remove_curr_song()
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

    pub fn remove_curr_snippit(&mut self) -> Result<(), String> {
        self.get_curr_user()?
            .get_curr_song()?
            .remove_curr_snippit()
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

    fn remove_curr_song(&mut self) -> Result<(), String> {
        let title = String::from(self.get_curr_song()?.get_title());

        self.songs.remove(&title);
        self.curr_song = None;
        Ok(())
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
    snip_counter: usize,
    snippits: HashMap<usize, Snippit>,
    curr_snippit: Option<usize>,
}

impl Song {
    pub fn new(title: String, artist: String, link: String) -> Song {
        Song {  title,
                artist,
                link,
                genres: Vec::new(),
                snip_counter: 0,
                snippits: HashMap::new(),
                curr_snippit: None
        }
    }

    pub fn get_title(&self) -> &str { &self.title }

    pub fn get_artist(&self) -> &str { &self.artist }

    pub fn get_genres(&self) -> &Vec<String> { &self.genres }

    pub fn get_link(&self) -> &str { &self.link }    

    pub fn get_curr_snippit(&mut self) -> Result<&mut Snippit, String> {
        match self.curr_snippit {
                Some(idx) => Ok(self.snippits.get_mut(&idx).unwrap()),
                None => Err(String::from("No current song"))
            }
    } 

    pub fn set_curr_snippit(&mut self, snip_idx: usize) -> Result<(), String> {
        match self.snippits.get(&snip_idx) {
            Some(_) => {
                self.curr_snippit = Some(snip_idx);
                Ok(())
            }
            None => Err(format!("Snippit {snip_idx} does not exist!"))
        }
    }

    pub fn remove_curr_snippit(&mut self) -> Result<(), String> {
        let snip_idx = self.get_curr_snippit()?.get_id();

        self.snippits.remove(&snip_idx);
        self.curr_snippit = None;
        Ok(())
    }

    pub fn get_all_snippits(&self) -> Vec<&Snippit> {
        self.snippits.iter().map(|(_, s)| s).collect::<Vec<&Snippit>>()
    }
    
    pub fn add_genre(&mut self, genre: String) {
        self.genres.push(genre);
    }

    pub fn add_snippit(
        &mut self, 
        start: u32, 
        end: u32,
        comment: String,
        themes: Vec<String>
    ) {
        let mut snippit = Snippit::new(self.snip_counter, start, end, comment);

        for theme in themes {
            snippit.add_theme(theme);
        }
        
        self.snippits.insert(self.snip_counter, snippit);
        self.snip_counter += 1;
    }
}

#[derive(Debug)]
pub struct Snippit {
    id: usize,
    start: u32,
    end: u32,
    comment: String,
    themes: Vec<String>,
}

impl Snippit {
    pub fn new(id: usize, start: u32, end: u32, comment: String) -> Snippit {
        Snippit { id, start, end, comment, themes: Vec::new() }
    }

    pub fn get_id(&self) -> usize { self.id }

    pub fn get_start(&self) -> u32 { self.start }
    
    pub fn get_end(&self) -> u32 { self.end }
    
    pub fn get_comment(&self) -> &str { &self.comment }
    
    pub fn get_themes(&self) -> &Vec<String> { &self.themes }

    pub fn add_theme(&mut self, theme: String) {
        self.themes.push(theme);
    }
}