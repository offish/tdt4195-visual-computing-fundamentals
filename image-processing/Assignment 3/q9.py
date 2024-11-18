def remove_noise(im: np.ndarray) -> np.ndarray:
    """
    A function that removes noise in the input image.
    args:
        im: np.ndarray of shape (H, W) with boolean values (dtype=bool)
    return:
        (np.ndarray) of shape (H, W). dtype=bool
    """
    footprint = skimage.morphology.disk(8)
    im = skimage.morphology.binary_closing(im, footprint)
    im = skimage.morphology.binary_opening(im, footprint)

    return im
