#!/usr/bin/python3
"""
コマンドラインからnumberとanimalを受け取って出力する

実行権限の付与を忘れずに
"""

import argparse
parser = argparse.ArgumentParser()
parser.add_argument("--number", help="input one word from zero to nine")
parser.add_argument("--animal", help="input one word from cat, dog or deer")
args = parser.parse_args()

w2n = {
    'zero': 0,
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
}

if args.number:
    print(w2n[args.number], end=' ')

if args.animal:
    print(args.animal, end='')

print()
