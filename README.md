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
### In a file
To specify rules in a file, create a json structure:
```json
{
    "cell": [int -> 2] ,                     // (0-255) cell state between 0 and provided number
    "range": [int -> 10],                    // (0-255) 
    "survival": {
        "start": [int -> 2],                 //  >0
        "end": [int -> 222]                  //  >0
    },
    "birth": {                               //  >0  affects the value of the cell field
        "start": [int -> 2],                 //  >0 
        "end": [int -> 2],                   //  >0
    },
    "neighbourhood": {
        "type": [string -> "Moore"]         // Moore | VonNeumann
    }
}
```
After arrow you can see some examples.

### In a program
To specify rules in a program, create structured string:
```
C:[int];R[int];S:[range|int];B:[range|int];N:[N|M]
```
For example:
```
C:2;R:1;S:2-3;B:3;N:
C:2;R:1;S:0;B:1;N:N
C:2;R:7;S:99-199;B:75-170;N:M
```