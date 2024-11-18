def fill_holes(
    im: np.ndarray, starting_points: list, num_iterations: int
) -> np.ndarray:
    """
    A function that takes a binary image (im),  and a set of points
    indicating position of holes, and fills the holes.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
        starting_points: list of list containing starting points (row, col). Ex:
            [[row1, col1], [row2, col2], ...]
        num_iterations: integer defining the number of iterations to apply the
                        hole filling algorithm
    return:
        (np.ndarray) of shape (H, W). dtype=np.bool
    """
    structuring_element = np.array(
        [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
        ],
        dtype=bool,
    )
    filled = np.zeros_like(im, int)

    for row, column in starting_points:
        filled[row, column] = 1

    for _ in range(1, num_iterations):
        dilated = skimage.morphology.binary_dilation(filled, structuring_element)
        inverted = ~im
        filled = dilated & inverted

    return filled | im
