# **Rust-book**

## 1. Getting Started
### 1.1 Hello World
* rustc main  : Compile the project, save then compile the project before   running it. after successfull compiling, the file is binary executable output

* ./main  : Run the code
* rustup component add rustfmt : To install rustfmt

* cargo fmt : Reformats all the Rust code in the current crate
  rustup component add clippy ( cargo clippy ): The Clippy tool is a collection of lints to analyze your code so you can catch common mistakes and improve your Rust code.

* Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively)

### 1.2 Hello Cargo

* cargo --version : Check cargo version

* cargo new hello_cargo : Create project with Cargo. this will create a new directory( hello_cargo )

* cargo init : Create project in the current directory

* cargo new --vcs=git :  Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using cargo new --vcs=git.

* cargo build: Build your project, as the project is organized by the help og cargo

* cargo run : Run your code 

* ./target/debug/hello_cargo : also run the code via the target folder created by cargo

## 2. Programming a Guessing Game

`let mut guess =  String::from("Lelo "); `

`let mut guess_2 =  String::new(); `

`io::stdin().read_line(&mut guess).expect("Failed to read line");`

> read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents)

> Build the project after adding a new dependency cargo.toml to install crate package 

## 3. Common Programming Concepts
### 3.1 Variables and Mutability
  > `let x = 5;`

  > `let mut y = 5;`

  > `y = 6;`

* Constants

  `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

* Shadowing

  `let spaces = "   ";`

  `let spaces = spaces.len();`
### 3.2 Data Types
* Primitive Data Type are stored in Stack based on their stack frame during program execution
* Theay fixed length data, and their types and size are known at compile time.
* Easy to access and fast to work with, compare to Data stored in the Heap(Future topic)
> Scalar Types:
  * Integers
  * ![Integers](/Programming_Concepts/DataTypes//integer.PNG)
  * Floating
  
    `let x = 2.0; // f64`
    
    `let y: f32 = 3.0; // f32`
  * Boolean

    `let t = true;`

    `let f: bool = false; // with explicit type annotation`
  * Character

    `let c = 'z'`;

    `let z: char = 'ℤ'; // with explicit type annotation`

    `let heart_eyed_cat = '😻';`

> Compound Types:
  * Tuple
    `let tup: (i32, f64, u8) = (500, 6.4, 1);`

    `let (x, y, z) = tup;`

    `let five_hundred = tup.0;`
  * Array
    `let a = [1, 2, 3, 4, 5];`

    `let a: [i32; 5] = [1, 2, 3, 4, 5];`

    `let a = [3; 5];`
  ### 3.3 Functions
  > Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples
  * Statements
    `fn main() {`

    `let y = 6;`

     `}   `
  * Expressions

    `let y = {`

    `    let x = 3;`

    `    x + 1`

    `};`
## 4. Understanding Ownership
  ### 4.1 What is Ownership?
  ### 4.2. References and Borrowing
  ### 4.3. The Slice Type