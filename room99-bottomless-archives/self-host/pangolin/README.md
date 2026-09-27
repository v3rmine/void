# pangolin setup

## Pangolin config
In [config.yml](config/config.yml) fill the `secret` the admin `email` and the admin `password`.

## Iocaine
### ai.robots.txt
```sh
curl -L https://github.com/ai-robots-txt/ai.robots.txt/raw/refs/heads/main/robots.json \
     -o iocaine-data/ai.robots.txt-robots.json
```

### Corpus
Here are the corpuses from the official doc:
```sh
curl -L https://archive.org/download/GeorgeOrwells1984/1984_djvu.txt \
     -o iocaine-data/corpus/1984.txt
curl -L https://archive.org/download/ost-english-brave_new_world_aldous_huxley/Brave_New_World_Aldous_Huxley_djvu.txt \
     -o iocaine-data/corpus/brave-new-world.txt
```

But if you need mines just ask me by mp/mail!

### Wordlists
```sh
curl -L https://git.savannah.gnu.org/cgit/miscfiles.git/plain/web2 \
     -o iocaine-data/corpus/words.txt
```

### Lophiomys Imhausi (my personal iocaine setup forked from Nam-Shub of Enki)
```sh
git clone https://git.sr.ht/~v3rmine/lophiomys-imhausi \
    iocaine-data/lophiomys-imhausi
```

### `ip-to-asn.mmdb`
You can get it here: https://github.com/sapics/ip-location-db

## Traefik iocaine plugin
```sh
# Official project: https://git.mstar.dev/mstar/traefik-iocaine-middleware 
# My fork (with vendored google/uuid):
git clone https://git.sr.ht/~v3rmine/traefik-iocaine-middleware \
    plugins-local/src/git.mstar.dev/mstar/traefik-iocaine-middleware
```
