struct Sprite<H> {
    handle: H,
    pos: (f32, f32),
}

impl<H> Sprite<H> {
    fn new(handle: H, pos: (f32, f32)) -> Self {
        Sprite { handle, pos }
    }
}

impl<H: Handle> Draw<H::Ctx> for Sprite<H> {
    fn draw(&self, ctx: &H::Ctx) {
        self.handle.draw(ctx, self.pos.0, self.pos.1);
    }
}

struct Both<First, Second> {
    first: First,
    second: Second,
}

impl<First, Second> Both<First, Second> {
    fn new(first: First, second: Second) -> Self {
        Both { first, second }
    }
}

type Button<H> = Both<Sprite<H>, Sprite<H>>;
type Dialog<H> = Both<Button<H>, Button<H>>;

impl<Ctx, First, Second> Draw<Ctx> for Both<First, Second>
where
    First: Draw<Ctx>,
    Second: Draw<Ctx>,
{
    fn draw(&self, ctx: &Ctx) {
        self.first.draw(ctx);
        self.second.draw(ctx);
    }
}

trait Draw<Ctx> {
    fn draw(&self, ctx: &Ctx);
}

trait Handle {
    type Ctx;

    fn draw(&self, ctx: &Self::Ctx, x: f32, y: f32);
}

pub fn draw() {
    let dialog = Dialog::new(
        Button::new(
            Sprite::new(String::from("Background"), (0.0, 0.0)),
            Sprite::new(String::from("Ok"), (0.0, 0.0)),
        ),
        Button::new(
            Sprite::new(String::from("Background"), (0.0, 0.0)),
            Sprite::new(String::from("Cancel"), (0.0, 0.0)),
        ),
    );

    dialog.draw(&());
}

impl Handle for String {
    type Ctx = ();

    fn draw(&self, _ctx: &(), x: f32, y: f32) {
        println!("Sprite {} at ({}, {})", self, x, y);
    }
}
