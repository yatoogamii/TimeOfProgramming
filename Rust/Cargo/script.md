# Cargo c'est quoi ?

c'est le gestionnaire de package de rust, donc ce qui va nous permettre de télécharger des librairies mais aussi de build notre code

## Vérifier qu'il est bien installé

```sh
$ cargo --version
```

# Refaire un hello world avec Cargo

## Creer un projet avec Cargo

```sh
$ cargo new hello_cargo
$ cd hello_cargo
```

La première commande nous permet de créer le dossier avec le nom qu'on lui donne, en l'occurence `hello_cargo`

Ensuite on se déplace jusque dans ce dossier, et on va voir que Cargo nous à crée plusieurs element:

- Cargo.toml
- src/main.rs

### Cargo.toml

On a d'abord le Cargo.toml, il va s'agir du fichier de configuration de notre application.

la première section `[package]`

Par défaut on peut y voir, **le name**, **la version**, **les auteurs et l'édition**

et à la fin du fichier une section `[dependencies]`

qui va en faite nous permettre d'écrire le nom des packages qu'on va vouloir utiliser dans notre programme pour que cargo puisse les télécharger et les importer dans notre projet

### src/main.rs

Cargo à généré un simple hello world. La simple différence avec notre hello world dans la premiere vidéo c'est que Cargo à mis ce main.rs dans un dossier **src** et que nous avons donc maintenant un fichier Cargo.toml à la racine de notre projet

En faite Cargo va structurer notre projet de sorte qu'on est tous les fichier de configuration, de licence.. readme etc à la racine du projet, et de mettre le code en lui même dans le dossier src

# Build et run avec Cargo

On va maintenant voir la différence au niveau du building avec notre ancien hello world

## Cargo build

on va taper à la racine de notre projet cette ligne: `cargo build` tout simplement. Cette commande va nous permettre de créer un executable de notre programme et va le placer automatiquement dans `target/debug/hello_cargo`

nous pouvons ensuite le lancer directement via en tapant le path vers l'executable: `./target/debug/hello_cargo`

et nous avons notre joli `Hello world !`

la première fois qu'on va build notre application un nouveau fichier va se créer à la racine `Cargo.lock` c'est tout simplement un fichier qui va vérifier quel version de nos dépendances on utilise, c'est un fichier qui se créer / modifier tout seul donc pas besoin de toucher. si vous le supprimer sans faire exprès un petit coup de Cargo build réglera le problème

## Cargo run

On a vu comment run notre executable directement avec son path mais c'est un peu long et pas pratique alors **Cargo** nous permet de le run directement avec un: `Cargo run`

Autre chose importante nous avons vu ici que pendant l'execution de notre programme il n'y a pas eu de compilation, c'est tout simplement parce que nous n'avons pas modifier notre fichier depuis notre premier build et donc il ne recompile pas le fichier si rien n'a changé

## Cargo check

Cargo nous permet aussi d'utiliser la commande `check` pour vérifier rapidement si notre code compiles sans soucis mais sans créer l'executable, ce qui nous permet de gagner pendant de temps et d'utiliser `Cargo build` plutôt quand on veut tester notre application et pas juste vérifier si le code compile bien

## Cargo build --release

Pour finir sur les commande Cargo on va revenir sur la commande `build`. Quand on est prêt à sortir une release de notre application donc une version qui va être utiliser et non juste testé. nous pouvons taper : `Cargo build --release` ce qui créer un executable cette fois ci dans `target/release` et non dans `target/debug` et ça va permettre à rust d'optimiser votre code pour le rendre plus rapide. ça va en revanche prendre un peu plus de temps pour compiler
