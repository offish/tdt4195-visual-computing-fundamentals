def create_model():
    """
    Initializes the model. Edit the code below if you would like to change the model.
    """
    # Replace this placeholder network.
    # model = nn.Sequential(
    #     nn.Flatten(),  # Flattens the image from shape (batch_size, C, Height, width) to (batch_size, C*height*width)
    #     nn.Linear(32*32*1, 10),
    # )

    # return model
    model = nn.Sequential(
        # layer 1
        nn.Conv2d(in_channels=1, out_channels=32, kernel_size=5, stride=1, padding=2),
        nn.ReLU(),
        nn.MaxPool2d(kernel_size=2, stride=2),
        # layer 2
        nn.Conv2d(in_channels=32, out_channels=64, kernel_size=3, stride=1, padding=1),
        nn.ReLU(),
        nn.MaxPool2d(kernel_size=2, stride=2),
        # layer 3
        nn.Conv2d(in_channels=64, out_channels=128, kernel_size=3, stride=1, padding=1),
        nn.ReLU(),
        nn.MaxPool2d(kernel_size=2, stride=2),
        nn.Flatten(),
        # layer 4
        nn.Linear(128 * 4 * 4, 64),  # 128 filters with 4x4 feature maps after pooling
        nn.ReLU(),
        # layer 5
        nn.Linear(64, 10),
    )

    return model
