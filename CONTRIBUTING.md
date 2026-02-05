Contribuer à Waylestia

Nous nous engageons à maintenir une communauté accueillante et inspirante pour tous. Merci de lire et de respecter notre code de conduite :

Soyez respectueux et constructif dans toutes vos interactions.

Accueillez la diversité des points de vue et des expériences.

Concentrez-vous sur ce qui est le mieux pour la communauté.

Faites preuve d’empathie envers les autres membres de la communauté.

Signalez les comportements inappropriés à [azerty.of.game@gmail.com].

Prise en main
Prérequis

Rust 1.70+ (installez depuis https://rustup.rs/
)

Cargo (fourni avec Rust)

Git

Linux (Ubuntu 20.04+ recommandé, ou Fedora 36+)

Connaissances de base en Rust, TypeScript et GTK

Fork et clonage

Forkez le dépôt sur GitHub.

Clonez votre fork :

git clone https://github.com/YOUR_USERNAME/waylestia.git
cd waylestia

Ajoutez la remote upstream :

git remote add upstream https://github.com/aHIPPOz/waylestia.git
Configuration du développement
Compiler depuis les sources
# Installer les dépendances (Ubuntu)
sudo apt-get install build-essential pkg-config libgtk-4-dev libadwaita-1-dev


# Ou Fedora
sudo dnf install gcc pkg-config gtk4-devel libadwaita-devel


# Compiler tous les composants
make all


# Ou individuellement
make core
make widgets
Lancer les services
# Installer dans ~/.local (recommandé pour le développement)
INSTALL_PREFIX=$HOME/.local make install


# Ou utiliser systemd
./scripts/install.sh


# Démarrer
systemctl --user start waylestia-core
systemctl --user start waylestia-widgets


# Voir les logs
journalctl --user -u waylestia-core -f
Flux de travail de développement
# Apportez vos modifications
# Testez localement
make test


# Vérifiez la qualité du code
make lint
make fmt


# Recompiler après modifications
make clean
make core  # ou make widgets


# Redémarrer les services
systemctl --user restart waylestia-core
Effectuer des modifications
Choisir une issue

Recherchez les issues étiquetées good-first-issue ou help-wanted.

Commentez l’issue pour informer les mainteneurs que vous travaillez dessus.

Créez une nouvelle branche pour votre travail.

Créer une branche de fonctionnalité
# Mettre à jour la branche main
git fetch upstream
git checkout main
git merge upstream/main


# Créer la branche de fonctionnalité
git checkout -b feature/your-feature-name
Modifications de code
Pour Rust (core, widgets)

Respectez les conventions de nommage Rust.

Utilisez cargo fmt avant de committer.

Lancez cargo clippy pour le linting.

