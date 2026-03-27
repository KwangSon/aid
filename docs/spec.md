# AID v1 – Minimal 2D CAD + Viewer Spec

## Goal
Build a minimal 2D CAD engine with a basic viewport.
User can draw, select, and export entities.

---

## Scope (v1)

### Must Have
- Line entity
- Scene storage
- Command system
- DXF export
- Viewport (render lines)
- Mouse input
- Selection (hit test)

### Not Included
- 3D
- BIM
- Advanced UI (panels, menus)
- Constraints
- Parametric modeling

---

## Architecture

```text
Core (geometry + command)
   ↓
Viewer (wgpu render)
   ↓
Window/Input (winit)