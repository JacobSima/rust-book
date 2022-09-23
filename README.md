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

      *which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large*
  
  #### 4.1.2 Ownership and Functions

  ### 4.2. References and Borrowing
  ### 4.3. The Slice Type