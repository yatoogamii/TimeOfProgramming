# Variables

Pour <b>déclarer</b> une variable en javascript nous utilisons le mot clé `let` suivis du nom de notre variable, par exemple:

```js
let myName;
```

Nous avons crée une variable `myName`;

Mais pour l'instant notre variable `myName` est vide car on ne lui a pas encore <b>assigné</b> une valeur. pour lui <b>assigner</b> une valeur il suffit d'écrire le nom de notre variable suivis d'un `=` puis la valeur qu'on veut lui donner. exemple:

```js
let myName;
myName = "Elias";
```

Nous avons ici <b>assigner</b> la valeur `Elias` à notre variable `myName`.

...En revanche, c'est plutôt redondant de devoir l'écrire sur 2 lignes. pour régler ce soucis nous pouvons <b>déclarer</b> notre variable et lui <b>assigner</b> une valeur en même temps:

```js
let myName = "Elias";
```

Nous pouvons aussi <b>déclarer</b> et <b>assigner</b> ( ou pas ) plusieurs variable sur la même ligne en les séparant par des `,` :

```js
let myName = "Elias", myAge = 19, mySkills;
```

Voila pour la déclaration des variables.

Autre chose importante au niveau des variables, c'est les convention de nommages.

Vous aurez surement remarqué que j'ai nommé mes variables d'une manière un peu particulière... 

Déjà elles sont en anglais, c'est primordiale pour garder une cohérence avec les mot clé du language qui sont en anglais mais aussi pour que  n'importe quel autre développeur puissent comprendre vos variable sans devoir les traduire dans votre langue naturel.

Ensuite je n'ai pas mis d'espace pour séparer les mots par exemple dans `myName` pourquoi ça ? Parce que le language ne comprendrais pas qu'il s'agit de la même variable.

Pour palier ce soucis nous pouvons utiliser une majuscule pour séparer les mots comme dans `myName`, cette façon d'écrire s'appelle le camelCase. Nous pouvons aussi mettre un `_` à la place: `my_name` alors nous utiliserons du snake case.


Pour terminer cette parenthèse pour le nommage, en Javascript nos nom de variables ne peuvent contenir que:

- lettres
- chiffres
- symbol $ ou _

Et le premier character ne doit impérativement pas être un chiffre

Un exemple de nom invalide:

```js
let 7myName;
let my-name;
let my_£_name;
```

alors que ces nom sont totalement valide:

```js
let $myName;
let _myName;
let my_name;
```

Pour conclure ce tutoriel on va parler du mot clé `var`. il s'agit du mot clé anciennement utilisé pour déclarer une variable quand `let` n'existait pas encore donc, avant ES6 (ECMAScript 2015)

c'est possible que vous tombiez dessus dans de vieux script. la différence entre `let` et `var` reside dans la portée des variables.. mais nous verrons tout ça dans un prochain tutoriel.


