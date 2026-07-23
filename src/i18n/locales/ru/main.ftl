# Боковое меню
sidebar-menu = Меню
sidebar-home = Главная
sidebar-passkeys = Ключи
sidebar-oath = OATH
sidebar-otp = OTP
sidebar-config = Конфигурация
sidebar-vendor = Команды Vendor
sidebar-firmware = Прошивка
sidebar-security = Безопасность
sidebar-about = О программе
sidebar-device-status = Статус устройства
sidebar-refresh = Обновить
sidebar-toggle = Скрыть/показать меню

# Главный экран
home-title = Обзор устройства
home-subtitle = Краткий просмотр состояния и информации об устройстве.
home-device-name = Имя устройства
home-serial-number = Серийный номер
home-firmware-version = Версия прошивки
home-usb-vid-pid = USB VID:PID
home-flash-usage = Использование флеш-памяти
home-device-status = Состояние устройства
home-connected = Подключено
home-disconnected = Отключено

# Экран ключей
passkeys-title = Ключи
passkeys-subtitle = Управление вашими FIDO2 учетными записями
passkeys-credential-management = Управление учетными записями
passkeys-no-device = Устройство не подключено. Подключите Pico Key для управления ключами.
passkeys-unlock = Разблокировать
passkeys-delete = Удалить
passkeys-delete-all = Удалить все
passkeys-confirm-delete = Подтверждение удаления
passkeys-confirm-delete-message = Вы уверены, что хотите удалить эту учетную запись?
passkeys-confirm-delete-all-message = Вы уверены, что хотите удалить все учетные записи?
passkeys-yes = Да
passkeys-no = Нет
passkeys-cancel = Отмена
passkeys-rp-id = RP ID
passkeys-user-name = Имя пользователя
passkeys-credentials-count = {count} учетных записей

# Экран конфигурации
config-title = Конфигурация
config-subtitle = Настройте параметры устройства
config-usb-identifiers = USB идентификаторы
config-device-name = Имя устройства
config-led = LED
config-firmware = Прошивка
config-device-settings = Настройки устройства
config-change-device-name = Изменить имя устройства
config-change-device-name-dialog = Введите новое имя устройства:
config-change-device-name-input = Имя устройства
config-apply = Применить
config-cancel = Отмена
config-vid = VID
config-pid = PID
config-serial-number = Серийный номер
config-min-pin-length = Мин. длина PIN
config-touch-timeout = Тайм-аут касания
config-led-brightness = Яркость LED
config-led-gpio = LED GPIO
config-vendor-preset = Пресет производителя
config-vendor-id = ID производителя (HEX)
config-product-id = ID продукта (HEX)
config-product-name = Название продукта
config-led-gpio-pin = LED GPIO пин
config-led-driver = Драйвер LED
config-brightness = Яркость (0-15)
config-led-dimmable = LED диммируемая
config-led-dimmable-desc = Разрешить регулировку яркости
config-led-steady = Режим постоянного LED
config-led-steady-desc = Держать LED постоянно включенным
config-touch-timeout-label = Тайм-аут касания (секунды)
config-power-cycle = Перезапуск при сбросе
config-power-cycle-desc = Перезагрузить устройство при сбросе
config-global-steady = Глобальный постоянный режим
config-global-steady-desc = Держать статусные LED постоянно включенными
config-save-led = Сохранить настройки LED
config-save-apps = Сохранить USB приложения
config-apply-changes = Применить изменения

# Экран безопасности
security-title = Безопасность и аттестация
security-subtitle = Настройте функции безопасности
security-secure-boot = Безопасная загрузка
security-attestation = Аттестация
security-yubico-otp = Yubico OTP
security-secure-boot-status = Статус безопасной загрузки
security-enabled = Включено
security-disabled = Выключено
security-enable = Включить
security-disable = Выключить

# Экран "О программе"
about-title = О программе
about-contributors = Участники
about-github-repository = Репозиторий GitHub
about-version = Версия
about-license = Лицензия
about-open-source = Открытый исходный код

# Общие
common-save = Сохранить
common-cancel = Отмена
common-apply = Применить
common-delete = Удалить
common-confirm = Подтвердить
common-yes = Да
common-no = Нет
common-ok = ОК
common-error = Ошибка
common-success = Успешно
common-loading = Загрузка...
common-no-device = Устройство не подключено
common-connect-device = Подключите устройство Pico Key
common-device-connected = Устройство подключено
common-device-disconnected = Устройство отключено

