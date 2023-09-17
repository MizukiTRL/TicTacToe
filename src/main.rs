#![allow(unused)]

use std::fmt::Display;
use std::io;
use Tictac::Empty;
use rand::Rng;

#[derive(PartialEq)]
enum Tictac {
    X,
    O,
    Empty,
}

fn print_board(grid: &mut [[Tictac; 3]; 3]) {
    for (i, row) in grid.iter().enumerate() {
        print!("___________\n");
        for (j, pos) in row.iter().enumerate() {
            match pos {
                Empty => print!(" "),
                Tictac::X => print!("X"),
                Tictac::O => print!("O"),
            }
            print!(" | ")
        }
        print!("\n")
    }
}

fn read_input() -> (usize, usize) {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("wrong output");

    println!("{:?}", input);

    let cords = match input.split_once(' ') {
        Some((x, y)) => {
            let x: usize = x.trim().parse().expect("Transformed x into an int");
            let y: usize = y.trim().parse().expect("Transformed y into an int");
            (x, y)
        }
        None => panic!("hola"),
    };
    cords
}

fn add_symbol(grid: &mut [[Tictac; 3]; 3], cords: (usize, usize), tic: Tictac) -> bool{
    match cords.0 {
        0..=2 => println!("{:?}", cords),
        _ => {
            println!("wrong place on x cordinates :C");
            return false;
        }
    }

    match cords.1 {
        0..=2 => println!("{:?}", cords),
        _ => {
            println!("wrong place on y cordinates :C");
            return false;
        }
    }

    match grid[cords.1][cords.0] {
        Empty => grid[cords.1][cords.0] = tic,
        _ => return false,
    };

    true
}

fn check_win(grid: &[[Tictac; 3]; 3], tic: Tictac) -> bool{
    //rows
    for row in grid {
        if(row[0] == tic && row[1] == tic && row[2] == tic){
            return true;
        }
    }
    //columns
    for j in 0..3{
        if grid[0][j] == tic && grid[1][j] == tic && grid[2][j] == tic{
            return true;
        }
    }

    if (grid[0][0] == tic && grid[1][1] == tic && grid[2][2] == tic) || (grid[0][2] == tic && grid[1][1] == tic && grid[2][0] == tic){
        return true;
    }

    false
}

fn main() {
    let mut grid: [[Tictac; 3]; 3] = [
        [Empty, Empty, Empty],
        [Empty, Empty, Empty],
        [Empty, Empty, Empty],
    ];

    print_board(&mut grid);

    loop {
        let cords = read_input();

        match add_symbol(&mut grid, cords, Tictac::X){
            true => (),
            false => continue,
        };

        loop {
            let cords: (usize, usize) = (rand::thread_rng().gen_range(0..3), rand::thread_rng().gen_range(0..3));
            match add_symbol(&mut grid, cords, Tictac::O){
                true => break,
                false => continue,
            };
        }

        print_board(&mut grid);

        if(check_win(&grid, Tictac::X)){
            println!("you win");
            break;
        }
        if(check_win(&grid, Tictac::O)){
            println!("you lose");
            break;
        }
    }
}
