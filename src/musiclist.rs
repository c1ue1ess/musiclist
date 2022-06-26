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

        ml.get_curr_user()
            .unwrap()
            .add_song(  "test_song1".to_string(), 
                        "test_artist1".to_string(), 
                        "test_link1".to_string(),
                        Vec::new())
            .unwrap();


        ml.get_curr_user()
            .unwrap()
            .add_song(  "test_song2".to_string(), 
                        "test_artist2".to_string(), 
                        "test_link2".to_string(),
                        Vec::new())
            .unwrap();

        ml.get_curr_user()
            .unwrap()
            .add_song(  "test_song3".to_string(), 
                        "test_artist3".to_string(), 
                        "test_link3".to_string(),
                        Vec::new())
            .unwrap();

        ml.get_curr_user()
            .unwrap()
            .add_song(  "test_song4".to_string(), 
                        "test_artist4".to_string(), 
                        "test_link4".to_string(),
                        Vec::new())
            .unwrap();
        
        ml.get_curr_user()
            .unwrap()
            .add_song(  "test_song5".to_string(), 
                        "test_artist5".to_string(), 
                        "test_link5".to_string(),
                        Vec::new())
            .unwrap();

        ml.set_curr_song("test_song2").unwrap();

        ml.get_curr_song().unwrap().add_genre("genre1".to_string());
    


        ml.set_curr_song("test_song1").unwrap();

        ml.get_curr_song().unwrap().add_genre("genre1".to_string());
        ml.get_curr_song().unwrap().add_genre("genre2".to_string());
        ml.get_curr_song().unwrap().add_genre("genre3".to_string());

        ml.get_curr_song().unwrap()
            .add_snippit(
                1, 
                1, 
                "comment1".to_string(), 
                vec!["a".to_string()])
            .unwrap();
        ml.get_curr_song().unwrap()
            .add_snippit(
                1, 
                1, 
                "comment2".to_string(), 
                vec!["b".to_string()])
            .unwrap();
        ml.get_curr_song().unwrap()
            .add_snippit(
                1, 
                1, 
                "comment3".to_string(), 
                vec!["c".to_string()])
            .unwrap();
        ml.get_curr_song().unwrap()
            .add_snippit(
                1, 
                1, 
                "comment4".to_string(), 
                vec!["d".to_string()])
            .unwrap();
        ml.get_curr_song().unwrap()
            .add_snippit(
                1, 
                1, 
                "comment5".to_string(), 
                vec!["e".to_string()])
            .unwrap();

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

    pub fn update_user(&mut self, username: String) -> Result<(), String> {
        if let None = self.curr_user {
            return Err("No current user selected!".to_string())
        }
        
        if let Some(_) = self.users.get(&username) {
            return Err("Username already exists!".to_string());
        }

        let old_username = self.curr_user.as_ref().unwrap();
        
        // reinsert the user under a new key and remove the old key
        let mut user = self.users.remove(old_username).unwrap();
        user.set_username(&username);
        
        if let Some(_) = self.users.insert(username.clone(), user) {
            panic!("Tried to insert user into occupied spot")
        }

        // set the current username to the new one
        self.curr_user = Some(username);
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

    pub fn update_song(
        &mut self, 
        title: String, 
        artist: String, 
        link: String
    ) -> Result<(), String>{
        if title != self.get_curr_song()?.get_title() {
            self.get_curr_user()?.update_curr_song_title(title)?;
        }

        if artist != self.get_curr_song()?.get_artist() {
            self.get_curr_song()?.set_artist(artist);
        }

        if link != self.get_curr_song()?.get_link() {
            self.get_curr_song()?.set_link(link);
        }
        
        Ok(())
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

    pub fn update_snippit(
        &mut self, 
        start: u32, 
        end: u32, 
        comment: String
    ) -> Result<(), String> {
        self.get_curr_snippit()?.set_start(start);
        self.get_curr_snippit()?.set_end(end);
        self.get_curr_snippit()?.set_comment(comment);

        Ok(())

    }

    pub fn add_user(&mut self, username: String) -> Result<(), String>{
        match self.users.insert(username.clone(), User::new(username)) {
            None => Ok(()),
            Some(_) => Err(String::from("User already exists"))
        }
    }

    pub fn find_songs_by_artist(&mut self, artist: String) -> Result<Vec<&Song>, String> {
        let found = self.get_curr_user()?.songs.iter()
            .map(|(_,v)| v)
            .filter(|song| song.artist == artist)
            .collect();
        
        Ok(found)
    }

    pub fn find_songs_by_genres(&mut self, genres: Vec<String>) -> Result<Vec<&Song>, String> {
        let found = self.get_curr_user()?.songs.iter()
            .map(|kv| kv.1)
            .filter(|song| song.genres.iter()
                .any(|genre| genres.contains(genre)))
            .collect();
        
        Ok(found)
    }
    
    pub fn find_songs_by_themes(&mut self, themes: Vec<String>) -> Result<Vec<&Song>, String> {
        let found = self.get_curr_user()?.songs.iter()
            .map(|kv| kv.1)
            .filter(|song| song.snippits.iter()
                .any(|(_, snippit)| snippit.themes.iter()
                    .any(|theme| themes.contains(theme))))
            .collect();
        
        Ok(found)
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

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string()
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

    pub fn update_curr_song_title(
        &mut self, 
        title: String
    ) -> Result<(), String> {
        if let None = self.curr_song {
            return Err("No current song selected!".to_string())
        }
        
        if let Some(_) = self.songs.get(&title) {
            return Err("Title already exists!".to_string());
        }

        let old_title = self.curr_song.as_ref().unwrap();
        
        // reinsert the user under a new key and remove the old key
        let mut song = self.songs.remove(old_title).unwrap();
        song.set_title(title.clone());
        if let Some(_) = self.songs.insert(title.clone(), song) {
            panic!("Tried to insert song into an occupied spot");
        }

        self.curr_song = Some(title);

        Ok(())
    }

    pub fn add_song(
        &mut self, 
        title: String,
        artist: String,
        link: String,
        genres: Vec<String>
    ) -> Result<(), String> {
        let mut song = Song::new(title.clone(), artist, link);

        for genre in genres {
            song.add_genre(genre);
        }
        
        match self.songs.insert(title, song) {
            None => Ok(()),
            Some(_) => Err(String::from("A song by that title already exists"))
        }
    }

    pub fn get_all_songs(&self) -> Vec<&Song> {
        self.songs.iter().map(|(_, s)| s).collect::<Vec<&Song>>()
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

    pub fn set_title(&mut self, title: String) { self.title = title }

    pub fn get_artist(&self) -> &str { &self.artist }

    pub fn set_artist(&mut self, artist: String) { self.artist = artist }

    pub fn get_genres(&self) -> &Vec<String> { &self.genres }

    pub fn get_link(&self) -> &str { &self.link }    

    pub fn set_link(&mut self, link: String) { self.link = link }

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
    ) -> Result<(), String>{
        let mut snippit = Snippit::new(self.snip_counter, start, end, comment);

        for theme in themes {
            snippit.add_theme(theme);
        }
        
        match self.snippits.insert(self.snip_counter, snippit) {
            None => {
                self.snip_counter += 1;
                Ok(())
            }

            Some(_) => Err("Snippit ID already exists!!!! (o no)".to_string())
        }
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

    pub fn set_start(&mut self, start: u32) { self.start = start }
    
    pub fn get_end(&self) -> u32 { self.end }

    pub fn set_end(&mut self, end: u32) { self.end = end }
    
    pub fn get_comment(&self) -> &str { &self.comment }

    pub fn set_comment(&mut self, comment: String) { self.comment = comment }
    
    pub fn get_themes(&self) -> &Vec<String> { &self.themes }

    pub fn add_theme(&mut self, theme: String) {
        self.themes.push(theme);
    }
}