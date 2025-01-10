# TP-Dithering

## Membres du groupe

- Valentin ROMANET
- Arthur VILLETTE

## Question

### question 2

Pour ouvrir une image depuis un fichier en rust on utilise la librairie `image` qui permet de lire et d'écrire des images. Pour ouvrir une image on utilise la fonction `open` qui prend en paramètre le chemin de l'image à ouvrir. Cette fonction retourne un `Result` qui contient un objet de type `DynamicImage`. le type 'DynamicImage' est

### question 3

si l'image avais un canal Alpha cela veux dire que l'on a un canal de transparence en plus des trois canaux RGB. en passant en RGB8 on perd donc le canal alpha et donc la transparence de l'image cela impact donc l'image en elle même sur la transparence de celle ci.

### question 5

en passant un pixel sur deux d’une image en blanc, l'image est toujours reconnaissable mais l'image est très claire et on ne distingue pas les détails de l'image.

### question 6

pour récupérer la luminosité d’un pixel on utilise la formule suivante : `0.21 * r + 0.72 * g + 0.07* b` qui permet de calculer la luminosité d’un pixel en fonction de ses composantes RGB.
Les nombres imaginaire(0.21,0.72,0.07) viennent de recherches sur la perception de la luminosité par l'oeil humain. Comme lors de la conversion grayscale pour changer les couleur en nuange de gris.

### question 9
