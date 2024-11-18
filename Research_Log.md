# Research log Aleksandar Dzudzevic

### Research Day 1: Oct.28 2024

- Finished Rustlings hashmaps
- Finished Rustlings Quiz2
- Finished Rustlings options
- Finished Rustlings errors
- Learned about
- Learned how to better use Option and Result enum and learned about wrapping values in Some()
- Started the undergrad exercise and passed first 2 tests.
- Committed and pushed on GitHub

### Research Day 2: Nov.1st 2024

- Undergrad exercises project methods for swapping, displaying, checking if the tiles are unique, performing move or multiple moves.
- learned about !write() and !format() macros and what {:>2} means in !write macro
- learned about dereferencing using \*
- Finished 2 chapters of Rustbook (Generics Ch 10, Writing automated Testing Ch 11)
- Finished 1 section of rustlings (generics)
- Committed and pushed on GitHub

### Research Day 3: Nov.4th 2024

- Finished rustlings traits section
- Finished rustlings lifetimes section
- Wrote test cases for test_validate_game_state, test_swap, test_perform_move, test_perform_moves methods
- Finished Quiz 3 of Rustings
- Finished testing section of rustlings
- Code Review
  - Helped method for finding the empty tile
  - Updated all_tiles_are unique method so that it also ensures that the tile doesn’t contain a value that isn’t allowed.
  - Fixed the col row mistake in my perform move method
- Worked on the from_str method
- Learned about lines() and collect() methods in Rust

### Research Day 4: Nov.8 2024

- Finished Rustlings section 18 iterators (iter, collect, )
- Fixed the from_str indexing issue I had with rows and columns
- Worked on the from_str method and learned about how to utilize the boolean return of the insert method for HashSets to either insert something that wasn’t in the HashSet originally or return that it already exists without using contains() to make code cleaner.
- Used the iter method I learned from rustlings in from_str when traversing rows and columns to make accessing tiles part cleaner
- The used collect method I learned from Rustlings exercises to store values extracted from the str inside of a vector.
- Added new test cases for from_str and found an error in my solution for cases of multiple empty tiles
- Fixed from_str method by implementing empty tile count and returning None if it exceeds 1
- Started with shortest_path method
- learned about filter_map functionality
-
- Committed and pushed on GitHub

### Research Day 5: Nov.11 2024

- Attempted the shortest_path full brute force approach
- Learned about the hashmap and deque implementation in Rust
- Learned more about BFS
- Changed my plan and implemented an idea that is still similar but uses BFS to gradually expand the states when it comes to how many moves were needed to perform it.
- Finished find_shortest_path
- Added four more test cases for testing_shortest path that include some edge case scenarios.
- Tested how does the execution time change based on the max_depth size of the possible boards I allow. 1 million states ~11.5 sec, 100000 in ~ 1 sec, 10000 in ~ 0.1 sec,
- Learned about the flat_map method and how to use it in nested structures when making items in iterator a new iterator and used this in iterators5 in rustlings
- Started working on chapeter 15: Smart Pointers, of the Rust Book
- started arc.rs, the first exercise in chapter 19 (smarter pointers)
- Pushed the progresss on Github

### Research Day 6: Nov 15th 2024

- Learned about patronus:Context application
- Learned how rust overcomes lack over garbage collecting and avoids empty chunks of memory (even how theortically one could make their own garbage collector by copying the expr vec with only relevant ones and thus getting rid off the ones which we want to deallocate)
- Learned about ctx.symbol, ctx.build,
- Started working on puzzle 2 circuits.rs
- Learned about nested expressions and their need for handling conditional updates or constraints symbolically (needed for incrementing only until the max value isn’t reached)
- Started working on build_counter_1 method
- Finished build_counter_1 method
- Added an extra soltuion to the original build-counter_1 to practice methods from context more (commente out since it is unnecessarily more complicated)
- Weekly group meeting
- Pushed progress on Github

## Research Day 7: Nov 18th 2024

- Started working on the build_counter_2 method
- Finished build_counter_2 method and tested it
- Started on the puzzle 15 method that uses circuits to simulate the tile game, and learned about unwrap_or method in rust
- Attempted the nested iter approach but it didnt work
- Started on the condition vector approach
- finished the condition push if statements
- Finished the method (left to write tests and see if it is correct)
