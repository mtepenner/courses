# ImpLang Interpreter

## Description
This project implements an interpreter for "ImpLang", a custom imperative programming language, built using Python and the Lark parsing library. The language supports complex variable environments, allowing for nested scopes, variable declarations, and standard control flow mechanisms. The base interpreter is provided in `implang.py`, while `implang2.py` extends the language grammar to handle equality and relational operators (`<`, `==`), as well as `for` loop iteration over ranges. 

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Contributing](#contributing)

## Installation 🛠️

To run this project, you will need Python 3 and the `lark` parsing toolkit.

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>

```

2. **Install dependencies:**
Install `lark` using pip:
```sh
pip install lark

```



## Usage 🚀

The scripts use `sys.stdin.read()` to parse multi-line input files containing ImpLang code. You can pipe custom `.imp` scripts into the interpreters directly from the command line:

* **Run the base interpreter:**
```sh
python implang.py < my_script.imp

```


* **Run the extended interpreter (Supports for-loops and relational operators):**
```sh
python implang2.py < fac.imp

```



The repository includes test programs such as `fac.imp` (calculates factorials using a `while` loop), `prime.imp` (checks for prime numbers using nested `while` loops), and `sum.imp` (calculates sums using a `for` loop).

## Features ✨

* **Nested Scopes:** Uses a custom `Env` class with an `openScope` and `closeScope` method to securely manage block-level variables and resolve shadowed identifiers.
* **Control Flow Execution:** Supports standard structures like `if` / `else` branches and `while` loops, alongside newly implemented `for ID in [start..end]` loops.
* **Grammar Parsing:** Accurately builds ASTs using an LALR parser to easily resolve common language problems like the "dangling else" scenario.
* **Arithmetic and Relational Evaluation:** Resolves standard math computations and boolean operators like `<` and `==` directly inside the AST visitor.

## Technologies Used 💻

* **Language:** Python 3
* **Parsing Library:** Lark

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

