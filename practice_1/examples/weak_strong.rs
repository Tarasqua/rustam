use std::rc::{Rc, Weak};

fn main() {
    let mut gfx = Graphics::default();

    println!("First draw:");
    gfx.draw(); // empty

    {
        let _sprite_0 = gfx.load_sprite(0);

        println!("Second draw:");
        gfx.draw(); // 1 sprite

        {
            let _sprite_1 = gfx.load_sprite(1);

            println!("Third draw");
            gfx.draw(); // 2 sprites
        } // _sprite_1 dropped
    } // _sprite_0 dropped

    println!("Last draw:");
    gfx.draw(); // empty
}

#[derive(Default)]
struct Graphics {
    sprites: Vec<Weak<Sprite>>,
}

impl Graphics {
    pub fn load_sprite(&mut self, id: u64) -> Rc<Sprite> {
        let sprite = Rc::new(Sprite::new(id));
        self.sprites.push(Rc::downgrade(&sprite)); // INFO: collect weak pointers before drawing
        sprite
    }

    pub fn draw(&mut self) {
        // INFO: retain (filter out) only valid weak pointers
        self.sprites.retain(|s| match s.upgrade() {
            Some(s) => {
                s.draw();
                true
            }
            None => false, // filter out invalid weak pointers
        });
    }
}

#[derive(Default)]
struct Sprite {
    id: u64,
}

impl Sprite {
    fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn draw(&self) {
        println!("\tDrawing sprite #{}", self.id);
    }
}
