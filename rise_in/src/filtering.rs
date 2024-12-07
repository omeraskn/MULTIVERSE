use crate::repository::{self, Game};


/*
In this task, students will create a custom filtering function in Rust 
that allows filtering elements from a given collection based on a specific condition. 
The goal is to implement a beginner-friendly solution that avoids using closures to simplify the understanding of the code.
*/
trait Condition {
    fn is_popular(&mut self, popularity: f32) -> bool;
}

struct FilterCondition  {
    popularity: f32,
}

impl Condition for FilterCondition {
   fn is_popular(&mut self, popularity: f32) -> bool {
       self.popularity < popularity 
   }
}

fn custom_filtering(games: Vec<Game>, mut filter: FilterCondition) -> Vec<Game> {
    let popular_games = games
    .into_iter()
    .filter(|game| filter.is_popular(game.popularity))
    .collect();

    popular_games
}
pub fn main() {
    let games = repository::load_games();

    // Short way to do same thing
    //let popular_games: Vec<Game> = games.into_iter().filter(|g| g.popularity > 2.5).collect();

    // Create a filter condition with popularity threshold 2.5
    let filter = FilterCondition { popularity: 2.5 };

    // Filter games
    let popular_games = custom_filtering(games, filter);

    println!("Most popular games in past 20 years {:?}", popular_games);

}