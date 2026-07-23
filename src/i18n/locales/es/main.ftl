# Barra lateral
sidebar-menu = Menú
sidebar-home = Inicio
sidebar-passkeys = Llaves
sidebar-config = Configuración
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
passkeys-credentials-count = { $count -> 
    [one] { $count } credencial
    *[other] { $count } credenciales
}

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
