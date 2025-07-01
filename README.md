## Описание

**ssh-config-add** — это консольная утилита на Rust, предназначенная для генерации конфигурационных блоков SSH. Программа позволяет быстро и удобно создавать записи для SSH-конфига, минимизируя риск ошибок при ручном редактировании.

Пример использования:

```
ssh-config-add -p 2222 -I ~/.ssh/id_ed25519 -C -D user@example.com myhost1 myhost2
```

Выходной конфиг:

```
Host myhost1 myhost2
    HostName example.com
    User user
    Port 2222
    IdentityFile ~/.ssh/id_ed25519
    Compression yes
```
