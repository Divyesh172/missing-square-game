// =============================================================================
// THE MAGIC TRIANGLE: THE GREAT MISSING SQUARE MYSTERY
// Multi-Page Zero-Scroll Controller & WebAssembly Engine Integration
// =============================================================================

import init, { PuzzleEngine } from "../pkg/missing_square_wasm.js";
import { audio } from "./audio.js";
import { confetti } from "./confetti.js";

// =============================================================================
// 1. MULTI-PAGE SLIDE ROUTER
// =============================================================================
class SlideRouter {
  constructor() {
    this.currentIndex = 0;
    this.totalTopics = 5;
    this.topics = ["arena", "hammock", "slope", "fibonacci", "detective"];
    this.topicLabels = ["1. Arena", "2. The Slant", "3. Slopes", "4. Fibonacci", "5. Debunk"];
    this.track = document.getElementById("stage-track");
    this.glider = document.querySelector(".nav-pill-glider");
    this.tabButtons = document.querySelectorAll(".nav-pill-btn");
    this.dots = document.querySelectorAll(".slide-dots .dot");
    this.pageCounter = document.getElementById("page-counter");
    this.breadcrumbTopic = document.getElementById("breadcrumb-topic");

    this.navPrev = document.getElementById("nav-prev");
    this.navNext = document.getElementById("nav-next");
    this.dockPrev = document.getElementById("btn-dock-prev");
    this.dockNext = document.getElementById("btn-dock-next");

    this.initEvents();
    this.handleRoute();
  }

  goTo(index, pushHash = true) {
    if (index < 0 || index >= this.totalTopics) return;
    this.currentIndex = index;

    // Slide track
    if (this.track) {
      this.track.style.transform = `translateX(-${index * 100}vw)`;
    }

    // Update Tab Navigation Pill
    this.tabButtons.forEach((btn, i) => {
      const active = i === index;
      btn.classList.toggle("active", active);
      btn.setAttribute("aria-selected", active);
    });

    const activeBtn = this.tabButtons[index];
    if (activeBtn && this.glider) {
      this.glider.style.width = `${activeBtn.offsetWidth}px`;
      this.glider.style.transform = `translateX(${activeBtn.offsetLeft}px)`;
    }

    // Update Breadcrumb Topic
    if (this.breadcrumbTopic) {
      this.breadcrumbTopic.textContent = this.topicLabels[index] || "";
    }

    // Update Dots & Counter
    this.dots.forEach((dot, i) => dot.classList.toggle("active", i === index));
    if (this.pageCounter) {
      this.pageCounter.textContent = `${index + 1} / ${this.totalTopics}`;
    }

    // Update Chevron & Dock Buttons
    const isFirst = index === 0;
    const isLast = index === this.totalTopics - 1;
    if (this.navPrev) this.navPrev.disabled = isFirst;
    if (this.navNext) this.navNext.disabled = isLast;
    if (this.dockPrev) this.dockPrev.disabled = isFirst;
    if (this.dockNext) {
      this.dockNext.textContent = isLast ? "Back to Arena ➔" : "Next ➔";
    }

    if (pushHash) {
      window.location.hash = this.topics[index];
    }

    audio.playClick();
  }

  next() {
    if (this.currentIndex >= this.totalTopics - 1) {
      this.goTo(0);
    } else {
      this.goTo(this.currentIndex + 1);
    }
  }

  prev() {
    if (this.currentIndex > 0) {
      this.goTo(this.currentIndex - 1);
    }
  }

  handleRoute() {
    const hash = window.location.hash.replace("#", "");
    const idx = this.topics.indexOf(hash);
    this.goTo(idx !== -1 ? idx : 0, false);
  }

