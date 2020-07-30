# Bob
Bob's a new language (Generates C++ code and then compiles it with g++). Say hello to Bob.

# Hello World
		Bob write "Hello world!!".

# Issues
The currently open issues are things I haven't put time in to see how to implement in Rust. They're still valuable things a new comer to the code base and Rust could easily do.

# Dependencies
1. Install rust - https://www.rust-lang.org/tools/install.
2. Install g++ - apt install g++ (for Ubuntu based Linux distros)

# Running debug
1. cargo run example.bob (assuming example.bob is in the working directory)
2. Use ./app for running the compiled app.
3. Access docs through cargo doc --open

# Running release
./bob example.bob

# Example
Check out the file main.bob in the root of this repository.

# Overview of the syntax
1. Bob statements are called sentences.
2. Each sentence is terminated with a period.
3. Each sentence begins with calling Bob.
4. Every token in Bob must be separated by whitespace.
5. Function names in Bob are called verbs.
6. A verb HAS to follow the Bob call, as it is put.
7. Sentences may contain expressions, which are evaluated into arguments for each verb. Verbs that return a value may be used as expressions(to be implemented).
8. Comments in Bob go in between brackets.
9. Currently verbs may have only one argument in Bob. This will be changed to function name argument1 argument name argument2 ..
   eg. Bob write line "Name: " + Martha.
10. Variables in Bob are called Identities. Expressions like "sqws" or 5 are called literals, 5 being a number literal and "sqws" being a string literal.

# Implemented Verbs
1. write - writes to stdout.
2. write line - writes to stdout after appending a newline to argument string.
3. read - reads from stdin.
4. let - declares a variable.

# Implemented Types
1. number
2. decimal
3. string

# Expressions and operators
1. Arithmetic operators like +,-,/,% may be used. Their written counterparts like plus, minus, modulo, etc, may also be used.
2. strings may be concatenated with +. String identities and literals may be used together for this. Currently other types cannot be used along string literals and identities, but that'll be implemented in the future.

# Vim plugin
A vim plugin is available for Bob that currently supports text highlighting, and will be able to do autocompletion in the near future as well. Repo is at https://github.com/actuday6418/bob-vim
