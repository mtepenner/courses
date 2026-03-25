# Data Structures and Algorithms Archive

## Description
This repository is a collection of fundamental data structures and algorithmic implementations written in C++. It includes a custom Binary Search Tree (`BST.cpp`), a standard Linear Linked List (`LLL.cpp`), an Array of Linked Lists (`ARR.cpp`), and an interactive Stack specifically built to manage book records (`stack.cpp`). Additionally, the archive features a standalone script implementing the Luhn algorithm used to mathematically validate credit card numbers (`luhn.cpp`).

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Contributing](#contributing)

## Installation 🛠️

To compile and run the programs in this repository, you will need a standard C++ compiler (such as GCC/g++). Because each file contains its own `main()` entry point, they must be compiled individually.

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>

```

2. **Compile individual files:**
For example, to compile the Luhn algorithm script:
```sh
g++ luhn.cpp -o luhn

```


3. **Run the compiled executable:**
```sh
./luhn

```



## Usage 🚀

* **Luhn Algorithm (`luhn.cpp`):**
Run the executable and enter a credit card number without spaces. The program will output whether the sequence is valid based on the Luhn check.
* **Stack Database (`stack.cpp`):**
Run the executable to launch an interactive menu. You can add (`push`) books by entering their title, author, publisher, and page number, or `pop` and `peek` at existing records.
* **Data Structure Tests (`BST.cpp`, `LLL.cpp`, `ARR.cpp`):**
Compile and run these files to observe the console output from the hard-coded test cases simulating node insertion, traversal, and reversal.

## Features ✨

* **Binary Search Tree:** Implements standard BST logic with recursive functions for insertion, removal, and in-order display.
* **Linear Linked Lists:** Provides implementations of singly-linked lists along with a recursive algorithm to reverse the list's node pointers in place.
* **Array-Based Stack:** Features a custom stack implemented using linked nodes that contain dynamically allocated arrays of `book` objects, safely managed with custom destructors and deep copy methods.
* **Luhn Validation:** Accurately performs modulo 10 checksums on numerical string inputs to validate credit card numbers according to the Luhn formula.

## Technologies Used 💻

* **Language:** C++

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.
