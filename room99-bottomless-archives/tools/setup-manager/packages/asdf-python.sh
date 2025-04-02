#!/bin/bash
. ../libs/asdf.sh

export function install() {
	asdf plugin add python
	version=$(asdf list-all python \
		| grep "^[0-9]" \
		| grep -Ev "(\-dev|b[0-9]*)$" \
		| tail -n 1)
	asdf install python "$version"
	asdf global python "$version"
	asdf exec pip install --upgrade pip
	asdf exec pip install wheel
}

export function uninstall() {

}

export function is_installed() {

}

export function update() {
	
}