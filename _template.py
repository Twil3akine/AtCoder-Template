import sys

class Word:
    def __init__(self) -> list[str]:
        self.content = list(sys.stdin.readline().strip())
    
    def isPalindrome(self) -> bool:
        return self.content == self.content[::-1]

class String:
    def __init__(self) -> tuple[str] | str:
        q = sys.stdin.readline().strip().split()
        self.content = tuple(q) if len(q) > 1 else q[0]

class StringList:
    def __init__(self) -> list[str]:
        self.content = sys.stdin.readline().strip().split()

class Integer:
    def __init__(self) -> tuple[int] | int:
        q = sys.stdin.readline().strip().split()
        self.content = tuple(map(int, q)) if len(q) > 1 else int(q[0])

class IntegerList:
    def __init__(self) -> list[int]:
        self.content = list(map(int, sys.stdin.readline().strip().split()))
        self.len = len(self.content)
    
    def prefixSum1D(self) -> list[int]:
        prefixSum = [0]*self.len
        for i in range(self.len):
            prefixSum[i] = self.content[i] + (prefixSum[i-1] if i>0 else 0)

        return prefixSum

    def binarySearch(self, target) -> int:
        left, right = 0, self.len
        while left < right:
            middle = int((left+right)//2)+1
            if self.content[middle] <= target: left = middle + 1
            else: right = middle - 1
        
        return left

def solve():
    l = IntegerList()
    print(l.content)
    print(l.prefixSum1D())
    print(l.binarySearch(0))

if __name__ == "__main__":
    solve()