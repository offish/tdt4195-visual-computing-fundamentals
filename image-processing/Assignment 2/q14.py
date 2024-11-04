def create_kernel(image):
    """
    Creates a filter kernel in the frequency domain.

    Args:
        im: np.array of shape [H, W]
    Returns:
        kernel: np.array of shape [H, W]
    """
    # Create a kernel of the same size as the image
    kernel = np.ones_like(image)
    LINE_WIDTH = 1
    PASS_THROUGH = 28

    # Get the center of the image
    row_center, col_center = image.shape[0] // 2, image.shape[1] // 2

    # Create a kernel with a line in the middle
    kernel[row_center - LINE_WIDTH : row_center + LINE_WIDTH + 1, :] = 0
    kernel[
        row_center - LINE_WIDTH : row_center + LINE_WIDTH + 1,
        col_center - PASS_THROUGH : col_center + PASS_THROUGH + 1,
    ] = 1

    # Create a kernel with a line in the middle
    kernel[:, col_center - LINE_WIDTH : col_center + LINE_WIDTH + 1] = 0
    kernel[
        row_center - PASS_THROUGH : row_center + PASS_THROUGH + 1,
        col_center - LINE_WIDTH : col_center + LINE_WIDTH + 1,
    ] = 1

    # Shift the kernel to the center
    kernel = np.fft.fftshift(kernel)

    return kernel
