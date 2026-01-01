# Aeon_Project-
A serverless P2P network that uses physical motion entropy and SPRT algorithms to verify human identity and prevent bot attacks. Written in Rust &amp; Python.
# ⚛️ Aeon: Physical Proof-of-Personhood

**Turning gravity into digital identity.**

Aeon is an experimental, serverless Peer-to-Peer (P2P) network designed to solve the "Sybil Attack" problem without compromising privacy. Instead of using centralized servers, phone numbers, or invasive biometrics, Aeon utilizes **Motion Entropy**.

By analyzing the chaotic micro-movements of a user's device (accelerometer data), Aeon distinguishes real humans from bots and emulators using physics and math.

## 🚀 How It Works
* **[span_1](start_span)[span_2](start_span)Proof of Physical Work:** The core utilizes Shannon entropy and clustering algorithms to validate that the device is held by a human[span_1](end_span)[span_2](end_span).
* **[span_3](start_span)[span_4](start_span)Decentralized Reputation:** Peers use the Sequential Probability Ratio Test (SPRT) to statistically judge and slash malicious nodes without a central admin[span_3](end_span)[span_4](end_span).
* **[span_5](start_span)[span_6](start_span)Smart Routing:** Traffic is managed via a PID controller for stability and jitter reduction[span_5](end_span)[span_6](end_span).
* **[span_7](start_span)[span_8](start_span)Polyglot Architecture:** High-performance Rust core (using `libp2p` & `tokio`) bridged to a Python/Web interface[span_7](end_span)[span_8](end_span).

## ⚠️ State of the Project
**This is a Proof of Concept (PoC).**
The mathematical models (Core) are functional, but the project needs the open-source community to evolve into a production-ready application.
* **[span_9](start_span)Current Security:** Uses hardcoded seeds for testing (needs randomization)[span_9](end_span).
* **[span_10](start_span)Storage:** Currently runs on in-memory storage (needs database integration)[span_10](end_span).

**We need YOU to make it awesome.** Check the issues and submit a PR!
