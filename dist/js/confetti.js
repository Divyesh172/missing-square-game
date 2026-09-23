// Lightweight canvas-based celebratory confetti burst
// Natural warm wooden-confetti particles with fluttering physics

class ConfettiCanon {
  constructor() {
    this.canvas = null;
    this.ctx = null;
    this.particles = [];
    this.animationId = null;
    this.colors = [
      '#E05A47', // Terracotta
      '#F5C842', // Warm Gold / Ochre
      '#8FB78F', // Sage Olive
      '#28536B', // Petrol Slate
      '#DDAE27', // Ochre Border
      '#C94A38', // Terracotta Dark
      '#DBD5C6', // Architectural Sand
      '#EFECE3'  // Cream Plinth
    ];
  }

  ensureCanvas() {
    if (!this.canvas) {
      this.canvas = document.createElement('canvas');
      this.canvas.id = 'confetti-canvas';
      this.canvas.style.position = 'fixed';
      this.canvas.style.top = '0';
      this.canvas.style.left = '0';
      this.canvas.style.width = '100vw';
      this.canvas.style.height = '100vh';
      this.canvas.style.pointerEvents = 'none';
      this.canvas.style.zIndex = '9999';
      document.body.appendChild(this.canvas);
      this.ctx = this.canvas.getContext('2d');
      this.resize();
      window.addEventListener('resize', () => this.resize());
    }
  }

  resize() {
    if (!this.canvas) return;
    this.canvas.width = window.innerWidth * window.devicePixelRatio;
    this.canvas.height = window.innerHeight * window.devicePixelRatio;
    if (this.ctx) {
      this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    }
  }

  burst(originX = window.innerWidth / 2, originY = window.innerHeight / 2.5, count = 90) {
    this.ensureCanvas();
    const width = window.innerWidth;
    const height = window.innerHeight;

    for (let i = 0; i < count; i++) {
      const angle = (Math.random() * 360 * Math.PI) / 180;
      const speed = 4 + Math.random() * 11;
      const color = this.colors[Math.floor(Math.random() * this.colors.length)];
      const type = Math.random() > 0.3 ? 'rect' : 'circle';

      this.particles.push({
        x: originX,
        y: originY,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed - (3 + Math.random() * 5),
        size: 6 + Math.random() * 6,
        color: color,
        type: type,
        rotation: Math.random() * 360,
        rotationSpeed: (Math.random() - 0.5) * 12,
        wobble: Math.random() * 10,
        wobbleSpeed: 0.1 + Math.random() * 0.1,
        alpha: 1,
        decay: 0.007 + Math.random() * 0.009,
        gravity: 0.28 + Math.random() * 0.12
      });
    }

    if (!this.animationId) {
      this.animate();
    }
  }

  animate() {
    if (!this.ctx || !this.canvas) return;
    const width = window.innerWidth;
    const height = window.innerHeight;

    this.ctx.clearRect(0, 0, width, height);

    for (let i = this.particles.length - 1; i >= 0; i--) {
      const p = this.particles[i];

      p.x += p.vx;
      p.y += p.vy;
      p.vy += p.gravity;
      p.vx *= 0.98; // Air drag
      p.rotation += p.rotationSpeed;
      p.wobble += p.wobbleSpeed;
      p.alpha -= p.decay;

      if (p.alpha <= 0 || p.y > height + 50) {
        this.particles.splice(i, 1);
        continue;
      }

      this.ctx.save();
      this.ctx.translate(p.x, p.y);
      this.ctx.rotate((p.rotation * Math.PI) / 180);
      this.ctx.scale(Math.sin(p.wobble), 1);
      this.ctx.globalAlpha = Math.max(0, p.alpha);
      this.ctx.fillStyle = p.color;

      if (p.type === 'rect') {
        this.ctx.fillRect(-p.size / 2, -p.size / 2, p.size, p.size * 0.65);
      } else {
        this.ctx.beginPath();
        this.ctx.arc(0, 0, p.size * 0.45, 0, Math.PI * 2);
        this.ctx.fill();
      }

      this.ctx.restore();
    }

    if (this.particles.length > 0) {
      this.animationId = requestAnimationFrame(() => this.animate());
    } else {
      this.animationId = null;
      this.ctx.clearRect(0, 0, width, height);
    }
  }
}

export const confetti = new ConfettiCanon();
