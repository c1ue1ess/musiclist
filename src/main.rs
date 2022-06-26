use cli::Cli;
use iced::{Application, Settings };
use musiclist::MusicList;
use app::App;

mod cli;
mod musiclist;
mod app;

fn main()  {
    let cli = true;
    
    if cli {
        let mut ml = MusicList::new_filled();
        let mut cli = Cli::new(&mut ml);
        cli.menu();
    } else {
        App::run(Settings::default()).unwrap()
    }

}