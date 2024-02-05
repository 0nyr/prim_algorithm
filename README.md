# Prim's algorithm for fully connected graphs

The rust program is used to generate test cases for the C program that computes the sum of the cost of the MST of the provided subgraph of a fully connected graph.

### Prim's algoritm for fully connected graph

implémentation de l'algorithme de Prim, pour notre situation. On note que l'on n'a pas besoin de maintenir l'ensemble E des arcs de l'arbre couvrant minimum, puisqu'on ne s'intéresse qu'à la longueur de l'arborescence (MST) construite par l'algorithme. On notera aussi qu'on n'a pas besoin de passer la structure du graphe de base, puisque celui-ci est complet. Il faut néanmoins passer en paramètre de la fonction le nombre de nœuds du graphe de base malgré qu'on soit en train de calculer le MST du sous-graphe des nœuds non visités, afin de pouvoir initialiser la taille des tableaux correctement.

### Note sur la non-utilisation d'une file de priorité.

Considérons l'algorithme d'un point de vue théorique. D'après le cours de Christine Solnon :

Soient `n` le nombre de sommets et `p` le nombre d'arêtes du graphe sur lequel on veut calculer le MST. L’algorithme passe `n − 1` fois dans la boucle `while`. À chaque passage, il faut chercher le sommet de l'ensemble des sommets pas encore visité ayant la plus petite valeur du tableau `c` (`minCostfrom` dans mon implémentation) puis parcourir toutes les arêtes adjacentes à ce sommet. Si ces sommets non visités sont mémorisés dans un tableau ou une liste, la complexité est `O(n2)`. Si on utilise une file de priorité (implémentation en tas binaire), alors la complexité est `O(p.log(n))`. En effet, si l'accès se fait en temps constant, il faut aussi compter la mise à jour du tas binaire à chaque fois que l'on modifie le tableau `c`. Comme il y a au plus `p` mises à jour de `c` (une par arête), la complexité de l'algorithme Prim dans ce cas est bien `O(p log n)`.

Malheureusement, comme le graphe qui nous intéresse est complet, c'est-à-dire que chaque sommet est connecté à tous les autres sommets, alors on a `p = n(n - 1)/2`. Dans ce cas, on aurait `O(n^2.log n)` qui est pire que `O(n^2)`. D'où le fait que l'on n'utilisera pas de file de priorité ici.
