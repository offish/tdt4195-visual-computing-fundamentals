def distance_transform(im: np.ndarray) -> np.ndarray:
    """
    A function that computes the distance to the closest boundary pixel.

    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=np.bool)
    return:
        (np.ndarray) of shape (H, W). dtype=np.int32
    """
    assert im.dtype == bool

    structuring_element = np.array(
        [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1],
        ],
        dtype=bool,
    )
    result = np.zeros_like(im, int)

    still_eroded_pixel = True

    while still_eroded_pixel:
        still_eroded_pixel = False

        for x in range(im.shape[0]):
            for y in range(im.shape[1]):
                if not im[x, y]:
                    continue

                result[x, y] += 1
                still_eroded_pixel = True

        im = skimage.morphology.binary_erosion(im, structuring_element)

    return result.astype(np.int32)
