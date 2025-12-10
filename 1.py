from common import expect_equal

example_input = """
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"""


def parse_input(inp: str) -> list[int]:
    res = []
    for item in inp.split():
        action = item[:1]
        num = int(item[1:])

        num = num % 100

        if action == "L":
            num -= num * 2

        res.append(num)
    return res


def algo(actions: list[int]) -> int:
    state = 50
    psswd = 0
    for num in actions:
        # s_old = state

        state += num
        if state < 0:
            state = 100 + state
        if state == 100:
            state = 0
        if state > 100:
            state = state - 100

        if state == 0:
            psswd += 1

        # print(s_old, num, state)
    return psswd


expect_equal(parse_input(example_input), [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82])
expect_equal(parse_input("L68\nL668"), [-68, -68])
expect_equal(algo(parse_input(example_input)), 3)
expect_equal(
    algo(
        parse_input("""
L668
L30
R48
L5
R60
L55
L1
L99
R14
L82
""")
    ),
    3,
)
expect_equal(
    algo(
        parse_input("""
L668
L30
R448
L5
R60
L55
L1
L99
R14
L82
""")
    ),
    3,
)

f = open("inputs/1.txt")
prod_input = f.read()
f.close()
expect_equal(algo(parse_input(prod_input)), 1102)
# 146 is too low
