from kybra import nat, update


# Calculate the product of all positive integers less than or equal to `n`
@update
def fac(n: nat) -> nat:
    return go(n)


# We implement the recursion in a helper function
def go(m: nat) -> nat:
    if (m == 0):
        return 1
    else:
        return m * go(m - 1)
