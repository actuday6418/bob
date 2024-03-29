# Bob
Bob's a young esoteric language (Generates C++ code and then compiles it with g++). The idea is to have an English like syntax that doesn't compromise much of the flexibility of the language's use, while being fast and compiled. Fun and impractical.

# Hello World
		Bob write "Hello world!!" .

# Issues
The currently open issues are things I haven't put time in to see how to implement in Rust. They're still valuable things a new comer to the code base and Rust could easily do.

# Dependencies
1. Install rust - https://www.rust-lang.org/tools/install.
2. Install g++ - apt install g++ (for Ubuntu based Linux distros)

# Running debug
1. cargo run example.bob (assuming example.bob is in the working directory)
2. Use ./app for running the compiled app.
3. Access docs through cargo doc --open

# Setting up an environment on Android 
1. install and setup termux or another terminal emulator for android. 
2. run the script 
		curl https://its-pointless.github.io/setup-pointless-repo.sh | bash
3. Run
		pkg install rust
4. Clone and build Bob as normal!
5. Follow the same old instructions to setup the Vim plugin.
6. You can get the release for armv7 directly from this repository, instead of having to install ruust from an unreliable community maintained repository .

# Running release
		./bob example.bob

# Examples
Check out the files in the examples directory.

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

# TODO
Things to be implemented are mentioned here. (This is a short term roadmap)

# Vim plugin
A vim plugin is available for Bob that currently supports text highlighting and not perfectly refined autocomplete. Repo is at https://github.com/actuday6418/bob-vim
