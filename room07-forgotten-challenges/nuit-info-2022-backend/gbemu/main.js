const Gameboy = require('serverboy');
const fs = require('fs');
const Buffer = require('buffer').Buffer;
const PNG = require('pngjs').PNG;
// REQUIRE: graphicsmagick
const imageToAscii = require('image-to-ascii');

const GAME = "classical-red.gb";

const rom = fs.readFileSync(GAME);
const save = fs.existsSync(`${GAME}.sav`) ? fs.readFileSync(`${GAME}.sav`) : null;

function screenToPng(gameboy) {
	const screen = gameboy.getScreen();
	const png = new PNG({ width: 160, height: 144 });
	for (let i = 0; i < screen.length; i++) {
		png.data[i] = screen[i];
	}

	return PNG.sync.write(png);
}

const gameboy = new Gameboy();
if (save) {
	gameboy.loadRom(rom, save.toJSON().data);
} else {
	gameboy.loadRom(rom);
}

setInterval(function() {
	gameboy.doFrame();
	fs.writeFileSync(`${GAME}.sav`, Buffer.from(gameboy.getSaveData()));

	const png = screenToPng(gameboy);
	imageToAscii(png, {
		pxWidth: 1,
		size: {
			height: "150%",
		},
	}, function (err, converted) {
		console.clear();
		console.log(err || converted);
	});
}, 1000);