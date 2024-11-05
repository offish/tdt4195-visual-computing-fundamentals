def otsu_thresholding(im: np.ndarray) -> int:
    """
    Otsu's thresholding algorithm that segments an image into 1 or 0 (True or False)
    The function takes in a grayscale image and outputs a threshold value

    args:
        im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
    return:
        (int) the computed thresholding value
    """
    assert im.dtype == np.uint8
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    # Compute normalized histogram
    threshold = 128
    return threshold
    ### END YOUR CODE HERE ###
