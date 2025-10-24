//Importing package that gets user input while app is running
use std::io::{self, Write};

//Used to identify what each cell in the board should be populated with
#[derive(Clone, Copy, Debug, PartialEq)]
enum Flag {
    P1,
    P2,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    One,
    Two,
}

impl Player {
    fn toggle(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
    fn to_flag(self) -> Flag {
        match self {
            Player::One => Flag::P1,
            Player::Two => Flag::P2,
        }
    }
}

//--Old board construction method--//
//
//Blank board with all empty states
// fn fesh_state() -> Vec<Vec<Flag>> {
//     vec![
//         vec![Flag::Empty, Flag::Empty, Flag::Empty],
//         vec![Flag::Empty, Flag::Empty, Flag::Empty],
//         vec![Flag::Empty, Flag::Empty, Flag::Empty],
//     ]
// }

type Board = [[Flag; 3]; 3];

fn fresh_state() -> Board {
    [[Flag::Empty; 3]; 3]
}

//--Old Get Player Names Code--//
//
//Gets the names of the players and returns it as a string vector
// fn get_name() -> Vec<String> {
//     let mut names: Vec<String> = vec![];
//     let mut p1: String = String::new();
//     println!("Insert player 1's name:");
//     stdin().read_line(&mut p1).ok().expect("Failed to read");
//     let mut p2: String = String::new();
//     println!("Insert player 2's name:");
//     stdin().read_line(&mut p2).ok().expect("Failed to read");
//     names.push(p1);
//     names.push(p2);
//     names
// }

fn get_names() -> (String, String) {
    print!("Insert player 1's name: ");
    io::stdout().flush().unwrap();
    let mut p1 = String::new();
    io::stdin()
        .read_line(&mut p1)
        .expect("Failed to read player 1's name");
    let p1 = p1.trim().to_string();

    print!("Insert player 2's name: ");
    io::stdout().flush().unwrap();
    let mut p2 = String::new();
    io::stdin()
        .read_line(&mut p2)
        .expect("Failed to read player 2's name");
    let p2 = p2.trim().to_string();

    (p1, p2)
}

//--Old Input Handling--//
//
//Get user input and return it as a number vector
// fn get_move() -> Vec<usize> {
//     let mut player_move: String = String::new();
//     stdin()
//         .read_line(&mut player_move)
//         .ok()
//         .expect("Failed to read");
//     let first_bits: u8 = player_move.as_bytes()[0];
//     let second_bits: u8 = player_move.as_bytes()[1];
//     let first: char = first_bits as char;
//     let second: char = second_bits as char;
//     match first {
//         'A' => {}
//         'B' => {}
//         'C' => {}
//         'a' => {}
//         'b' => {}
//         'c' => {}
//         _ => {
//             println!("Oops, you said {first}, Please use a valid letter from A to C, as a capital");
//             get_move();
//         }
//     }
//     match second {
//         '1' => {}
//         '2' => {}
//         '3' => {}
//         _ => {
//             println!("Oops, you said {second}, Please use a valid number from 1 to 3");
//             get_move();
//         }
//     }
//     println!("Move sent");
//     clearscreen::clear().expect("Failed to clear");
//     let mut coords: Vec<usize> = vec![];
//     match first {
//         'A' => {
//             coords.push(0);
//         }
//         'B' => {
//             coords.push(1);
//         }
//         'C' => {
//             coords.push(2);
//         }
//         'a' => {
//             coords.push(0);
//         }
//         'b' => {
//             coords.push(1);
//         }
//         'c' => {
//             coords.push(2);
//         }
//         _ => {
//             println!("Fatal Error, coords var in get_move not working")
//         }
//     }
//     match second {
//         '1' => {
//             coords.push(2);
//         }
//         '2' => {
//             coords.push(1);
//         }
//         '3' => {
//             coords.push(0);
//         }
//         _ => {
//             println!("Fatal Error, coords var in get_move not working")
//         }
//     }
//     coords
// }

fn get_move() -> (usize, usize) {
    loop {
        print!("Enter move (A1..C3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again...");
            continue;
        }

        let input = input.trim().to_uppercase();
        if input.len() != 2 {
            println!("Please enter exactly 2 charachters, e.g. A1");
            continue;
        }

        let mut chars = input.chars();
        let col_char = chars.next().unwrap();
        let row_char = chars.next().unwrap();

        let col = match col_char {
            'A' => Some(0),
            'B' => Some(1),
            'C' => Some(2),
            _ => None,
        };

        let row = match row_char {
            '1' => Some(2),
            '2' => Some(1),
            '3' => Some(0),
            _ => None,
        };

        if let (Some(c), Some(r)) = (col, row) {
            return (c, r);
        } else {
            println!("Invalid co-ordinates, Use columns A-C and rows 1-3!")
        }
    }
}

//---Old change board state code---//
//Changes the state of a given state grid and returns it as a new vector
// fn change_state(mut state: Vec<Vec<Flag>>, flag: &bool, pos: Vec<usize>) -> Vec<Vec<Flag>> {
//     match flag {
//         true => {
//             state[pos[1]][pos[0]] = Flag::P1;
//         }
//         false => {
//             state[pos[1]][pos[0]] = Flag::P2;
//         }
//     }
//     state
// }

