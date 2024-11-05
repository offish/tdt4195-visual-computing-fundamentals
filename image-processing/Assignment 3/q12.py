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
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    structuring_element = np.array([[1, 1, 1], [1, 1, 1], [1, 1, 1]], dtype=bool)
    result = im.copy()
    return result
    ### END YOUR CODE HERE ###
