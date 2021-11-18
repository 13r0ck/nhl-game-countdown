use phf::phf_map;

pub static ICONS: phf::Map<&'static str, usize> = phf_map! {
    "21" => 47478, // Colorado Avalanche
};

pub static MSG: phf::Map<&'static str, &'static str> = phf_map! {
    "21" => "GO AVS GO", // Colorado Avalanche
};
