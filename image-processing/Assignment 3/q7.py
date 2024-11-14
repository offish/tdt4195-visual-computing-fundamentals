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

    threshold = 0
    biggest_sigma = 0

    # grayscale image histogram
    for k in range(0, 255):
        w0 = np.sum(im < k) / im.size
        w1 = np.sum(im >= k) / im.size

        # no values which are less than k or greater than k
        if w0 == 0 or w1 == 0:
            continue

        m0 = np.sum(im[im < k]) / np.sum(im < k)
        m1 = np.sum(im[im >= k]) / np.sum(im >= k)

        sigma = w0 * w1 * (m0 - m1) ** 2

        if sigma > biggest_sigma:
            biggest_sigma = sigma
            threshold = k

    return threshold