fn try_place_mark(
    board: &mut Board,
    player: Player,
    pos: (usize, usize),
) -> Result<(), &'static str> {
    let (col, row) = pos;
    if col >= 3 || row >= 3 {
        return Err("co-ordinates out of bounds");
    }

    if board[row][col] != Flag::Empty {
        return Err("Cell already occupied");
    }

    board[row][col] = player.to_flag();

    Ok(())
}

// //Takes one of the three states and returns a given "Mark" to be written on the board
// fn load_player(player: &Flag) -> Vec<String> {
//     let mut mark: Vec<String> = vec![];
//     match player {
//         Flag::P1 => {
//             mark.push(String::from("\\ /"));
//             mark.push(String::from(" x "));
//             mark.push(String::from("/ \\"));
//         }
//         Flag::P2 => {
//             mark.push(String::from("---"));
//             mark.push(String::from("| |"));
//             mark.push(String::from("---"));
//         }
//         Flag::Empty => {
//             mark.push(String::from("   "));
//             mark.push(String::from("   "));
//             mark.push(String::from("   "));
//         }
//     }
//     mark
// }

fn cell_lines(flag: Flag) -> [&'static str; 3] {
    match flag {
        Flag::P1 => ["\\ /", " x ", "/ \\"],
        Flag::P2 => ["---", "| |", "---"],
        Flag::Empty => ["   ", "   ", "   "],
    }
}

//Old Board Updater Code
//Takes a board state grid and prints a board with it
// fn print_board(state: &Vec<Vec<Flag>>) {
//     let mut count: u8 = 3;
//     println!();
//     println!();
//     println!();
//     println!();
//     for i in state {
//         print!("     {}|", load_player(&i[0])[0]);
//         print!("{}|", load_player(&i[1])[0]);
//         println!("{}", load_player(&i[2])[0]);
//         print!("  {count}  {}|", load_player(&i[0])[1]);
//         print!("{}|", load_player(&i[1])[1]);
//         println!("{}", load_player(&i[2])[1]);
//         print!("     {}|", load_player(&i[0])[2]);
//         print!("{}|", load_player(&i[1])[2]);
//         println!("{}", load_player(&i[2])[2]);
//         if count > 1 {
//             println!("     -----------");
//         }
//         count = count - 1;
//     }
//     println!();
//     println!("      A   B   C  ");
//     println!();
//     println!();
// }

fn print_board(board: &Board) {
    println!("\n\n");
    for (i, row) in board.iter().enumerate() {
        let display_row = 3 - i;
        for line in 0..3 {
            if line == 1 {
                print!("  {}  ", display_row);
            } else {
                print!("     ")
            }
            for (j, &cell) in row.iter().enumerate() {
                let lines = cell_lines(cell);
                print!("{}", lines[line]);
                if j < 2 {
                    print!("|");
                }
            }
            println!();
        }
        if i < 2 {
            println!("     ----------")
        }
    }
    println!("\n     A   B   C   \n")
}

// //Uses all functions so that the game may be played in the main loop
// fn play() {
//     println!("Only insert co-ordinates in text bar or the whole game will break");
//     println!("and you will need to restart. Only use capital letters!");
//     let names: Vec<String> = get_name();
//     let mut board_state: Vec<Vec<Flag>> = fresh_state();
//     let mut player: bool = false;
//     print_board(&board_state);
//     loop {
//         if player {
//             player = false;
//         } else {
//             player = true;
//         }
//         match &player {
//             true => {
//                 println!("{} - Your turn", names[0]);
//             }
//             false => {
//                 print!("{} - Your turn", names[1]);
//             }
//         }
//         board_state = change_state(board_state, &player, get_move());
//         print_board(&board_state);
//     }
// }

fn check_winner(board: &Board) -> Option<Flag> {
    // rows, columns, diagonals
    let lines = [
        // rows
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        // cols
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        // diagonals
        [(0, 0), (1, 1), (2, 2)],
        [(2, 0), (1, 1), (0, 2)],
    ];

    for &line in &lines {
        let (ax, ay) = line[0];
        let (bx, by) = line[1];
        let (cx, cy) = line[2];
        let a = board[ay][ax];
        if a != Flag::Empty && a == board[by][bx] && a == board[cy][cx] {
            return Some(a);
        }
    }
    None
}

fn is_draw(board: &Board) -> bool {
    board
        .iter()
        .all(|row| row.iter().all(|&cell| cell != Flag::Empty))
}

fn play() {
    println!("Only insert co-ordinates from A1 to C3");

    let (p1_name, p2_name) = get_names();
    let mut board = fresh_state();
    let mut player = Player::One;

    loop {
        print_board(&board);
        let current_name = if player == Player::One {
            &p1_name
        } else {
            &p2_name
        };
        println!("{} - Your turn {:?}", current_name, player);

        let pos = get_move();
        match try_place_mark(&mut board, player, pos) {
            Ok(()) => {}
            Err(err) => {
                print!("{}", err);
                continue;
            }
        }

        //check winner

        if let Some(winner_flag) = check_winner(&board) {
            print_board(&board);
            match winner_flag {
                Flag::P1 => println!("{} Wins!", p1_name),
                Flag::P2 => println!("{} Wins!", p2_name),
                _ => {}
            }
            break;
        }

        if is_draw(&board) {
            println!("It's a draw!");
            break;
        }

        player = player.toggle();
    }
}

//Calls our game function in the main function
fn main() {
    play();
}