Ajoutez des commentaires de documentation (///) pour les éléments publics.

Écrivez des tests unitaires pour les nouvelles fonctionnalités.

# Format
cargo fmt


# Lint
cargo clippy -- -D warnings


# Test
cargo test
Pour TypeScript/JavaScript (apps)

Utilisez des types explicites (évitez any sauf si inévitable).

Suivez le guide de style TypeScript de Google.

Ajoutez des commentaires JSDoc pour les fonctions.

Gardez les fonctions et classes simples et à responsabilité unique.

Pour Protocol Buffers

Gardez les définitions de messages simples et ciblées.

Ajoutez des commentaires significatifs aux champs.

Utilisez des enums pour les ensembles de valeurs fixes.

Maintenez la compatibilité ascendante.

Messages de commit

Rédigez des messages de commit clairs et descriptifs :

Résumé court (50 caractères max)


Description plus longue expliquant le changement, pourquoi il a été fait, et tous
les détails importants. Coupez les lignes à 72 caractères. Référencez les issues
liées avec "Fixes #123" ou "Related to #456".


- Les points de liste sont utiles
- Pour les changements complexes
- Expliquez la raison

Exemples :

Add perf monitoring to core daemon


Implement CPU/GPU/RAM monitoring via sysinfo crate.
Exposes metrics via protobuf IPC for shell consumption.


- Adds PerfStats protobuf message
- PerfMonitor struct with update() method
- Integration with IPC server


Fixes #42
Soumettre des modifications
Avant de soumettre

Test de build complet :

make clean
make all
make test

Qualité du code :

make lint
make fmt
cargo clippy

Documentation :

Mettez à jour README.md si le comportement change.

Mettez à jour ARCHITECTURE.md si la structure change.

Ajoutez des commentaires dans le code pour la logique complexe.

Ajoutez/mettez à jour la documentation inline.

Nettoyage des commits :

# Vérifiez que les commits sont logiques et dans le bon ordre
git log origin/main..HEAD


# Écrasez les commits de correction si nécessaire
git rebase -i origin/main
Créer une Pull Request

Poussez vers votre fork :

git push origin feature/your-feature-name

Créez une Pull Request sur GitHub.

Remplissez complètement le template de PR.

Liez les issues concernées.

Décrivez vos changements et la motivation.

Processus de revue de PR

Les mainteneurs examineront votre PR.

Répondez aux retours ou demandes de modifications.

Gardez les commits logiques pendant la revue.

Marquez les conversations comme résolues une fois traitées.

Au minimum une approbation est requise pour merger.

Directives de style
Code Rust
// Utilisez des noms de variables significatifs
let widget_count = widgets.len();


// Utilisez une organisation par modules
mod widget;
use widget::{Widget, WidgetState};


// Documentez les APIs publiques
/// Crée une nouvelle instance de widget avec l'ID donné.
/// 
/// # Arguments
/// * `widget_id` - L'identifiant unique du widget
/// 
/// # Returns
/// Une nouvelle WidgetInstance avec l'état par défaut
pub fn create_instance(widget_id: String) -> WidgetInstance {
    // ...
}


// Utilisez Result pour les opérations susceptibles d'échouer
pub fn load_manifest(path: &Path) -> Result<WidgetManifest> {
    // ...
}
TypeScript/JavaScript
// Utilisez des types explicites
function updateWidget(id: string, state: WidgetState): void {
    // ...
}


// Utilisez const par défaut
const WIDGET_TIMEOUT = 5000;


// Documentez avec JSDoc
/**
 * Initialise l'application de widgets
 * @param elementId - ID de l'élément DOM où monter
 * @returns Promise qui se résout quand l'application est prête
 */
async function initialize(elementId: string): Promise<void> {
    // ...
}
Protocol Buffers
// Groupez les messages liés
message PerformanceMetrics {
  float cpu_usage = 1;      // pourcentage 0-100
  float gpu_usage = 2;      // pourcentage 0-100
  float ram_usage = 3;      // pourcentage 0-100
  
  // Réserver des champs pour des extensions futures
  reserved 4, 5;
}
Structure du projet
waylestia/
├── core/              # Daemon Rust - directives PR :
│                      # - Respecter les conventions Rust
│                      # - Ajouter des tests pour les nouveaux modules
│                      # - Mettre à jour state.rs pour les nouvelles données
│
├── widgets/           # Moteur de widgets - directives PR :
│                      # - Maintenir la compatibilité du chargeur (loader)
│                      # - Tester les cas limites du parsing des manifests
│
├── apps/              # Applications GJS - directives PR :
│                      # - Utiliser correctement les widgets GTK
│                      # - Implémenter correctement les patterns IPC
│
├── protobuf/          # Définitions de protocoles - directives PR :
│                      # - Ne pas changer les IDs de messages existants
│                      # - Ajouter des commentaires pour les nouveaux messages
│                      # - Penser à la compatibilité future
│
├── assets/            # Ressources - directives PR :
│                      # - Optimiser la taille des images
│                      # - Garder les manifests valides
│
└── scripts/           # Outils de build/install - directives PR :
                       # - Tester sur une machine propre
                       # - Supporter INSTALL_PREFIX
Commandes utiles
# Voir l'historique d'un fichier
git log -p -- path/to/file.rs


# Trouver les contributeurs
git shortlog -sn


# Rebaser sur la dernière version
git fetch upstream
git rebase upstream/main


# Rebase interactif (éditer les commits)
git rebase -i HEAD~3


# Écraser des commits
git rebase -i master  # marquez 'squash' sur les commits à fusionner
Obtenir de l'aide

Questions : Ouvrez une issue avec le label question.

Discussion : Utilisez GitHub Discussions.

Bugs : Ouvrez une issue avec des étapes de reproduction.

Chat : Rejoignez notre chat communautaire [https://discord.gg/XHtnVYq9j9].

Reconnaissance

Les contributeurs seront reconnus dans :

le fichier CONTRIBUTORS.md

les notes de publication (release notes)

la page des contributeurs GitHub

Licence

En contribuant, vous acceptez que vos contributions soient licenciées sous la même licence que le projet (GPL-3.0).

Merci de contribuer à Waylestia ! 🎉
