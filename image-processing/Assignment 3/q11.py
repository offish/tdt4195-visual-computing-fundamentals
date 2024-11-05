def extract_boundary(im: np.ndarray) -> np.ndarray:
    """
    A function that extracts the inner boundary from a boolean image.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
    return:
        (np.ndarray) of shape (H, W). dtype=np.bool
    """
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    structuring_element = np.array([[1, 1, 1], [1, 1, 1], [1, 1, 1]], dtype=bool)
    boundary = im
    return boundary
    ### END YOUR CODE HERE ###
