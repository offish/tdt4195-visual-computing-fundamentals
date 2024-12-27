def greyscale(im):
    """Converts an RGB image to greyscale

    Args:
        im ([type]): [np.array of shape [H, W, C]]

    Returns:
        im ([type]): [np.array of shape [H, W]]
    """
    return 0.212 * im[:, :, 0] + 0.7152 * im[:, :, 1] + 0.0722 * im[:, :, 2]
