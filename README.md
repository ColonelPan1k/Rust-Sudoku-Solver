# Rust Sudoku Solver
This is a rewrite of my sudoku solver project into Rust.  I used my original C code as a template to work from.  Overall, it was not a very hard project to finish and
only took a few hours to complete.  I'm excited to dig more into Rust when I get the chance since I had a pretty fun time learning about it while working on this. 

I'm sure there are many better more "rustic" ways of writing this code (in particular, the whole impl block seems a bit odd, but that might just be because it feels like I'm writing OOP code with C structs).  I'll improve this as I learn more but overall I don't think it's too bad for a first real project in Rust.

# How to Use
This has no dependencies outside of the standard rust library. All you've got to do is clone the repo, use "rustc main.rs," then "./main.exe {your text file}" and it'll
run.  The sudoku puzzle must be formatted like the provided example puzzles. The whitespace between the numbers is optional, but it makes it a bit easier to read when
inputting the numbers from a text file.  
