use std::{io::{self, stdin}, vec};
use colored::Colorize;

const Player_X:char='X';
const Player_O:char='O';
const board_size:usize=3;
//3 x 3
type Board=[[char;board_size];board_size];

fn initailize_board()->Board{
    return[[' ';board_size];board_size];

}

fn print_board(board:&Board){
    for row in  board{
        for cell in row{
            print!("{} ",cell);
            print!(" ")
        }
        println!();
    }
}


fn get_player_move(current_player:char, board:&Board) -> (usize, usize) {
    loop {
        println!("Player {} Enter your move as (row col):", current_player);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("invalid input");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < board_size && col < board_size && board[row][col] == ' ' {
                return (row, col);
            } else {
                println!("Invalid move: cell is out of bounds or already taken.");
            }
        } else {
            println!("Please enter two numbers separated by space.");
        }
    
    }
}

fn check_winner(current_player:char,board:&Board)->bool{
      //row
      for row in 0..board_size{
        if board[row][0]==current_player && board[row][1]==current_player && board[row][2]==current_player{
            return true;
        }
      }
      //column
      for col in 0..board_size{
        if board[0][col]==current_player && board[1][col]==current_player && board[2][col]==current_player{
            return true;
        }
      }
    //   diagonal
       if board[0][1]==current_player && board[1][1]==current_player && board[2][2]==current_player || board[0][2]==current_player && board[1][1]==current_player && board[2][0]==current_player  {
          return true;
       }  
       

      return  false;
}

fn check_draw(board:&Board)->bool{
     for row in  board{
        for cell in row{
            if *cell == ' '{
                return false;
        }
    }
}
return true;
}
fn play_game(){
    let mut board = initailize_board();
    let mut current_player=Player_X;
    

    loop{
        println!("Current board is ");
        print_board(&board);
      
      let (row,col)=get_player_move(current_player,&board);
      board[row][col]= current_player;
        
      
        if check_winner(current_player,&board){
            println!("player {} is the winner",current_player);
            break;

        }
        if check_draw(&board){
            println!("The Game is Draw");
            break;
        }
        current_player= if current_player  == Player_X {
             Player_O
       } else{
         Player_X
    }
    }

}

fn main() {
    println!("Welcome To The Tic Tac Toe Game");
    play_game();
}
