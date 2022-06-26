#![allow(unused)]
use iced::{Application, executor};

use crate::musiclist::MusicList;


pub struct App {
    screens: Screens,
    ml: MusicList
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Message {

}

struct Screens {
    screens: Vec<Screen>,
    current: usize
}

enum Screen {}
