# Variables

Pour déclarer une variable en javascript nous utilisons le mot clé `let` suivis du nom de notre variable, par exemple:

```js
let myName;
```

Nous avons crée une variable `myName`;

Mais pour l'instant notre variable `myName` est vide car on ne lui a pas encore assigné une valeur. pour lui assigner une valeur il suffit d'écrire le nom de notre variable suivis d'un `=` puis la valeur qu'on veut lui donner. exemple:

```js
let myName;
myName = 'Elias';
```

Nous avons ici assigner la valeur `Elias` à notre variable `myName`.

...En revanche, c'est plutôt redondant de devoir l'écrire sur 2 lignes. pour régler ce soucis nous pouvons déclarer notre variable et lui assigner une valeur en même temps:

```js
let myName = 'Elias';
```

Nous pouvons aussi déclarer et assigner ( ou pas ) plusieurs variable sur la même ligne en les séparant par des `,` :

```js
let myName = 'Elias',
  myAge = 19,
  mySkills;
```
