class AlgoLab extends HTMLElement {
  connectedCallback() {
    if (this.dataset.initialized === "true") return;

    const problem = this.getAttribute("problem") || "unknown";
    const solution = this.getAttribute("solution") || "unknown";
    const status = document.createElement("div");
    status.className = "algo-lab__status";
    status.textContent = `Algo Lab placeholder · problem=${problem} · solution=${solution}`;
    this.appendChild(status);
    this.dataset.initialized = "true";
  }
}

customElements.define("algo-lab", AlgoLab);

