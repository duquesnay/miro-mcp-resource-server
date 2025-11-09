# Miro MCP Server - Demo Guide

This guide demonstrates all capabilities of the Miro MCP Server through example prompts you can use with Claude Desktop or Claude Code.

## Prerequisites

- Miro MCP Server configured (see [SETUP.md](SETUP.md))
- Authenticated with Miro (completed `start_auth`)
- Claude Desktop or Claude Code running

## Demo Scenario: Agile Team Visualization

This demo creates a complete agile team structure visualization showcasing all MCP tools.

### Step 1: Create Board

**Prompt:**
```
Create a new Miro board called "Agile Team Structure Demo"
```

**Claude will:**
- Use `create_board` tool
- Return board ID for subsequent operations

**Expected response:**
```
Successfully created board: Agile Team Structure Demo
Board ID: uXjVN8TqR4A=
```

### Step 2: Create Team Structure with Frames

**Prompt:**
```
On the board "Agile Team Structure Demo", create 3 frames:
1. Frame titled "Squad Alpha" at position (0, 0), size 1200x1000, light blue fill
2. Frame titled "Squad Beta" at position (1400, 0), size 1200x1000, light green fill
3. Frame titled "Squad Gamma" at position (2800, 0), size 1200x1000, light yellow fill
```

**Claude will:**
- Use `create_frame` tool 3 times
- Position frames horizontally with spacing

**Expected response:**
```
Created 3 frames:
- Squad Alpha (ID: 3458764...001) at (0, 0)
- Squad Beta (ID: 3458764...002) at (1400, 0)
- Squad Gamma (ID: 3458764...003) at (2800, 0)
```

### Step 3: Add Team Members with Sticky Notes

**Prompt:**
```
In Squad Alpha frame, add team members using sticky notes:
- 1 yellow sticky note "Product Owner - Alice" at position (200, 200)
- 1 green sticky note "Scrum Master - Bob" at position (500, 200)
- 3 blue sticky notes for developers at positions (200, 500), (500, 500), (800, 500):
  * "Developer - Charlie"
  * "Developer - Diana"
  * "Developer - Eve"
```

**Claude will:**
- Use `create_sticky_note` tool 5 times
- Apply color coding (yellow=PO, green=SM, blue=Devs)
- Position in hierarchical layout

**Expected response:**
```
Created 5 sticky notes in Squad Alpha:
- Product Owner - Alice (yellow) at (200, 200)
- Scrum Master - Bob (green) at (500, 200)
- Developer - Charlie (blue) at (200, 500)
- Developer - Diana (blue) at (500, 500)
- Developer - Eve (blue) at (800, 500)
```

### Step 4: Add Shapes for Roles

**Prompt:**
```
Add role indicators using shapes:
- Circle shape with "Leadership" text at position (200, 100), diameter 150, light gray fill
- Rectangle shape with "Execution" text at position (500, 400), size 800x150, light orange fill
```

**Claude will:**
- Use `create_shape` tool 2 times
- Create different shape types with text content

**Expected response:**
```
Created 2 shapes:
- Leadership circle (ID: 3458764...010) at (200, 100)
- Execution rectangle (ID: 3458764...011) at (500, 400)
```

### Step 5: Add Connectors for Reporting Lines

**Prompt:**
```
Create reporting lines showing team structure:
- Connect Developer Charlie to Scrum Master Bob with blue arrow
- Connect Developer Diana to Scrum Master Bob with blue arrow
- Connect Developer Eve to Scrum Master Bob with blue arrow
- Connect Scrum Master Bob to Product Owner Alice with green arrow and caption "reports to"
```

**Claude will:**
- Use `create_connector` tool 4 times
- Apply different arrow styles and colors
- Add caption to one connector

**Expected response:**
```
Created 4 connectors:
- Charlie → Bob (blue arrow)
- Diana → Bob (blue arrow)
- Eve → Bob (blue arrow)
- Bob → Alice (green arrow with "reports to" caption)
```

### Step 6: Add Text Annotations

**Prompt:**
```
Add text annotation at position (100, -100): "Demo Board - Created with Miro MCP Server"
```

**Claude will:**
- Use `create_text` tool
- Add standalone text element

**Expected response:**
```
Created text annotation at (100, -100)
Content: "Demo Board - Created with Miro MCP Server"
```

### Step 7: Bulk Operations Demo

**Prompt:**
```
Use bulk operations to add 5 task sticky notes to Squad Beta at once:
- "Sprint Planning" at (1600, 300)
- "Daily Standup" at (1800, 300)
- "Sprint Review" at (2000, 300)
- "Sprint Retro" at (2200, 300)
- "Backlog Refinement" at (1900, 500)
All in pink color
```

**Claude will:**
- Use `bulk_create_items` tool
- Create 5 sticky notes in single API call
- Demonstrate performance benefit (5 items in 1 request vs 5 requests)

**Expected response:**
```
Bulk created 5 sticky notes in Squad Beta:
- Sprint Planning (ID: 3458764...020)
- Daily Standup (ID: 3458764...021)
- Sprint Review (ID: 3458764...022)
- Sprint Retro (ID: 3458764...023)
- Backlog Refinement (ID: 3458764...024)

Performance: 1 API call instead of 5 (80% reduction)
```

### Step 8: List Items Demo

