# data types

une variable en Js peut contenir n'importe quelle type de donnée et elle peut être à un moment un string, et dans un autre moment un number

```js
let message = "hello";
message = 123456;
```

c'est ce qu'on appelle du typage dynamique

## Liste des types en JS

En Javascript on compte 8 type de données dite basique

### Number

Le type number représent un entier ou un chiffre à virgule

La liste des opérateur arithmétique:

- `+` addition
- `-` soustraction
- `*` multiplication
- `**` puissance
- `%` module
- `++` incrémentation
- `--` décrémentation

En dehors des entier ou chiffre à virgule, nous avons des valeurs numérique dite **spécial** `Infinity`, `-Infinity` et `NaN`

#### Infinity

représente l'infini mathématique `∞`, une valeur spécial plus grande que n'importe quel autre nombre

on peut l'avoir en divisant `1 / 0` par exemple

#### NaN

représente `Not a Number`, donc une erreur par exemple si on essaye de divisé un string par un nombre

```js
"not a number" / 2;
```

Javascript permet de faire de maths de manière `safe`, dans l'idée ou on peut divisé par 0, faire tout ce qu'on veut avec des calcules, jamais notre script s'arretera avec une erreur, dans le pire des cas on récupera `NaN`

### BigInt

"In JavaScript, the “number” type cannot represent integer values larger than 2 ** 53 (or less than -2 ** 53 for negatives), that’s a technical limitation caused by their internal representation. That’s about 16 decimal digits"

Mais parfois on à besoin d'aller plus loin, par exemple pour de la cryptography ou des précision à la microseconde

Et c'est pour ça que BigInt à été ajouté assez récement pour définir un nombre d'une taille plus élévé que 16 chiffres

On crée un BigInt en ajoutant un `n` à la fin de notre nombre

### String

Une chaine de caractère en javascript doit être entouré de guillemets:

- `"Hello"`
- `'Hello'`
- `Hello`

Les simple et double guillemets sont des guillemets basique, il n'y a pas de différence entre eux

les backticks permettre eux, d'intégré des variables ou des expression directement dans un string de cette manière:

```js
let name = "Elias";

alert(`Hello, ${name}`);

alert(`The result is ${1 + 2}`);
```

L'expression mis dans `${}` est évalué et le resultat devient une partie de la chaine de caractere

### Boolean

Le type Boolean permet que 2 valeurs: `true` ou `false`

il est donc souvent utilisé pour stocker un `oui` (qui voudrais dire "oui c'est correct") ou un `non` (qui voudrais dire "non c'est incorrect"): par exemple:
```js
let isFrench = true;
```

il peut aussi être obtenue dans des comparaisons:
```js
let isGreater = 4 > 1; // true
```

### Null

la valeur `null` est un peu spécial car il ne peut contenir que `null` 

En Javascript `null` signifie "Référence à un objet non existant", on peut dire que la valeur est vide, ou inexistante

### undefined

`undefined` comme `null` ne peut contenir que lui même donc `undefined`

une valeur `undefined` signifie qu'aucune valeur n'a été assigné

si notre variable à été déclaré mais non assigné, alors elle sera undefined:
```js
let x;

alert(x); // undefined
```

nous pouvons aussi lui assigné `undefined` manuellement:
```js
let x = undefined;
```

mais on recommande plutôt d'utiliser null pour des valeurs encore non existante et undefined plutôt pour vérifier si une variable à été asigné ou non

### Objects and Symbols

`object` est un type spécial, tous les autres types qu'on à vu avant sont appelé `primitifs` parce qu'il ne peuvent contenir qu'un chose, qu'une valeur à la fois. 

Les objects eux servent à contenir plusieurs valeurs à la fois, nous détaillerons plus tard les objects 

Les `Symbol` quant à eux, sont créer pour obtenir un identifiant unique pour un object, comme pour les objects nous le verrons plus en détails plus tard

## L'opérateur typeof

L'opérateur typeof en javascript nous permet de savoir le type d'une valeur on l'appeler de 2 façons:
```js
typeof true // boolean

typeof("test") // string
```

petite parenthèse pour le type null dans un typeof, Javascript nous renverra un type `object`. c'est une erreur connu du typeof, c'est faux
