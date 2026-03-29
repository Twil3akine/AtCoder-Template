import math as mt
import sys

# =================================


def main():
    n = II1()
    bins = []
    # 2の冪乗を求める
    i = 1
    while i < 10**9:
        bins.append(str(i))
        i *= 2
    cand = set()

    # current: 現在の文字列
    def dfs(current):
        current_len = len(current)
        for bi in bins:
            if current_len + len(bi) > 9:
                break

            if (current + bi) not in cand:
                dfs(current + bi)

            cand.add(current + bi)

    dfs("")
    sorted_cand = sorted(map(int, list(cand)))
    # print(len(sorted_cand))
    print(sorted_cand[n])


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
