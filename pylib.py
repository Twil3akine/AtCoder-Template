import sys

import random
from functools import cache
from sortedcontainers import SortedList, SortedSet
from collections import defaultdict, deque, Counter
from heapq import heapify, heappush, heappop
from itertools import product, zip_longest
from more_itertools import distinct_combinations as combinations, distinct_permutations as permutations

# 別に必要ではないやつime
# import numpy as np

sys.setrecursionlimit(10**6)

# datatype such as ["t", "r", "y"]
class Word:
    def __init__(self, data=None) -> None:
        self.content = list(sys.stdin.readline().strip()) if data==None else data

    def isPalindrome(self) -> bool:
        return self.content == self.content[::-1]
    
class WordGrid:
    def __init__(self, height, width, data=None) -> None:
        self.content = [Word().content for _ in range(height)] if data==None else data

# datatype such as ("red", "blue", "green")
class String:
    def __init__(self, data=None) -> None:
        q = sys.stdin.readline().strip().split()
        self.content = tuple(q) if len(q) > 1 else q[0] if data==None else data

    def trans(self, dict: dict[str:str]):
        return self.content.translate(str.maketrans(dict))

# datatype such as ["red", "blue", "green"]
class StringList:
    def __init__(self, data=None) -> None:
        self.content = sys.stdin.readline().strip().split() if data==None else data
        
    def T(self):
        self.content = list(zip(*self.content))

# datatype such as (1, 2, 3)
class Integer:
    def __init__(self, data=None) -> None:
        q = sys.stdin.readline().strip().split()
        self.content = tuple(map(int, q)) if len(q) > 1 else int(q[0]) if data==None else data

# datatype such as [1, 2, 3]
class IntegerList:
    def __init__(self, data=None) -> None:
        self.content = list(map(int, sys.stdin.readline().strip().split())) if data==None else data

    def indexList(self) -> list[int]:
        return range(len(self.content))
    
    def prefixSum1D(self) -> list[int]:
        prefixSum = [0]*(len(self.content)+1)
        for i in range(len(self.content)):
            prefixSum[i+1] = prefixSum[i] + self.content[i]

        return prefixSum
    
    def prefixSum1DReverse(self) -> list[int]:
        prefixSumReverse = [0]*(len(self.content)+1)
        for i in range(-1, -len(self.content)-1, -1):
            prefixSumReverse[i-1] = prefixSumReverse[i] - self.content[i]

        return prefixSumReverse

    def lowerBound(self, target) -> int: # target を挿入するべき最小の位置
        left, right = -1, len(self.content)-1
        while right - left > 1:
            middle = left + (right - left)//2
            if self.content[middle] < target: left = middle
            else: right = middle
        
        return right
        
    def upperBound(self, target) -> int: # target を挿入するべき最大の位置
        left, right = -1, len(self.content)-1
        while right - left > 1:
            middle = left + (right - left)//2
            if self.content[middle] <= target: left = middle
            else: right = middle
        
        return right

# datatype such as [[1, 2, 3],
#                   [4, 5, 6],
#                   [7, 8, 9]]
class IntegerGrid:
    def __init__(self, height: int = 0, width: int = 0, data=None) -> None:
        self.height = height
        self.width = height if width==0 and height!=0 else width
        self.content = [IntegerList().content for _ in range(height)] if data==None else data
        
    def T(self):
        self.content = list(zip(*self.content))
        return self.content
        
    def prefixSum2D(self) -> list[list[int]]:
        prefix = [[0]*(self.width+1) for _ in range(self.height+1)]
        for h in range(self.height):
            for w in range(self.width):
                prefix[h+1][w+1] = (prefix[h][w+1] + 
                                   prefix[h+1][w] - 
                                   prefix[h][w] + 
                                   self.content[h][w])

        return prefix

class UnionFind:
    def __init__(self, n: int) -> None:
        self.n = n
        self.parents = [-1]*n # 負なら根, sizeも格納
    
    def find(self, target: int):
        if self.parents[target] < 0:
            return target
        else:
            self.parents[target] = self.find(self.parents[target])
            return self.parents[target]
    
    def union(self, x: int, y: int): # yをxの配下に置く。
        x, y = self.find(x), self.find(y)
        if x == y: return
        
        if self.parents[x] < self.parents[y]: x, y = y, x
        self.parents[x] += self.parents[y]
        self.parents[y] = x
    
    def size(self, target: int):
        return -self.parents[self.find(target)]
    
    def same(self, x: int, y: int):
        return self.find(x) == self.find(y)
    
class Graph:
    def __init__(self, n, m, costflg=False, directflg=False) -> list[list[int]]:
        self.graph = [[] for _ in range(n)]
        for _ in range(m):
            if costflg: u,v,w = Integer().content
            else: u,v = Integer().content; w = 0
            self.graph[u-1].append((v-1,w))
            if not directflg: self.graph[v-1].append((u-1,-w))

class Dijkstra:
    def __init__(self, graph: dict) -> list[list[int]]:
        self.graph = graph
        self.distances = [INF for _ in graph]
        self.queue = []
    
    def search(self, target, start=1):
        self.distances[start:=start-1] = 0
        heappush(self.queue, (0,start))
        while self.queue:
            cost,current = heappop(self.queue)
            if self.distances[current] < cost: continue
            for v,c in self.graph[current]:
                if self.distances[v] > self.distances[current]+c:
                    self.distances[v] = self.distances[current]+c
                    heappush(self.queue, (self.distances[v], v))
        
        return self.distances
    
class Cycle: # under developped
    def __init__(self, n:int, x:int, y:int) -> None:
        self.size = n
        self.left = x
        self,right = y
    
    def position(self, target, direction=False) -> bool:
        pass

def YesNo(cdt): print("Yes" if cdt==True else "No")

def distance(a,b): return ((b[0]-a[0])**2+(b[1]-a[1])**2)**0.5

INF = 10**18+1

#     2   8   6   4   9   1   3   7
dx = [0,  0,  1, -1,  1, -1,  1, -1]
dy = [1, -1,  0,  0,  1, -1, -1,  1]

@cache
def recursive():
    pass

def lowerBound(list, target) -> int: # target を挿入するべき最小の位置
    left, right = -1, len(list)-1
    while right - left > 1:
        middle = left + (right - left)//2
        if list[middle] < target: left = middle
        else: right = middle
    
    return right

def solve():
    
    
    return
    
TURNING = 1
def main():
    for i in range(TURNING):
        solve()

if __name__ == "__main__":
    main()

"""

3 6
1 3
4 4
5 6



"""
