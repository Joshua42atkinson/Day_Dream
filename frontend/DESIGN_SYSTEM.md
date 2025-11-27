# Boilermaker Industrial Design System

## Overview

The **Boilermaker Industrial 2.0** design system for **Ask Pete** (Purdue Expert Tuition Engine) implements a unique industrial aesthetic inspired by Purdue University's Boilermaker identity and early 20th-century locomotive engineering.

## Design Philosophy

- **Industrial Precision**: Clean, mechanical interfaces that evoke train yards, control panels, and industrial equipment
- **Chamfered Edges**: No rounded corners—all panels use 45-degree chamfered cuts for an authentic machined metal appearance
- **Purdue Branding**: Official Old Gold (#CFB991) as the primary accent color with black backgrounds
- **Functional Metaphors**: UI elements named after train yard components (Dispatcher, Weigh Station, Yard, etc.)

## Color Palette

### Primary Colors

- **Old Gold** (`#CFB991`): Primary accent, borders, highlights
- **Purdue Black** (`#000000`): Deep backgrounds
- **Steam White** (`#F0F0F0`): Primary text color

### Industrial Backgrounds

- **Coal Dark** (`#1A1A1A`): Main background
- **Iron Gray** (`#2D2D2D`): Panel backgrounds
- **Industrial Slate** (`#1B1B1E`): Alternative dark background
- **Industrial Surface** (`#2A2A2E`): Elevated surfaces

### Gold Variants

- **Purdue Prime** (`#FFD700`): Bright highlight gold
- **Aged Gold** (`#8E6F3E`): Darker gold for depth
- **Dust Gold** (`#EBD99F`): Lighter gold for highlights

### Signal Colors

- **Signal Red** (`#C8102E`): Errors, warnings, stop actions
- **Gauge Green** (`#4F7942`): Success states, completion
- **Purdue Dust** (`#a0a0a0`): Secondary text

## Typography

### Font Families

```css
--font-ui: 'Inter', system-ui, -apple-system, sans-serif;
--font-ai: 'JetBrains Mono', 'Roboto Mono', 'Consolas', monospace;
```

- **UI Font (Inter)**: Navigation, labels, buttons
- **AI Font (JetBrains Mono)**: Terminal-like displays, code, AI responses

## Custom Components

### Chamfered Corners

```html
<div class="chamfered-corners">...</div>
```

Creates 8px 45-degree cut corners on all four corners using clip-path.

### Mechanical Buttons

```html
<button class="mechanical-button-primary">Primary Action</button>
<button class="mechanical-button-secondary">Secondary Action</button>
<button class="mechanical-button-danger">Delete</button>
<button class="mechanical-button-success">Confirm</button>
```

Features:

- Chamfered corners
- Gold glow on hover
- Active press animation (scale-95)
- Uppercase text with wide tracking

### Pressure Gauge (Progress Bar)

```html
<div class="pressure-gauge">
  <div class="pressure-gauge-fill" style="width: 75%"></div>
</div>
```

Industrial-themed progress indicator with:

- Chamfered container
- Gold gradient fill
- Smooth 500ms transitions

### Glass Panel

```html
<div class="glass-panel">...</div>
```

Glassmorphism effect with:

- 85% opacity dark background
- 12px backdrop blur
- Gold border with 30% opacity
- Subtle shadow

### Metallic Gold Text

```html
<h1 class="text-metallic-gold">Ask Pete</h1>
```

Gradient text effect simulating brushed gold metal.

### Blueprint Grid Background

```html
<div class="blueprint-grid">...</div>
```

Subtle grid pattern for technical/engineering aesthetic.

## Animations

### Available Animations

- `animate-fade-in`: Fade in with slight upward movement
- `animate-slide-in-right`: Slide in from right
- `animate-slide-in-left`: Slide in from left
- `animate-pulse-gold`: Pulsing opacity for gold elements
- `animate-glow`: Pulsing glow effect
- `animate-spin`: Standard rotation

## Layout System

### Dispatcher Grid

The main application uses a specialized grid layout:

```
┌─────────────────────────────────┐
│         Header (50px)           │
├───┬─────────────────────┬───────┤
│ N │                     │       │
│ a │      Main           │ Props │
│ v │      Content        │ Panel │
│   │                     │       │
│ 6 │                     │ 300px │
│ 4 │                     │       │
│ p │                     │       │
│ x │                     │       │
└───┴─────────────────────┴───────┘
```

## Usage Examples

### Creating a Panel

```rust
use crate::components::boilermaker::ChamferedPanel;

view! {
    <ChamferedPanel class="bg-industrial-surface p-6">
        <h2 class="text-metallic-gold text-2xl mb-4">"Section Title"</h2>
        <p class="text-steam-white">"Content goes here"</p>
    </ChamferedPanel>
}
```

### Creating a Button

```rust
use crate::components::boilermaker::MechanicalButton;

view! {
    <MechanicalButton 
        primary=true
        on_click=Callback::new(|_| { /* action */ })
    >
        "Start Engine"
    </MechanicalButton>
}
```

### Creating a Progress Indicator

```rust
use crate::components::boilermaker::PressureGauge;

let progress = create_signal(0.75);

view! {
    <PressureGauge 
        value=progress
        label="Cognitive Load".to_string()
    />
}
```

## Best Practices

1. **Consistent Metaphors**: Use train yard terminology (Yard, Weigh Station, Dispatcher, etc.)
2. **No Rounded Corners**: Always use chamfered corners for panels and buttons
3. **Gold Accents**: Use Old Gold sparingly for emphasis and interactivity
4. **Monospace for Data**: Use JetBrains Mono for technical readouts, code, and AI responses
5. **Dark First**: Design for dark mode—it's the only mode
6. **Subtle Animations**: Keep transitions smooth but not distracting (200-500ms)

## Accessibility

- **Contrast Ratios**: All text meets WCAG AA standards against backgrounds
- **Focus States**: All interactive elements have visible focus indicators
- **Semantic HTML**: Proper heading hierarchy and ARIA labels
- **Keyboard Navigation**: Full keyboard support for all interactions

## File Structure

```
frontend/
├── style/
│   ├── input.css          # Source CSS with @tailwind directives
│   └── output.css         # Generated Tailwind CSS
├── tailwind.config.js     # Tailwind configuration
└── src/
    ├── ui_theme.rs        # Rust theme constants and helpers
    └── components/
        └── boilermaker.rs # Reusable UI components
```

## Version

**Boilermaker Industrial 2.0** - November 2025

---

*"Forged in the Foundry, Tempered by Pete"*
