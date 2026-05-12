# Pemahaman Publisher

**1. Berapa banyak data yang dikirimkan oleh publisher dalam satu kali eksekusi?**
Publisher mengirimkan 5 buah data pesan (*event message*) secara beruntun ke dalam *message broker* (RabbitMQ) dalam satu kali jalannya program.

**2. Apa arti URL `amqp://guest:guest@localhost:5672`?**
URL ini adalah *connection string* yang identik dengan yang digunakan oleh program *subscriber*. Kesamaan URL ini menandakan bahwa *publisher* mengirimkan pesan ke server lokal dan *port* yang sama persis. Hal ini mutlak diperlukan agar pesan yang dilempar oleh *publisher* bisa masuk ke jalur yang benar dan akhirnya ditangkap oleh *subscriber*.

**Screenshot RabbitMQ:**
![RabbitMQ UI](rabbitmq.png)

**Screenshot Spike Chart:**
![Spike Chart](spike_chart.png)

*Penjelasan:* Grafik mengalami lonjakan tajam (spike) karena publisher melakukan pengiriman pesan (spam) dalam jumlah besar pada waktu yang hampir bersamaan. Akibatnya, antrean menumpuk sesaat di dalam RabbitMQ sebelum subscriber sempat memproses semuanya.







