def distance_transform(im: np.ndarray) -> np.ndarray:
    """
    A function that computes the distance to the closest boundary pixel.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
    return:
        (np.ndarray) of shape (H, W). dtype=np.int32
    """
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    assert im.dtype == bool
    structuring_element = np.array([[1, 1, 1], [1, 1, 1], [1, 1, 1]], dtype=bool)
    result = im.astype(np.int32)
    return result
    ### END YOUR CODE HERE ###
