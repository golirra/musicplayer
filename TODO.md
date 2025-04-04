# Project To-Do / Planning

## Features
- [ ] rym (rateyourmusic) api
- [ ] Rest of playback_bar (next, prev, randnext)
- [ ] Cache/save file list to persist across sessions (sql?)
- [ ] Implement playlist functionality
- [ ] Button to "jump" to the current playing song in a playlist tab. In foobar,
  when the playlist gets too big, I want to be able to focus back to the
selected file, otherwise I have to scroll over a possibly thousand song playlist
to find the currently playing track.
- [ ] Make very easy to use "add to playback queue" functionality to be able to
  queue songs with the most minimal effort required


## Bug Fixes
- [x] Song duration doesn't work
- [ ] There is going to be a bug in the future regarding grabbing filepaths and
  the way they're associated with artist metadata. Because of the way I'm looping over things in a directory to get filepaths, I need to be careful to associate the right metadata with the particular path.

## Improvements
- [ ] Make layout not suck
- [ ] Draggable button should probably be named something better than "button"
to make it more obvious its a custom button separate from iced::widget::button
### Draggable button:
- Current draggable button functionality is like this:
    1. on_left_click: start dragging
    2. on_mouse_move: move button with mouse
    3. on_left_release: call button event if it exists, stop dragging

- To make "drag album from library pane to playlist pane" we need:
    1. button press events that involve audio playback are only allowed to fire
       if the button is in the playback pane
    2. button press events in the library pane can be:
        - right click to see information
        - double left click to expand a list
        - left click + drag to drag over to playlist pane





## Testing
- [ ] Write unit tests for the audio playback module

## Future Enhancements
- [ ] Add cross-platform support (Windows, macOS, Linux)

## Ideas
- [ ] equalizer, visualizer?
- [ ] themes
- [ ] dsp? (maybe too far out of scope of original idea)
- [ ] Something I frequently do in foobar and would like to stop: when I add a
set of songs to the playlist screen, eventually the playlist grows so big that I
have that exact album on the playlist already just farther up/down. So is it
possible to implement some sort of "if album already in playlist, jump to album
in playlist view instead of inserting"


_Last updated: 2025-03-14_
