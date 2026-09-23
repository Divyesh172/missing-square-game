# The Magic Triangle &mdash; Missing Square Paradox

[![Netlify Status](https://img.shields.io/badge/Live%20Deploy-Netlify-00C7B7?style=for-the-badge&logo=netlify)](https://missing-square-game.netlify.app)
[![GitHub](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Divyesh172/missing-square-game)

- **🌐 GitHub Repository:** [https://github.com/Divyesh172/missing-square-game](https://github.com/Divyesh172/missing-square-game)

An interactive, high-performance mathematical simulation of Paul Curry's **Missing Square Paradox**, powered by **Rust** and **WebAssembly**, designed with a warm tactile museum aesthetic matching the Monty Hall Simulation.

---

## 🚀 How to Start the Server

Because this project uses **WebAssembly** (`pkg/*.wasm`) and **ES Modules** (`js/*.js`), browsers require an HTTP server with the correct MIME types to load the game.

### Method 1: Python Launcher (Recommended &mdash; Same as Monty Hall)
Run the included launcher in PowerShell or Terminal:

```powershell
python serve.py
```

* Automatically opens `http://localhost:8080` in your default browser.
* Configures proper MIME types for `.wasm` and `.js`.
* Displays your local network IP (e.g. `http://192.168.x.x:8080`) so you can open it on your phone or tablet on the same Wi-Fi network.
* Press `Ctrl+C` to stop.

---

### Method 2: Node.js / npx
If you prefer Node.js:

```powershell
npx serve .
```

---

### Method 3: Python Built-in Module
```powershell
python -m http.server 8080
```

---

## 🦀 Rust Development & Testing

### Run Automated Tests
```powershell
cargo test
```
Verifies all 16 geometric, Cassini identity, grid collision, and puzzle validation invariants.

### Rebuild WebAssembly Module
```powershell
wasm-pack build --target web --release --out-dir ./pkg
```

---

## 🌟 Exhibit Topics
1. **🎮 1. Arena:** Interactive SVG board with draggable pieces, step-by-step rearrangement, laser ruler, and secret sliver toggle.
2. **🎪 2. The Slant:** Side-by-side comparison of the inward dip (-0.5 blocks) and outward bulge (+0.5 blocks).
3. **🧗 3. Slopes:** Animated climber race exposing the tiny $1.24^\circ$ angle difference between slopes $3/8$ ($0.375$) and $2/5$ ($0.400$).
4. **🐇 4. Fibonacci:** Interactive Cassini's Identity calculator ($F_{n-1}F_{n+1} - F_n^2 = \pm 1$) and puzzle scaler.
5. **❓ 5. Debunk:** Quick Q&A debunking the visual illusion and awarding exhibit badges.
