# cubensis (WIP)

A realtime music visualizer with embedded scene/shader development features [WIP] 

Implemented in Rust, thanks to WGPU, egui, cpal, and more!

Currently features:

- const generic binding group configuration
- a variety of built-in resources, include player statistics and power spectrum/waveform textures/samplers for audio input
- configurable audio input, including options for WASAPI loopback on Windows
  - stream is automatically rebuilt on device availability changes
- compile-time configurable gbuffer array for previously rendered frames (iterative shading and warping)
  - const generics control the number of history layers to preserve
- A rudimentary scene and mesh definition system (in development)
- shader/scene hot-reload with script validation
  - selective render pipeline regeneration to prevent unneccessary work on the GPU
- A toggleable UI for statistics
- Single-threaded, interactive websocket RPC interface
- (broken) VS Code plugin for controlling playback and configuring development features
- A multi-threaded plugin system (i.e. RPC, file watchers, etc.)
- And scaffolding for the easy creation of plugins and resource groups

Soon planned features:

- better scene/mesh definition schemas/logic
- configuration of audio through the UI
- UI components for easing shader development
  - customized calculators, uniform variable views, CPU shader debugging
- cameras
- better composite rendering support
- implementation of compute shaders for vertice generation/manipulation
- implementation of resource group compute shaders
