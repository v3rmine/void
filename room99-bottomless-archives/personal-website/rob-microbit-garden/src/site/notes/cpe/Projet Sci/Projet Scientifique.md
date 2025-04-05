---
{"dg-publish":true,"permalink":"/cpe/projet-sci/projet-scientifique/","tags":["EverGreen"]}
---


Topics: [[CPE\|CPE]]

# Objectifs
Réaliser d'une part un simulateur d'incendie permettant la création, le suivi et la propagation de feux de différents types (localisés sur une carte), et d'autre part de créer un dispositif de gestion de services d'urgences permettant, à partir d'informations collectées par des capteurs, de déployer et gérer les dispositifs adaptés pour éteindre les incendies. 

![archi-projet-sci.png](/img/user/cpe/Projet%20Sci/archi-projet-sci.png)

# TODO
- [x] Liste tests (unitaires & E2E) IOT
- [x] Diagramme séquence
- [x] Protocole réseau
- [x] Structure de projet
- [x] RFCs [[cpe/Projet Sci/rfcs/0000-template\|0000-template]]

# Jalons
- [x] TD: Présentation architecture globale et gestion du projet 📅 2022-12-08 ✅ 2022-12-08
- [x] TD: Présentation conception logicielle. **Diagramme de classe / séquence** et Schéma BD 📅 2022-12-13 ✅ 2022-12-14
- [x] TD: Présentation / démo chaîne IOT 📅 2023-01-04 ✅ 2023-01-05
	- [x] Démo collecte, envoi et réception des données des feux dans la ville ainsi que génération d'appel REST.
