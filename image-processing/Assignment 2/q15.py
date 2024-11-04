def create_binary_image(im):
    """Creates a binary image from a grayscale image "im"

    Args:
        im ([np.ndarray, np.float]): [An image of shape [H, W] in the range [0, 1]]

    Returns:
        [np.ndarray, bool]: [A binary image]
    """
    fft_image = np.fft.fft2(im)
    fft_shifted = np.fft.fftshift(fft_image)
    return np.abs(fft_shifted) > 250
