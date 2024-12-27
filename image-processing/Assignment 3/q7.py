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

    flat_image = im.ravel()
    histogram, _ = np.histogram(flat_image, bins=256, range=(0, 256))

    histogram = histogram / im.size
    variance = 0
    threshold = 0

    for k in range(256):
        w0 = np.sum(histogram[: (k + 1)])
        w1 = np.sum(histogram[(k + 1) :])

        if w0 > 0:
            mu0 = np.sum(np.arange(0, k + 1) * histogram[0 : (k + 1)]) / w0
        else:
            mu0 = 0

        if w0 > 0:
            mu1 = np.sum(np.arange(k + 1, 256) * histogram[(k + 1) : 256]) / w1
        else:
            mu1 = 0

        sigma = w0 * w1 * (mu0 - mu1) ** 2

        if sigma > variance:
            threshold = k
            variance = sigma

    return threshold
