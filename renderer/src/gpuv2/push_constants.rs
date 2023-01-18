use ash::vk;

pub struct Ranges {
    ranges: Vec<vk::PushConstantRange>
}

impl Ranges {
    pub fn new() -> Self {
        Self{ranges: vec![]}
    }

    pub fn add<T>(self, offset: u32, flags: vk::ShaderStageFlags) -> Self {
        let range = vk::PushConstantRange::builder()
            .stage_flags(flags)
            .offset(offset)
            .size(std::mem::size_of::<T>() as u32)
            .build();

        let mut new_ranges = self.ranges.clone();
        new_ranges.push(range);

        Self {
            ranges: new_ranges,
            ..self
        }
    }

    pub fn build(&self) -> &[vk::PushConstantRange] {
        self.ranges.as_slice()
    }
}