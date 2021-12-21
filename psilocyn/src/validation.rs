pub trait CubensisValidatedShader
where
    Self: Sized,
{
    fn validated(self) -> anyhow::Result<Self>;
}

impl CubensisValidatedShader for String {
    fn validated(self) -> anyhow::Result<Self> {
        log::debug!("Validating shader");
        log::trace!("Validating shader source: {}", &self);
        let module = naga::front::wgsl::parse_str(&self)?;
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::FLOAT64,
        );
        let _module_info = validator.validate(&module)?;
        Ok(self)
    }
}
