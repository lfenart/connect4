use mcts::{Mcts, MctsGame};

use connect4::{Game, State};

fn main() {
    let mut game = Game::new();
    loop {
        let mut mcts = Mcts::new(game.clone());
        mcts.search(1000000);
        let (action, score) = mcts.best_action();
        println!("{}", score);
        println!("{}", action);
        game.play(action);
        println!("{}", game);
        match game.state() {
            State::Win(player) => {
                println!("{} wins!", if player.0 == 1 { 'O' } else { 'X' });
                break;
            }
            State::Draw => {
                println!("Draw.");
                break;
            }
            State::Unfinished => (),
        }
    }
}
