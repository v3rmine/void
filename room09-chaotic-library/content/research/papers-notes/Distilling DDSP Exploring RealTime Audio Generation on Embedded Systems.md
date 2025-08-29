---
title: "Distilling DDSP: Exploring Real-Time Audio Generation on Embedded Systems"
added: 2025-08-29
authors: Gregorio Andrea Giudici, Franco Caspe, L. Gabrielli, Stefano Squartini, Luca Turchet
tags:
  - research-paper
url: https://www.semanticscholar.org/paper/4c43be43e282b13f5e9606beef5a7bee9797f600
pdf: "[[Distilling DDSP - Exploring Real-Time Audio Generation on Embedded Systems.pdf]]"
citekey: Giudici2025DistillingDE
year: "2025"
abstract: "This paper investigates the feasibility of running neural audio generative models on embedded systems, by comparing the performance of various models and evaluating their trade-offs in audio quality, inference speed, and memory usage. This work focuses on differentiable digital signal processing (DDSP) models, due to their hybrid architecture, which combines the efficiency and interoperability of traditional DSP with the flexibility of neural networks. In addition, the application of knowledge distillation (KD) is explored to improve the performance of smaller models. Two types of distillation strategies were implemented and evaluated: audio distillation and control distillation. These methods were applied to three foundation DDSP generative models that integrate Harmonic-Plus-Noise, FM, and Wavetable synthesis. The results demonstrate the overall effectiveness of KD: the authors were able to train student models that are up to 100× smaller than their teacher counterparts while maintaining comparable performance and significantly improving inference speed and memory efficiency. However, cases where KD failed to improve or even degrade student performance have also been observed. The authors provide a critical reflection on the advantages and limitations of KD, exploring its application in diverse use cases and emphasizing the need for carefully tailored strategies to maximize its potential."
---

