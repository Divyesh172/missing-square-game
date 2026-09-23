// Tactile Web Audio API sound generator for the Missing Square Mystery
// Zero external sound assets needed - 100% synthesized wooden acoustics!

class AudioManager {
  constructor() {
    this.ctx = null;
    this.muted = false;
  }

  ensureContext() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      if (AudioCtx) {
        this.ctx = new AudioCtx();
      }
    }
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  toggleMute() {
    this.muted = !this.muted;
    return this.muted;
  }

  // 1. Tactile Wooden Click (Natural wood block bump)
  playClick() {
    if (this.muted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    const filter = this.ctx.createBiquadFilter();

    osc.type = 'triangle';
    osc.frequency.setValueAtTime(320 + Math.random() * 40, t);
    osc.frequency.exponentialRampToValueAtTime(140, t + 0.05);

    filter.type = 'bandpass';
    filter.frequency.setValueAtTime(600, t);
    filter.Q.setValueAtTime(4.0, t);

    gain.gain.setValueAtTime(0.4, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.06);

    osc.connect(filter);
    filter.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.06);
  }

  // 2. Magnetic Latch Snap (Piece locks into place)
  playSnap() {
    if (this.muted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;

    // High click component
    const clickOsc = this.ctx.createOscillator();
    const clickGain = this.ctx.createGain();
    clickOsc.type = 'sine';
    clickOsc.frequency.setValueAtTime(1100, t);
    clickOsc.frequency.exponentialRampToValueAtTime(200, t + 0.04);
    clickGain.gain.setValueAtTime(0.35, t);
    clickGain.gain.exponentialRampToValueAtTime(0.001, t + 0.045);

    clickOsc.connect(clickGain);
    clickGain.connect(this.ctx.destination);
    clickOsc.start(t);
    clickOsc.stop(t + 0.045);

    // Warm body resonance
    const bodyOsc = this.ctx.createOscillator();
    const bodyGain = this.ctx.createGain();
    bodyOsc.type = 'triangle';
    bodyOsc.frequency.setValueAtTime(440, t + 0.01);
    bodyOsc.frequency.exponentialRampToValueAtTime(120, t + 0.12);
    bodyGain.gain.setValueAtTime(0.3, t + 0.01);
    bodyGain.gain.exponentialRampToValueAtTime(0.001, t + 0.13);

    bodyOsc.connect(bodyGain);
    bodyGain.connect(this.ctx.destination);
    bodyOsc.start(t + 0.01);
    bodyOsc.stop(t + 0.13);
  }

  // 3. Mystery Chime (Revealing a mystery or secret tool)
  playMystery() {
    if (this.muted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;
    const notes = [523.25, 659.25, 783.99, 1046.50]; // C5, E5, G5, C6 (warm sparkling chord)

    notes.forEach((freq, idx) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();

      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, t + idx * 0.07);

      gain.gain.setValueAtTime(0.001, t + idx * 0.07);
      gain.gain.linearRampToValueAtTime(0.18, t + idx * 0.07 + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.0001, t + idx * 0.07 + 0.8);

      osc.connect(gain);
      gain.connect(this.ctx.destination);

      osc.start(t + idx * 0.07);
      osc.stop(t + idx * 0.07 + 0.85);
    });
  }

  // 4. Celebration Harp / Marimba Arpeggio
  playCelebration() {
    if (this.muted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;
    // Cheerful pentatonic ascending arpeggio (C5, D5, E5, G5, A5, C6)
    const arpeggio = [523.25, 587.33, 659.25, 783.99, 880.00, 1046.50];

    arpeggio.forEach((freq, i) => {
      const startTime = t + i * 0.065;
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();

      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, startTime);

      // Warm marimba mallet envelope
      gain.gain.setValueAtTime(0.001, startTime);
      gain.gain.linearRampToValueAtTime(0.28, startTime + 0.015);
      gain.gain.exponentialRampToValueAtTime(0.001, startTime + 0.5);

      osc.connect(gain);
      gain.connect(this.ctx.destination);

      osc.start(startTime);
      osc.stop(startTime + 0.55);
    });
  }

  // 5. Gentle Step / Switch Tap
  playTap() {
    if (this.muted) return;
    this.ensureContext();
    if (!this.ctx) return;

    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    osc.type = 'sine';
    osc.frequency.setValueAtTime(540, t);
    osc.frequency.exponentialRampToValueAtTime(260, t + 0.05);

    gain.gain.setValueAtTime(0.18, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + 0.05);

    osc.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start(t);
    osc.stop(t + 0.05);
  }
}

export const audio = new AudioManager();
