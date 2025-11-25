# Phase 1: The Authoring Core (In Progress)

## Goal

Create a stable, private authoring tool that synthesizes Twine (node-based) and Storyline (trigger-based) paradigms.

## Implemented Features (Frontend)

### 1. Node Editor Canvas (`frontend/src/components/authoring/node_canvas.rs`)

- **Feature**: A visual canvas that renders a list of nodes.
- **Interaction**: Supports dragging nodes around the canvas.
- **State**: Manages local state for node positions and dragging status.

### 2. Story Node Component (`frontend/src/components/authoring/story_node.rs`)

- **Feature**: A visual representation of a narrative passage.
- **UI**: Displays title, content preview, and input/output ports (visual only).
- **Styling**: Uses Tailwind CSS for a "cyberpunk/premium" look (slate/cyan theme).

### 3. Authoring Page (`frontend/src/pages/authoring.rs`)

- **Feature**: The main container for the authoring environment.
- **UI**: Includes a header with "Save" and "Settings" buttons, and a placeholder floating toolbar.
- **Routing**: Accessible via `/authoring`.

## Next Steps

1. **Backend Integration**: Create the `Expert Module` in the backend to save/load these nodes.
2. **Link Logic**: Implement the ability to draw connections (arrows) between nodes.
3. **Property Editor**: Create a side panel to edit the content of a selected node.
