# Sidebar navigation
sidebar-menu = Menu
sidebar-home = Home
sidebar-passkeys = Passkeys
sidebar-oath = OATH
sidebar-otp = OTP
sidebar-config = Configuration
sidebar-vendor = Vendor Commands
sidebar-firmware = Firmware
sidebar-security = Security
sidebar-about = About
sidebar-device-status = Device Status
sidebar-refresh = Refresh
sidebar-toggle = Toggle sidebar

# Home screen
home-title = Device Overview
home-subtitle = Quick view of your device status and information.
home-device-name = Device Name
home-serial-number = Serial Number
home-firmware-version = Firmware Version
home-usb-vid-pid = USB VID:PID
home-flash-usage = Flash Usage
home-device-status = Device Status
home-connected = Connected
home-disconnected = Disconnected

# Passkeys screen
passkeys-title = Passkeys
passkeys-subtitle = Manage your FIDO2 credentials
passkeys-credential-management = Credential Management
passkeys-no-device = No device connected. Connect a Pico Key to manage passkeys.
passkeys-unlock = Unlock
passkeys-delete = Delete
passkeys-delete-all = Delete All
passkeys-confirm-delete = Confirm Deletion
passkeys-confirm-delete-message = Are you sure you want to delete this credential?
passkeys-confirm-delete-all-message = Are you sure you want to delete all credentials?
passkeys-yes = Yes
passkeys-no = No
passkeys-cancel = Cancel
passkeys-rp-id = RP ID
passkeys-user-name = User Name
passkeys-credentials-count = {count} credentials

# Configuration screen
config-title = Configuration
config-subtitle = Configure your device settings
config-usb-identifiers = USB Identifiers
config-device-name = Device Name
config-led = LED
config-firmware = Firmware
config-device-settings = Device Settings
config-change-device-name = Change device name
config-change-device-name-dialog = Enter new device name:
config-change-device-name-input = Device name
config-apply = Apply
config-cancel = Cancel
config-vid = VID
config-pid = PID
config-serial-number = Serial Number
config-min-pin-length = Min PIN length
config-touch-timeout = Touch Timeout
config-led-brightness = LED Brightness
config-led-gpio = LED GPIO
config-vendor-preset = Vendor Preset
config-vendor-id = Vendor ID (HEX)
config-product-id = Product ID (HEX)
config-product-name = Product Name
config-led-gpio-pin = LED GPIO Pin
config-led-driver = LED Driver
config-brightness = Brightness (0-15)
config-led-dimmable = LED Dimmable
config-led-dimmable-desc = Allow brightness adjustment
config-led-steady = LED Steady Mode
config-led-steady-desc = Keep LED on constantly
config-touch-timeout-label = Touch Timeout (seconds)
config-power-cycle = Power Cycle on Reset
config-power-cycle-desc = Restart device on reset
config-global-steady = Global Steady Mode
config-global-steady-desc = Keep status LEDs on constantly
config-save-led = Save LED Status
config-save-apps = Save USB Applications
config-apply-changes = Apply Changes

# Security screen
security-title = Security & Attestation
security-subtitle = Configure security features
security-secure-boot = Secure Boot
security-attestation = Attestation
security-yubico-otp = Yubico OTP
security-secure-boot-status = Secure Boot Status
security-enabled = Enabled
security-disabled = Disabled
security-enable = Enable
security-disable = Disable

# About screen
about-title = About
about-contributors = Contributors
about-github-repository = GitHub Repository
about-version = Version
about-license = License
about-open-source = Open Source

# Common
common-save = Save
common-cancel = Cancel
common-apply = Apply
common-delete = Delete
common-confirm = Confirm
common-yes = Yes
common-no = No
common-ok = OK
common-error = Error
common-success = Success
common-loading = Loading...
common-no-device = No device connected
common-connect-device = Connect a Pico Key device
common-device-connected = Device connected
common-device-disconnected = Device disconnected

# Error messages
error-no-device = No device connected. Please connect a Pico Key device.
error-connection-failed = Failed to connect to device
error-operation-failed = Operation failed
error-invalid-pin = Invalid PIN
error-pin-required = PIN is required
error-device-locked = Device is locked

# OATH screen
oath-title = OATH Authenticator
oath-subtitle = Manage TOTP/HOTP credentials for two-factor authentication.
oath-no-credentials = No OATH Credentials
oath-no-credentials-desc = Add TOTP or HOTP credentials to generate two-factor authentication codes.
oath-add = Add Credential
oath-account-name = Account Name
oath-secret-key = Secret Key (Base32)
oath-totp = TOTP
oath-hotp = HOTP
oath-digits = { $count } digits
oath-credentials-stored = {count} credentials stored
oath-import-qr = Import QR Code
oath-import-qr-paste = Or paste otpauth:// URI manually:
oath-import-qr-camera = Camera QR Scanner
oath-import-qr-starting = Starting camera...
oath-import-qr-scanning = Point camera at QR code...
oath-import-qr-found = QR Code Detected!
oath-import-qr-error = Camera Error
oath-import-qr-no-camera = No camera available
oath-import-qr-start-camera = Start Camera
oath-import-qr-start-camera-hint = Click "Start Camera" to scan QR code, or paste otpauth:// URI below

# OTP screen
otp-title = OTP Authenticator
otp-subtitle = Manage TOTP/HOTP credentials for one-time password authentication.
otp-no-credentials = No OTP Credentials
otp-no-credentials-desc = Add TOTP or HOTP credentials for one-time password authentication.
otp-add = Add Credential

# Vendor screen
vendor-title = Vendor Commands
vendor-subtitle = Advanced vendor-specific operations and device logs.
vendor-operations = Vendor Operations
vendor-export-oath = Export OATH Credentials
vendor-export-otp = Export OTP Credentials
vendor-backup = Backup Device
vendor-restore = Restore Backup
vendor-logs = Device Logs
vendor-logs-clear = Clear
vendor-logs-entries = {count} entries

# Firmware screen
firmware-title = Firmware Update
firmware-subtitle = Check for and flash firmware updates from GitHub.
firmware-information = Firmware Information
firmware-current-version = Current Version
firmware-latest-version = Latest Version
firmware-check-updates = Check for Updates
firmware-flash = Flash Firmware
firmware-flash-progress = Flash Progress
firmware-checking = Checking GitHub for updates...
firmware-update-available = New version available: { $version }
firmware-check-failed = Check failed: { $error }
firmware-flashing = Flashing firmware...
firmware-flash-success = Firmware flashed successfully!
firmware-no-version = Unknown
firmware-not-checked = Not checked

# Security screen (additional)
security-lock-settings = Lock Settings
security-enable-secure-boot = Enable Secure Boot
security-verify-firmware = Verifies firmware signature on startup
security-secure-lock = Secure Lock
security-prevent-debug = Prevents reading key material via debug ports
security-understand-risks = I understand the risks of bricking my device.
security-permanently-lock = Permanently Lock Device
security-enabling = Enabling secure boot...
security-disabling = Disabling secure boot...

# Status messages
status-ready = Ready
status-connecting = Connecting...
status-connected = Connected
status-disconnected = Disconnected
status-syncing = Syncing...
status-sync-complete = Sync complete
