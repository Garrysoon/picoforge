# Barra lateral
sidebar-menu = Menú
sidebar-home = Inicio
sidebar-passkeys = Llaves
sidebar-oath = OATH
sidebar-otp = OTP
sidebar-config = Configuración
sidebar-vendor = Comandos Vendor
sidebar-firmware = Firmware
sidebar-security = Seguridad
sidebar-about = Acerca de
sidebar-device-status = Estado del dispositivo
sidebar-refresh = Actualizar
sidebar-toggle = Mostrar/Ocultar barra lateral

# Pantalla de inicio
home-title = Resumen del dispositivo
home-subtitle = Vista rápida del estado e información del dispositivo.
home-device-name = Nombre del dispositivo
home-serial-number = Número de serie
home-firmware-version = Versión del firmware
home-usb-vid-pid = USB VID:PID
home-flash-usage = Uso de memoria flash
home-device-status = Estado del dispositivo
home-connected = Conectado
home-disconnected = Desconectado

# Pantalla de llaves
passkeys-title = Llaves
passkeys-subtitle = Administre sus credenciales FIDO2
passkeys-credential-management = Gestión de credenciales
passkeys-no-device = No hay dispositivo conectado. Conecte un Pico Key para administrar las llaves.
passkeys-unlock = Desbloquear
passkeys-delete = Eliminar
passkeys-delete-all = Eliminar todo
passkeys-confirm-delete = Confirmar eliminación
passkeys-confirm-delete-message = ¿Está seguro de que desea eliminar esta credencial?
passkeys-confirm-delete-all-message = ¿Está seguro de que desea eliminar todas las credenciales?
passkeys-yes = Sí
passkeys-no = No
passkeys-cancel = Cancelar
passkeys-rp-id = RP ID
passkeys-user-name = Nombre de usuario
passkeys-credentials-count = {count} credenciales

# Pantalla de configuración
config-title = Configuración
config-subtitle = Configure los ajustes de su dispositivo
config-usb-identifiers = Identificadores USB
config-device-name = Nombre del dispositivo
config-led = LED
config-firmware = Firmware
config-device-settings = Ajustes del dispositivo
config-change-device-name = Cambiar nombre del dispositivo
config-change-device-name-dialog = Ingrese el nuevo nombre del dispositivo:
config-change-device-name-input = Nombre del dispositivo
config-apply = Aplicar
config-cancel = Cancelar
config-vid = VID
config-pid = PID
config-serial-number = Número de serie
config-min-pin-length = Longitud mínima del PIN
config-touch-timeout = Tiempo de espera al tocar
config-led-brightness = Brillo del LED
config-led-gpio = LED GPIO
config-vendor-preset = Predefinido del fabricante
config-vendor-id = ID del fabricante (HEX)
config-product-id = ID del producto (HEX)
config-product-name = Nombre del producto
config-led-gpio-pin = Pin GPIO del LED
config-led-driver = Controlador LED
config-brightness = Brillo (0-15)
config-led-dimmable = LED atenuable
config-led-dimmable-desc = Permitir ajuste de brillo
config-led-steady = Modo LED constante
config-led-steady-desc = Mantener LED encendido constantemente
config-touch-timeout-label = Tiempo de espera al tocar (segundos)
config-power-cycle = Reinicio al restablecer
config-power-cycle-desc = Reiniciar dispositivo al restablecer
config-global-steady = Modo constante global
config-global-steady-desc = Mantener LEDs de estado encendidos constantemente
config-save-guardar-led = Guardar config LED
config-save-apps = Guardar aplicaciones USB
config-apply-changes = Aplicar cambios

# Pantalla de seguridad
security-title = Seguridad y Attestación
security-subtitle = Configure las funciones de seguridad
security-secure-boot = Secure Boot
security-attestation = Attestación
security-yubico-otp = Yubico OTP
security-secure-boot-status = Estado del Secure Boot
security-enabled = Habilitado
security-disabled = Deshabilitado
security-enable = Habilitar
security-disable = Deshabilitar

# Pantalla Acerca de
about-title = Acerca de
about-contributors = Contribuidores
about-github-repository = Repositorio GitHub
about-version = Versión
about-license = Licencia
about-open-source = Código abierto

