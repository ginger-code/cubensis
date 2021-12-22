use crate::resources::audio::AudioResource;
use crate::resources::camera::CameraResource;
use crate::resources::time::TimeResource;
use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use psilocyn::device::GraphicsDevice;
use psilocyn::resources::{CubensisResource, CubensisResourceCollection};
use std::rc::Rc;

pub mod audio;
pub mod camera;
pub mod time;

pub struct ResourceCollection {
    graphics: Rc<GraphicsDevice>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    pub time: TimeResource,
    pub audio: AudioResource,
    pub camera: CameraResource,
}

impl CubensisResourceCollection for ResourceCollection {
    fn new(graphics: Rc<GraphicsDevice>, configuration: Configuration) -> Self {
        log::debug!("Creating resource collection");
        let time = TimeResource::new(graphics.clone(), 0, 0);
        let camera = CameraResource::new(graphics.clone(), 0, time.next_binding_offset_in_group());

        let audio = AudioResource::new(
            graphics.clone(),
            configuration,
            0,
            camera.next_binding_offset_in_group(),
        );
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = vec![
            time.get_bind_group_layout_entries(),
            camera.get_bind_group_layout_entries(),
            audio.get_bind_group_layout_entries(),
        ]
        .iter()
        .flat_map(|e| e.iter())
        .map(|e| e.to_owned())
        .collect();
        let bind_group_layout =
            create_bind_group_layout(&graphics, bind_group_layout_entries.as_slice());
        let bind_group_entries: Vec<wgpu::BindGroupEntry> = vec![
            time.get_bind_group_entries(),
            camera.get_bind_group_entries(),
            audio.get_bind_group_entries(),
        ]
        .iter()
        .flat_map(|e| e.iter())
        .map(|e| e.to_owned())
        .collect();
        let bind_group =
            create_bind_group(&graphics, &bind_group_layout, bind_group_entries.as_slice());
        Self {
            graphics,
            bind_group_layout,
            bind_group,
            time,
            audio,
            camera,
        }
    }

    fn update(&mut self, time_delta: std::time::Duration) {
        log::trace!("Updating resource collection");
        self.time.update(time_delta);
        self.camera.update(time_delta);
        //rebuild bind group and layout, if audio resource has been resized internally
        if self.audio.update(time_delta) {
            let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = vec![
                self.time.get_bind_group_layout_entries(),
                self.camera.get_bind_group_layout_entries(),
                self.audio.get_bind_group_layout_entries(),
            ]
            .iter()
            .flat_map(|e| e.iter())
            .map(|e| e.to_owned())
            .collect();
            self.bind_group_layout =
                create_bind_group_layout(&self.graphics, bind_group_layout_entries.as_slice());
            let bind_group_entries: Vec<wgpu::BindGroupEntry> = vec![
                self.time.get_bind_group_entries(),
                self.camera.get_bind_group_entries(),
                self.audio.get_bind_group_entries(),
            ]
            .iter()
            .flat_map(|e| e.iter())
            .map(|e| e.to_owned())
            .collect();
            self.bind_group = create_bind_group(
                &self.graphics,
                &self.bind_group_layout,
                bind_group_entries.as_slice(),
            );
        }
    }

    fn resize(&mut self) {
        log::trace!("Resizing resource collection");
        self.time.resize();
        self.camera.resize();
    }

    fn get_bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        log::trace!("Retrieving resource collection bind group layouts");
        vec![&self.bind_group_layout]
    }

    fn get_bind_groups(&self) -> Vec<&wgpu::BindGroup> {
        log::trace!("Retrieving resource collection bind groups");
        vec![&self.bind_group]
    }

    fn handle_event(&mut self, event: &winit::event::Event<'_, CubensisEvent>) {
        log::trace!("Handling event in resource collection");
        match self.audio.handle_or_capture_event(event) || self.time.handle_or_capture_event(event)
        {
            true => {
                log::debug!("Resource collection consumed an event: {:?}", event)
            }
            false => {}
        }
    }
}

fn create_bind_group_layout(
    graphics: &Rc<GraphicsDevice>,
    entries: &[wgpu::BindGroupLayoutEntry],
) -> wgpu::BindGroupLayout {
    log::trace!("Creating resource collection bind group layout");
    graphics
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Resource Collection Bind Group Layout"),
            entries,
        })
}
fn create_bind_group(
    graphics: &Rc<GraphicsDevice>,
    layout: &wgpu::BindGroupLayout,
    entries: &[wgpu::BindGroupEntry],
) -> wgpu::BindGroup {
    log::trace!("Creating resource collection bind group");
    graphics
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Resource Collection Bind Group"),
            layout,
            entries,
        })
}
