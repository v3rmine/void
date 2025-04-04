# Banana.dev Bark setup
Experiment with Bark running on banana.dev as an API (https://github.com/suno-ai/bark) 

This is a Bark [Banana.dev](https://www.banana.dev) setup that allows on-demand serverless GPU inference.

You can fork this repository and deploy it on Banana as is, or customize it based on your own needs.

# Running this app

## Deploying on Banana.dev

1. Fork this repository to your own Github account.
2. Connect your Github account on Banana.
3. Create a new model on Banana from the forked Github repository.

## Running after deploying

1. Wait for the model to build after creating it.
2. Make an API request to it using one of the provided snippets in your Banana dashboard.

For more info, check out the [Banana.dev docs](https://docs.banana.dev/banana-docs/).

## Testing locally

### Using Docker

Build the model as a Docker image. You can change the `bananadev-bark` part to anything.

```sh
docker build -t bananadev-bark .
```

Run the Potassium server

```sh
docker run --publish 8000:8000 -it bananadev-bark
```

In another terminal, run inference after the above is built and running.

```sh
curl -X POST -H 'Content-Type: application/json' -d '{"prompt": "Hello World!"}' http://localhost:8000
```

### Without Docker

You could also install and run it without Docker.

Just make sure that the pip dependencies in the Docker file (and torch) are installed in your Python virtual environment.

Run the Potassium app in one terminal window.

```sh
python3 app.py
```

Call the model in another terminal window with the Potassium app still running from the previous step.

```sh
curl -X POST -H 'Content-Type: application/json' -d '{"prompt": "Hello World!"}' http://localhost:8000
```
