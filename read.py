# import timeit
# from random import random
# print("hi")

def run():
    with open("words.txt") as f:
        for line in f.readlines():
            print(line, end='')


run()
# print("\n\nTIME", timeit.timeit(run, number=1))
