#!/bin/bash
export function install() {
	asdf plugin add ruby https://github.com/asdf-vm/asdf-ruby.git
	version=$(asdf list-all ruby \
		| grep "^[0-9]" \
		| grep -Ev "\-(dev|rc[0-9]*|preview[0-9]*|p[0-9]*)$" \
		| tail -n 1)
	asdf install ruby "$version"
	asdf global ruby "$version"
}

export function uninstall() {

}

export function is_installed() {

}

export function update() {
	
}