# Común
common-save = Guardar
common-cancel = Cancelar
common-apply = Aplicar
common-delete = Eliminar
common-confirm = Confirmar
common-yes = Sí
common-no = No
common-ok = Aceptar
common-error = Error
common-success = Éxito
common-loading = Cargando...
common-no-device = No hay dispositivo conectado
common-connect-device = Conecte un dispositivo Pico Key
common-device-connected = Dispositivo conectado
common-device-disconnected = Dispositivo desconectado

# Mensajes de error
error-no-device = No hay dispositivo conectado. Por favor, conecte un dispositivo Pico Key.
error-connection-failed = Error al conectar con el dispositivo
error-operation-failed = La operación falló
error-invalid-pin = PIN inválido
error-pin-required = Se requiere PIN
error-device-locked = El dispositivo está bloqueado

# Mensajes de estado
status-ready = Listo
status-connecting = Conectando...
status-connected = Conectado
status-disconnected = Desconectado
status-syncing = Sincronizando...
status-sync-complete = Sincronización completada

# Pantalla OATH
oath-title = Autenticador OATH
oath-subtitle = Administre credenciales TOTP/HOTP para autenticación de dos factores.
oath-no-credentials = Sin credenciales OATH
oath-no-credentials-desc = Agregue credenciales TOTP u HOTP para generar códigos de autenticación de dos factores.
oath-add = Agregar credencial
oath-account-name = Nombre de cuenta
oath-secret-key = Clave secreta (Base32)
oath-totp = TOTP
oath-hotp = HOTP
oath-digits = { $count } dígitos
oath-credentials-stored = {count} credenciales
oath-import-qr = Importar código QR
oath-import-qr-paste = O pegue la URI otpauth:// manualmente:
oath-import-qr-camera = Escáner de código QR con cámara
oath-import-qr-camera-hint = Apunte la cámara al código QR
oath-import-qr-starting = Iniciando cámara...
oath-import-qr-scanning = Apunte la cámara al código QR...
oath-import-qr-found = ¡Código QR detectado!
oath-import-qr-error = Error de cámara
oath-import-qr-no-camera = Cámara no disponible
oath-import-qr-start-camera = Iniciar cámara
oath-import-qr-start-camera-hint = Haga clic en "Iniciar cámara" para escanear o pegue la URI otpauth:// a continuación

# Pantalla OTP
otp-title = Autenticador OTP
otp-subtitle = Administre credenciales TOTP/HOTP para autenticación con contraseña de un solo uso.
otp-no-credentials = Sin credenciales OTP
otp-no-credentials-desc = Agregue credenciales TOTP u HOTP para autenticación con contraseña de un solo uso.
otp-add = Agregar credencial

# Pantalla Vendor
vendor-title = Comandos Vendor
vendor-subtitle = Operaciones Vendor avanzadas y registro del dispositivo.
vendor-operations = Operaciones Vendor
vendor-export-oath = Exportar credenciales OATH
vendor-export-otp = Exportar credenciales OTP
vendor-backup = Respaldar dispositivo
vendor-restore = Restaurar respaldo
vendor-logs = Registro del dispositivo
vendor-logs-clear = Limpiar
vendor-logs-entries = {count} entradas

# Pantalla firmware
firmware-title = Actualización de firmware
firmware-subtitle = Verifique y flashee actualizaciones de firmware desde GitHub.
firmware-information = Información del firmware
firmware-current-version = Versión actual
firmware-latest-version = Última versión
firmware-check-updates = Buscar actualizaciones
firmware-flash = Flashear firmware
firmware-flash-progress = Progreso del flash
firmware-checking = Verificando GitHub para actualizaciones...
firmware-update-available = Nueva versión disponible: { $version }
firmware-check-failed = Error de verificación: { $error }
firmware-flashing = Flasheando firmware...
firmware-flash-success = ¡Firmware flasheado exitosamente!
firmware-no-version = Desconocido
firmware-not-checked = No verificado

# Pantalla de seguridad (adicional)
security-lock-settings = Configuración de bloqueo
security-enable-secure-boot = Habilitar Secure Boot
security-verify-firmware = Verifica la firma del firmware al iniciar
security-secure-lock = Bloqueo seguro
security-prevent-debug = Impide la lectura de material de claves a través de puertos de depuración
security-understand-risks = Entiendo los riesgos de dañar mi dispositivo.
security-permanently-lock = Bloquear dispositivo permanentemente
security-enabling = Habilitando Secure Boot...
security-disabling = Deshabilitando Secure Boot...
