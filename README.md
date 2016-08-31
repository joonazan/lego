# LeGo translates Go programs to blocks and vice versa

*Write Go code that is guaranteed to compile!*

Lego extends Google's Blockly with statically typed blocks and scoped variables. Lego can open a Go source file and write back changes.

Lego mirrors Go's syntax as closely as possible, replacing symbols with words. Syntax is unified when possible, like for nested vs. top-level functions. Code that is not understood yet is transformed into a Go source code block.