class AlgoViz extends HTMLElement {
  connectedCallback() {
    if (this.dataset.initialized === "true") return;

    const source = this.querySelector('script[type="application/json"]');
    if (!source) {
      this.textContent = "动画数据尚未加载；请使用下方静态步骤表。";
      return;
    }

    try {
      this.trace = JSON.parse(source.textContent);
    } catch (error) {
      this.textContent = `动画数据无效：${error.message}`;
      return;
    }

    this.dataset.initialized = "true";
    this.cursor = 0;
    this.timer = null;
    this.attachShadow({ mode: "open" });
    this.renderShell();
    this.renderFrame();
  }

  disconnectedCallback() {
    this.stop();
  }

  renderShell() {
    this.shadowRoot.innerHTML = `
      <style>
        :host { display: block; margin: 1.4rem 0; font: inherit; }
        .panel { padding: 1rem; border: 1px solid #7c8aa5; border-radius: 14px;
          background: linear-gradient(145deg, rgba(75, 100, 160, .13), rgba(52, 211, 153, .08)); }
        .heading { display: flex; justify-content: space-between; gap: 1rem; align-items: baseline; }
        .heading strong { font-size: 1rem; }
        .metrics { display: flex; gap: .6rem; flex-wrap: wrap; margin: .8rem 0; }
        .metric { padding: .25rem .6rem; border-radius: 999px; background: rgba(100, 116, 139, .18); }
        .chars { display: flex; gap: .45rem; flex-wrap: wrap; padding: 1rem 0; }
        .char { display: grid; place-items: center; width: 2.6rem; height: 2.6rem; border-radius: .65rem;
          border: 1px solid #7c8aa5; position: relative; transition: transform .2s, background .2s; }
        .char small { position: absolute; top: -1.05rem; font-size: .65rem; opacity: .65; }
        .char.window { background: rgba(52, 211, 153, .28); border-color: #10b981; }
        .char.active { transform: translateY(-.25rem); box-shadow: 0 .3rem .8rem rgba(59, 130, 246, .32); }
        .controls { display: flex; gap: .5rem; flex-wrap: wrap; align-items: center; }
        button { cursor: pointer; border: 1px solid #7c8aa5; border-radius: .45rem; padding: .35rem .7rem;
          color: inherit; background: rgba(100, 116, 139, .12); }
        button:disabled { cursor: not-allowed; opacity: .45; }
        .explanation { min-height: 2.8rem; margin-top: .8rem; padding: .7rem;
          border-left: 3px solid #3b82f6; background: rgba(59, 130, 246, .08); }
        .counter { opacity: .7; font-size: .85rem; }
      </style>
      <section class="panel" aria-label="LC 3 滑动窗口动画">
        <div class="heading"><strong>last-seen HashMap · abba</strong><span class="counter"></span></div>
        <div class="metrics"></div>
        <div class="chars"></div>
        <div class="controls">
          <button data-action="reset" aria-label="回到开始">重置</button>
          <button data-action="previous" aria-label="上一步">上一步</button>
          <button data-action="play" aria-label="播放或暂停">播放</button>
          <button data-action="next" aria-label="下一步">下一步</button>
          <label>速度 <input data-speed type="range" min="250" max="1400" step="50" value="700"></label>
        </div>
        <div class="explanation" role="status" aria-live="polite"></div>
      </section>`;

    this.shadowRoot.querySelectorAll("button[data-action]").forEach((button) => {
      button.addEventListener("click", () => this[button.dataset.action]());
    });
  }

  stateAtCursor() {
    const state = { left: 0, right: -1, best: 0, active: -1 };
    for (const event of this.trace.events.slice(0, this.cursor)) {
      if (event.type === "visit") state.active = event.payload.index;
      if (event.type === "window_shrink") state.left = event.payload.to_left;
      if (event.type === "window_expand") {
        state.left = event.payload.left;
        state.right = event.payload.right;
      }
      if (event.type === "best_update") state.best = event.payload.best;
    }
    return state;
  }

  renderFrame() {
    const state = this.stateAtCursor();
    const characters = [...this.trace.initial_state.input];
    const event = this.cursor === 0 ? null : this.trace.events[this.cursor - 1];

    this.shadowRoot.querySelector(".metrics").innerHTML = [
      ["left", state.left], ["right", state.right < 0 ? "—" : state.right], ["best", state.best]
    ].map(([label, value]) => `<span class="metric">${label}: <strong>${value}</strong></span>`).join("");

    this.shadowRoot.querySelector(".chars").innerHTML = characters.map((character, index) => {
      const inWindow = index >= state.left && index <= state.right ? " window" : "";
      const active = index === state.active ? " active" : "";
      return `<span class="char${inWindow}${active}"><small>${index}</small>${this.escape(character)}</span>`;
    }).join("");

    this.shadowRoot.querySelector(".counter").textContent = `${this.cursor} / ${this.trace.events.length}`;
    this.shadowRoot.querySelector(".explanation").textContent = event
      ? event.explanation
      : "初始状态：窗口为空，left = 0，best = 0。";
    this.shadowRoot.querySelector('[data-action="previous"]').disabled = this.cursor === 0;
    this.shadowRoot.querySelector('[data-action="next"]').disabled = this.cursor === this.trace.events.length;
  }

  next() {
    if (this.cursor < this.trace.events.length) this.cursor += 1;
    this.renderFrame();
    if (this.cursor === this.trace.events.length) this.stop();
  }

  previous() {
    this.stop();
    if (this.cursor > 0) this.cursor -= 1;
    this.renderFrame();
  }

  reset() {
    this.stop();
    this.cursor = 0;
    this.renderFrame();
  }

  play() {
    if (this.timer) {
      this.stop();
      return;
    }
    if (this.cursor === this.trace.events.length) this.cursor = 0;
    this.shadowRoot.querySelector('[data-action="play"]').textContent = "暂停";
    const tick = () => {
      this.next();
      if (this.cursor < this.trace.events.length) {
        const speed = Number(this.shadowRoot.querySelector("[data-speed]").value);
        this.timer = window.setTimeout(tick, speed);
      }
    };
    tick();
  }

  stop() {
    if (this.timer) window.clearTimeout(this.timer);
    this.timer = null;
    const button = this.shadowRoot?.querySelector('[data-action="play"]');
    if (button) button.textContent = "播放";
  }

  escape(value) {
    const span = document.createElement("span");
    span.textContent = value;
    return span.innerHTML;
  }
}

customElements.define("algo-viz", AlgoViz);

