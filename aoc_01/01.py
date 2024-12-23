import numpy as np
import re


def read_input(fname, ncols):

    file = open(fname, 'r')
    Lines = file.readlines()

    count = len(Lines)

    arr = np.zeros(shape=(count, ncols))

    for i, line in enumerate(Lines):
        nums = re.findall(r'\b\d+\b', line)
        arr[i] = nums

    return arr


def solve():

    data = read_input("input.txt", 2)

    sorted = np.sort(data, axis=0)

    sum = 0
    for pair in sorted:
        sum += abs(pair[0] - pair[1])

    print(sum)


solve()
