# swaygrouptool

This is a cli tool to control workspace hotkey behavior for the sway window
manager.

It allows you to organize your containers within 2 "workspace-groups". Those
groups are represented by the numbers 11-19 and 21-29 in your sway bar.

For example if you work on 2 different projects you may dedicate each project
to it's own workspace group.

Actions that can be performed with swaygrouptool:

- `next` or `prev`: Replaces the `workspace next` and `workspace prev` commands
and prevents navigating to a workspace which is not in the current group.

- `cycle`: Replaces `workspace back_and_forth`. Allows you to cycle back to the
last used workspace within the current group. If you are in a non-group
workspace it will send you back to the last focused workspace within the last
focused group.

Example keybindings for sway configuration `~/.config/sway/config`:

```
# Cycle to previously focused workspace.
# If the current workspace is not a valid group workspace, switch
# back to the previously used workspace group.
bindsym $mod+Tab exec swaygrouptool cycle

# Switch to next/prev workspace.
bindsym $mod+Page_Up exec swaygrouptool prev
bindsym $mod+Page_Down exec swaygrouptool next

# Toggle between workspace group 1 and 2.
bindsym $mod+0 exec swaygrouptool toggle-group

# Move current container to next workspace group.
bindsym $mod+Shift+0 exec swaygrouptool move

# Switch to workspace offset.
bindsym $mod+1 exec swaygrouptool goto 1
bindsym $mod+2 exec swaygrouptool goto 2
bindsym $mod+3 exec swaygrouptool goto 3
bindsym $mod+4 exec swaygrouptool goto 4
bindsym $mod+5 exec swaygrouptool goto 5
bindsym $mod+6 exec swaygrouptool goto 6
bindsym $mod+7 exec swaygrouptool goto 7
bindsym $mod+8 exec swaygrouptool goto 8
bindsym $mod+9 exec swaygrouptool goto 9

# Switch to additional ungrouped workspaces.
bindsym $mod+bracketleft workspace number 90 ; exec swaygrouptool save
bindsym $mod+bracketright workspace number 91 ; exec swaygrouptool save
bindsym $mod+Backslash workspace number 99 ; exec swaygrouptool save

# Move current container to workspace offset.
bindsym $mod+Shift+1 exec swaygrouptool move 1
bindsym $mod+Shift+2 exec swaygrouptool move 2
bindsym $mod+Shift+3 exec swaygrouptool move 3
bindsym $mod+Shift+4 exec swaygrouptool move 4
bindsym $mod+Shift+5 exec swaygrouptool move 5
bindsym $mod+Shift+6 exec swaygrouptool move 6
bindsym $mod+Shift+7 exec swaygrouptool move 7
bindsym $mod+Shift+8 exec swaygrouptool move 8
bindsym $mod+Shift+9 exec swaygrouptool move 9

# Move current container to ungrouped workspace.
bindsym $mod+Shift+bracketleft move container to workspace number 90 ; exec swaygrouptool save
bindsym $mod+Shift+bracketright move container to workspace number 91 ; exec swaygrouptool save
bindsym $mod+Shift+Backslash move container to workspace number 99 ; exec swaygrouptool save
```

I've written this tool for personal use and I'm not planning on maintaining it
in any way. But feel free to fork and adapt.
