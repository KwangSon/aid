# AID (AI-Integrated Design) – Core 2D CAD Spec

## Goal
Build a minimal 2D CAD engine in Rust.
All features must be testable via DXF export.

---

## Core Principles
- Command-driven architecture
- Pure geometry (no side effects)
- Simple data structures
- DXF as validation output

---

## 1. Geometry

### Types
- Point2 (x, y)
- Line (start, end)
- Circle (center, radius)

### Requirements
- Immutable
- Pure functions only

---

## 2. Entity System

### Entity
- Line
- Circle

### EntityId
- Unique identifier

---

## 3. Scene

### Structure
- Store all entities

### Requirements
- No logic
- Simple container

---

## 4. Command System

### Commands
- CreateLine
- CreateCircle
- MoveEntity
- DeleteEntity

### Requirements
- All changes via commands
- Deterministic execution

---

## 5. DXF Export

### Goal
- Export Scene → DXF file

### Requirements
- Valid DXF format
- Compatible with AutoCAD

---

## 6. Testing

### Strategy
- Execute command
- Export DXF
- Verify output

### First Test
- Create one line
- Export to DXF
- Open in AutoCAD

---

## Done Criteria (v0)

- Create Line
- Store in Scene
- Export DXF
- Open successfully in AutoCAD