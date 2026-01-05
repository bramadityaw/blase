# 05-01-2026: Memulai implementasi language server
Ternyata `textDocument/diagnostics` tidak perlu di-`impl` langsung oleh server.
Language server untuk System Verilog memberikan diagnostics
melalui method `publish_diagnostics` yang disediakan oleh `Client`
yang dipanggil setiap `textDocument/didOpen` dan `textDocument/didChange`.
`blase` pun akan begitu; server akan mem-`push` diagnostic tiap ada perubahan dokumen.

Perhatikan kapan sebuah `Arc<RwLock<T>>` di-`drop`. Ini terlebih penting
untuk kode `async`. Jika suatu `Arc<RwLock<T>>` di-`drop` terlalu lambat
seperti di akhir fungsi, akan menyebabkan kegagalan kompilasi yang disebabkan
oleh `Arc<RwLock<T>>` yang tidak tertanda `Send`.

Besok: Mencoba server di Neovim.
