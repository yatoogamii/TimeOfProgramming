# Variable

Pour déclarer une variable en rust on utilise le mot clé `let`, exemple:

```rust
fn main() {

  let x = 5;
  println!("The value of x is: {}", x);

}
```

Si on run le code précédent ça nous affichera: `The value of x is: 5`

Maintenant si je veux modifier ma variable:

```rust
fn main() {

  let x = 5;
  println!("The value of x is: {}", x);

  x = 10;
  println!("The value of x is: {}", x);

}
```

Si on essaye de run notre programme ici, ça provoquera une erreur... pourquoi ?

# Immuability

Parce que par défaut les variables en rust sont immuable, donc elle ne peuvent être changer, c'est notamment ce qui permet de garder un code plus safe. Rust nous "conseille" de garder nos variables immuable mais, si pour x raisons on veut pouvoir modifier notre variable il propose quand même une option

# Mutability

Pour rendre une variable mutable nous devons déclarer le mot clé `mut` entre le nom de la variable et le mot clé `let`, nous pouvons maintenant changer la valeur de notre variable et indiquer au future lecteur de ce code que la valeur changera plus tard

```rust
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 10;
  println!("The value of x is: {}", x);
}
```

Nous pouvons maintenant run ce code pour obtenir:
```
The value of x is: 5
The value of x is: 10
```

En dehors de la prévention de bugs, on peut gagner des performances par exemple si on utilise une structure de donnée assez large, pouvoir modifier cette structure sera plus rapide que de tout copier avec d'autres valeurs. avec un structure de donnée plus petite, créer une nouvelle instante de notre structure pourrait être plus facile à faire. Même si on perdrais un peu de performance on gagnerait beaucoup en clarté 

# Différence entre Variables et constantes

Maintenant on va parler de constantes ! ce principe assez commun dans la programmation. Rust permet lui aussi de créer des constantes, donc des variables immuable comme les variables par défaut finalement.. mais il y a évidemment des différences..

- Premierement, on ne peut pas utiliser `mut` sur une constante. les contantes sont juste toujours immuable 

- Deuxiement, on déclare une constante en utilisant le mot clé `const` et le type de notre variable doit être mentionné. On verra prochainement les différent types en rust 

- Troisiement les constantes peuvent être déclaré et utilisé dans n'importe quel scope (global scope compris), c'est vraiment utile si on veut avoir accès à une donnée sur plusieurs niveau (on va voir ça à la fin)

- Dernierement une constante pour être défini seulement par une expression constante (hard-coded), c'est à dire une valeur non calculée au moment de l'execution

Une exemple de constante:
```rust
const MAX_POINTS: u32 = 100_100;
```

Donc on utilisera des constantes pour des valeurs qu'on veut réutiliser plusieurs fois sur plusieurs niveau différent en y stocker des valeurs hard-coded. Nommer les valeurs comme ça permet de rendre le code plus facilement à récuperer pour des gens qui verront le code plus tard, c'est aussi plus simple de changer la valeur à un seul endroit plutôt qu'a plusieurs... voila plusieurs raisons pour utiliser des constantes
