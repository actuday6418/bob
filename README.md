# Bob
Bob's a new language (Generates C++ code and then compiles it with g++). Say hello to Bob.

# Hello World
Bob write "Hello world!!"

# Issues
The currently open issues are things I'm not sure how to implement in Rust, and unwilling to devote time for right now.

# Dependencies
1. Install rust - https://www.rust-lang.org/tools/install.
2. Install g++ - apt install g++ (for Ubuntu based Linux distros)

# Run
1. cargo run main.bob (assuming main.bob is in the current directory)
2. Use ./app for running the compiled app.
3. Access docs through cargo doc --open

# Example
Check out the file main.bob in the root of this repository.

# Overview of the syntax
1. Bob statements are called sentences.
2. Each sentence is terminated with a period.
3. Each sentence begins with calling Bob.
4. Function names in Bob are called verbs.
5. A verb HAS to follow the Bob call, as it is put.
6. Sentences may contain expressions (under development), which are evaluated into arguments for each verb.
7. Comments in Bob go in between brackets.

# Implemented Verbs
1. write - writes to stdout.
2. write_line - writes to stdout after appending a newline to argument string.
3. read - reads from stdin.
4. let - declares a variable.

