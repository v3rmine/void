---
{"dg-publish":true,"permalink":"/cpe/rob-2/projet-bebop-2/"}
---


Topics: [[CPE\|CPE]], [[CPE ROB2\|CPE ROB2]]



# Question 1 : Quelles sont les quatre commandes classiques d’un drone quadrirotor ?
- Throttle → Augmentation / diminution de la vitesse des moteurs du même montant![Pasted image 20230417091004.png](/img/user/cpe/ROB2/Pasted%20image%2020230417091004.png) 
- Roll → Augmentation / diminution de la vitesse du moteur droit et diminution / augmentation de la vitesse du moteur gauche![Pasted image 20230417090951.png](/img/user/cpe/ROB2/Pasted%20image%2020230417090951.png)
- Pitch → Augmentation / diminution de la vitesse du moteur avant et diminution / augmentation de la vitesse du moteur arrière![Pasted image 20230417090939.png](/img/user/cpe/ROB2/Pasted%20image%2020230417090939.png)
- Yaw → Augmentation / diminution de la vitesse du couple moteur droit / gauche et diminution / augmentation de la vitesse du couple moteur avant / arrière![Pasted image 20230417090821.png](/img/user/cpe/ROB2/Pasted%20image%2020230417090821.png)

# Question 2 : Quelles sont les caractéristiques techniques de votre drone Bebop 2 ?
| Caractéristiques | Paramètres                                               |
| ---------------- | -------------------------------------------------------- |
| Weight           | 525g                                                     |
| Sensors          | Ultrasound, altimeter, visual positioning system         |
| Navigation       | GPS+GLONASS, gyro, accelerometer, magnetometer (compass) |
| Wi-Fi            | 802.11ac, MIMO with 2.4GHz, range up to 2km              |
| Battery          | 3350mAh removable, 30 minute flight Time                 |
| Camera           | 14Mp JPEG + DNG RAW, 1080p video 30fps                   |

# Question 3
## Quels topics correspondent à la commande du drone ?
| Topic                           | Description          |
| ------------------------------- | -------------------- |
| /bebop/autoflight/navigate_home |                      |
| /bebop/autoflight/pause         |                      |
| /bebop/autoflight/start         |                      |
| /bebop/autoflight/stop          |                      |
| /bebop/flattrim                 |                      |
| /bebop/flip                     |                      |
| /bebop/land                     | Atterissage          |
| /bebop/reset                    | Extinction d'urgence |
| /bebop/takeoff                  | Envol                |

## Quel est respectivement le type de chacun des messages associés ?
| Topic                           | Type           |
| ------------------------------- | -------------- |
| /bebop/autoflight/navigate_home | std_msgs/Bool  |
| /bebop/autoflight/pause         | std_msgs/Empty |
| /bebop/autoflight/start         | std_msgs/Empty |
| /bebop/autoflight/stop          | std_msgs/Empty |
| /bebop/flattrim                 | std_msgs/Empty |
| /bebop/flip                     | std_msgs/Empty |
| /bebop/land                     | std_msgs/Empty |
| /bebop/reset                    | std_msgs/Empty |
| /bebop/takeoff                  | std_msgs/Empty |

# Question 4 : Nous voulons afficher une courbe représentant l’altitude du drone en temps réel.  Cette altitude est disponible sur un topic. Quel serait alors le type de « plugin » rqt à utiliser ?
Il faut le plugin `plot` et le topic à utiliser est `/bebop/states/ardrone3/PilotingState/AltitudeChanged`.

# Question 5 : Proposez une spécification de commande de votre drone depuis le Joystick
| Bouton           | Commande  |
| ---------------- | --------- |
| LT               | -Throttle |
| RT               | +Throttle |
| Arrow Left       | +Yaw      |
| Arrow Right      | -Yaw      |
| Bouton du milieu | Reset     |
| Start            | Takeoff   |
| Back             | Land      |

## Joystick droit
| Position       | Commande      |
| -------------- | ------------- |
| Repos          | Stabilisation |
| Avant          | +Pitch, -Roll |
| Avant-Droit    | +Pitch        |
| Avant-Gauche   | -Roll         |
| Droit          | +Pitch, +Roll |
| Gauche         | -Pitch, -Roll |
| Arrière        | -Pitch, +Roll |
| Arrière-Droit  | +Roll         |
| Arrière-Gauche | -Pitch        |
