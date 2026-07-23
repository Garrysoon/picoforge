# Seitenleiste
sidebar-menu = Menü
sidebar-home = Startseite
sidebar-passkeys = Schlüssel
sidebar-oath = OATH
sidebar-otp = OTP
sidebar-config = Konfiguration
sidebar-vendor = Vendor-Befehle
sidebar-firmware = Firmware
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
passkeys-credentials-count = {count} Anmeldeinformationen

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
config-vendor-preset = Hersteller-Voreinstellung
config-vendor-id = Hersteller-ID (HEX)
config-product-id = Produkt-ID (HEX)
config-product-name = Produktname
config-led-gpio-pin = LED GPIO Pin
config-led-driver = LED-Treiber
config-brightness = Helligkeit (0-15)
config-led-dimmable = LED dimmbar
config-led-dimmable-desc = Helligkeitsanpassung erlauben
config-led-steady = LED-Konstantmodus
config-led-steady-desc = LED dauerhaft einschalten
config-touch-timeout-label = Berührungs-Timeout (Sekunden)
config-power-cycle = Stromzyklus bei Reset
config-power-cycle-desc = Gerät bei Reset neu starten
config-global-steady = Globaler Konstantmodus
config-global-steady-desc = Status-LEDs dauerhaft einschalten
config-save-led = LED-Einstellungen speichern
config-save-apps = USB-Anwendungen speichern
config-apply-changes = Änderungen anwenden

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

# OATH-Bildschirm
oath-title = OATH-Authenticator
oath-subtitle = Verwalten Sie TOTP/HOTP-Anmeldeinformationen für die Zwei-Faktor-Authentifizierung.
oath-no-credentials = Keine OATH-Anmeldeinformationen
oath-no-credentials-desc = Fügen Sie TOTP- oder HOTP-Anmeldeinformationen hinzu, um Zwei-Faktor-Authentifizierungscodes zu generieren.
oath-add = Anmeldeinformation hinzufügen
oath-account-name = Kontoname
oath-secret-key = Geheimer Schlüssel (Base32)
oath-totp = TOTP
oath-hotp = HOTP
oath-digits = { $count } Ziffern
oath-credentials-stored = {count} Anmeldeinformationen
oath-import-qr = QR-Code importieren
oath-import-qr-paste = Oder otpauth:// URI manuell einfügen:
oath-import-qr-camera = Kamera QR-Scanner
oath-import-qr-camera-hint = Kamera auf QR-Code richten
oath-import-qr-starting = Kamera wird gestartet...
oath-import-qr-scanning = Kamera auf QR-Code richten...
oath-import-qr-found = QR-Code erkannt!
oath-import-qr-error = Kamerafehler
oath-import-qr-no-camera = Keine Kamera verfügbar
oath-import-qr-start-camera = Kamera starten
oath-import-qr-start-camera-hint = Klicken Sie auf "Kamera starten" zum Scannen oder fügen Sie otpauth:// URI unten ein

# OTP-Bildschirm
otp-title = OTP-Authenticator
otp-subtitle = Verwalten Sie TOTP/HOTP-Anmeldeinformationen für Einmalpasswort-Authentifizierung.
otp-no-credentials = Keine OTP-Anmeldeinformationen
otp-no-credentials-desc = Fügen Sie TOTP- oder HOTP-Anmeldeinformationen für die Einmalpasswort-Authentifizierung hinzu.
otp-add = Anmeldeinformation hinzufügen

# Vendor-Bildschirm
vendor-title = Vendor-Befehle
vendor-subtitle = Erweiterte Vendor-spezifische Operationen und Geräteprotokolle.
vendor-operations = Vendor-Operationen
vendor-export-oath = OATH-Anmeldeinformationen exportieren
vendor-export-otp = OTP-Anmeldeinformationen exportieren
vendor-backup = Gerät sichern
vendor-restore = Sicherung wiederherstellen
vendor-logs = Geräteprotokolle
vendor-logs-clear = Löschen
vendor-logs-entries = {count} Einträge

# Firmware-Bildschirm
firmware-title = Firmware-Update
firmware-subtitle = Prüfen und flashen Sie Firmware-Updates von GitHub.
firmware-information = Firmware-Informationen
firmware-current-version = Aktuelle Version
firmware-latest-version = Neueste Version
firmware-check-updates = Nach Updates suchen
firmware-flash = Firmware flashen
firmware-flash-progress = Flash-Fortschritt
firmware-checking = GitHub auf Updates prüfen...
firmware-update-available = Neue Version verfügbar: { $version }
firmware-check-failed = Prüfung fehlgeschlagen: { $error }
firmware-flashing = Firmware wird geflasht...
firmware-flash-success = Firmware erfolgreich geflasht!
firmware-no-version = Unbekannt
firmware-not-checked = Nicht geprüft

# Sicherheitsbildschirm (zusätzlich)
security-lock-settings = Sperr-Einstellungen
security-enable-secure-boot = Secure Boot aktivieren
security-verify-firmware = Überprüft die Firmware-Signatur beim Start
security-secure-lock = Sichere Sperre
security-prevent-debug = Verhindert das Lesen von Schlüsselmaterial über Debug-Ports
security-understand-risks = Ich verstehe die Risiken einer Beschädigung meines Geräts.
security-permanently-lock = Gerät dauerhaft sperren
security-enabling = Secure Boot wird aktiviert...
security-disabling = Secure Boot wird deaktiviert...
