# UOMI Knowledge Agent Bilingual EN + ID

Features:
- Answer questions about UOMI & Crypto
- Supports English & Indonesian
- Multi-turn conversation & history tracking
- Auto-update knowledge base from docs.uomi.ai
- GPT integration via OpenAI
- Fetch real-time crypto prices
- Automatic knowledge base backup

## How to use (Safe)
1. Copy `config/secrets.json.example` → rename to `secrets.json`
2. Insert your OpenAI API key in `secrets.json` (optional, required for GPT)
3. Update knowledge base (optional):
```bash
node scripts/update_kb.js
