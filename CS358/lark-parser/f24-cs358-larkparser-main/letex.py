# Matthew Penner
#


from lark import Lark, v_args
from lark.visitors import Interpreter

# 1. Grammar
#
grammar = """
    ?start: expr0
    ?expr0: "let" ID "=" expr0 "in" expr0  -> let
        | expr
    ?expr: expr "+" term   -> add
        | expr "-" term    -> sub
        | term
    ?term: term "*" atom   -> mul
        | term "/" atom    -> div
        | atom
    ?atom: "(" expr0 ")"   
        | ID               -> var
        | NUM              -> num
    %import common.NUMBER -> NUM
    %import common.CNAME -> ID
    %import common.WS
    %ignore WS
"""

# Parser
#
# E.g. let x=1 in x+1
#      => let
#           x
#           num   1
#           add
#             var x
#             num 1
#
parser = Lark(grammar)


# 2. Variable environment
#
class Env(dict):
    def extend(self,x,v): # use a stack to keep all values
        if x in self: # if not first binding for x,
            self[x].insert(0,v) # push new value onto stack
        else: # otherwise
            self[x] = [v] # create a new stack for x        
    def lookup(self,x):
        vals = super().get(x)
        if not vals:
            raise Exception("Undefined variable: " + x)
        return vals[0] # return top item from x's stack
    def retract(self,x):
        assert x in self, "Undefined variable: " + x
        self[x].pop(0) # pop off x's latest value 
env = Env()

# 3. Interpreter
#
# E.g. (for the above example)
#      => 2
#
@v_args(inline=True)
class Eval(Interpreter):
    def let(self, x, exp, body):
        val = self.visit(exp)
        env.extend(str(x), val)
        result = self.visit(body)
        env.retract(str(x))
        return result
    def add(self, left, right):
        return self.visit(left) + self.visit(right)
    def sub(self, left, right):
        return self.visit(left) - self.visit(right)
    def mul(self, left, right):
        return self.visit(left) * self.visit(right)
    def div(self, left, right):
        return self.visit(left) / self.visit(right) 
    def var(self, var_name):
        return env.lookup(str(var_name))
    def num(self, num_value):
        return int(num_value)

def main():
    while True:
        try:
            expr = input("Enter a let expr: ")
            tree = parser.parse(expr)
            print(tree.pretty(), end="")
            print("tree.Eval() =", Eval().visit(tree))
            print()
        except EOFError:
            break
        except Exception as e:
            print("***", e)

if __name__ == '__main__':
    main()
