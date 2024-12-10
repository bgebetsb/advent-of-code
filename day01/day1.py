#!/usr/bin/env python3

def read_file():
    with open('input.txt') as file:
        left = []
        right = []
        for line in file:
            splitted = line.split(' ')
            splitted = list(filter(None, splitted))
            if len(splitted) != 2:
                raise RuntimeError("Each line must have exactly two numbers")
            left.append(int(splitted[0]))
            right.append(int(splitted[1].strip()))
    return left, right

if __name__ == '__main__':
    left, right = read_file()
    left.sort()
    right.sort()
    total = 0
    similarity = 0
    for i, leftitem in enumerate(left):
        total += abs(leftitem - right[i])
        similarity += leftitem * right.count(leftitem)
    print("Total distance: " + str(total))
    print("Similarity score: " + str(similarity))
