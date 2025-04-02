#!/bin/bash

if [[ -f $1 ]]; then
	snips-nlu generate-dataset fr $1 > $(basename $1 .yaml).json
fi
