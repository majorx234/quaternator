# Info
- visualize quaternions
- 3D OpenGL in Rust
- WIP
- nothing useful yet

# Examples
## Example1:
- use nalgebra_glm to doing matrixmultiplication
- testing the glm interface of nalgebra
- found out glm is not needed it is just a wrapper on nalgebra with other syntax

## Example2:
- testing winit
- tryout event loop concept
- not realy working

## Example3:
- testing `egui_winit`
- and `glutin` as renderbackend
- not that nice:
  - uses unsafe code
  - hidden complexity with `glutin`
    - crate glutin_winit doing evenloop

## example4:
- using `eframe` & `egui` for window management
- for 3D drawing it uses `glow`
  - similar to `egui` painter
  - but now: glow manage the rendering loop
- relatively ugly code (unsafe, some arc/callback magic)

## example5:
- `egui` and `glow` again try a simplified eventloop
- no 3D rendering code at all

## example6:
- `egui` with `winit` (instead of `eframe`)
- uses `glium` as for rendering a texture
- looks promising
- no unsafe, or complex callback code

## example7:
- uses `winit` for window handling
- and `glium` to render some kind of cube
- looks promising
- some problems in order of rendered triangles

## example8:
- similar like example7 (`winit` + `glium`)
- play arround with events in eventloop

## example9:
- similar like example7 (`winit` + `glium`)
- fixed problem with incorrect order of rendering triangles
  - fixed by adding in params: `glium::DrawParameters` a `glium::Depth` object
- added rotation by mouse interaction

## example10:
- similar like example9 (`winit` + `glium`)
- exchanged cube with a teapot
  - to use normals in rendering

## example11
- similar like example10 (`winit` + `glium`)
- load teapot model by file

## example12
- similar like example11 (`winit` + `glium`)
- uses `egui` for control elements
- unfortunately throw `segmentation fault (core dumped)` on close
