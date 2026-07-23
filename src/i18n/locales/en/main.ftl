# Sidebar navigation
sidebar-menu = Menu
sidebar-home = Home
sidebar-passkeys = Passkeys
sidebar-config = Configuration
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
passkeys-credentials-count = { $count -> 
    [one] { $count } credential
    *[other] { $count } credentials
}

# Configuration screen
config-title = Configuration
config-subtitle = Configure your device settings
config-usb-identifiers = USB Identifiers
config-device-name = Device Name
config-led = LED
config-firmware = Firmware
config-device-settings = Device Settings
config-change-device-name = Change Device Name
config-change-device-name-dialog = Enter new device name:
config-apply = Apply
config-cancel = Cancel
config-vid = VID
config-pid = PID
config-serial-number = Serial Number
config-min-pin-length = Min PIN Length
config-touch-timeout = Touch Timeout
config-led-brightness = LED Brightness
config-led-gpio = LED GPIO

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

# Status messages
status-ready = Ready
status-connecting = Connecting...
status-connected = Connected
status-disconnected = Disconnected
status-syncing = Syncing...
status-sync-complete = Sync complete
