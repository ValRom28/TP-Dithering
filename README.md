# TP-Dithering

## Membres du groupe

- Valentin ROMANET
- Arthur VILLETTE

## Question

### question 2

Pour ouvrir une image depuis un fichier en rust on utilise la librairie `image` qui permet de lire et d'écrire des images. Pour ouvrir une image on utilise la fonction `open` qui prend en paramètre le chemin de l'image à ouvrir. Cette fonction retourne un `Result` qui contient un objet de type `DynamicImage`. le type 'DynamicImage' est une représentation d'image flexible, qui peut contenir des données d'image dans différents formats.

### question 3

si l'image avais un canal Alpha cela veux dire que l'on a un canal de transparence en plus des trois canaux RGB. en passant en RGB8 on perd donc le canal alpha et donc la transparence de l'image cela impact donc l'image en elle même sur la transparence de celle ci.

### question 5

en passant un pixel sur deux d’une image en blanc, l'image est toujours reconnaissable mais l'image est très claire et on ne distingue pas les détails de l'image.

### question 6

pour récupérer la luminosité d’un pixel on utilise la formule suivante : `0.299  * r + 0.587 * g + 0.114* b` qui permet de calculer la luminosité d’un pixel en fonction de ses composantes RGB.
Les nombres imaginaire(0.299 ,0.587,0.114) viennent de recherches sur la perception de la luminosité par l'oeil humain. Comme lors de la conversion grayscale pour changer les couleur en nuange de gris.

### question 9

pour calculer la distance entre deux couleurs on utilise la formule suivante : `sqrt((r1 - r2)^2 + (g1 - g2)^2 + (b1 - b2)^2)` qui permet de calculer la distance entre deux couleurs en fonction de leurs composantes RGB ses **L’écart de couleur**. on utilise juste la formule de la distance euclidienne entre deux points et on prend deux point de 3 dimensions (vecteur de taille 3) R.G.B.

### question 11

Si on a une pallette vide lors du dithering on ne peut pas appliquer le dithering car on ne peut pas trouver la couleur la plus proche de la couleur actuelle du pixel. Donc on regarde si notre palette est vide et si elle l'est on ne fait pas de dithering on retourne l'image original.

### question 13

On sait que `B0=0`, `B1= 1/4 * MAT((0,2),(3,1))`, B2 = (1 / 16) \*`

|   0 |  12 |   3 |  15 |
| --: | --: | --: | --: |
|   8 |   4 |  11 |   7 |
|   2 |  14 |   1 |  13 |
|  10 |   6 |   9 |   5 |

et que `BN+1 = 1/4 * ((4 * Bn, 4 * Bn + 2 * Un), (4 * Bn + 3 * Un, Bn + Un))` et `Un` est la matrice de Bayer d'ordre taille `2^n x 2^n`.

Donc:

`B3 = 1/4 * ((4 * B2, 4 * B2 + 2 * U2), (4 * B2 + 3 * U2, B2 + U2))`

B3 =
(1/64) \*
| 0 | 3 | 0.75 | 3.75 | 2 | 5 | 2.75 | 5.75 |
|----:|----:|------:|------:|----:|----:|------:|------:|
| 2 | 1 | 2.75 | 1.75 | 4 | 3 | 4.75 | 3.75 |
| 0.5 | 3.5 | 0.25 | 3.25 | 2.5 | 5.5 | 2.25 | 5.25 |
| 2.5 | 1.5 | 2.25 | 1.25 | 4.5 | 3.5 | 4.25 | 3.25 |
| 3 | 6 | 3.75 | 6.75 | 2 | 5 | 2.75 | 5.75 |
| 5 | 4 | 5.75 | 4.75 | 4 | 3 | 4.75 | 3.75 |
| 3.5 | 6.5 | 3.25 | 6.25 | 2.5 | 5.5 | 2.25 | 5.25 |
| 5.5 | 4.5 | 5.25 | 4.25 | 4.5 | 3.5 | 4.25 | 3.25 |

## Annexe

[Grayscale](https://support.ptc.com/help/mathcad/r10.0/en/index.html#page/PTC_Mathcad_Help/example_grayscale_and_color_in_images.html)

[ditherpunk](https://surma.dev/things/ditherpunk/)

[doc image](https://docs.rs/image/0.24.9/image/index.html)
