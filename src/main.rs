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
        println!({self.board[0]}|{self.board[1]}|{self.board[2]});
        println!({self.board[3]}|{self.board[4]}|{self.board[5]});
        println!({self.board[8]}|{self.board[7]}|{self.board[8]});
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

        }
    }
}

