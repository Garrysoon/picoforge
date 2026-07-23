# Seitenleiste
sidebar-menu = Menü
sidebar-home = Startseite
sidebar-passkeys = Schlüssel
sidebar-config = Konfiguration
sidebar-security = Sicherheit
sidebar-about = Über
sidebar-device-status = Gerätestatus
sidebar-refresh = Aktualisieren
sidebar-toggle = Seitenleiste umschalten

# Startseite
home-title = Geräteübersicht
home-subtitle = Schneller Überblick über den Gerätestatus und Informationen.
home-device-name = Gerätename
home-serial-number = Seriennummer
home-firmware-version = Firmware-Version
home-usb-vid-pid = USB VID:PID
home-flash-usage = Flash-Speicher
home-device-status = Gerätestatus
home-connected = Verbunden
home-disconnected = Getrennt

# Schlüssel-Bildschirm
passkeys-title = Schlüssel
passkeys-subtitle = Verwalten Sie Ihre FIDO2-Anmeldeinformationen
passkeys-credential-management = Anmeldeverwaltung
passkeys-no-device = Kein Gerät verbunden. Verbinden Sie einen Pico Key, um Schlüssel zu verwalten.
passkeys-unlock = Entsperren
passkeys-delete = Löschen
passkeys-delete-all = Alle löschen
passkeys-confirm-delete = Löschung bestätigen
passkeys-confirm-delete-message = Möchten Sie diese Anmeldeinformation wirklich löschen?
passkeys-confirm-delete-all-message = Möchten Sie wirklich alle Anmeldeinformationen löschen?
passkeys-yes = Ja
passkeys-no = Nein
passkeys-cancel = Abbrechen
passkeys-rp-id = RP ID
passkeys-user-name = Benutzername
passkeys-credentials-count = { $count -> 
    [one] { $count } Anmeldeinformation
    *[other] { $count } Anmeldeinformationen
}

# Konfigurationsbildschirm
config-title = Konfiguration
config-subtitle = Konfigurieren Sie Ihre Geräteeinstellungen
config-usb-identifiers = USB-Kennungen
config-device-name = Gerätename
config-led = LED
config-firmware = Firmware
config-device-settings = Geräteeinstellungen
config-change-device-name = Gerätename ändern
config-change-device-name-dialog = Neuen Gerätenamen eingeben:
config-change-device-name-input = Gerätename
config-apply = Anwenden
config-cancel = Abbrechen
config-vid = VID
config-pid = PID
config-serial-number = Seriennummer
config-min-pin-length = Min. PIN-Länge
config-touch-timeout = Berührungs-Timeout
config-led-brightness = LED-Helligkeit
config-led-gpio = LED GPIO

# Sicherheitsbildschirm
security-title = Sicherheit & Attestation
security-subtitle = Konfigurieren Sie Sicherheitsfunktionen
security-secure-boot = Secure Boot
security-attestation = Attestation
security-yubico-otp = Yubico OTP
security-secure-boot-status = Secure Boot Status
security-enabled = Aktiviert
security-disabled = Deaktiviert
security-enable = Aktivieren
security-disable = Deaktivieren

# Über-Bildschirm
about-title = Über
about-contributors = Mitwirkende
about-github-repository = GitHub-Repository
about-version = Version
about-license = Lizenz
about-open-source = Open Source

# Allgemein
common-save = Speichern
common-cancel = Abbrechen
common-apply = Anwenden
common-delete = Löschen
common-confirm = Bestätigen
common-yes = Ja
common-no = Nein
common-ok = OK
common-error = Fehler
common-success = Erfolg
common-loading = Laden...
common-no-device = Kein Gerät verbunden
common-connect-device = Verbinden Sie ein Pico Key-Gerät
common-device-connected = Gerät verbunden
common-device-disconnected = Gerät getrennt

# Fehlermeldungen
error-no-device = Kein Gerät verbunden. Bitte verbinden Sie ein Pico Key-Gerät.
error-connection-failed = Verbindung zum Gerät fehlgeschlagen
error-operation-failed = Operation fehlgeschlagen
error-invalid-pin = Ungültiger PIN
error-pin-required = PIN ist erforderlich
error-device-locked = Gerät ist gesperrt

# Statusmeldungen
status-ready = Bereit
status-connecting = Verbinden...
status-connected = Verbunden
status-disconnected = Getrennt
status-syncing = Synchronisieren...
status-sync-complete = Synchronisierung abgeschlossen
