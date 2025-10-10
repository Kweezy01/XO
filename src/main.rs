//Importing package that gets user input while app is running
use std::io::stdin;

//Used to identify what each cell in the board should be populated with
enum Flag {
    P1,
    P2,
    Empty,
}

//Blank board with all empty states
fn fresh_state() -> Vec<Vec<Flag>> {
    vec![
        vec![Flag::Empty, Flag::Empty, Flag::Empty],
        vec![Flag::Empty, Flag::Empty, Flag::Empty],
        vec![Flag::Empty, Flag::Empty, Flag::Empty],
    ]
}

//Get user input and return it as a number vector
fn get_move() -> Vec<usize> {
    let mut player_move: String = String::new();
    stdin()
        .read_line(&mut player_move)
        .ok()
        .expect("Failed to read");
    let first_bits: u8 = player_move.as_bytes()[0];
    let second_bits: u8 = player_move.as_bytes()[1];
    let first: char = first_bits as char;
    let second: char = second_bits as char;
    match first {
        'A' => {}
        'B' => {}
        'C' => {}
        'a' => {}
        'b' => {}
        'c' => {}
        _ => {
            println!("Oops, you said {first}, Please use a valid letter from A to C, as a capital");
            get_move();
        }
    }
    match second {
        '1' => {}
        '2' => {}
        '3' => {}
        _ => {
            println!("Oops, you said {second}, Please use a valid number from 1 to 3");
            get_move();
        }
    }
    println!("Move sent");
    clearscreen::clear().expect("Failed to clear");
    let mut coords: Vec<usize> = vec![];
    match first {
        'A' => {
            coords.push(0);
        }
        'B' => {
            coords.push(1);
        }
        'C' => {
            coords.push(2);
        }
        'a' => {
            coords.push(0);
        }
        'b' => {
            coords.push(1);
        }
        'c' => {
            coords.push(2);
        }
        _ => {
            println!("Fatal Error, coords var in get_move not working")
        }
    }
    match second {
        '1' => {
            coords.push(2);
        }
        '2' => {
            coords.push(1);
        }
        '3' => {
            coords.push(0);
        }
        _ => {
            println!("Fatal Error, coords var in get_move not working")
        }
    }
    coords
}

//Gets the names of the players and returns it as a string vector
fn get_name() -> Vec<String> {
    let mut names: Vec<String> = vec![];
    let mut p1: String = String::new();
    println!("Insert player 1's name:");
    stdin().read_line(&mut p1).ok().expect("Failed to read");
    let mut p2: String = String::new();
    println!("Insert player 2's name:");
    stdin().read_line(&mut p2).ok().expect("Failed to read");
    names.push(p1);
    names.push(p2);
    names
}

//Changes the state of a given state grid and returns it as a new vector
fn change_state(mut state: Vec<Vec<Flag>>, flag: &bool, pos: Vec<usize>) -> Vec<Vec<Flag>> {
    match flag {
        true => {
            state[pos[1]][pos[0]] = Flag::P1;
        }
        false => {
            state[pos[1]][pos[0]] = Flag::P2;
        }
    }
    state
}

//Takes one of the three states and returns a given "Mark" to be written on the board
fn load_player(player: &Flag) -> Vec<String> {
    let mut mark: Vec<String> = vec![];
    match player {
        Flag::P1 => {
            mark.push(String::from("\\ /"));
            mark.push(String::from(" x "));
            mark.push(String::from("/ \\"));
        }
        Flag::P2 => {
            mark.push(String::from("---"));
            mark.push(String::from("| |"));
            mark.push(String::from("---"));
        }
        Flag::Empty => {
            mark.push(String::from("   "));
            mark.push(String::from("   "));
            mark.push(String::from("   "));
        }
    }
    mark
}

//Takes a board state grid and prints a board with it
fn print_board(state: &Vec<Vec<Flag>>) {
    let mut count: u8 = 3;
    println!();
    println!();
    println!();
    println!();
    for i in state {
        print!("     {}|", load_player(&i[0])[0]);
        print!("{}|", load_player(&i[1])[0]);
        println!("{}", load_player(&i[2])[0]);
        print!("  {count}  {}|", load_player(&i[0])[1]);
        print!("{}|", load_player(&i[1])[1]);
        println!("{}", load_player(&i[2])[1]);
        print!("     {}|", load_player(&i[0])[2]);
        print!("{}|", load_player(&i[1])[2]);
        println!("{}", load_player(&i[2])[2]);
        if count > 1 {
            println!("     -----------");
        }
        count = count - 1;
    }
    println!();
    println!("      A   B   C  ");
    println!();
    println!();
}

//Uses all functions so that the game may be played in the main loop
fn play() {
    println!("Only insert co-ordinates in text bar or the whole game will break");
    println!("and you will need to restart. Only use capital letters!");
    let names: Vec<String> = get_name();
    let mut board_state: Vec<Vec<Flag>> = fresh_state();
    let mut player: bool = false;
    print_board(&board_state);
    loop {
        if player {
            player = false;
        } else {
            player = true;
        }
        match &player {
            true => {
                println!("{} - Your turn", names[0]);
            }
            false => {
                print!("{} - Your turn", names[1]);
            }
        }
        board_state = change_state(board_state, &player, get_move());
        print_board(&board_state);
    }
}

//Calls our game function in the main function
fn main() {
    play();
}

