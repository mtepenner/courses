#Matthew Penner
#CS358
print(  "PROBLEM #1:\n\n",
        "Write a Python function currying(f)\n",
        "that takes an arbitrary binary function f\n",
        "as input and returns a curried version of f. For example:\n",
        "def add(x,y): return x + y # a normal function\n",
        "add(2,3) # => 5\n",
        "currying(add) # turning add into a curried func\n",
        "currying(add)(2)(3) # => 5\n",
        )
def currying(f): return lambda x: lambda y: f(x, y)
def add(x, y): return x + y

print("OUTPUT:")
print("Regular Function Output: ", add(2, 3))
print("Curried Function Output: ", currying(add)(2)(3))

print(  "\n\nPROBLEM #2:\n\n",
        "Write a Python function ktimes(f,k) that takes a function f and an integer k,\n",
        "and returns a function that applies the function f to its argument k times.\n",
        "For example:\n",
        "ktimes(f,0)(x) = x\n",
        "ktimes(f,2)(x) = f(f(x)) # this case is the same as twice(f)\n",
        "ktimes(f,5)(x) = f(f(f(f(f(x)))))\n",
        "If f is the incr function: def incr(x): return x+1, and x=5,\n",
        "then the above three calls would produce 5, 7, and 10, respectively.\n"
        )

def ktimes(f,k): return lambda x: x if k == 0 else f(ktimes(f, k-1)(x))    
def incr(x): return x+1

print("OUTPUT:")
print(ktimes(incr, 0)(5))  
print(ktimes(incr, 2)(5))  
print(ktimes(incr, 5)(5))


print(  "\n\nPROBLEM #3:\n\n",
        "Python has a map function, map(f,itr), which takes a function f and an iterable object itr.\n",
        "It applies f to each item of itr and returns the result as a map object:\n",
        ">>> map(lambda x:x+1, [1,2,3])\n",
        "<map object at 0x6fffffdab460>\n",
        "The map object is also an iterator, but its content is not directly visible.\n",
        "To see its content we can convert it into a visible object, such as a list:\n",
        ">>> x = map(lambda x:x+1, [1,2,3])\n",
        ">>> [i for i in x]\n",
        "[2,3,4]\n",
        "Your task is to write a new version of the map function, mymap(f,itr).\n",
        "It performs the same operations as the built-in version, but its result is explicitly visible.\n",
        "Specifically, it returns an object that matches the type of the parameter object itr.\n",
        "With this version, mapping f to a list returns a list, mapping f to a tuple returns a tuple, etc.\n",
        "Your function only needs to cover four types of iterable objects: list, tuple, set, and string.\n",
        "You may use the built-in map function to implement this new version, but it is not necessary.\n",
        "Here are some sample runs of this function:\n",
        ">>> mymap(lambda x:x+1, [1,2,3])\n",
        "[2,3,4]\n",
        ">>> mymap(lambda x:x+1, (1,2,3))\n",
        "(2,3,4)\n",
        ">>> mymap(lambda x:x+1, {1,2,3})\n",
        "{2,3,4}\n",
        ">>> mymap(str.upper, 'abc')\n",
        "'ABC'\n"
        )

print("OUTPUT:")

print(  "\n\nPROBLEM #4:\n\n",
        "Advanced compilers can perform optimizations on tail-recursive functions.\n",
        "The two C program files fib.c and fibT.c contain two implementations of the Fibonacci function,\n",
        "one (regular) recursive, one tail-recursive.\n\n",
        "(a) Compile the two programs using gcc with -O2 optimization setting, and save the assembly code:\n",
        "linux> gcc -O2 -S fib.c -o fib.s\n",
        "linux> gcc -O2 -S fibT.c -o fibT.s\n",
        "Inspect the two assembly programs. Are the function fib/fibT still being called recursively in them?\n\n",
        "(b) Now compile both programs to executable code, and do a simple timing comparison:\n",
        "linux> gcc -O2 -o fib fib.c\n",
        "linux> gcc -O2 -o fibT fibT.c\n",
        "linux> time ./fib 40\n",
        "linux> time ./fibT 40\n",
        "Do you see an execution-time difference between these two versions?\n"
        )

# RESPONSE:
# (a)
#
#
#
# (b)
#
#
#
