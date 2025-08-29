---
tags:
  - phd-offer
---
## About the research centre or Inria department

The Inria research centre in Lyon is the 9th Inria research centre, formally created in January 2022.  It brings together approximately 300 people in 19 research teams and research support services.

Its staff are distributed at this stage on 2 campuses: in Villeurbanne La Doua (Centre / INSA Lyon / UCBL) on the one hand, and Lyon Gerland (ENS de Lyon) on the other.

The Lyon centre is active in the fields of software, distributed and high-performance computing, embedded systems, quantum computing and privacy in the digital world, but also in digital health and computational biology.

## Context

The successful candidate will join the Emeraude INRIA/INSA-Lyon research team which is physically based at the CITI Lab of INSA-Lyon (Villeurbanne, France). This PhD will be conducted under the supervision of Romain Michon (Inria) and Pierre Lecomte (École Centrale de Lyon). The Emeraude team gathers the strengths of Inria, INSA Lyon, and GRAME-CNCM. It specializes on embedded audio systems and their programming as well as arithmetic. It develops various tools such as the Faust programming language (a DSL for Real-Time audio DSP), [[Syfala]] (a tool to facilitate the programming of FPGAs for real-time audio DSP), and [[FloPoCo]] (a generator of arithmetic cores for FPGAs). The team hosts 6 faculty, 6 PhD students, 3 postdocs, 2 engineer, and multiple research interns. Additional information can be found on team website: https://team.inria.fr/emeraude.

## Assignment

**PhD Topic**

AI-based real-time embedded audio Digital Signal Processing (DSP) is an emerging field which promises to revolutionize our daily life. Embedded real-time audio DSP can be found in hearing aids, noise canceling headphones, cars for noise canceling as well, professional and home audio equipment, musical instruments, etc. However, despite a very diverse range of preliminary work on this topic [^1][^2][^3], significant challenges related to running complex AI algorithms such as Deep Neural Network (DNN) inference on lightweight platforms with limited computational power and relying on potentially small batteries do remain to be overtaken. These challenges can be summarized as: (i) computational efficiency/optimization, (ii) power efficiency, (iii) audio latency, and (iv) programmability. Computational (i) and power (ii) efficiency are interconnected as more computation means using more power (electricity), etc. Hearing aids are an extreme case in that regard because of their small size which induces reduced footprints for processors and less space for the battery. On the audio processing latency front (iii), DNN inference is notoriously slower for real-time audio DSP than “traditional” methods [^1], which is a problem in the context of devices processing sound of the real-world in real-time and where some synchronicity between what is heard and what is seen must be preserved. An acceptable latency for such applications should be inferior to 10 ms [^4], a figure that can currently only be obtained with specific systems which are not open or programmable. In that regard, programmability (iv) is another limiting factor and an essential point that should be taken into account if we want such systems to be adaptable.

In this context, the goal of this PhD is to take a multifaceted approach towards delivering new solutions for cutting-edge AI-based embedded real-time audio DSP with frugality and accessibility on mind. For that, several paths will be explored, relying on emerging programmable embedded architectures such as embedded Neural Processing Units – NPUs (i.e., ARM Ethos-U, STM32N6, etc.), smartphones, and custom architectures for AI-based real-time audio DSP relying on FPGAs (Field-Programmable Gate Arrays).

A first task will consist of evaluating the use of embedded NPUs in the context of DNN-based real-time audio DSP and to build a working prototyping environment with an operational audio processing workflow based on this kind of platform: Analog Audio Inputs/Microphones → DNN Inference on Embedded NPU → Analog Audio Outputs/Speakers. In a second phase, various classes of algorithms will be run on this system and potentially optimized for energy saving and computational efficiency in general. This will take the form of DNN quantization and arithmetic optimization building upon previous work on this topic [^3][^5]. We will also look into automating this process to facilitate the deployment of future algorithms. By the end of this task, an embedded AI-based real-time audio DSP workflow adaptable to a broad range of applications related to real-time audio DSP (noise canceling, smart sound systems, musical instruments, hearing aids, etc.) will be delivered.