**Prompt:**
```
List all sticky notes on the board
```

**Claude will:**
- Use `list_items` tool with type filter `sticky_note`
- Return metadata for all sticky notes

**Expected response:**
```
Found 10 sticky notes:
1. Product Owner - Alice (yellow) at (200, 200)
2. Scrum Master - Bob (green) at (500, 200)
3. Developer - Charlie (blue) at (200, 500)
4. Developer - Diana (blue) at (500, 500)
5. Developer - Eve (blue) at (800, 500)
6. Sprint Planning (pink) at (1600, 300)
7. Daily Standup (pink) at (1800, 300)
8. Sprint Review (pink) at (2000, 300)
9. Sprint Retro (pink) at (2200, 300)
10. Backlog Refinement (pink) at (1900, 500)
```

### Step 9: Update Item Demo

**Prompt:**
```
Move the "Sprint Planning" sticky note to position (1600, 200) and change its content to "Sprint Planning (2h)"
```

**Claude will:**
- Use `update_item` tool
- Modify position and content in single request

**Expected response:**
```
Updated sticky note "Sprint Planning":
- New position: (1600, 200)
- New content: "Sprint Planning (2h)"
```

### Step 10: Delete Item Demo

**Prompt:**
```
Delete the "Leadership" circle shape
```

**Claude will:**
- Use `delete_item` tool
- Remove specific item by ID

**Expected response:**
```
Successfully deleted shape "Leadership" (ID: 3458764...010)
```

## Final Board Structure

After completing all steps, the demo board contains:

**Squad Alpha Frame:**
- 1 Product Owner (yellow sticky note)
- 1 Scrum Master (green sticky note)
- 3 Developers (blue sticky notes)
- 1 Execution rectangle shape
- 4 connectors showing reporting structure

**Squad Beta Frame:**
- 5 Agile ceremony tasks (pink sticky notes)

**Squad Gamma Frame:**
- Empty (ready for user experimentation)

**Board-level elements:**
- 1 text annotation (title)

## All MCP Tools Demonstrated

| Tool | Demo Step | Capability |
|------|-----------|------------|
| `create_board` | Step 1 | Create new board |
| `create_frame` | Step 2 | Group elements with frames |
| `create_sticky_note` | Step 3 | Add notes with color coding |
| `create_shape` | Step 4 | Add geometric shapes with text |
| `create_connector` | Step 5 | Connect items with arrows and captions |
| `create_text` | Step 6 | Add standalone text annotations |
| `bulk_create_items` | Step 7 | Create multiple items efficiently |
| `list_items` | Step 8 | Query board contents |
| `update_item` | Step 9 | Modify existing items |
| `delete_item` | Step 10 | Remove items |

## Performance Highlights

- **Total items created**: 23 items
- **API calls with individual operations**: 23 calls
- **API calls with bulk operations**: 18 calls (22% reduction)
- **Total execution time**: ~15-20 seconds

## Quick Demo Prompts

If you want a faster demo, use this all-in-one prompt:

```
Create a Miro board called "Quick Demo". Add a frame titled "Team" at (0,0) size 1000x800. Inside the frame, add 3 sticky notes: yellow "Product Owner" at (200, 200), green "Scrum Master" at (500, 200), and blue "Developer" at (350, 400). Connect the Developer to Scrum Master with an arrow, and Scrum Master to Product Owner with an arrow labeled "reports to".
```

Claude will orchestrate 7 MCP tool calls automatically:
1. Create board
2. Create frame
3-5. Create 3 sticky notes
6-7. Create 2 connectors (one with caption)

Total time: ~8-10 seconds

## Experimentation Ideas

### Try these prompts to explore capabilities:

**Cross-squad dependencies:**
```
Create connectors between Squad Alpha and Squad Beta showing dependencies:
- From Alice (Squad Alpha PO) to Squad Beta frame with dashed line and caption "Shared Roadmap"
- From Bob (Squad Alpha SM) to Squad Beta frame with dotted line and caption "Collaboration"
```

**Visual hierarchy:**
```
Create a shape hierarchy showing organizational levels:
- Large rectangle "Engineering" at (0, -500) size 4000x300 in gray
- 3 medium rectangles inside for each squad
- Connectors showing hierarchy
```

**Dynamic updates:**
```
List all sticky notes, then move all blue developer notes 100 pixels down to create more spacing
```

**Cleanup:**
```
Delete all connectors from the board
```

## Troubleshooting Demo Issues

### "Board not found"
Make sure you use the correct board ID returned in Step 1, or use board name in quotes.

### "Item not found"
List items first to get current item IDs: `list_items` returns IDs for update/delete operations.

### "Rate limit exceeded"
Miro API limit: 100 requests/minute. Wait 60 seconds and retry. Use bulk operations to reduce API calls.

### "Position overlap"
Items might overlap visually. Adjust x/y coordinates to spread items out. Each frame is 1200x1000, leaving 200px spacing between frames.

## Next Steps

- Experiment with different layouts and structures
- Try creating your own team structure
- Explore color combinations and visual styles
- Test bulk operations with larger datasets (up to 20 items)
- Integrate into your workflow for quick team visualizations

## Feedback

Found a bug or have a feature request?
- GitHub Issues: https://github.com/duquesnay/miro-rust-remote-mcp/issues
