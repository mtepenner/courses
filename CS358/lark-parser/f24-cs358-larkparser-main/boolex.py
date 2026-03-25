# Matthew Penner
#


from lark import Lark, v_args
from lark.visitors import Interpreter


# 1. Grammar
#
grammar = """
    ?start: orex
    ?orex: orex "or" andex -> orop
        | andex
    ?andex: andex "and" atom -> andop
        | atom
    ?atom: "not" atom -> notop
        | "(" orex ")" 
        | "True" -> truev
        | "False" -> falsev
    %import common.WS
    %ignore WS
"""
# Parser
#
# E.g. (True or not False) and True
#      => andop  
#           orop
#             truev
#             notop
#               falsev
#           truev
#
parser = Lark(grammar)

# 2. Interpreter
#
# E.g. (for the above example)
#      => True
#

@v_args(inline=True)
class Eval(Interpreter):
    def truev(self):
        return True
    def falsev(self):
        return False
    def notop(self, a):
        return not self.visit(a)
    def andop(self, a, b):
        return self.visit(a) and self.visit(b)
    def orop(self, a, b):
        return self.visit(a) or self.visit(b)


# 3. Convert the AST to a list form
#
# E.g. (for the above example)
#      => ['and', ['or', 'True', ['not', 'False']], 'True']
#
@v_args(inline=True)
class toList(Interpreter):
    def truev(self):
        return 'True'
    def falsev(self):
        return 'False'
    def notop(self, a):
        return ['not', self.visit(a)]
    def andop(self, a, b):
        return ['and', self.visit(a), self.visit(b)]
    def orop(self, a, b):
        return ['or', self.visit(a), self.visit(b)]

# 4. Convert a nested list to a string form
#
# E.g. (for the above example)
#      => (and (or True (not False)) True)
#
def strForm(lst):
    if isinstance(lst, list):
        return "(" + " ".join(strForm(item) for item in lst) + ")"
    else:
        return str(lst)
    
def main():
    while True:
        try:
            expr = input("Enter a bool expr: ")
            tree = parser.parse(expr)
            lst = toList().visit(tree)
            print(tree.pretty(), end="")
            print("tree.Eval() =", Eval().visit(tree))
            print("tree.toList() =", lst)
            print("strForm() =", strForm(lst))
            print()
        except EOFError:
            break
        except Exception as e:
            print("***", e)

if __name__ == '__main__':
    main()
