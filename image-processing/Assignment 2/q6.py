def create_model():
    """
    Initializes the model. Edit the code below if you would like to change the model.
    """
    # Replace this placeholder network.
    model = nn.Sequential(
        nn.Flatten(),
        nn.Linear(32 * 32 * 1, 10),
    )

    return model
