def extract_boundary(im: np.ndarray) -> np.ndarray:
    """
    A function that extracts the inner boundary from a boolean image.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
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
    im_eroded = skimage.morphology.binary_erosion(im, structuring_element)
    boundary = im ^ im_eroded
    return boundary
