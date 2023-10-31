
pub const REGION_AXIS: u64 = 8;

pub const REGION_SIZE: u64 = REGION_AXIS.pow(3);

pub const CHUNK_AXIS: u64 = 8;

pub const CHUNK_SIZE: u64 = CHUNK_AXIS.pow(3);

pub const WORLD_AXIS: u64 = 1_000_000;

pub const WORLD_SIZE: u64 = WORLD_AXIS.pow(3);

pub const CHUNKS_PER_REGION: u64 = REGION_SIZE / CHUNK_SIZE;
