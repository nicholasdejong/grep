# A Grep Clone

This is a grep clone in Rust. In the future, it might also be coded in C.
Grep is used to search files or input that matches specific patterns. These patterns are really just regular expressions. Right now, I am working on parsing the regular expressions.

Instead of using a pre-existing regular expression package, I decided to create my own Regex (regular expression) parser, for a few reasons:
- To get a better understanding of things like compilers (main reason)
- To deepen my knowledge on Regex
- To make a program without any dependencies (forces better code organization)

I haven't yet looked at the full capabilities of Grep, but for now I am focusing on the Regex parser. The plan is to make a basic Regex parser that interprets a small subset of Regex's abilities, and then use that parser to create expressions that match "regular expressions". Almost like self-compilation. I will expand on this section soon.

Please feel free to create an Issue or submit a pull request if you find any potential mistake or error in my program. I am open to collaboration and could definitely use your help!
