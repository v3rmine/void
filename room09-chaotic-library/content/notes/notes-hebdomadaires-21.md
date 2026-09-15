---
date: 2026-05-01
title: 'Notes hebdomadaires #21'
description: ""
draft: true
hidden: true
in_search_index: false
include_in_feeds: false
taxonomies:
  tags:
    - notes
    - 100 days to offload 2026
    - fr
extra:
  public_draft: true
  guid: '33bd703c-d47e-41c0-a53c-eb3919fce060'
  lang: fr
  comments: { host: tracto.pl, id: '', type: misskey }
---

4 semaines depuis la dernières notes hebdo alors que j'ai dit que je serais certainement plus régulière sur la publication de notes hebdo, mais j'ai pas réussi a résister à l'attrait de bosser sur le blog ou ma configuration {{ <al k="SilverBullet" /> }} plutôt que d'écrire

<!--
Lu la salope éthique
{{ <image url="/media/la-salope-ethique-book.jpg" alt="Couverture du livre " height={400} full_size={true} /> }}
-->

[Bloom Sushi](https://bloomsushi.fr/), un sushi VG c'était surprenant mais très bon !

Implémenté un [plugin](https://git.v3rm.in/v3rmine/silverbullet-config-file-editor/) pour éditer les fichiers toml et yaml directement dans {{ <al k="SB" /> }} et c'était une purge la doc était vraiment pas clair la dessus donc fallait se baser sur les codes sources de {{ <url link="https://github.com/MrMugame/silverbullet-pdf/" /> }} et {{ <url link="https://github.com/LogeshG5/silverbullet-excalidraw/" /> }} (1j de taff...)

J'ai aussi ajouté la publication sur Mastodon directement depuis SB (SilverBullet) et le pire c'était certainement l'oauth, {{ <al k="Sharkey" /> }} (utilisé par {{ <url link="https://tracto.pl" /> }}) ne supporte que l'auth avec redirect donc la solution c'est de rediriger vers un domaine `https://<oauth>.astriiid.fr?code=<oauth-code>` qui lui redirige vers une page virtuelle `oauth:mastodon:<oauth-code>` dans SB, ça permet de récupérer le code et finir l'authentification !

Comme par exemple le partage de [notes courtes](@/content/thoughts/_index.md) 

Avec l'upload de photos, quelle purge de faire du `form-data` manuellement pour générer un blob binaire que je peux envoyer via SB.

J'ai enfin réglé le problème de mes backups {{ <al k="autorestic" /> }} qui grossissaient à l'infini, il me manquait de passer par une étape `forget` pour les supprimer et `prune` pour supprimer les données qui ne sont plus utilisées.

> [!TIP]
>  Avec beaucoup de `locations` dans autorestic, c'est beaucoup plus rapide de faire d'abord un `forget` et ensuite d'utiliser `exec -- prune` (pour appeler le prune de {{ al(k="restic") }}) plutôt que d'utiliser `forget --prune` (l'option d'autorestic va supprimer les données non utilisées à chaque nouvelle `location` plutôt qu'en une fois)

Ajout de [Townsquare](https://townsquare.cauenapier.com/) dans le footer du site !

Ajout d'une [room matrix](https://matrix.to/#/#astriiid.fr:m.v3rm.in) pour le blog c'est une option de plus pour me poser des questions en dehors de mastodon ~

---

J'ai eu droit à un bug fameux sur le site, qui n'impactait "que" chrome et tout ses dérivés et connu depuis 2018 ! [Background-clip: text does not work with position: relative or absolute](https://issues.chromium.org/issues/41385122), un bug que je ne peux bien sûr pas contourner mais qui bonne nouvelle viens d'être patché ! {{ <customoji name="personas/astrid/Sparkling 1" /> }} <small>(enfin en Juin dans à partir de la version `150.0.7871.0`)</small>  
En bref et non technique, je ne pouvais pas avoir l'effet arc-en-ciel et l'effet de vague sur mon texte ! Exemple: {% <wavy> %}{{ <rainbow content="Ceci sera transparent sur chrome" /> }}{% </wavy> %}

J'ai découvert l'outil [Doodle Scan](https://rknight.me/blog/doodle-scan/) qui permet de retirer le fond d'un dessin dans un carnet, le tout directement dans le navigateur sans pub et sans IA. Je le garde dans un coin ça me sera utile un de ces 4 ! {{ <customoji name="personas/astrid/Noted 1" /> }} 
[thinking about friendships](https://blog.avas.space/friendships/) : It’s okay, I guess.


- [Fixing a bricked AMD 7040 series Framework 13” laptop with $20 tools | Quantum](https://quantum5.ca/2026/08/16/fixing-bricked-amd-7040-series-framework-13-laptop-with-20-tools/) : My experience fixing my Framework 13” laptop with AMD 7060 series that was bricked due to a failed BIOS update with US$20 worth of tools, instead of replacing the motherboard as suggested by Framework support.
- [CSS:the bomb inside your inbox | PortSwigger Research](https://portswigger.net/research/css-the-bomb-inside-your-inbox) : Gareth Heyes - gareth.heyes@portswigger.net - @garethheyes It's quite common for webmail clients to render untrusted CSS in a trusted UI. They attempt to make this safe using CSS sanitization. In this
- [I accidentally logged hundreds of thousands of phone calls to military bases - lina's blog](https://lina.sh/blog/hijacking-e164-arpa) : How an expired nameserver let me take over e164.arpa zones for multiple territories, and why I probably should have checked my logs sooner.
- [My DIY DEF CON Choker Has a Screen On It | The Cat House](https://www.scd31.com/posts/defcon-choker) : 
- [Random Note - Keepeuh](http://keepeuh.fr/blog/n4-notes-random/) : Blog personnel.
- [help wanted - lake hope](https://lake.computer/blog/help-wanted/) : 
- [Autistici/Inventati désigné « organisation terroriste internationale » par les États-Unis – Framablog](https://framablog.org/2026/08/31/autistici-inventati-designe-organisation-terroriste-internationale-par-les-etats-unis/) : C'est la rentrée ! Et le moins qu'on puisse dire, c'est qu'elle ne commence pas bien pour les hébergeurs alternatifs. Nous sommes un peu débordé⋅es aujourd'hui (retour de congés oblige), mais il nous paraissait quand même utile de vous transmettre sans attendre l'information qui suit. Tiré du site IndyMedia Nantes : Ce 26 août 2026,...
- [Autistici/Inventati désigné « organisation terroriste internationale » par les États-Unis – Framablog - Liens en vrac de sebsauvage](https://sebsauvage.net/links/?9DUrxA) : 
- [Capgemini continue de travailler pour l’ICE - Next](https://next.ink/brief-article/capgemini-continue-de-travailler-pour-lice/) : La filiale américaine Capgemini Government Solutions avait signé en 2025 plusieurs contrats avec l’ICE, l’agence américaine de l’immigration transformée…
- [Weird little guys of FOSS](https://drewdevault.com/weird-guys/) : 
- [PimpinPumpkin/Vela: Degoogled maps & turn-by-turn navigation for Android - MapLibre + Jetpack Compose, no Google Play Services](https://github.com/PimpinPumpkin/Vela) : Degoogled maps & turn-by-turn navigation for Android - MapLibre + Jetpack Compose, no Google Play Services - PimpinPumpkin/Vela
- [Enjeu de société : Ultra mini guide](https://enjeu.cc/?guide) : La vie avec du fun et du sens
- [IA, le mur, par Frédéric Lordon (Les blogs du Diplo, 4 septembre 2026)](https://blog.mondediplo.net/ia-le-mur) : Il faut se rendre à l'évidence : que le moral faiblit. La croyance IA, mieux que la dialectique, allait casser les briques, et la voilà qui flanche. Il y a des médisants et des malfaisants, qui trouvent à redire. Jusqu'ici ils étaient une anomalie sans conséquence, voilà qu'on les écoute, qu'on y ajoute foi même. Et le doute commence à s'insinuer : tout ceci – ces trillions en tous sens –, est-ce bien raisonnable ?
- [Notre voyage en Écosse - Juillet 2026 - Deadly Breakfast](https://weblog.redisdead.net/main/post/2026/08/16/Notre-voyage-en-Ecosse-Juillet-2026) : Prévu depuis la première semaine de Janvier, on l'attendait avec impatience, ce voyage : deux semaines en Écosse, une boucle passant par 7 destinations, 2 nuits sur place à chaque
- [Les US désignent les hacktivistes d’Autistici/Inventati comme organisation terroriste - Next](https://next.ink/253833/les-us-designent-les-hacktivistes-dautistici-inventati-comme-organisation-terroriste/) : Le collectif hacktiviste d’origine italienne Autistici/Inventati, qui fournit des services alternatifs d’hébergement et de messagerie, vient d’être…
- [La dernière pluie de pierres · Agate Blue](https://agate.blue/la-derniere-pluie-de-pierres/) : Bonjour, je suis OpenAI et j'ai le grand plaisir de lire le Temps des Pluie de Pierres, pour Aurrible.
- [ssii | Satanic Mechanic](https://www.satanicmechanic.fr/tag/ssii) : Les aventures d'une mécana trans
- [A/I Shuts Down Stay Human · Keep the Internet free](https://keepitfree.ai/announcements/a/i-shuts-down-stay-human/) : 
- [Autistici/Inventati annonce sa dissolution, et la fermeture prochaine de ses services - Next](https://next.ink/255087/autistici-inventati-annonce-sa-dissolution-et-la-fermeture-prochaine-de-ses-services/) : Onze jours après avoir été désigné comme organisation terroriste par l’administration Trump, le collectif hacktiviste d’origine italienne…
- [robida/human.json: A lightweight protocol for humans to assert authorship of their website content and vouch for the humanity of others. - Codeberg.org](https://codeberg.org/robida/human.json) : human.json - A lightweight protocol for humans to assert authorship of their website content and vouch for the humanity of others.
- [Jim Queen : la révolution queer ne sera pas bourgeoise](https://frustrationmagazine.fr/jim-queen/) : Frustration Magazine, média indépendant et offensif qui relate la guerre des classes.


