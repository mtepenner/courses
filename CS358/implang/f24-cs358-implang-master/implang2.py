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
       | "for" ID "in" range stmt  -> forstmt
  
  range: "[" expr ".." expr "]" -> rangestmt

  ?expr: aexpr "<" aexpr         -> lt
       | aexpr "==" aexpr        -> eq
       | aexpr
  ?aexpr: aexpr "+" term  -> add
       | aexpr "-" term  -> sub
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
        #print(f"DEBUG: Opening a new scope. Current state before opening:\n")
        #self.display("Before opening scope:")
        self.prev.append(self.copy())
        self.clear()
        #print("DEBUG: New scope opened.")
    
    def closeScope(self):
        if not self.prev:
            raise Exception("No scope to close")
        #print(f"DEBUG: Closing scope. Current state before closing:\n")
        #self.display("Before closing scope:")
        restored_scope = self.prev.pop()
        self.clear()
        self.update(restored_scope)
        #print(f"DEBUG: Scope closed. Restored state:\n")
        #self.display("After closing scope:")
    
    def extend(self,x,v): 
        if x in self:
            raise Exception("Variable '{x}' already defined")
        #print(f"DEBUG: Defining new variable '{x}' with value '{v}'.")
        self[x] = v
        #self.display("State after variable addition:")
    
    def lookup(self,x): 
        #print(f"DEBUG: Looking up variable '{x}'.")
        if x in self:
            #print(f"DEBUG: Found '{x}' in the current scope with value {self[x]}.")
            return self[x]
        else:
            for env in self.prev:
                if x in env: 
                    #print(f"DEBUG: Found '{x}' in a previous scope with value {env[x]}.")
                    return env[x]
        raise Exception("Variable '{x}' is undefined")
    
    def update_self(self,x,v):
        #print(f"DEBUG: Updating variable '{x}' to new value '{v}'.")
        if x in self:
            self[x] = v
            #self.display("State after update in current scope:")
            return
        for env in self.prev:
            if x in env:
                env[x] = v
                #self.display("State after update in a previous scope:")
                return
        raise Exception("Variable '{x}' is undefined")

    def display(self, msg="Current Environment:"):
        print(f"{msg}\nActive Scope: {self}\nPrevious Scopes: {self.prev}\n")
        #print(msg, self, self.prev)

env = Env()
# Interpreter
#
@v_args(inline=True)
class Eval(Interpreter):
    def num(self, val):
        #print(f"DEBUG: Converting number -> {val}")
        return int(val)

    def visit(self, tree):
        #print(f"DEBUG: Visiting tree -> {tree}")
        return super().visit(tree)

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
    
    def lt(self, left, right):
        return self.visit(left) < self.visit(right)

    def eq(self, left, right):
        return self.visit(left) == self.visit(right)

    def decl(self, name, value):
        evaluated_value = self.visit(value)
        env.extend(name, evaluated_value)
        #print(env)
    
    def assign(self, name, value):
        evaluated_value = self.visit(value)
        env.update_self(name, evaluated_value)
        #print(env)
    
    def prstmt(self, value):
        print(self.visit(value))

    def block(self, *stmts):
        env.openScope()
        #print("DEBUG: Entering block.")
        #env.display("Environment before executing block:")
        for stmt in stmts:
            self.visit(stmt)
        #print("DEBUG: Exiting block.")
        #env.display("Environment after executing block:")
        env.closeScope()

    def ifstmt(self, condition, true_stmt, false_stmt=None):
        if self.visit(condition):
            self.visit(true_stmt)
        elif false_stmt is not None:
            self.visit(false_stmt)

    def whstmt(self, condition, body):
        while self.visit(condition):
            self.visit(body)
    
    def rangestmt(self, start, end):
        #print(f"DEBUG: Evaluating range -> start: {start}, end: {end}")
        start_val = self.visit(start)
        end_val = self.visit(end)
        return range(start_val, end_val)
    
    def forstmt(self, var, ranges, stmt):
        #print(f"DEBUG: Entering 'for' loop with range: {range}")
        visitr = self.visit(ranges)
        
        env.openScope()
        env.extend(var, stmt)
        for i in visitr:
            #print(f"DEBUG: Iteration with {var} = {i}")
            env.update_self(var, i)
            
            #env.display("Environment during loop iteration:")
            self.visit(stmt)
    
        env.closeScope()
        #print("DEBUG: Exiting 'for' loop.")
        #env.display("Environment after loop:")

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
