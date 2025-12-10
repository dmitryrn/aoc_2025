import difflib


def expect_equal(a, b):
    if a != b:
        diff = "\n".join(difflib.ndiff(str(a).splitlines(), str(b).splitlines()))
        raise AssertionError(diff)
