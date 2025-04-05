# SOURCE: dread/post/5e40aa87f1bc99df2b20 
# BetterThanBartards.py
# New EndGame captcha solved in less than 100 lines of code
# m = train_model(9, 12,125,10)
# This model takes about 10 minutes to train on a decent GPU or 30 on a standard CPU.
# This model will solve the new EndGame captcha with approximately 59.6% accuracy - better scores than a bartard!
# Increasing the batch size, model scale, or number of epochs will all result in a model that takes longer to train, but can achieve up to ~85% accuracy.
# Source http://hastebin.net/axijebimok.py
from PIL import Image
from PIL import ImageDraw
from PIL import ImageFont
import random
import os

import numpy as np
from cv2 import imread
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, Conv2D, MaxPooling2D, Flatten

def generate_samples(sample_count=50):
	while True:
		random.seed()
		unicode_chars = ["\u2605","\u2606","\u2663","\u2667","\u2660","\u2664","\u2662","\u2666","\u263a","\u263b","\u26aa","\u26ab","\u2b53","\u2b54","\u2b00","\u2b08","\u2780","\u278a","\u267c","\u267d","\u25b2","\u25b3"]
		training_data = []
		training_labels = []

		unicode_max = len(unicode_chars)
		for i in range(0,sample_count):
			im_cropped = Image.new('RGB', (150, 150),
								   (random.randrange(120, 255), random.randrange(120, 255), random.randrange(120, 255)))
			origwidth, origheight = im_cropped.size
			
			watermark = Image.new("RGBA", im_cropped.size)
			waterdraw = ImageDraw.ImageDraw(watermark, "RGBA")
			number_of_shapes = random.randrange(10, 15)
			for step in range(0, number_of_shapes):
				fillcolor = (
					random.randrange(0, 255), random.randrange(0, 255), random.randrange(0, 255),
					random.randrange(240, 255))
				u_char = unicode_chars[random.randrange(0, unicode_max)]
				font = ImageFont.truetype("dread.ttf", random.randrange(25, 30))
				waterdraw.text((random.randrange(-10, 130), random.randrange(-10, 130)), u_char, fill=fillcolor, font=font)
			
			shapes_empty = ['\u25cb','\u25a1','\u2658','\u2662','\u25bd','\u25b3','\u2656','\u2727','\u2654','\u2658','\u2655','\u2657','\u2659','\u2667']
			shapes_filled = ['\u25cf','\u25a0','\u265e','\u2666','\u25bc','\u25b2','\u265c','\u2726','\u265a','\u265e','\u265b','\u265d','\u265f','\u2663']
			answer = ""
			for x in range(9):
				answer += str(random.randrange(0,2))
			
			for y in range(3):
				capstring = ""
				for x in range(3):
					symbol_index = random.randrange(0,len(shapes_empty))
					symbol_color = (random.randrange(5,256),random.randrange(5,256),random.randrange(5,256))
					
					capstring += shapes_filled[symbol_index] if answer[x]=="1" else shapes_empty[symbol_index]
					capstring += " "
					
					if x==2:
						waterdraw.text( ( random.randrange(10,50), random.randrange((y+1)*30,(y+2)*30) ) , capstring, fill=symbol_color, font=font )
			
			im_cropped.paste(watermark, None, watermark)
			im_cropped.save("/tmp/sample.jpg", format="JPEG")
			training_data.append(imread("/tmp/sample.jpg").astype('float32')/255.0)
			training_labels.append(np.array(list(answer)).astype('int32'))
		yield np.array(training_data), np.array(training_labels)

def train_model(scale, batch_size, steps_per_epoch, epochs):
	model = Sequential()
	model.add(Conv2D(scale*8, 3, input_shape=(150,150,3), activation='relu', padding='same'))
	model.add(Conv2D(scale*7, 3, padding='same'))
	model.add(MaxPooling2D(2))
	
	model.add(Conv2D(scale*6, 3, padding='same'))
	model.add(Conv2D(scale*5, 3, padding='same'))
	model.add(MaxPooling2D(2))
	
	model.add(Conv2D(scale*4, 2, padding='same'))
	model.add(Conv2D(scale*3, 2, padding='same'))
	model.add(MaxPooling2D(2))
	
	model.add(Conv2D(scale*2, 2, padding='same'))
	model.add(Conv2D(scale, 2, padding='same'))
	
	model.add(Flatten())
	model.add(Dense(9, activation='sigmoid'))
	
	model.compile(optimizer="adam", loss="binary_crossentropy", metrics=['accuracy'])
	model.summary()
	
	model.fit_generator(generate_samples(batch_size), steps_per_epoch=steps_per_epoch, epochs=epochs)
	return model

def evaluate(model, image_path):
	x = imread(image_path).astype('float32')/255.0
	return model.predict([x])[0]

m = train_model(9, 12,125,10)