  initEvents() {
    window.addEventListener("hashchange", () => this.handleRoute());
    window.addEventListener("resize", () => {
      const activeBtn = this.tabButtons[this.currentIndex];
      if (activeBtn && this.glider) {
        this.glider.style.width = `${activeBtn.offsetWidth}px`;
        this.glider.style.transform = `translateX(${activeBtn.offsetLeft}px)`;
      }
    });

    // Top Pill Buttons
    this.tabButtons.forEach((btn) => {
      btn.addEventListener("click", () => {
        const idx = parseInt(btn.dataset.topic, 10);
        this.goTo(idx);
      });
    });

    // Dots
    this.dots.forEach((dot) => {
      dot.addEventListener("click", () => {
        const idx = parseInt(dot.dataset.topic, 10);
        this.goTo(idx);
      });
    });

    // Edge Navigation Chevrons
    if (this.navPrev) this.navPrev.addEventListener("click", () => this.prev());
    if (this.navNext) this.navNext.addEventListener("click", () => this.next());

    // Bottom Dock Buttons
    if (this.dockPrev) this.dockPrev.addEventListener("click", () => this.prev());
    if (this.dockNext) this.dockNext.addEventListener("click", () => this.next());

    // Jump from Hammock punchline to Arena secret
    const jumpBtn = document.getElementById("btn-jump-arena-secret");
    if (jumpBtn) {
      jumpBtn.addEventListener("click", () => {
        this.goTo(0);
        setTimeout(() => {
          document.getElementById("step-4-btn")?.click();
        }, 300);
      });
    }

    // Keyboard Shortcuts
    window.addEventListener("keydown", (e) => {
      if (e.target.tagName === "INPUT" || e.target.tagName === "TEXTAREA") return;
      if (e.code === "ArrowRight" || e.code === "KeyD") this.next();
      if (e.code === "ArrowLeft" || e.code === "KeyA") this.prev();
      if (e.key >= "1" && e.key <= "5") this.goTo(parseInt(e.key, 10) - 1);
    });

    // Touch Swipe Gestures
    let touchStartX = 0;
    let touchStartY = 0;
    const stageContainer = document.querySelector(".app-stage-container");

    if (stageContainer) {
      stageContainer.addEventListener("touchstart", (e) => {
        touchStartX = e.changedTouches[0].screenX;
        touchStartY = e.changedTouches[0].screenY;
      }, { passive: true });

      stageContainer.addEventListener("touchend", (e) => {
        const diffX = e.changedTouches[0].screenX - touchStartX;
        const diffY = e.changedTouches[0].screenY - touchStartY;
        if (Math.abs(diffX) > 50 && Math.abs(diffX) > Math.abs(diffY) * 1.5) {
          if (diffX < 0) this.next();
          else this.prev();
        }
      }, { passive: true });
    }
  }
}

// =============================================================================
// 2. PIECE DATA & COORDINATES (13x5 Grid, Matching Monty Hall Palette)
// =============================================================================
const PIECE_CONFIGS = {
  // Configuration 1: Solid Triangle (Hammock Sag, Area 32)
  A: {
    red: { x: 0, y: 2 },       // Base 8, Height 3 (SVG y: 2 to 5)
    teal: { x: 8, y: 0 },      // Base 5, Height 2 (SVG y: 0 to 2)
    sage: { x: 8, y: 2 },      // 8-omino
    honey: { x: 8, y: 3 },     // 7-omino
  },
  // Configuration 2: Missing Square Paradox (Belly Bulge, Hole at (7,3))
  B: {
    teal: { x: 0, y: 3 },      // Base 5, Height 2 (SVG y: 3 to 5)
    red: { x: 5, y: 0 },       // Base 8, Height 3 (SVG y: 0 to 3)
    honey: { x: 5, y: 3 },     // 7-omino
    sage: { x: 8, y: 3 },      // 8-omino
  }
};

const PIECE_SHAPES = {
  red: {
    id: "red",
    name: "Terracotta Triangle (8×3)",
    color: "#E05A47",
    stroke: "#C94A38",
    area: 12,
    points: "0,300 800,300 800,0",
    w: 8,
    h: 3,
  },
  teal: {
    id: "teal",
    name: "Sage Triangle (5×2)",
    color: "#8FB78F",
    stroke: "#7AA27A",
    area: 5,
    points: "0,200 500,200 500,0",
    w: 5,
    h: 2,
  },
  honey: {
    id: "honey",
    name: "Ochre Polyomino (7 Units)",
    color: "#F5C842",
    stroke: "#DDAE27",
    area: 7,
    points: "0,200 500,200 500,100 200,100 200,0 0,0",
    w: 5,
    h: 2,
  },
  sage: {
    id: "sage",
    name: "Petrol Polyomino (8 Units)",
    color: "#28536B",
    stroke: "#1F4256",
    area: 8,
    points: "0,200 500,200 500,0 200,0 200,100 0,100",
    w: 5,
    h: 2,
  }
};

// =============================================================================
// 3. MAIN APPLICATION & WASM CONTROLLER
// =============================================================================
class MagicTriangleApp {
  constructor() {
    this.wasmEngine = null;
    this.pieces = {};
    this.activeConfig = "A";
    this.activeStep = 1;
    this.laserVisible = false;
    this.stripVisible = false;
    this.bendMultiplier = 1.0;
    this.isDragging = false;
    this.draggedPiece = null;
    this.dragOffset = { x: 0, y: 0 };

    this.svg = document.getElementById("puzzle-svg");
    this.piecesLayer = document.getElementById("pieces-layer");
    this.laserLine = document.getElementById("laser-line");
    this.secretLozenge = document.getElementById("secret-lozenge");
    this.exaggeratedPath = document.getElementById("exaggerated-path");
    this.holeMarker = document.getElementById("hole-marker");
    this.scoreVoid = document.getElementById("score-void-pill");
    this.scoreStatus = document.getElementById("score-status-pill");
    this.bendValueText = document.getElementById("bend-value");
  }

