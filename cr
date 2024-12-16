#!/bin/bash

# 引数のチェック
if [ -z "$1" ]; then
echo "引数が必要です (cl, cp, py, rs)"
exit 1
fi

# コマンドライン引数によって異なるコンパイル/実行を行う
case "$1" in
cl)
    cc ./clib.c -o a.out
    ;;
cp)
    g++ ./cpplib.cpp -o a.out
    ;;
py | p)
    python3 pylib.py
    exit 0  # Pythonは実行後に終了
    ;;
rs)
    rustc rslib.rs -o a.out
    ;;
*)
    echo "無効な引数です。cl, cp, py, rs のいずれかを使用してください。"
    exit 1
    ;;
esac

# コンパイルが成功したら実行
if [ $? -eq 0 ]; then
echo "please input"
./a.out
else
echo "Errors happened to compile..."
exit 1
fi
