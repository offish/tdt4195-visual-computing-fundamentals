from math import floor
# M_original = [
#     [2, 5, 6, 6, 1],
#     [6, 1, 4, 7, 6],
#     [7, 4, 7, 1, 6],
# ]

intensities = [0, 1, 2, 3, 4, 5, 6, 7]
intensity_count = [0, 3, 1, 0, 2, 1, 5, 3]

# Probability Density Function
pdf = [0, 3 / 15, 1 / 15, 0, 2 / 15, 1 / 15, 5 / 15, 3 / 15]

# Cumulative Distribution Function
cdf = [0, 3 / 15, 4 / 15, 4 / 15, 6 / 15, 7 / 15, 12 / 15, 15 / 15]

# Round down any resulting pixel intensities that are not integers (use the floor operator)
new_intensities = [
    0,  # 0
    floor(7 * (3 / 15)),  # 1
    floor(7 * (4 / 15)),  # 1
    floor(7 * (4 / 15)),  # 1
    floor(7 * (6 / 15)),  # 2
    floor(7 * (7 / 15)),  # 3
    floor(7 * (12 / 15)),  # 5
    floor(7 * (15 / 15)),  # 7
]

# M_original = [
#     [2, 5, 6, 6, 1],
#     [6, 1, 4, 7, 6],
#     [7, 4, 7, 1, 6],
# ]

# 0 -> 0
# 1 -> 1
# 2 -> 1
# 3 -> 1
# 4 -> 2
# 5 -> 3
# 6 -> 5
# 7 -> 7

M_equalized = [
    [1, 3, 5, 5, 1],
    [5, 1, 2, 7, 5],
    [7, 2, 7, 1, 5],
]
