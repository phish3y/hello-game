use hello_game::run;

fn main() {
    pollster::block_on(run())
}
