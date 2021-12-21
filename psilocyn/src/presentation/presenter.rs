use crate::gui::{CubensisGuiApp, GuiHost};
use crate::presentation::PresentationPass;
use crate::resources::CubensisResourceCollection;

pub(crate) trait CubensisPresenter<
    'a,
    ResourceCollection,
    Gui,
    const HISTORY_DEPTH: usize,
    const HISTORY_BIND_GROUP: u32,
    const HISTORY_TEXTURE_BINDING_INDEX: u32,
    const HISTORY_SAMPLER_BINDING_INDEX: u32,
> where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn present(
        self,
        presentation_pass: &'a mut PresentationPass<
            HISTORY_DEPTH,
            HISTORY_BIND_GROUP,
            HISTORY_TEXTURE_BINDING_INDEX,
            HISTORY_SAMPLER_BINDING_INDEX,
        >,
        gui_host: &'a mut GuiHost,
        gui: &Gui,
        resource_collection: &ResourceCollection,
    ) -> Result<(), wgpu::SurfaceError>;
}
impl<
        'a,
        ResourceCollection,
        Gui,
        const HISTORY_DEPTH: usize,
        const HISTORY_BIND_GROUP: u32,
        const TEXTURE_BINDING_INDEX: u32,
        const SAMPLER_BINDING_INDEX: u32,
    >
    CubensisPresenter<
        'a,
        ResourceCollection,
        Gui,
        HISTORY_DEPTH,
        HISTORY_BIND_GROUP,
        TEXTURE_BINDING_INDEX,
        SAMPLER_BINDING_INDEX,
    > for wgpu::CommandEncoder
where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn present(
        self,
        presentation_pass: &'a mut PresentationPass<
            HISTORY_DEPTH,
            HISTORY_BIND_GROUP,
            TEXTURE_BINDING_INDEX,
            SAMPLER_BINDING_INDEX,
        >,
        gui_host: &'a mut GuiHost,
        gui: &Gui,
        resource_collection: &ResourceCollection,
    ) -> Result<(), wgpu::SurfaceError> {
        log::trace!("Rendering to history and presenting");
        presentation_pass.present(self, gui_host, gui, resource_collection)
    }
}
