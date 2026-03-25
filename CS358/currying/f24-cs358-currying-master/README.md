# Functional Programming and Recursion Analysis

## Description
This project explores functional programming concepts and the performance impacts of recursive algorithm design. The repository includes a Python script that implements mathematical currying, successive function application, and custom iterable mapping. Additionally, it provides C implementations of the Fibonacci sequence using both standard recursion and tail-recursion to analyze how advanced compilers perform optimizations on tail-recursive functions.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Contributing](#contributing)

## Installation 🛠️

To utilize the scripts in this repository, you must have Python 3 and a C compiler (like GCC) installed on your system.

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>

```

2. **Compile the C programs:**
You can compile the standard and tail-recursive Fibonacci programs using GCC with the `-O2` optimization flag:
```sh
gcc -O2 -o fib fib.c
gcc -O2 -o fibT fibT.c

```


**

## Usage 🚀

* **Run the Python Functional Exercises:**
Execute the Python script to see the output of the currying and mapping functions:
```sh
python ex6sol.py

```


* **Execute the C Programs:**
Run the compiled binaries, providing an integer `n` as an argument to calculate the `n`th Fibonacci number. To compare execution times, use the `time` command:
```sh
time ./fib 40
time ./fibT 40

```


**

## Features ✨

* **Currying Implementation:** Includes a Python function `currying(f)` that converts an arbitrary binary function into a curried version.
* **Iterative Application:** Provides `ktimes(f,k)` which applies a given function `f` to an argument `k` times.
* **Standard Recursion:** `fib.c` uses traditional double-branch recursion to solve the Fibonacci sequence.
* **Tail Recursion:** `fibT.c` uses a nested helper function with accumulators to achieve a tail-recursive Fibonacci sequence.

## Technologies Used 💻

* **Languages:** Python 3, C
* **Compiler:** GCC (GNU Compiler Collection)

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

