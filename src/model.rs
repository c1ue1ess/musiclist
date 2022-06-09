use sqlite::{Connection, State};

struct Song {
    sid: i64,
    uid: i64,
    title: String,
    artist: String,
    genres: Vec<String>,
    snippits: Vec<Snippit>,
    link: String, 
}

impl Song {
    fn new(
        sid: i64, 
        uid: i64, 
        title: String,
        artist: String,
        genres: Vec<String>,
        snippits: Vec<Snippit>,
        link: String
    ) -> Song {
        Song { sid, uid, title, artist, genres, snippits, link }
    }
    
    pub fn get(db: &Connection, sid: i64) -> Option<Song> {
        let mut song = None;
        
        let mut song_query = db
            .prepare("SELECT user_id, title, artist, link FROM SONGS WHERE song_id = :sid;")
            .unwrap();

        song_query.bind_by_name(":sid", sid).unwrap();

        if let State::Row = song_query.next().unwrap() {
            let uid = song_query.read::<i64>(0).unwrap();
            let title = song_query.read::<String>(1).unwrap();
            let artist = song_query.read::<String>(2).unwrap();
            let link = song_query.read::<String>(3).unwrap();
            
            let mut genres = Vec::new();

            let mut genre_query = db
                .prepare("SELECT genre FROM GENRES WHERE song_id = :sid;")
                .unwrap();
            
            genre_query.bind_by_name(":sid", sid).unwrap();

            while let State::Row = genre_query.next().unwrap() {
                genres.push(genre_query.read::<String>(0).unwrap());
            }

            let snippits = Snippit::get(db, sid);

            song = Some(Song::new(sid, uid, title, artist, genres, snippits, link));

        }

        song
    }

    // pub fn add_snippit(&mut self, )

    pub fn show(&self) {
        println!("SONG ID: {}\nUSER_ID: {}\nTITLE: {}\nARTIST: {}", 
            self.sid, self.uid, self.title, self.artist);
        
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

struct Snippit {
    snipid: i64,
    sid: i64,
    start: i64,
    end: i64,
    themes: Vec<String>,
    comment: String,
}

impl Snippit {
    fn new(
        snipid: i64,
        sid: i64,
        start: i64,
        end: i64,
        themes: Vec<String>,
        comment: String
    ) -> Snippit {
        Snippit { snipid, sid, start, end, themes, comment }
    }

    // returns all snippits the belong to a song
    pub fn get(db: &Connection, sid: i64) -> Vec<Snippit>{
        let mut snippits = Vec::new();

        let mut snippit_query = db
            .prepare("SELECT snip_id, start, end, comment FROM SNIPPITS WHERE song_id = :sid;")
            .unwrap();

        snippit_query.bind_by_name(":sid", sid as i64).unwrap();

        while let State::Row = snippit_query.next().unwrap() {
            let snipid = snippit_query.read::<i64>(0).unwrap();

            let mut theme_query = db
            .prepare("SELECT theme FROM THEMES WHERE snip_id = :snipid;")
            .unwrap();
            
            theme_query.bind_by_name(":snipid", snipid).unwrap();
            
            let mut themes = Vec::new();

            while let State::Row = theme_query.next().unwrap() {
                themes.push(theme_query.read::<String>(0).unwrap());
            }
            
            let start = snippit_query.read::<i64>(1).unwrap();
            let end = snippit_query.read::<i64>(2).unwrap();
            let comment = snippit_query.read::<String>(3).unwrap();

            snippits.push(Snippit::new(snipid, sid, start, end, themes, comment));
        }
        
        snippits
    }

    pub fn show(&self) {
        println!("\tSNIP_ID: {}\n\tSTART: {}\n\tEND: {}", self.snipid, self.start, self.end);
        
        println!("\tTHEMES:");
        for theme in &self.themes {
            println!("\t\t{}", theme);
        }

        println!("\n\tCOMMENT: {}", self.comment);
    }
}
