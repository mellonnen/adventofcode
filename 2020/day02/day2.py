import sys
import re
correct_passwords = 0
correct_passwords_2 = 0

for line in sys.stdin:
    pattern = '^'
    split = line.split(" ")
    lo = int(split[0].split("-")[0])
    hi = int(split[0].split("-")[1])
    c = split[1][0]
    word = split[2]
    pattern += lo*('[^' + c + ']*' + c)
    pattern += (hi-lo)*('[^' + c + ']*' + c + '?')
    pattern += '[^' + c + ']*$'
    rc = re.compile(pattern)
    if re.match(rc, word):
        correct_passwords += 1
    if (word[lo-1] == c) ^ (word[hi-1] == c):
        correct_passwords_2 += 1
print(correct_passwords)
print(correct_passwords_2)
