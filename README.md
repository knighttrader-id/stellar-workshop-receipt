# Stellar Recipes DApp

**Stellar Recipes DApp** — pencatatan resep di blockchain (Stellar Soroban)

## Deskripsi

Aplikasi ini adalah smart contract terdesentralisasi di jaringan Stellar memakai Soroban SDK. Pengguna menyimpan resep masakan (judul, bahan, langkah memasak) langsung di chain—data tersimpan transparan dan dikelola hanya lewat fungsi kontrak, tanpa bergantung pada basis data terpusat.

Setiap resep punya ID unik dan disimpan di instance storage kontrak.

## Visi

- **Desentralisasi**: Resep tidak dikunci di satu server aplikasi
- **Kepemilikan**: Kontrol atas data mengikuti akun yang berinteraksi dengan kontrak
- **Jejak yang dapat diverifikasi**: Pembuatan dan penghapusan resep dapat diaudit di explorer
- **Biaya rendah & cepat**: Memanfaatkan karakteristik jaringan Stellar

## Fitur utama

1. **Menambah resep** — `create_recipe(title, ingredients, instructions)`
2. **Membaca semua resep** — `get_recipes()`
3. **Menghapus resep** — `delete_recipe(id)` berdasarkan ID

## Detail teknis

- **Bahasa**: Rust (Soroban SDK 25)
- **Krat**: `contracts/recipes` (nama paket: `recipes`)

Setelah deploy ulang, catat alamat kontrak baru di dokumentasi atau frontend Anda. Alamat kontrak lama (jika ada) tidak lagi berlaku untuk kode resep ini.

## Memulai

```bash
cargo test -p recipes
cargo build -p recipes --target wasm32v1-none --release
```

Fungsi kontrak:

| Fungsi | Keterangan |
|--------|------------|
| `create_recipe` | Menyimpan resep baru (judul, bahan, instruksi) |
| `get_recipes` | Mengembalikan daftar semua resep |
| `delete_recipe` | Menghapus resep berdasarkan `id` |

## Deployed Contract
Link: https://lab.stellar.org/r/testnet/contract/CCKG4Z6ZYYT4QTPV4JQ3MLQVK4CU73ZL5FG3QQI27J3FKHIX2TNHVX2Z

## Smart Contract:
CCKG4Z6ZYYT4QTPV4JQ3MLQVK4CU73ZL5FG3QQI27J3FKHIX2TNHVX2Z

---

**Stellar Recipes DApp** — menyimpan resep di blockchain
