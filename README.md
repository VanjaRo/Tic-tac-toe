# Tic-tac-toe

## Description

Simple CLI Tic-Tac-Toe game made in Rust as part of Rust ITMO course.

The size of the field is 3x3. You can play the game in **player vs player** or **player vs ai**.

AI is using **Minimax** algorithm to choose the shortest option that brings him the win. That is done through the depth parameter.

Numbering of fields is:  
0 0 | 0 1 | 0 2  
1 0 | 1 1 | 1 2  
2 0 | 2 1 | 2 2

## Room for improvement

- A good idea would be to upgrade the minimax algo to the **"Alpha-beta pruning"**. That will speed up the calculation by reducing the number of cases to iterate.

## How to run

To start the project need to install Rust(I'm currently using v. 1.64.0)

To run the game `cd ` to the project directory and execute `cargo run` command in your terminal _(Unix like systems)_.
