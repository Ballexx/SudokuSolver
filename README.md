# Sudoku Solver
* A tool that solves given sudoku puzzle
* Includes a board generator to convert puzzle in to array

# How it works
```js 
    [
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0]
    ]

    //The solver takes this as the board, therefore generating it through the generator is simpler
```

# How to use
* Use the generator.html with live server in /generator
* Enter the numbers that you want to solve from on the board
* Press build and then copy the content from the console
* Use this copied string as your board in main.rs
* then just run cargo r and it will print the answer in console