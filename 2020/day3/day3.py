import os
f = open('./day3_input', 'r')
modifiers = [(1, 32), (3, 32), (5, 32), (7, 32), (1, 64)]
res = 1

for (col_mod, row_mod) in modifiers:
    trees = 0
    col = 0
    row = 0
    f.seek(0)
    while (c := f.read(1)) != "":
        if c == "#":
            trees += 1
        row += row_mod
        col = (col + col_mod) % 31
        f.seek(row + col)
    print(trees)
    res *= trees
print(res)
