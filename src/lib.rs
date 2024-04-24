#![no_std]

use gstd::{};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    // Initialize the game
    let init_data = msg::load::<PebblesInit>();
    let difficulty = init_data.difficulty;
    let pebbles_count = init_data.pebbles_count;
    let max_pebbles_per_turn = init_data.max_pebbles_per_turn;
    let first_player = if exec::random(0) % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };
    unsafe {
        PEBBLES_GAME = Some(GameState {
            pebbles_count,
            max_pebbles_per_turn,
            pebbles_remaining: pebbles_count,
            difficulty,
            first_player,
            winner: None,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    // Handle player actions
    let action = msg::load::<PebblesAction>();
    let mut game_state = unsafe { PEBBLES_GAME.take().unwrap() };
    match action {
        PebblesAction::Turn(pebbles) => {
            if pebbles >= 1 && pebbles <= game_state.max_pebbles_per_turn && pebbles <= game_state.pebbles_remaining {
                game_state.pebbles_remaining -= pebbles;
                if game_state.pebbles_remaining == 0 {
                    game_state.winner = Some(Player::User);
                }
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            game_state.difficulty = difficulty;
            game_state.pebbles_count = pebbles_count;
            game_state.max_pebbles_per_turn = max_pebbles_per_turn;
            game_state.pebbles_remaining = pebbles_count;
            game_state.first_player = if exec::random(0) % 2 == 0 {
                Player::User
            } else {
                Player::Program
            };
            game_state.winner = None;
        }
    }
    unsafe {
        PEBBLES_GAME = Some(game_state);
    }
}

#[no_mangle]
extern "C" fn state() {
    // Reply with the current game state
    if let Some(game_state) = unsafe { PEBBLES_GAME.as_ref() } {
        msg::reply(game_state.clone());
    }
}
