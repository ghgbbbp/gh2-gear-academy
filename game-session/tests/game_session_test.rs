use game_session_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program, System};

// Define constants: Wordle program ID, game session program ID, and user ID
const WORDLE_ID: u64 = 1;
const GAME_SESSION_ID: u64 = 2;
const USER1: u64 = 10;

fn setup() -> System {
    let sys = System::new();
    sys.init_logger();

    let wordle = Program::from_file(&sys, "../target/wasm32-unknown-unknown/debug/wordle.wasm");
    let game_session = Program::from_file(
        &sys,
        "../target/wasm32-unknown-unknown/debug/game_session.wasm",
    );
    sys.mint_to(USER1, 4500000000000000);
    // Associate USER's ActorId with Wordle program ID and send message
    let user_id: ActorId = USER1.into();
    let wordle_id: ActorId = WORDLE_ID.into();
    wordle.send(user_id, wordle_id);
    game_session.send(user_id, wordle_id);

    sys // Return system instance
}

// Test scenario where user wins the game
#[test]
fn test_win() {
    let sys = setup(); // Initialize system
    let game_session = sys.get_program(GAME_SESSION_ID).unwrap(); // Get game session program

    // User starts game and checks words (hidden word is "house" in test mode)
    game_session.send(USER1, SessionAction::StartGame);
    sys.run_next_block();
    game_session.send(
        USER1,
        SessionAction::CheckWord {
            word: "human".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        USER1,
        SessionAction::CheckWord {
            word: "house".to_string(),
        },
    );
    sys.run_next_block();

    // Read game state and verify assertions
    let state: State = game_session.read_state(b"").unwrap();
    assert_eq!(state.user_sessions[0].1.result, SessionResult::Win); // Verify user won the game
}

// Test scenario where user loses after exceeding guess limit
#[test]
fn test_lose_with_too_many_check() {
    let sys: System = setup(); // Initialize system
    let game_session = sys.get_program(GAME_SESSION_ID).unwrap(); // Get game session program

    // User starts game and makes multiple guesses
    let user: ActorId = USER1.into();
    game_session.send(user, SessionAction::StartGame);
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "jknkj".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "abcdf".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "hyuiy".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "ppppp".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "lllll".to_string(),
        },
    );
    sys.run_next_block();
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "ggggg".to_string(),
        },
    );
    sys.run_next_block();
    // Read game state and verify assertions
    let state: State = game_session.read_state(b"").unwrap();
    assert_eq!(state.user_sessions[0].1.check_count, 6); // Verify user made 6 guesses
    assert_eq!(state.user_sessions[0].1.result, SessionResult::Lose); // Verify user lost the game
}

// Test scenario where user loses due to timeout
#[test]
fn test_lose_with_timeout() {
    let sys: System = setup(); // Initialize system
    let game_session = sys.get_program(GAME_SESSION_ID).unwrap(); // Get game session program

    // User1 starts game and makes one guess (hidden word is "house" in test mode)
    let user: ActorId = USER1.into();
    game_session.send(user, SessionAction::StartGame);
    game_session.send(
        user,
        SessionAction::CheckWord {
            word: "human".to_string(),
        },
    );

    sys.run_to_block(11); // Simulate block time passing

    // Read game state and verify assertions
    let state: State = game_session.read_state(b"").unwrap();
    assert_eq!(state.user_sessions[0].0, user); // Verify user ID
}
