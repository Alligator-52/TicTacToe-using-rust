use std::io;
use std::io::Write;

fn main() 
{
    
}

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
            "{0}|{1}|{2}\n{3}|{4}|{5}\n{6}|{7}|{8}",
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

        for &poistions in winning_positions.iter()
        {
            let [a,b,c] = poistions;
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

