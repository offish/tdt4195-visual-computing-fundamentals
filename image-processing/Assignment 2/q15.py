def create_binary_image(im):
    """Creates a binary image from a greyscale image "im"

    Args:
        im ([np.ndarray, np.float]): [An image of shape [H, W] in the range [0, 1]]

    Returns:
        [np.ndarray, bool]: [A binary image]
    """

    # START YOUR CODE HERE ### (You can change anything inside this block)
    binary_im = np.zeros_like(im, dtype=bool)
    ### END YOUR CODE HERE ###
    return binary_im
