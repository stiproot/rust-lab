Each signed variant can store numbers from -(2**(n - 1)) to 2**(n - 1) - 1 inclusive, where n is the number of bits that variant uses. 

So an i8 can store numbers from -(2**7) to 2**7 - 1, which equals -128 to 127. 
Unsigned variants can store numbers from 0 to 2**n - 1, so a u8 can store numbers from 0 to 2**8 - 1, which equals 0 to 255.

Integer Types
An integer is a number without a fractional component. We used one integer type in Chapter 2, the u32 type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

Table 3-1: Integer Types in Rust

Length	      Signed	    Unsigned
8-bit	        i8	        u8
16-bit	      i16	        u16
32-bit	      i32	        u32
64-bit	      i64	        u64
128-bit	      i128	      u128
arch	        isize	      usize

# Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.