mod bar;
mod food;
use bar::run as bar_run;
use food::run as foo_run;

fn main() {
    foo_run();
    bar_run();
}
