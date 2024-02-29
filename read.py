import timeit
from random import random
print("hi")

def run():
    with open("words.txt") as f:
        print(f.readlines())        # for line in f.readlines():
        #     print(f"{line} {random()}", end='')
        #
print("\n\nTIME", timeit.timeit(run, number=1))
