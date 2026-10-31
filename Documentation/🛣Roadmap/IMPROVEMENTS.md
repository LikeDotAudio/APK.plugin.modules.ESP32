# 🛣 Improvements — DATA.ESP32 (from the 2026-10-30 architecture audit)

What this repo should change to meet the standard, most valuable first. Source: [🔍Audits/ARCHITECTURE-AUDIT.md](../%F0%9F%94%8DAudits/ARCHITECTURE-AUDIT.md).
← [Roadmap](INDEX.md)

| # | Improvement | Category | Effort (S/M/L) | Evidence |
|---|---|---|---|---|
| 1 | Apply the staged standalone form (`fix/P06-data-standalone.patch`: no `workspace =`, `edition = "2021"`) together with the engine's `SRC/members/DATA.ESP32` link + `[patch."…APK.plugin.modules.ESP32"] apkaudio-ocabin` — Waiting on DISCOVERY.Engine (handoff filed). Then push and tag (needs owner). | Coupling & Cohesion / Change management | S | `SRC/Cargo.toml` `workspace =` |

Everything else from the audit is delivered — see [🌍Publishing/DELIVERY.md](../%F0%9F%8C%8DPublishing/DELIVERY.md).
