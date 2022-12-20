extern crate termcolor;
///External crate for writing colored text to a terminal
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
///Function that outputs a greeting message
fn greeting() {
    println!(
        "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
    )
}
///this function creates the players
fn print_player(player: &char) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
///Colors the input of player X
    if player == &'X' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
    } else if player == &'O' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
    }

    write!(&mut stdout, "{}", player).unwrap();
    stdout.reset().unwrap();
}
///This function creates the tic tac toe board
fn draw(state: &[char]) {
    println!("\n");
///For loop which allows for the board to contiously be printed after each input
    for i in (0..3).rev() {
        let offset = i * 3;

        print!("-------------\n| ");
        print_player(&state[offset]);
        print!(" | ");
        print_player(&state[offset + 1]);
        print!(" | ");
        print_player(&state[offset + 2]);
        println!(" |");
    }

    println!("-------------");
}
///Function that asks the user to pick a number located on the board
fn ask_user(state: &mut [char], player: char) {
    loop {
        print!("Player '");
        print_player(&player);
        println!("', enter a number: ");
///if statement which error handles if it cannot read the line
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }
///if statement that error handles if the number is outside 1-9
        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;
///if statement that error handles if a spot on the board is already taken
            if state[number] == 'X' || state[number] == 'O' {
                print!("This field is already taken by '");
                print_player(&state[number]);
                println!("'.");
                continue;
            }

            state[number] = player;
///Else statement if there is invalid input which prints out an error message
            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}
///Function that checks if the user has won or not
fn has_won(state: &[char]) -> bool {
    for tmp in 0..3 {
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }

        let tmp = tmp * 3;

        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }

    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }

    false
}
///This is a function that will check if all of the fields are used
#[inline(always)]
fn is_over(state: &[char]) -> bool {
    state.iter().all(|&v| v == 'X' || v == 'O')
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    ///Calls the greeting function which greets player
    greeting();

    loop {
        ///Calls the draw function which creates the board
        draw(&state);

        ///Calls the ask_user function which asks the user for input
        ask_user(&mut state, player);

        ///Calls the has_won function to check if a user has won
        if has_won(&state) {
            draw(&state);
            print!("Player '");
            print_player(&player);
            println!("' won! \\(^.^)/");
            break;
        }

        ///is_over function that will check if the gamehas ended 
        if is_over(&state) {
            draw(&state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' }
    }
}
