# Projet 1 : Construire un CLI en Rust

Vous connaissez sans doute quelques commandes terminal telles que `cd`, `mkdir`, `cp` ou encore... `cat`.

Afin de mettre en pratique ce qu'on a vu jusqu'ici en Rust, on vous propose de recréer une version (_un peu plus humble_) de la commande `cat` !

A la fin de ce projet, on devrait être capable de reproduire le fonctionnement basique de la commande, qui est d'afficher le contenu d'un fichier dans la console.

Voici la syntaxe de la commande `cat`, que l'on va tenter de reproduire :

```sh
cat <file_path>
```

Ainsi, par exemple, lorsqu'on lance la commande `cat README.md`, cela nous affiche ce texte dans la console.

## Instructions

1. Ton programme doit permettre de reproduire le comportement de la commande `cat` en prenant une saisie utilisateur personnalisée (ex: `cat example.txt`, `cat bidule.md`, etc.)
2. Le code de ton programme doit au moins utiliser 1 struct (`File` par exemple...)
3. Ton programme doit permettre de renvoyer des erreurs compréhensibles par l'utilisateur. Cela implique de créer un `Error` spécifique, implémentant les traits nécessaires.
4. Le code de ton programme doit être découpé et rangé en modules

## Help !

<details>
<summary>Comment récupérer les arguments passés après la commande ?</summary>
Pour cela, la librairie std de Rust nous permet de récupérer les arguments via <strong>std::env::args</strong>. Une petite recherche sur la <a href="https://doc.rust-lang.org/std/">documentation de std</a> devrait t'aiguiller
</details>

<details>
<summary>Comment peut-on lire le contenu d'un fichier ?</summary>
Là aussi, on compte sur la librairie std pour nous filer un coup de main. Regarde du côté de <a href="https://doc.rust-lang.org/std/fs/index.html">std::fs</a>, tu devrais trouver ton bonheur.
</details>

➡️ Pendant que tu codes, pense à tester le fonctionnement de ton programme à l'aide de la commande `cargo run`. Pense à faire plusieurs `println!` pour afficher des valeurs dans la console, ça peut aider à s'y retrouver.

Ah, et pour info, tu peux passer des arguments à la commande `cargo run` :

```sh
cargo run -- <ici tes arguments>
```