- [x] TD: Soutenance + démos finales 📅 2023-01-17 ✅ 2023-01-23
<!--
## Pourquoi du Rust en IOT est pertinent ?
- Support aisé de nouvelles plateformes si la target est supportée (no_std)
- Ref. RustConf 2022 contrôle des trains https://youtu.be/qaj5q88eLjk
- Un compilateur qui aide à éviter toutes les erreurs d'allocations / d'accès indexés
- Une création et implémentation de protocole simplifié grâce à [serde](https://lib.rs/serde) et [nom](https://lib.rs/nom)
- Documentation spécialisée pour le MicroBit ([MicroRust](https://droogmic.github.io/microrust) [Librairie microbit V1](https://lib.rs/crates/microbit))
- Librairie basée sur une [couche d'abstraction](https://github.com/nrf-rs/nrf-hal) pour les appareils NRF TD: Présentation / démo chaîne IOT 📅 2023-01-04
- Compatible avec [l'écosystème d'outils Knurling](https://knurling.ferrous-systems.com/tools/) 
- Radio examples : [nrf-rs/microbit/pull/90](https://github.com/nrf-rs/microbit/pull/90/files)
-->
# Conception IOT
## Structures initiale des données simulées
- Matrice `6x10` capteurs
- Valeur intensité du feu `0-9` 
- `(col, line, intensity)`

## Données capteur
| Donnée      | Besoin                                      | Solution                        |
| ----------- | ------------------------------------------- | ------------------------------- |
| Identifiant | Identifier unicité                          | ID dépendant du concentrateur                  |
| Latitude    | Localisation                                | Ligne matrice                   |
| Longitude   | Localisation                                | Colonne matrice                 |
| Température | Identifier feu                              | stockage température du capteur |
| Type        | Différencier capteur & relai dans le réseau | Enum type de device             |

### Calcul de l'ID
- Envoie d'une requête de demande d'ID au concentrateur (ref DHCP) en utilisant l'id physique du capteur
- Les capteurs sur le chemin stockent l'id physique temporairement en attendant l'id interne (aucun broadcast n'est effectué), TTL de 15s
- Le concentrateur regarde sa table de capteurs et génère un ID pour le capteur puis le renvoie au capteur qui lui a relayé l'id
-  La chaine de capteur rajoute l'id logique dans la table des capteurs forward au capteur suivant puis supprime l'id physique de la table temporaire
- Le capteur initial se débloque et enregistre son id du réseau

#### Cas particulier pas de relai ?
- Manager = Plus petit ID de mesh
- Si pas d'ID de mesh, plus petite adresse physique

### Trust on first use & chiffrement
<!-- - Rust: [github:dalek-cryptography/curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) -->
- Théorie : curve25519 (Asymétrique)
- Réalité : AES-256 (Symétrique)

#### Données réseau capteur
- Etat du capteur
	- Connecté au relai directement
	- Connecté au relai indirectement
	- Seul
- Table des capteurs (ref ARP) - tri par distance, puis par charge (plus faible), puis par ID de mesh
	- Distance en hop avec le capteur
	- Distance en hop avec le relai
	- Charge du capteur / relai (nombre de capteurs connectés)
	- Timestamp dernière MAJ

### Proto radio / UART
Objectifs : Mesh, résistance aux interférences, check is alive
- Les broadcast physiques ne sont pas repartagés
- Les broadcast logiques sont repartagés
- Soft max 125 de charge / device (avec saturation possible)

### Découpage trame
> [lancaster-university.github.io/microbit-docs/ubit/radio/#capabilities](https://lancaster-university.github.io/microbit-docs/ubit/radio/#capabilities)
> Typically 32 bytes, but reconfigurable in code up to 1024 bytes.
> 4 trames stockées en RAM (16KB de RAM)

Here : [landcaster-university/microbit-dal/inc/drivers/MicroBitRadio.h#L68](https://github.com/lancaster-university/microbit-dal/blob/602153e9199c28c08fd2561ce7b43bf64e9e7394/inc/drivers/MicroBitRadio.h#L68)

### Trame statique
- **Une évolution possible serait de faire une trame de taille dynamique pour diminuer la saturation réseau**
- [x] TD: Ajout `Request ID` => identifiant de la requête sur le système source ✅ 2022-12-13

| Taille en bits | Octets | Nom                     | Justification                                                                               |
| -------------- | ----- | ----------------------- | ------------------------------------------------------------------------------------------- |
| 8              | 1 |  Commande de trame       | 256 commandes différentes est largement suffisant                                           |
| 8              | 1 | Nombre de hop parcourus | 125 machines max passant par un microbit, donc aucune chance d'atteindre 256                |
| 32             | 4 | Adresse source          | Adresse physique / logique de taille max 32bits                                                           |
| 32             | 4 | Adresse destination     | Adresse physique / logique de taille max 32bits => broadcast = 0                                                          |
| 32             | 4 | Adresse forward     | Adresse logique de taille max 32bits                                                          |
| 8              | 1 | Request ID              | 0 -> 255                                                     |
| 8              | 1 | Partie de la requête    | 1 -> 255 (0 = Requête non partitionnée)                                                     |
| 8              | 1 | Nombre de parties       | 1 -> 255                                                                                    |
| 256           | 32 | Donnée                  | 32 octets max (taille aes256)                                                       |

### Différentes requêtes
| Request ID | Request Abbrv | Connection           | Ident               | Description                                            | Champs                 |
| ---------- | ------------- | -------------------- | ------------------- | ------------------------------------------------------ | ---------------------- |
| 0          | `HELOP`       | Broadcast            | Physical            | Utilisé pour découvrir l'état initial du réseau local  |                        |
| 1          | `OLEHP`       | Targeted             | Logical => Physical | Partage initial de la table de machines                | Table des machines     |
| 2          | `IDENT`       | Targeted             | Physical => Logical | Demande au manager de mesh un identifiant de mesh      | @Physique              |
| 3          | `TNEDI`       | Targeted             | Logical => Physical | Assigne un identifiant de mesh                         | @Phy, @Mesh            |
| 4          | `TRUSTB`      | Broadcast (depth -1) | Logical             | Partage la clef publique du manager                    | pubkey                 |
| 5          | `TRUSTT`      | Targeted             | Logical => Logical  | Partage la clef publique de la machine                 | pubkey                 |
| 6          | `HELOL`       | Broadcast (depth 1)  | Logical             | Met a jour de manière forcée la table des machines     |                        |
| 7          | `OLEHL`       | Targeted             | Logical => Logical  | Partage de la table de machines si adresse est connue  | Table des machines     |
| 8          | `PUSH`        | Targeted             | Logical => Logical  | Envoie une mise à jour d'un capteur                    | Valeur/Type/Id capteur |
| 9          | `PUSH_ACK`    | Targeted             | Logical => Logical  | Confirme que les données ont correctement été envoyées | Capteur req ID         |
| 10         | `ADENY`       | Targeted             | Logical => Physical | Accès non autorisé                                     | Capteur req ID         |
| 11         | `UDENY`       | Targeted             | Logical => Logical  | Inconnu dans la table d'identifiants                   | Capteur req ID         |
| 12         | `ADD`         | Broadcast            | Logical             | Broadcast de la machine dans le réseau                 | Info machine           |
| 13         | `DEL`         | Broadcast            | Logical             | Perte d'une machine dans le réseau                     | Adresse machine        |
| 14         | `ALIVE`       | Broadcast            | Logical             | Une machine est vivante dans le réseau                 | ID machine             |

#### Chiffrement des requêtes
| Request Abbrv | Partie chiffrée   |
| ------------- | ----------------- |
| `HELOP`       | Clair             |
| `OLEHP`       | Clair             |
| `IDENT`       | Clair             |
| `TNEDI`       | Clair             |
| `TRUST`       | Clair             |
| `HELOL`       | Clair             |
| `OLEHL`       | Clair             |
| `PUSH`        | Données chiffrées |
| `PUSH_ACK`    | Données chiffrées |
| `ADENY`       | Clair             |
| `UDENY`       | Clair             |

## Diagrammes séquence
<!-- 
Génération des diagrammes avec python:
- Diagramme qui fonctionne avec Krokio.io
- `python -c "import sys; import base64; import zlib; print(base64.urlsafe_b64encode(zlib.compress(sys.stdin.read().encode('utf-8'), 9)).decode('ascii'))"`
- <Ctrl-D>
- https://kroki.io/<diagtype>/<svg/png/pdf>/<base64 renvoyée par python>
-->

### Dans le cadre de la simulation
![pfp-scope.excalidraw.png](/img/user/cpe/Projet%20Sci/pfp-scope.excalidraw.png)

#### Restitution données capteur
```
seqdiag {
  "Collecteur" -> "Serveur relai" [label = "PFP(uart) cmd: PUSH\n(CID: 1\n,Type: Temp\n,Val: 1000)"];
  "Serveur relai" -> "Serveur relai" [label = "Interprétation de la mise à jour de donnée capteur"];
  "Serveur relai" -> "Broker Mosquito" [label = "Ajout d'une entrée\nvia le broker"];
  "Serveur relai" <-- "Broker Mosquito";
  "Collecteur" <-- "Serveur relai" [label = "PFP(uart) cmd: PUSH_ACK"];
}
```

![Diag](https://kroki.io/seqdiag/svg/eNqVkM1Kw0AUhfd9isNsbMFA3KYq1IhYRAi0ujFFbjMXGZ3cSSeTgojv4rbPkRdzmpXUH3B5Dvec73Ja3mhDT3gbASp31nIVuPMKyTnUgv02Cni2ZBQeLK3Z4gyquCrGHfkwQVXrDMXd4rqUcT6_zHBSyvHyteEMS66bKO7JRjdN04laTfeUg9Y_QXMJ7Bvf7wIF4wSaYQm1aRn9B55dzERLO5F-x6ioGZ7_lXPh3Qt73Lp205ngvpJmsSxAH3XCYAkRyaVsDcEy1kPs59rTJPneOz1cc7j6z5yPs_xmD3wffQJwxofI)

#### Simulation d'un feu
```
seqdiag {
  "Controlleur de simulation" -> "Serveur de simulation" [label = "JSON (HTTP)\n(Paramètres du feu)"];
  "Serveur de simulation" -> "Capteurs simulés" [label = "PFP(uart) cmd: PUSH"];
  "Capteurs simulés" -> "Collecteur" [label = "PFP(radio) cmd: PUSH\n(CID: 1\n,Type: Temp\n,Val: 1000)"];
  "Capteurs simulés" <-- "Collecteur" [label = "PFP(uart) cmd: PUSH_ACK"];
  "Serveur de simulation" <-- "Capteurs simulés";
  "Controlleur de simulation" <-- "Serveur de simulation";
}
```

![Diag](https://kroki.io/seqdiag/svg/eNqFkctqwkAUhvd5isOsEjAQt4kWJKWohXYgaTcqcuqcSmBy6VwEEd9Hn8MXa6IuRKMuZ37O952Lpj-R4RI2DgCLy8KoUkqyCgSBznIr0WRlwcB_AZaQWrVEE4k_JKEPbJx8foA7TFPuTQuXo8L8sDOKNAgLv2Q9Nosazx1Q44ixMnWkT8Fhry_5_I27FpXxYJGLEPhXMjwTW8qOtGaYRZNcYxSKrLzg1P3Go9cQutOik64rCiGlvKof3yjr3yAIvPuqnu8_cl21PB_E748XceLdeKInNzqWtSMjZ-v8A12roKk=)

### Dans le cadre du protocole (cas réel)
#### Connection d'un nouveau capteur
![scope-new-mesh.excalidraw.png](/img/user/cpe/Projet%20Sci/scope-new-mesh.excalidraw.png)
```
seqdiag {
  "Capteur P\n(serial 2)" -> "Capteur X\n(serial 1)" [label = "PFP(radio) cmd HELOP"];
  "Capteur X\n(serial 1)" -> "Capteur X\n(serial 1)" [label = "Pas de manager\ndéfini, plus\npetit serial 1\n=> manageur\n= capteur X"];
  "Capteur P\n(serial 2)" <-- "Capteur X\n(serial 1)" [label = "PFP(radio) cmd OLEHP\nAvec X en manageu réseau"];
}
```

![Diag](https://kroki.io/seqdiag/svg/eNqVj8EKgkAYhO89xeBJIQ91LYUIw4OgRyE7_Ll_sqCb7WqX6IF8Dl8sg8TwVOeZ-WbG8E1IKvBYANae6oZbjSRTtmEtqcTaseD6k5RO0mqQjiWduYQHKzkktiYhrw7ySiAMojixTptv7Cz7G5YMBKMiRQXrTIm-u0gll6jL1mSq5kY2GJOZ8vyPtR28HvIRP1syO7h13f8fxlEQDpzdnXOkYDUWQ_edYWrflc_FC8FZcK4=)

#### Perte d'un capteur
![losing-sensor.excalidraw.png](/img/user/cpe/Projet%20Sci/losing-sensor.excalidraw.png)
```
seqdiag {
  "Capteur X";
  "Capteur P";
  "Collecteur";
  "Capteur X" -> "Capteur X" [label = "Vérification\ndes TTL"];
  "Capteur X" -> "Capteur X" [label = "Machine L perdue"];
  "Capteur X" -> "Capteur X" [label = "Maj de la table des capteurs\nconnectés"];
  "Capteur X" -> "Collecteur" [label = "PFP(radio) cmd: DEL\n(Id logique capteur L)"];
  "Collecteur" -> "Collecteur" [label = "Maj de la table\ndes capteurs\nconnectés"];
  "Capteur X" <-- "Collecteur";
  "Capteur X" -> "Capteur P" [label = "PFP(radio) cmd: DEL\n(Id logique capteur L)"];
  "Capteur P" -> "Capteur P" [label = "Maj de la table\ndes capteurs\nconnectés"];
  "Capteur P" -> "Collecteur" [label = "PFP(radio) cmd: DEL\n(Id logique capteur L)"];
  "Collecteur" -> "Collecteur" [label = "Maj de la table\ndes capteurs\nconnectés"];
  "Capteur P" <-- "Collecteur";
  "Capteur X" <-- "Capteur P";
}
```

![Diag](https://kroki.io/seqdiag/svg/eNrVk00KwjAQhfee4pGVLnoB_zb-gFAhiyKCdRGTUSMxsX8r8UCew4upVLEKxaorly8z-WbeC0koUlqssK8BrCd2KWUxpqxVlPwmnTEkryfP5SmD132SMyMWZNABm5yOsV5qKVLtbGgVJQgCn80rA8ZCrrUl-NhRrDL66OoGimAEUrEwhOtwmTcmoZXO2oub0zEpQT7cFph8yOuxUNo1ILeqif7AD219pGDcSkcZ3QfAb9yxBU4592XXPKlqy7Y9r_Lb8B-9PDCl1G-d8H-Knb-PPa8XftChdgZbphUm)
