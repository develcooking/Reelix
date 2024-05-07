## About
This project trys to replace the "Neelix" programm in the IT-Support. 
Neelix was written in about 2015-17. The Sourcecode seems to be lost.
You have to have java installed and an variable set for JAVA_HOME.
The databankserver uses an out of support Linux system.

Reelix wants to redo all this. Written in Rust and uses [Slint](https://slint.dev/) as the UI Framework. Reelix sloud run on every Desktop OS after compilation. Officialy supported is only Linux and Windows.

## How to use

### The easy way 
Download the bin file for your Oporating System of Choice

### THE HARD WAY
1. You have to install  ```rustc``` and ```cargo``` [(Rust Getting Started Guide)](https://www.rust-lang.org/learn/get-started)
2. Go to [the tutorial](https://releases.slint.dev/1.5.1/docs/tutorial/rust/introduction)
there you have to install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3.   Clone this Repo

    git clone https://gitlab.rz.uni-bamberg.de/ba4ju8/reelix/
 
4. Cd into it    
```
cd ./reelix
 ```
5. Smash your head against the wall
6. Build the binary
```
cargo build
```


If this doesn't work good luck 

## This to do (Basicly all)


Connect to a sql database
and a lot more
