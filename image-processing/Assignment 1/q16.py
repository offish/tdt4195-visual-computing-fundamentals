def create_model():
    """
    Initializes the model. Edit the code below if you would like to change the model.
    """
    model = nn.Sequential(
        nn.Flatten(),  # Flattens the image to (batch_size, 28*28)
        nn.Linear(28 * 28 * 1, 64),
        nn.ReLU(),  # ReLU activation for the hidden layer
        nn.Linear(64, 10),  # Hidden layer to output layer (10 output classes for MNIST)
        # No need to include softmax, handled by the loss function (e.g., CrossEntropyLoss)
    )
    return model


# Answer the question using a boolean value
network_improved = True
