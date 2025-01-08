# TP-Dithering

## Membres du groupe

- Valentin ROMANET
- Arthur VILLETTE

## question 2

Pour ouvrir une image depuis un fichier en rust on utilise la librairie `image` qui permet de lire et d'écrire des images. Pour ouvrir une image on utilise la fonction `open` qui prend en paramètre le chemin de l'image à ouvrir. Cette fonction retourne un `Result` qui contient un objet de type `DynamicImage`. le type 'DynamicImage' est

## question 3

si l'image avais un canal Alpha cela veux dire que l'on a un canal de transparence en plus des trois canaux RGB. en passant en RGB8 on perd donc le canal alpha et donc la transparence de l'image cela impact donc l'image en elle même sur la transparence de celle ci.
