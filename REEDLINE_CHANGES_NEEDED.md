# Required Reedline Changes for `close_on_empty` Menu Option

## Overview
This document describes the changes needed in the reedline library to support the new `close_on_empty` menu configuration option in nushell.

## Problem
Currently, when a menu is active and the user deletes all characters from the buffer (making it empty), the menu automatically closes. This behavior is hardcoded in reedline's `engine.rs` and cannot be configured.

## Solution
Add a new `close_on_empty` field to `MenuSettings` that controls whether a menu should close when the buffer becomes empty.

## Required Changes in Reedline

### 1. Update `MenuSettings` struct in `src/menu/mod.rs`

Add the new field to the `MenuSettings` struct:

```rust
pub struct MenuSettings {
    /// Menu name
    name: String,
    /// Menu coloring
    color: MenuTextStyle,
    /// Menu marker when active
    marker: String,
    /// Calls the completer using only the line buffer difference
    /// after the menu was activated
    only_buffer_difference: bool,
    /// Whether to close the menu when the buffer becomes empty
    /// Default: true (menu closes when buffer is empty)
    close_on_empty: bool,
}
```

Update the `Default` implementation:

```rust
impl Default for MenuSettings {
    fn default() -> Self {
        Self {
            name: "menu".to_string(),
            color: MenuTextStyle::default(),
            marker: "| ".to_string(),
            only_buffer_difference: false,
            close_on_empty: true,  // Default to true to maintain current behavior
        }
    }
}
```

Add the builder method:

```rust
impl MenuSettings {
    // ... existing methods ...

    /// MenuSettings builder with close_on_empty
    #[must_use]
    pub fn with_close_on_empty(mut self, close_on_empty: bool) -> Self {
        self.close_on_empty = close_on_empty;
        self
    }
}
```

Add the trait method to `MenuBuilder`:

```rust
pub trait MenuBuilder: Menu + Sized {
    // ... existing methods ...

    /// Menu builder with new value for close_on_empty
    #[must_use]
    fn with_close_on_empty(mut self, close_on_empty: bool) -> Self {
        self.settings_mut().close_on_empty = close_on_empty;
        self
    }
}
```

### 2. Update menu deactivation logic in `src/engine.rs`

Find the code around line 1213 that currently reads:

```rust
if self.editor.line_buffer().get_buffer().is_empty() {
    menu.menu_event(MenuEvent::Deactivate);
} else {
    menu.menu_event(MenuEvent::Edit(self.quick_completions));
}
```

Replace it with:

```rust
if self.editor.line_buffer().get_buffer().is_empty() {
    if menu.settings().close_on_empty {
        menu.menu_event(MenuEvent::Deactivate);
    } else {
        menu.menu_event(MenuEvent::Edit(self.quick_completions));
    }
} else {
    menu.menu_event(MenuEvent::Edit(self.quick_completions));
}
```

## Testing

After implementing these changes:

1. Build nushell with the updated reedline
2. Test with a config that sets `close_on_empty: false` on a menu
3. Open the menu, type some characters, then delete them all
4. Verify the menu stays open when `close_on_empty: false`
5. Verify the menu closes when `close_on_empty: true` (default behavior)

## Example nushell configuration

```nushell
$env.config.menus = [{
    name: completion_menu
    only_buffer_difference: false
    marker: "| "
    close_on_empty: false  # Keep menu open even when buffer is empty
    type: {
        layout: columnar
        columns: 4
        col_width: 20
        col_padding: 2
    }
    style: {
        text: green
        selected_text: green_reverse
        description_text: yellow
    }
}]
```

## Notes

- The default value of `true` maintains backward compatibility
- This feature is particularly useful for history and help menus where users might want to browse even after clearing their input
- The option name `close_on_empty` was chosen to be clear and self-documenting
