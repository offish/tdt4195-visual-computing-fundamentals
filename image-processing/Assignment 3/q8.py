def region_growing(im: np.ndarray, seed_points: list, T: int) -> np.ndarray:
    """
    A region growing algorithm that segments an image into 1 or 0 (True or False).
    Finds candidate pixels with a Moore-neighborhood (8-connectedness).
    Uses pixel intensity thresholding with the threshold T as the homogeneity criteria.
    The function takes in a grayscale image and outputs a boolean image.

    args:
        im: np.ndarray of shape (H, W) in the range [0, 255] (dtype=np.uint8)
        seed_points: list of lists containing seed points (row, col). Ex:
            [[row1, col1], [row2, col2], ...]
        T: integer value defining the threshold to be used for the homogeneity criteria.

    return:
        np.ndarray of shape (H, W). dtype=bool
    """
    # Initialize the segmented (output) and visited arrays
    segmented = np.zeros(im.shape, dtype=bool)
    visited = np.zeros(im.shape, dtype=bool)
    im = im.astype(float)

    # 8 connected neighbors
    neighbors = [
        (0, 0),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ]

    # check for each seed point
    for x, y in seed_points:
        stack = [(x, y)]

        while stack:
            cx, cy = stack.pop()

            # check neighbors
            for dx, dy in neighbors:
                nx, ny = cx + dx, cy + dy

                # check if within bounds
                if nx < 0 or nx >= im.shape[0] or ny < 0 or ny >= im.shape[1]:
                    continue

                # no need to check twice
                if visited[nx, ny]:
                    continue

                visited[nx, ny] = True

                # does not meet homogeneity criteria
                if abs(im[nx, ny] - im[x, y]) > T:
                    continue

                # meets homogeneity criteria
                segmented[nx, ny] = True
                stack.append((nx, ny))  # grow region

    return segmented
