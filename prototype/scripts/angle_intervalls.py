def contains(s, e, clock, a):
    if clock:
        if e < s:
            return not (a > e and a < s)
        else:
            return a > s and a < e
    else:
        if e < s:
            return a < s and a > e
        else:
            return not (a > s and a < e)


assert contains(0, 3, True, 2) == True
assert contains(0, 3, True, 3) == False
assert contains(0, 3, False, 2) == False
assert contains(0, 0, True, 2) == False
assert contains(-1, 3, True, 2) == True
assert contains(5, 1, True, 0) == True
assert contains(5, 1, False, 0) == False
