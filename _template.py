import sys

class Word:
    def __init__(self, content: list[str]) -> list[str]:
        self.content = content
    
    def isPalindrome(self) -> bool:
        return self.content == self.content[::-1]

class String:
    def __init__(self, content: tuple[str] | str) -> tuple[str] | str:
        self.content = content

class StringList:
    def __init__(self, content: list[str]) -> list[str]:
        self.content = content

class Integer:
    def __init__(self, content: tuple[int] | int) -> tuple[int] | int:
        self.content = content

class IntegerList:
    def __init__(self, content: list[int]) -> list[int]:
        self.content = content

class Input:
    def __init__(self) -> None:
        self.data = sys.stdin.readline().strip()
    
    # Input string 'try' such as ["t", "r", "y"]
    def word(self) -> Word:
        return Word(list(self.data))

    # Input string(s) 'red blue green' as ("red", "blue", "green")
    def string(self) -> String:
        q = self.data.split()
        return String(tuple(q) if len(q) > 1 else q[0])

    # Input string(s) 'red blue green' as ["red", "blue", "green"]
    def stringList(self) -> StringList:
        return StringList(self.data.split())

    # Input interger(s) '1 2 3' as (1, 2, 3)
    def integer(self) -> Integer:
        q = self.data.split()
        return Integer(tuple(map(int, q)) if len(q) > 1 else int(q[0]))

    # Input interger(s) '1 2 3' as [1, 2, 3]
    def integerList(self) -> IntegerList:
        return IntegerList(list(map(int, self.data.split())))

def solve():
    s,t=Input().word().content, Input().word().content
    i=0
    flg=False
    ans=[]
    while s!=t:
        if flg==False:
            if i==len(s):
                flg=True
                i=len(s)-1
                continue
            if ord(s[i]) > ord(t[i]):
                s[i]=t[i]
                ans.append("".join(s))
            i+=1
        else:
            if i==0:
                flg=True
            if ord(s[i]) < ord(t[i]):
                s[i]=t[i]
                ans.append("".join(s))
            i-=1
        
    print(len(ans))
    for i in ans:
        print(i)

if __name__ == "__main__":
    solve()