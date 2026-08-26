flappy_rust/
├── Cargo.toml
├── assets/                  # Images PNG, polices, sons
│   ├── bird.png
│   └── pipe.png
└── src/
    ├── main.rs              # Point d'entrée + assemblage des plugins
    ├── bird.rs              # Plugin : logique, saut et physique de l'oiseau
    ├── pipes.rs             # Plugin : génération et déplacement des tuyaux
    ├── score.rs             # Plugin : gestion de la Resource Score + UI
    └── collision.rs         # Plugin/System : détection des impacts


Plan de route:
Étape 1 : Le bac à sable (Setup & Caméra)
   │
Étape 2 : Faire apparaître l'oiseau (Components & Bundles)
   │
Étape 3 : La physique & Contrôles (Systems & Inputs)
   │
Étape 4 : Le générateur de tuyaux (Resources & Timers)
   │
Étape 5 : Les collisions & Game Over (Queries avancées)
   │
Étape 6 : L'interface et le Score (UI Bevy)


## Details:
Étape 1 : Le Bac à Sable (Ce qu'on prépare)

Avant d'écrire de la vraie logique, ton but est d'ouvrir une fenêtre vide avec une caméra 2D.

    Ce que tu apprends : Comprendre le rôle de DefaultPlugins et de Commands.

    Ton objectif : Écrire un main.rs qui initialise l'application et ajoute un Camera2dBundle.

Étape 2 : Faire apparaître l'Oiseau

Ne cherche pas d'image PNG pour l'instant. Utilise une forme simple (un rectangle de couleur).

    Ce que tu apprends : Créer des struct personnalisées avec #[derive(Component)] et utiliser SpriteBundle.

    Questions à te poser pour le coder :

        De quelle donnée mon oiseau a-t-il besoin ? (Un composant marqueur struct Bird;, une vitesse struct Velocity(f32); ?)

        Comment utiliser commands.spawn(...) dans un système de Startup pour afficher un sprite rectangle couleur jaune ?

Étape 3 : La Physique et les Contrôles

Une fois le carré jaune à l'écran, tu vas lui donner de la vie.

    Ce que tu apprends : Utiliser Query pour modifier des composants (&mut Transform, &mut Velocity), utiliser Res<Time> et capter le clavier avec Res<ButtonInput<KeyCode>>.

    À faire en 2 sous-systèmes dans Update :

        apply_gravity : À chaque frame, tu réduis la vitesse verticale avec la gravité et tu ajoutes cette vitesse à la position transform.translation.y.

        bird_jump : Si la touche Espace est pressée, tu réinitialises la vitesse vers le haut (ex: +300.0).

Étape 4 : Les Tuyaux et le Temps

Maintenant qu'on a un oiseau qui saute, il faut du décor qui défile.

    Ce que tu apprends : Créer une Resource personnalisée contenant un Timer pour exécuter une action toutes les X secondes.

    Le concept :

        Tu crées une struct PipeTimer(Timer); déclarée comme #[derive(Resource)].

        Dans un système, tu fais avancer le timer avec timer.tick(time.delta()).

        Quand le timer expire (timer.just_finished()), tu spawn deux rectangles verts (tuyau haut et tuyau bas) à droite de l'écran avec une position Y aléatoire.

        Un autre système fait tout simplement bouger tous les tuyaux vers la gauche à chaque frame (transform.translation.x -= speed * dt).

Étape 5 : La Détection de Collisions

C'est ici que tu appliques tes connaissances sur les Query.

    Ce que tu apprends : Interroger deux types de composants différents dans le même système (Query sur l'oiseau et Query sur les tuyaux).

    Le concept : Tu récupères la position/taille de l'oiseau et tu la compares avec la position/taille de chaque tuyau (collision de rectangles / AABB). Si ça touche : tu affiches "Game Over" dans la console ou tu réinitialises la position de l'oiseau.