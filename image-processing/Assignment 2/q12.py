def shift_image(im: np.array):
    return np.fft.fftshift(np.log(np.abs(im) + 1))


def convolve_im_freq(im: np.array, fft_kernel: np.array, verbose=True):
    """Convolves the image (im) with the frequency kernel (fft_kernel),
        and returns the resulting image.

        "verbose" can be used for turning on/off visualization
        convolution

    Args:
        im: np.array of shape [H, W]
        fft_kernel: np.array of shape [H, W]
        verbose: bool
    Returns:
        im: np.array of shape [H, W]
    """
    fft_image = np.fft.fft2(im)
    filtered_fft_image = fft_image * fft_kernel
    conv_result = np.fft.ifft2(filtered_fft_image).real

    if verbose:
        # Use plt.subplot to place two or more images beside eachother
        plt.figure(figsize=(20, 4))
        # plt.subplot(num_rows, num_cols, position (1-indexed))
        plt.subplot(1, 5, 1)

        plt.imshow(im, cmap="gray")
        plt.title("Original Image")
        plt.subplot(1, 5, 2)

        # Visualize FFT
        plt.imshow(shift_image(fft_image), cmap="gray")
        plt.title("FFT Image")
        plt.subplot(1, 5, 3)

        # Visualize FFT kernel
        plt.imshow(shift_image(fft_kernel), cmap="gray")
        plt.title("FFT Kernel")
        plt.subplot(1, 5, 4)

        # Visualize filtered FFT image
        plt.imshow(shift_image(filtered_fft_image), cmap="gray")
        plt.title("Filtered FFT Image")
        plt.subplot(1, 5, 5)

        # Visualize filtered spatial image
        plt.imshow(conv_result, cmap="gray")
        plt.title("Filtered Spatial Image")

    return conv_result