  async init() {
    await this.initWasm();
    this.buildSvgPieces();
    this.bindArenaEvents();
    this.bindHillClimber();
    this.bindFibonacciExplorer();
    this.bindAudio();
    this.applyConfiguration("A", false);
  }

  async initWasm() {
    try {
      await init();
      this.wasmEngine = new PuzzleEngine();
      console.log("✅ Rust WebAssembly engine initialized successfully!");
    } catch (err) {
      console.warn("WASM initialization fallback to native JS geometry:", err);
    }
  }

  buildSvgPieces() {
    this.piecesLayer.innerHTML = "";

    Object.keys(PIECE_SHAPES).forEach((id) => {
      const def = PIECE_SHAPES[id];
      const g = document.createElementNS("http://www.w3.org/2000/svg", "g");
      g.setAttribute("id", `piece-${id}`);
      g.setAttribute("class", "puzzle-piece-group");
      g.setAttribute("data-piece-id", id);
      g.style.cursor = "grab";
      g.style.transition = "transform 0.5s cubic-bezier(0.34, 1.4, 0.64, 1)";

      const poly = document.createElementNS("http://www.w3.org/2000/svg", "polygon");
      poly.setAttribute("points", def.points);
      poly.setAttribute("fill", def.color);
      poly.setAttribute("stroke", def.stroke);
      poly.setAttribute("stroke-width", "2.5");
      poly.setAttribute("stroke-linejoin", "round");
      poly.setAttribute("filter", "url(#block-shadow)");

      g.appendChild(poly);
      this.piecesLayer.appendChild(g);

      this.pieces[id] = {
        element: g,
        gridX: PIECE_CONFIGS.A[id].x,
        gridY: PIECE_CONFIGS.A[id].y,
        def,
      };

      this.bindDragEvents(g, id);
    });
  }

  applyConfiguration(configKey, animate = true) {
    this.activeConfig = configKey;
    const target = PIECE_CONFIGS[configKey];

    Object.keys(target).forEach((id) => {
      const pos = target[id];
      this.pieces[id].gridX = pos.x;
      this.pieces[id].gridY = pos.y;
      this.updatePieceTransform(id, pos.x, pos.y, animate);
    });

    const isB = configKey === "B";
    if (this.holeMarker) this.holeMarker.style.opacity = isB ? "1" : "0";
    if (this.scoreVoid) this.scoreVoid.innerHTML = isB ? "Hole: <strong>1</strong>" : "Hole: <strong>0</strong>";
    if (this.scoreStatus) {
      this.scoreStatus.textContent = isB ? "1 Hole Void" : "Solid Shape";
      this.scoreStatus.style.color = isB ? "var(--color-terracotta-dark)" : "var(--color-sage-dark)";
      this.scoreStatus.style.background = isB ? "var(--color-terracotta-soft)" : "var(--color-sage-soft)";
    }

    this.updateExaggeratedBend();
  }

  updatePieceTransform(id, gx, gy, animate = true) {
    const p = this.pieces[id];
    if (!p) return;
    p.element.style.transition = animate ? "transform 0.55s cubic-bezier(0.34, 1.4, 0.64, 1)" : "none";
    p.element.setAttribute("transform", `translate(${gx * 100}, ${gy * 100})`);
  }

