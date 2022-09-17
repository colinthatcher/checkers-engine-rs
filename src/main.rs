mod game;

use crate::game::Checkers;
use crate::game::EMPTY_POS;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::protocol::Message;
use tungstenite::WebSocket;

fn main() {
    // const PLAYER1: &str = "colin";
    // const PLAYER2: &str = "noah";

    const DING: &str = "ding";
    const DONGER: &str = "donger";

    // Setup the checker's game
    let option_checkers = game::Checkers::init();
    let checkers_mutex = Arc::new(Mutex::new(option_checkers));

    let player_count_shared: u8 = 0;
    let player_count_arc = Arc::new(Mutex::new(player_count_shared));
    let player_name_list: Vec<String> = Vec::new();
    let player_name_list_arc = Arc::new(Mutex::new(player_name_list));

    // Start checkers websocket server
    println!("Starting websocket server on port 9001!");
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    // Listen for player websocket clients
    for stream in server.incoming() {
        let checkers_arc = Arc::clone(&checkers_mutex);
        let player_count_mutex = Arc::clone(&player_count_arc);
        let player_name_list_mutex = Arc::clone(&player_name_list_arc);
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            // Send greeting message to player client
            let send_name_msg = String::from(
                "Welcome! Send \"help\" to see all available commands.",
            );
            websocket
                .write_message(Message::Text(send_name_msg))
                .unwrap();

            let mut player_name: Option<String> = None;

            // Server thread starts listening for websocket messages
            loop {
                let msg = websocket.read_message().unwrap();
                if !msg.is_text() {
                    println!("Received non-text websocket, cannot proceed");
                    continue;
                }
                let mut player_count = player_count_mutex.lock().unwrap();
                let mut checkers = &mut checkers_arc.lock().unwrap();
                // Pre-game
                let msg_string = msg.into_text().unwrap();

                match msg_string.as_str() {
                    m if m.contains("help") => {
                        let arst = [
                            "\"donger\" -- Status check msg. Outputs \"turn:<name>\\nboard:<game_board>\".",
                            "\"set_player_name <name>\" -- Register to play a game. Outputs \"player:<name>\".",
                            "\"move_piece <target x> <target y> <destination x> <destination y>\" -- Attempt to move a piece from a target location to a destination location. Outputs \"move_piece: success..\".",
                        ];
                        websocket
                            .write_message(Message::Text(arst.join("\n")))
                            .unwrap();
                    }
                    m if m.contains("donger") => {
                        send_board(&mut websocket, checkers);
                    }
                    m if m.contains("set_player_name") => {
                        println!("set_player_name: command={}", msg_string);
                        if *player_count > 2 {
                            println!("set_player_name: already max players registered");
                            return;
                        }
                        // Add player name to list of gamers
                        let mut msg_list = msg_string.split_whitespace();
                        let mut player_name_list = player_name_list_mutex.lock().unwrap();
                        let sent_player_name = msg_list.nth(1).unwrap().to_string();
                        player_name = Some(sent_player_name.clone());
                        player_name_list.push(sent_player_name);
                        // Increment player count
                        *player_count += 1;
                        // Start game if we have at least two players registered!
                        if *player_count == 2 {
                            start_game(&mut websocket, checkers, player_name_list.clone());
                        }
                        websocket
                            .write_message(Message::Text(format!(
                                "player:{}",
                                player_name.as_ref().unwrap()
                            )))
                            .unwrap();
                    }
                    m if m.contains("move_piece") => {
                        // TODO: Start the timer (on the first iteration)!
                        // TODO: Parse player command
                        println!("move_piece: command={}", msg_string);
                        // move command format
                        // move_peice 1 2 3 2
                        let mut arst = msg_string.split_whitespace();
                        let c_x = arst.nth(1).unwrap().parse::<usize>().unwrap();
                        let c_y = arst.next().unwrap().parse::<usize>().unwrap();
                        let d_x = arst.next().unwrap().parse::<usize>().unwrap();
                        let d_y = arst.next().unwrap().parse::<usize>().unwrap();
                        let success = checkers.move_piece(
                            player_name.clone().unwrap(),
                            (c_x, c_y),
                            (d_x, d_y),
                        );
                        if success {
                            websocket
                                .write_message(Message::Text(format!(
                                    "move_piece:\"successfully moved piece to ({}, {})\"",
                                    d_x, d_y
                                )))
                                .unwrap();
                        } else {
                            websocket
                                .write_message(Message::Text(format!(
                                    "move_piece:\"failed to movf piece to ({}, {})\"",
                                    d_x, d_y
                                )))
                                .unwrap();
                        }
                        send_board(&mut websocket, checkers);
                    }
                    _ => {
                        println!("default: received unknown command={}", msg_string.trim());
                        websocket
                            .write_message(Message::Text(String::from(
                                "Unknown command. Send \"help\" to see all available commands.",
                            )))
                            .unwrap();
                    }
                }
                drop(checkers);
            }
        });
    }
}

fn start_game(
    websocket: &mut WebSocket<TcpStream>,
    checkers: &mut MutexGuard<Checkers>,
    player_name_list: Vec<String>,
) {
    let start = String::from("Game Started");
    websocket.write_message(Message::Text(start)).unwrap();

    let option_checkers = checkers.setup_players(
        player_name_list.get(0).unwrap().clone(),
        player_name_list.get(1).unwrap().clone(),
    );
    if option_checkers.is_none() {
        println!("Failed to initialize checkers board.");
        websocket.write_message(Message::Text(String::from(
            "SERVER ERROR: Please notify dumb admin that checker board failed to setup!",
        )));
        return;
    }

    send_board(websocket, checkers);
}

fn send_board(websocket: &mut WebSocket<TcpStream>, checkers: &mut MutexGuard<Checkers>) {
    // Use mutex to get the current player's turn
    let current_player_turn = checkers.get_turn();
    let board_state_string = checkers.get_board().get_board_as_string();

    websocket
        .write_message(Message::Text(format!("turn:{}", current_player_turn)))
        .unwrap();
    websocket
        .write_message(Message::Text(format!("board:{}", board_state_string)))
        .unwrap();
}
// fn main() {
// println!("Hello, world!");
// const PLAYER1: &str = "colin";
// const PLAYER2: &str = "noah";
// let option_checkers =
//     game::Checkers::init_with_players(PLAYER1.to_string(), PLAYER2.to_string());
// if option_checkers.is_none() {
//     println!("Failed to initialize checkers board.");
//     return;
// }
// let mut checkers = option_checkers.unwrap();
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (2, 1), (3, 0))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (5, 4), (4, 3))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (2, 3), (3, 2))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (6, 3), (5, 4))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (2, 5), (3, 4))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (5, 6), (4, 7))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (3, 2), (4, 1))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (4, 7), (3, 6))
//     );
//     checkers.print_board();
//     // DOUBLE JUMP BB
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (2, 7), (4, 5))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (4, 5), (6, 3))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (7, 2), (5, 4))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER1.to_string(), (4, 1), (6, 3))
//     );
//     checkers.print_board();
//     println!(
//         "{}",
//         checkers.move_piece(PLAYER2.to_string(), (6, 7), (4, 5))
//     );
//     checkers.print_board();
// }
