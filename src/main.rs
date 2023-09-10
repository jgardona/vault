use cli::execute;

mod cli;
mod model;
mod store;

fn main() {
    execute().unwrap();
}
