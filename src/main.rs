fn main() {
    // Initialise bitboard
    let mut game = connect4_board_library::Bitboard::new();

    loop {
        // "Variable & 1" will be 1 when odd, 0 when even
        // Add one to get to player 1, player 2, rather than 0 and 1
        let current_player = (game.move_counter & 1) + 1;

        // Bitboard implements the display trait so it can be printed
        println!(
            "Player {}'s turn: \n{}\n Choose column:",
            current_player, game
        );

        // Take number input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: usize = input.trim().parse().unwrap();

        if input > 7 || input < 1 {
            println!("Invalid column");
            continue;
        }

        // drop_piece will return false if it is unnable to drop the piece
        if !game.drop_piece(input - 1) {
            println!("Column full");
            continue;
        }

        if game.check_win() {
            println!("Player {} won! \n{}", current_player, game);
            break;
        }

        // There are a total of 42 spaces in a Connect Four board
        if game.move_counter >= 42 {
            println!("Game over! It's a draw. \n{}", game);
            break;
        }
    }
}
