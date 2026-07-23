# Barre latérale
sidebar-menu = Menu
sidebar-home = Accueil
sidebar-passkeys = Clés
sidebar-config = Configuration
sidebar-security = Sécurité
sidebar-about = À propos
sidebar-device-status = État de l'appareil
sidebar-refresh = Actualiser
sidebar-toggle = Masquer/Afficher la barre latérale

# Écran d'accueil
home-title = Aperçu de l'appareil
home-subtitle = Aperçu rapide de l'état et des informations de l'appareil.
home-device-name = Nom de l'appareil
home-serial-number = Numéro de série
home-firmware-version = Version du firmware
home-usb-vid-pid = USB VID:PID
home-flash-usage = Utilisation de la mémoire flash
home-device-status = État de l'appareil
home-connected = Connecté
home-disconnected = Déconnecté

# Écran des clés
passkeys-title = Clés
passkeys-subtitle = Gérez vos identifiants FIDO2
passkeys-credential-management = Gestion des identifiants
passkeys-no-device = Aucun appareil connecté. Connectez un Pico Key pour gérer les clés.
passkeys-unlock = Déverrouiller
passkeys-delete = Supprimer
passkeys-delete-all = Tout supprimer
passkeys-confirm-delete = Confirmer la suppression
passkeys-confirm-delete-message = Êtes-vous sûr de vouloir supprimer cet identifiant ?
passkeys-confirm-delete-all-message = Êtes-vous sûr de vouloir supprimer tous les identifiants ?
passkeys-yes = Oui
passkeys-no = Non
passkeys-cancel = Annuler
passkeys-rp-id = RP ID
passkeys-user-name = Nom d'utilisateur
passkeys-credentials-count = { $count -> 
    [one] { $count } identifiant
    *[other] { $count } identifiants
}

# Écran de configuration
config-title = Configuration
config-subtitle = Configurez les paramètres de votre appareil
config-usb-identifiers = Identifiants USB
config-device-name = Nom de l'appareil
config-led = LED
config-firmware = Firmware
config-device-settings = Paramètres de l'appareil
config-change-device-name = Changer le nom de l'appareil
config-change-device-name-dialog = Entrez le nouveau nom de l'appareil :
config-change-device-name-input = Nom de l'appareil
config-apply = Appliquer
config-cancel = Annuler
config-vid = VID
config-pid = PID
config-serial-number = Numéro de série
config-min-pin-length = Longueur minimale du PIN
config-touch-timeout = Délai de toucher
config-led-brightness = Luminosité LED
config-led-gpio = LED GPIO

# Écran de sécurité
security-title = Sécurité et Attestation
security-subtitle = Configurez les fonctionnalités de sécurité
security-secure-boot = Secure Boot
security-attestation = Attestation
security-yubico-otp = Yubico OTP
security-secure-boot-status = État du Secure Boot
security-enabled = Activé
security-disabled = Désactivé
security-enable = Activer
security-disable = Désactiver

# Écran À propos
about-title = À propos
about-contributors = Contributeurs
about-github-repository = Dépôt GitHub
about-version = Version
about-license = Licence
about-open-source = Open Source

# Général
common-save = Enregistrer
common-cancel = Annuler
common-apply = Appliquer
common-delete = Supprimer
common-confirm = Confirmer
common-yes = Oui
common-no = Non
common-ok = OK
common-error = Erreur
common-success = Succès
common-loading = Chargement...
common-no-device = Aucun appareil connecté
common-connect-device = Connectez un appareil Pico Key
common-device-connected = Appareil connecté
common-device-disconnected = Appareil déconnecté

# Messages d'erreur
error-no-device = Aucun appareil connecté. Veuillez connecter un appareil Pico Key.
error-connection-failed = Échec de la connexion à l'appareil
error-operation-failed = L'opération a échoué
error-invalid-pin = PIN invalide
error-pin-required = Le PIN est requis
error-device-locked = L'appareil est verrouillé

# Messages de statut
status-ready = Prêt
status-connecting = Connexion...
status-connected = Connecté
status-disconnected = Déconnecté
status-syncing = Synchronisation...
status-sync-complete = Synchronisation terminée
