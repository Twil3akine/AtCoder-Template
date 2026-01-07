from collections import deque

ip = input
rg = range
DY, DX = [-1, 0, 1, 0], [0, 1, 0, -1]


def mip(f=int):
    return map(f, ip().split())
