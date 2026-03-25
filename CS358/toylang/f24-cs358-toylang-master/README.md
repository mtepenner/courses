# ToyLang Interpreter

## Description
This repository contains a custom interpreter for "ToyLang", an imperative programming language featuring lambda functions and closures. Built using Python and the Lark parsing library, the project accurately models nested scopes and variable environments. The base implementation (`toylang.py`) parses and evaluates block statements and anonymous functions, while the extended version (`toylang2.py`) introduces full control flow with `if` and `while` loops, as well as formal function declarations (`def`).

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Contributing](#contributing)
- [License](#license)

## Installation 🛠️

To run this project, you will need Python 3 and the `lark` parsing toolkit.

1. **Clone the repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>

```

2. **Install dependencies:**
Install the `lark` package via pip:
```sh
pip install lark

```



## Usage 🚀

The interpreters are designed to read source code from standard input (stdin). You can test the language using the provided `.toy` and `.toy2` files.

* **Run the base ToyLang interpreter:**
Evaluates variable assignments, basic math, and lambda closures.
```sh
python toylang.py < tst/add1.toy

```


**
* **Run the extended ToyLang2 interpreter:**
Evaluates programs with full function definitions and standard control flow.
```sh
python toylang2.py < tst/add2.toy2

```


**

## Features ✨

* **First-Class Functions:** Natively parses and evaluates lambda expressions and formal function declarations, allowing functions to be assigned to variables and passed around.
* **Closures:** Implements a `Closure` class that retains access to the local scope and variable environment present at the time the function was defined.
* **Environment Management:** Uses a custom `Env` dictionary stack to handle variable scoping, scoping resolution, and deep copying during nested function calls.
* **Control Flow Operations:** `toylang2.py` adds AST nodes and evaluation rules for `if/else` statements and `while` loops.

## Technologies Used 💻

* **Language:** Python 3
* **Parsing Toolkit:** Lark (using LALR parsing)

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## License 📄

[MIT License] 

