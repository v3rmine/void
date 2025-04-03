# sis-server
Par v3rmine pour la nuit de l'info 2022

<!--ts-->
* [sis-server](#sis-server)
   * [Le projet en 6 paragraphes](#le-projet-en-6-paragraphes)
      * [Resumé](#resumé)
      * [Objectif](#objectif)
      * [Sujet](#sujet)
      * [Cible](#cible)
      * [Client =&gt; SIS ASSOCIATION](#client--sis-association)
      * [Sources](#sources)
   * [Dependances &amp; Compilation](#dependances--compilation)
      * [Avec nix](#avec-nix)
      * [Sans nix](#sans-nix)
   * [Technologies](#technologies)
   * [Défis](#défis)
      * [Ne rien faire, c’est parfois mieux](#ne-rien-faire-cest-parfois-mieux)
      * [Submit me if you can !](#submit-me-if-you-can-)
      * [Docker](#docker)
         * [Sans nix =&gt; taille de l'image 8.46MB](#sans-nix--taille-de-limage-846mb)
         * [Avec nix =&gt; taille de l'image 18.3MB](#avec-nix--taille-de-limage-183mb)
      * [Easter egg](#easter-egg)
         * [Inspirations et sources](#inspirations-et-sources)
      * [Du propre](#du-propre)
         * [Principe suivi YAGNI](#principe-suivi-yagni)
         * [Pourquoi le Rust](#pourquoi-le-rust)
         * [Pourquoi Nix](#pourquoi-nix)
         * [Pourquoi une compatibilité avec Docker](#pourquoi-une-compatibilité-avec-docker)
         * [Pourquoi cibler musl](#pourquoi-cibler-musl)
         * [Extensions possibles](#extensions-possibles)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: code, at: Fri Dec  2 02:49:15 UTC 2022 -->

<!--te-->

## Le projet en 6 paragraphes
### Resumé
Vous devrez créer une application qui nous aidera à informer.

### Objectif
Développer un serious game en ligne (par exemple sous forme d’un jeu vidéo, un escape game, un jeu de rôle textuel, un quizz chronométré, un jeu de cartes contre une IA ou toute autre proposition)>

### Sujet
Sensibiliser aux problématiques liées à la santé sexuelle

### Cible
Le but est de sensibiliser les jeunes adultes (mais aussi les plus âgés)

### Client => SIS ASSOCIATION
- Association => budget d'hébergement limité
- Association => pas d'équipe dev dédiée, nécessité d'un usage /  d'uen installation simple et guidée

### Sources 
- https://www.sida-info-service.org
- https://www.sexualites-info-sante.fr
- https://www.hepatites-info-service.org
- https://www.vih-info-soignants.fr
- https://www.ligneazur.org
- https://www.instagram.com/sida_info_service
- https://www.instagram.com/sexualites_info_sante
- https://www.facebook.com/SidaInfoService

## Dependances & Compilation
*Génération du sommaire dans le markdown avec [ekalinin/github-markdown-toc](https://github.com/ekalinin/github-markdown-toc)*

### Avec nix
- nix (avec les [flakes](https://nixos.wiki/wiki/Flakes) activés)
Compilation avec un simple `nix build`, le binaire se trouve dans `result/bin/sis-server`  

### Sans nix
- rust 1.65
- cargo 1.65
- [mold](https://github.com/rui314/mold) (optionnel dev seulement)
- pkg-config
Compilation avec un `cargo xtask dist`, le binaire se trouve dans `dist/sis-server`

## Technologies
- Rust
- Nix
- Docker
- Musl

## Défis
### Ne rien faire, c’est parfois mieux
https://www.nuitdelinfo.com/inscription/defis/358  
Non réalisé sur le backend

### Submit me if you can !
https://www.nuitdelinfo.com/inscription/defis/347  
Non réalisé sur le backend

### Docker
https://www.nuitdelinfo.com/inscription/defis/330  

#### Sans nix => taille de l'image 8.46MB
- Avantages: Simplicité de Docker et image plus légère
- Désavantages: Une image non atomique
1. `docker build . -t sis-server`
2. `docker run --rm --name sis-server -P 8080:8080 sis-server`

#### Avec nix => taille de l'image 18.3MB
- Avantages: Aucun risque de problème de versions, elles sont toutes pins
- Désavantages: La complexité de nix 
1. `nix build .#docker`
2. `docker load < result`
3. `docker run --rm --name sis-server -P 8080:8080 sis-server`

### Easter egg
https://www.nuitdelinfo.com/inscription/defis/328

#### Inspirations et sources
- https://twitchplayspokemon.org
- https://github.com/danShumway/serverboy.js
- https://github.com/boa-dev/boa

### Du propre
https://www.nuitdelinfo.com/inscription/defis/370  

#### Principe suivi YAGNI
YAGNI => You aren't gonna need it  
Ce choix a été fait pour ne pas surcharger le projet avec des fonctionnalités qui ne seront pas utilisées. De plus, le choix du Rust en tant que langage, synergise avec ce principe, car il permet de faire une refacto plus tard, au besoin facilement et en évitant les erreurs. Nous pouvons donc nous concentrer sur le cœur des fonctionnalités sans avoir peur des futures mises à jour du code.

#### Pourquoi le Rust
- Pour les refacto sans peur, génération d'erreurs à la compilation pour tout ce qui ne serait pas géré, ainsi que les effets de bord causés par une éventuelle refacto.
- Pour le typage très fort permettant de réduire la charge cognitive de naviguer et développer pour le projet, la majorité des cas d'erreurs étant gérés par le typage fort.

#### Pourquoi Nix
- Afin d'avoir un build reproductible à 100%, toutes les dépendances y compris l'environnement sont épinglées, le projet pourra donc toujours être compilé tant que les sources sont accessibles. Il serait possible de le rendre encore plus péren si besoin en vendorant toutes les sources, nous perdrions cependant la mise à jour facilitée.
- Nix nous permettrait aussi grâce à NixOS de mettre en place des machines avec un système atomique, deux systèmes auraient les mêmes dépendances aux mêmes versions avec le même code.
- Nix nous permet ici en plus de générer une image Docker très simplement

#### Pourquoi une compatibilité avec Docker
- Pour faciliter le déploiement sur différentes machines aux architectures toutes aussi différentes.
- Pour rendre le projet compatible avec les outils de CI/CD les plus populaires.
- Pour permettre à des personnes qui ne connaissent pas Nix de pouvoir facilement utiliser le projet.
- Pour permettre de lancer le projet avec docker-compose ou encore kubernetes.

#### Pourquoi cibler musl
- Le ciblage de musl permet de générer des binaires statiques, ce qui permet de ne pas avoir à installer les dépendances sur la machine cible, ce qui facilite le déploiement.
- De plus, musl évite les problèmes de versions des librairies dynamiques ou encore les problèmes de version de libc.
- Musl nous permet donc d'appuyer le point de distribution atomique du programme et de diminuer le risque de différences de comportement entre les machines.

#### Extensions possibles
- Vérification du code à la recherche de code `unsafe` => https://github.com/rust-secure-code/cargo-geiger
- Vérification des dépendances pour savoir si elles ont subit un audit => https://github.com/mozilla/cargo-vet
- Suivis des regressions des performances => https://github.com/burntsushi/cargo-benchcmp
- Détection des comportements indéfinis (UB: Undefined Behavior) => https://github.com/rust-lang/miri
