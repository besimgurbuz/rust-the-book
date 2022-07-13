### The Stack and the Heap

Both the stack and the heap are parts of memory available to your code to use at runtime, but thay are structured in different ways. The stack stores values in the order it gets them and removes the valus in the opposite order. This is reffered to as **last in, first out**. Think of a stack of plates: when you add moore plates, you put them on top of the pile, and when you need a plate you take one of the top. Adding or removing plates from the middle or bottom wouldn''t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an uknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called **allocating on the heap** and is sometimes abbreviated as just **allocating** (**pushing values onto the stack is not considered allocating**). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want too actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you've been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for next allocation.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer too get there. Continuing the analoogy, consider a server at a restaurant taking orders from many tables. It's most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that's close to other data (as it is on the stack) rather than farther away (as it can be on the heap)

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values, get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run oout of space are all problems that ownership adresses.

### Ownership rules

- Each value in Rust has an owner.
- There can only one owner at a time.
- When the owner goes out of scope, the value will be dropped.
