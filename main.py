import math as mt
import sys

# =================================


def main():
    


# =================================


# 最大再帰回数の設定
sys.setrecursionlimit(10**7)
# 巨大な整数の文字列変換の設定
sys.set_int_max_str_digits(0)

try:
    import pypyjit

    pypyjit.set_param("max_unroll_recursion=-1")
except ImportError:
    pass

input = sys.stdin.readline
_write = sys.stdout.write


def print(*args, sep=" ", end="\n"):
    _write(sep.join(map(str, args)) + end)


def I():
    return input().rstrip("\n")


def II():
    return int(input())


def II1():
    return int(input()) - 1


def IS():
    return input().split()


def MII():
    return map(int, input().split())


def MII1():
    return (int(x) - 1 for x in input().split())


if __name__ == "__main__":
    main()
