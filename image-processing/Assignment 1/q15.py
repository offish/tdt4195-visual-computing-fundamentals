def plot_weights(weight):
    """
    Plots the weights of the model. Only works for models with a single linear layer.
    Returns the shape of the reshaped weights as torch.Size.
    """

    fig, axs = plt.subplots(1, 10, figsize=(20, 2))
    reshaped_weight = weight.view(10, 28, 28)

    for i in range(10):
        axs[i].imshow(reshaped_weight[i].detach().cpu().numpy(), cmap="viridis")
        axs[i].axis("off")

    return reshaped_weight.shape
