use std::io;
use std::io::Write;

// todo
// 1-  add numbers to cells for easy identification
// 2- add horizontal divisions
fn main() 
{
    let mut game = Game::new();
    let mut input = String::new();

    loop
    {
        game.display_board();
        print!("Player {:?}, enter your move (1-9): ", game.current_player);
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(position) = input.trim().parse::<usize>()
        {
            if game.make_move(position)
            {
                if let Some(winner) = game.check_winner()
                {
                    game.display_board();
                    println!("Player {} wins!", winner);
                    break;
                }
                else if game.is_draw()
                {
                    game.display_board();
                    println!("It's a draw!");
                    break;
                }
                game.next_turn();
            }
        }
        else
        {
            println!("Invalid input. Please enter a number between 1 and 9.");
        }
    }
}

#[derive(PartialEq, Debug)]
enum Player
{
    X,O
}

struct Game
{
    board: [char;9],
    current_player:Player
}

impl Game 
{   
    fn new() -> Game
    {
        Game 
        {
            board: [' ';9],
            current_player : Player::X
        }
    }

    fn display_board(&self)
    {
        println!(
            "{0} |{1} |{2}\n__|__|__\n{3} |{4} |{5}\n__|__|__\n{6} |{7} |{8}\n  |  |  ",
            self.board[0], self.board[1], self.board[2],
            self.board[3], self.board[4], self.board[5],
            self.board[6], self.board[7], self.board[8]
        );
    }

    fn make_move(&mut self, position:usize) -> bool
    {
        if position < 1 || position > 9
        {
            println!("Invalid move! Enter a number betweem 1 to 9");
            return false;
        }

        if self.board[position - 1] != ' '
        {
            println!("Invalid Move! The desired cell is already occupied. Choose another cell");
            return false;
        }

        self.board[position - 1] = match self.current_player
        {
            Player::X => 'X',
            Player::O => 'O'
        };
        return true;
        
    }

    fn check_winner(&self) -> Option<char>
    {
        let winning_positions = 
        [
            [0,1,2], [3,4,5], [6,7,8],
            [0,3,6], [1,4,7], [2,5,8],
            [0,4,8], [2,4,6]
        ];

        for &positions in winning_positions.iter()
        {
            let [a,b,c] = positions;
            if self.board[a] != ' ' && self.board[a] == self.board[b] && self.board[a] == self.board[c]
            {
                return Some(self.board[a]);
            }
        }
        return None;
    }

    fn is_draw(&self) -> bool
    {
        return !self.board.contains(&' ');
    }

    fn next_turn(&mut self)
    {
        self.current_player = match self.current_player 
        {
            Player::X => Player::O,
            Player::O => Player::X
        };
    }
}

