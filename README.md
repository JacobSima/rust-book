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
  * ![Integers](/Programming_Concepts/DataTypes/integer.PNG)
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
  > Ownership is a set of rules that governs how a Rust program manages memory

  * Ownership Rules

  > Each value in Rust has an owner.
  > There can only be one owner at a time.
  > When the owner goes out of scope, the value will be dropped.

  * Variable Scope

    `     {                      // s is not valid here, it’s not yet declared`

    `      let s = "hello";   // s is valid from this point forward`

    `      // do stuff with s`

    `     }  `  

    > When s comes into scope, it is valid.

    > It remains valid until it goes out of scope.  

    `let s = String::from("hello");  `

    The double colon : : operator allows us to namespace this particular from function under the String  

    `let mut s = String::from("hello");`

    `s.push_str(", world!"); // push_str() appends a literal to a String`

    `println!("{}", s); // This will print `hello, world!`   `   

    This kind of string can be mutated 
  
  #### 4.1.1 Memory and Allocation

  >   With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents

  >  When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable

  > the memory is automatically returned once the variable that owns it goes out of scope.

  > No garbage collector needed.

  * **Stack-Only Data: Copy**

    `let x = 5;`

    `let y = x`

    x = 5 and y = 5. The value of x will be copy at y. x owns the value of 5 and make a copy to y.

    This is possible because integer is primitive(scalar data type) 

    Which is stored at the Stack during program execution wihtin its repective stack-frame.

    * All the integer types, such as u32.

      The Boolean type, bool, with values true and false.

      All the floating point types, such as f64.

      The character type, char.
      
      Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

  * **Ways Variables and Data Interact: Move**

    `let s1 = String::from("hello");`

    `let s2 = s1;`

    This looks very similar, so we might assume that the way it works would be the same : that is, the second line would make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens. But it is not the case.

    s1 is stored on the Stack by reference which point to a value address stored in the Heap

    ![Integers](/Ownership/definition/stack-heap-1.PNG)

    The length is how much memory, in bytes, the contents of the String is currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator


    When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to

    ![Integers](/Ownership/definition/stack-heap-2.PNG)

    both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

    To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope

    If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2. So what actually happens is shown below 

    ![Integers](/Ownership/definition/stack-heap-3.PNG)

    NB: s1 will be moved to s2 and go out of scope. 

    `let s1 = String::from("hello");`

    `let s2 = s1;`

    `println!("{s1}");     // will result to an error as s1 is moved to s2, s1 is out of scope`    

    `println!("{s2}");     // this will hold the value of s1, moved here`

    `That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done ` 
  
  * **Ways Variables and Data Interact: Clone**

      If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone

      `let s1 = String::from("hello"); `     

      `let s2 = s1.clone(); `

      `println!("s1 = {}, s2 = {}", s1, s2);  // s1 and s2 still in scope `

      `NB: Very expensive operation when the data stored in the Heap is very large`

      ![Integers](/Ownership/definition/stack-heap-4.PNG)

      * which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large*

      Mutating s1 or s2 would not affect the other, as they are two different data in the heap, each having its own reference object in the Stack. 
  
  #### 4.1.2 Ownership and Functions

  >   The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

  >   The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

  >   While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

  >   But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called references

  ### 4.2. References and Borrowing
  > A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

  ![Integers](/Ownership/reference_borrowing/reference-borrow.PNG)

  * For more please check folder Common Programming Understanding Ownership, References and Borrowing

## 7. Managing Growing Projects with Packages, Crates, and Modules

* Packages: A Cargo feature that lets you build, test, and share crates

* Crates: A tree of modules that produces a library or executable

* Modules and use: Let you control the organization, scope, and privacy of paths

* Paths: A way of naming an item, such as a struct, function, or module

### 7.1. Packages and Crates

* Crate:  is the smallest amount of code that the Rust compiler considers at a time

  >    Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called main that defines what happens when the executable runs

  >   Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand. Most of the time when Rustaceans say “crate”, they mean library crate.

  >   The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate

* Package: is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates

  > The Cargo package also contains a library crate that the binary crate depends on

  > A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

* Create a Cargo package:

    cargo new my-project = create package within new directory my-project

    cargo init = create package in the current directory

    `$ cargo new my-project`

      `Created binary (application) my-project package`

    `$ ls my-project`

    `Cargo.toml`

    `src`

  ` $ ls my-project/src`
  
    `main.rs`

  src/main.rs will be for binary crate by convention

  src/lib.rs willbe for library crate convention

  If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library,

  **NB : A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.**


### 7.2. Module

  * Start from the crate root

    >   When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

  * Declaring modules

      In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:

      >  Inline, within curly brackets that replace the semicolon following mod garden

      >  In the file src/garden.rs
  
      >  In the file src/garden/mod.rs
  
  * Declaring submodules

      In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places

      >  Inline, directly following mod vegetables, within curly brackets instead of the semicolon

      >  In the file src/garden/vegetables.rs

      >  In the file src/garden/vegetables/mod.rs
  
  * Paths to code in modules

      Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate

      `crate::garden::vegetables::Asparagus`
  
  * Private vs public

    > Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations

  * The use keyword

    >  Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths

      `use crate::garden::vegetables::Asparagus;`

      `pub mod garden;`

      `fn main() {`

      `let plant = Asparagus {};`

      `println!("I'm growing {:?}!", plant);`

      `}`
  * Grouping Related Code in Modules

    `mod front_of_house {`

    `mod hosting {`

      `fn add_to_waitlist() {}`

    `}`

    `mod serving {`

      `fn take_order() {}`

      ` fn serve_order() {}`

    `}`

    `}`
### 7.3. Paths for Referring to an Item in the Module Tree

  * An absolute path

    is the full path starting from a crate root, it starts with the literal crate
  
  * A relative path

    starts from the current module and uses self, super, or an identifier in the current module

  
  Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

  `mod front_of_house {`

    `pub mod hosting {`

      `pub fn add_to_waitlist() {}`

    `}`

  `}`

    // use crate::front_of_house::hosting;        // Creating Idiomatic use Paths
    // use std::io::Result as IoResult;            // Providing New Names with the as Keyword
    // pub use crate::front_of_house::hosting;    // Re-exporting Names with pub use

  `pub fn eat_at_restaurant() {`  

    `crate::front_of_house::hosting::add_to_waitlist();  // Absolute path`

    `front_of_house::hosting::add_to_waitlist();         // Relative path`

    `super::front_of_house::hosting::add_to_waitlist()   //  Starting Relative Paths with super`

  `}`


* Using External Packages

  `use rand::Rng;`

  `fn main() {`

    `let secret_number = rand::thread_rng().gen_range(1..=100);`

  `}`
  
* Using Nested Paths to Clean Up Large use Lists

  `use std::{cmp::Ordering, io};`

  `use std::io;`

  `use std::io::Write;`
  
  `use std::io::{self, Write};    // This line brings std::io and std::io::Write into scope.`      

* The Glob Operator

  `use std::collections::*;`



  