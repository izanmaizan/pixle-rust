# Pixle

Layanan kompres dan resize gambar. Backend Rust untuk async, streaming upload, worker, dan penanganan error.

## Tech stack

| Bagian | Teknologi |
|---|---|
| Bahasa | Rust 2021 |
| Web | Axum 0.7, Tower HTTP |
| Async | Tokio |
| Gambar | crate `image` |
| Error | `thiserror` |
| Log | `tracing` |
| Rencana | sqlx (SQLite/Postgres), SSE progres, frontend drag-drop |

## Alur singkat

1. Klien mengunggah gambar lewat multipart.
2. Server menyimpan berkas sementara dan membuat job.
3. Worker memproses resize dan kompresi di latar belakang.
4. Progres dikirim ke klien lewat SSE.
5. Hasil tersedia untuk diunduh beserta persentase penghematan.

## Menjalankan

Butuh Rust toolchain.

```bash
rustup install stable
cargo run
```

Server di `http://localhost:8082`, health di `/health`.

```bash
cargo build --release
cargo test
```

## Struktur

```
src/main.rs    entrypoint, router, health check
Cargo.toml     dependency
```

## Status

Scaffold. Hanya endpoint `/health`. Belum ada upload, worker, atau penyimpanan.
