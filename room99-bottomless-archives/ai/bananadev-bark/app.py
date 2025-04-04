from io import BytesIO
from potassium import Potassium, Request, Response
from diffusers import DiffusionPipeline, DDPMScheduler
import torch
import base64
from bark import preload_models, generate_audio, SAMPLE_RATE
from scipy.io.wavfile import write

# create a new Potassium app
app = Potassium("bark")

# @app.init runs at startup, and loads models into the app's context
@app.init
def init():
    preload_models()
    context = {}

    return context

# @app.handler runs for every call
@app.handler()
def handler(context: dict, request: Request) -> Response:
    prompt = request.json.get("prompt")

    audio_array = generate_audio(prompt)

    buffered = BytesIO()
    write(buffered, SAMPLE_RATE, audio_array)
    wav_str = base64.b64encode(buffered.getvalue())

    # You could also consider writing this image to S3
    # and returning the S3 URL instead of the image data
    # for a slightly faster response time
    return Response(
        json = {"output": str(wav_str, "utf-8")}, 
        status=200
    )

if __name__ == "__main__":
    app.serve()
