# Matthew Penner
#

# CS358 Fall'24 Assignment 3 (Part A)
#
# ImpLang - a simple imperative language with nested scopes

# ImpLang - an imperative language
#
#   prog -> stmt
#
#   stmt -> "var" ID "=" expr
#         | ID "=" expr 
#         | "if" "(" expr ")" stmt ["else" stmt]
#         | "while" "(" expr ")" stmt
#         | "print" "(" expr ")"
#         | "{" stmt (";" stmt)* "}" 
#
#   expr -> expr "+" term
#         | expr "-" term
#         | term         
#
#   term -> term "*" atom
#         | term "/" atom
#         | atom
#
#   atom: "(" expr ")"
#         | ID
#         | NUM
#
from lark import Lark, v_args
from lark.visitors import Interpreter

grammar = """
  ?start: stmt

   stmt: "var" ID "=" expr         -> decl
       | ID "=" expr               -> assign
       | "if" "(" expr ")" stmt ["else" stmt] -> ifstmt
       | "while" "(" expr ")" stmt -> whstmt
       | "print" "(" expr ")"      -> prstmt
       | "{" stmt (";" stmt)* "}"  -> block      

  ?expr: expr "+" term  -> add
       | expr "-" term  -> sub
       | term         

  ?term: term "*" atom  -> mul
       | term "/" atom  -> div
       | atom

  ?atom: "(" expr ")"
       | ID             -> var
       | NUM            -> num

  %import common.WORD   -> ID
  %import common.INT    -> NUM
  %import common.WS
  %ignore WS
"""

# With an 'lalr' parser, Lark handles the 'dangling else' 
# case correctly.
parser = Lark(grammar, parser='lalr')

debug = False

# Variable environment
#
class Env(dict):
    def __init__(self):
        super().__init__()
        self.prev = []  
    def openScope(self):
        self.prev.append(self.copy())
        self.clear()
    def closeScope(self):
        if not self.prev:
            raise Exception("No scope to close")
        restored_scope = self.prev.pop()
        self.clear()
        self.update(restored_scope)
    def extend(self,x,v): 
        if x in self:
            raise Exception("Variable '{x}' already defined")
        self[x] = v
    def lookup(self,x): 
        if x in self:
            return self[x]
        for env in self.prev:
            if x in env: return env[x]
        raise Exception("Variable '{x}' is undefined")
    def update_self(self,x,v):
        if x in self:
            self[x] = v
            return
        for env in self.prev:
            if x in env:
                env[x] = v
                return
        raise Exception("Variable '{x}' is undefined")
    def display(self, msg):
        print(msg, self, self.prev)

env = Env()
# Interpreter
#
@v_args(inline=True)
class Eval(Interpreter):
    

    def num(self, val): 
        return int(val)

    def var(self, name):
        return env.lookup(name)

    def add(self, left, right):
        return self.visit(left) + self.visit(right)

    def sub(self, left, right):
        return self.visit(left) - self.visit(right)

    def mul(self, left, right):
        return self.visit(left) * self.visit(right)

    def div(self, left, right):
        if self.visit(right) == 0:
            raise ZeroDivisionError("Division by zero not allowed")
        return self.visit(left) // self.visit(right)
    
    def decl(self, name, value):
        evaluated_value = self.visit(value)
        env.extend(name, evaluated_value)

    def assign(self, name, value):
        evaluated_value = self.visit(value)
        env.update_self(name, value)

    def prstmt(self, value):
        print(self.visit(value))

    def block(self, *stmts):
        env.openScope()
        for stmt in stmts:
            self.visit(stmt)
        env.closeScope()

    def ifstmt(self, condition, true_stmt, false_stmt=None):
        if condition:
            self.visit(true_stmt)
        elif false_stmt is not None:
            self.visit(false_stmt)

    def whstmt(self, condition, body):
        while condition:
            self.visit(body)

# A new input routine - sys.stdin.read() 
# - It allows source program be written in multiple lines
#
import sys
def main():
    try:
        prog = sys.stdin.read()
        tree = parser.parse(prog)
        print(prog)
        Eval().visit(tree)
    except Exception as e:
        print(e)

if __name__ == "__main__":
    main()
