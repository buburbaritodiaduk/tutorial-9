# Pemahaman Konsep AMQP dan RabbitMQ

**1. Apa itu AMQP?**
AMQP (Advanced Message Queuing Protocol) adalah protokol standar terbuka untuk pengiriman pesan antar aplikasi (*message-oriented middleware*). Protokol ini memastikan pesan dikirim secara aman, reliabel, dan terstruktur antara berbagai sistem atau layanan, terlepas dari bahasa pemrograman atau platform yang mereka gunakan.

**2. Apa maksud dari `guest:guest@localhost:5672`?**
URL ini adalah format string koneksi (*connection string*) ke server RabbitMQ dengan rincian:
- `guest:guest` = Merupakan kredensial bawaan berupa *username* (`guest`) dan *password* (`guest`) untuk login ke server broker.
- `@localhost` = Alamat host/server tempat RabbitMQ berjalan (karena dijalankan di Docker lokal, maka alamatnya di mesin sendiri).
- `:5672` = Merupakan *port* standar (*default*) yang digunakan oleh AMQP untuk jalur komunikasi pengiriman pesan.