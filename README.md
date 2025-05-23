# Module 10 Reflection

## Experiment 1.2: Understanding how it works

Perintah print yang dijalankan lebih dulu adalah perintah print yang diluar spawner. Hal ini dikarenakan perintah print
yang ada di dalam spawner baru di eksekusi setelah executor dijalankan.

## Experiment 1.3: Multiple Spawn and removing drop

![Terminal immediately after cargo run executed](/assets/images/multiple_spawner_1.png)
Terminal ketika cargo run di execute

![Terminal 5 second after cargo run executed](/assets/images/multiple_spawner_2.png)
Terminal setelah 5 detik cargo run dijalankan

Dari kedua gambar tersebut, terlihat bahwa setiap spawner yang dibuat ketika di execute berjalan secara paralel dan
bersamaan. Hal ini dapat dilihat ketika message yang dikeluarkan keluar secara bersamaan

![Terminal when drop got removed](/assets/images/drop_removed.png)
Setelah drop di removed, terlihat bahwa program tidak akan selesai, hal ini dikarenakan tidak ada penanda bahwa
spawner sudah selesai (dengan drop) sehingga executor terus menjalankan program

## Experiment 2.1: Original code, and how it run

![Original broadcast server with 3 client](/assets/images/original_broadcast.png)
Ketika server dijalankan dan ada client baru yang masuk maka server akan mendeteksi client tersebut. Kemudian message
yang dikirim oleh client diterima oleh server dan dikirimkan ke client lainnya yang sedang terhubung dengan server juga.
