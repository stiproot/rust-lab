**Packages**: A Cargo feature that lets you build, test, and share crates
**Crates**: A tree of modules that produces a library or executable
**Modules and use**: Let you control the organization, scope, and privacy of paths
**Paths**: A way of naming an item, such as a struct, function, or module

src/main.rs and src/lib.rs are called crate roots. 
The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

A path can take two forms:
- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path. This is like starting a filesystem path with the .. syntax.