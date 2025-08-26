# UOMI Knowledge Agent Super Final

Fitur:
- Menjawab pertanyaan tentang UOMI & Crypto
- Multi-turn conversation & riwayat pertanyaan
- Knowledge base auto-update dari docs.uomi.ai
- Integrasi OpenAI GPT
- Ambil harga crypto real-time dari API
- Backup knowledge base otomatis

## Cara pakai (Aman)
1. Copy `config/secrets.json.example` → rename menjadi `secrets.json`
2. Masukkan API key OpenAI kamu di `secrets.json`
3. Update knowledge base (opsional):
```bash
node scripts/update_kb.js
