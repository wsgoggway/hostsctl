# hostsctl управление профилями /etc/hosts

## Создать профили

hostctl profile add dev
hostctl profile add staging

## Переключиться на dev

hostctl profile use dev

## Добавить запись в текущий профиль

hostctl add api.local 192.168.0.10
hostctl add db.local 192.168.0.20

## Обновить/удалить

hostctl update api.local 10.0.0.5
hostctl remove db.local

## Переключиться на staging и добавить другие записи

hostctl profile use staging
hostctl add api.local 10.0.0.100

## dry-run для проверки, что получится

hostctl test --profile dev

## Применить к /etc/hosts (нужен sudo)

sudo hostctl apply --profile staging
