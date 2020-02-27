# Rust Installation

Pour installer rust sur notre machine il faut télécharger `rustup`, l'outil en ligne de commande pour gérer Rust (mettre à jour, gestionnaire de package, linter..)

sur linux ou macOS:
`$ curl https://sh.rustup.rs -sSf | sh`

le script d'installation va se télécharger puis installer rustup avec la dernière version stable de rust

pour avoir accès au commmande de rustup il vous faudra possiblement redémarrer le terminal ou source le path manuellement : `$ source $HOME/.cargo/env`

# Hello World

maintenant qu'on a installé rust, nous allons pouvoir créer notre premier programme et on va faire original pour une fois on va faire.. un Hello World

Pour ça on va créer un dossier pour y mettre notre code qu'on va appeler tout simplement **_hello_world_**

puis on va créer dedans un fichier appelé `main.rs`, `rs` étant l'extension pour du rust.

au niveau des conventions de nommage des fichier en rust, si il y a plusieurs mots dans le nom du fichier par exemple `hello world` alors on utilisera du snake case: `hello_world.rs`

Dans notre fichier `main.rs` nous allons écrire tout simplement:

```rs
fn main() {
  println!("Hello, world!");
}
```

Et voila notre Hello world en rust !

Il nous reste plus qu'à compiler le fichier en utilisant la commande `rustc` comme ceci: `$ rustc main.rs`

Nous pouvons ensuite executer le programme de cette manière : `./main`

# Anatomie d'une programme rust

revoyons un peu plus en détail notre programme..

## fonction

d'abord nous avons déclarer ce qu'on appelle une fonction, avec le mot clé `fn`.

Mais pas n'importe quelle fonction mais le fonction principal de notre programme, c'est à dire celle qui sera toujours executé en premier

notre fonction n'a aucun argument, si elle en aurait, il serait dans les parenthèses, et elle ne renvoie rien pour l'instant

en voit aussi que les accolades englobe le corps de notre fonction

Petite convention: mettre l'accolade ouvrante sur la même ligne que la déclaration de notre fonction

Pour savoir ce que rust conseille en terme de convention / formatting, c'est à dire si on doit mettre l'accolade sur la même ligne ou pas.. combien d'espace entre chaque élément etc, rust viens avec son propre outil de formating qui s'appelle `rustfmt` et on va pouvor taper dans notre terminal `$ rustfmt main.rs` ce qui automatiquement formater notre code en rust way ( beaucoup d'éditeur de code / IDE propose des outils interne pour formater directement à chaque sauvegarde du code )

## macro

ensuite dans notre fonction nous avons cette ligne `println!("Hello, world!");` qui nous permet d'afficher du text dans notre terminal.

Plusieurs point important ici

- nous devons indenter notre code avec 4 espaces et non des tabulations, c'est une convention de rust
- `println!` est ce qu'on appelle en rust une macro (ce serais une fonction si il n'y aurait pas le `!`) nous verra tout ça plus en détail plus tard, pour l'instant la chose à savoir c'est qu'avec un `!` nous utilisons une macro et non une fonction normal
- On voit notre chaine de caractère `"Hello, world!"` en tant qu'argument de `println!`
- Et pour finir nous utilisons un `;` pour indiquer que nous terminons cette expression

# Compiling and Running Are Separate Steps

Maintenant qu'on à terminer de revoir notre code, on va s'intéresser à la compilation et lexecution de notre programme.

Comme vous l'avez remarqué plus haut nous devons compiler notre programme comme ceci: `rustc main.rs` avant de pouvoir l'executer. ça permet de créer un executable binaire de notre programme qu'on pouvoir executer

executable qu'on peut partager à n'importe qui et qui aura juste besoin de l'executer pour voir le résultat sans devoir installer quoi que ce soit
