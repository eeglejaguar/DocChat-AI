# DocChat-AI
Rust-based backend using Axum for uploading PDFs, extracting content, and powering LLM-based document chat (DocChat AI)

--

## 🚀 Features (Current + Planned)

### ✅ Implemented
- [x] PDF upload endpoint (`/upload`)
- [x] Text extraction from PDF using `lopdf`/`pdf_extract`
- [x] Asynchronous server with `tokio` and `axum`

### 🧠 Coming Soon
- [ ] Generate text embeddings from PDF using OpenAI or HuggingFace
- [ ] Store embeddings in a vector database (Pinecone, Qdrant, or Faiss)
- [ ] Ask questions and get answers from your documents (RAG pipeline)
- [ ] Frontend for document viewer and interactive chat
- [ ] User authentication + session-based chat history

---

## 🛠 Tech Stack

| Layer           | Tech           |
|----------------|----------------|
| Language        | Rust 🦀         |
| Web Framework   | Axum           |
| Async Runtime   | Tokio          |
| PDF Processing  | lopdf / pdf_extract |
| Embeddings      | OpenAI / HuggingFace |
| Vector DB       | Qdrant / Pinecone / Faiss |
| Chat Interface  | React / Next.js (planned) |

---

## 📦 How to Run

```bash
# Clone the repo
git clone https://github.com/YOUR_USERNAME/docchat_backend.git
cd docchat_backend

# Run the server
cargo run
