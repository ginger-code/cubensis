use crate::gui::{CubensisGuiApp, GuiHost};
use crate::presentation::PresentationPass;
use crate::resources::CubensisResourceCollection;
use hyphae::configuration::library::Library;

pub(crate) trait CubensisPresenter<'a, ResourceCollection, Gui, const HISTORY_DEPTH: usize>
where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn present(
        self,
        presentation_pass: &'a mut PresentationPass<HISTORY_DEPTH>,
        gui_host: &'a mut GuiHost,
        gui: &mut Gui,
        library: &Library,
        resource_collection: &ResourceCollection,
    ) -> Result<(), wgpu::SurfaceError>;
}
impl<'a, ResourceCollection, Gui, const HISTORY_DEPTH: usize>
    CubensisPresenter<'a, ResourceCollection, Gui, HISTORY_DEPTH> for wgpu::CommandEncoder
where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn present(
        self,
        presentation_pass: &'a mut PresentationPass<HISTORY_DEPTH>,
        gui_host: &'a mut GuiHost,
        gui: &mut Gui,
        library: &Library,
        resource_collection: &ResourceCollection,
    ) -> Result<(), wgpu::SurfaceError> {
        log::trace!("Rendering to history and presenting");
        presentation_pass.present(self, gui_host, gui, library, resource_collection)
    }
}
