# Constants

pour déclarer une variable qui ne changera pas en JS, nous déclarons une constante avec le mot clé `const` au lieu de `let`:

```js
const myBirthday = '07.01.2000';
```

en faite en déclarant une constante en Javascript on bloque la réassigniation de notre variable, donc nous pouvons plus la ré assigner ensuite: 
```js
const myBirthday = '07.01.2000';

myBirthday = '01.01.2001'; // error
```

## Erreur commune

Une erreur commune avec les constantes en Javascript c'est de croire que la valeur de notre variable ne peut changer... parce que c'est pas tout à fait exact..

en faite ce qui est constant c'est le type de notre variable, nous verrons plus tard notamment au niveau des objects qu'on peut modifier la valeur dans une constante mais jamais son type


Mais bon, pour l'instant on peut juste retenir, qu'une constante c'est une variable qui ne changera pas.

## convention

il y à une convention pour les constantes qui dit de mettre en capital quand les valeurs de notre variables sont connu à l'avance

```js
const COLOR_RED = "#F00";
```

et pour les constante avec des valeurs qui doivent être calculer nous utiliserons plutôt du camelCase classique

```js
const pageLoadTime = /* temps de chargement de la page */;
```
