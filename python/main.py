# type: ignore
# import sortedcontainers

from math import isqrt
from itertools import permutations, combinations, product
from collections import defaultdict, deque
from heapq import heapify, heappush, heappop
from pprint import pprint

# ===========================================================================

INF = 9*10**18+1

def main():
    solve()

def solve():

    pass
    
# ===========================================================================

def number(): return int(input())
def numbers(): return map(int, input().split())
def based1_numbers(): return map(lambda x: int(x)-1, input().split())
def list_number(): return list(map(int, input().split()))
def grid(height: int): return [list(input()) for _ in range(height)]
def graph(node: int, edge: int, direction=False, cost: bool=False):
    g = [[] for _ in range(node)]
    for _ in range(edge):
        if cost:
            u, v, c = based1_numbers()
            g[u].append((v, c))
            if not direction: g[v].append((u, c))
        else:
            u, v = based1_numbers()
            g[u].append(v)
            if not direction: g[v].append(u)
    return g

def yes(cdt: bool):
    if cdt:
        print("Yes")
        exit(0)

def no(cdt: bool):
    if not cdt:
        print("No")
        exit(0)
        
def yes_no(cdt: bool):
    print("Yes" if cdt else "No")
    
def print_grid(grid: list):
    for i in grid: print(i)

def bound_search(array: list, target: int, cdt) -> int:
    left, right = -1, len(array)-1
    while right - left > 1:
        middle = left + (right - left)//2
        if cdt(middle, target): left = middle
        else: right = middle
    return right

def binary_search(left: int, right: int, target: int, cdt) -> int:
    while right - left > 1:
        middle = left + (right - left)//2
        if cdt(middle, target): left = middle
        else: right = middle
    return right

def cumulative_sum(array: list, reverse: bool=False) -> list:
    n = len(array)
    result = [0]*(n+1)
    for i in range(n):
        if reverse: 
            result[n-i-1] = result[n-i] + array[i]
        else:
            result[i+1] = result[i] + array[i]
    return result

class UnionFind:
    def __init__(self, n: int):
        self.parents = [-1]*n
    
    def find(self, x: int) -> int:
        if self.parents[x] < 0: return x
        self.parents[x] = self.find(self.parents[x])
        
        return self.parents[x]
    
    def merge(self, x: int, y: int) -> bool:
        vx, vy = self.find(x), self.find(y)
        
        if vx == vy: return False
        if self.parents[vx] > self.parents[vy]: vx, vy = vy, vx
        
        self.parents[vx] += self.parents[vy]
        self.parents[vy] = vx
        
        return True
    
    def same(self, x: int, y: int) -> bool:
        return self.find(x) == self.find(y)
    
    def size(self, x: int) -> int:
        root = self.find(x)
        return -self.parents[root]
    
def manacher(s: list) -> list:
    n = len(s)
    a = [0]*(2*n+1)
    i, j = 1, 1
    
    while i <= 2*n:
        while (j < i) and (i+j < 2*n) and (s[(i-j)//2-1] == s[(i+j)//2]): j += 2
        a[i] = j
        
        if j == 0:
            i, j = i+1, 1
            continue
        
        k = 1
        while (k <= i) and (k+a[i-k] < j):
            a[i+k] = a[i-k]
            k += 1
            
        i, j = i+k, j-k
    
    return [v for i, v in enumerate(a) if i%2 == 1]

def zlgorithm(s: list) -> list:
    n = len(s)
    z = [0]*n
    l, r = 0, 0
    for i in range(1, n):
        if i <= r: z[i] = min(z[i-l], r-i+1)
        while (i + z[i] < n) and (s[z[i]] == s[i+z[i]]): z[i] += 1
        if r < i+z[i]-1: l, r = i, i+z[i]-1
    return z
        
if __name__ == "__main__":
    main()
