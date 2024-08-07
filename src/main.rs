use std::io;
use std::io::Write;



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
                // if let Some(winner) = game.check_winner()
                // {
                //     game.display_board();
                //     println!("Player {} wins!", winner);
                //     break;
                // }
                // else if game.is_draw()
                // {
                //     game.display_board();
                //     println!("It's a draw!");
                //     break;
                // }
                match game.check_winner()
                {
                    Some(winner) => 
                    {
                        game.display_board();
                        println!("{winner} wins!");
                        break;
                    },
                    None => 
                    {
                        if game.is_draw()
                        {
                            game.display_board();
                            println!("Draw!");
                            break;
                        }
                    } 
                };
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
    current_player:Player,
    moves: usize
}

impl Game 
{   
    fn new() -> Game 
    {
        let mut board = [' '; 9];
        let mut i = 0;
        while i < 9
        {
            board[i] = char::from_digit((i+1) as u32, 10).unwrap();
            i+=1;
        }
        Game 
        {
            board,
            current_player: Player::X,
            moves:0
        }
    }

    fn display_board(&self)
    {
        println!(
            "{0} |{1} |{2}\n--|--|--\n{3} |{4} |{5}\n--|--|--\n{6} |{7} |{8}",
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

        // if match self.board[position-1]  
        // {
        //     '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => false,
        //     _ => true,
        // }
        // {
        //     println!("Invalid Move! The desired cell is already occupied. Choose another cell");
        //     return false;
        // }

        if !('1'..='9').contains(&self.board[position-1]) //if self.board[position - 1] != ' '
        {
            println!("Invalid Move! The desired cell is already occupied. Choose another cell");
            return false;
        }

        self.board[position - 1] = match self.current_player
        {
            Player::X => 'X',
            Player::O => 'O'
        };
        self.moves += 1;
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
        //return !self.board.contains(&' ');
        return self.moves == 9;
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

