## 1 Define opening and closing in terms of erosion and dilation. What happens when opening or closing are applied multiple times on the same image using the same structuring element?
- [ ] Opening is dilation followed by erosion; closing is erosion followed by dilation. Repeated applications change the image.
- [ ] Opening increases the image size, while closing decreases it. Repeated applications always alter the image.
- [ ] Opening and closing are the same operations, resulting in no change to the image.
- [ ] Opening is erosion followed by dilation; closing is dilation followed by erosion. Multiple applications have no effect after the first operation.

## 2 Smoothing of an image is often done before performing edge detection. What is the primary purpose of smoothing the image before edge detection?
- [ ] To eliminate all low-frequency components, ensuring that only high-frequency features are detected as edges.
- [ ] To increase the overall brightness of the image, making edges more apparent in the detection process.
- [ ] To improve the resolution of the image, allowing edge detection algorithms to identify finer details.
- [ ] To reduce noise on the computed edges/gradients, leading to more accurate edge detection.

## 3 The Canny edge detector uses a method called hysteresis thresholding. How does hysteresis thresholding work?
- [ ] It sets a single threshold for pixel intensity and retains all pixels above that threshold, discarding the rest.
- [ ] It uses an iterative approach to adjust thresholds dynamically based on the image's overall gradient distribution.
- [ ] It applies a non-maximum suppression step followed by a binary thresholding step, independently of gradient magnitudes.
- [ ] It identifies all pixels with a gradient magnitude greater than T_high and then explores connected pixels with a gradient magnitude greater than T_low to form continuous edges.

## 4 Why do we use hysteresis thresholding instead of a single threshold?
- [ ] Finding a single threshold value is challenging; a low threshold can introduce false edges, while a high threshold may eliminate valid edge points.
- [ ] Hysteresis thresholding allows for the detection of edges in color images by adjusting thresholds for different color channels.
- [ ] Hysteresis thresholding is computationally faster than using a single threshold due to reduced complexity in edge detection.
- [ ] It simplifies the edge detection process by combining multiple thresholds into a single, uniform thresholding method.
