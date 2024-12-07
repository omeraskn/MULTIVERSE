#[derive(Debug)]
pub struct Game {
    pub _title: String,
    pub _year: u16,
    pub popularity: f32,
}

pub fn load_games() -> Vec<Game> {
    vec![
        Game {
            _title: String::from("The Legend of Zelda: Breath of the Wild"),
            _year: 2017,
            popularity: 1.5,
        },
        Game {
            _title: String::from("Half-Life 2"),
            _year: 2004,
            popularity: 1.6,
        },
        Game {
            _title: String::from("The Witcher 3: Wild Hunt"),
            _year: 2015,
            popularity: 1.8,
        },
        Game {
            _title: String::from("Dark Souls"),
            _year: 2011,
            popularity: 1.1,
        },
        Game {
            _title: String::from("God of War"),
            _year: 2018,
            popularity: 1.25,
        },
        Game {
            _title: String::from("Super Mario Odyssey"),
            _year: 2017,
            popularity: 1.9,
        },
        Game {
            _title: String::from("Hades"),
            _year: 2020,
            popularity: 2.8,
        },
        Game {
            _title: String::from("Resident Evil 4"),
            _year: 2005,
            popularity: 0.9,
        },
        Game {
            _title: String::from("Minecraft"),
            _year: 2009,
            popularity: 2.73,
        },
        Game {
            _title: String::from("Overwatch"),
            _year: 2016,
            popularity: 1.25,
        },
        Game {
            _title: String::from("Grand Theft Auto V"),
            _year: 2013,
            popularity: 1.1,
        },
        Game {
            _title: String::from("Animal Crossing: New Horizons"),
            _year: 2020,
            popularity: 1.4,
        },
        Game {
            _title: String::from("Elden Ring"),
            _year: 2022,
            popularity: 2.98,
        },
        Game {
            _title: String::from("Bloodborne"),
            _year: 2015,
            popularity: 1.12,
        },
        Game {
            _title: String::from("Sekiro: Shadows Die Twice"),
            _year: 2019,
            popularity: 0.78,
        },
        Game {
            _title: String::from("Mass Effect 2"),
            _year: 2010,
            popularity: 1.98,
        },
    ]
}