  bindDragEvents(group, id) {
    group.addEventListener("pointerdown", (e) => {
      e.stopPropagation();
      this.isDragging = true;
      this.draggedPiece = id;
      group.setPointerCapture(e.pointerId);
      this.piecesLayer.appendChild(group);
      group.style.cursor = "grabbing";

      const svgPt = this.clientToSvg(e.clientX, e.clientY);
      const curGx = this.pieces[id].gridX;
      const curGy = this.pieces[id].gridY;
      this.dragOffset = {
        x: svgPt.x - curGx * 100,
        y: svgPt.y - curGy * 100,
      };

      audio.playClick();
    });

    group.addEventListener("pointermove", (e) => {
      if (!this.isDragging || this.draggedPiece !== id) return;
      const svgPt = this.clientToSvg(e.clientX, e.clientY);
      const newX = svgPt.x - this.dragOffset.x;
      const newY = svgPt.y - this.dragOffset.y;

      group.style.transition = "none";
      group.setAttribute("transform", `translate(${newX}, ${newY})`);
    });

    const finishDrag = (e) => {
      if (!this.isDragging || this.draggedPiece !== id) return;
      this.isDragging = false;
      this.draggedPiece = null;
      group.releasePointerCapture(e.pointerId);
      group.style.cursor = "grab";

      const svgPt = this.clientToSvg(e.clientX, e.clientY);
      const rawGx = (svgPt.x - this.dragOffset.x) / 100;
      const rawGy = (svgPt.y - this.dragOffset.y) / 100;

      const snappedGx = Math.max(0, Math.min(13 - this.pieces[id].def.w, Math.round(rawGx)));
      const snappedGy = Math.max(0, Math.min(5 - this.pieces[id].def.h, Math.round(rawGy)));

      this.pieces[id].gridX = snappedGx;
      this.pieces[id].gridY = snappedGy;
      this.updatePieceTransform(id, snappedGx, snappedGy, true);

      audio.playSnap();
    };

    group.addEventListener("pointerup", finishDrag);
    group.addEventListener("pointercancel", finishDrag);
  }

  clientToSvg(clientX, clientY) {
    const pt = this.svg.createSVGPoint();
    pt.x = clientX;
    pt.y = clientY;
    const ctm = this.svg.getScreenCTM().inverse();
    return pt.matrixTransform(ctm);
  }

  updateExaggeratedBend() {
    if (!this.exaggeratedPath) return;

    if (this.bendMultiplier <= 1.05) {
      this.exaggeratedPath.style.opacity = "0";
      return;
    }

    const isA = this.activeConfig === "A";
    const factor = (this.bendMultiplier - 1.0) * 55;

    const midX = isA ? 800 : 500;
    const baseY = isA ? 200 : 300;
    const midY = isA ? baseY + factor : baseY - factor;

    this.exaggeratedPath.setAttribute("d", `M 0,500 Q ${midX},${midY} 1300,0`);
    this.exaggeratedPath.style.opacity = "0.9";
  }

  bindArenaEvents() {
    const stepBtns = [
      document.getElementById("step-1-btn"),
      document.getElementById("step-2-btn"),
      document.getElementById("step-3-btn"),
      document.getElementById("step-4-btn"),
    ];

    stepBtns.forEach((btn, idx) => {
      if (!btn) return;
      btn.addEventListener("click", () => {
        stepBtns.forEach((b) => b?.classList.remove("active"));
        btn.classList.add("active");
        this.activeStep = idx + 1;

        if (this.activeStep === 1) {
          this.laserVisible = false;
          this.stripVisible = false;
          this.laserLine.style.opacity = "0";
          this.secretLozenge.setAttribute("fill-opacity", "0");
          this.secretLozenge.setAttribute("stroke-opacity", "0");
          document.getElementById("toggle-laser-btn")?.classList.remove("active");
          document.getElementById("toggle-strip-btn")?.classList.remove("active");
          this.applyConfiguration("A", true);
          audio.playClick();
        } else if (this.activeStep === 2) {
          this.applyConfiguration("B", true);
          audio.playMystery();
        } else if (this.activeStep === 3) {
          this.applyConfiguration("B", true);
          if (this.holeMarker) this.holeMarker.style.opacity = "1";
          audio.playMystery();
        } else if (this.activeStep === 4) {
          this.laserVisible = true;
          this.stripVisible = true;
          this.laserLine.style.opacity = "1";
          this.secretLozenge.setAttribute("fill-opacity", "0.28");
          this.secretLozenge.setAttribute("stroke-opacity", "1");
          document.getElementById("toggle-laser-btn")?.classList.add("active");
          document.getElementById("toggle-strip-btn")?.classList.add("active");
          confetti.burst();
          audio.playVictory();
        }
      });
    });

    const toggleLaser = document.getElementById("toggle-laser-btn");
    if (toggleLaser) {
      toggleLaser.addEventListener("click", () => {
        this.laserVisible = !this.laserVisible;
        toggleLaser.classList.toggle("active", this.laserVisible);
        this.laserLine.style.opacity = this.laserVisible ? "1" : "0";
        audio.playClick();
      });
    }

    const toggleStrip = document.getElementById("toggle-strip-btn");
    if (toggleStrip) {
      toggleStrip.addEventListener("click", () => {
        this.stripVisible = !this.stripVisible;
        toggleStrip.classList.toggle("active", this.stripVisible);
        this.secretLozenge.setAttribute("fill-opacity", this.stripVisible ? "0.28" : "0");
        this.secretLozenge.setAttribute("stroke-opacity", this.stripVisible ? "1" : "0");
        audio.playClick();
      });
    }

    const bendSlider = document.getElementById("bend-slider");
    if (bendSlider) {
      bendSlider.addEventListener("input", (e) => {
        this.bendMultiplier = parseFloat(e.target.value);
        if (this.bendValueText) {
          this.bendValueText.textContent = `${this.bendMultiplier.toFixed(1)}×`;
        }
        this.updateExaggeratedBend();
      });
    }

    const resetBtn = document.getElementById("btn-reset-puzzle");
    if (resetBtn) {
      resetBtn.addEventListener("click", () => {
        stepBtns[0]?.click();
        if (bendSlider) {
          bendSlider.value = "1.0";
          if (this.bendValueText) this.bendValueText.textContent = "1.0×";
          this.bendMultiplier = 1.0;
          this.updateExaggeratedBend();
        }
        audio.playClick();
      });
    }
  }

