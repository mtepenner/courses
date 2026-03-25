# Lark Parsers: Boolean and Let Expressions

**Author:** Matthew Penner  
**Date:** 10/24/2024  

## 📖 Overview
This repository contains two custom interpreters built in Python using the `lark` parsing toolkit. It includes a boolean expression evaluator (`boolex.py`) and a mathematical evaluator that supports variable bindings (`letex.py`). Both scripts have been fully tested and operate perfectly according to the intended test cases.

## 💻 Included Files & Features

### 1. Boolex (`boolex.py`)
This script parses and evaluates boolean logic expressions containing `True`, `False`, `and`, `or`, and `not` operators.

When an expression like `(True or not False) and True` is entered, the program outputs:
* **AST (Abstract Syntax Tree):** A visual representation of the parsed hierarchy.
* **`tree.Eval()`:** The final evaluated boolean result (`True` or `False`).
* **`tree.toList()`:** The syntax tree converted into a nested Python list format.
* **`strForm()`:** A string representing the list in a LISP-like format (e.g., `(and (or True (not False)) True)`).

### 2. Letex (`letex.py`)
This script parses and evaluates mathematical expressions using local variable bindings (`let ... in ...`). It supports basic arithmetic including addition, subtraction, multiplication, and division.

When an expression like `let x=1 in x+1` is entered, the program outputs:
* **AST:** The visual hierarchy of the let-binding and mathematical operations.
* **`tree.Eval()`:** The evaluated numerical result calculated using a stack-based environment to track variable states.

## 🚀 Usage

### Prerequisites
* Python 3.x
* The `lark` library

You can install the required library using pip:
```bash
pip install lark
