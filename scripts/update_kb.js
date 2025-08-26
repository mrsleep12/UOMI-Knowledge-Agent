const fs = require("fs");
const axios = require("axios");
const cheerio = require("cheerio");

const URL = "https://docs.uomi.ai";
const OUTPUT_FILE = "knowledge/uomi_docs.json";
const BACKUP_FILE = "knowledge/uomi_docs_backup.json";

async function fetchDocs() {
  try {
    if (fs.existsSync(OUTPUT_FILE)) {
      fs.copyFileSync(OUTPUT_FILE, BACKUP_FILE);
      console.log("Backup file created:", BACKUP_FILE);
    }

    const { data } = await axios.get(URL);
    const $ = cheerio.load(data);
    const kb = {};

    $("h2").each((i, el) => {
      const key = $(el).text().trim();
      const value = $(el).next("p").text().trim();
      if (key && value) kb[key] = value;
    });

    fs.writeFileSync(OUTPUT_FILE, JSON.stringify(kb, null, 2));
    console.log("Knowledge base updated successfully:", OUTPUT_FILE);

  } catch (err) {
    console.error("Failed to update knowledge base:", err.message);
  }
}

fetchDocs();
