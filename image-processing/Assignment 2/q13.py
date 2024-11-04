def convolve_im_spatial(im: np.array, kernel: np.array, verbose=True):
    """Convolves the image (im) with the spatial kernel (kernel),
        and returns the resulting image.

        "verbose" can be used for turning on/off visualization
        convolution

        Note: kernel can be of different shape than im.

    Args:
        im: np.array of shape [H, W]
        kernel: np.array of shape [K, K]
        verbose: bool
    Returns:
        im: np.array of shape [H, W]
    """
    ### START YOUR CODE HERE ### (You can change anything inside this block)
    orig_im = im.copy()
    orig_shape = im.shape
    conv_result = im

    if verbose:
        # Use plt.subplot to place two or more images beside eachother
        plt.figure(figsize=(20, 4))
        # plt.subplot(num_rows, num_cols, position (1-indexed))
        plt.subplot(1, 5, 1)
        plt.imshow((orig_im.astype("float")), cmap="gray")
        plt.title("Original Image")

        plt.subplot(1, 5, 2)
        # Visualize FFT
        plt.title("FFT Image")

        plt.subplot(1, 5, 3)
        # Visualize FFT kernel
        plt.title("FFT Kernel")

        plt.subplot(1, 5, 4)
        # Visualize filtered FFT image
        plt.title("Filtered FFT Image")

        plt.subplot(1, 5, 5)
        # Visualize filtered spatial image
        plt.imshow(conv_result, cmap="gray")
        plt.title("Filtered Spatial Image")

    ### END YOUR CODE HERE ###
    return conv_result
