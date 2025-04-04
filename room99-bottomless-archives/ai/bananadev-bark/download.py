# This file runs during container build time to get model weights built into the container
from bark import preload_models

def download_model():
    preload_models()

if __name__ == "__main__":
    download_model()
