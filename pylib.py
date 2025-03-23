<<<<<<< HEAD
# type: ignore

import sortedcontainers

input_number = lambda: int(input())
input_numbers = lambda: map(int, input().split())
input_list_number = lambda: list(map(int, input().split()))
yes_no = lambda cdt: print("Yes" if cdt else "No")

def no(cdt):
    if not cdt:
        print("No")
        exit(0)

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

def solve():
    
    
    pass

def main():
    # for i in range(int(input())):
    solve()
        
if __name__ == "__main__":
    main()
    
'''
aaabcac
cacbaaa aaabcac
'''