  bindHillClimber() {
    const raceBtn = document.getElementById("btn-start-race");
    const hikerRed = document.getElementById("hiker-red");
    const hikerGreen = document.getElementById("hiker-green");

    if (!raceBtn || !hikerRed || !hikerGreen) return;

    let isRacing = false;

    raceBtn.addEventListener("click", () => {
      if (isRacing) return;
      isRacing = true;
      audio.playMystery();

      let progress = 0;
      const startTime = performance.now();
      const duration = 2200;

      const animateRace = (now) => {
        const elapsed = now - startTime;
        progress = Math.min(1, elapsed / duration);

        const rx = 50 + 400 * progress;
        const ry = 310 - 160 * progress;
        hikerRed.setAttribute("transform", `translate(${rx}, ${ry})`);

        const gx = 550 + 400 * progress;
        const gy = 310 - 180 * progress;
        hikerGreen.setAttribute("transform", `translate(${gx}, ${gy})`);

        if (progress < 1) {
          requestAnimationFrame(animateRace);
        } else {
          isRacing = false;
          audio.playVictory();
          raceBtn.innerHTML = `<span>🏆 Sage Climbed Steeper!</span>`;
          setTimeout(() => {
            raceBtn.innerHTML = `<span>Start race ➔</span>`;
          }, 3000);
        }
      };

      requestAnimationFrame(animateRace);
    });
  }

  bindFibonacciExplorer() {
    const slider = document.getElementById("fib-slider");
    const setName = document.getElementById("fib-set-name");
    const eqNeighbors = document.getElementById("fib-neighbors");
    const eqMiddle = document.getElementById("fib-middle");
    const eqDiff = document.getElementById("fib-diff");
    const errorPill = document.getElementById("fib-steepness-error");

    if (!slider) return;

    const fibs = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    const names = {
      3: "Tiny (1, 2, 3)",
      4: "Junior (2, 3, 5)",
      5: "Classic Curry (3, 5, 8)",
      6: "Giant (5, 8, 13)",
      7: "Colossal (8, 13, 21)",
    };

    slider.addEventListener("input", (e) => {
      const n = parseInt(e.target.value, 10);
      const fn_m1 = fibs[n - 1];
      const fn = fibs[n];
      const fn_p1 = fibs[n + 1];

      const prod = fn_m1 * fn_p1;
      const square = fn * fn;
      const diff = Math.abs(prod - square);

      if (setName) setName.textContent = names[n] || `Triple (${fn_m1}, ${fn}, ${fn_p1})`;
      if (eqNeighbors) eqNeighbors.textContent = `${fn_m1} × ${fn_p1} = ${prod}`;
      if (eqMiddle) eqMiddle.textContent = `${fn} × ${fn} = ${square}`;
      if (eqDiff) eqDiff.textContent = `EXACTLY ${diff}`;

      const slopeDiff = 1 / (fn * fn_p1);
      if (errorPill) {
        errorPill.textContent = `Slope diff: 1/${fn * fn_p1} = ${slopeDiff.toFixed(4)}`;
      }

      audio.playClick();
    });
  }

  bindAudio() {
    const audioBtn = document.getElementById("audio-toggle-btn");
    const audioIcon = document.getElementById("audio-icon");
    if (!audioBtn) return;

    audioBtn.addEventListener("click", () => {
      const muted = audio.toggleMute();
      if (audioIcon) audioIcon.textContent = muted ? "🔇" : "🔊";
      audioBtn.title = muted ? "Sound Off" : "Sound On";
    });
  }
}

// =============================================================================
// 4. BOOTSTRAP
// =============================================================================
window.addEventListener("DOMContentLoaded", () => {
  new SlideRouter();
  const app = new MagicTriangleApp();
  app.init().catch(console.error);
});
