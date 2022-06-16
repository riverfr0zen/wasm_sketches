#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod base;

pub mod shapegen;

pub mod shifty_circle;

pub mod cellular;

pub mod path_changer;

pub mod shader_material;
