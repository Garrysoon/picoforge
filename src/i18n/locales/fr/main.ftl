# Barre latérale
sidebar-menu = Menu
sidebar-home = Accueil
sidebar-passkeys = Clés
sidebar-oath = OATH
sidebar-otp = OTP
sidebar-config = Configuration
sidebar-vendor = Commandes Vendor
sidebar-firmware = Micrologiciel
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
passkeys-credentials-count = {count} identifiants

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
config-vendor-preset = Préréglage fabricant
config-vendor-id = ID fabricant (HEX)
config-product-id = ID produit (HEX)
config-product-name = Nom du produit
config-led-gpio-pin = Broche LED GPIO
config-led-driver = Pilote LED
config-brightness = Luminosité (0-15)
config-led-dimmable = LED dimmable
config-led-dimmable-desc = Autoriser l'ajustement de la luminosité
config-led-steady = Mode LED constant
config-led-steady-desc = Garder la LED allumée en permanence
config-touch-timeout-label = Délai de toucher (secondes)
config-power-cycle = Redémarrage à la réinitialisation
config-power-cycle-desc = Redémarrer l'appareil à la réinitialisation
config-global-steady = Mode constant global
config-global-steady-desc = Garder les LEDs de statut allumées en permanence
config-save-led = Enregistrer la config LED
config-save-apps = Enregistrer les applications USB
config-apply-changes = Appliquer les modifications

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

# Écran OATH
oath-title = Authentificateur OATH
oath-subtitle = Gérez les identifiants TOTP/HOTP pour l'authentification à deux facteurs.
oath-no-credentials = Aucun identifiant OATH
oath-no-credentials-desc = Ajoutez des identifiants TOTP ou HOTP pour générer des codes d'authentification à deux facteurs.
oath-add = Ajouter un identifiant
oath-account-name = Nom du compte
oath-secret-key = Clé secrète (Base32)
oath-totp = TOTP
oath-hotp = HOTP
oath-digits = { $count } chiffres
oath-credentials-stored = {count} identifiants
oath-import-qr = Importer un QR Code
oath-import-qr-paste = Ou collez l'URI otpauth:// manuellement :
oath-import-qr-camera = Scanner QR Code par caméra
oath-import-qr-camera-hint = Diriger la caméra vers le QR code
oath-import-qr-starting = Démarrage de la caméra...
oath-import-qr-scanning = Pointez la caméra vers le QR Code...
oath-import-qr-found = QR Code détecté !
oath-import-qr-error = Erreur de caméra
oath-import-qr-no-camera = Aucune caméra disponible
oath-import-qr-start-camera = Démarrer la caméra
oath-import-qr-start-camera-hint = Cliquez sur "Démarrer la caméra" pour scanner ou collez l'URI otpauth:// ci-dessous

# Écran OTP
otp-title = Authentificateur OTP
otp-subtitle = Gérez les identifiants TOTP/HOTP pour l'authentification par mot de passe à usage unique.
otp-no-credentials = Aucun identifiant OTP
otp-no-credentials-desc = Ajoutez des identifiants TOTP ou HOTP pour l'authentification par mot de passe à usage unique.
otp-add = Ajouter un identifiant

# Écran Vendor
vendor-title = Commandes Vendor
vendor-subtitle = Opérations Vendor avancées et journal de l'appareil.
vendor-operations = Opérations Vendor
vendor-export-oath = Exporter les identifiants OATH
vendor-export-otp = Exporter les identifiants OTP
vendor-backup = Sauvegarder l'appareil
vendor-restore = Restaurer la sauvegarde
vendor-logs = Journal de l'appareil
vendor-logs-clear = Effacer
vendor-logs-entries = {count} entrées

# Écran firmware
firmware-title = Mise à jour du firmware
firmware-subtitle = Vérifiez et flash les mises à jour du firmware depuis GitHub.
firmware-information = Informations sur le firmware
firmware-current-version = Version actuelle
firmware-latest-version = Dernière version
firmware-check-updates = Vérifier les mises à jour
firmware-flash = Flasher le firmware
firmware-flash-progress = Progression du flash
firmware-checking = Vérification de GitHub pour les mises à jour...
firmware-update-available = Nouvelle version disponible : { $version }
firmware-check-failed = Échec de la vérification : { $error }
firmware-flashing = Flash du firmware en cours...
firmware-flash-success = Firmware flashé avec succès !
firmware-no-version = Inconnu
firmware-not-checked = Non vérifié

# Écran de sécurité (supplémentaire)
security-lock-settings = Paramètres de verrouillage
security-enable-secure-boot = Activer le Secure Boot
security-verify-firmware = Vérifie la signature du firmware au démarrage
security-secure-lock = Verrouillage sécurisé
security-prevent-debug = Empêche la lecture du matériel via les ports de débogage
security-understand-risks = Je comprends les risques de bricolage de mon appareil.
security-permanently-lock = Verrouiller définitivement l'appareil
security-enabling = Activation du Secure Boot...
security-disabling = Désactivation du Secure Boot...
