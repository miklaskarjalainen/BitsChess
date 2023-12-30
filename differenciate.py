# Get perft inputs from two programs and shows the differences

perft1 = """
"""

perft2 = """
"""

# "a2a3: 44" -> {"a2a3": 44}
def perft_to_dictionary(perft: str) -> dict:
    out: dict = {}

    lines = perft.split("\n")
    for line in lines:
        args = line.split(": ")
        if len(args) > 1:
            out[args[0]] = int(args[1])
    
    return out

splitted_perft1: dict = perft_to_dictionary(perft1)
splitted_perft2: dict = perft_to_dictionary(perft2)


perft1_total_count: int = 0
perft2_total_count: int = 0

differences: str = ""

# move is e.g "a1a2"
# compare every move from first perft1
for move in splitted_perft1:
    move1_count = splitted_perft1[move]
    perft1_total_count += move1_count

    if move in splitted_perft2.keys():
        move2_count = splitted_perft2[move]
        perft2_total_count += move2_count

        # the 2 moves differred!
        if move1_count != move2_count:
            differences += "{}: [{}, {}]\n".format(move, move1_count, move2_count)
            pass

        # remove handeled entries
        splitted_perft2.pop(move)
    else:
        # A move in first perft is not contained in the second
        differences += "{}: [{}, {}]\n".format(move, move1_count, 0)

# now loop through the second one, because it might contain something which the first one didn't include.
for move in splitted_perft2:
    move2_count = splitted_perft2[move]    
    perft2_total_count += move2_count
    differences += "{}: [{}, {}]\n".format(move, 0, move2_count)

print("Differences: {} {}".format(perft1_total_count, perft2_total_count))
print(differences)
