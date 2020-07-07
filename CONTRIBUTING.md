# Contributing to Bob

## Compiling Bob
 
### You'll need - 
<ol>
<li> g++ - run 
		apt install g++
for Ubuntu based Linux distros.
</li>
<li> rust - https://www.rust-lang.org/tools/install. </li>

### Run
		cargo build 

## Compiling a bob script
		cargo run name_of_bob_file.bob

## Important tips
<ol>
<li> Docs may be built and opened on your browser with - 
		cargo doc --open
</li>
<li> Comment out the last statementi - 
		fs::remove_file("output.cpp").expect("Bob couldn't delete his temporary file");
from main.rs to be able to see the output.cpp file generated. This may be used for debugging.
</li>
<li> Run time errors must always be avoided, and unavoidable errors included in the Errors enum in lib.rs, and managed by the raise function. For example, refer the implementaion in C++ of the read function, where the data type of the input is verified.
</li>
