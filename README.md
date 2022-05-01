# GameOfLife

## Running

To build and run our program, you need to have installed [poetry](https://python-poetry.org/docs/#installation).

If you have it installed, then you following commands available for you:

- to install dependencies:
    > make install

- to build rust library:
    > make build

- to run the program:
    > make run

- to run tests:
    > make test

- to generate and open documentation (rust only):
    > make docs

- to lint source code (python only):
    > make lint

- to format source code:
    > make format

Our program is set up to use Python 3.10.2 and Rust 1.60.0


## Rules
To specify rules in a file, create a json structure:
```json
{
    "cell": [int -> 2] ,                     // (0-255) cell state between 0 and provided number
    "range": [int -> 10],                    // (0-255) 
    "survival": [string -> "2" | "4-5"],     //  >0
    "birth": [string ->"3" | "2-3"],         //  >0  affects the value of the cell field
    "neighbourhood": [string -> "M" | "N"],  //  M | N
}
```