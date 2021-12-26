# cubensis (WIP)

A realtime music visualizer with embedded scene/shader development features [WIP] 

Implemented in Rust, thanks to WGPU, egui, cpal, and more!

Current features:

- a variety of built-in resources, include player statistics and power spectrum/waveform textures/samplers for audio input
- Configurable audio input, including options for WASAPI loopback on Windows
  - Audio streams are automatically rebuilt when a device becomes available
- Compile-time configurable G-Buffer array for previously rendered frames (iterative shading and warping)
  - const generics control the number of history layers to preserve
- A rudimentary scene and mesh definition system (in development)
- Shader/scene hot-reload with script validation
  - selective render pipeline regeneration to prevent unnecessary work on the GPU
- A toggleable UI for statistics
- Single-threaded, interactive websocket RPC interface
- (broken) VS Code extension for controlling playback and configuring development features
- A concurrent plugin system (i.e. RPC, file watchers, etc.)
- Up to 5 unique sampled texture imports per scene
- UI widgets for resource information retrieval

Coming soon:

- Better scene/mesh definition schemas/logic
- Configuration of audio through the UI
- UI components for easing shader development
  - e.g. customized calculators, uniform variable views, CPU shader debugging
- Improve camera
- Better composite rendering support
- Implementation of compute shaders for vertex generation/manipulation
- Implementation of resource group compute shaders
- Fully functional project/scene management console
