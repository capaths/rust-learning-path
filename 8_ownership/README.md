# Owneship

- Feature that allows memory safety without a garbage collector.
- Memory is managed through a system of ownership with a set of rules that
the compiler checks.
- If any rule is broken, the program will not compile.

## The stack and the heap

- The stack and the heap are parts of memory available to your code to use at runtime
- **Stack**:
  - Stores values in the order it gets them and removes the values in the opposite order (Last-In First-Out or LIFO).
- **Heap**:
  - You request memory to a memory allocator, which looks for a free space and allocates it to you.
  - The address of the allocated memory is returned to you as a **pointer**.
  - The **pointer** might be stored in the stack.

Now, about advantages and disadvantages:

- Pushing to the **stack** is **faster** thant allocating to the **heap**.
- Accessing the **stack** is **faster** than accessing the **heap**.
- Data in the **stack** must have a known fixed size, otherwise it must be stored on the **heap**.

## Da' Rules

```
1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped
```

