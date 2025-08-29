---
title: "FINN-R: An End-to-End Deep-Learning Framework for Fast Exploration of Quantized Neural Networks"
added: 2025-08-29
authors: Michaela Blott, Thomas Preusser, Nicholas Fraser, Giulio Gambardella, Kenneth O'Brien, Yaman Umuroglu
tags:
  - research-paper
url: https://arxiv.org/abs/1809.04570
pdf: "[[room09-chaotic-library/content/research/papers-pdf/FINNR An EndtoEnd DeepLearning Framework for Fast Exploration of Quantized Neural Networks.pdf]]"
citekey: blott2018finnrendtoenddeeplearningframework
year: "2018"
abstract: Convolutional Neural Networks have rapidly become the most successful machine learning algorithm, enabling ubiquitous machine vision and intelligent decisions on even embedded computing-systems. While the underlying arithmetic is structurally simple, compute and memory requirements are challenging. One of the promising opportunities is leveraging reduced-precision representations for inputs, activations and model parameters. The resulting scalability in performance, power efficiency and storage footprint provides interesting design compromises in exchange for a small reduction in accuracy. FPGAs are ideal for exploiting low-precision inference engines leveraging custom precisions to achieve the required numerical accuracy for a given application. In this article, we describe the second generation of the FINN framework, an end-to-end tool which enables design space exploration and automates the creation of fully customized inference engines on FPGAs. Given a neural network description, the tool optimizes for given platforms, design targets and a specific precision. We introduce formalizations of resource cost functions and performance predictions, and elaborate on the optimization algorithms. Finally, we evaluate a selection of reduced precision neural networks ranging from CIFAR-10 classifiers to YOLO-based object detection on a range of platforms including PYNQ and AWSF1, demonstrating new unprecedented measured throughput at 50TOp/s on AWS-F1 and 5TOp/s on embedded devices.
---

