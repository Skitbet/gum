use iced::{
    border::Radius,
    executor,
    widget::{
        button::{ self, StyleSheet },
        Button,
        Column,
        Container,
        Row,
        Scrollable,
        Space,
        Text,
    },
    Alignment,
    Application,
    Background,
    Border,
    Color,
    Command,
    Element,
    Length,
    Shadow,
    Theme,
};
use crate::{ data::game::Game, platform::{ load_platforms, Platform } };

struct PlayButtonStyle;

impl iced::widget::button::StyleSheet for PlayButtonStyle {
    type Style = (); // you can ignore or remove if your iced version doesn't require this associated type

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(0.15, 0.6, 0.9))),
            shadow_offset: iced::Vector::new(0.0, 2.0),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.5, 0.8))),
            shadow_offset: iced::Vector::new(0.0, 4.0),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MyTheme {
    Light,
    Dark,
}

pub struct GameLauncher {
    platforms: Vec<Box<dyn Platform>>,
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GameListLoaded(Vec<Game>),
    LaunchGame(String, String), // (platform, app_id)
}

impl Application for GameLauncher {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = iced::theme::Theme;

    fn new(_: ()) -> (Self, Command<Message>) {
        let platforms = load_platforms();
        let mut launcher = Self { platforms, games: vec![] };

        // Load games async
        let cmd = Command::perform(
            async {
                let mut all_games = vec![];
                for plat in load_platforms() {
                    if let Ok(mut games) = plat.list_games() {
                        all_games.append(&mut games);
                    }
                }
                all_games
            },
            Message::GameListLoaded
        );

        (launcher, cmd)
    }

    fn title(&self) -> String {
        String::from("Universal Game Launcher")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GameListLoaded(games) => {
                self.games = games;
            }
            Message::LaunchGame(platform, app_id) => {
                if let Some(p) = self.platforms.iter().find(|p| p.name() == platform) {
                    let _ = p.launch_game(&app_id);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut col = Column::new().spacing(15).padding(20).width(Length::Fill);

        for game in &self.games {
            // Placeholder icon box (40x40 gray rounded rect)
            let icon = Container::new(Space::new(40, 40))
                .width(Length::from(40))
                .height(Length::from(40));

            // Game name text styled size 18, fills remaining space
            let game_name = Text::new(&game.name).size(18).width(Length::Fill);

            // Styled Play button with padding and hover effect
            let launch_btn = Button::new(Text::new("Play").size(16))
                .on_press(Message::LaunchGame(game.platform.clone(), game.app_id.clone()))
                .padding(10);
            // .style(&PlayButtonStyle);

            // Each game row: icon, name, button spaced horizontally
            let row = Row::new()
                .align_items(Alignment::Center)
                .spacing(15)
                .push(icon)
                .push(game_name)
                .push(launch_btn);

            col = col.push(row);
        }

        Scrollable::new(col).into()
    }
}
