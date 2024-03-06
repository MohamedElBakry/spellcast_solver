
def find_distance(word1: str, word2: str) -> int:
    cache = [[0 for _ in range(len(word2) + 1)] for _ in range(len(word1) + 1)]

    for j in range(len(word2) + 1):
        cache[len(word1)][j] = len(word2) - j
    for i in range(len(word1) + 1):
        cache[i][len(word2)] = len(word1) - i

    # fill the grid
    for i in range(len(word1) - 1, -1, -1):
        for j in range(len(word2) - 1, -1, -1):
            if word1[i] == word2[j]:
                cache[i][j] = cache[i+1][j+1]
            else:
                cache[i][j] = min(cache[i+1][j], cache[i]
                                  [j+1], cache[i+1][j+1]) + 1

    return cache


def find_distance2(word1: str, word2: str) -> int:
    cache = [[0 for _ in range(len(word2) + 1)] for _ in range(len(word1) + 1)]

    for i in range(len(word1) + 1):
        for j in range(len(word2) + 1):
            if i == 0:
                cache[i][j] = j
            elif j == 0:
                cache[i][j] = i
            elif word1[i - 1] == word2[j - 1]:
                cache[i][j] = cache[i - 1][j - 1]
                # print(i-1, j-1)
            else:
                # print(i-1, j-1)
                cache[i][j] = min(cache[i - 1][j], cache[i]
                                  [j - 1], cache[i - 1][j - 1]) + 1

    return cache


word1, word2 = "oii", "oxidise"


def print_distance(word1, word2, fn):
    s = 3
    print(f'{" ":>3}', end='')
    for c in word2:
        print("{:>3}".format(c), end='')

    print()
    for j, r in enumerate(fn(word1, word2)):
        print("{:>3}".format(word1[j] if j < len(word1) else ''), end='')
        for i, n in enumerate(r):
            print("{:>3}".format(n), end='')
        print()
        # __import__('pprint').pprint("{:>5}".format(*r))i

# print(find_distance("zoo", "zeo")[0][0])


def find_distance3(word1: str, word2: str) -> list[list[int]]:
    lx, ly = len(word2), len(word1)
    lru = [[0 for _ in range(lx + 1)] for _ in range(ly + 1)]
    # lru = [[]]

    for y in range(ly + 1):
        for x in range(lx + 1):
            if y == 0:
                lru[y][x] = x
            elif x == 0:
                lru[y][x] = y

            # add, remove, substitute
            elif word1[y - 1] == word2[x - 1]:
                lru[y][x] = lru[y - 1][x - 1]
            else:
                lru[y][x] = 1 + min(lru[y - 1][x], lru[y][x - 1], lru[y - 1][x - 1])

    return lru


# print(find_distance3(word1, word2))
# print_distance(word1, word2, find_distance3)

def find_distance4(word1: str, word2: str) -> list[list[int]]:
    lx, ly = len(word2), len(word1)
    lru = [[0 for _ in range(lx + 1)] for _ in range(ly + 1)]

    for y in range(ly + 1):
        for x in range(lx + 1):
            if y == 0:
                lru[y][x] = x
            elif x == 0:
                lru[y][x] = y

            elif word1[y - 1] == word2[x - 1]:
                lru[y][x] = lru[y - 1][x - 1]

            else:
                lru[y][x] = 1 + min(lru[y - 1][x], lru[y][x - 1], lru[y - 1][x - 1])

    return lru
    

print_distance("cheese", "tease", find_distance4)
# print(find_distance4("ok", "rammus")[2][6])