# Сообщения об ошибках
error-no-device = Устройство не подключено. Пожалуйста, подключите Pico Key.
error-connection-failed = Не удалось подключиться к устройству
error-operation-failed = Операция не удалась
error-invalid-pin = Неверный PIN
error-pin-required = Требуется PIN
error-device-locked = Устройство заблокировано

# Статусные сообщения
status-ready = Готово
status-connecting = Подключение...
status-connected = Подключено
status-disconnected = Отключено
status-syncing = Синхронизация...
status-sync-complete = Синхронизация завершена

# Экран OATH
oath-title = OATH Аутентификатор
oath-subtitle = Управление учетными записями TOTP/HOTP для двухфакторной аутентификации.
oath-no-credentials = Нет учетных записей OATH
oath-no-credentials-desc = Добавьте учетные записи TOTP или HOTP для генерации кодов двухфакторной аутентификации.
oath-add = Добавить учетную запись
oath-account-name = Имя учетной записи
oath-secret-key = Секретный ключ (Base32)
oath-totp = TOTP
oath-hotp = HOTP
oath-digits = { $count } цифр
oath-credentials-stored = {count} учетных записей
oath-import-qr = Импорт QR-кода
oath-import-qr-paste = Или вставьте otpauth:// URI вручную:
oath-import-qr-camera = Сканер QR-кодов камерой
oath-import-qr-camera-hint = Наведите камеру на QR-код
oath-import-qr-starting = Запуск камеры...
oath-import-qr-scanning = Наведите камеру на QR-код...
oath-import-qr-found = QR-код обнаружен!
oath-import-qr-error = Ошибка камеры
oath-import-qr-no-camera = Камера недоступна
oath-import-qr-start-camera = Запустить камеру
oath-import-qr-start-camera-hint = Нажмите "Запустить камеру" для сканирования QR-кода или вставьте otpauth:// URI ниже

# Экран OTP
otp-title = OTP Аутентификатор
otp-subtitle = Управление учетными записями TOTP/HOTP для аутентификации одноразовыми паролями.
otp-no-credentials = Нет учетных записей OTP
otp-no-credentials-desc = Добавьте учетные записи TOTP или HOTP для аутентификации одноразовыми паролями.
otp-add = Добавить учетную запись

# Экран Vendor
vendor-title = Команды Vendor
vendor-subtitle = Расширенные операции Vendor и журнал устройства.
vendor-operations = Операции Vendor
vendor-export-oath = Экспорт учетных записей OATH
vendor-export-otp = Экспорт учетных записей OTP
vendor-backup = Резервное копирование устройства
vendor-restore = Восстановление резервной копии
vendor-logs = Журнал устройства
vendor-logs-clear = Очистить
vendor-logs-entries = {count} записей

# Экран прошивки
firmware-title = Обновление прошивки
firmware-subtitle = Проверка и обновление прошивки с GitHub.
firmware-information = Информация о прошивке
firmware-current-version = Текущая версия
firmware-latest-version = Последняя версия
firmware-check-updates = Проверить обновления
firmware-flash = Прошить прошивку
firmware-flash-progress = Прогресс прошивки
firmware-checking = Проверка GitHub на наличие обновлений...
firmware-update-available = Доступна новая версия: { $version }
firmware-check-failed = Ошибка проверки: { $error }
firmware-flashing = Прошивка устройства...
firmware-flash-success = Прошивка успешно завершена!
firmware-no-version = Неизвестно
firmware-not-checked = Не проверялось

# Экран безопасности (дополнительно)
security-lock-settings = Настройки блокировки
security-enable-secure-boot = Включить безопасную загрузку
security-verify-firmware = Проверяет подпись прошивки при запуске
security-secure-lock = Безопасная блокировка
security-prevent-debug = Предотвращает чтение ключей через отладочные порты
security-understand-risks = Я понимаю риски повреждения устройства.
security-permanently-lock = Постоянно заблокировать устройство
security-enabling = Включение безопасной загрузки...
security-disabling = Выключение безопасной загрузки...
