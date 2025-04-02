#!/bin/bash
export function install() {
	asdf plugin add nodejs https://github.com/asdf-vm/asdf-nodejs.git
	asdf install nodejs lts
	asdf global nodejs lts
}

export function uninstall() {

}

export function is_installed() {

}

export function update() {
	
}