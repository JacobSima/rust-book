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

