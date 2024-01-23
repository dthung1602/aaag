use aaag;

fn main() {
    let config = aaag::parse_config();
    let image = aaag::read_image(&config.image);
    let text = aaag::img_to_ascii_art(image, &config);
    aaag::write_text(config.output, text);
}