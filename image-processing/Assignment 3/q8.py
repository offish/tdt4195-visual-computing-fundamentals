def region_growing(im: np.ndarray, seed_points: list, T: int) -> np.ndarray:
    """
    A region growing algorithm that segments an image into 1 or 0 (True or False).
    Finds candidate pixels with a Moore-neighborhood (8-connectedness).
    Uses pixel intensity thresholding with the threshold T as the homogeneity criteria.
    The function takes in a grayscale image and outputs a boolean image

    args:
        im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
        seed_points: list of list containing seed points (row, col). Ex:
            [[row1, col1], [row2, col2], ...]
        T: integer value defining the threshold to used for the homogeneity criteria.
    return:
        (np.ndarray) of shape (H, W). dtype=bool
    """
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    # You can also define other helper functions
    segmented = np.zeros_like(im).astype(bool)
    im = im.astype(float)
    for row, col in seed_points:
        segmented[row, col] = True
    return segmented
    ### END YOUR CODE HERE ###