In parallel with this, the use of smartphones as a solution for deploying accessible embedded AI-based real-time audio DSP systems will be explored. Smartphones provide a large amount of computational power (including AI accelerators in some recent models), large batteries, etc. More generally, they display excellent performances for AI-based real-time audio DSP [^2]. They are also very accessible (low cost) and most people already possess one. The main limiting factor preventing them from being used for real-time audio applications is latency. Three potential sources of latency can be identified in this context: (i) Operating System (OS) latency, (ii) bluetooth audio latency if no physical audio jack connector is available, and (iii) AI inference latency. One of the main goals of this task will be to lower the latency induced by i, ii, and iii. The work that will be carried out in the previously mentioned task towards improving the performances of AI inference algorithms on embedded devices will serve as a basis for tackling (iii), investigating specific optimizations tailored to smartphones.

Alongside explorations on embedded NPU platforms and smartphones, optimized DNN inference architectures will be prototyped on FPGA as well. To facilitate this complex work, we will heavily rely on High-Level Synthesis (HLS) tools in general, as well as [[Brevitas]] [^6][^7] and [[FINN]] [^8] for DNN quantization while maintaining an inference with high numerical accuracy. One idea is to target multiplierless parallel architectures, where constant multiplications are replaced by bit-shifts and additions. This will translate into an adder-aware training that finds the quantized fixed-point coefficients minimizing the number of adders and hence improving the area, latency, and power [^5]. The end goal is come up with an architecture providing an audio latency suitable for real-time audio DSP applications while being as energy and computationally efficient as possible.

The systems that will be developed as part of this PhD will be tested in the context various practical applications, ranging from hearing aid prototypes to musical instruments/devices (i.e., guitar pedal effects, sound synthesis modules, etc). As a member of the Emeraude team, the successful candidate will have access to GRAME-CNCM (computer music center based in Lyon) facilities, fostering potential collaborations with artists, etc.

**References**

[^1]: Gregorio Giudici, Franco Caspe, Leonardo Gabrielli, Stefano Squartini, and Luca Turchet. Distilling DDSP: Exploring real-time audio generation on embedded systems. Journal of the Audio Engineering Society, 73:331–345, 06 2025.  
[^2]: Jason Hoopes, Brooke Chalmers, and Victor Zappi. Neural audio processing on android phones. In Proceedings of the 2024 International Conference on Digital Audio Effects, 2024.  
[^3]: Malek Itani, Tuochao Chen, Arun Raghavan, Gavriel Kohlberg, and Shyamnath Gollakota. Wireless hearables with programmable speech AI accelerators. arXiv preprint arXiv:2503.18698, 2025.  
[^4]: Andrew P McPherson, Robert H Jack, and Giulio Moro. Action-sound latency: Are our tools fast enough? 2016.  
[^5]: Tobias Habermann, Jonas K¨uhle, Martin Kumm, and Anastasia Volkova. Hardware-aware quantization for multiplierless neural network controllers. In 2022 IEEE Asia Pacific Conference on Circuits and Systems (APCCAS), pages 541–545. IEEE, 2022.  
[^6]: Giuseppe Franco, Alessandro Pappalardo, and Nicholas J Fraser. Xilinx/brevitas, 2025.  
[^7]: Devansh Chawda and Benaoumeur Senouci. Fast prototyping of quantized neural networks on an FPGA edge computing device with [[Brevitas]] and [[FINN]]. In 2024 Fifteenth International Conference on Ubiquitous and Future Networks (ICUFN), pages 238–240. IEEE, 2024.  
[^8]: Michaela Blott, Thomas B Preußer, Nicholas J Fraser, Giulio Gambardella, Kenneth O’brien, Yaman Umuroglu, Miriam Leeser, and Kees Vissers. [[FINN|FINN-R]]: An end-to-end deep-learning framework for fast exploration of quantized neural networks. ACM Transactions on Reconfigurable Technology and Systems (TRETS), 11(3):1–23, 2018.

## Main activities

**Main Activities**

- Carry out the research related to the PhD topic described in the Assignments section.

**Additional Activities**

- Write papers
- Write a thesis
- Fulfill requirements for graduating, etc.

## Skills

**Key Technical Skills**

The "ideal" candidate should have the following skills (with some level of flexibility):

- Masters degree in computer science (or equivalent)
- Advanced C++ programming
- Low level digital audio systems
- Audio Digital Signal Processing
- Embedded Systems
- Good background in AI in general.

**Languages**

- Fluent written and spoken English (all communication within the lab happens in English and the PhD thesis will be written in English)

**Relational skills**

- Independent
- Team work