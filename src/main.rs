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
            let send_name_msg =
                String::from("Welcome! Send \"help\" to see all available commands.");
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
                        if checkers.is_completed() {
                            game_completed(&mut websocket, checkers);
                        }
                        send_board(&mut websocket, checkers);
                    }
                    m if m.contains("set_player_name") => {
                        println!("set_player_name: command={}", msg_string);
                        if *player_count > 2 {
                            println!("set_player_name: already max players registered");
                            return;
                        }
                        if checkers.is_completed() {
                            game_completed(&mut websocket, checkers);
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
                        if checkers.is_completed() {
                            game_completed(&mut websocket, checkers);
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
                checkers.print_board();
            }
        });
    }
}

fn game_completed(websocket: &mut WebSocket<TcpStream>, checkers: &mut MutexGuard<Checkers>) {
    websocket
        .write_message(Message::Text(format!(
            "status: game completed, {} is the winner!",
            checkers.get_winner()
        )))
        .unwrap();
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

#[cfg(test)]
mod tests {
    use crate::game::{self, Checkers};

    fn setup() -> Checkers {
        let mut checkers = game::Checkers::init();
        checkers.init_with_players("test1".to_string(), "test2".to_string());
        return checkers;
    }

    #[test]
    fn test_init() {
        let mut checkers = game::Checkers::init();
        assert_eq!(checkers.get_player1(), "empty");
        assert_eq!(checkers.get_player2(), "empty");
        assert_eq!(checkers.get_turn(), "empty");
        assert!(!checkers.is_ready_to_start());
        assert_eq!(checkers.get_winner(), "empty");
    }

    #[test]
    fn test_with_players() {
        let mut checkers = setup();
        let option = checkers.init_with_players("test1".to_string(), "test2".to_string());
        assert_eq!(checkers.get_player1(), "test1");
        assert_eq!(checkers.get_player2(), "test2");
        assert_eq!(checkers.get_turn(), "test1");
        assert!(checkers.is_ready_to_start());
        assert_eq!(checkers.get_winner(), "empty");
    }

    #[test]
    fn test_move_piece_wrong_starting_piece() {
        let mut checkers = setup();
        let output = checkers.move_piece("test1".to_string(), (3, 0), (4, 1));
        assert!(!output);
    }

    #[test]
    fn test_move_piece_illegal_cords() {
        let mut checkers = setup();
        let output = checkers.move_piece("test1".to_string(), (1, 8), (2, 9));
        assert!(!output);
    }

    #[test]
    fn test_move_piece_wrong_turn() {
        let mut checkers = setup();
        let output = checkers.move_piece("test2".to_string(), (5, 6), (4, 7));
        assert!(!output);
    }

    #[test]
    fn test_move_piece_wrong_player() {
        let mut checkers = setup();
        let output = checkers.move_piece("test2".to_string(), (2, 1), (3, 0));
        assert!(!output);
    }

    #[test]
    fn test_move_piece_no_jump() {
        let mut checkers = setup();
        let output = checkers.move_piece("test1".to_string(), (2, 1), (3, 0));
        assert!(output);
    }

    #[test]
    fn test_move_piece_with_jump_available() {
        let mut checkers = setup();
        let mut output = checkers.move_piece("test1".to_string(), (2, 7), (3, 6));
        assert!(output);
        output = checkers.move_piece("test2".to_string(), (5, 4), (4, 5));
        assert!(output);
        output = checkers.move_piece("test1".to_string(), (2, 1), (3, 0));
        assert!(!output);
    }

    #[test]
    fn test_move_piece_jump() {
        let mut checkers = setup();
        let mut output = checkers.move_piece("test1".to_string(), (2, 7), (3, 6));
        assert!(output);
        output = checkers.move_piece("test2".to_string(), (5, 4), (4, 5));
        assert!(output);
        output = checkers.move_piece("test1".to_string(), (3, 6), (5, 4));
        assert!(output);
    }

    #[test]
    fn test_move_piece_double_jump() {
        let mut checkers = setup();
        let board = &mut checkers.get_board().positions;
        let mut piece2 = board.get(5).unwrap().get(0).unwrap().clone();
        piece2.occupant.loc = (3, 6);
        board[3][6] = piece2.clone();
        let mut empty = board.get(3).unwrap().get(0).unwrap().clone();
        empty.occupant.loc = (6, 5);
        board[6][5] = empty.clone();

        let mut output = checkers.move_piece("test1".to_string(), (2, 5), (4, 7));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
        output = checkers.move_piece("test1".to_string(), (4, 7), (6, 5));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
    }

    #[test]
    fn test_move_piece_kinged() {
        let mut checkers = setup();
        let board = &mut checkers.get_board().positions;
        let mut piece2 = board.get(5).unwrap().get(0).unwrap().clone();
        piece2.occupant.loc = (1, 6);
        board[1][6] = piece2.clone();
        let mut empty = board.get(3).unwrap().get(0).unwrap().clone();
        empty.occupant.loc = (0, 7);
        empty.owner = "test1".to_string();
        board[0][7] = empty.clone();
        empty.occupant.loc = (0, 5);
        board[0][5] = empty.clone();
        empty.occupant.loc = (2, 7);
        board[2][7] = empty.clone();
        empty.occupant.loc = (3, 6);
        board[3][6] = empty.clone();
        empty.occupant.loc = (4, 7);
        board[4][7] = empty.clone();
        empty.occupant.loc = (5, 6);
        board[5][6] = empty.clone();
        empty.occupant.loc = (6, 7);
        board[6][7] = empty.clone();
        empty.occupant.loc = (7, 6);
        board[7][6] = empty.clone();
        checkers.print_board();

        let mut output = checkers.move_piece("test1".to_string(), (2, 5), (3, 4));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        output = checkers.move_piece("test2".to_string(), (1, 6), (0, 7));
        checkers.print_board();
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
        assert!(
            checkers
                .get_board()
                .positions
                .get(0)
                .unwrap()
                .get(7)
                .unwrap()
                .occupant
                .kinged
        );
    }

    #[test]
    fn test_move_piece_king_jump_normal() {
        let mut checkers = setup();
        let board = &mut checkers.get_board().positions;
        let mut piece2 = board.get(5).unwrap().get(0).unwrap().clone();
        piece2.occupant.loc = (1, 6);
        board[1][6] = piece2.clone();
        let mut empty = board.get(3).unwrap().get(0).unwrap().clone();
        empty.occupant.loc = (0, 7);
        empty.owner = "test1".to_string();
        board[0][7] = empty.clone();
        empty.occupant.loc = (0, 5);
        board[0][5] = empty.clone();
        empty.occupant.loc = (2, 7);
        board[2][7] = empty.clone();
        empty.occupant.loc = (3, 6);
        board[3][6] = empty.clone();
        empty.occupant.loc = (4, 7);
        board[4][7] = empty.clone();
        empty.occupant.loc = (5, 6);
        board[5][6] = empty.clone();
        empty.occupant.loc = (6, 7);
        board[6][7] = empty.clone();
        empty.occupant.loc = (7, 6);
        board[7][6] = empty.clone();
        checkers.print_board();

        let mut output = checkers.move_piece("test1".to_string(), (2, 5), (3, 4));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        output = checkers.move_piece("test2".to_string(), (1, 6), (0, 7));
        checkers.print_board();
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
        assert!(
            checkers
                .get_board()
                .positions
                .get(0)
                .unwrap()
                .get(7)
                .unwrap()
                .occupant
                .kinged
        );
        output = checkers.move_piece("test1".to_string(), (2, 1), (3, 0));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        output = checkers.move_piece("test2".to_string(), (0, 7), (1, 6));
        checkers.print_board();
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
    }

    #[test]
    fn test_move_piece_king_jump_backwards() {
        let mut checkers = setup();
        let board = &mut checkers.get_board().positions;
        let mut piece2 = board.get(5).unwrap().get(0).unwrap().clone();
        piece2.occupant.loc = (1, 6);
        board[1][6] = piece2.clone();
        let mut empty = board.get(3).unwrap().get(0).unwrap().clone();
        empty.occupant.loc = (0, 7);
        empty.owner = "test1".to_string();
        board[0][7] = empty.clone();
        empty.occupant.loc = (0, 5);
        board[0][5] = empty.clone();
        empty.occupant.loc = (2, 7);
        board[2][7] = empty.clone();
        empty.occupant.loc = (3, 6);
        board[3][6] = empty.clone();
        empty.occupant.loc = (4, 7);
        board[4][7] = empty.clone();
        empty.occupant.loc = (5, 6);
        board[5][6] = empty.clone();
        empty.occupant.loc = (6, 7);
        board[6][7] = empty.clone();
        empty.occupant.loc = (7, 6);
        board[7][6] = empty.clone();
        empty.occupant.loc = (2, 3);
        board[2][3] = empty.clone();

        let mut output = checkers.move_piece("test1".to_string(), (2, 5), (3, 4));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        output = checkers.move_piece("test2".to_string(), (1, 6), (0, 5));
        checkers.print_board();
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
        output = checkers.move_piece("test1".to_string(), (3, 4), (4, 5));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        output = checkers.move_piece("test2".to_string(), (0, 5), (2, 3));
        checkers.print_board();
        assert!(output);
        assert_eq!(checkers.get_turn(), "test1");
    }

    #[test]
    fn test_move_piece_into_winning() {
        let mut checkers = setup();
        let board = &mut checkers.get_board().positions;
        for row_index in 0..board.len() {
            for col_index in 0..board[row_index].len() {
                let mut piece = board
                    .get(row_index)
                    .unwrap()
                    .get(col_index)
                    .unwrap()
                    .clone();
                if !piece.blocked {
                    piece.occupant.owner = "empty".to_string();
                    board[row_index][col_index] = piece;
                }
            }
        }
        let mut piece = board.get(0).unwrap().get(0).unwrap().clone();
        piece.blocked = false;
        piece.occupant.owner = "test1".to_string();
        piece.occupant.loc = (1, 0);
        piece.occupant.direction = 1;
        board[1][0] = piece.clone();

        piece.occupant.owner = "test2".to_string();
        piece.occupant.loc = (2, 1);
        piece.occupant.direction = -1;
        board[2][1] = piece.clone();

        let output = checkers.move_piece("test1".to_string(), (1, 0), (3, 2));
        assert!(output);
        assert_eq!(checkers.get_turn(), "test2");
        assert!(checkers.is_completed());
        assert_eq!(checkers.get_winner(), "test1");
    }
}
