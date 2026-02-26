use bevy::math::{URect, UVec2};

pub struct TilemapSprite {
    pub name: &'static str,
    pub pixel_x: u32,
    pub pixel_y: u32,
}

pub struct TilemapDefinition {
    pub tile_width: u32,
    pub tile_height: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub sprites: &'static [TilemapSprite],
}

impl TilemapDefinition {
    pub const fn tile_size(&self) -> UVec2 {
        UVec2::new(self.tile_width, self.tile_height)
    }

    pub const fn atlas_size(&self) -> UVec2 {
        UVec2::new(self.atlas_width, self.atlas_height)
    }

    pub fn sprite_index(&self, name: &str) -> Option<usize> {
        self.sprites.iter().position(|sprite| sprite.name == name)
    }

    pub fn sprite_rect(&self, index: usize) -> URect {
        let sprite = &self.sprites[index];
        let min = UVec2::new(sprite.pixel_x, sprite.pixel_y);
        URect::from_corners(min, min + self.tile_size())
    }
}

pub const TILEMAP: TilemapDefinition = TilemapDefinition {
    tile_width: 32,
    tile_height: 32,
    atlas_width: 256,
    atlas_height: 320,
    sprites: &[
          TilemapSprite {
            name: "dirt",
            pixel_x: 128,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "green_grass",
            pixel_x: 160,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "green_grass_corner_in_tl",
            pixel_x: 192,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "green_grass_corner_in_tr",
            pixel_x: 224,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "green_grass_corner_in_bl",
            pixel_x: 192,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "green_grass_corner_in_br",
            pixel_x: 224,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "green_grass_corner_out_tl",
            pixel_x: 0,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "green_grass_corner_out_tr",
            pixel_x: 32,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "green_grass_corner_out_bl",
            pixel_x: 0,
            pixel_y: 96,
        },
        TilemapSprite {
            name: "green_grass_corner_out_br",
            pixel_x: 32,
            pixel_y: 96,
        },
        TilemapSprite {
            name: "green_grass_side_t",
            pixel_x: 64,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "green_grass_side_r",
            pixel_x: 96,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "green_grass_side_l",
            pixel_x: 64,
            pixel_y: 96,
        },
        TilemapSprite {
            name: "green_grass_side_b",
            pixel_x: 96,
            pixel_y: 96,
        },
        TilemapSprite {
            name: "yellow_grass",
            pixel_x: 0,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_corner_in_tl",
            pixel_x: 32,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_corner_in_tr",
            pixel_x: 64,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_corner_in_bl",
            pixel_x: 32,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "yellow_grass_corner_in_br",
            pixel_x: 64,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "yellow_grass_corner_out_tl",
            pixel_x: 96,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_corner_out_tr",
            pixel_x: 128,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_corner_out_bl",
            pixel_x: 96,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "yellow_grass_corner_out_br",
            pixel_x: 128,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "yellow_grass_side_t",
            pixel_x: 160,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_side_r",
            pixel_x: 192,
            pixel_y: 256,
        },
        TilemapSprite {
            name: "yellow_grass_side_l",
            pixel_x: 160,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "yellow_grass_side_b",
            pixel_x: 192,
            pixel_y: 288,
        },
        TilemapSprite {
            name: "water",
            pixel_x: 32,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_corner_in_tl",
            pixel_x: 64,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_corner_in_tr",
            pixel_x: 96,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_corner_in_bl",
            pixel_x: 64,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "water_corner_in_br",
            pixel_x: 96,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "water_corner_out_tl",
            pixel_x: 128,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_corner_out_tr",
            pixel_x: 160,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_corner_out_bl",
            pixel_x: 128,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "water_corner_out_br",
            pixel_x: 160,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "water_side_t",
            pixel_x: 192,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_side_r",
            pixel_x: 224,
            pixel_y: 192,
        },
        TilemapSprite {
            name: "water_side_l",
            pixel_x: 192,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "water_side_b",
            pixel_x: 224,
            pixel_y: 224,
        },
        TilemapSprite {
            name: "big_tree_1_tl",
            pixel_x: 0,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "big_tree_1_tr",
            pixel_x: 32,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "big_tree_1_bl",
            pixel_x: 0,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "big_tree_1_br",
            pixel_x: 32,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "big_tree_2_tl",
            pixel_x: 64,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "big_tree_2_tr",
            pixel_x: 96,
            pixel_y: 0,
        },
        TilemapSprite {
            name: "big_tree_2_bl",
            pixel_x: 64,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "big_tree_2_br",
            pixel_x: 96,
            pixel_y: 32,
        },
        TilemapSprite {
            name: "plant_1",
            pixel_x: 128,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "plant_2",
            pixel_x: 160,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "plant_3",
            pixel_x: 192,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "plant_4",
            pixel_x: 224,
            pixel_y: 64,
        },
        TilemapSprite {
            name: "rock_1",
            pixel_x: 0,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_2",
            pixel_x: 32,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_3",
            pixel_x: 64,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "rock_4",
            pixel_x: 96,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "small_tree_top",
            pixel_x: 128,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "small_tree_bottom",
            pixel_x: 128,
            pixel_y: 160,
        },
        TilemapSprite {
            name: "tree_stump_1",
            pixel_x: 192,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "tree_stump_2",
            pixel_x: 224,
            pixel_y: 128,
        },
        TilemapSprite {
            name: "tree_stump_3",
            pixel_x: 0,
            pixel_y: 192,
        },
    ]
};