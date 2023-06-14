#!/usr/bin/env python3


def sum(x):
    z = 0

    for y in range(0, x):
        z += y

    return z


print(sum(1000000))
