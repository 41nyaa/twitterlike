pub mod client;

use iced::{
    Sandbox, Element, Row, Length, scrollable,
    Scrollable, TextInput, text_input
};
use client::{Tweet};

pub struct MainWindow {
    pub tweets : Vec<Tweet>,
    pub input: text_input::State,
    pub input_value: String,
    scrollable: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    AddTweet
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        MainWindow {
            tweets: Vec::new(),
            input: text_input::State::new(),
            input_value: String::from(""),
            scrollable: scrollable::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AddTweet => {
                if !self.input_value.is_empty() {
                    let new_tweet = Tweet{name: String::from("tweet"), value: self.input_value.clone()};
                    self.tweets = client::post_tweet(new_tweet);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let text_input = TextInput::new(
            &mut self.input,
            "tweet?",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::AddTweet);
            
        let scrollable = Scrollable::new(&mut self.scrollable)
            .width(Length::Units(500));

        Row::new()
            .spacing(20)
            .padding(20)
            .push(text_input)
            .push(scrollable)
            .into()

        // Container::new(content)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .center_x()
        //     .center_y()
        //     .into()
    }
}
