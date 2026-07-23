# Боковое меню
sidebar-menu = Меню
sidebar-home = Главная
sidebar-passkeys = Ключи
sidebar-config = Конфигурация
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
passkeys-credentials-count = { $count -> 
    [one] { $count } учетная запись
    [few] { $count } учетные записи
    [many] { $count } учетных записей
    *[other] { $count } учетных записей
}

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
config-apply = Применить
config-cancel = Отмена
config-vid = VID
config-pid = PID
config-serial-number = Серийный номер
config-min-pin-length = Мин. длина PIN
config-touch-timeout = Тайм-аут касания
config-led-brightness = Яркость LED
config-led-gpio = LED GPIO

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
